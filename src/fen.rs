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

use crate::types::{Color, Piece, PieceType, Position};
use crate::board::Board;
use std::collections::HashMap;

/// Errors that can occur during FEN parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    InvalidFormat,
    InvalidBoardSection,
    InvalidPiece(char),
    InvalidRankCount,
    InvalidFileCount,
    InvalidTurn,
    InvalidMoveCount,
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
fn piece_to_fen(piece: Piece) -> char {
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
        assert_eq!(parse_piece('K'), Some(Piece::new(PieceType::General, Color::Red)));
        assert_eq!(parse_piece('k'), Some(Piece::new(PieceType::General, Color::Black)));
        assert_eq!(parse_piece('R'), Some(Piece::new(PieceType::Chariot, Color::Red)));
        assert_eq!(parse_piece('r'), Some(Piece::new(PieceType::Chariot, Color::Black)));
        assert_eq!(parse_piece('C'), Some(Piece::new(PieceType::Cannon, Color::Red)));
        assert_eq!(parse_piece('c'), Some(Piece::new(PieceType::Cannon, Color::Black)));
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
}
