use cn_chess_tui::{Game, Position};

#[test]
fn test_game_to_pgn() {
    let mut game = Game::new();

    // Make a few moves
    // 炮二平五 (Cannon from H7 to E7)
    game.make_move(Position::from_xy(7, 7), Position::from_xy(4, 7)).unwrap();
    // 马８进７ (Horse from H0 to G2)
    game.make_move(Position::from_xy(7, 0), Position::from_xy(6, 2)).unwrap();

    // Convert to PGN
    let pgn_game = game.to_pgn();
    let pgn_string = pgn_game.to_pgn();

    // Verify standard tags are present
    assert!(pgn_string.contains("[Game \"Chinese Chess\"]"));
    assert!(pgn_string.contains("[Red \"?\"]"));
    assert!(pgn_string.contains("[Black \"?\"]"));
    assert!(pgn_string.contains("[Result \"*\"]")); // Game is still playing
    assert!(pgn_string.contains("[Date \"????.??.??\"]"));

    // Verify moves are present in Chinese notation
    assert!(pgn_string.contains("炮二平五"));
    // The second move should contain 马 (Horse)
    assert!(pgn_string.contains("马"));
}

#[test]
fn test_game_to_pgn_with_result() {
    let mut game = Game::new();

    // Make some moves
    game.make_move(Position::from_xy(7, 7), Position::from_xy(4, 7)).unwrap();
    game.make_move(Position::from_xy(7, 0), Position::from_xy(6, 2)).unwrap();

    // Force a checkmate state for testing
    game.force_state_for_testing(cn_chess_tui::GameState::Checkmate(
        cn_chess_tui::Color::Red,
    ));

    // Convert to PGN
    let pgn_game = game.to_pgn();
    let pgn_string = pgn_game.to_pgn();

    // Verify result is set correctly
    assert!(pgn_string.contains("[Result \"1-0\"]")); // Red wins

    // Verify moves are still present
    assert!(pgn_string.contains("炮二平五"));
    // The second move should contain 马 (Horse)
    assert!(pgn_string.contains("马"));
}

#[test]
fn test_game_to_pgn_empty_game() {
    let game = Game::new();

    // Convert to PGN
    let pgn_game = game.to_pgn();
    let pgn_string = pgn_game.to_pgn();

    // Verify tags are present even with no moves
    assert!(pgn_string.contains("[Game \"Chinese Chess\"]"));
    assert!(pgn_string.contains("[Red \"?\"]"));
    assert!(pgn_string.contains("[Black \"?\"]"));
    assert!(pgn_string.contains("[Result \"*\"]"));

    // Verify no moves are present
    // The PGN should not contain move numbers
    assert!(!pgn_string.contains("1."));
}

#[test]
fn test_game_to_pgn_stalemate() {
    let mut game = Game::new();

    // Make a move
    game.make_move(Position::from_xy(7, 7), Position::from_xy(4, 7)).unwrap();

    // Force a stalemate state for testing
    game.force_state_for_testing(cn_chess_tui::GameState::Stalemate);

    // Convert to PGN
    let pgn_game = game.to_pgn();
    let pgn_string = pgn_game.to_pgn();

    // Verify result is set to draw
    assert!(pgn_string.contains("[Result \"1/2-1/2\"]"));

    // Verify move is present
    assert!(pgn_string.contains("炮二平五"));
}
