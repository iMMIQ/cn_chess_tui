pub mod board;
pub mod fen;
pub mod fen_io;
pub mod fen_print;
pub mod game;
pub mod notation;
pub mod pgn;
pub mod types;
pub mod ui;
pub mod xml;
pub mod ucci;

pub use board::Board;
pub use fen::{board_to_fen, fen_to_board, FenError};
pub use fen_io::{load_fen_file, read_fen_file, write_fen_file};
pub use fen_print::{print_board_ascii, print_game_state};
pub use game::{Game, GameResult, GameState, Move, MoveError};
pub use pgn::{PgnGame, PgnGameResult, PgnMove, PgnTag};
// Re-export PgnGameResult as PgnResult for convenience
pub use pgn::PgnGameResult as PgnResult;
pub use types::{move_to_simple_notation, Color, Piece, PieceType, Position};
pub use xml::{escape_xml, pgn_to_xml, save_content, unescape_xml, xml_to_pgn};

// Re-export UI for testing
pub use ui::UI;

// Re-export notation types
pub use notation::iccs::{iccs_to_move, iccs_to_position, move_to_iccs, position_to_iccs};
pub use notation::{move_to_chinese, move_to_chinese_with_context, piece_to_chinese, MovementDirection};
