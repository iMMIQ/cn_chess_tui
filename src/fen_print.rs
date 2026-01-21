//! Terminal position printing without entering game loop

use crate::board::Board;
use crate::types::Position;

/// Print a board position to stdout using ASCII art
///
/// This function prints a simplified text representation of the board
/// without using the full TUI framework
pub fn print_board_ascii(board: &Board) {
    println!("┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐");

    for y in 0..10 {
        if y > 0 {
            // Print river separator between ranks 4 and 5
            if y == 5 {
                println!("├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤");
                println!("│  楚河  │     │     │     │     │     │     │     │  汉界  │");
                println!("├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤");
            } else {
                println!("├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤");
            }
        }

        print!("│");
        for x in 0..9 {
            let pos = Position::from_xy(x, y);
            match board.get(pos) {
                Some(piece) => {
                    print!("  {}  │", piece);
                }
                None => {
                    print!("     │");
                }
            }
        }
        println!();
    }

    println!("└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_print_board_ascii() {
        let board = Board::new();
        // Just verify it doesn't panic
        print_board_ascii(&board);
    }
}
