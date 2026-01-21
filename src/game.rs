use crate::board::Board;
use crate::types::{Color, Position};
use std::fmt::{self, Display, Formatter};

/// Result of a completed game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    RedWins,
    BlackWins,
    Draw,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::RedWins => write!(f, "Red Wins"),
            GameResult::BlackWins => write!(f, "Black Wins"),
            GameResult::Draw => write!(f, "Draw"),
        }
    }
}

/// Current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Checkmate(Color),
    Stalemate,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Playing => write!(f, "Playing"),
            GameState::Checkmate(color) => write!(f, "Checkmate - {} Wins", color),
            GameState::Stalemate => write!(f, "Stalemate"),
        }
    }
}

/// A single move record with from and to positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }
}

/// Errors that can occur during move operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveError {
    NoPieceAtPosition,
    WrongTurn(Color),
    InvalidMove,
    WouldLeaveInCheck,
    GameOver(GameResult),
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::NoPieceAtPosition => write!(f, "No piece at the specified position"),
            MoveError::WrongTurn(color) => write!(f, "It is {}'s turn", color),
            MoveError::InvalidMove => write!(f, "Invalid move according to chess rules"),
            MoveError::WouldLeaveInCheck => write!(f, "Move would leave your general in check"),
            MoveError::GameOver(result) => write!(f, "Game is over: {}", result),
        }
    }
}

impl std::error::Error for MoveError {}

/// Main game structure managing board, turn, and game state
#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    turn: Color,
    move_history: Vec<MoveRecord>,
    state: GameState,
}

/// Internal record for move history (includes captured piece info)
#[derive(Debug, Clone)]
struct MoveRecord {
    mv: Move,
    captured: Option<crate::types::Piece>,
}

impl Game {
    /// Create a new game with initial board setup
    pub fn new() -> Self {
        let board = Board::new();
        let turn = Color::Red;
        let move_history = Vec::new();
        let state = GameState::Playing;

        Self {
            board,
            turn,
            move_history,
            state,
        }
    }

    /// Get a reference to the board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get the current turn
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Get the current game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Get the move history
    pub fn move_history(&self) -> &[Move] {
        // Extract just the moves from the history
        self.move_history.iter().map(|r| r.mv).collect::<Vec<_>>().leak()
    }

    /// Get a reference to the move history as a Vec
    pub fn get_moves(&self) -> Vec<Move> {
        self.move_history.iter().map(|r| r.mv).collect()
    }

    /// Make a move on the board
    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        // Check if game is already over
        if !matches!(self.state, GameState::Playing) {
            let result = match self.state {
                GameState::Checkmate(Color::Red) => GameResult::RedWins,
                GameState::Checkmate(Color::Black) => GameResult::BlackWins,
                GameState::Stalemate => GameResult::Draw,
                _ => return Err(MoveError::GameOver(GameResult::Draw)),
            };
            return Err(MoveError::GameOver(result));
        }

        // Check if there's a piece at the from position
        let piece = match self.board.get(from) {
            Some(p) => *p,
            None => return Err(MoveError::NoPieceAtPosition),
        };

        // Check if it's the correct turn
        if piece.color != self.turn {
            return Err(MoveError::WrongTurn(self.turn));
        }

        // Check if the move is legal
        if !self.board.is_legal_move(from, to) {
            return Err(MoveError::InvalidMove);
        }

        // Record the captured piece if any
        let captured = self.board.get(to).copied();

        // Make the move
        self.board.move_piece(from, to);

        // Record the move in history
        self.move_history.push(MoveRecord {
            mv: Move::new(from, to),
            captured,
        });

        // Switch turns
        self.turn = match self.turn {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        // Update game state (check for checkmate/stalemate)
        self.update_state();

        Ok(())
    }

    /// Undo the last move
    pub fn undo_move(&mut self) -> bool {
        if let Some(record) = self.move_history.pop() {
            // Move the piece back
            let piece = self.board.get(record.mv.to).copied().unwrap();
            self.board.remove_piece(record.mv.to);
            self.board.place_piece(record.mv.from, piece);

            // Restore captured piece if there was one
            if let Some(captured) = record.captured {
                self.board.place_piece(record.mv.to, captured);
            }

            // Switch turn back
            self.turn = match self.turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };

            // Reset state to playing
            self.state = GameState::Playing;

            true
        } else {
            false
        }
    }

    /// Check if the current player is in check
    pub fn is_in_check(&self) -> bool {
        self.board.is_in_check(self.turn)
    }

    /// Check if a specific color is in check
    pub fn is_color_in_check(&self, color: Color) -> bool {
        self.board.is_in_check(color)
    }

    /// Update the game state based on current position
    fn update_state(&mut self) {
        // First, check if current player is in check
        let in_check = self.is_in_check();

        // Check if current player has any legal moves
        if !self.has_legal_moves(self.turn) {
            if in_check {
                // No legal moves while in check = checkmate
                let winner = match self.turn {
                    Color::Red => Color::Black,
                    Color::Black => Color::Red,
                };
                self.state = GameState::Checkmate(winner);
            } else {
                // No legal moves but not in check = stalemate
                self.state = GameState::Stalemate;
            }
        } else {
            self.state = GameState::Playing;
        }
    }

    /// Check if a player has any legal moves
    fn has_legal_moves(&self, color: Color) -> bool {
        // Get all pieces of the current color
        for (pos, _piece) in self.board.pieces_of_color(color) {
            // Check all possible destination squares
            for y in 0..self.board.height() {
                for x in 0..self.board.width() {
                    let dest = Position::from_xy(x, y);
                    if dest == pos {
                        continue;
                    }
                    if self.board.is_legal_move(pos, dest) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get a mutable reference to the board (use with caution)
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
