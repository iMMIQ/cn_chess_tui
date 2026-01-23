use cn_chess_tui::game::{AiMode, GameController};
use cn_chess_tui::types::Position;

#[test]
fn test_controller_new() {
    let controller = GameController::new();
    assert_eq!(controller.ai_mode(), AiMode::Off);
    assert!(!controller.is_engine_thinking());
    assert_eq!(controller.turn(), cn_chess_tui::types::Color::Red);
}

#[test]
fn test_set_ai_mode() {
    let mut controller = GameController::new();
    controller.set_ai_mode(AiMode::PlaysBlack);
    assert_eq!(controller.ai_mode(), AiMode::PlaysBlack);
}

#[test]
fn test_from_fen() {
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let controller = GameController::from_fen(fen).unwrap();
    assert_eq!(controller.turn(), cn_chess_tui::types::Color::Red);
}

#[test]
fn test_human_move() {
    let mut controller = GameController::new();
    let from = Position::from_xy(4, 6); // Red Central Pawn
    let to = Position::from_xy(4, 5);

    // Should succeed
    assert!(controller.human_move(from, to).is_ok());
}

#[test]
fn test_should_ai_move_off() {
    let controller = GameController::new();
    assert!(!controller.is_engine_thinking());
}

#[test]
fn test_should_ai_move_plays_black() {
    let mut controller = GameController::new();
    controller.set_ai_mode(AiMode::PlaysBlack);
    // Red's turn
    assert_eq!(controller.turn(), cn_chess_tui::types::Color::Red);

    // After Red moves, Black's turn
    let _ = controller.human_move(
        Position::from_xy(4, 6), // Red Central Pawn
        Position::from_xy(4, 5),
    );
    assert_eq!(controller.turn(), cn_chess_tui::types::Color::Black);
}
