use cn_chess_tui::fen_to_board;

#[test]
fn test_4k4_fen() {
    let fen = "4k4/9/9/9/9/9/9/9/9/9 w - - 0 1";
    let result = fen_to_board(fen);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
