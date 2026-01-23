use cn_chess_tui::{Game, Position};

#[test]
fn test_game_move_history_includes_iccs() {
    let mut game = Game::new();

    // Make a move: 炮二平五 (Cannon from H7 to E7)
    let from = Position::from_xy(7, 7);
    let to = Position::from_xy(4, 7);

    assert!(game.make_move(from, to).is_ok());

    // Get move history with ICCS notation
    let moves = game.get_moves_with_iccs();
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], "h7e7");
}

#[test]
fn test_game_multiple_moves_iccs_history() {
    let mut game = Game::new();

    // 炮二平五
    game.make_move(Position::from_xy(7, 7), Position::from_xy(4, 7))
        .unwrap();
    // 马８进７
    game.make_move(Position::from_xy(7, 0), Position::from_xy(6, 2))
        .unwrap();

    let moves = game.get_moves_with_iccs();
    assert_eq!(moves.len(), 2);
    assert_eq!(moves[0], "h7e7");
    assert_eq!(moves[1], "h0g2");
}
