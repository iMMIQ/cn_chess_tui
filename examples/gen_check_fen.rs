use cn_chess_tui::{Game, Position};

fn main() {
    // Test checkmate scenario: Black's general cornered
    // General at (4,0), blocked by own pieces and threatened
    let simple_checkmate = "4k4/9/9/9/9/9/9/9/9/RNBKABNR w - - 0 1";
    println!("Testing checkmate FEN: {}", simple_checkmate);

    match Game::from_fen(simple_checkmate) {
        Ok(mut game) => {
            println!("Loaded successfully!");
            println!("Is in check: {}", game.is_in_check());
            println!("Turn: {:?}", game.turn());
            println!("State: {:?}", game.state());

            // Try to see if Red can deliver checkmate
            // Move Red's chariot to checking position
            match game.make_move(Position::from_xy(8, 9), Position::from_xy(8, 0)) {
                Ok(_) => {
                    println!("After Red's chariot moves to check:");
                    println!("New FEN: {}", game.to_fen());
                    println!("Is in check: {}", game.is_in_check());
                    println!("State: {:?}", game.state());
                }
                Err(e) => println!("Move failed: {:?}", e),
            }
        }
        Err(e) => println!("Failed: {:?}", e),
    }
}
