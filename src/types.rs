use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    General,    // 帅/将
    Advisor,    // 仕/士
    Elephant,   // 相/象
    Horse,      // 马
    Chariot,    // 车
    Cannon,     // 炮
    Soldier,    // 兵/卒
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Black => write!(f, "Black"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn red(piece_type: PieceType) -> Self {
        Self::new(piece_type, Color::Red)
    }

    pub fn black(piece_type: PieceType) -> Self {
        Self::new(piece_type, Color::Black)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match (&self.color, &self.piece_type) {
            (Color::Red, PieceType::General) => "帅",
            (Color::Red, PieceType::Advisor) => "仕",
            (Color::Red, PieceType::Elephant) => "相",
            (Color::Red, PieceType::Horse) => "马",
            (Color::Red, PieceType::Chariot) => "车",
            (Color::Red, PieceType::Cannon) => "炮",
            (Color::Red, PieceType::Soldier) => "兵",
            (Color::Black, PieceType::General) => "将",
            (Color::Black, PieceType::Advisor) => "士",
            (Color::Black, PieceType::Elephant) => "象",
            (Color::Black, PieceType::Horse) => "马",
            (Color::Black, PieceType::Chariot) => "车",
            (Color::Black, PieceType::Cannon) => "炮",
            (Color::Black, PieceType::Soldier) => "卒",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,  // 0-8 (file/column)
    pub y: usize,  // 0-9 (rank/row)
}

impl Position {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        if x < 9 && y < 10 {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub fn from_xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x < 9 && self.y < 10
    }

    pub fn in_palace(&self, color: Color) -> bool {
        let x_ok = self.x >= 3 && self.x <= 5;
        let y_ok = match color {
            Color::Red => self.y >= 7 && self.y <= 9,
            Color::Black => self.y >= 0 && self.y <= 2,
        };
        x_ok && y_ok
    }

    pub fn on_same_file(&self, other: Position) -> bool {
        self.x == other.x
    }

    pub fn on_same_rank(&self, other: Position) -> bool {
        self.y == other.y
    }

    pub fn file_distance(&self, other: Position) -> usize {
        self.x.abs_diff(other.x)
    }

    pub fn rank_distance(&self, other: Position) -> usize {
        self.y.abs_diff(other.y)
    }

    pub fn chebyshev_distance(&self, other: Position) -> usize {
        self.file_distance(other).max(self.rank_distance(other))
    }
}
