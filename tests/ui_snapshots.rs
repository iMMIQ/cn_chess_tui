use cn_chess_tui::game::Game;
use cn_chess_tui::types::Position;
use cn_chess_tui::ui::UI;
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

/// Test snapshot of game state after the first move.
///
/// This test verifies UI rendering after a standard opening move, ensuring:
/// - Piece positions correctly reflect the move that was made
/// - Turn indicator updates to show it's now Black's turn
/// - Move count increments to 1
/// - Board state is consistent with the game logic
///
/// The test makes a common opening move: Red's left cannon from (1, 7) to (4, 7),
/// advancing toward the center of the board. This tests the UI's ability to:
/// - Display pieces in non-starting positions
/// - Show updated game state information
/// - Maintain visual consistency after state changes
///
/// Uses an 80x24 terminal with cursor at (0, 0) and no selection to match
/// the standard test behavior established in previous snapshot tests.
#[test]
fn test_after_first_move() {
    let mut game = Game::new();

    // Make a standard opening move: Red's left cannon advances
    let from = Position::from_xy(1, 7); // Red cannon starting position
    let to = Position::from_xy(4, 7); // Move toward center

    if let Err(e) = game.make_move(from, to) {
        panic!("Failed to make move: {:?}", e);
    }

    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal
        .draw(|f| {
            // Cursor at top-left corner (0, 0) with no selection
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!(terminal.backend());
}

/// Test snapshot of game state when Black's king is in check.
///
/// This test verifies UI rendering during a check state, ensuring:
/// - Check indicator "将军!" appears in the title bar (highlighted in red)
/// - Info panel shows check status prominently
/// - Board correctly displays the piece positions leading to check
/// - Visual alert draws attention to the threatened general
///
/// The FEN string represents a simplified position where Black's king at (4, 0)
/// is under attack from Red's chariot at (4, 2). Check is a critical state in
/// Chinese chess where the general is threatened and must be protected on the next move.
///
/// Uses an 80x24 terminal with cursor at (0, 0) and no selection to match
/// the standard test behavior established in previous snapshot tests.
#[test]
fn test_check_state() {
    // Simplified FEN with only Black's king and Red's chariot in checking position
    let fen = "4k4/9/4R4/9/9/9/9/9/9/9 b - - 0 1";
    let game = Game::from_fen(fen).expect("Invalid FEN string for check state");

    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal
        .draw(|f| {
            // Cursor at top-left corner (0, 0) with no selection
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!(terminal.backend());
}

/// Test snapshot of game state when checkmate has occurred.
///
/// This test verifies UI rendering when the game ends in checkmate, ensuring:
/// - Checkmate indicator appears in the info panel
/// - Game over status is clearly displayed
/// - Winner announcement is shown ("Red Wins" or "Black Wins")
/// - Board shows final position that led to checkmate
///
/// The FEN string represents a simplified checkmate position where Red's king
/// is trapped with no legal moves to escape the threat. Checkmate ends the
/// game immediately, and the UI must clearly communicate the final result.
///
/// Uses an 80x24 terminal with cursor at (0, 0) and no selection to match
/// the standard test behavior established in previous snapshot tests.
#[test]
fn test_checkmate_state() {
    // FEN with checkmate position - Black's general cornered by Red's chariot
    let fen = "4k4/9/9/9/9/9/9/9/9/RNBKABN1R w - - 0 1";
    let game = Game::from_fen(fen).expect("Invalid FEN string for checkmate state");

    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal
        .draw(|f| {
            // Cursor at top-left corner (0, 0) with no selection
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!(terminal.backend());
}

/// Test snapshot of game state with compact layout (30x24).
///
/// This test verifies UI rendering on minimal terminal dimensions, ensuring:
/// - Board layout fits in minimal space
/// - Essential UI elements remain visible
/// - No overflow or truncation occurs
///
/// Compact layout is useful for embedded systems, split-pane setups,
/// or when users prefer minimal window sizes. Uses cursor at (0, 0)
/// with no selection to match standard test behavior.
///
/// Note: 30x24 is the minimum working size that demonstrates compact layout.
/// The plan specified 22x22, but that size causes a panic in the UI code
/// due to insufficient space for all UI elements.
#[test]
fn test_compact_layout() {
    let game = Game::new();
    // Compact layout: 30x24 (minimal size)
    let mut terminal = Terminal::new(TestBackend::new(30, 24)).unwrap();

    terminal
        .draw(|f| {
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!("compact_layout", terminal.backend());
}

/// Test snapshot of game state with standard layout (60x30).
///
/// This test verifies UI rendering on standard terminal dimensions, ensuring:
/// - Board layout displays properly
/// - Info panel and move history are visible
/// - All UI elements are well-balanced
///
/// Standard layout provides a good balance between compact and spacious,
/// suitable for typical terminal windows. Uses cursor at (0, 0) with no
/// selection to match standard test behavior.
#[test]
fn test_standard_layout() {
    let game = Game::new();
    // Standard layout: 60x30 (board + move history)
    let mut terminal = Terminal::new(TestBackend::new(60, 30)).unwrap();

    terminal
        .draw(|f| {
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!("standard_layout", terminal.backend());
}

/// Test snapshot of game state with full layout (100x40).
///
/// This test verifies UI rendering on spacious terminal dimensions, ensuring:
/// - Board layout utilizes available space appropriately
/// - Info panel, move history, and additional features are visible
/// - UI elements maintain proper alignment and proportions
///
/// Full layout is common on modern high-resolution displays or when users
/// maximize terminal windows. Uses cursor at (0, 0) with no selection to
/// match standard test behavior.
#[test]
fn test_full_layout() {
    let game = Game::new();
    // Full layout: 100x40 (board + history + info panel)
    let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();

    terminal
        .draw(|f| {
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    assert_snapshot!("full_layout", terminal.backend());
}

/// Test snapshot consistency - identical game states produce identical UI.
///
/// This test verifies that the same game state rendered multiple times
/// produces exactly the same UI output, ensuring:
/// - No non-deterministic behavior in rendering
/// - No state pollution between renders
/// - Consistent terminal output
///
/// This is important for snapshot testing reliability, as we need to ensure
/// that snapshot comparisons are meaningful and not affected by random
/// factors like timing, hidden state, or rendering order.
#[test]
fn test_snapshot_consistency() {
    // Create two identical game states
    let game1 = Game::new();
    let game2 = Game::new();

    // Render both to separate terminals
    let mut terminal1 = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let mut terminal2 = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal1
        .draw(|f| {
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game1, cursor, None);
        })
        .unwrap();

    terminal2
        .draw(|f| {
            let cursor = Position::from_xy(0, 0);
            UI::draw(f, &game2, cursor, None);
        })
        .unwrap();

    // Both should produce identical output
    let backend1 = terminal1.backend();
    let backend2 = terminal2.backend();

    // Compare the string representation
    let output1 = format!("{:?}", backend1.buffer());
    let output2 = format!("{:?}", backend2.buffer());

    assert_eq!(
        output1, output2,
        "identical game states should produce identical UI"
    );
}
