use cn_chess_tui::{
    notation::wxf::*,
    types::{Color, Piece, PieceType, Position},
};

#[test]
fn test_piece_to_wxf_letter() {
    // Test all piece types
    assert_eq!(piece_to_wxf_letter(PieceType::General), "K");
    assert_eq!(piece_to_wxf_letter(PieceType::Advisor), "A");
    assert_eq!(piece_to_wxf_letter(PieceType::Elephant), "E");
    assert_eq!(piece_to_wxf_letter(PieceType::Horse), "H");
    assert_eq!(piece_to_wxf_letter(PieceType::Chariot), "R");
    assert_eq!(piece_to_wxf_letter(PieceType::Cannon), "C");
    assert_eq!(piece_to_wxf_letter(PieceType::Soldier), "P");
}

#[test]
fn test_direction_to_wxf() {
    use cn_chess_tui::notation::MovementDirection;
    assert_eq!(direction_to_wxf(MovementDirection::Forward), "+");
    assert_eq!(direction_to_wxf(MovementDirection::Backward), "-");
    assert_eq!(direction_to_wxf(MovementDirection::Horizontal), ".");
}

#[test]
fn test_wxf_letter_to_piece_type() {
    assert_eq!(wxf_letter_to_piece_type("K"), Some(PieceType::General));
    assert_eq!(wxf_letter_to_piece_type("A"), Some(PieceType::Advisor));
    assert_eq!(wxf_letter_to_piece_type("E"), Some(PieceType::Elephant));
    assert_eq!(wxf_letter_to_piece_type("H"), Some(PieceType::Horse));
    assert_eq!(wxf_letter_to_piece_type("R"), Some(PieceType::Chariot));
    assert_eq!(wxf_letter_to_piece_type("C"), Some(PieceType::Cannon));
    assert_eq!(wxf_letter_to_piece_type("P"), Some(PieceType::Soldier));

    // Test invalid letters
    assert_eq!(wxf_letter_to_piece_type("X"), None);
    assert_eq!(wxf_letter_to_piece_type(""), None);
}

#[test]
fn test_move_to_wxf_horizontal() {
    // C2.5: Cannon from file 2 horizontally to file 5
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(7, 7); // File 2 for Red (9-7=2)
    let to = Position::from_xy(4, 7); // File 5 for Red (9-4=5)
    assert_eq!(move_to_wxf(piece, from, to), "C2.5");

    // R5.3: Chariot from file 5 horizontally to file 3
    let piece = Piece::new(PieceType::Chariot, Color::Red);
    let from = Position::from_xy(4, 5); // File 5 for Red (9-4=5)
    let to = Position::from_xy(6, 5); // File 3 for Red (9-6=3)
    assert_eq!(move_to_wxf(piece, from, to), "R5.3");
}

#[test]
fn test_move_to_wxf_forward() {
    // H2+3: Horse from file 2 forward 3 steps
    let piece = Piece::new(PieceType::Horse, Color::Red);
    let from = Position::from_xy(7, 9); // File 2 (9-7=2)
    let to = Position::from_xy(7, 6); // Forward 3 steps
    assert_eq!(move_to_wxf(piece, from, to), "H2+3");

    // P5+1: Soldier from file 5 forward 1 step
    let piece = Piece::new(PieceType::Soldier, Color::Red);
    let from = Position::from_xy(4, 6); // File 5 (9-4=5)
    let to = Position::from_xy(4, 5); // Forward 1 step
    assert_eq!(move_to_wxf(piece, from, to), "P5+1");
}

#[test]
fn test_move_to_wxf_backward() {
    // C5-2: Cannon from file 5 backward 2 steps
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(4, 5); // File 5 (9-4=5)
    let to = Position::from_xy(4, 7); // Backward 2 steps
    assert_eq!(move_to_wxf(piece, from, to), "C5-2");

    // E7-2: Elephant from file 7 backward 2 steps
    let piece = Piece::new(PieceType::Elephant, Color::Red);
    let from = Position::from_xy(2, 5); // File 7 (9-2=7)
    let to = Position::from_xy(2, 7); // Backward 2 steps
    assert_eq!(move_to_wxf(piece, from, to), "E7-2");
}

#[test]
fn test_move_to_wxf_black() {
    // Black pieces use the same format
    // C5.6: Black cannon from file 5 horizontally to file 6
    let piece = Piece::new(PieceType::Cannon, Color::Black);
    let from = Position::from_xy(4, 2); // File 5 for Black (4+1=5)
    let to = Position::from_xy(5, 2); // File 6 for Black (5+1=6)
    assert_eq!(move_to_wxf(piece, from, to), "C5.6");

    // H3+2: Black horse from file 3 forward 2 steps
    let piece = Piece::new(PieceType::Horse, Color::Black);
    let from = Position::from_xy(2, 0); // File 3 for Black (2+1=3)
    let to = Position::from_xy(2, 2); // Forward 2 steps
    assert_eq!(move_to_wxf(piece, from, to), "H3+2");
}

#[test]
fn test_parse_wxf_move() {
    use cn_chess_tui::notation::MovementDirection;

    // Parse horizontal move: C2.5
    let result = parse_wxf_move("C2.5");
    assert_eq!(
        result,
        Some((PieceType::Cannon, 2, MovementDirection::Horizontal, 5))
    );

    // Parse forward move: H2+3
    let result = parse_wxf_move("H2+3");
    assert_eq!(
        result,
        Some((PieceType::Horse, 2, MovementDirection::Forward, 3))
    );

    // Parse backward move: C5-2
    let result = parse_wxf_move("C5-2");
    assert_eq!(
        result,
        Some((PieceType::Cannon, 5, MovementDirection::Backward, 2))
    );

    // Parse all piece types
    assert_eq!(
        parse_wxf_move("K1.2"),
        Some((PieceType::General, 1, MovementDirection::Horizontal, 2))
    );
    assert_eq!(
        parse_wxf_move("A3+1"),
        Some((PieceType::Advisor, 3, MovementDirection::Forward, 1))
    );
    assert_eq!(
        parse_wxf_move("E7-2"),
        Some((PieceType::Elephant, 7, MovementDirection::Backward, 2))
    );
    assert_eq!(
        parse_wxf_move("R9.1"),
        Some((PieceType::Chariot, 9, MovementDirection::Horizontal, 1))
    );
    assert_eq!(
        parse_wxf_move("P4+1"),
        Some((PieceType::Soldier, 4, MovementDirection::Forward, 1))
    );

    // Invalid formats
    assert_eq!(parse_wxf_move(""), None);
    assert_eq!(parse_wxf_move("X2.5"), None); // Invalid piece
    assert_eq!(parse_wxf_move("C2"), None); // Missing destination
    assert_eq!(parse_wxf_move("C2.5.3"), None); // Too many parts
    assert_eq!(parse_wxf_move("C0.5"), None); // Invalid file number
    assert_eq!(parse_wxf_move("C10.5"), None); // Invalid file number
}

#[test]
fn test_roundtrip_wxf() {
    use cn_chess_tui::notation::MovementDirection;

    // Test that we can parse what we generate
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(7, 7);
    let to = Position::from_xy(4, 7);
    let wxf = move_to_wxf(piece, from, to);
    assert_eq!(wxf, "C2.5");

    let parsed = parse_wxf_move(&wxf);
    assert_eq!(
        parsed,
        Some((PieceType::Cannon, 2, MovementDirection::Horizontal, 5))
    );

    // Test forward move
    let piece = Piece::new(PieceType::Horse, Color::Red);
    let from = Position::from_xy(7, 9);
    let to = Position::from_xy(7, 6);
    let wxf = move_to_wxf(piece, from, to);
    assert_eq!(wxf, "H2+3");

    let parsed = parse_wxf_move(&wxf);
    assert_eq!(
        parsed,
        Some((PieceType::Horse, 2, MovementDirection::Forward, 3))
    );

    // Test backward move
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(4, 5);
    let to = Position::from_xy(4, 7);
    let wxf = move_to_wxf(piece, from, to);
    assert_eq!(wxf, "C5-2");

    let parsed = parse_wxf_move(&wxf);
    assert_eq!(
        parsed,
        Some((PieceType::Cannon, 5, MovementDirection::Backward, 2))
    );
}
