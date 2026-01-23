use crate::board::Board;
use crate::fen::FenError;
use crate::notation::iccs;
use crate::notation::move_to_chinese_with_context;
use crate::pgn::{PgnGame, PgnGameResult};
use crate::types::{Color, Position};
use crate::ucci::UcciClient;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

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

/// AI mode for game controller
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiMode {
    Off,        // Player vs Player
    PlaysRed,   // AI plays Red
    PlaysBlack, // AI plays Black
    PlaysBoth,  // AI vs AI (spectator mode)
}

/// AI configuration
#[derive(Debug, Clone, Default)]
pub struct AiConfig {
    pub engine_path: Option<PathBuf>,
    pub show_thinking: bool,
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
    #[allow(dead_code)]
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
    piece: crate::types::Piece,
    captured: Option<crate::types::Piece>,
}

/// Game controller with AI support
pub struct GameController {
    game: Game,
    ai_mode: AiMode,
    ai_client: Option<UcciClient>,
    ai_config: AiConfig,
    engine_thinking: bool,
}

impl Default for GameController {
    fn default() -> Self {
        Self::new()
    }
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

    /// Get a reference to the move history as a Vec
    pub fn get_moves(&self) -> Vec<Move> {
        self.move_history.iter().map(|r| r.mv).collect()
    }

    /// Get move history with piece information for notation display
    pub fn get_notated_moves(&self) -> Vec<(crate::types::Piece, Move)> {
        self.move_history.iter().map(|r| (r.piece, r.mv)).collect()
    }

    /// Get move history in ICCS notation format
    #[allow(dead_code)]
    pub fn get_moves_with_iccs(&self) -> Vec<String> {
        self.move_history
            .iter()
            .map(|r| iccs::move_to_iccs(r.mv.from, r.mv.to))
            .collect()
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
            piece,
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
            let piece = self
                .board
                .get(record.mv.to)
                .copied()
                .expect("undo_move: piece must exist at move destination");
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    #[allow(dead_code)]
    /// Force a game state for testing purposes
    pub fn force_state_for_testing(&mut self, state: GameState) {
        self.state = state;
    }

    /// Create a game from a FEN string
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let (board, turn) = crate::fen::fen_to_board(fen)?;

        Ok(Self {
            board,
            turn,
            move_history: Vec::new(),
            state: GameState::Playing,
        })
    }

    /// Export the current game state to FEN format
    pub fn to_fen(&self) -> String {
        // Calculate full move number from history
        // Each full move = two half-moves (one by each side)
        let full_move_count = (self.move_history.len() / 2) + 1;

        crate::fen::board_to_fen(&self.board, self.turn, 0, full_move_count as u32)
    }

    /// Export the game to PGN format
    ///
    /// Creates a PgnGame with standard tags and move history in Chinese notation.
    ///
    /// # Examples
    /// ```
    /// use cn_chess_tui::Game;
    ///
    /// let mut game = Game::new();
    /// // Make some moves...
    /// let pgn = game.to_pgn();
    /// let pgn_str = pgn.to_string();
    /// assert!(pgn_str.contains("[Red \"?\"]"));
    /// assert!(pgn_str.contains("[Black \"?\"]"));
    /// ```
    #[allow(dead_code)]
    pub fn to_pgn(&self) -> PgnGame {
        let mut pgn_game = PgnGame::new();

        // Set standard tags
        pgn_game.set_tag("Game", "Chinese Chess");
        pgn_game.set_tag("Red", "?");
        pgn_game.set_tag("Black", "?");

        // Set result based on game state
        let result = match self.state {
            GameState::Checkmate(Color::Red) => PgnGameResult::RedWins,
            GameState::Checkmate(Color::Black) => PgnGameResult::BlackWins,
            GameState::Stalemate => PgnGameResult::Draw,
            GameState::Playing => PgnGameResult::Unknown,
        };
        pgn_game.set_tag("Result", result.to_pgn_string());

        // Set date to today (using placeholder format)
        pgn_game.set_tag("Date", "????.??.??");

        // Add move history using Chinese notation with context
        for record in &self.move_history {
            let chinese_notation =
                move_to_chinese_with_context(self, record.piece, record.mv.from, record.mv.to);
            pgn_game.add_move(chinese_notation);
        }

        pgn_game.result = result;
        pgn_game
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl GameController {
    /// Create a new game controller
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            ai_mode: AiMode::Off,
            ai_client: None,
            ai_config: AiConfig::default(),
            engine_thinking: false,
        }
    }

    /// Create controller from FEN
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        Ok(Self {
            game: Game::from_fen(fen)?,
            ai_mode: AiMode::Off,
            ai_client: None,
            ai_config: AiConfig::default(),
            engine_thinking: false,
        })
    }

    /// Create controller from existing Game
    pub fn from_game(game: Game) -> Self {
        Self {
            game,
            ai_mode: AiMode::Off,
            ai_client: None,
            ai_config: AiConfig::default(),
            engine_thinking: false,
        }
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn board(&self) -> &Board {
        self.game.board()
    }

    pub fn turn(&self) -> Color {
        self.game.turn()
    }

    pub fn state(&self) -> GameState {
        self.game.state()
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.game.get_moves()
    }

    pub fn get_notated_moves(&self) -> Vec<(crate::types::Piece, Move)> {
        self.game.get_notated_moves()
    }

    pub fn to_fen(&self) -> String {
        self.game.to_fen()
    }

    pub fn is_in_check(&self) -> bool {
        self.game.is_in_check()
    }

    /// Get current AI mode
    pub fn ai_mode(&self) -> AiMode {
        self.ai_mode
    }

    /// Set AI mode
    pub fn set_ai_mode(&mut self, mode: AiMode) {
        self.ai_mode = mode;
    }

    /// Check if engine is currently thinking
    pub fn is_engine_thinking(&self) -> bool {
        self.engine_thinking
    }

    /// Get AI config
    pub fn ai_config(&self) -> &AiConfig {
        &self.ai_config
    }

    /// Set AI config
    pub fn set_ai_config(&mut self, config: AiConfig) {
        self.ai_config = config;
    }

    /// Initialize AI engine with given path
    pub fn init_engine(&mut self, engine_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if path exists
        if !Path::new(engine_path).exists() {
            return Err("Engine path does not exist".into());
        }

        // Create client
        let mut client = UcciClient::new(engine_path)?;

        // Initialize engine
        client.initialize()?;

        self.ai_client = Some(client);
        self.ai_config.engine_path = Some(PathBuf::from(engine_path));

        Ok(())
    }

    /// Check if engine is initialized
    pub fn has_engine(&self) -> bool {
        self.ai_client.is_some()
    }

    /// Make a move as a human player (not AI)
    pub fn human_move(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        // If AI is thinking, don't allow human moves
        if self.engine_thinking {
            return Err(MoveError::InvalidMove);
        }

        self.game.make_move(from, to)
    }

    /// Undo the last move
    pub fn undo_move(&mut self) -> bool {
        if self.engine_thinking {
            return false; // Don't allow undo while AI is thinking
        }
        self.game.undo_move()
    }

    /// Check if AI should make the next move
    fn should_ai_move(&self) -> bool {
        if matches!(self.game.state(), GameState::Playing) {
            match self.ai_mode {
                AiMode::Off => false,
                AiMode::PlaysRed => self.game.turn() == Color::Red,
                AiMode::PlaysBlack => self.game.turn() == Color::Black,
                AiMode::PlaysBoth => true,
            }
        } else {
            false
        }
    }

    /// Trigger AI to make a move
    pub fn trigger_ai_move(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.should_ai_move() {
            return Ok(());
        }

        let client = self.ai_client.as_mut().ok_or("AI engine not initialized")?;

        // Sync engine with current position
        let fen = self.game.to_fen();
        let moves = self.game.get_moves_with_iccs();
        client.set_position(&fen, &moves)?;

        // Start depth search (depth 10)
        client.go_depth(10)?;

        self.engine_thinking = true;
        Ok(())
    }

    /// Check if engine has responded, apply move if ready
    pub fn check_engine_response(
        &mut self,
    ) -> Result<Option<(Position, Position)>, Box<dyn std::error::Error>> {
        if !self.engine_thinking {
            return Ok(None);
        }

        let client = self.ai_client.as_mut().ok_or("AI engine not initialized")?;

        // Check if engine is ready
        if !client.is_ready()? {
            return Ok(None);
        }

        // Get the move
        let result = client.stop()?;
        let mv = match result {
            crate::ucci::MoveResult::Move(mv_str, _) => {
                match crate::notation::parse_iccs_move(&mv_str) {
                    Ok(pos) => pos,
                    Err(_) => {
                        self.engine_thinking = false;
                        return Ok(None);
                    }
                }
            }
            crate::ucci::MoveResult::NoMove => {
                self.engine_thinking = false;
                return Ok(None);
            }
            crate::ucci::MoveResult::Draw | crate::ucci::MoveResult::Resign => {
                self.engine_thinking = false;
                return Ok(None);
            }
        };

        // Apply the move to the game
        self.game.make_move(mv.0, mv.1)?;

        self.engine_thinking = false;
        Ok(Some(mv))
    }
}
