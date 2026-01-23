use cn_chess_tui::game::{AiMode, GameController};

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
