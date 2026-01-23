//! UCCI (Universal Chinese Chess Protocol) implementation
//!
//! This module provides a full UCCI 3.0 compliant interface for communicating
//! with external Chinese chess engines.

pub mod protocol;
pub mod serializer;
pub mod parser;
pub mod state;
pub mod engine;

pub use protocol::{
    EngineState, GoMode, MoveResult, OptionType, UcciCommand, UcciResponse,
};
pub use parser::ParseError;
pub use state::{StateError, UcciStateMachine};
pub use engine::EngineError;
