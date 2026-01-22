use crate::types::{Color, Piece, PieceType, Position};
use std::collections::HashMap;

const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Create a board from a pieces HashMap (for FEN loading)
    pub fn from_pieces(pieces: HashMap<Position, Piece>) -> Self {
        Self { pieces }
    }

    pub fn new() -> Self {
        let mut pieces = HashMap::new();

        // Red pieces (bottom, rows 7-9)
        // Row 9 (back rank for Red)
        pieces.insert(Position::from_xy(0, 9), Piece::red(PieceType::Chariot));
        pieces.insert(Position::from_xy(1, 9), Piece::red(PieceType::Horse));
        pieces.insert(Position::from_xy(2, 9), Piece::red(PieceType::Elephant));
        pieces.insert(Position::from_xy(3, 9), Piece::red(PieceType::Advisor));
        pieces.insert(Position::from_xy(4, 9), Piece::red(PieceType::General));
        pieces.insert(Position::from_xy(5, 9), Piece::red(PieceType::Advisor));
        pieces.insert(Position::from_xy(6, 9), Piece::red(PieceType::Elephant));
        pieces.insert(Position::from_xy(7, 9), Piece::red(PieceType::Horse));
        pieces.insert(Position::from_xy(8, 9), Piece::red(PieceType::Chariot));

        // Red cannons
        pieces.insert(Position::from_xy(1, 7), Piece::red(PieceType::Cannon));
        pieces.insert(Position::from_xy(7, 7), Piece::red(PieceType::Cannon));

        // Red soldiers
        for x in [0, 2, 4, 6, 8] {
            pieces.insert(Position::from_xy(x, 6), Piece::red(PieceType::Soldier));
        }

        // Black pieces (top, rows 0-2)
        // Row 0 (back rank for Black)
        pieces.insert(Position::from_xy(0, 0), Piece::black(PieceType::Chariot));
        pieces.insert(Position::from_xy(1, 0), Piece::black(PieceType::Horse));
        pieces.insert(Position::from_xy(2, 0), Piece::black(PieceType::Elephant));
        pieces.insert(Position::from_xy(3, 0), Piece::black(PieceType::Advisor));
        pieces.insert(Position::from_xy(4, 0), Piece::black(PieceType::General));
        pieces.insert(Position::from_xy(5, 0), Piece::black(PieceType::Advisor));
        pieces.insert(Position::from_xy(6, 0), Piece::black(PieceType::Elephant));
        pieces.insert(Position::from_xy(7, 0), Piece::black(PieceType::Horse));
        pieces.insert(Position::from_xy(8, 0), Piece::black(PieceType::Chariot));

        // Black cannons
        pieces.insert(Position::from_xy(1, 2), Piece::black(PieceType::Cannon));
        pieces.insert(Position::from_xy(7, 2), Piece::black(PieceType::Cannon));

        // Black soldiers
        for x in [0, 2, 4, 6, 8] {
            pieces.insert(Position::from_xy(x, 3), Piece::black(PieceType::Soldier));
        }

        Self { pieces }
    }

    pub fn get(&self, pos: Position) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Piece> {
        self.pieces.get_mut(&pos)
    }

    #[allow(dead_code)]
    pub fn piece_at(&self, x: usize, y: usize) -> Option<&Piece> {
        self.get(Position::from_xy(x, y))
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        !self.pieces.contains_key(&pos)
    }

    pub fn is_empty_xy(&self, x: usize, y: usize) -> bool {
        self.is_empty(Position::from_xy(x, y))
    }

    pub fn place_piece(&mut self, pos: Position, piece: Piece) {
        self.pieces.insert(pos, piece);
    }

    pub fn remove_piece(&mut self, pos: Position) -> Option<Piece> {
        self.pieces.remove(&pos)
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Option<Piece> {
        let piece = self.pieces.remove(&from)?;
        self.pieces.insert(to, piece);
        self.pieces.get(&to).copied()
    }

    pub fn pieces(&self) -> impl Iterator<Item = (Position, Piece)> + '_ {
        self.pieces.iter().map(|(p, piece)| (*p, *piece))
    }

    pub fn pieces_of_color(&self, color: Color) -> impl Iterator<Item = (Position, Piece)> + '_ {
        self.pieces
            .iter()
            .filter(move |(_, p)| p.color == color)
            .map(|(p, piece)| (*p, *piece))
    }

    pub fn find_general(&self, color: Color) -> Option<Position> {
        self.pieces
            .iter()
            .find(|(_, p)| p.color == color && p.piece_type == PieceType::General)
            .map(|(p, _)| *p)
    }

    /// Count pieces between two positions (exclusive) on the same file/rank
    pub fn count_between(&self, from: Position, to: Position) -> usize {
        if !from.on_same_file(to) && !from.on_same_rank(to) {
            return 0;
        }

        let (start, end) = if from.y < to.y || (from.y == to.y && from.x < to.x) {
            (from, to)
        } else {
            (to, from)
        };

        let mut count = 0;

        if from.on_same_file(to) {
            // Same file - count on Y axis
            for y in (start.y + 1)..end.y {
                if self.get(Position::from_xy(start.x, y)).is_some() {
                    count += 1;
                }
            }
        } else {
            // Same rank - count on X axis
            for x in (start.x + 1)..end.x {
                if self.get(Position::from_xy(x, start.y)).is_some() {
                    count += 1;
                }
            }
        }

        count
    }

    /// Check if generals face each other directly with no pieces in between
    pub fn generals_facing(&self) -> bool {
        let red_general = match self.find_general(Color::Red) {
            Some(pos) => pos,
            None => return false,
        };
        let black_general = match self.find_general(Color::Black) {
            Some(pos) => pos,
            None => return false,
        };

        if !red_general.on_same_file(black_general) {
            return false;
        }

        self.count_between(red_general, black_general) == 0
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let general_pos = match self.find_general(color) {
            Some(pos) => pos,
            None => return false, // General captured - game over
        };

        let enemy_color = match color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        // Check if any enemy piece can capture the general
        for (pos, piece) in self.pieces_of_color(enemy_color) {
            if self.is_valid_move(pos, general_pos, piece) {
                return true;
            }
        }

        false
    }

    fn is_valid_move(&self, from: Position, to: Position, piece: Piece) -> bool {
        match piece.piece_type {
            PieceType::General => self.can_general_move(from, to, piece.color),
            PieceType::Advisor => self.can_advisor_move(from, to, piece.color),
            PieceType::Elephant => self.can_elephant_move(from, to, piece.color),
            PieceType::Horse => self.can_horse_move(from, to, piece.color),
            PieceType::Chariot => self.can_chariot_move(from, to, piece.color),
            PieceType::Cannon => self.can_cannon_move(from, to, piece.color),
            PieceType::Soldier => self.can_soldier_move(from, to, piece.color),
        }
    }

    fn can_general_move(&self, from: Position, to: Position, color: Color) -> bool {
        if !to.in_palace(color) {
            return false;
        }
        if from.chebyshev_distance(to) != 1 {
            return false;
        }
        // Must move along file or rank (not diagonal)
        from.on_same_file(to) || from.on_same_rank(to)
    }

    fn can_advisor_move(&self, from: Position, to: Position, color: Color) -> bool {
        if !to.in_palace(color) {
            return false;
        }
        // Diagonal move only
        from.file_distance(to) == 1 && from.rank_distance(to) == 1
    }

    fn can_elephant_move(&self, from: Position, to: Position, color: Color) -> bool {
        // Cannot cross river
        let valid_y = match color {
            Color::Red => to.y >= 5,
            Color::Black => to.y <= 4,
        };
        if !valid_y {
            return false;
        }
        // Diagonal 2 squares
        if from.file_distance(to) != 2 || from.rank_distance(to) != 2 {
            return false;
        }
        // Check blocking eye
        let eye_x = (from.x + to.x) / 2;
        let eye_y = (from.y + to.y) / 2;
        self.is_empty_xy(eye_x, eye_y)
    }

    fn can_horse_move(&self, from: Position, to: Position, _color: Color) -> bool {
        let dx = from.x as isize - to.x as isize;
        let dy = from.y as isize - to.y as isize;

        // Horse moves in "L" shape: 2 in one direction, 1 in perpendicular
        let (abs_dx, abs_dy) = (dx.abs(), dy.abs());
        if !((abs_dx == 2 && abs_dy == 1) || (abs_dx == 1 && abs_dy == 2)) {
            return false;
        }

        // Check hobbling (blocking leg)
        let (block_x, block_y) = if abs_dx == 2 {
            (from.x as isize - dx.signum(), from.y as isize)
        } else {
            (from.x as isize, from.y as isize - dy.signum())
        };

        if (0..9).contains(&block_x) && (0..10).contains(&block_y)
            && !self.is_empty_xy(block_x as usize, block_y as usize)
        {
            return false;
        }

        true
    }

    fn can_chariot_move(&self, from: Position, to: Position, _color: Color) -> bool {
        // Must be on same file or rank
        if !from.on_same_file(to) && !from.on_same_rank(to) {
            return false;
        }
        // Path must be clear
        self.count_between(from, to) == 0
    }

    fn can_cannon_move(&self, from: Position, to: Position, _color: Color) -> bool {
        // Must be on same file or rank
        if !from.on_same_file(to) && !from.on_same_rank(to) {
            return false;
        }

        let pieces_between = self.count_between(from, to);
        let target_is_occupied = self.get(to).is_some();

        match (target_is_occupied, pieces_between) {
            // Moving to empty square: need 0 pieces between
            (false, 0) => true,
            // Capturing: need exactly 1 piece between (the screen)
            (true, 1) => true,
            _ => false,
        }
    }

    fn can_soldier_move(&self, from: Position, to: Position, color: Color) -> bool {
        // Soldiers move 1 square
        if from.chebyshev_distance(to) != 1 {
            return false;
        }

        // Determine if soldier has crossed river
        let crossed_river = match color {
            Color::Red => from.y <= 4,
            Color::Black => from.y >= 5,
        };

        // Check if move is forward
        let forward = match color {
            Color::Red => to.y < from.y, // Red moves up (decreasing Y)
            Color::Black => to.y > from.y, // Black moves down (increasing Y)
        };

        // Check if move is sideways
        let sideways = from.on_same_rank(to);

        if crossed_river {
            // After crossing river: can move forward or sideways, not backward
            forward || sideways
        } else {
            // Before crossing river: can only move forward
            forward && !sideways
        }
    }

    /// Check if a move is legal according to all rules
    pub fn is_legal_move(&self, from: Position, to: Position) -> bool {
        let piece = match self.get(from) {
            Some(p) => *p,
            None => return false,
        };

        // Target must be valid position
        if !to.is_valid() {
            return false;
        }

        // Cannot capture own piece
        if let Some(target) = self.get(to) {
            if target.color == piece.color {
                return false;
            }
        }

        // Check piece-specific movement rules
        if !self.is_valid_move(from, to, piece) {
            return false;
        }

        // Simulate move to check if it leaves king in check
        let mut test_board = self.clone();
        test_board.move_piece(from, to);

        // Check for flying general violation
        if test_board.generals_facing() {
            return false;
        }

        // Cannot leave own general in check
        if test_board.is_in_check(piece.color) {
            return false;
        }

        true
    }

    pub fn width(&self) -> usize {
        BOARD_WIDTH
    }

    pub fn height(&self) -> usize {
        BOARD_HEIGHT
    }
}
