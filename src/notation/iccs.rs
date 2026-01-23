//! ICCS (Internet Chinese Chess Server) coordinate notation
//!
//! Format: "H2-E2" or "h2e2" (from-position to-position)
//! Files: a-i (left to right from Red's perspective)
//! Ranks: 0-9 (bottom to top, Red's home is rank 9)

use crate::types::Position;

/// Convert a Position to ICCS coordinate string
///
/// Examples:
/// - (0, 0) -> "a0" (Black's left chariot corner)
/// - (4, 9) -> "e9" (Red's general position)
/// - (8, 4) -> "i4" (Right side, middle rank)
#[allow(dead_code)]
pub fn position_to_iccs(pos: Position) -> String {
    let file_char = (b'a' + pos.x as u8) as char;
    format!("{}{}", file_char, pos.y)
}

/// Parse ICCS coordinate string to Position
///
/// Returns None if the coordinate is invalid
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::iccs::iccs_to_position;
/// use cn_chess_tui::types::Position;
///
/// assert_eq!(iccs_to_position("a0"), Some(Position::from_xy(0, 0)));
/// assert_eq!(iccs_to_position("e9"), Some(Position::from_xy(4, 9)));
/// assert_eq!(iccs_to_position("z9"), None); // Invalid file
/// ```
#[allow(dead_code)]
pub fn iccs_to_position(s: &str) -> Option<Position> {
    let mut chars = s.chars();

    let file_char = chars.next()?;
    let rank_str: String = chars.collect();

    // File must be a-i (ASCII 97-105)
    if !('a'..='i').contains(&file_char) {
        return None;
    }

    let x = (file_char as u8 - b'a') as usize;

    // Rank must be 0-9
    let y = rank_str.parse::<usize>().ok()?;
    if y > 9 {
        return None;
    }

    Some(Position::from_xy(x, y))
}

/// Convert a move to ICCS format (compact, no dash)
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::iccs::move_to_iccs;
/// use cn_chess_tui::types::Position;
///
/// let from = Position::from_xy(7, 7); // h7 in ICCS
/// let to = Position::from_xy(4, 7);   // e7 in ICCS
/// assert_eq!(move_to_iccs(from, to), "h7e7");
/// ```
#[allow(dead_code)]
pub fn move_to_iccs(from: Position, to: Position) -> String {
    format!("{}{}", position_to_iccs(from), position_to_iccs(to))
}

/// Parse ICCS move string to (from, to) positions
///
/// Accepts both "h2e2" and "H2-E2" formats
#[allow(dead_code)]
pub fn iccs_to_move(s: &str) -> Option<(Position, Position)> {
    // Remove dash if present and convert to lowercase
    let s = s.replace('-', "").to_lowercase();

    if s.len() != 4 {
        return None;
    }

    let from = iccs_to_position(&s[0..2])?;
    let to = iccs_to_position(&s[2..4])?;

    Some((from, to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_iccs_corners() {
        assert_eq!(position_to_iccs(Position::from_xy(0, 0)), "a0");
        assert_eq!(position_to_iccs(Position::from_xy(8, 0)), "i0");
        assert_eq!(position_to_iccs(Position::from_xy(0, 9)), "a9");
        assert_eq!(position_to_iccs(Position::from_xy(8, 9)), "i9");
    }

    #[test]
    fn test_iccs_to_position_valid() {
        assert_eq!(iccs_to_position("a0"), Some(Position::from_xy(0, 0)));
        assert_eq!(iccs_to_position("e5"), Some(Position::from_xy(4, 5)));
        assert_eq!(iccs_to_position("i9"), Some(Position::from_xy(8, 9)));
    }

    #[test]
    fn test_iccs_to_position_invalid() {
        assert_eq!(iccs_to_position("j0"), None); // File beyond i
        assert_eq!(iccs_to_position("a10"), None); // Rank beyond 9
        assert_eq!(iccs_to_position(""), None); // Empty
        assert_eq!(iccs_to_position("abc"), None); // Invalid format
    }

    #[test]
    fn test_move_to_iccs() {
        let from = Position::from_xy(7, 2);
        let to = Position::from_xy(4, 2);
        assert_eq!(move_to_iccs(from, to), "h2e2");
    }

    #[test]
    fn test_iccs_to_move_formats() {
        // Compact format
        assert_eq!(
            iccs_to_move("h2e2"),
            Some((Position::from_xy(7, 2), Position::from_xy(4, 2)))
        );

        // With dash
        assert_eq!(
            iccs_to_move("H2-E2"),
            Some((Position::from_xy(7, 2), Position::from_xy(4, 2)))
        );
    }
}
