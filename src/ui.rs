use crate::game::Game;
use crate::types::{Color, Position};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color as RColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const BOARD_WIDTH: u16 = 18;  // 9 files * 2 - 1 (with padding)
const BOARD_HEIGHT: u16 = 20; // 10 ranks * 2 (with padding)

pub struct UI;

impl UI {
    pub fn draw(
        f: &mut Frame,
        game: &Game,
        cursor: Position,
        selection: Option<Position>,
    ) {
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
        Self::draw_board(f, chunks[1], game, cursor, selection);
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

    fn draw_board(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
    ) {
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

        // Draw cursor highlight (green border)
        Self::draw_cursor_highlight(f, inner, cursor);

        // Draw selection highlight (yellow border)
        if let Some(selected_pos) = selected {
            Self::draw_selection_highlight(f, inner, selected_pos);
        }

        // Draw pieces
        Self::draw_pieces(f, inner, game);
    }

    fn draw_grid(f: &mut Frame, area: Rect) {
        let grid_style = Style::default().fg(RColor::DarkGray);

        // Draw the complete grid using proper box-drawing characters
        for y in 0..10 {
            for x in 0..9 {
                let px = area.x + x as u16 * 2;
                let py = area.y + y as u16 * 2;

                // Determine the character based on position
                let c = match (x, y) {
                    // Corners
                    (0, 0) => "┌",           // Top-left
                    (8, 0) => "┐",           // Top-right
                    (0, 9) => "└",           // Bottom-left
                    (8, 9) => "┘",           // Bottom-right

                    // Top edge (excluding corners)
                    (_, 0) => "┬",

                    // Bottom edge (excluding corners)
                    (_, 9) => "┴",

                    // Left/right edges
                    (0, y) if y > 0 && y < 9 => {
                        // Special handling for river edges
                        if y == 4 { "├" }     // Above river
                        else if y == 5 { "├" } // Below river
                        else { "├" }
                    }
                    (8, y) if y > 0 && y < 9 => {
                        if y == 4 { "┤" }     // Above river
                        else if y == 5 { "┤" } // Below river
                        else { "┤" }
                    }

                    // Inner grid points
                    (_, y) if y == 4 => "┼",  // Above river
                    (_, y) if y == 5 => "┼",  // Below river
                    _ => "┼",                  // Other intersections
                };

                // Draw the intersection character
                let span = Span::styled(c, grid_style);
                let paragraph = Paragraph::new(span);
                let cell_area = Rect { x: px, y: py, width: 1, height: 1 };
                f.render_widget(paragraph, cell_area);

                // Draw horizontal line to the right (except on rightmost column)
                if x < 8 {
                    let h_span = Span::styled("─", grid_style);
                    let h_paragraph = Paragraph::new(h_span);
                    let h_area = Rect { x: px + 1, y: py, width: 1, height: 1 };
                    f.render_widget(h_paragraph, h_area);
                }
            }

            // Draw vertical line below (except on bottom row)
            if y < 9 {
                for x in 0..9 {
                    let px = area.x + x as u16 * 2;
                    let py = area.y + y as u16 * 2 + 1;

                    // Don't draw vertical lines through the river (between y=4 and y=5)
                    if y == 4 {
                        continue;
                    }

                    let v_span = Span::styled("│", grid_style);
                    let v_paragraph = Paragraph::new(v_span);
                    let v_area = Rect { x: px, y: py, width: 1, height: 1 };
                    f.render_widget(v_paragraph, v_area);
                }
            }
        }
    }

    fn draw_river(f: &mut Frame, area: Rect) {
        // River is between y=4 and y=5 (at row 4*2 + 1 = 9)
        let river_y = area.y + 9;

        // "楚河" (Chu River) on the left, "汉界" (Han Border) on the right
        // Positioned nicely with proper spacing
        let chu_he = "楚河 Chu River";
        let han_jie = "汉界 Han Border";

        // Left side - 楚河
        let left_paragraph = Paragraph::new(chu_he)
            .style(Style::default().fg(RColor::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Left);

        let left_area = Rect {
            x: area.x + 2,
            y: river_y,
            width: 14,
            height: 1,
        };
        f.render_widget(left_paragraph, left_area);

        // Right side - 汉界
        let right_paragraph = Paragraph::new(han_jie)
            .style(Style::default().fg(RColor::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Right);

        let right_area = Rect {
            x: area.x + BOARD_WIDTH - 14,
            y: river_y,
            width: 14,
            height: 1,
        };
        f.render_widget(right_paragraph, right_area);
    }

    fn draw_palace_lines(f: &mut Frame, area: Rect) {
        let palace_style = Style::default().fg(RColor::DarkGray);

        // Top palace (Black) - diagonals from (3,0) to (5,2)
        // This is the 3x3 palace area in the top half
        let top_palace_x_start = 3;
        let top_palace_y_start = 0;

        // Draw diagonal from (3,0) to (5,2) - going down-right (\)
        // We draw X characters at intermediate positions to create the diagonal effect
        for i in 0..5 {
            // Calculate position along the diagonal from (3,0) to (5,2)
            // x goes from 3 to 5, y goes from 0 to 2
            let progress = i as f64 / 4.0;
            let px = top_palace_x_start as f64 + progress * 2.0;
            let py = top_palace_y_start as f64 + progress * 2.0;

            let screen_x = area.x + (px * 2.0).round() as u16;
            let screen_y = area.y + (py * 2.0).round() as u16;

            let span = Span::styled("X", palace_style);
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: screen_x, y: screen_y, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);
        }

        // Draw diagonal from (5,0) to (3,2) - going down-left (/)
        for i in 0..5 {
            let progress = i as f64 / 4.0;
            let px = 5.0 - progress * 2.0;
            let py = top_palace_y_start as f64 + progress * 2.0;

            let screen_x = area.x + (px * 2.0).round() as u16;
            let screen_y = area.y + (py * 2.0).round() as u16;

            let span = Span::styled("X", palace_style);
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: screen_x, y: screen_y, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);
        }

        // Bottom palace (Red) - diagonals from (3,7) to (5,9)
        let bottom_palace_x_start = 3;
        let bottom_palace_y_start = 7;

        // Draw diagonal from (3,7) to (5,9) - going down-right (\)
        for i in 0..5 {
            let progress = i as f64 / 4.0;
            let px = bottom_palace_x_start as f64 + progress * 2.0;
            let py = bottom_palace_y_start as f64 + progress * 2.0;

            let screen_x = area.x + (px * 2.0).round() as u16;
            let screen_y = area.y + (py * 2.0).round() as u16;

            let span = Span::styled("X", palace_style);
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: screen_x, y: screen_y, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);
        }

        // Draw diagonal from (5,7) to (3,9) - going down-left (/)
        for i in 0..5 {
            let progress = i as f64 / 4.0;
            let px = 5.0 - progress * 2.0;
            let py = bottom_palace_y_start as f64 + progress * 2.0;

            let screen_x = area.x + (px * 2.0).round() as u16;
            let screen_y = area.y + (py * 2.0).round() as u16;

            let span = Span::styled("X", palace_style);
            let paragraph = Paragraph::new(span);
            let cell_area = Rect { x: screen_x, y: screen_y, width: 1, height: 1 };
            f.render_widget(paragraph, cell_area);
        }
    }

    fn draw_pieces(f: &mut Frame, area: Rect, game: &Game) {
        for (pos, piece) in game.board().pieces() {
            let px = area.x + pos.x as u16 * 2;
            let py = area.y + pos.y as u16 * 2;

            // Color selection with improved styling
            let (fg_color, bg_color) = match piece.color {
                Color::Red => (RColor::Red, RColor::Reset),
                Color::Black => (RColor::Black, RColor::Reset),
            };

            let piece_text = piece.to_string();

            // Create styled span with bold modifier and proper colors
            let span = Span::styled(
                piece_text,
                Style::default()
                    .fg(fg_color)
                    .bg(bg_color)
                    .add_modifier(Modifier::BOLD),
            );

            let paragraph = Paragraph::new(span).alignment(Alignment::Center);

            // Center the piece in the cell
            let cell_area = Rect {
                x: px,
                y: py,
                width: 2,
                height: 1,
            };
            f.render_widget(paragraph, cell_area);
        }
    }

    fn draw_cursor_highlight(f: &mut Frame, inner: Rect, cursor: Position) {
        let px = inner.x + cursor.x as u16 * 2;
        let py = inner.y + cursor.y as u16 * 2;

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

    fn draw_selection_highlight(f: &mut Frame, inner: Rect, selected: Position) {
        let px = inner.x + selected.x as u16 * 2;
        let py = inner.y + selected.y as u16 * 2;

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
