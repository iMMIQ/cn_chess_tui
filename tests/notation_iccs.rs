use cn_chess_tui::{
    notation::iccs::iccs_to_position, notation::iccs::position_to_iccs, types::Position,
};

#[test]
fn test_position_to_iccs_red_home() {
    // Red's home row (y=9) should map to files a-i and rank 9
    let pos = Position::from_xy(0, 9); // Red chariot, left corner
    assert_eq!(position_to_iccs(pos), "a9");
}

#[test]
fn test_position_to_iccs_black_home() {
    // Black's home row (y=0) should map to files a-i and rank 0
    let pos = Position::from_xy(0, 0); // Black chariot, left corner
    assert_eq!(position_to_iccs(pos), "a0");
}

#[test]
fn test_position_to_iccs_center() {
    let pos = Position::from_xy(4, 5); // Center of board
    assert_eq!(position_to_iccs(pos), "e5");
}

#[test]
fn test_iccs_to_position_roundtrip() {
    let original = Position::from_xy(7, 3);
    let iccs = position_to_iccs(original);
    let recovered = iccs_to_position(&iccs).unwrap();
    assert_eq!(original, recovered);
}

#[test]
fn test_iccs_to_position_invalid_format() {
    assert!(iccs_to_position("z9").is_none()); // Invalid file
    assert!(iccs_to_position("a10").is_none()); // Invalid rank
    assert!(iccs_to_position("abc").is_none()); // Invalid format
}
