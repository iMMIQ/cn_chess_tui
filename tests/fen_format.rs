use cn_chess_tui::{board_to_fen, fen_to_board, FenError, Game};
use cn_chess_tui::{Board, Color, Piece, PieceType, Position};

#[test]
fn test_game_from_fen() {
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let game = Game::from_fen(fen).unwrap();

    assert_eq!(game.turn(), Color::Red);
    assert_eq!(game.state(), cn_chess_tui::GameState::Playing);

    // Check that the board has the correct setup
    let red_general = game.board().get(Position::from_xy(4, 9));
    assert!(red_general.is_some());
    assert_eq!(red_general.unwrap().piece_type, PieceType::General);
}

#[test]
fn test_game_to_fen() {
    let game = Game::new();
    let fen = game.to_fen();

    assert_eq!(
        fen,
        "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1"
    );
}

#[test]
fn test_game_fen_roundtrip() {
    let original_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let game1 = Game::from_fen(original_fen).unwrap();
    let reconstructed_fen = game1.to_fen();

    assert_eq!(original_fen, reconstructed_fen);
}

#[test]
fn test_fen_empty_board() {
    let fen = "9/9/9/9/9/9/9/9/9/9 w - - 0 1";
    let (board, turn) = fen_to_board(fen).unwrap();

    assert_eq!(turn, Color::Red);
    // Board should have no pieces
    let piece_count = board.pieces().count();
    assert_eq!(piece_count, 0);
}

#[test]
fn test_fen_mid_game_position() {
    // A common opening after first move - chariot at (8,9) moved to (8,8)
    // The '1' at the end means the last square (8,9) is empty
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABN1 w - - 0 1";
    let game = Game::from_fen(fen).unwrap();

    // Red Chariot should have moved from (8,9) to (8,8) - indicated by '1' at end
    assert_eq!(game.board().get(Position::from_xy(8, 9)), None);
    // But other chariot should still be at (0,9)
    assert_eq!(
        game.board()
            .get(Position::from_xy(0, 9))
            .unwrap()
            .piece_type,
        PieceType::Chariot
    );
}

#[test]
fn test_fen_all_pieces_on_one_rank() {
    // Edge case: many pieces on one rank
    let mut pieces = std::collections::HashMap::new();
    pieces.insert(
        Position::from_xy(0, 0),
        Piece::new(PieceType::Chariot, Color::Black),
    );
    pieces.insert(
        Position::from_xy(1, 0),
        Piece::new(PieceType::Horse, Color::Black),
    );
    pieces.insert(
        Position::from_xy(2, 0),
        Piece::new(PieceType::Elephant, Color::Black),
    );
    pieces.insert(
        Position::from_xy(3, 0),
        Piece::new(PieceType::Advisor, Color::Black),
    );
    pieces.insert(
        Position::from_xy(4, 0),
        Piece::new(PieceType::General, Color::Black),
    );
    pieces.insert(
        Position::from_xy(5, 0),
        Piece::new(PieceType::Advisor, Color::Black),
    );
    pieces.insert(
        Position::from_xy(6, 0),
        Piece::new(PieceType::Elephant, Color::Black),
    );
    pieces.insert(
        Position::from_xy(7, 0),
        Piece::new(PieceType::Horse, Color::Black),
    );
    pieces.insert(
        Position::from_xy(8, 0),
        Piece::new(PieceType::Chariot, Color::Black),
    );

    let board = Board::from_pieces(pieces);
    let fen = board_to_fen(&board, Color::Black, 0, 1);

    // Should have all 9 pieces on first rank
    assert!(fen.starts_with("rnbakabnr/"));
}

#[test]
fn test_fen_black_turn() {
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR b - - 0 1";
    let game = Game::from_fen(fen).unwrap();

    assert_eq!(game.turn(), Color::Black);
}

#[test]
fn test_fen_alternate_red_notation() {
    // Test that 'r' is also accepted for Red (as mentioned in spec)
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR r - - 0 1";
    let game = Game::from_fen(fen).unwrap();

    assert_eq!(game.turn(), Color::Red);
}

#[test]
fn test_fen_consecutive_empty_squares() {
    // Test various consecutive empty squares
    let fen = "9/9/9/9/9/9/9/9/4K4/9 w - - 0 1";
    let (board, _turn) = fen_to_board(fen).unwrap();

    // Red general at center (4, 8)
    let general = board.get(Position::from_xy(4, 8));
    assert!(general.is_some());
    assert_eq!(general.unwrap().piece_type, PieceType::General);
}

#[test]
fn test_to_fen_preserves_move_count() {
    let mut game = Game::new();
    let fen = game.to_fen();

    // Make a move - soldier at (0, 6) moves forward to (0, 5)
    game.make_move(Position::from_xy(0, 6), Position::from_xy(0, 5))
        .unwrap();
    let fen2 = game.to_fen();

    // FEN should be different after a move
    assert_ne!(fen, fen2);
}

#[test]
fn test_fen_rank_count_validation() {
    // Only 9 ranks instead of 10
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/RNBAKABNR w - - 0 1";
    let result = fen_to_board(fen);

    assert!(matches!(result, Err(FenError::InvalidRankCount)));
}

#[test]
fn test_reconstruct_board_at_move() {
    let mut game = Game::new();

    // Make a few moves
    // Red soldier at (0, 6) moves forward
    game.make_move(Position::from_xy(0, 6), Position::from_xy(0, 5))
        .unwrap();
    // Black soldier at (0, 3) moves forward
    game.make_move(Position::from_xy(0, 3), Position::from_xy(0, 4))
        .unwrap();

    // Reconstruct at move 1 (after first move)
    let (board, turn) = game.reconstruct_board_at_move(1);
    assert_eq!(turn, Color::Black);

    // Board should have piece at the destination
    assert!(board.get(Position::from_xy(0, 5)).is_some());

    // Reconstruct at move 2 (after second move)
    let (board2, turn2) = game.reconstruct_board_at_move(2);
    assert_eq!(turn2, Color::Red);
    assert!(board2.get(Position::from_xy(0, 4)).is_some());
}

#[test]
fn test_reconstruct_board_with_capture() {
    // This test verifies that the bug fix works correctly
    // The bug was placing captured pieces back on their squares
    // We verify that after reconstruction, destination squares have only ONE piece

    let mut game = Game::new();

    // Make simple forward moves
    game.make_move(Position::from_xy(0, 6), Position::from_xy(0, 5))
        .unwrap();
    game.make_move(Position::from_xy(0, 3), Position::from_xy(0, 4))
        .unwrap();
    game.make_move(Position::from_xy(0, 5), Position::from_xy(0, 4))
        .unwrap(); // capture!

    // Reconstruct after capture
    let (board, turn) = game.reconstruct_board_at_move(3);

    // The destination should have ONE piece (the capturing Red soldier)
    let piece = board.get(Position::from_xy(0, 4));
    assert!(piece.is_some(), "Destination square should have a piece");

    // Should be Red's piece that captured
    assert_eq!(piece.unwrap().color, Color::Red);

    // The starting position should be empty
    assert!(
        board.get(Position::from_xy(0, 5)).is_none(),
        "Starting square should be empty after move"
    );

    // Verify turn is correct
    assert_eq!(turn, Color::Black);
}
