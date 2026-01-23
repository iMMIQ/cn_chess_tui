use cn_chess_tui::{
    types::{Color, Position},
    notation::chinese::*,
};

#[test]
fn test_red_file_number() {
    // Red's files are numbered 1-9 from RIGHT to LEFT
    // Position x=8 (rightmost) should be file 1 (一)
    assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Red), 1);
    // Position x=4 (center) should be file 5 (五)
    assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Red), 5);
    // Position x=0 (leftmost) should be file 9 (九)
    assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Red), 9);
}

#[test]
fn test_black_file_number() {
    // Black's files are numbered 1-9 from LEFT to RIGHT (their perspective)
    // But from Red's view, we still count left-to-right as 9-1
    // Position x=0 (leftmost from Red's view) is file 1 for Black
    assert_eq!(position_to_file_number(Position::from_xy(0, 5), Color::Black), 1);
    // Position x=4 (center) should be file 5 (五)
    assert_eq!(position_to_file_number(Position::from_xy(4, 5), Color::Black), 5);
    // Position x=8 (rightmost from Red's view) is file 9 for Black
    assert_eq!(position_to_file_number(Position::from_xy(8, 5), Color::Black), 9);
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
    let to = Position::from_xy(4, 7);   // 五 file
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
