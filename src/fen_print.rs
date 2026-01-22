//! Terminal position printing without entering game loop

use crate::board::Board;
use crate::types::{Position, move_to_simple_notation};
use crate::game::Game;

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

/// Print complete game state with FEN, turn, and move history
///
/// This function provides a comprehensive view of the game state
/// useful for debugging, testing, and FEN preview
pub fn print_game_state(game: &Game) {
    println!("FEN: {}", game.to_fen());
    println!("Turn: {} | State: {}", game.turn(), game.state());

    if game.is_in_check() {
        println!("★ CHECK!");
    }

    println!();
    print_board_ascii(game.board());

    // Print move history
    let moves = game.get_notated_moves();
    if !moves.is_empty() {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("            着法记录 Move History");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        for (i, (piece, mv)) in moves.iter().enumerate() {
            let notation = move_to_simple_notation(*piece, mv.from, mv.to);
            println!("  {:2}. {}", i + 1, notation);
        }
    }
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
