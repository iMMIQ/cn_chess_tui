use cn_chess_tui::Game;

fn main() {
    // Test with initial position FEN
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    println!("Testing initial position FEN");
    
    match Game::from_fen(fen) {
        Ok(_game) => {
            println!("SUCCESS! Initial position loaded");
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
}
