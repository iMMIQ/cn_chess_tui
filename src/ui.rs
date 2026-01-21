use crate::game::Game;
use crate::types::Color;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color as RColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const BOARD_WIDTH: u16 = 17;  // 9 files * 2 - 1
const BOARD_HEIGHT: u16 = 19; // 10 ranks * 2 - 1

pub struct UI;

impl UI {
    pub fn draw(f: &mut Frame, game: &Game) {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Board
                Constraint::Length(3),  // Status bar
            ])
            .split(size);

        Self::draw_header(f, chunks[0], game);
        Self::draw_board(f, chunks[1], game);
        Self::draw_status(f, chunks[2], game);
    }

    fn draw_header(f: &mut Frame, area: Rect, game: &Game) {
        let title = "中国象棋 Chinese Chess";
        let turn_text = match game.turn() {
            Color::Red => "红方执棋 Red's Turn",
            Color::Black => "黑方执棋 Black's Turn",
        };

        let check_indicator = if game.is_in_check() {
            " [将军! CHECK!]"
        } else {
            ""
        };

        let spans = vec![
            Span::styled(
                title,
                Style::default()
                    .fg(RColor::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("{}{}", turn_text, check_indicator),
                Style::default()
                    .fg(match game.turn() {
                        Color::Red => RColor::Red,
                        Color::Black => RColor::Gray,
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ];

        let paragraph = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn draw_board(f: &mut Frame, area: Rect, game: &Game) {
        let board_area = Self::centered_rect(BOARD_WIDTH + 4, BOARD_HEIGHT + 2, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("棋盘 Board");

        f.render_widget(block, board_area);

        let inner = board_area.inner(Margin::new(1, 1));

        // Draw board grid lines
        Self::draw_grid(f, inner);

        // Draw river
        Self::draw_river(f, inner);

        // Draw palace diagonals
        Self::draw_palace_lines(f, inner);

        // Draw pieces
        Self::draw_pieces(f, inner, game);
    }

    fn draw_grid(f: &mut Frame, area: Rect) {
        // Horizontal lines
        for y in 0..10 {
            let py = area.y + y as u16 * 2;
            for x in 0..8 {
                let px = area.x + x as u16 * 2 + 1;
                let span = Span::styled("─", Style::default().fg(RColor::DarkGray));
                let paragraph = Paragraph::new(span);
                let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
                f.render_widget(paragraph, cell_area);
            }
        }

        // Vertical lines (top half - above river)
        for y in 0..5 {
            for x in 0..9 {
                let px = area.x + x as u16 * 2;
                let py = area.y + y as u16 * 2;

                let c = if y > 0 {
                    if x == 0 || x == 8 {
                        "├" // Left/right edge with vertical
                    } else {
                        "┼" // Cross
                    }
                } else {
                    if x == 0 {
                        "┌"
                    } else if x == 8 {
                        "┐"
                    } else {
                        "┬"
                    }
                };

                let span = Span::styled(c, Style::default().fg(RColor::DarkGray));
                let paragraph = Paragraph::new(span);
                let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
                f.render_widget(paragraph, cell_area);
            }
        }

        // Vertical lines (bottom half - below river)
        for y in 5..10 {
            for x in 0..9 {
                let px = area.x + x as u16 * 2;
                let py = area.y + y as u16 * 2;

                let c = if y < 9 {
                    if x == 0 || x == 8 {
                        "├" // Left/right edge with vertical
                    } else {
                        "┼" // Cross
                    }
                } else {
                    if x == 0 {
                        "└"
                    } else if x == 8 {
                        "┘"
                    } else {
                        "┴"
                    }
                };

                let span = Span::styled(c, Style::default().fg(RColor::DarkGray));
                let paragraph = Paragraph::new(span);
                let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
                f.render_widget(paragraph, cell_area);
            }
        }
    }

    fn draw_river(f: &mut Frame, area: Rect) {
        let river_y = area.y + 9; // Between y=4 and y=5 (4*2 + 1)
        let river_text = "楚河              汉界";
        let paragraph = Paragraph::new(river_text)
            .style(Style::default().fg(RColor::Yellow))
            .alignment(Alignment::Center);

        let river_area = Rect {
            x: area.x,
            y: river_y,
            width: BOARD_WIDTH,
            height: 1,
        };
        f.render_widget(paragraph, river_area);
    }

    fn draw_palace_lines(f: &mut Frame, area: Rect) {
        // Top palace (Black) - diagonals from (3,0) to (5,2)
        // Using X characters for diagonal lines
        for i in 0..3 {
            let px = area.x + 3 * 2 + i as u16 * 2;
            let py = area.y + i as u16 * 2;
            let span = Span::styled("\\", Style::default().fg(RColor::DarkGray));
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);

            let px2 = area.x + 5 * 2 - i as u16 * 2;
            let span2 = Span::styled("/", Style::default().fg(RColor::DarkGray));
            let paragraph2 = Paragraph::new(span2);
            let cell_area2 = Rect { x: px2, y: py, width: 1, height: 1 };
            f.render_widget(paragraph2, cell_area2);
        }

        // Bottom palace (Red) - diagonals from (3,7) to (5,9)
        for i in 0..3 {
            let px = area.x + 3 * 2 + i as u16 * 2;
            let py = area.y + 7 * 2 + i as u16 * 2;
            let span = Span::styled("\\", Style::default().fg(RColor::DarkGray));
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);

            let px2 = area.x + 5 * 2 - i as u16 * 2;
            let span2 = Span::styled("/", Style::default().fg(RColor::DarkGray));
            let paragraph2 = Paragraph::new(span2);
            let cell_area2 = Rect { x: px2, y: py, width: 1, height: 1 };
            f.render_widget(paragraph2, cell_area2);
        }
    }

    fn draw_pieces(f: &mut Frame, area: Rect, game: &Game) {
        for (pos, piece) in game.board().pieces() {
            let px = area.x + pos.x as u16 * 2;
            let py = area.y + pos.y as u16 * 2;

            let color = match piece.color {
                Color::Red => RColor::Red,
                Color::Black => RColor::Gray,
            };

            let piece_text = piece.to_string();
            let span = Span::styled(
                piece_text,
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            );

            let paragraph = Paragraph::new(span).alignment(Alignment::Center);

            let cell_area = Rect {
                x: px,
                y: py,
                width: 2,
                height: 1,
            };
            f.render_widget(paragraph, cell_area);
        }
    }

    fn draw_status(f: &mut Frame, area: Rect, game: &Game) {
        let help_text = "Arrow keys: Move cursor | Enter: Select | q: Quit | u: Undo";
        let move_text = if game.get_moves().is_empty() {
            "No moves yet".to_string()
        } else {
            let moves = game.get_moves();
            let last_move = moves.last().unwrap();
            format!("Last move: ({},{}) -> ({},{})", last_move.from.x, last_move.from.y, last_move.to.x, last_move.to.y)
        };

        let spans = vec![
            Span::raw(help_text),
            Span::raw(" | "),
            Span::styled(move_text, Style::default().fg(RColor::Yellow)),
        ];

        let paragraph = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    /// Helper function to center a rectangle within the given area
    fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
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
