//! Engine process spawning and communication

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::Duration;

/// Error type for engine operations
#[derive(Debug)]
pub enum EngineError {
    SpawnFailed(std::io::Error),
    WriteFailed(std::io::Error),
    ReadFailed(std::io::Error),
    UnexpectedEof,
    Crashed(i32),
    Timeout,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::SpawnFailed(e) => write!(f, "Failed to spawn engine: {}", e),
            EngineError::WriteFailed(e) => write!(f, "Failed to write to engine: {}", e),
            EngineError::ReadFailed(e) => write!(f, "Failed to read from engine: {}", e),
            EngineError::UnexpectedEof => write!(f, "Unexpected end of input from engine"),
            EngineError::Crashed(code) => write!(f, "Engine crashed with exit code {}", code),
            EngineError::Timeout => write!(f, "Engine operation timed out"),
        }
    }
}

impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::SpawnFailed(e) => Some(e),
            EngineError::WriteFailed(e) => Some(e),
            EngineError::ReadFailed(e) => Some(e),
            _ => None,
        }
    }
}

/// Manages communication with an external UCCI engine process
pub struct EngineProcess {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl EngineProcess {
    /// Spawn a new engine process
    pub fn spawn(executable: &str) -> Result<Self, EngineError> {
        let mut child = Command::new(executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // Inherit stderr for debugging
            .spawn()
            .map_err(EngineError::SpawnFailed)?;

        let stdin = child.stdin.take().ok_or_else(|| {
            EngineError::SpawnFailed(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Failed to open stdin",
            ))
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            EngineError::SpawnFailed(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Failed to open stdout",
            ))
        })?;

        let stdin = BufWriter::new(stdin);
        let stdout = BufReader::new(stdout);

        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    /// Send a command to the engine
    pub fn send_command(&mut self, cmd: &str) -> Result<(), EngineError> {
        writeln!(self.stdin, "{}", cmd).map_err(EngineError::WriteFailed)?;
        self.stdin.flush().map_err(EngineError::WriteFailed)?;
        Ok(())
    }

    /// Read a single line from the engine
    pub fn read_line(&mut self) -> Result<String, EngineError> {
        let mut line = String::new();
        self.stdout
            .read_line(&mut line)
            .map_err(EngineError::ReadFailed)?;

        if line.is_empty() {
            return Err(EngineError::UnexpectedEof);
        }

        // Trim newline but preserve other whitespace
        Ok(line.trim_end().to_string())
    }

    /// Read a line with timeout (NOT YET IMPLEMENTED - currently blocks)
    ///
    /// TODO: Implement actual timeout with async I/O or separate thread.
    /// For now, this blocks indefinitely just like read_line().
    pub fn read_line_timeout(&mut self, _timeout_ms: u64) -> Result<String, EngineError> {
        // TODO: Implement actual timeout with async I/O or separate thread
        // For now, this blocks indefinitely
        self.read_line()
    }

    /// Check if the engine process is still running
    pub fn is_running(&mut self) -> bool {
        self.child
            .try_wait()
            .map(|status| status.is_none())
            .unwrap_or(false)
    }

    /// Terminate the engine process gracefully
    pub fn terminate(mut self) -> Result<(), EngineError> {
        // Try to send quit command first
        let _ = self.send_command("quit");

        // Give engine time to exit gracefully
        thread::sleep(Duration::from_millis(100));

        // Check if it exited
        if let Ok(Some(_)) = self.child.try_wait() {
            return Ok(());
        }

        // Force kill if still running
        self.child.kill().map_err(|_| {
            EngineError::SpawnFailed(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Failed to kill engine",
            ))
        })?;

        Ok(())
    }

    /// Get the process ID
    pub fn pid(&self) -> u32 {
        self.child.id()
    }
}

/// Ensure the engine process is properly terminated when dropped
impl Drop for EngineProcess {
    fn drop(&mut self) {
        // Force kill if still running to prevent zombie processes
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Create a mock engine script that echoes commands
    #[cfg(unix)]
    fn create_mock_engine() -> tempfile::TempPath {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"#!/bin/bash
while read line; do
    case "$line" in
        "ucci")
            echo "id name MockEngine"
            echo "id author Test"
            echo "ucciok"
            ;;
        "isready")
            echo "readyok"
            ;;
        "go "*)
            echo "info depth 10 score 100"
            echo "bestmove h2e2"
            ;;
        "quit")
            echo "bye"
            exit 0
            ;;
    esac
done
"#
        )
        .unwrap();

        // Flush and sync to ensure all data is written
        file.as_file().flush().unwrap();
        file.as_file().sync_all().unwrap();

        // Make executable
        use std::os::unix::fs::PermissionsExt;
        let mut perm = file.as_file().metadata().unwrap().permissions();
        perm.set_mode(0o755);
        file.as_file().set_permissions(perm).unwrap();

        // Convert to TempPath to release the file handle
        file.into_temp_path()
    }

    #[test]
    #[cfg(unix)]
    fn test_spawn_and_communicate() {
        let mock = create_mock_engine();
        let mut engine = EngineProcess::spawn(mock.to_str().unwrap()).unwrap();

        engine.send_command("ucci").unwrap();
        let resp1 = engine.read_line().unwrap();
        assert!(resp1.contains("id name"));

        let resp2 = engine.read_line().unwrap();
        assert!(resp2.contains("id author"));

        let resp3 = engine.read_line().unwrap();
        assert_eq!(resp3, "ucciok");

        engine.send_command("isready").unwrap();
        let resp = engine.read_line().unwrap();
        assert_eq!(resp, "readyok");

        engine.terminate().unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn test_go_command() {
        let mock = create_mock_engine();
        let mut engine = EngineProcess::spawn(mock.to_str().unwrap()).unwrap();

        engine.send_command("ucci").unwrap();
        while engine.read_line().unwrap() != "ucciok" {}

        engine.send_command("go depth 10").unwrap();
        let resp1 = engine.read_line().unwrap();
        assert!(resp1.contains("info depth"));

        let resp2 = engine.read_line().unwrap();
        assert_eq!(resp2, "bestmove h2e2");

        engine.terminate().unwrap();
    }
}
