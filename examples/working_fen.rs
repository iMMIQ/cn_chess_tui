use cn_chess_tui::Game;

fn main() {
    // This FEN worked before
    let fen = "rnbakabnr/9/4R4/9/9/9/9/9/9/9/RNBAKABNR b - - 0 1";

    match Game::from_fen(fen) {
        Ok(game) => {
            println!("SUCCESS! Is in check: {}", game.is_in_check());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
}
