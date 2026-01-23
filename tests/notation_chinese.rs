use cn_chess_tui::{
    notation::chinese::*,
    types::{Color, Piece, PieceType, Position},
};

#[test]
fn test_red_file_number() {
    // Red's files are numbered 1-9 from RIGHT to LEFT
    // Position x=8 (rightmost) should be file 1 (一)
    assert_eq!(
        position_to_file_number(Position::from_xy(8, 5), Color::Red),
        1
    );
    // Position x=4 (center) should be file 5 (五)
    assert_eq!(
        position_to_file_number(Position::from_xy(4, 5), Color::Red),
        5
    );
    // Position x=0 (leftmost) should be file 9 (九)
    assert_eq!(
        position_to_file_number(Position::from_xy(0, 5), Color::Red),
        9
    );
}

#[test]
fn test_black_file_number() {
    // Black's files are numbered 1-9 from LEFT to RIGHT (their perspective)
    // But from Red's view, we still count left-to-right as 9-1
    // Position x=0 (leftmost from Red's view) is file 1 for Black
    assert_eq!(
        position_to_file_number(Position::from_xy(0, 5), Color::Black),
        1
    );
    // Position x=4 (center) should be file 5 (五)
    assert_eq!(
        position_to_file_number(Position::from_xy(4, 5), Color::Black),
        5
    );
    // Position x=8 (rightmost from Red's view) is file 9 for Black
    assert_eq!(
        position_to_file_number(Position::from_xy(8, 5), Color::Black),
        9
    );
}

#[test]
fn test_file_number_to_chinese() {
    assert_eq!(file_number_to_chinese(1), "一");
    assert_eq!(file_number_to_chinese(5), "五");
    assert_eq!(file_number_to_chinese(9), "九");
}

#[test]
fn test_movement_direction() {
    // 炮二平五: same rank (horizontal movement)
    let from = Position::from_xy(6, 7); // 二 file (Red's perspective, from right)
    let to = Position::from_xy(4, 7); // 五 file
    let dir = get_movement_direction(from, to, Color::Red);
    assert_eq!(dir, MovementDirection::Horizontal);

    // 马二进三: forward (Red moves toward smaller y)
    let from = Position::from_xy(6, 9);
    let to = Position::from_xy(6, 7);
    let dir = get_movement_direction(from, to, Color::Red);
    assert_eq!(dir, MovementDirection::Forward);

    // 仕４进５: forward (Black moves toward larger y)
    let from = Position::from_xy(3, 1);
    let to = Position::from_xy(4, 2);
    let dir = get_movement_direction(from, to, Color::Black);
    assert_eq!(dir, MovementDirection::Forward);
}

#[test]
fn test_piece_to_chinese() {
    // Red pieces
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::General, Color::Red)),
        "帅"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Advisor, Color::Red)),
        "仕"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Elephant, Color::Red)),
        "相"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Horse, Color::Red)),
        "马"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Chariot, Color::Red)),
        "车"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Cannon, Color::Red)),
        "炮"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Soldier, Color::Red)),
        "兵"
    );

    // Black pieces
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::General, Color::Black)),
        "将"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Advisor, Color::Black)),
        "士"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Elephant, Color::Black)),
        "象"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Horse, Color::Black)),
        "马"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Chariot, Color::Black)),
        "车"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Cannon, Color::Black)),
        "炮"
    );
    assert_eq!(
        piece_to_chinese(Piece::new(PieceType::Soldier, Color::Black)),
        "卒"
    );
}

#[test]
fn test_full_move_notation() {
    // 炮二平五: Cannon from file 2 horizontally to file 5
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(7, 7); // File 2 (二) for Red (9-7=2)
    let to = Position::from_xy(4, 7); // File 5 (五) for Red (9-4=5)
    assert_eq!(move_to_chinese(piece, from, to), "炮二平五");

    // 马二进三: Horse from file 2 forward 3 steps
    let piece = Piece::new(PieceType::Horse, Color::Red);
    let from = Position::from_xy(7, 9); // File 2 (二) (9-7=2)
    let to = Position::from_xy(7, 6); // Forward 3 steps (same file)
    assert_eq!(move_to_chinese(piece, from, to), "马二进三");

    // 炮五退二: Cannon from file 5 backward 2 steps
    let piece = Piece::new(PieceType::Cannon, Color::Red);
    let from = Position::from_xy(4, 5); // File 5 (五) (9-4=5)
    let to = Position::from_xy(4, 7); // Backward 2 steps (same file)
    assert_eq!(move_to_chinese(piece, from, to), "炮五退二");

    // Black piece notation
    // 将５平６: Black general from file 5 horizontally to file 6
    let piece = Piece::new(PieceType::General, Color::Black);
    let from = Position::from_xy(4, 0); // File 5 (五) for Black (4+1=5)
    let to = Position::from_xy(5, 0); // File 6 (六) for Black (5+1=6)
    assert_eq!(move_to_chinese(piece, from, to), "将五平六");
}
