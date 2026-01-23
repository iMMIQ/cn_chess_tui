//! UCCI engine state machine

use crate::ucci::protocol::{EngineState, UcciCommand, UcciResponse};

/// Error type for state machine violations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    InvalidCommand(String),
    UnexpectedResponse(String),
    NotInIdle,
    NotInThinking,
}

/// UCCI state machine to validate protocol flow
#[derive(Debug, Clone)]
pub struct UcciStateMachine {
    state: EngineState,
    #[allow(dead_code)]
    supports_ponder: bool,
}

impl UcciStateMachine {
    /// Create a new state machine in Boot state
    pub fn new() -> Self {
        Self {
            state: EngineState::Boot,
            supports_ponder: false,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> EngineState {
        self.state
    }

    /// Check if a command can be sent in current state
    pub fn can_send(&self, cmd: &UcciCommand) -> bool {
        match &self.state {
            EngineState::Boot => matches!(cmd, UcciCommand::Ucci),
            EngineState::Idle => !matches!(cmd, UcciCommand::PonderHit { .. }),
            EngineState::Thinking => {
                matches!(cmd, UcciCommand::Stop | UcciCommand::PonderHit { .. })
            }
        }
    }

    /// Transition state based on command being sent
    pub fn transition(&mut self, cmd: &UcciCommand) -> Result<(), StateError> {
        if !self.can_send(cmd) {
            return Err(StateError::InvalidCommand(format!(
                "{:?} cannot be sent in {:?} state",
                cmd, self.state
            )));
        }

        match cmd {
            UcciCommand::Ucci => {
                // Stay in Boot until ucciok received
            }
            UcciCommand::Go { .. } => {
                self.state = EngineState::Thinking;
            }
            UcciCommand::Quit => {
                // Will terminate after bye
            }
            _ => {
                // Other commands don't change state
            }
        }

        Ok(())
    }

    /// Update state based on response received
    pub fn on_response(&mut self, resp: &UcciResponse) -> Result<(), StateError> {
        match resp {
            UcciResponse::UcciOk => {
                if self.state != EngineState::Boot {
                    return Err(StateError::UnexpectedResponse(
                        "ucciok not expected in current state".to_string(),
                    ));
                }
                self.state = EngineState::Idle;
            }
            UcciResponse::BestMove { .. } | UcciResponse::NoBestMove => {
                if self.state != EngineState::Thinking {
                    return Err(StateError::UnexpectedResponse(
                        "bestmove/nobestmove not in thinking state".to_string(),
                    ));
                }
                self.state = EngineState::Idle;
            }
            UcciResponse::Bye => {
                // Engine terminating
            }
            _ => {
                // Other responses don't change state
            }
        }
        Ok(())
    }

    /// Check if in idle state
    pub fn is_idle(&self) -> bool {
        self.state == EngineState::Idle
    }

    /// Check if in thinking state
    pub fn is_thinking(&self) -> bool {
        self.state == EngineState::Thinking
    }

    /// Check if in boot state
    pub fn is_boot(&self) -> bool {
        self.state == EngineState::Boot
    }
}

impl Default for UcciStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ucci::GoMode;

    #[test]
    fn test_initial_state() {
        let sm = UcciStateMachine::new();
        assert_eq!(sm.current_state(), EngineState::Boot);
        assert!(sm.is_boot());
        assert!(!sm.is_idle());
        assert!(!sm.is_thinking());
    }

    #[test]
    fn test_can_send_ucci_in_boot() {
        let sm = UcciStateMachine::new();
        assert!(sm.can_send(&UcciCommand::Ucci));
    }

    #[test]
    fn test_cannot_send_go_in_boot() {
        let sm = UcciStateMachine::new();
        assert!(!sm.can_send(&UcciCommand::Go {
            mode: GoMode::Depth(10),
            ponder: false,
            draw: false,
        }));
    }

    #[test]
    fn test_ucciok_transitions_to_idle() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        assert_eq!(sm.current_state(), EngineState::Idle);
        assert!(sm.is_idle());
    }

    #[test]
    fn test_go_transitions_to_thinking() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        sm.transition(&UcciCommand::Go {
            mode: GoMode::Depth(10),
            ponder: false,
            draw: false,
        })
        .unwrap();
        assert_eq!(sm.current_state(), EngineState::Thinking);
        assert!(sm.is_thinking());
    }

    #[test]
    fn test_bestmove_transitions_to_idle() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        sm.transition(&UcciCommand::Go {
            mode: GoMode::Depth(10),
            ponder: false,
            draw: false,
        })
        .unwrap();
        sm.on_response(&UcciResponse::BestMove {
            mv: "h2e2".to_string(),
            ponder: None,
            draw: false,
            resign: false,
        })
        .unwrap();
        assert_eq!(sm.current_state(), EngineState::Idle);
    }

    #[test]
    fn test_can_send_stop_in_thinking() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        sm.transition(&UcciCommand::Go {
            mode: GoMode::Depth(10),
            ponder: false,
            draw: false,
        })
        .unwrap();
        assert!(sm.can_send(&UcciCommand::Stop));
    }

    #[test]
    fn test_cannot_send_position_in_thinking() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        sm.transition(&UcciCommand::Go {
            mode: GoMode::Depth(10),
            ponder: false,
            draw: false,
        })
        .unwrap();
        assert!(!sm.can_send(&UcciCommand::Position {
            fen: "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1"
                .to_string(),
            moves: vec![],
        }));
    }

    #[test]
    fn test_invalid_command_in_boot() {
        let mut sm = UcciStateMachine::new();
        let result = sm.transition(&UcciCommand::IsReady);
        assert!(result.is_err());
    }

    #[test]
    fn test_unexpected_ucciok() {
        let mut sm = UcciStateMachine::new();
        sm.transition(&UcciCommand::Ucci).unwrap();
        sm.on_response(&UcciResponse::UcciOk).unwrap();
        let result = sm.on_response(&UcciResponse::UcciOk);
        assert!(result.is_err());
    }
}
