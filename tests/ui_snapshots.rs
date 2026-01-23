use cn_chess_tui::game::Game;
use cn_chess_tui::ui::UI;
use cn_chess_tui::types::Position;
use insta::assert_snapshot;
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_initial_position_ui() {
    let game = Game::new();
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal
        .draw(|f| {
            // Use default cursor at (4, 0) and no selection for initial position
            let cursor = Position::from_xy(4, 0);
            UI::draw(f, &game, cursor, None);
        })
        .unwrap();

    // insta automatically captures snapshot and compares to stored version
    assert_snapshot!(terminal.backend());
}
