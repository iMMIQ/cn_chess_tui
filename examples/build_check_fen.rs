use cn_chess_tui::Game;

fn main() {
    // Start with initial position
    let game = Game::new();
    println!("Initial FEN: {}", game.to_fen());
    
    // The initial FEN is:
    // rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1
    
    // Let's modify rank 2 (index 2) from "1c5c1" to "9" (all empty)
    // and add a Red chariot at (4, 2)
    // Rank 2 should be "4R4" (4 empty, R, 4 empty)
    
    // Actually, let me just manually create a simple FEN with chariot check
    // Rank 0: rnbakabnr (Black pieces)
    // Rank 1: 9 (empty)
    // Rank 2: 4R4 (Red chariot at file 4)  
    // Ranks 3-8: 9 (empty)
    // Rank 9: RNBAKABNR (Red pieces)
    
    // But wait - that would put Red chariot on Black's side of the river!
    // That's not valid in Chinese chess - pieces can't teleport
    
    // Let me think of a simpler approach. What if I just move a Red chariot 
    // to a position where it checks Black's general?
    
    // Actually, the simplest is: remove all pieces except:
    // - Black's general at (4, 0)
    // - Red's chariot at (4, 2) or similar position that can check
    
    let fen = "4k4/9/4R4/9/9/9/9/9/9/9 b - - 0 1";
    println!("\nTesting simplified check FEN: {}", fen);
    
    match Game::from_fen(fen) {
        Ok(game) => {
            println!("SUCCESS! Is in check: {}", game.is_in_check());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }
}
