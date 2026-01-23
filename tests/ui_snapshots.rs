use cn_chess_tui::game::Game;
use cn_chess_tui::ui::UI;
use cn_chess_tui::types::Position;
use insta::assert_snapshot;
use ratatui::{backend::TestBackend, Terminal};

/// Test snapshot of initial game position UI rendering.
///
/// This test captures the complete UI state at game start, including:
/// - Title bar with game name and turn indicator
/// - Full board layout with all pieces in starting positions
/// - River text "楚河 汉界" (Chu River, Han Border)
/// - Info panel showing current turn and move count
/// - Help bar with keyboard shortcuts
///
/// Uses an 80x24 terminal size (standard dimensions) with cursor at
/// (0, 0) (top-left corner) and no selection to verify the default
/// rendering state before any user interaction.
#[test]
fn test_initial_position_ui() {
    let game = Game::new();
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal
        .draw(|f| {
            // Cursor at top-left corner (0, 0) with no selection for initial position
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    // insta automatically captures snapshot and compares to stored version
    assert_snapshot!(terminal.backend());
}
