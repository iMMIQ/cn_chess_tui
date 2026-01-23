mod board;
mod fen;
mod fen_io;
mod fen_print;
mod game;
mod notation;
mod pgn;
mod types;
mod ucci;
mod ui;

use crate::fen::FenError;
use crate::game::Game;
use crate::types::Position;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use std::io;
use std::process;
use std::time::{Duration, Instant};

fn print_usage() {
    println!("Chinese Chess TUI - Usage:");
    println!("  cn_chess_tui               Start a new game");
    println!("  cn_chess_tui --print <fen> Print FEN position to terminal and exit");
    println!("  cn_chess_tui --fen <fen>   Load and play from FEN string");
    println!("  cn_chess_tui --file <path> Load and play from .fen file");
    println!("  cn_chess_tui --pgn <path>  Load and play from .pgn file");
    println!("  cn_chess_tui --export-pgn  Export current game to PGN (not yet implemented)");
    println!("  cn_chess_tui --export-xml  Export current game to XML (not yet implemented)");
    println!("  cn_chess_tui --help        Show this help message");
}

fn print_fen_position(fen: &str) -> Result<(), FenError> {
    let game = Game::from_fen(fen)?;
    fen_print::print_game_state(&game);
    Ok(())
}

/// Selection state for piece movement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionState {
    SelectingSource,
    SelectingDestination(Position),
}

/// Main application state
struct App {
    game: Game,
    cursor: Position,
    selection: SelectionState,
    message: Option<String>,
    message_time: Instant,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            game: Game::new(),
            cursor: Position::from_xy(4, 9), // Start at Red General's position
            selection: SelectionState::SelectingSource,
            message: None,
            message_time: Instant::now(),
            running: true,
        }
    }

    fn from_fen(fen: &str) -> Result<Self, FenError> {
        Ok(Self {
            game: Game::from_fen(fen)?,
            cursor: Position::from_xy(4, 9),
            selection: SelectionState::SelectingSource,
            message: None,
            message_time: Instant::now(),
            running: true,
        })
    }

    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let fen = crate::fen_io::read_fen_file(path)?;
        let game = Game::from_fen(&fen)?;
        Ok(Self {
            game,
            cursor: Position::from_xy(4, 9),
            selection: SelectionState::SelectingSource,
            message: None,
            message_time: Instant::now(),
            running: true,
        })
    }

    fn from_pgn(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read PGN file
        let pgn_content = std::fs::read_to_string(path)?;

        // Parse PGN
        let pgn_game =
            crate::pgn::PgnGame::parse(&pgn_content).ok_or("Failed to parse PGN file")?;

        // Create game and apply moves from PGN
        let mut game = Game::new();

        // Check if FEN tag is present and use it
        if let Some(fen) = pgn_game.get_tag("FEN") {
            if !fen.is_empty() {
                game = Game::from_fen(fen)?;
            }
        }

        // Apply all moves from the PGN
        for pgn_move in &pgn_game.moves {
            // Parse the move notation (assuming ICCS format)
            let notation = &pgn_move.notation;

            // ICCS notation is 4 characters: from_x, from_y, to_x, to_y
            // Example: "h2e2" means from h2 to e2
            if notation.len() >= 4 {
                let chars: Vec<char> = notation.chars().collect();

                // Parse from position (e.g., "h2" -> x=7, y=1)
                // Files: a=0, b=1, ..., h=7, i=8
                // Ranks: 0=0, 1=1, ..., 9=9
                let from_file = (chars[0] as i8) - (b'a' as i8);
                let from_rank = (chars[1] as i8) - (b'0' as i8) - 1;

                // Parse to position (e.g., "e2" -> x=4, y=1)
                let to_file = (chars[2] as i8) - (b'a' as i8);
                let to_rank = (chars[3] as i8) - (b'0' as i8) - 1;

                // Validate coordinates are within board bounds
                if (0..9).contains(&from_file)
                    && (0..10).contains(&from_rank)
                    && (0..9).contains(&to_file)
                    && (0..10).contains(&to_rank)
                {
                    let from = Position::from_xy(from_file as usize, from_rank as usize);
                    let to = Position::from_xy(to_file as usize, to_rank as usize);

                    // Attempt to make the move
                    if game.make_move(from, to).is_err() {
                        // If move fails, continue with next move
                        // This allows partially loading games with invalid moves
                        eprintln!("Warning: Failed to apply move {}", notation);
                    }
                }
            }
        }

        Ok(Self {
            game,
            cursor: Position::from_xy(4, 9),
            selection: SelectionState::SelectingSource,
            message: None,
            message_time: Instant::now(),
            running: true,
        })
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Char('r') => {
                // Restart the game
                *self = Self::new();
            }
            KeyCode::Char('u') => {
                if self.game.undo_move() {
                    self.show_message("Move undone".to_string());
                } else {
                    self.show_message("No moves to undo".to_string());
                }
                self.selection = SelectionState::SelectingSource;
            }
            KeyCode::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor.y < 9 {
                    self.cursor.y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor.x < 8 {
                    self.cursor.x += 1;
                }
            }
            KeyCode::Enter => {
                self.handle_selection();
            }
            _ => {}
        }
    }

    fn handle_selection(&mut self) {
        match self.selection {
            SelectionState::SelectingSource => {
                // Check if there's a piece at cursor position
                if let Some(piece) = self.game.board().get(self.cursor) {
                    // Check if it's the current player's piece
                    if piece.color == self.game.turn() {
                        self.selection = SelectionState::SelectingDestination(self.cursor);
                    } else {
                        self.show_message(format!(
                            "Not your piece - it's {}'s turn",
                            self.game.turn()
                        ));
                    }
                } else {
                    self.show_message("No piece at this position".to_string());
                }
            }
            SelectionState::SelectingDestination(source) => {
                // Try to make the move
                let result = self.game.make_move(source, self.cursor);
                match result {
                    Ok(()) => {
                        self.show_message("Move successful".to_string());
                    }
                    Err(e) => {
                        self.show_message(format!("Invalid move: {}", e));
                    }
                }
                self.selection = SelectionState::SelectingSource;
            }
        }
    }

    fn show_message(&mut self, msg: String) {
        self.message = Some(msg);
        self.message_time = Instant::now();
    }

    fn draw(&mut self, f: &mut Frame) {
        // Convert SelectionState to Option<Position>
        let selection = match self.selection {
            SelectionState::SelectingSource => None,
            SelectionState::SelectingDestination(pos) => Some(pos),
        };

        // Draw the main game UI with cursor and selection
        // (includes game over popup when game is not in Playing state)
        ui::UI::draw(f, &self.game, self.cursor, selection);

        // Draw message overlay if active
        if let Some(ref msg) = self.message {
            if self.message_time.elapsed() < Duration::from_secs(2) {
                self.draw_message(f, msg);
            } else {
                self.message = None;
            }
        }
    }

    fn draw_message(&self, f: &mut Frame, message: &str) {
        use ratatui::style::Color as RColor;

        let size = f.area();

        let msg_area = self.centered_rect(message.len() as u16 + 6, 3, size);

        let paragraph = Paragraph::new(message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(RColor::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .title(Span::styled(
                        " 提示 Info ",
                        Style::default().fg(RColor::LightCyan),
                    ))
                    .style(Style::default().bg(RColor::Black)),
            )
            .alignment(Alignment::Center);

        f.render_widget(Clear, msg_area);
        f.render_widget(paragraph, msg_area);
    }

    /// Helper function to center a rectangle within the given area
    fn centered_rect(&self, width: u16, height: u16, r: Rect) -> Rect {
        let patch_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((r.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((r.height.saturating_sub(height)) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((r.width.saturating_sub(width)) / 2),
            ])
            .split(patch_layout[1])[1]
    }
}

fn run_game(app: &mut App) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Main loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    while app.running {
        // Draw
        terminal.draw(|f| app.draw(f))?;

        // Handle input with timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // No arguments - start new game
    if args.len() == 1 {
        let mut app = App::new();
        if let Err(e) = run_game(&mut app) {
            eprintln!("Error running game: {}", e);
            process::exit(1);
        }
        return;
    }

    // Parse CLI arguments
    match args[1].as_str() {
        "--help" | "-h" => {
            print_usage();
        }
        "--print" => {
            if args.len() < 3 {
                eprintln!("Error: --print requires a FEN string");
                println!();
                print_usage();
                process::exit(1);
            }
            let fen = &args[2];
            if let Err(e) = print_fen_position(fen) {
                eprintln!("Error parsing FEN: {}", e);
                process::exit(1);
            }
        }
        "--fen" => {
            if args.len() < 3 {
                eprintln!("Error: --fen requires a FEN string");
                println!();
                print_usage();
                process::exit(1);
            }
            let fen = &args[2];
            match App::from_fen(fen) {
                Ok(mut app) => {
                    if let Err(e) = run_game(&mut app) {
                        eprintln!("Error running game: {}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing FEN: {}", e);
                    process::exit(1);
                }
            }
        }
        "--file" => {
            if args.len() < 3 {
                eprintln!("Error: --file requires a file path");
                println!();
                print_usage();
                process::exit(1);
            }
            let path = &args[2];
            match App::from_file(path) {
                Ok(mut app) => {
                    if let Err(e) = run_game(&mut app) {
                        eprintln!("Error running game: {}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error loading file: {}", e);
                    process::exit(1);
                }
            }
        }
        "--pgn" => {
            if args.len() < 3 {
                eprintln!("Error: --pgn requires a file path");
                println!();
                print_usage();
                process::exit(1);
            }
            let path = &args[2];
            match App::from_pgn(path) {
                Ok(mut app) => {
                    if let Err(e) = run_game(&mut app) {
                        eprintln!("Error running game: {}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error loading PGN file: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown argument: {}", args[1]);
            println!();
            print_usage();
            process::exit(1);
        }
    }
}
