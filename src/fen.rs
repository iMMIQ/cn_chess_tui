//! FEN (Forsyth-Edwards Notation) format support for Chinese Chess
//!
//! FEN format: `board_setup turn - - half_move_count full_move_count`
//!
//! Example initial position:
//! `rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1`
//!
//! Piece mapping:
//! - Upper case: Red (R=车, N=马, B=相/象, A=仕/士, K=帅, C=炮, P=兵)
//! - Lower case: Black (r=车, n=马, b=相/象, a=仕/士, k=将, c=炮, p=卒)

use crate::board::Board;
use crate::types::{Color, Piece, PieceType, Position};
use std::collections::HashMap;

/// Errors that can occur during FEN parsing
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum FenError {
    InvalidFormat,
    #[allow(dead_code)]
    InvalidBoardSection,
    InvalidPiece(char),
    InvalidRankCount,
    InvalidFileCount,
    InvalidTurn,
    InvalidMoveCount,
    #[allow(dead_code)]
    MissingMovesKeyword,
    #[allow(dead_code)]
    EmptyMovesList,
    #[allow(dead_code)]
    InvalidMoveInHistory(String),
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenError::InvalidFormat => write!(f, "Invalid FEN format"),
            FenError::InvalidBoardSection => write!(f, "Invalid board section in FEN"),
            FenError::InvalidPiece(c) => write!(f, "Invalid piece character: {}", c),
            FenError::InvalidRankCount => write!(f, "Invalid rank count (expected 10)"),
            FenError::InvalidFileCount => write!(f, "Invalid file count (expected 9)"),
            FenError::InvalidTurn => write!(f, "Invalid turn indicator (expected 'w' or 'b')"),
            FenError::InvalidMoveCount => write!(f, "Invalid move count"),
            FenError::MissingMovesKeyword => write!(f, "Missing 'moves' keyword in FEN with moves"),
            FenError::EmptyMovesList => write!(f, "Empty moves list in FEN with moves"),
            FenError::InvalidMoveInHistory(mv) => write!(f, "Invalid move in history: {}", mv),
        }
    }
}

impl std::error::Error for FenError {}

/// Parse a single piece character to a Piece
pub fn parse_piece(ch: char) -> Option<Piece> {
    let (piece_type, color) = match ch {
        // Red pieces (uppercase)
        'R' => (PieceType::Chariot, Color::Red),
        'N' => (PieceType::Horse, Color::Red),
        'B' => (PieceType::Elephant, Color::Red),
        'A' => (PieceType::Advisor, Color::Red),
        'K' => (PieceType::General, Color::Red),
        'C' => (PieceType::Cannon, Color::Red),
        'P' => (PieceType::Soldier, Color::Red),
        // Black pieces (lowercase)
        'r' => (PieceType::Chariot, Color::Black),
        'n' => (PieceType::Horse, Color::Black),
        'b' => (PieceType::Elephant, Color::Black),
        'a' => (PieceType::Advisor, Color::Black),
        'k' => (PieceType::General, Color::Black),
        'c' => (PieceType::Cannon, Color::Black),
        'p' => (PieceType::Soldier, Color::Black),
        _ => return None,
    };
    Some(Piece::new(piece_type, color))
}

/// Convert a Piece to its FEN character representation
pub fn piece_to_fen(piece: Piece) -> char {
    match (piece.color, piece.piece_type) {
        (Color::Red, PieceType::Chariot) => 'R',
        (Color::Red, PieceType::Horse) => 'N',
        (Color::Red, PieceType::Elephant) => 'B',
        (Color::Red, PieceType::Advisor) => 'A',
        (Color::Red, PieceType::General) => 'K',
        (Color::Red, PieceType::Cannon) => 'C',
        (Color::Red, PieceType::Soldier) => 'P',
        (Color::Black, PieceType::Chariot) => 'r',
        (Color::Black, PieceType::Horse) => 'n',
        (Color::Black, PieceType::Elephant) => 'b',
        (Color::Black, PieceType::Advisor) => 'a',
        (Color::Black, PieceType::General) => 'k',
        (Color::Black, PieceType::Cannon) => 'c',
        (Color::Black, PieceType::Soldier) => 'p',
    }
}

/// Parse a single rank (row) of the FEN board section
fn parse_rank(rank_str: &str, y: usize) -> Result<Vec<(Position, Piece)>, FenError> {
    let mut pieces = Vec::new();
    let mut x = 0;

    for ch in rank_str.chars() {
        if ch == '/' {
            continue; // Skip rank separator
        }

        if let Some(digit) = ch.to_digit(10) {
            // Empty squares
            let empty_count = digit as usize;
            x += empty_count;
        } else if let Some(piece) = parse_piece(ch) {
            if x >= 9 {
                return Err(FenError::InvalidFileCount);
            }
            pieces.push((Position::from_xy(x, y), piece));
            x += 1;
        } else {
            return Err(FenError::InvalidPiece(ch));
        }
    }

    if x != 9 {
        return Err(FenError::InvalidFileCount);
    }

    Ok(pieces)
}

/// Parse a FEN string and create a Board from it
///
/// Returns (Board, turn) tuple on success
pub fn fen_to_board(fen: &str) -> Result<(Board, Color), FenError> {
    let parts: Vec<&str> = fen.split_whitespace().collect();

    if parts.len() != 6 {
        return Err(FenError::InvalidFormat);
    }

    // Parse board section
    let board_str = parts[0];
    let ranks: Vec<&str> = board_str.split('/').collect();

    if ranks.len() != 10 {
        return Err(FenError::InvalidRankCount);
    }

    let mut pieces = HashMap::new();

    for (y, rank_str) in ranks.iter().enumerate() {
        let rank_pieces = parse_rank(rank_str, y)?;
        for (pos, piece) in rank_pieces {
            pieces.insert(pos, piece);
        }
    }

    // Parse turn
    let turn = match parts[1] {
        "w" | "W" | "r" | "R" => Color::Red, // Accept w, W, r, R as Red
        "b" | "B" => Color::Black,
        _ => return Err(FenError::InvalidTurn),
    };

    // Parts 2 and 3 are always "-" for Chinese Chess (no castling, no en passant)
    // We don't need to validate them

    // Parse move counts (optional validation)
    if parts[4].parse::<u32>().is_err() {
        return Err(FenError::InvalidMoveCount);
    }
    if parts[5].parse::<u32>().is_err() {
        return Err(FenError::InvalidMoveCount);
    }

    let board = Board::from_pieces(pieces);

    Ok((board, turn))
}

/// Convert a Board position to FEN string format
///
/// Arguments:
/// - board: The board to convert
/// - turn: Current turn (Red or Black)
/// - half_move_count: Number of half-moves since last capture/pawn move
/// - full_move_count: Current full move number
pub fn board_to_fen(
    board: &Board,
    turn: Color,
    half_move_count: u32,
    full_move_count: u32,
) -> String {
    let mut fen_parts = Vec::new();

    // Build board section
    let mut rank_strings = Vec::new();

    for y in 0..10 {
        let mut rank_str = String::new();
        let mut empty_count = 0;

        for x in 0..9 {
            let pos = Position::from_xy(x, y);
            match board.get(pos) {
                Some(piece) => {
                    // Add empty count before piece
                    if empty_count > 0 {
                        rank_str.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    rank_str.push(piece_to_fen(*piece));
                }
                None => {
                    empty_count += 1;
                }
            }
        }

        // Add trailing empty count
        if empty_count > 0 {
            rank_str.push_str(&empty_count.to_string());
        }

        rank_strings.push(rank_str);
    }

    fen_parts.push(rank_strings.join("/"));

    // Turn (always use 'w' for Red, 'b' for Black)
    fen_parts.push(if turn == Color::Red {
        "w".to_string()
    } else {
        "b".to_string()
    });

    // Chinese Chess doesn't have castling or en passant
    fen_parts.push("-".to_string());
    fen_parts.push("-".to_string());

    // Move counts
    fen_parts.push(half_move_count.to_string());
    fen_parts.push(full_move_count.to_string());

    fen_parts.join(" ")
}

/// Parse FEN with moves format and create a Game
///
/// Accepts two formats:
/// 1. UCCI format: `position fen <fen_string> moves <move1> <move2> ...`
/// 2. Simplified: `<fen_string> moves <move1> <move2> ...`
///
/// Moves are in ICCS format (e.g., "b2c5", "h3e3")
pub fn fen_with_moves_to_game(input: &str) -> Result<crate::game::Game, FenError> {
    // Remove "position" prefix if present
    let input = input.strip_prefix("position ").unwrap_or(input);

    // Split by whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(FenError::InvalidFormat);
    }

    // Find "fen" keyword and extract FEN string
    let fen_start = if parts[0] == "fen" { 1 } else { 0 };

    // FEN string has 6 parts: board turn - - half_move full_move
    if parts.len() < fen_start + 6 {
        return Err(FenError::InvalidFormat);
    }

    let fen_string = parts[fen_start..fen_start + 6].join(" ");

    // Find "moves" keyword
    let moves_start = fen_start + 6;
    if moves_start >= parts.len() || parts[moves_start] != "moves" {
        return Err(FenError::MissingMovesKeyword);
    }

    // Extract move strings
    let move_strings: Vec<&str> = parts[moves_start + 1..].to_vec();
    if move_strings.is_empty() {
        return Err(FenError::EmptyMovesList);
    }

    // Create game
    let mut game = crate::game::Game::from_fen(&fen_string)?;

    // Apply each move
    for mv_str in move_strings {
        // Parse ICCS move
        let (from, to) = crate::notation::iccs::iccs_to_move(mv_str)
            .ok_or_else(|| FenError::InvalidMoveInHistory(mv_str.to_string()))?;

        // Apply move
        game.make_move(from, to)
            .map_err(|_| FenError::InvalidMoveInHistory(mv_str.to_string()))?;
    }

    Ok(game)
}

// TODO: Add from_fen and to_fen functions in subsequent tasks

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_position() {
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
        let result = fen_to_board(fen);
        assert!(result.is_ok(), "Should parse initial position");

        let (board, turn) = result.unwrap();
        assert_eq!(turn, Color::Red);

        // Check red general at (4, 9)
        let red_general = board.get(Position::from_xy(4, 9));
        assert!(red_general.is_some());
        assert_eq!(red_general.unwrap().piece_type, PieceType::General);
        assert_eq!(red_general.unwrap().color, Color::Red);

        // Check black general at (4, 0)
        let black_general = board.get(Position::from_xy(4, 0));
        assert!(black_general.is_some());
        assert_eq!(black_general.unwrap().piece_type, PieceType::General);
        assert_eq!(black_general.unwrap().color, Color::Black);
    }

    #[test]
    fn test_parse_piece_characters() {
        assert_eq!(
            parse_piece('K'),
            Some(Piece::new(PieceType::General, Color::Red))
        );
        assert_eq!(
            parse_piece('k'),
            Some(Piece::new(PieceType::General, Color::Black))
        );
        assert_eq!(
            parse_piece('R'),
            Some(Piece::new(PieceType::Chariot, Color::Red))
        );
        assert_eq!(
            parse_piece('r'),
            Some(Piece::new(PieceType::Chariot, Color::Black))
        );
        assert_eq!(
            parse_piece('C'),
            Some(Piece::new(PieceType::Cannon, Color::Red))
        );
        assert_eq!(
            parse_piece('c'),
            Some(Piece::new(PieceType::Cannon, Color::Black))
        );
        assert_eq!(parse_piece('X'), None);
    }

    #[test]
    fn test_fen_invalid_format() {
        let result = fen_to_board("invalid");
        assert!(matches!(result, Err(FenError::InvalidFormat)));
    }

    #[test]
    fn test_fen_invalid_turn() {
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR x - - 0 1";
        let result = fen_to_board(fen);
        assert!(matches!(result, Err(FenError::InvalidTurn)));
    }

    #[test]
    fn test_board_to_fen_initial_position() {
        let board = Board::new();
        let fen = board_to_fen(&board, Color::Red, 0, 1);

        // The FEN should match the standard initial position
        assert_eq!(
            fen,
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1"
        );
    }

    #[test]
    fn test_fen_roundtrip() {
        let original_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
        let (board, turn) = fen_to_board(original_fen).unwrap();
        let reconstructed_fen = board_to_fen(&board, turn, 0, 1);

        assert_eq!(original_fen, reconstructed_fen);
    }

    #[test]
    fn test_piece_to_fen_characters() {
        let red_k = piece_to_fen(Piece::new(PieceType::General, Color::Red));
        assert_eq!(red_k, 'K');

        let black_k = piece_to_fen(Piece::new(PieceType::General, Color::Black));
        assert_eq!(black_k, 'k');

        let red_c = piece_to_fen(Piece::new(PieceType::Cannon, Color::Red));
        assert_eq!(red_c, 'C');
    }

    #[test]
    fn test_board_to_fen_custom_position() {
        // Create a simple position: just the two generals
        let mut pieces = HashMap::new();
        pieces.insert(
            Position::from_xy(4, 9),
            Piece::new(PieceType::General, Color::Red),
        );
        pieces.insert(
            Position::from_xy(4, 0),
            Piece::new(PieceType::General, Color::Black),
        );

        let board = Board::from_pieces(pieces);
        let fen = board_to_fen(&board, Color::Red, 0, 1);

        // Expected: 9/9/9/9/9/9/9/9/9/4K4 w - - 0 1
        // But we need to check if it matches the correct format
        assert!(
            fen.contains("4K4"),
            "FEN should contain '4K4' for red general"
        );
        assert!(
            fen.contains("4k4"),
            "FEN should contain '4k4' for black general"
        );
    }

    #[test]
    fn test_parse_fen_with_moves_simple() {
        // Start position, one move (soldier from a6 to a5)
        let input =
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1 moves a6a5";
        let result = fen_with_moves_to_game(input);
        assert!(result.is_ok(), "Should parse FEN with moves");

        let game = result.unwrap();
        // Move should have been applied
        assert_eq!(game.turn(), Color::Black);
        assert_eq!(game.get_moves().len(), 1);
    }

    #[test]
    fn test_parse_fen_with_moves_ucci_format() {
        // Full UCCI format with "position fen" prefix
        let input = "position fen rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1 moves a6a5";
        let result = fen_with_moves_to_game(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_fen_with_moves_invalid_iccs() {
        let input =
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1 moves invalid";
        let result = fen_with_moves_to_game(input);
        assert!(matches!(result, Err(FenError::InvalidMoveInHistory(_))));
    }

    #[test]
    fn test_parse_fen_with_moves_missing_moves_keyword() {
        let input = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
        let result = fen_with_moves_to_game(input);
        assert!(matches!(result, Err(FenError::MissingMovesKeyword)));
    }
}
