//! Move notation formats for Chinese Chess
//!
//! Supports three formats:
//! - ICCS: Internet Chinese Chess Server coordinate format (e.g., "h2e2")
//! - Chinese: Traditional vertical line format (e.g., "炮二平五")
//! - WXF: World XiangQi Federation format (e.g., "C2.5")

pub mod chinese;
pub mod iccs;
pub mod wxf;

// Re-export Chinese notation types and functions
pub use chinese::{move_to_chinese, move_to_chinese_with_context, piece_to_chinese, MovementDirection};
