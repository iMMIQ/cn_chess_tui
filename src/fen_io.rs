//! FEN file I/O utilities
//!
//! Provides functions to read and write .fen files

use std::fs;
use std::path::Path;

/// Read a FEN string from a file
///
/// # Arguments
/// - `path`: Path to the .fen file
///
/// # Returns
/// The FEN string as a String
///
/// # Errors
/// Returns io::Error if the file cannot be read
pub fn read_fen_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path)?;
    // Remove any trailing whitespace/newlines
    Ok(content.trim().to_string())
}

/// Write a FEN string to a file
///
/// # Arguments
/// - `path`: Path where the .fen file should be written
/// - `fen`: The FEN string to write
///
/// # Errors
/// Returns io::Error if the file cannot be written
pub fn write_fen_file<P: AsRef<Path>>(path: P, fen: &str) -> Result<(), std::io::Error> {
    fs::write(path, fen)
}

/// Read a FEN file and return the parsed (Board, turn) tuple
///
/// # Arguments
/// - `path`: Path to the .fen file
///
/// # Returns
/// (Board, Color) tuple
///
/// # Errors
/// Returns io::Error for file errors, or FenError for parsing errors
pub fn load_fen_file<P: AsRef<Path>>(path: P) -> Result<(crate::board::Board, crate::types::Color), Box<dyn std::error::Error>> {
    let fen = read_fen_file(path)?;
    let (board, turn) = crate::fen::fen_to_board(&fen)?;
    Ok((board, turn))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_fen_file() {
        use tempfile::NamedTempFile;

        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";

        let temp_file = NamedTempFile::new().unwrap();
        write_fen_file(&temp_file.path(), fen).unwrap();

        let read_fen = read_fen_file(&temp_file.path()).unwrap();
        assert_eq!(read_fen, fen);
    }

    #[test]
    fn test_read_fen_file_with_newline() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1\n";

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(fen.as_bytes()).unwrap();

        let read_fen = read_fen_file(&temp_file.path()).unwrap();
        assert_eq!(read_fen, fen.trim());
    }
}
