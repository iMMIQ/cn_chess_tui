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
// These are public APIs - allow unused_imports for external use
#[allow(unused_imports)]
pub use chinese::{
    move_to_chinese, move_to_chinese_with_context, piece_to_chinese, MovementDirection,
};

// Re-export WXF notation functions
// These are public APIs - allow unused_imports for external use
#[allow(unused_imports)]
pub use wxf::{
    direction_to_wxf, move_to_wxf, parse_wxf_move, piece_to_wxf_letter, wxf_letter_to_piece_type,
    wxf_symbol_to_direction,
};
