pub mod board;
pub mod game;
pub mod types;
pub mod ui;

pub use board::Board;
pub use game::{Game, GameResult, GameState, Move, MoveError};
pub use types::{Color, Piece, PieceType, Position};

// Re-export UI for testing
pub use ui::UI;
