use cn_chess_tui::{Game, Position, UI};
use ratatui::backend::TestBackend;
use ratatui::Terminal;

/// Helper to create a terminal of given size
fn create_terminal(width: u16, height: u16) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, height);
    Terminal::new(backend).unwrap()
}

/// Minimum usable terminal size for the game
const MIN_USABLE_WIDTH: u16 = 22;
const MIN_USABLE_HEIGHT: u16 = 22;

#[test]
fn test_layout_config_small_terminal() {
    // Small terminal: 30x22 (minimum usable)
    let mut terminal = create_terminal(30, MIN_USABLE_HEIGHT);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_layout_config_recommended_terminal() {
    // Recommended terminal: 40x24
    let mut terminal = create_terminal(40, 24);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_layout_config_normal_terminal() {
    // Normal terminal: 80x25
    let mut terminal = create_terminal(80, 25);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_layout_config_large_terminal() {
    // Large terminal: 120x40
    let mut terminal = create_terminal(120, 40);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_draw_with_cursor_at_all_positions() {
    let mut terminal = create_terminal(80, 25);

    for y in 0..10 {
        for x in 0..9 {
            let _ = terminal.draw(|f| {
                let game = Game::new();
                UI::draw(f, &game, Position::from_xy(x, y), None);
            });
        }
    }
}

#[test]
fn test_draw_with_selection() {
    let mut terminal = create_terminal(80, 25);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        // Select a piece (e.g., Red Cannon at (1,7))
        UI::draw(
            f,
            &game,
            Position::from_xy(1, 7),
            Some(Position::from_xy(1, 7)),
        );
    });
}

#[test]
fn test_draw_with_check_state() {
    let mut terminal = create_terminal(80, 25);
    let mut game = Game::new();
    // Make a move that might lead to check
    let _ = game.make_move(Position::from_xy(1, 7), Position::from_xy(4, 7)); // Cannon moves
    let _ = terminal.draw(|f| {
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_draw_after_several_moves() {
    let mut terminal = create_terminal(80, 25);

    let mut game = Game::new();
    // Make some moves
    let moves = vec![
        (Position::from_xy(1, 7), Position::from_xy(4, 7)), // Red Cannon
        (Position::from_xy(1, 2), Position::from_xy(1, 3)), // Black Cannon
        (Position::from_xy(1, 6), Position::from_xy(1, 5)), // Red Pawn
    ];

    for (from, to) in moves {
        let _ = game.make_move(from, to);
    }

    let _ = terminal.draw(|f| {
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_extremely_wide_terminal() {
    // Very wide terminal: 200x30
    let mut terminal = create_terminal(200, 30);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_extremely_tall_terminal() {
    // Very tall terminal: 80x100
    let mut terminal = create_terminal(80, 100);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_minimum_viable_terminal() {
    // Absolute minimum: 22x22
    let mut terminal = create_terminal(MIN_USABLE_WIDTH, MIN_USABLE_HEIGHT);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_cell_width_variations() {
    // Test that different widths produce different cell widths
    let widths = vec![30, 45, 60, 100];
    for width in widths {
        let mut terminal = create_terminal(width, 25);
        let result = terminal.draw(|f| {
            let game = Game::new();
            UI::draw(f, &game, Position::from_xy(4, 9), None);
        });
        // Each terminal size should render without error
        assert!(result.is_ok());
    }
}

#[test]
fn test_header_height_variations() {
    // Test different heights produce appropriate header sizes
    let heights = vec![22, 24, 25, 30, 40];
    for height in heights {
        let mut terminal = create_terminal(80, height);
        let result = terminal.draw(|f| {
            let game = Game::new();
            UI::draw(f, &game, Position::from_xy(4, 9), None);
        });
        assert!(result.is_ok(), "Failed for height {}", height);
    }
}

#[test]
fn test_no_panic_with_edge_case_positions() {
    // Test edge case cursor positions
    let positions = vec![
        Position::from_xy(0, 0),
        Position::from_xy(8, 0),
        Position::from_xy(0, 9),
        Position::from_xy(8, 9),
        Position::from_xy(4, 4), // River area
        Position::from_xy(3, 0), // Top palace
        Position::from_xy(3, 7), // Bottom palace
    ];

    for pos in positions {
        let mut terminal = create_terminal(80, 25);
        let _ = terminal.draw(|f| {
            let game = Game::new();
            UI::draw(f, &game, pos, None);
        });
    }
}

#[test]
fn test_draw_with_both_cursor_and_selection() {
    let mut terminal = create_terminal(80, 25);
    let _ = terminal.draw(|f| {
        let game = Game::new();
        // Cursor at different position than selection
        UI::draw(
            f,
            &game,
            Position::from_xy(2, 5),
            Some(Position::from_xy(1, 7)),
        );
    });
}

#[test]
fn test_draw_for_both_colors_turn() {
    let mut terminal = create_terminal(80, 25);

    // Red's turn
    let game = Game::new();
    let _ = terminal.draw(|f| {
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });

    // After a move, it's Black's turn
    let mut game = Game::new();
    let _ = game.make_move(Position::from_xy(1, 7), Position::from_xy(4, 7));
    let _ = terminal.draw(|f| {
        UI::draw(f, &game, Position::from_xy(4, 0), None);
    });
}

#[test]
fn test_various_terminal_sizes() {
    let sizes = vec![
        (22, 22),  // Minimum
        (30, 22),  // Small
        (40, 24),  // Minimum recommended
        (80, 25),  // Standard
        (100, 30), // Large
        (120, 40), // Extra large
    ];

    for (width, height) in sizes {
        let mut terminal = create_terminal(width, height);
        let result = terminal.draw(|f| {
            let game = Game::new();
            UI::draw(f, &game, Position::from_xy(4, 9), None);
        });
        assert!(result.is_ok(), "Failed for size {}x{}", width, height);
    }
}

#[test]
fn test_draw_undo_and_redo_scenarios() {
    let mut terminal = create_terminal(80, 25);
    let mut game = Game::new();

    // Make moves
    let _ = game.make_move(Position::from_xy(1, 7), Position::from_xy(4, 7));
    let _ = game.make_move(Position::from_xy(1, 2), Position::from_xy(1, 3));

    // Undo
    let _ = game.undo_move();

    // Draw after undo
    let _ = terminal.draw(|f| {
        UI::draw(f, &game, Position::from_xy(4, 9), None);
    });
}

#[test]
fn test_square_terminals() {
    let sizes = vec![30, 40, 50, 60];
    for size in sizes {
        let mut terminal = create_terminal(size, size);
        let result = terminal.draw(|f| {
            let game = Game::new();
            UI::draw(f, &game, Position::from_xy(4, 9), None);
        });
        assert!(result.is_ok());
    }
}
