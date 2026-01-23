//! World XiangQi Federation (WXF) notation
//!
//! Format: "C2.5" or "H2+3" (piece letter + file + direction + destination)
//!
//! In WXF notation:
//! - Piece letters: K (King/General), R (Rook/Chariot), H (Horse),
//!                 E (Elephant), A (Advisor), C (Cannon), P (Pawn/Soldier)
//! - Files are numbered 1-9 from each player's perspective
//! - Red: files numbered right-to-left (from Red's view)
//! - Black: files numbered left-to-right (from Red's view)
//! - Direction: + (forward), - (backward), . (horizontal)
//! - For horizontal moves: destination is file number (e.g., C2.5)
//! - For forward/backward moves: destination is number of steps (e.g., H2+3)

use crate::types::{Piece, PieceType, Position};
use super::chinese::{MovementDirection, get_movement_direction, position_to_file_number};

/// Convert a piece type to its WXF letter representation
///
/// # Examples
/// ```
/// use cn_chess_tui::{types::PieceType, notation::wxf::piece_to_wxf_letter};
///
/// assert_eq!(piece_to_wxf_letter(PieceType::General), "K");
/// assert_eq!(piece_to_wxf_letter(PieceType::Chariot), "R");
/// assert_eq!(piece_to_wxf_letter(PieceType::Cannon), "C");
/// assert_eq!(piece_to_wxf_letter(PieceType::Soldier), "P");
/// ```
pub fn piece_to_wxf_letter(piece_type: PieceType) -> &'static str {
    match piece_type {
        PieceType::General => "K",   // King
        PieceType::Advisor => "A",
        PieceType::Elephant => "E",
        PieceType::Horse => "H",
        PieceType::Chariot => "R",   // Rook
        PieceType::Cannon => "C",
        PieceType::Soldier => "P",   // Pawn
    }
}

/// Convert movement direction to WXF symbol
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::wxf::direction_to_wxf;
/// use cn_chess_tui::notation::chinese::MovementDirection;
///
/// assert_eq!(direction_to_wxf(MovementDirection::Forward), "+");
/// assert_eq!(direction_to_wxf(MovementDirection::Backward), "-");
/// assert_eq!(direction_to_wxf(MovementDirection::Horizontal), ".");
/// ```
pub fn direction_to_wxf(dir: MovementDirection) -> &'static str {
    match dir {
        MovementDirection::Forward => "+",
        MovementDirection::Backward => "-",
        MovementDirection::Horizontal => ".",
    }
}

/// Convert a WXF letter to a piece type
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::wxf::wxf_letter_to_piece_type;
/// use cn_chess_tui::types::PieceType;
///
/// assert_eq!(wxf_letter_to_piece_type("K"), Some(PieceType::General));
/// assert_eq!(wxf_letter_to_piece_type("C"), Some(PieceType::Cannon));
/// assert_eq!(wxf_letter_to_piece_type("X"), None);
/// ```
pub fn wxf_letter_to_piece_type(letter: &str) -> Option<PieceType> {
    match letter {
        "K" => Some(PieceType::General),
        "A" => Some(PieceType::Advisor),
        "E" => Some(PieceType::Elephant),
        "H" => Some(PieceType::Horse),
        "R" => Some(PieceType::Chariot),
        "C" => Some(PieceType::Cannon),
        "P" => Some(PieceType::Soldier),
        _ => None,
    }
}

/// Convert a WXF direction symbol to MovementDirection
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::wxf::wxf_symbol_to_direction;
/// use cn_chess_tui::notation::chinese::MovementDirection;
///
/// assert_eq!(wxf_symbol_to_direction("+"), Some(MovementDirection::Forward));
/// assert_eq!(wxf_symbol_to_direction("-"), Some(MovementDirection::Backward));
/// assert_eq!(wxf_symbol_to_direction("."), Some(MovementDirection::Horizontal));
/// assert_eq!(wxf_symbol_to_direction("x"), None);
/// ```
pub fn wxf_symbol_to_direction(symbol: &str) -> Option<MovementDirection> {
    match symbol {
        "+" => Some(MovementDirection::Forward),
        "-" => Some(MovementDirection::Backward),
        "." => Some(MovementDirection::Horizontal),
        _ => None,
    }
}

/// Convert a move to WXF notation
///
/// Format: "C2.5" or "H2+3" (Piece + FromFile + Direction + ToFile/Steps)
///
/// # Examples
/// ```
/// use cn_chess_tui::{
///     types::{Color, Piece, PieceType, Position},
///     notation::wxf::move_to_wxf
/// };
///
/// // C2.5: Cannon from file 2 horizontally to file 5
/// let piece = Piece::new(PieceType::Cannon, Color::Red);
/// let from = Position::from_xy(7, 7); // File 2 for Red (9-7=2)
/// let to = Position::from_xy(4, 7);   // File 5 for Red (9-4=5)
/// assert_eq!(move_to_wxf(piece, from, to), "C2.5");
///
/// // H2+3: Horse from file 2 forward 3 steps
/// let piece = Piece::new(PieceType::Horse, Color::Red);
/// let from = Position::from_xy(7, 9); // File 2
/// let to = Position::from_xy(7, 6);   // Forward 3 steps
/// assert_eq!(move_to_wxf(piece, from, to), "H2+3");
///
/// // C5-2: Cannon from file 5 backward 2 steps
/// let piece = Piece::new(PieceType::Cannon, Color::Red);
/// let from = Position::from_xy(4, 5); // File 5
/// let to = Position::from_xy(4, 7);   // Backward 2 steps
/// assert_eq!(move_to_wxf(piece, from, to), "C5-2");
/// ```
pub fn move_to_wxf(piece: Piece, from: Position, to: Position) -> String {
    let piece_letter = piece_to_wxf_letter(piece.piece_type);
    let from_file = position_to_file_number(from, piece.color);
    let direction = get_movement_direction(from, to, piece.color);
    let dir_symbol = direction_to_wxf(direction);

    let destination = if direction == MovementDirection::Horizontal {
        // For horizontal moves, use destination file number
        position_to_file_number(to, piece.color)
    } else {
        // For forward/backward moves, use number of steps
        from.y.abs_diff(to.y)
    };

    format!("{}{}{}{}", piece_letter, from_file, dir_symbol, destination)
}

/// Parse a WXF move string into its components
///
/// Returns: Some((piece_type, from_file, direction, destination))
/// Returns None if the string is invalid
///
/// # Examples
/// ```
/// use cn_chess_tui::{notation::wxf::parse_wxf_move, types::PieceType};
/// use cn_chess_tui::notation::chinese::MovementDirection;
///
/// // Parse horizontal move: C2.5
/// let result = parse_wxf_move("C2.5");
/// assert_eq!(result, Some((PieceType::Cannon, 2, MovementDirection::Horizontal, 5)));
///
/// // Parse forward move: H2+3
/// let result = parse_wxf_move("H2+3");
/// assert_eq!(result, Some((PieceType::Horse, 2, MovementDirection::Forward, 3)));
///
/// // Parse backward move: C5-2
/// let result = parse_wxf_move("C5-2");
/// assert_eq!(result, Some((PieceType::Cannon, 5, MovementDirection::Backward, 2)));
///
/// // Invalid format
/// assert_eq!(parse_wxf_move("X2.5"), None);
/// ```
pub fn parse_wxf_move(s: &str) -> Option<(PieceType, usize, MovementDirection, usize)> {
    if s.len() < 4 {
        return None;
    }

    let chars: Vec<char> = s.chars().collect();

    // Extract piece letter (first character)
    let piece_letter = chars[0].to_string();
    let piece_type = wxf_letter_to_piece_type(&piece_letter)?;

    // Find direction symbol (+, -, or .)
    let mut dir_idx = None;
    for (i, &c) in chars.iter().enumerate() {
        if c == '+' || c == '-' || c == '.' {
            dir_idx = Some(i);
            break;
        }
    }

    let dir_idx = dir_idx?;

    // Extract from_file (between piece letter and direction)
    let from_file_str: String = chars[1..dir_idx].iter().collect();
    let from_file: usize = from_file_str.parse().ok()?;
    if from_file < 1 || from_file > 9 {
        return None;
    }

    // Extract direction symbol
    let dir_symbol = chars[dir_idx].to_string();
    let direction = wxf_symbol_to_direction(&dir_symbol)?;

    // Extract destination (after direction)
    let dest_str: String = chars[dir_idx + 1..].iter().collect();
    let destination: usize = dest_str.parse().ok()?;
    if destination < 1 || destination > 9 {
        return None;
    }

    Some((piece_type, from_file, direction, destination))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Color;

    #[test]
    fn test_piece_to_wxf_letter() {
        assert_eq!(piece_to_wxf_letter(PieceType::General), "K");
        assert_eq!(piece_to_wxf_letter(PieceType::Advisor), "A");
        assert_eq!(piece_to_wxf_letter(PieceType::Elephant), "E");
        assert_eq!(piece_to_wxf_letter(PieceType::Horse), "H");
        assert_eq!(piece_to_wxf_letter(PieceType::Chariot), "R");
        assert_eq!(piece_to_wxf_letter(PieceType::Cannon), "C");
        assert_eq!(piece_to_wxf_letter(PieceType::Soldier), "P");
    }

    #[test]
    fn test_direction_to_wxf() {
        assert_eq!(direction_to_wxf(MovementDirection::Forward), "+");
        assert_eq!(direction_to_wxf(MovementDirection::Backward), "-");
        assert_eq!(direction_to_wxf(MovementDirection::Horizontal), ".");
    }

    #[test]
    fn test_wxf_letter_to_piece_type() {
        assert_eq!(wxf_letter_to_piece_type("K"), Some(PieceType::General));
        assert_eq!(wxf_letter_to_piece_type("A"), Some(PieceType::Advisor));
        assert_eq!(wxf_letter_to_piece_type("E"), Some(PieceType::Elephant));
        assert_eq!(wxf_letter_to_piece_type("H"), Some(PieceType::Horse));
        assert_eq!(wxf_letter_to_piece_type("R"), Some(PieceType::Chariot));
        assert_eq!(wxf_letter_to_piece_type("C"), Some(PieceType::Cannon));
        assert_eq!(wxf_letter_to_piece_type("P"), Some(PieceType::Soldier));
        assert_eq!(wxf_letter_to_piece_type("X"), None);
    }

    #[test]
    fn test_move_to_wxf_horizontal() {
        // C2.5: Cannon from file 2 horizontally to file 5
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(7, 7); // File 2 for Red (9-7=2)
        let to = Position::from_xy(4, 7);   // File 5 for Red (9-4=5)
        assert_eq!(move_to_wxf(piece, from, to), "C2.5");

        // R5.3: Chariot from file 5 horizontally to file 3
        let piece = Piece::new(PieceType::Chariot, Color::Red);
        let from = Position::from_xy(4, 5); // File 5 for Red (9-4=5)
        let to = Position::from_xy(6, 5);   // File 3 for Red (9-6=3)
        assert_eq!(move_to_wxf(piece, from, to), "R5.3");
    }

    #[test]
    fn test_move_to_wxf_forward() {
        // H2+3: Horse from file 2 forward 3 steps
        let piece = Piece::new(PieceType::Horse, Color::Red);
        let from = Position::from_xy(7, 9); // File 2
        let to = Position::from_xy(7, 6);   // Forward 3 steps
        assert_eq!(move_to_wxf(piece, from, to), "H2+3");

        // P5+1: Soldier from file 5 forward 1 step
        let piece = Piece::new(PieceType::Soldier, Color::Red);
        let from = Position::from_xy(4, 6); // File 5
        let to = Position::from_xy(4, 5);   // Forward 1 step
        assert_eq!(move_to_wxf(piece, from, to), "P5+1");
    }

    #[test]
    fn test_move_to_wxf_backward() {
        // C5-2: Cannon from file 5 backward 2 steps
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(4, 5); // File 5
        let to = Position::from_xy(4, 7);   // Backward 2 steps
        assert_eq!(move_to_wxf(piece, from, to), "C5-2");

        // E7-2: Elephant from file 7 backward 2 steps
        let piece = Piece::new(PieceType::Elephant, Color::Red);
        let from = Position::from_xy(2, 5); // File 7
        let to = Position::from_xy(2, 7);   // Backward 2 steps
        assert_eq!(move_to_wxf(piece, from, to), "E7-2");
    }

    #[test]
    fn test_move_to_wxf_black() {
        // Black pieces use the same format
        // C5.6: Black cannon from file 5 horizontally to file 6
        let piece = Piece::new(PieceType::Cannon, Color::Black);
        let from = Position::from_xy(4, 2); // File 5 for Black
        let to = Position::from_xy(5, 2);   // File 6 for Black
        assert_eq!(move_to_wxf(piece, from, to), "C5.6");

        // H3+2: Black horse from file 3 forward 2 steps
        let piece = Piece::new(PieceType::Horse, Color::Black);
        let from = Position::from_xy(2, 0); // File 3
        let to = Position::from_xy(2, 2);   // Forward 2 steps
        assert_eq!(move_to_wxf(piece, from, to), "H3+2");
    }

    #[test]
    fn test_parse_wxf_move() {
        // Parse horizontal move: C2.5
        let result = parse_wxf_move("C2.5");
        assert_eq!(result, Some((PieceType::Cannon, 2, MovementDirection::Horizontal, 5)));

        // Parse forward move: H2+3
        let result = parse_wxf_move("H2+3");
        assert_eq!(result, Some((PieceType::Horse, 2, MovementDirection::Forward, 3)));

        // Parse backward move: C5-2
        let result = parse_wxf_move("C5-2");
        assert_eq!(result, Some((PieceType::Cannon, 5, MovementDirection::Backward, 2)));

        // Parse all piece types
        assert_eq!(
            parse_wxf_move("K1.2"),
            Some((PieceType::General, 1, MovementDirection::Horizontal, 2))
        );
        assert_eq!(
            parse_wxf_move("A3+1"),
            Some((PieceType::Advisor, 3, MovementDirection::Forward, 1))
        );
        assert_eq!(
            parse_wxf_move("E7-2"),
            Some((PieceType::Elephant, 7, MovementDirection::Backward, 2))
        );
        assert_eq!(
            parse_wxf_move("R9.1"),
            Some((PieceType::Chariot, 9, MovementDirection::Horizontal, 1))
        );
        assert_eq!(
            parse_wxf_move("P4+1"),
            Some((PieceType::Soldier, 4, MovementDirection::Forward, 1))
        );

        // Invalid formats
        assert_eq!(parse_wxf_move(""), None);
        assert_eq!(parse_wxf_move("X2.5"), None); // Invalid piece
        assert_eq!(parse_wxf_move("C2"), None);   // Missing destination
        assert_eq!(parse_wxf_move("C2.5.3"), None); // Too many parts
        assert_eq!(parse_wxf_move("C0.5"), None);  // Invalid file number
        assert_eq!(parse_wxf_move("C10.5"), None); // Invalid file number
    }

    #[test]
    fn test_roundtrip_wxf() {
        // Test that we can parse what we generate
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(7, 7);
        let to = Position::from_xy(4, 7);
        let wxf = move_to_wxf(piece, from, to);
        assert_eq!(wxf, "C2.5");

        let parsed = parse_wxf_move(&wxf);
        assert_eq!(
            parsed,
            Some((PieceType::Cannon, 2, MovementDirection::Horizontal, 5))
        );

        // Test forward move
        let piece = Piece::new(PieceType::Horse, Color::Red);
        let from = Position::from_xy(7, 9);
        let to = Position::from_xy(7, 6);
        let wxf = move_to_wxf(piece, from, to);
        assert_eq!(wxf, "H2+3");

        let parsed = parse_wxf_move(&wxf);
        assert_eq!(parsed, Some((PieceType::Horse, 2, MovementDirection::Forward, 3)));

        // Test backward move
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(4, 5);
        let to = Position::from_xy(4, 7);
        let wxf = move_to_wxf(piece, from, to);
        assert_eq!(wxf, "C5-2");

        let parsed = parse_wxf_move(&wxf);
        assert_eq!(parsed, Some((PieceType::Cannon, 5, MovementDirection::Backward, 2)));
    }
}
