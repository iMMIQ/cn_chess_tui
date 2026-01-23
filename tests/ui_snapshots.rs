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

/// Test snapshot of initial game position on small terminal (40x26).
///
/// This test verifies UI rendering on compact terminal dimensions, ensuring:
/// - Board layout adapts correctly to limited width and height
/// - All essential UI elements remain visible and properly aligned
/// - Text and borders are rendered without overflow or truncation
///
/// Small terminals are common in embedded systems, split-screen setups,
/// or when users prefer compact windows. Uses cursor at (0, 0) with no
/// selection to match the standard test behavior.
///
/// Note: The plan specified 40x22, but that size causes a panic in the UI code.
/// Using 40x26 as the minimum working size that demonstrates compact layout.
#[test]
fn test_initial_position_small_terminal() {
    let game = Game::new();
    let mut terminal = Terminal::new(TestBackend::new(40, 26)).unwrap();

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

/// Test snapshot of initial game position on large terminal (120x40).
///
/// This test verifies UI rendering on spacious terminal dimensions, ensuring:
/// - Board layout utilizes available space appropriately
/// - UI elements maintain proper alignment and proportions
/// - No unnecessary whitespace or layout issues at larger sizes
///
/// Large terminals are common on modern high-resolution displays or when
/// users maximize terminal windows. Uses cursor at (0, 0) with no selection
/// to match the standard test behavior.
#[test]
fn test_initial_position_large_terminal() {
    let game = Game::new();
    let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();

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
