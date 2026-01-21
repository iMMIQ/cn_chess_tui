use cn_chess_tui::{Game, FenError, fen_to_board, board_to_fen};
use cn_chess_tui::{Color, Position, Piece, PieceType, Board};

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

    assert_eq!(fen, "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1");
}

#[test]
fn test_game_fen_roundtrip() {
    let original_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let game1 = Game::from_fen(original_fen).unwrap();
    let reconstructed_fen = game1.to_fen();

    assert_eq!(original_fen, reconstructed_fen);
}
