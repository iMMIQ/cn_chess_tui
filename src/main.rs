mod types;
mod board;
mod game;
mod ui;

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
    style::{Color as RColor, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use std::io;
use std::time::{Duration, Instant};

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
                        self.show_message(format!("Not your piece - it's {}'s turn", self.game.turn()));
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
        let size = f.area();

        let msg_area = self.centered_rect(message.len() as u16 + 4, 3, size);

        let paragraph = Paragraph::new(message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(RColor::Cyan))
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

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

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
