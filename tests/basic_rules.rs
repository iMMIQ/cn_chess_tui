//! Basic rule tests for Chinese Chess
//!
//! Tests verify fundamental game rules including:
//! - Initial position setup
//! - Soldier movement rules
//! - Flying general rule

use cn_chess_tui::board::Board;
use cn_chess_tui::types::{Color, Piece, PieceType, Position};

/// Test that the initial board setup is correct
#[test]
fn test_initial_position_setup() {
    let board = Board::new();

    // Red general should be at (4, 9)
    let red_general_pos = Position::from_xy(4, 9);
    let red_general = board.get(red_general_pos);
    assert!(red_general.is_some(), "Red general should exist at (4, 9)");
    assert_eq!(
        red_general.unwrap().piece_type,
        PieceType::General,
        "Piece at (4, 9) should be a General"
    );
    assert_eq!(
        red_general.unwrap().color,
        Color::Red,
        "General at (4, 9) should be Red"
    );

    // Black general should be at (4, 0)
    let black_general_pos = Position::from_xy(4, 0);
    let black_general = board.get(black_general_pos);
    assert!(black_general.is_some(), "Black general should exist at (4, 0)");
    assert_eq!(
        black_general.unwrap().piece_type,
        PieceType::General,
        "Piece at (4, 0) should be a General"
    );
    assert_eq!(
        black_general.unwrap().color,
        Color::Black,
        "General at (4, 0) should be Black"
    );
}

/// Test that red soldier can move forward
#[test]
fn test_soldier_forward_move_red() {
    let mut board = Board::new();

    // Place a red soldier at (4, 5) - crossed river
    let soldier = Piece::red(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 5), soldier);

    // Should be able to move forward (decreasing Y for red)
    let from = Position::from_xy(4, 5);
    let to = Position::from_xy(4, 4);
    assert!(
        board.is_legal_move(from, to),
        "Red soldier should move forward from (4,5) to (4,4)"
    );
}

/// Test that black soldier can move forward
#[test]
fn test_soldier_forward_move_black() {
    let mut board = Board::new();

    // Place a black soldier at (4, 4) - crossed river
    let soldier = Piece::black(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 4), soldier);

    // Should be able to move forward (increasing Y for black)
    let from = Position::from_xy(4, 4);
    let to = Position::from_xy(4, 5);
    assert!(
        board.is_legal_move(from, to),
        "Black soldier should move forward from (4,4) to (4,5)"
    );
}

/// Test that red soldier cannot move backward
#[test]
fn test_soldier_cannot_move_backward_red() {
    let mut board = Board::new();

    // Place a red soldier at (4, 5)
    let soldier = Piece::red(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 5), soldier);

    // Should NOT be able to move backward (increasing Y for red)
    let from = Position::from_xy(4, 5);
    let to = Position::from_xy(4, 6);
    assert!(
        !board.is_legal_move(from, to),
        "Red soldier should NOT move backward from (4,5) to (4,6)"
    );
}

/// Test that black soldier cannot move backward
#[test]
fn test_soldier_cannot_move_backward_black() {
    let mut board = Board::new();

    // Place a black soldier at (4, 4)
    let soldier = Piece::black(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 4), soldier);

    // Should NOT be able to move backward (decreasing Y for black)
    let from = Position::from_xy(4, 4);
    let to = Position::from_xy(4, 3);
    assert!(
        !board.is_legal_move(from, to),
        "Black soldier should NOT move backward from (4,4) to (4,3)"
    );
}

/// Test the flying general rule - generals cannot face each other directly
#[test]
fn test_flying_general_rule() {
    let mut board = Board::new();

    // Clear the path between generals by removing pieces
    // Let's set up a simple scenario:
    // Red general at (4, 7), Black general at (4, 2) with nothing in between

    // Remove all pieces first
    let all_positions: Vec<_> = board.pieces().map(|(pos, _)| pos).collect();
    for pos in all_positions {
        board.remove_piece(pos);
    }

    // Place generals on the same file with nothing between
    board.place_piece(Position::from_xy(4, 7), Piece::red(PieceType::General));
    board.place_piece(Position::from_xy(4, 2), Piece::black(PieceType::General));

    // Generals should be facing each other
    assert!(
        board.generals_facing(),
        "Generals should be facing each other with no pieces in between"
    );
}

/// Test that a move causing flying general is illegal
#[test]
fn test_flying_general_move_is_illegal() {
    let mut board = Board::new();

    // Set up: Red general at (4, 9), Black general at (4, 0)
    // Remove a piece that's blocking between them (e.g., black soldier at (4, 3))

    // First, let's create a scenario where moving a piece causes flying general
    // Remove the black soldier at (4, 3) that blocks the generals
    board.remove_piece(Position::from_xy(4, 3));

    // Now try to move the red soldier at (4, 6) - this is a legal move
    // that clears the path between generals
    // But wait, we need to ensure the general path is clear first

    // Better approach: Create custom board state
    // Place generals on same file with one piece between
    // Moving that piece should cause flying general

    let mut board = Board::new();

    // Clear all pieces
    let all_positions: Vec<_> = board.pieces().map(|(pos, _)| pos).collect();
    for pos in all_positions {
        board.remove_piece(pos);
    }

    // Place generals
    board.place_piece(Position::from_xy(3, 8), Piece::red(PieceType::General));
    board.place_piece(Position::from_xy(3, 1), Piece::black(PieceType::General));

    // Place a chariot between them
    board.place_piece(Position::from_xy(3, 4), Piece::red(PieceType::Chariot));

    // Moving the chariot away should cause flying general
    // Try moving chariot from (3,4) to (2,4)
    assert!(
        !board.is_legal_move(Position::from_xy(3, 4), Position::from_xy(2, 4)),
        "Moving chariot should not cause flying general"
    );
}

/// Test that soldier can move sideways after crossing river
#[test]
fn test_soldier_sideways_after_crossing_river() {
    let mut board = Board::new();

    // Red soldier at (4, 4) - has crossed river (Y <= 4)
    let soldier = Piece::red(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 4), soldier);

    // Should be able to move sideways
    let from = Position::from_xy(4, 4);
    let to_left = Position::from_xy(3, 4);
    let to_right = Position::from_xy(5, 4);

    assert!(
        board.is_legal_move(from, to_left),
        "Red soldier should move sideways left after crossing river"
    );
    assert!(
        board.is_legal_move(from, to_right),
        "Red soldier should move sideways right after crossing river"
    );
}

/// Test that soldier cannot move sideways before crossing river
#[test]
fn test_soldier_cannot_move_sideways_before_crossing_river() {
    let mut board = Board::new();

    // Red soldier at (4, 6) - has NOT crossed river (Y > 4)
    let soldier = Piece::red(PieceType::Soldier);
    board.place_piece(Position::from_xy(4, 6), soldier);

    // Should NOT be able to move sideways
    let from = Position::from_xy(4, 6);
    let to = Position::from_xy(5, 6);

    assert!(
        !board.is_legal_move(from, to),
        "Red soldier should NOT move sideways before crossing river"
    );
}
