pub mod board;
pub mod fen;
pub mod fen_io;
pub mod fen_print;
pub mod game;
pub mod notation;
pub mod types;
pub mod ui;

pub use board::Board;
pub use fen::{board_to_fen, fen_to_board, FenError};
pub use fen_io::{load_fen_file, read_fen_file, write_fen_file};
pub use fen_print::{print_board_ascii, print_game_state};
pub use game::{Game, GameResult, GameState, Move, MoveError};
pub use types::{move_to_simple_notation, Color, Piece, PieceType, Position};

// Re-export UI for testing
pub use ui::UI;

// Re-export notation types
pub use notation::iccs::{iccs_to_move, iccs_to_position, move_to_iccs, position_to_iccs};
pub use notation::{move_to_chinese, piece_to_chinese, MovementDirection};
