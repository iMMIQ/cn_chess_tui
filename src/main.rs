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
    style::{Color as RColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use std::io;
use std::time::{Duration, Instant};

const BOARD_WIDTH: u16 = 17;  // 9 files * 2 - 1
const BOARD_HEIGHT: u16 = 19; // 10 ranks * 2 - 1

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
        // Draw the main game UI
        ui::UI::draw(f, &self.game);

        // Draw cursor
        self.draw_cursor(f);

        // Draw selection highlight
        if let SelectionState::SelectingDestination(source) = self.selection {
            self.draw_selection(f, source);
        }

        // Draw message overlay if active
        if let Some(ref msg) = self.message {
            if self.message_time.elapsed() < Duration::from_secs(2) {
                self.draw_message(f, msg);
            } else {
                self.message = None;
            }
        }

        // Draw game over screen if game is over
        if !matches!(self.game.state(), crate::game::GameState::Playing) {
            self.draw_game_over(f);
        }
    }

    fn draw_cursor(&self, f: &mut Frame) {
        let size = f.area();

        // Calculate board area (same as in UI::draw_board)
        let board_area = self.centered_rect(BOARD_WIDTH + 4, BOARD_HEIGHT + 2, size);
        let inner = board_area.inner(ratatui::layout::Margin::new(1, 1));

        let px = inner.x + self.cursor.x as u16 * 2;
        let py = inner.y + self.cursor.y as u16 * 2;

        // Draw cursor as a highlighted background
        let cursor_area = Rect {
            x: px,
            y: py,
            width: 2,
            height: 1,
        };

        let cursor_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(RColor::Green));

        f.render_widget(cursor_block, cursor_area);
    }

    fn draw_selection(&self, f: &mut Frame, source: Position) {
        let size = f.area();

        // Calculate board area
        let board_area = self.centered_rect(BOARD_WIDTH + 4, BOARD_HEIGHT + 2, size);
        let inner = board_area.inner(ratatui::layout::Margin::new(1, 1));

        let px = inner.x + source.x as u16 * 2;
        let py = inner.y + source.y as u16 * 2;

        // Draw selection as a highlighted background
        let selection_area = Rect {
            x: px,
            y: py,
            width: 2,
            height: 1,
        };

        let selection_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(RColor::Yellow));

        f.render_widget(selection_block, selection_area);
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

    fn draw_game_over(&self, f: &mut Frame) {
        let size = f.area();

        let overlay_area = self.centered_rect(40, 10, size);

        let (title, color) = match self.game.state() {
            crate::game::GameState::Checkmate(winner) => {
                (format!("CHECKMATE! {} Wins!", winner), winner)
            }
            crate::game::GameState::Stalemate => ("STALEMATE - Draw".to_string(), crate::types::Color::Red),
            _ => return,
        };

        let fg_color = match color {
            crate::types::Color::Red => RColor::Red,
            crate::types::Color::Black => RColor::Gray,
        };

        let lines = vec![
            Line::from(vec![
                Span::styled(
                    title,
                    Style::default().fg(fg_color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from("Press 'q' to quit"),
        ];

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(RColor::Yellow))
                    .style(Style::default().bg(RColor::Black)),
            )
            .alignment(Alignment::Center);

        f.render_widget(Clear, overlay_area);
        f.render_widget(paragraph, overlay_area);
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
