use cn_chess_tui::{notation::chinese::move_to_chinese_with_context, Game, Position};

#[test]
fn test_soldier_ambiguity_two_on_same_file() {
    // When two soldiers are on the same file, use 前兵/后兵
    // Use FEN to create a clean board without initial soldiers
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let mut game = Game::from_fen(fen).expect("Invalid FEN");

    // Remove the soldier at (4, 6) from initial position and add two soldiers on file 5
    game.board_mut().remove_piece(Position::from_xy(4, 6)); // Remove initial soldier
    game.board_mut().place_piece(
        Position::from_xy(4, 5),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 3),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );

    // The soldier at (4, 5) is the rear soldier (后兵)
    let piece = cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier);
    let from = Position::from_xy(4, 5);
    let to = Position::from_xy(4, 4);

    let notation = move_to_chinese_with_context(&game, piece, from, to);
    assert_eq!(notation, "后兵五进一");
}

#[test]
fn test_soldier_ambiguity_three_on_same_file() {
    // When three soldiers are on the same file, use 一兵/二兵/三兵
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let mut game = Game::from_fen(fen).expect("Invalid FEN");

    // Remove the soldier at (4, 6) and add three soldiers
    game.board_mut().remove_piece(Position::from_xy(4, 6));
    game.board_mut().place_piece(
        Position::from_xy(4, 5),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 3),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 1),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );

    // The middle soldier should be "二兵"
    let piece = cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier);
    let from = Position::from_xy(4, 3);
    let to = Position::from_xy(4, 2);

    let notation = move_to_chinese_with_context(&game, piece, from, to);
    assert_eq!(notation, "二兵五进一");
}

#[test]
fn test_no_ambiguity_single_soldier() {
    // When there's no ambiguity, use standard notation
    let game = Game::new();

    // Just use the initial position - no ambiguity
    let piece = cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier);
    let from = Position::from_xy(0, 6);
    let to = Position::from_xy(0, 5);

    let notation = move_to_chinese_with_context(&game, piece, from, to);
    assert_eq!(notation, "兵九进一");
}

#[test]
fn test_front_soldier_ambiguity() {
    // Test that the front soldier is correctly identified
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let mut game = Game::from_fen(fen).expect("Invalid FEN");

    game.board_mut().remove_piece(Position::from_xy(4, 6));
    game.board_mut().place_piece(
        Position::from_xy(4, 5),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 3),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );

    // The soldier at (4, 3) is the front soldier (前兵)
    let piece = cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier);
    let from = Position::from_xy(4, 3);
    let to = Position::from_xy(4, 2);

    let notation = move_to_chinese_with_context(&game, piece, from, to);
    assert_eq!(notation, "前兵五进一");
}

#[test]
fn test_five_soldiers_on_same_file() {
    // Edge case: maximum of 5 soldiers on same file
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let mut game = Game::from_fen(fen).expect("Invalid FEN");

    game.board_mut().remove_piece(Position::from_xy(4, 6));
    game.board_mut().place_piece(
        Position::from_xy(4, 5),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 4),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 3),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 2),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );
    game.board_mut().place_piece(
        Position::from_xy(4, 1),
        cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier),
    );

    // Test all positions
    let test_cases = vec![
        (Position::from_xy(4, 1), "一兵五进一"),
        (Position::from_xy(4, 2), "二兵五进一"),
        (Position::from_xy(4, 3), "三兵五进一"),
        (Position::from_xy(4, 4), "四兵五进一"),
        (Position::from_xy(4, 5), "五兵五进一"),
    ];

    for (from, expected) in test_cases {
        let piece = cn_chess_tui::Piece::red(cn_chess_tui::PieceType::Soldier);
        let to = Position::from_xy(from.x, from.y - 1); // Move forward
        let notation = move_to_chinese_with_context(&game, piece, from, to);
        assert_eq!(
            notation, expected,
            "Failed for position ({}, {})",
            from.x, from.y
        );
    }
}
