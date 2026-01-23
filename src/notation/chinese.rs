//! Chinese move notation (traditional format)
//!
//! Format: "炮二平五" (Cannon 2 horizontal 5)
//!
//! In Chinese notation:
//! - Files are numbered 1-9 from each player's perspective
//! - Red: files numbered right-to-left (from Red's view)
//! - Black: files numbered left-to-right (from Red's view)
//! - Directions: 进 (forward), 退 (backward), 平 (horizontal/same rank)
//! - Uses Chinese numerals: 一二三四五六七八九

use crate::types::{Color, Piece, Position};

/// Direction of piece movement in Chinese notation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MovementDirection {
    /// 进 - forward (toward opponent's side)
    Forward,
    /// 退 - backward (toward own side)
    Backward,
    /// 平 - horizontal (same rank)
    Horizontal,
}

/// Convert a position to file number (1-9) from a player's perspective
///
/// # Examples
/// ```
/// use cn_chess_tui::{types::{Color, Position}, notation::chinese::position_to_file_number};
///
/// // Red's files are numbered right-to-left
/// assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Red), 1);
/// assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Red), 5);
/// assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Red), 9);
///
/// // Black's files are numbered left-to-right (from Red's view)
/// assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Black), 1);
/// assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Black), 5);
/// assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Black), 9);
/// ```
#[allow(dead_code)]
pub fn position_to_file_number(pos: Position, color: Color) -> usize {
    match color {
        Color::Red => 9 - pos.x,  // Right-to-left for Red
        Color::Black => pos.x + 1, // Left-to-right for Black
    }
}

/// Convert file number (1-9) to Chinese numeral
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::chinese::file_number_to_chinese;
///
/// assert_eq!(file_number_to_chinese(1), "一");
/// assert_eq!(file_number_to_chinese(5), "五");
/// assert_eq!(file_number_to_chinese(9), "九");
/// ```
#[allow(dead_code)]
pub fn file_number_to_chinese(n: usize) -> &'static str {
    match n {
        1 => "一",
        2 => "二",
        3 => "三",
        4 => "四",
        5 => "五",
        6 => "六",
        7 => "七",
        8 => "八",
        9 => "九",
        _ => "?",
    }
}

/// Determine the direction of movement for Chinese notation
///
/// # Examples
/// ```
/// use cn_chess_tui::{types::{Color, Position}, notation::chinese::{get_movement_direction, MovementDirection}};
///
/// // Horizontal movement (same rank)
/// let from = Position::from_xy(6, 7);
/// let to = Position::from_xy(4, 7);
/// assert_eq!(get_movement_direction(from, to, Color::Red), MovementDirection::Horizontal);
///
/// // Forward movement (Red moves toward smaller y)
/// let from = Position::from_xy(6, 9);
/// let to = Position::from_xy(6, 7);
/// assert_eq!(get_movement_direction(from, to, Color::Red), MovementDirection::Forward);
///
/// // Forward movement (Black moves toward larger y)
/// let from = Position::from_xy(3, 1);
/// let to = Position::from_xy(4, 2);
/// assert_eq!(get_movement_direction(from, to, Color::Black), MovementDirection::Forward);
/// ```
#[allow(dead_code)]
pub fn get_movement_direction(from: Position, to: Position, color: Color) -> MovementDirection {
    if from.y == to.y {
        MovementDirection::Horizontal
    } else {
        let is_forward = match color {
            Color::Red => to.y < from.y, // Red moves up (decreasing y)
            Color::Black => to.y > from.y, // Black moves down (increasing y)
        };
        if is_forward {
            MovementDirection::Forward
        } else {
            MovementDirection::Backward
        }
    }
}

/// Convert movement direction to Chinese character
///
/// # Examples
/// ```
/// use cn_chess_tui::notation::chinese::{direction_to_chinese, MovementDirection};
///
/// assert_eq!(direction_to_chinese(MovementDirection::Forward), "进");
/// assert_eq!(direction_to_chinese(MovementDirection::Backward), "退");
/// assert_eq!(direction_to_chinese(MovementDirection::Horizontal), "平");
/// ```
#[allow(dead_code)]
pub fn direction_to_chinese(dir: MovementDirection) -> &'static str {
    match dir {
        MovementDirection::Forward => "进",
        MovementDirection::Backward => "退",
        MovementDirection::Horizontal => "平",
    }
}

/// Convert a piece to its Chinese name
///
/// # Examples
/// ```
/// use cn_chess_tui::{types::{Color, Piece, PieceType}, notation::chinese::piece_to_chinese};
///
/// // Red pieces
/// assert_eq!(piece_to_chinese(Piece::new(PieceType::Cannon, Color::Red)), "炮");
/// assert_eq!(piece_to_chinese(Piece::new(PieceType::General, Color::Red)), "帅");
///
/// // Black pieces
/// assert_eq!(piece_to_chinese(Piece::new(PieceType::Cannon, Color::Black)), "炮");
/// assert_eq!(piece_to_chinese(Piece::new(PieceType::General, Color::Black)), "将");
/// ```
#[allow(dead_code)]
pub fn piece_to_chinese(piece: Piece) -> &'static str {
    match (piece.color, piece.piece_type) {
        (Color::Red, crate::types::PieceType::General) => "帅",
        (Color::Red, crate::types::PieceType::Advisor) => "仕",
        (Color::Red, crate::types::PieceType::Elephant) => "相",
        (Color::Red, crate::types::PieceType::Horse) => "马",
        (Color::Red, crate::types::PieceType::Chariot) => "车",
        (Color::Red, crate::types::PieceType::Cannon) => "炮",
        (Color::Red, crate::types::PieceType::Soldier) => "兵",
        (Color::Black, crate::types::PieceType::General) => "将",
        (Color::Black, crate::types::PieceType::Advisor) => "士",
        (Color::Black, crate::types::PieceType::Elephant) => "象",
        (Color::Black, crate::types::PieceType::Horse) => "马",
        (Color::Black, crate::types::PieceType::Chariot) => "车",
        (Color::Black, crate::types::PieceType::Cannon) => "炮",
        (Color::Black, crate::types::PieceType::Soldier) => "卒",
    }
}

/// Convert a move to Chinese notation
///
/// Format: "炮二平五" (Piece + FromFile + Direction + ToFile)
///
/// # Examples
/// ```
/// use cn_chess_tui::{
///     types::{Color, Piece, PieceType, Position},
///     notation::chinese::move_to_chinese
/// };
///
/// // 炮二平五: Cannon from file 2 horizontally to file 5
/// let piece = Piece::new(PieceType::Cannon, Color::Red);
/// let from = Position::from_xy(6, 7); // File 2 (二) for Red
/// let to = Position::from_xy(4, 7);   // File 5 (五) for Red
/// assert_eq!(move_to_chinese(piece, from, to), "炮二平五");
/// ```
#[allow(dead_code)]
pub fn move_to_chinese(piece: Piece, from: Position, to: Position) -> String {
    let piece_name = piece_to_chinese(piece);
    let from_file = position_to_file_number(from, piece.color);
    let from_chinese = file_number_to_chinese(from_file);
    let direction = get_movement_direction(from, to, piece.color);
    let dir_chinese = direction_to_chinese(direction);

    let to_chinese = if direction == MovementDirection::Horizontal {
        // For horizontal moves, use destination file number
        let to_file = position_to_file_number(to, piece.color);
        file_number_to_chinese(to_file)
    } else {
        // For forward/backward moves, use number of steps
        let steps = from.y.abs_diff(to.y);
        file_number_to_chinese(steps)
    };

    format!("{}{}{}{}", piece_name, from_chinese, dir_chinese, to_chinese)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PieceType;

    #[test]
    fn test_red_file_number() {
        // Red's files are numbered 1-9 from RIGHT to LEFT
        assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Red), 1);
        assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Red), 5);
        assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Red), 9);
    }

    #[test]
    fn test_black_file_number() {
        // Black's files are numbered 1-9 from LEFT to RIGHT
        assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Black), 1);
        assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Black), 5);
        assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Black), 9);
    }

    #[test]
    fn test_file_number_to_chinese() {
        assert_eq!(file_number_to_chinese(1), "一");
        assert_eq!(file_number_to_chinese(2), "二");
        assert_eq!(file_number_to_chinese(3), "三");
        assert_eq!(file_number_to_chinese(4), "四");
        assert_eq!(file_number_to_chinese(5), "五");
        assert_eq!(file_number_to_chinese(6), "六");
        assert_eq!(file_number_to_chinese(7), "七");
        assert_eq!(file_number_to_chinese(8), "八");
        assert_eq!(file_number_to_chinese(9), "九");
    }

    #[test]
    fn test_movement_direction() {
        // Horizontal movement
        let from = Position::from_xy(6, 7);
        let to = Position::from_xy(4, 7);
        assert_eq!(get_movement_direction(from, to, Color::Red), MovementDirection::Horizontal);

        // Forward (Red moves toward smaller y)
        let from = Position::from_xy(6, 9);
        let to = Position::from_xy(6, 7);
        assert_eq!(get_movement_direction(from, to, Color::Red), MovementDirection::Forward);

        // Backward (Red moves toward larger y)
        let from = Position::from_xy(6, 7);
        let to = Position::from_xy(6, 9);
        assert_eq!(get_movement_direction(from, to, Color::Red), MovementDirection::Backward);

        // Forward (Black moves toward larger y)
        let from = Position::from_xy(3, 1);
        let to = Position::from_xy(4, 2);
        assert_eq!(get_movement_direction(from, to, Color::Black), MovementDirection::Forward);

        // Backward (Black moves toward smaller y)
        let from = Position::from_xy(4, 2);
        let to = Position::from_xy(3, 1);
        assert_eq!(get_movement_direction(from, to, Color::Black), MovementDirection::Backward);
    }

    #[test]
    fn test_direction_to_chinese() {
        assert_eq!(direction_to_chinese(MovementDirection::Forward), "进");
        assert_eq!(direction_to_chinese(MovementDirection::Backward), "退");
        assert_eq!(direction_to_chinese(MovementDirection::Horizontal), "平");
    }

    #[test]
    fn test_piece_to_chinese() {
        // Red pieces
        assert_eq!(piece_to_chinese(Piece::new(PieceType::General, Color::Red)), "帅");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Advisor, Color::Red)), "仕");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Elephant, Color::Red)), "相");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Horse, Color::Red)), "马");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Chariot, Color::Red)), "车");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Cannon, Color::Red)), "炮");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Soldier, Color::Red)), "兵");

        // Black pieces
        assert_eq!(piece_to_chinese(Piece::new(PieceType::General, Color::Black)), "将");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Advisor, Color::Black)), "士");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Elephant, Color::Black)), "象");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Horse, Color::Black)), "马");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Chariot, Color::Black)), "车");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Cannon, Color::Black)), "炮");
        assert_eq!(piece_to_chinese(Piece::new(PieceType::Soldier, Color::Black)), "卒");
    }

    #[test]
    fn test_move_to_chinese_horizontal() {
        // 炮二平五: Cannon from file 2 horizontally to file 5
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(7, 7); // File 2 (二) for Red (9-7=2)
        let to = Position::from_xy(4, 7);   // File 5 (五) for Red (9-4=5)
        assert_eq!(move_to_chinese(piece, from, to), "炮二平五");
    }

    #[test]
    fn test_move_to_chinese_forward() {
        // 马二进三: Horse from file 2 forward 3 steps
        let piece = Piece::new(PieceType::Horse, Color::Red);
        let from = Position::from_xy(7, 9); // File 2 (二) (9-7=2)
        let to = Position::from_xy(7, 6);   // Forward 3 steps (same file)
        assert_eq!(move_to_chinese(piece, from, to), "马二进三");
    }

    #[test]
    fn test_move_to_chinese_backward() {
        // 炮五退二: Cannon from file 5 backward 2 steps
        let piece = Piece::new(PieceType::Cannon, Color::Red);
        let from = Position::from_xy(4, 5); // File 5 (五)
        let to = Position::from_xy(4, 7);   // Backward 2 steps
        assert_eq!(move_to_chinese(piece, from, to), "炮五退二");
    }
}
