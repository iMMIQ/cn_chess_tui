use crate::game::{Game, GameState};
use crate::types::{Color, Position};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color as RColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

// Base board dimensions (9x10 grid)
const BOARD_COLS: usize = 9;
const BOARD_ROWS: usize = 10;

// Minimum terminal sizes
const MIN_WIDTH: u16 = 40;
const MIN_HEIGHT: u16 = 24;

// Color scheme - Traditional Chinese inspired
const C_PRIMARY: RColor = RColor::Cyan;
const C_SECONDARY: RColor = RColor::LightBlue;
const C_ACCENT: RColor = RColor::LightCyan;
const C_GOLD: RColor = RColor::Yellow;
const C_GRID: RColor = RColor::DarkGray;
const C_RIVER: RColor = RColor::LightYellow;

// Piece colors
const C_RED_PIECE: RColor = RColor::Red;
const C_BLACK_PIECE: RColor = RColor::Gray;

// Highlight colors
const C_CURSOR: RColor = RColor::Green;
const C_SELECTION: RColor = RColor::Yellow;
const C_CHECK: RColor = RColor::LightRed;

// Border styles
const BORDER_ALL: Borders = Borders::ALL;

/// Responsive layout configuration
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
    pub header_height: u16,
    pub status_height: u16,
    pub cell_width: u16,
    pub cell_height: u16,
    pub show_full_header: bool,
    pub show_full_status: bool,
    pub show_river_text: bool,
    pub popup_width: u16,
    pub popup_height: u16,
}

impl LayoutConfig {
    fn from_terminal_size(size: Rect) -> Self {
        let width = size.width;
        let height = size.height;

        // Calculate available height after accounting for header and status
        let min_board_height = BOARD_ROWS as u16 * 2; // At least 20 for the board
        let available_for_header_status = height.saturating_sub(min_board_height);

        let is_small = width < MIN_WIDTH || height < MIN_HEIGHT;
        let is_very_small = width < 30 || height < 22;
        let is_tiny = height < 20;

        let header_height = if is_tiny {
            1
        } else if is_very_small {
            2
        } else if is_small {
            2
        } else {
            3
        };

        let status_height = if is_tiny || is_very_small {
            1
        } else {
            2
        };

        // Cell sizing based on terminal width
        let cell_width = if width >= 60 { 3 } else if width >= 45 { 2 } else { 1 };
        let cell_height = 2;

        let show_full_header = !is_small && !is_very_small;
        let show_full_status = !is_very_small;
        let show_river_text = width >= 45;

        let popup_width = (width * 60 / 100).min(40).max(25);
        let popup_height = (height * 40 / 100).min(12).max(8);

        LayoutConfig {
            header_height,
            status_height,
            cell_width,
            cell_height,
            show_full_header,
            show_full_status,
            show_river_text,
            popup_width,
            popup_height,
        }
    }

    fn cell_pos(&self, x: usize, y: usize) -> (u16, u16) {
        let px = (x as u16) * self.cell_width + (self.cell_width / 2);
        let py = (y as u16) * self.cell_height;
        (px, py)
    }
}

pub struct UI;

impl UI {
    pub fn draw(
        f: &mut Frame,
        game: &Game,
        cursor: Position,
        selection: Option<Position>,
    ) {
        let size = f.area();
        let config = LayoutConfig::from_terminal_size(size);

        // For very small terminals, skip status bar
        let total_ui_height = config.header_height + config.status_height;
        let (chunks, show_status) = if size.height < total_ui_height + 20 {
            // Not enough space - skip status bar
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(config.header_height),
                    Constraint::Min(0),
                ])
                .split(size);
            (chunks, false)
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(config.header_height),
                    Constraint::Min(0),
                    Constraint::Length(config.status_height),
                ])
                .split(size);
            (chunks, true)
        };

        Self::draw_header(f, chunks[0], game, &config);
        Self::draw_board(f, chunks[1], game, cursor, selection, &config);
        if show_status && chunks.len() > 2 {
            Self::draw_status(f, chunks[2], game, &config);
        }

        if game.state() != GameState::Playing {
            Self::draw_game_over_popup(f, size, game.state(), &config);
        }
    }

    fn draw_header(f: &mut Frame, area: Rect, game: &Game, config: &LayoutConfig) {
        if area.height < 2 {
            let title = if area.width < 20 { "象棋" } else { "中国象棋" };
            let turn = match game.turn() {
                Color::Red => "红",
                Color::Black => "黑",
            };
            let check = if game.is_in_check() { "!" } else { "" };
            let text = format!("{} {}{}{}", title, turn, check,
                if game.is_in_check() { "将军" } else { "" });

            f.render_widget(
                Paragraph::new(text).style(Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center),
                area,
            );
            return;
        }

        if !config.show_full_header {
            let turn = match game.turn() {
                Color::Red => "红方",
                Color::Black => "黑方",
            };
            let check = if game.is_in_check() { " 将军!" } else { "" };
            let turn_color = match game.turn() {
                Color::Red => C_RED_PIECE,
                Color::Black => C_SECONDARY,
            };

            let line = vec![
                Span::styled(" 中国象棋 ", Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD)),
                Span::styled(turn, Style::default().fg(turn_color).add_modifier(Modifier::BOLD)),
                Span::styled(check, Style::default().fg(C_CHECK).add_modifier(Modifier::BOLD)),
            ];

            f.render_widget(
                Paragraph::new(Line::from(line))
                    .block(Block::default().borders(BORDER_ALL).border_style(Style::default().fg(C_PRIMARY)))
                    .alignment(Alignment::Center),
                area,
            );
            return;
        }

        let turn_text = match game.turn() {
            Color::Red => " 红方执棋 ",
            Color::Black => " 黑方执棋 ",
        };
        let turn_english = match game.turn() {
            Color::Red => " Red's Turn ",
            Color::Black => " Black's Turn ",
        };
        let (turn_color, check_symbol) = if game.is_in_check() {
            (C_CHECK, " ✦ CHECK! ✦ ")
        } else {
            (match game.turn() {
                Color::Red => C_RED_PIECE,
                Color::Black => C_SECONDARY,
            }, "")
        };

        let line1 = vec![
            Span::styled("◆", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
            Span::styled(" 中国象棋 Chinese Chess ", Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled("◆", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
        ];

        let line2 = vec![
            Span::styled(turn_text, Style::default().fg(turn_color).add_modifier(Modifier::BOLD)),
            Span::styled(turn_english, Style::default().fg(C_ACCENT)),
            if !check_symbol.is_empty() {
                Span::styled(check_symbol, Style::default().fg(C_CHECK).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            },
        ];

        f.render_widget(
            Paragraph::new(vec![Line::from(line1), Line::from(line2)])
                .block(Block::default().borders(BORDER_ALL).border_style(Style::default().fg(C_PRIMARY)))
                .alignment(Alignment::Center),
            area,
        );
    }

    fn draw_board(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
        config: &LayoutConfig,
    ) {
        let board_width = (BOARD_COLS as u16) * config.cell_width + 2;
        let board_height = (BOARD_ROWS as u16) * config.cell_height + 2;
        let board_area = Self::centered_rect(board_width, board_height, area);

        let block = if area.width > 30 {
            Block::default()
                .borders(BORDER_ALL)
                .border_style(Style::default().fg(C_SECONDARY))
                .title(Span::styled(" 棋盘 ", Style::default().fg(C_ACCENT)))
        } else {
            Block::default()
                .borders(BORDER_ALL)
                .border_style(Style::default().fg(C_SECONDARY))
        };

        f.render_widget(block, board_area);

        let inner = board_area.inner(Margin::new(1, 1));

        Self::draw_grid(f, inner, config);
        if config.show_river_text {
            Self::draw_river(f, inner, config);
        }
        Self::draw_cursor_highlight(f, inner, cursor, config);
        if let Some(sel) = selected {
            Self::draw_selection_highlight(f, inner, sel, config);
        }
        Self::draw_pieces(f, inner, game, config);
    }

    fn draw_grid(f: &mut Frame, area: Rect, config: &LayoutConfig) {
        let grid_style = Style::default().fg(C_GRID);
        let corner_style = Style::default().fg(C_SECONDARY);

        for y in 0..BOARD_ROWS {
            for x in 0..BOARD_COLS {
                let (px, py) = config.cell_pos(x, y);
                let px = area.x + px;
                let py = area.y + py;

                let (c, is_corner) = if x == 0 && y == 0 {
                    ("┌", true)
                } else if x == BOARD_COLS - 1 && y == 0 {
                    ("┐", true)
                } else if x == 0 && y == BOARD_ROWS - 1 {
                    ("└", true)
                } else if x == BOARD_COLS - 1 && y == BOARD_ROWS - 1 {
                    ("┘", true)
                } else if x == 0 {
                    ("├", false)
                } else if x == BOARD_COLS - 1 {
                    ("┤", false)
                } else if y == 0 {
                    ("┬", false)
                } else if y == BOARD_ROWS - 1 {
                    ("┴", false)
                } else {
                    ("┼", false)
                };

                let style = if is_corner { corner_style } else { grid_style };
                f.render_widget(Paragraph::new(Span::styled(c, style)), Rect { x: px, y: py, width: 1, height: 1 });

                // Horizontal lines
                if x < BOARD_COLS - 1 && config.cell_width > 1 {
                    for i in 1..config.cell_width {
                        let hx = px + i;
                        f.render_widget(
                            Paragraph::new(Span::styled("─", grid_style)),
                            Rect { x: hx, y: py, width: 1, height: 1 },
                        );
                    }
                }
            }

            // Vertical lines (skip river area)
            if y < BOARD_ROWS - 1 {
                for x in 0..BOARD_COLS {
                    let (px, py) = config.cell_pos(x, y);
                    let px = area.x + px;
                    let py = area.y + py + 1;

                    if y == 4 { continue; } // Skip river

                    f.render_widget(
                        Paragraph::new(Span::styled("│", grid_style)),
                        Rect { x: px, y: py, width: 1, height: 1 },
                    );
                }
            }
        }
    }

    fn draw_river(f: &mut Frame, area: Rect, config: &LayoutConfig) {
        let river_y = area.y + config.cell_height * 5;

        let chu = " 楚河";
        let han = "汉界";

        let river_style = Style::default().fg(C_RIVER).add_modifier(Modifier::BOLD);

        let left_w = 6 * config.cell_width;
        let right_w = 6 * config.cell_width;

        f.render_widget(
            Paragraph::new(chu).style(river_style).alignment(Alignment::Left),
            Rect { x: area.x, y: river_y, width: left_w, height: 1 },
        );

        f.render_widget(
            Paragraph::new(han).style(river_style).alignment(Alignment::Right),
            Rect { x: area.x + (BOARD_COLS as u16) * config.cell_width - right_w, y: river_y, width: right_w, height: 1 },
        );
    }

    fn draw_pieces(f: &mut Frame, area: Rect, game: &Game, config: &LayoutConfig) {
        for (pos, piece) in game.board().pieces() {
            let (px, py) = config.cell_pos(pos.x, pos.y);
            let px = area.x + px;
            let py = area.y + py;

            let fg = match piece.color {
                Color::Red => C_RED_PIECE,
                Color::Black => C_BLACK_PIECE,
            };

            let piece_text = piece.to_string();
            let piece_width = config.cell_width.min(2);

            f.render_widget(
                Paragraph::new(piece_text)
                    .style(Style::default().fg(fg).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center),
                Rect { x: px, y: py, width: piece_width, height: 1 },
            );
        }
    }

    fn draw_cursor_highlight(f: &mut Frame, inner: Rect, cursor: Position, config: &LayoutConfig) {
        let (px, py) = config.cell_pos(cursor.x, cursor.y);
        let px = inner.x + px;
        let py = inner.y + py;
        let w = config.cell_width.min(2);

        f.render_widget(
            Block::default().borders(BORDER_ALL).border_style(
                Style::default().fg(C_CURSOR).add_modifier(Modifier::BOLD)
            ),
            Rect { x: px, y: py, width: w, height: 1 },
        );
    }

    fn draw_selection_highlight(f: &mut Frame, inner: Rect, selected: Position, config: &LayoutConfig) {
        let (px, py) = config.cell_pos(selected.x, selected.y);
        let px = inner.x + px;
        let py = inner.y + py;
        let w = config.cell_width.min(2);

        f.render_widget(
            Block::default().borders(BORDER_ALL).border_style(
                Style::default().fg(C_SELECTION).add_modifier(Modifier::BOLD)
            ),
            Rect { x: px, y: py, width: w, height: 1 },
        );
    }

    fn draw_status(f: &mut Frame, area: Rect, game: &Game, config: &LayoutConfig) {
        // Get move info once to avoid borrowing issues
        let moves = game.get_moves();
        let last_move_str = if moves.is_empty() {
            " 无着法".to_string()
        } else {
            let m = &moves[moves.len() - 1];
            format!(" ({},{})→({},{})", m.from.x, m.from.y, m.to.x, m.to.y)
        };

        if !config.show_full_status || area.height < 2 {
            let turn = match game.turn() {
                Color::Red => "红",
                Color::Black => "黑",
            };
            let text = format!("{}{} 方向键 Enter q:Quit", turn, last_move_str);

            f.render_widget(
                Paragraph::new(text)
                    .block(Block::default().borders(BORDER_ALL).border_style(Style::default().fg(C_SECONDARY)))
                    .alignment(Alignment::Center),
                area,
            );
            return;
        }

        let turn = match game.turn() {
            Color::Red => "●红",
            Color::Black => "●黑",
        };
        let turn_color = match game.turn() {
            Color::Red => C_RED_PIECE,
            Color::Black => C_SECONDARY,
        };

        let help = "Arrows:Move Enter:Select q:Quit r:Restart";

        let spans = vec![
            Span::styled("┈", Style::default().fg(C_SECONDARY)),
            Span::styled(turn, Style::default().fg(turn_color).add_modifier(Modifier::BOLD)),
            Span::styled(last_move_str, Style::default().fg(C_GOLD)),
            Span::styled(" | ", Style::default().fg(C_GRID)),
            Span::styled(help, Style::default().fg(C_ACCENT)),
            Span::styled("┈", Style::default().fg(C_SECONDARY)),
        ];

        f.render_widget(
            Paragraph::new(Line::from(spans))
                .block(Block::default().borders(BORDER_ALL).border_style(Style::default().fg(C_SECONDARY)))
                .alignment(Alignment::Center),
            area,
        );
    }

    pub fn draw_game_over_popup(f: &mut Frame, area: Rect, state: GameState, config: &LayoutConfig) {
        let popup_area = Self::centered_rect(config.popup_width, config.popup_height, area);

        let (text, color) = match state {
            GameState::Checkmate(Color::Red) => ("★ 红方胜利!\nRed Wins!", C_RED_PIECE),
            GameState::Checkmate(Color::Black) => ("★ 黑方胜利!\nBlack Wins!", C_SECONDARY),
            GameState::Stalemate => ("♦ 和棋!\nDraw", C_GOLD),
            GameState::Playing => return,
        };

        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled(text, Style::default().fg(color).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(vec![
                Span::styled("q", Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD)),
                Span::raw(":Quit  "),
                Span::styled("r", Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD)),
                Span::raw(":Restart"),
            ]),
            Line::from(""),
        ];

        f.render_widget(Clear, popup_area);
        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                )
                .alignment(Alignment::Center),
            popup_area,
        );
    }

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
