pub mod board;
pub mod fen;
pub mod fen_io;
pub mod fen_print;
pub mod game;
pub mod notation;
pub mod types;
pub mod ui;

pub use board::Board;
pub use fen::{FenError, fen_to_board, board_to_fen};
pub use fen_io::{load_fen_file, read_fen_file, write_fen_file};
pub use fen_print::{print_board_ascii, print_game_state};
pub use game::{Game, GameResult, GameState, Move, MoveError};
pub use types::{Color, Piece, PieceType, Position, move_to_simple_notation};

// Re-export UI for testing
pub use ui::UI;

// Re-export notation types
pub use notation::iccs::{position_to_iccs, iccs_to_position, move_to_iccs, iccs_to_move};
