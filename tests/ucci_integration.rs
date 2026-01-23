mod ucci;

use cn_chess_tui::game::Game;
use cn_chess_tui::types::Position;
use cn_chess_tui::ucci::UcciCommand;

#[test]
fn test_ucci_position_with_moves() {
    let mut game = Game::new();
    // Make a simple move - advance red soldier from (0,6) to (0,5)
    game.make_move(Position::from_xy(0, 6), Position::from_xy(0, 5))
        .unwrap();

    let fen_with_moves = game.to_fen_with_moves();

    let cmd = UcciCommand::Position {
        fen: game.to_fen(),
        moves: game.get_moves_with_iccs(),
    };

    let serialized = cmd.serialize();

    // Verify the serialized command contains the expected parts
    assert!(serialized.contains("position fen"));
    assert!(serialized.contains("moves"));

    // Verify the exact move notation "a6a5" is in the serialized output
    assert!(
        serialized.contains("a6a5"),
        "Serialized command should contain move 'a6a5'"
    );

    // Verify format is: "position fen <fen> moves <move1> <move2> ..."
    let parts: Vec<&str> = serialized.split(' ').collect();
    assert_eq!(parts[0], "position");
    assert_eq!(parts[1], "fen");

    // Find where "moves" keyword appears
    let moves_index = parts.iter().position(|&x| x == "moves").unwrap();
    assert!(moves_index > 2); // Should have FEN parts before "moves"

    // Verify we have at least one move after "moves"
    assert!(moves_index + 1 < parts.len());

    // Verify the FEN in the serialized command matches the game's FEN
    assert!(
        serialized.contains(&game.to_fen()),
        "Serialized command should contain the game's FEN"
    );

    println!("FEN with moves: {}", fen_with_moves);
    println!("Serialized command: {}", serialized);
}
