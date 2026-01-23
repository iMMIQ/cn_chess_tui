//! High-level UCCI client API

use std::collections::HashMap;

use crate::ucci::engine::{EngineError, EngineProcess};
use crate::ucci::parser::parse_response;
use crate::ucci::protocol::{GoMode, OptionType, UcciCommand};
use crate::ucci::state::UcciStateMachine;

// Re-export MoveResult since it's part of the public API
pub use crate::ucci::protocol::MoveResult;

/// Info data received during engine search
#[derive(Debug, Clone)]
pub struct Info {
    pub time_ms: Option<u64>,
    pub nodes: Option<u64>,
    pub depth: Option<u32>,
    pub score: Option<i32>,
    pub pv: Vec<String>,
    pub currmove: Option<String>,
    pub message: Option<String>,
}

/// Engine information collected during initialization
#[derive(Debug, Clone)]
pub struct EngineInfo {
    pub name: String,
    pub author: Option<String>,
    pub copyright: Option<String>,
    pub user: Option<String>,
}

impl Default for EngineInfo {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            author: None,
            copyright: None,
            user: None,
        }
    }
}

/// High-level UCCI client
pub struct UcciClient {
    engine: EngineProcess,
    state: UcciStateMachine,
    info: EngineInfo,
    options: HashMap<String, EngineOption>,
    last_infos: Vec<Info>,
}

impl UcciClient {
    /// Create a new UCCI client and spawn the engine
    pub fn new(executable: &str) -> Result<Self, EngineError> {
        let engine = EngineProcess::spawn(executable)?;
        Ok(Self {
            engine,
            state: UcciStateMachine::new(),
            info: EngineInfo::default(),
            options: HashMap::new(),
            last_infos: Vec::new(),
        })
    }

    /// Initialize the engine (send ucci and wait for ucciok)
    pub fn initialize(&mut self) -> Result<(), EngineError> {
        self.engine.send_command("ucci")?;

        // Read responses until ucciok
        loop {
            let line = self.engine.read_line()?;
            let resp = parse_response(&line).map_err(|_| EngineError::ReadFailed(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Parse error"),
            ))?;

            match resp {
                crate::ucci::UcciResponse::UcciOk => {
                    self.state.on_response(&crate::ucci::UcciResponse::UcciOk).map_err(
                        |e| EngineError::WriteFailed(std::io::Error::other(
                            format!("State error: {:?}", e),
                        )),
                    )?;
                    break;
                }
                crate::ucci::UcciResponse::Id { field, value } => {
                    match field.as_str() {
                        "name" => self.info.name = value,
                        "author" => self.info.author = Some(value),
                        "copyright" => self.info.copyright = Some(value),
                        "user" => self.info.user = Some(value),
                        _ => {}
                    }
                }
                crate::ucci::UcciResponse::Option {
                    name,
                    type_,
                    min,
                    max,
                    vars,
                    default,
                } => {
                    self.options.insert(
                        name.clone(),
                        EngineOption {
                            name,
                            type_,
                            min,
                            max,
                            vars,
                            default,
                        },
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Check if engine is ready
    pub fn is_ready(&mut self) -> Result<bool, EngineError> {
        self.engine.send_command("isready")?;
        let line = self.engine.read_line()?;
        Ok(line == "readyok")
    }

    /// Set an engine option
    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        self.ensure_idle()?;
        let cmd = UcciCommand::SetOption {
            name: name.to_string(),
            value: Some(value.to_string()),
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Set the board position
    pub fn set_position(&mut self, fen: &str, moves: &[String]) -> Result<(), EngineError> {
        self.ensure_idle()?;
        let cmd = UcciCommand::Position {
            fen: fen.to_string(),
            moves: moves.to_vec(),
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Set banned moves (for solving repetition problems)
    pub fn ban_moves(&mut self, moves: &[String]) -> Result<(), EngineError> {
        self.ensure_idle()?;
        let cmd = UcciCommand::BanMoves {
            moves: moves.to_vec(),
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Start searching to a specific depth
    pub fn go_depth(&mut self, depth: u32) -> Result<(), EngineError> {
        self.ensure_idle()?;
        self.last_infos.clear();
        let cmd = UcciCommand::Go {
            mode: GoMode::Depth(depth),
            ponder: false,
            draw: false,
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Start searching with a time limit (in milliseconds)
    pub fn go_time(&mut self, time_ms: u64) -> Result<(), EngineError> {
        self.ensure_idle()?;
        self.last_infos.clear();
        let cmd = UcciCommand::Go {
            mode: GoMode::Time {
                time: time_ms,
                movestogo: None,
                increment: None,
                opptime: None,
                oppmovestogo: None,
                oppincrement: None,
            },
            ponder: false,
            draw: false,
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Start infinite search (until stop)
    pub fn go_infinite(&mut self) -> Result<(), EngineError> {
        self.ensure_idle()?;
        self.last_infos.clear();
        let cmd = UcciCommand::Go {
            mode: GoMode::Infinite,
            ponder: false,
            draw: false,
        };
        self.state.transition(&cmd).map_err(|e| {
            EngineError::WriteFailed(std::io::Error::other(format!("{:?}", e)))
        })?;
        self.engine.send_command(&cmd.serialize())
    }

    /// Stop the current search and get the result
    pub fn stop(&mut self) -> Result<MoveResult, EngineError> {
        if !self.state.is_thinking() {
            return Err(EngineError::WriteFailed(std::io::Error::other(
                "Not in thinking state",
            )));
        }

        self.engine.send_command("stop")?;

        // Read info messages until bestmove
        loop {
            let line = self.engine.read_line()?;
            let resp = parse_response(&line).map_err(|_| EngineError::ReadFailed(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Parse error"),
            ))?;

            match resp {
                crate::ucci::UcciResponse::BestMove {
                    ref mv,
                    ref ponder,
                    draw,
                    resign,
                } => {
                    let result = if resign {
                        MoveResult::Resign
                    } else if draw {
                        MoveResult::Draw
                    } else {
                        MoveResult::Move(mv.clone(), ponder.clone())
                    };

                    self.state.on_response(&resp).map_err(|e| {
                        EngineError::WriteFailed(std::io::Error::other(
                            format!("{:?}", e),
                        ))
                    })?;

                    return Ok(result);
                }
                crate::ucci::UcciResponse::NoBestMove => {
                    self.state.on_response(&resp).map_err(|e| {
                        EngineError::WriteFailed(std::io::Error::other(
                            format!("{:?}", e),
                        ))
                    })?;
                    return Ok(MoveResult::NoMove);
                }
                crate::ucci::UcciResponse::Info {
                    time,
                    nodes,
                    depth,
                    score,
                    pv,
                    currmove,
                    message,
                } => {
                    self.last_infos.push(Info {
                        time_ms: time,
                        nodes,
                        depth,
                        score,
                        pv,
                        currmove,
                        message,
                    });
                }
                _ => {}
            }
        }
    }

    /// Get the engine info
    pub fn engine_info(&self) -> &EngineInfo {
        &self.info
    }

    /// Read and drain the last info messages received during the previous search
    pub fn read_info(&mut self) -> Vec<Info> {
        std::mem::take(&mut self.last_infos)
    }

    /// Get available engine options
    pub fn options(&self) -> &HashMap<String, EngineOption> {
        &self.options
    }

    /// Check if currently thinking
    pub fn is_thinking(&self) -> bool {
        self.state.is_thinking()
    }

    /// Check if currently idle
    pub fn is_idle(&self) -> bool {
        self.state.is_idle()
    }

    /// Shutdown the engine gracefully
    pub fn shutdown(mut self) -> Result<(), EngineError> {
        self.engine.send_command("quit")?;

        // Wait for bye
        let _ = self.engine.read_line();

        self.engine.terminate()
    }

    fn ensure_idle(&self) -> Result<(), EngineError> {
        if !self.state.is_idle() {
            return Err(EngineError::WriteFailed(std::io::Error::other(
                format!("Not idle, current state: {:?}", self.state.current_state()),
            )));
        }
        Ok(())
    }
}

/// Engine option descriptor
#[derive(Debug, Clone)]
pub struct EngineOption {
    pub name: String,
    pub type_: OptionType,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub vars: Vec<String>,
    pub default: Option<String>,
}
