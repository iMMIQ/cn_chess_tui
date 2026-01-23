use crate::game::{Game, GameState, AiMode};
use crate::types::{move_to_simple_notation, Color, Position};
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
const C_SELECTION_BG: RColor = RColor::DarkGray;
const C_CHECK: RColor = RColor::LightRed;

// Border styles
const BORDER_ALL: Borders = Borders::ALL;

/// Layout zone types for the new UI
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutZone {
    /// Compact layout - board only with minimal info
    Compact,
    /// Standard layout - board + history
    Standard,
    /// Full layout - board + history + info panel
    Full,
}

/// Responsive layout configuration
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
    pub layout_zone: LayoutZone,
    pub title_height: u16,
    pub help_height: u16,
    pub cell_width: u16,
    pub cell_height: u16,
    pub show_river_text: bool,
    pub popup_width: u16,
    pub popup_height: u16,
}

impl LayoutConfig {
    fn from_terminal_size(size: Rect) -> Self {
        let width = size.width;
        let height = size.height;

        // Determine layout type based on terminal size
        let layout_zone = if width < 80 || height < 26 {
            LayoutZone::Compact
        } else if width < 110 || height < 28 {
            LayoutZone::Standard
        } else {
            LayoutZone::Full
        };

        let title_height = 3;
        let help_height = 3;

        // Cell sizing based on terminal width
        let cell_width = if width >= 100 {
            4
        } else if width >= 70 {
            3
        } else {
            2
        };
        let cell_height = 2;

        let show_river_text = width >= 60;

        let popup_width = (width * 50 / 100).clamp(30, 50);
        let popup_height = (height * 40 / 100).clamp(10, 15);

        LayoutConfig {
            layout_zone,
            title_height,
            help_height,
            cell_width,
            cell_height,
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

/// AI menu selection state
#[derive(Debug, Clone, Copy, Default)]
pub struct AiMenuState {
    pub selected: usize,
    pub show_thinking: bool,
}

pub struct UI;

impl UI {
    pub fn draw(f: &mut Frame, game: &Game, cursor: Position, selection: Option<Position>) {
        let size = f.area();
        let config = LayoutConfig::from_terminal_size(size);

        // Main vertical layout: title + content + help
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(config.title_height),
                Constraint::Min(0),
                Constraint::Length(config.help_height),
            ])
            .split(size);

        // Draw title bar
        Self::draw_title_bar(f, main_chunks[0], game, &config);

        // Draw content area based on layout type
        match config.layout_zone {
            LayoutZone::Compact => {
                Self::draw_compact_layout(f, main_chunks[1], game, cursor, selection, &config);
            }
            LayoutZone::Standard => {
                Self::draw_standard_layout(f, main_chunks[1], game, cursor, selection, &config);
            }
            LayoutZone::Full => {
                Self::draw_full_layout(f, main_chunks[1], game, cursor, selection, &config);
            }
        }

        // Draw help bar
        Self::draw_help_bar(f, main_chunks[2], &config);

        // Draw game over popup if needed
        if game.state() != GameState::Playing {
            Self::draw_game_over_popup(f, size, game.state(), &config);
        }
    }

    /// Compact layout: board with minimal surrounding info
    fn draw_compact_layout(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
        config: &LayoutConfig,
    ) {
        // Split into board + small info panel
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(40), Constraint::Length(20)])
            .split(area);

        Self::draw_board(f, chunks[0], game, cursor, selected, config);
        Self::draw_mini_info(f, chunks[1], game, config);
    }

    /// Standard layout: board + move history
    fn draw_standard_layout(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
        config: &LayoutConfig,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Length(28)])
            .split(area);

        Self::draw_board(f, chunks[0], game, cursor, selected, config);
        Self::draw_move_history(f, chunks[1], game, config);
    }

    /// Full layout: board + history + info panel
    fn draw_full_layout(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
        config: &LayoutConfig,
    ) {
        // Split into board (left) and sidebar (right)
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(55), Constraint::Length(35)])
            .split(area);

        // Split sidebar into history (top) and info (bottom)
        let sidebar_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(12), Constraint::Length(10)])
            .split(horizontal_chunks[1]);

        Self::draw_board(f, horizontal_chunks[0], game, cursor, selected, config);
        Self::draw_move_history(f, sidebar_chunks[0], game, config);
        Self::draw_game_info(f, sidebar_chunks[1], game, config);
    }

    /// Draw the title bar at the top
    fn draw_title_bar(f: &mut Frame, area: Rect, game: &Game, _config: &LayoutConfig) {
        let border_style = Style::default().fg(C_PRIMARY);

        let line1 = vec![
            Span::styled(
                "◆",
                Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " 中国象棋 ",
                Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Chinese Chess ", Style::default().fg(C_ACCENT)),
            Span::styled(
                "◆",
                Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD),
            ),
        ];

        let check_indicator = if game.is_in_check() {
            Span::styled(
                " 将军! ",
                Style::default().fg(C_CHECK).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw("")
        };

        let turn_text = match game.turn() {
            Color::Red => " 红方 ",
            Color::Black => " 黑方 ",
        };
        let turn_style = match game.turn() {
            Color::Red => Style::default()
                .fg(C_RED_PIECE)
                .add_modifier(Modifier::BOLD),
            Color::Black => Style::default()
                .fg(C_BLACK_PIECE)
                .add_modifier(Modifier::BOLD),
        };

        let line2 = vec![
            Span::styled("当前回合: ", Style::default().fg(C_SECONDARY)),
            Span::styled(turn_text, turn_style),
            check_indicator,
            Span::styled(
                format!("着法: {}", game.get_moves().len()),
                Style::default().fg(C_GOLD),
            ),
        ];

        let line3 = vec![
            Span::styled("┈", Style::default().fg(C_GRID)),
            Span::styled(" q:退出 ", Style::default().fg(C_ACCENT)),
            Span::styled(" r:重开 ", Style::default().fg(C_ACCENT)),
            Span::styled(" u:撤销 ", Style::default().fg(C_ACCENT)),
            Span::styled(" 方向键:移动 Enter:选择 ", Style::default().fg(C_SECONDARY)),
            Span::styled("┈", Style::default().fg(C_GRID)),
        ];

        let lines = vec![Line::from(line1), Line::from(line2), Line::from(line3)];

        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(border_style),
                )
                .alignment(Alignment::Center),
            area,
        );
    }

    /// Draw the help bar at the bottom
    fn draw_help_bar(f: &mut Frame, area: Rect, _config: &LayoutConfig) {
        let help_text = vec![
            Line::from(vec![Span::styled(
                " 快捷键 Help ",
                Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled(" ↑↓←→ ", Style::default().fg(C_ACCENT)),
                Span::styled("移动光标  ", Style::default().fg(C_SECONDARY)),
                Span::styled(" Enter ", Style::default().fg(C_ACCENT)),
                Span::styled("选择/确认  ", Style::default().fg(C_SECONDARY)),
                Span::styled(" u ", Style::default().fg(C_ACCENT)),
                Span::styled("撤销  ", Style::default().fg(C_SECONDARY)),
                Span::styled(" r ", Style::default().fg(C_ACCENT)),
                Span::styled("重开  ", Style::default().fg(C_SECONDARY)),
                Span::styled(" q/Esc ", Style::default().fg(C_ACCENT)),
                Span::styled("退出", Style::default().fg(C_SECONDARY)),
            ]),
            Line::from(""),
        ];

        f.render_widget(
            Paragraph::new(help_text)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(C_SECONDARY)),
                )
                .alignment(Alignment::Center),
            area,
        );
    }

    /// Draw the game board
    fn draw_board(
        f: &mut Frame,
        area: Rect,
        game: &Game,
        cursor: Position,
        selected: Option<Position>,
        config: &LayoutConfig,
    ) {
        let board_width = ((BOARD_COLS as u16) * config.cell_width + 2).min(area.width);
        let board_height = ((BOARD_ROWS as u16) * config.cell_height + 2).min(area.height);
        let board_area = Self::centered_rect(board_width, board_height, area);

        let block = Block::default()
            .borders(BORDER_ALL)
            .border_style(Style::default().fg(C_SECONDARY))
            .title(Span::styled(
                " 棋盘 Board ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            ));

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

    /// Draw mini info panel for compact layout
    fn draw_mini_info(f: &mut Frame, area: Rect, game: &Game, _config: &LayoutConfig) {
        let turn = match game.turn() {
            Color::Red => "● 红方",
            Color::Black => "● 黑方",
        };
        let turn_color = match game.turn() {
            Color::Red => C_RED_PIECE,
            Color::Black => C_BLACK_PIECE,
        };

        let check = if game.is_in_check() { "将军!" } else { "" };

        let lines = vec![
            Line::from(vec![Span::styled(
                " 信息 Info ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("回合:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    turn,
                    Style::default().fg(turn_color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("步数:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    format!(" {}", game.get_moves().len()),
                    Style::default().fg(C_GOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                check,
                Style::default().fg(C_CHECK).add_modifier(Modifier::BOLD),
            )]),
        ];

        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(C_SECONDARY)),
                )
                .alignment(Alignment::Left),
            area,
        );
    }

    /// Draw the move history panel
    fn draw_move_history(f: &mut Frame, area: Rect, game: &Game, _config: &LayoutConfig) {
        let moves = game.get_notated_moves();
        let mut move_lines: Vec<Line> = vec![
            Line::from(vec![Span::styled(
                " 着法记录 History ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
        ];

        // Show recent moves with numbering
        let recent_moves: Vec<(usize, String)> = moves
            .iter()
            .enumerate()
            .rev()
            .take(15)
            .map(|(i, (piece, mv))| {
                let notation = move_to_simple_notation(*piece, mv.from, mv.to);
                (i + 1, notation)
            })
            .collect();

        if recent_moves.is_empty() {
            move_lines.push(Line::from(vec![Span::styled(
                "  暂无着法",
                Style::default().fg(C_GRID),
            )]));
        } else {
            for (num, notation) in recent_moves.into_iter().rev() {
                let color = if num % 2 == 1 {
                    C_RED_PIECE // Red moves first (odd numbers)
                } else {
                    C_BLACK_PIECE
                };
                move_lines.push(Line::from(vec![
                    Span::styled(format!("{:2}. ", num), Style::default().fg(C_SECONDARY)),
                    Span::styled(notation, Style::default().fg(color)),
                ]));
            }
        }

        f.render_widget(
            Paragraph::new(move_lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(C_SECONDARY)),
                )
                .alignment(Alignment::Left),
            area,
        );
    }

    /// Draw the game info panel
    fn draw_game_info(f: &mut Frame, area: Rect, game: &Game, _config: &LayoutConfig) {
        let turn = match game.turn() {
            Color::Red => "● 红方",
            Color::Black => "● 黑方",
        };
        let turn_color = match game.turn() {
            Color::Red => C_RED_PIECE,
            Color::Black => C_BLACK_PIECE,
        };

        let check_indicator = if game.is_in_check() {
            "将军!"
        } else {
            "正常"
        };

        let (state_text, state_color) = match game.state() {
            GameState::Playing => ("进行中", C_PRIMARY),
            GameState::Checkmate(c) => {
                if c == Color::Red {
                    ("红胜!", C_RED_PIECE)
                } else {
                    ("黑胜!", C_BLACK_PIECE)
                }
            }
            GameState::Stalemate => ("和棋", C_GOLD),
        };

        let lines = vec![
            Line::from(vec![Span::styled(
                " 游戏信息 Info ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("当前回合:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    turn,
                    Style::default().fg(turn_color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("总步数:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    format!(" {}", game.get_moves().len()),
                    Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("将军状态:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    check_indicator,
                    Style::default().fg(C_CHECK).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("游戏状态:", Style::default().fg(C_SECONDARY)),
                Span::styled(
                    state_text,
                    Style::default()
                        .fg(state_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(C_SECONDARY)),
                )
                .alignment(Alignment::Left),
            area,
        );
    }

    fn draw_grid(f: &mut Frame, area: Rect, config: &LayoutConfig) {
        let grid_style = Style::default().fg(C_GRID);
        let corner_style = Style::default().fg(C_SECONDARY);

        // Calculate how many rows and cols fit in the available area
        let max_rows = (area.height / config.cell_height).min(BOARD_ROWS as u16) as usize;
        let max_cols = (area.width / config.cell_width).min(BOARD_COLS as u16) as usize;

        for y in 0..max_rows {
            for x in 0..max_cols {
                let (px, py) = config.cell_pos(x, y);
                let px = area.x + px;
                let py = area.y + py;

                // Skip if this position is outside the area bounds
                if px >= area.x + area.width || py >= area.y + area.height {
                    continue;
                }

                let (c, is_corner) = if x == 0 && y == 0 {
                    ("┌", true)
                } else if x == max_cols - 1 && y == 0 && max_cols == BOARD_COLS {
                    ("┐", true)
                } else if x == 0 && y == max_rows - 1 && max_rows == BOARD_ROWS {
                    ("└", true)
                } else if x == max_cols - 1
                    && y == max_rows - 1
                    && max_cols == BOARD_COLS
                    && max_rows == BOARD_ROWS
                {
                    ("┘", true)
                } else if x == 0 {
                    ("├", false)
                } else if x == max_cols - 1 && max_cols == BOARD_COLS {
                    ("┤", false)
                } else if y == 0 {
                    ("┬", false)
                } else if y == max_rows - 1 && max_rows == BOARD_ROWS {
                    ("┴", false)
                } else {
                    ("┼", false)
                };

                let style = if is_corner { corner_style } else { grid_style };
                f.render_widget(
                    Paragraph::new(Span::styled(c, style)),
                    Rect {
                        x: px,
                        y: py,
                        width: 1,
                        height: 1,
                    },
                );

                // Horizontal lines
                if x < max_cols - 1 && config.cell_width > 1 {
                    for i in 1..config.cell_width {
                        let hx = px + i;
                        f.render_widget(
                            Paragraph::new(Span::styled("─", grid_style)),
                            Rect {
                                x: hx,
                                y: py,
                                width: 1,
                                height: 1,
                            },
                        );
                    }
                }
            }

            // Vertical lines (skip river area)
            if y < max_rows - 1 {
                for x in 0..max_cols {
                    let (px, py) = config.cell_pos(x, y);
                    let px = area.x + px;
                    let py = area.y + py + 1;

                    if y == 4 {
                        continue;
                    } // Skip river

                    // Skip if this position is outside the area bounds
                    if px >= area.x + area.width || py >= area.y + area.height {
                        continue;
                    }

                    f.render_widget(
                        Paragraph::new(Span::styled("│", grid_style)),
                        Rect {
                            x: px,
                            y: py,
                            width: 1,
                            height: 1,
                        },
                    );
                }
            }
        }
    }

    fn draw_river(f: &mut Frame, area: Rect, config: &LayoutConfig) {
        let river_y = area.y + config.cell_height * 5 - 1;

        // Skip if river is outside area bounds
        if river_y >= area.y + area.height {
            return;
        }

        let chu = " 楚河";
        let han = "汉界";

        let river_style = Style::default().fg(C_RIVER).add_modifier(Modifier::BOLD);

        let left_w = (6 * config.cell_width).min(area.width);
        let right_w = (6 * config.cell_width).min(area.width);

        f.render_widget(
            Paragraph::new(chu)
                .style(river_style)
                .alignment(Alignment::Left),
            Rect {
                x: area.x,
                y: river_y,
                width: left_w,
                height: 1,
            },
        );

        f.render_widget(
            Paragraph::new(han)
                .style(river_style)
                .alignment(Alignment::Right),
            Rect {
                x: area.x + (BOARD_COLS as u16) * config.cell_width - right_w,
                y: river_y,
                width: right_w,
                height: 1,
            },
        );
    }

    fn draw_pieces(f: &mut Frame, area: Rect, game: &Game, config: &LayoutConfig) {
        let max_rows = (area.height / config.cell_height).min(BOARD_ROWS as u16) as usize;
        let max_cols = (area.width / config.cell_width).min(BOARD_COLS as u16) as usize;

        for (pos, piece) in game.board().pieces() {
            // Skip pieces outside the visible grid
            if pos.x >= max_cols || pos.y >= max_rows {
                continue;
            }

            let (px, py) = config.cell_pos(pos.x, pos.y);
            let px = area.x + px;
            let py = area.y + py;

            // Skip if this position is outside the area bounds
            if px >= area.x + area.width || py >= area.y + area.height {
                continue;
            }

            let fg = match piece.color {
                Color::Red => C_RED_PIECE,
                Color::Black => C_BLACK_PIECE,
            };

            let piece_text = piece.to_string();
            let piece_width = config.cell_width.min(3);

            f.render_widget(
                Paragraph::new(piece_text)
                    .style(Style::default().fg(fg).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center),
                Rect {
                    x: px,
                    y: py,
                    width: piece_width,
                    height: 1,
                },
            );
        }
    }

    fn draw_cursor_highlight(f: &mut Frame, inner: Rect, cursor: Position, config: &LayoutConfig) {
        let (px, py) = config.cell_pos(cursor.x, cursor.y);
        let px = inner.x + px;
        let py = inner.y + py;
        let w = config.cell_width.min(3);

        // Skip if outside area bounds
        if px >= inner.x + inner.width || py >= inner.y + inner.height {
            return;
        }

        f.render_widget(
            Block::default()
                .borders(BORDER_ALL)
                .border_style(Style::default().fg(C_CURSOR).add_modifier(Modifier::BOLD)),
            Rect {
                x: px,
                y: py,
                width: w,
                height: 1,
            },
        );
    }

    fn draw_selection_highlight(
        f: &mut Frame,
        inner: Rect,
        selected: Position,
        config: &LayoutConfig,
    ) {
        let (px, py) = config.cell_pos(selected.x, selected.y);
        let px = inner.x + px;
        let py = inner.y + py;
        let w = config.cell_width.min(3);

        // Skip if outside area bounds
        if px >= inner.x + inner.width || py >= inner.y + inner.height {
            return;
        }

        f.render_widget(
            Paragraph::new("")
                .block(
                    Block::default().borders(BORDER_ALL).border_style(
                        Style::default()
                            .fg(C_SELECTION)
                            .add_modifier(Modifier::BOLD),
                    ),
                )
                .style(Style::default().bg(C_SELECTION_BG)),
            Rect {
                x: px,
                y: py,
                width: w,
                height: 1,
            },
        );
    }

    pub fn draw_game_over_popup(
        f: &mut Frame,
        area: Rect,
        state: GameState,
        config: &LayoutConfig,
    ) {
        let popup_area = Self::centered_rect(config.popup_width, config.popup_height, area);

        let (text, color) = match state {
            GameState::Checkmate(Color::Red) => ("★ 红方胜利!\nRed Wins!", C_RED_PIECE),
            GameState::Checkmate(Color::Black) => ("★ 黑方胜利!\nBlack Wins!", C_BLACK_PIECE),
            GameState::Stalemate => ("♦ 和棋!\nDraw", C_GOLD),
            GameState::Playing => return,
        };

        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " q ",
                    Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD),
                ),
                Span::raw(": 退出游戏    "),
                Span::styled(
                    " r ",
                    Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD),
                ),
                Span::raw(": 重新开始"),
            ]),
            Line::from(""),
        ];

        f.render_widget(Clear, popup_area);
        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(BORDER_ALL)
                        .border_style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
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

    /// Draw AI mode selection menu overlay
    pub fn draw_ai_menu(
        f: &mut Frame,
        current_mode: AiMode,
        show_thinking: bool,
        menu_state: &AiMenuState,
    ) {
        let size = f.area();
        let width = 35;
        let height = 11;
        let menu_area = Self::centered_rect(width, height, size);

        let options = vec![
            ("Off (Player vs Player)", AiMode::Off),
            ("AI plays Black", AiMode::PlaysBlack),
            ("AI plays Red", AiMode::PlaysRed),
            ("AI plays Both (spectate)", AiMode::PlaysBoth),
        ];

        let mut lines = vec![
            Line::from(Span::styled(
                " AI Mode Selection ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        for (i, (text, mode)) in options.iter().enumerate() {
            let is_selected = menu_state.selected == i;
            let is_current = *mode == current_mode;

            let prefix = if is_current { "[*] " } else { "[ ] " };
            let style = if is_selected {
                Style::default().fg(C_PRIMARY).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(C_SECONDARY)
            };

            lines.push(Line::from(Span::styled(format!("{}{}", prefix, text), style)));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(if menu_state.show_thinking {
            "[*] Show thinking output"
        } else {
            "[ ] Show thinking output"
        }));
        lines.push(Line::from(""));
        lines.push(Line::from("[↑↓] Navigate  [Enter] Select  [Esc] Cancel"));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(BORDER_ALL)
                    .border_style(Style::default().fg(C_PRIMARY))
                    .style(Style::default().bg(RColor::Black)),
            )
            .alignment(Alignment::Left);

        f.render_widget(Clear, menu_area);
        f.render_widget(paragraph, menu_area);
    }
}
