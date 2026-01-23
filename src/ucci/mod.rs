//! UCCI (Universal Chinese Chess Protocol) implementation
//!
//! This module provides a full UCCI 3.0 compliant interface for communicating
//! with external Chinese chess engines.

pub mod client;
pub mod engine;
pub mod parser;
pub mod protocol;
pub mod serializer;
pub mod state;

pub use client::{EngineInfo, Info, MoveResult, UcciClient};
pub use engine::EngineError;
pub use parser::ParseError;
pub use protocol::{EngineState, GoMode, OptionType, UcciCommand, UcciResponse};
pub use state::{StateError, UcciStateMachine};
