use cn_chess_tui::Game;

fn main() {
    let fen = "rnbakabnr/9/4R4/9/9/9/9/9/9/9/9/RNBAKABNR b - - 0 1";
    println!("Testing FEN: {}", fen);
    
    match Game::from_fen(fen) {
        Ok(game) => {
            println!("Success! Is in check: {}", game.is_in_check());
        }
        Err(e) => {
            println!("Failed: {:?}", e);
        }
    }
}
