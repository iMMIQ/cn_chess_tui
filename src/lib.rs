pub mod board;
pub mod fen;
pub mod fen_io;
pub mod fen_print;
pub mod game;
pub mod types;
pub mod ui;

pub use board::Board;
pub use fen::{FenError, fen_to_board, board_to_fen};
pub use fen_io::{load_fen_file, read_fen_file, write_fen_file};
pub use fen_print::print_board_ascii;
pub use game::{Game, GameResult, GameState, Move, MoveError};
pub use types::{Color, Piece, PieceType, Position};

// Re-export UI for testing
pub use ui::UI;
