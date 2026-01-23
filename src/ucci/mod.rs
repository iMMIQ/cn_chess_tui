//! UCCI (Universal Chinese Chess Protocol) implementation
//!
//! This module provides a full UCCI 3.0 compliant interface for communicating
//! with external Chinese chess engines.

pub mod protocol;

pub use protocol::{
    EngineState, GoMode, MoveResult, OptionType, UcciCommand, UcciResponse,
};
