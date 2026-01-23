//! Tests for PGN (Portable Game Notation) parsing

use cn_chess_tui::pgn::{split_quoted, PgnGame, PgnGameResult, PgnMove, PgnTag};

#[test]
fn test_pgn_tag_parse_simple() {
    let tag = PgnTag::parse(r#"[Event "World Championship"]"#).unwrap();
    assert_eq!(tag.key, "Event");
    assert_eq!(tag.value, "World Championship");
}

#[test]
fn test_pgn_tag_parse_all_standard_tags() {
    let tags = vec![
        ("Event", "World Championship"),
        ("Site", "Beijing"),
        ("Date", "2023.01.15"),
        ("Round", "1"),
        ("Red", "Hu Ronghua"),
        ("Black", "Liu Dahua"),
        ("Result", "1-0"),
        ("Time", "14:30:00"),
        ("UTCDate", "2023.01.15"),
        ("UTCTime", "06:30:00"),
        (
            "FEN",
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1",
        ),
    ];

    for (key, value) in tags {
        let input = format!(r#"[{} "{}"]"#, key, value);
        let tag = PgnTag::parse(&input).unwrap();
        assert_eq!(tag.key, key);
        assert_eq!(tag.value, value);
    }
}

#[test]
fn test_pgn_tag_parse_with_spaces_in_value() {
    let tag = PgnTag::parse(r#"[Event "World Championship 2023"]"#).unwrap();
    assert_eq!(tag.key, "Event");
    assert_eq!(tag.value, "World Championship 2023");
}

#[test]
fn test_pgn_tag_parse_invalid() {
    // Missing closing bracket
    assert!(PgnTag::parse(r#"[Event "Test""#).is_none());

    // Missing opening bracket
    assert!(PgnTag::parse(r#"Event "Test"]"#).is_none());

    // Empty line
    assert!(PgnTag::parse("").is_none());
}

#[test]
fn test_pgn_tag_display() {
    let tag = PgnTag::new("Event", "World Championship");
    assert_eq!(tag.to_string(), r#"[Event "World Championship"]"#);

    let tag2 = PgnTag::new("Red", "Hu Ronghua");
    assert_eq!(tag2.to_string(), r#"[Red "Hu Ronghua"]"#);
}

#[test]
fn test_pgn_game_result_parse() {
    assert_eq!(PgnGameResult::parse("1-0"), Some(PgnGameResult::RedWins));
    assert_eq!(PgnGameResult::parse("0-1"), Some(PgnGameResult::BlackWins));
    assert_eq!(PgnGameResult::parse("1/2-1/2"), Some(PgnGameResult::Draw));
    assert_eq!(PgnGameResult::parse("*"), Some(PgnGameResult::Unknown));
    assert_eq!(PgnGameResult::parse("invalid"), None);
}

#[test]
fn test_pgn_game_result_to_string() {
    assert_eq!(PgnGameResult::RedWins.to_pgn_string(), "1-0");
    assert_eq!(PgnGameResult::BlackWins.to_pgn_string(), "0-1");
    assert_eq!(PgnGameResult::Draw.to_pgn_string(), "1/2-1/2");
    assert_eq!(PgnGameResult::Unknown.to_pgn_string(), "*");
}

#[test]
fn test_pgn_game_parse_minimal() {
    let pgn = r#"[Event "Test"]
[Result "*"]

h2e2"#;

    let game = PgnGame::parse(pgn).unwrap();
    assert_eq!(game.tags.len(), 2);
    assert_eq!(game.moves.len(), 1);
    assert_eq!(game.moves[0].notation, "h2e2");
}

#[test]
fn test_pgn_game_parse_full() {
    let pgn = r#"[Event "World Championship"]
[Site "Beijing"]
[Date "2023.01.15"]
[Round "1"]
[Red "Hu Ronghua"]
[Black "Liu Dahua"]
[Result "1-0"]
[Time "14:30:00"]
[UTCDate "2023.01.15"]
[UTCTime "06:30:00"]
[FEN "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1"]

h2e2 h9g7 h3g3 i9h9 b0c2 b9a7 c2e4 c7e5 g0g3 i9h9
"#;

    let game = PgnGame::parse(pgn).unwrap();
    assert_eq!(game.tags.len(), 11);
    assert_eq!(
        game.get_tag("Event"),
        Some(&"World Championship".to_string())
    );
    assert_eq!(game.get_tag("Red"), Some(&"Hu Ronghua".to_string()));
    assert_eq!(game.get_tag("Black"), Some(&"Liu Dahua".to_string()));
    assert_eq!(game.moves.len(), 10);
    assert_eq!(game.result, PgnGameResult::RedWins);
}

#[test]
fn test_pgn_game_parse_with_result_in_moves() {
    let pgn = r#"[Event "Test Game"]
[Red "Player1"]
[Black "Player2"]

h2e2 h9g7 h3g3 1-0"#;

    let game = PgnGame::parse(pgn).unwrap();
    assert_eq!(game.moves.len(), 3);
    assert_eq!(game.result, PgnGameResult::RedWins);
}

#[test]
fn test_pgn_game_to_pgn() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Test Game");
    game.set_tag("Red", "Player1");
    game.set_tag("Black", "Player2");
    game.set_tag("Result", "1-0");
    game.add_move("h2e2");
    game.add_move("h9g7");
    game.add_move("h3g3");
    game.result = PgnGameResult::RedWins;

    let pgn = game.to_pgn();

    assert!(pgn.contains(r#"[Event "Test Game"]"#));
    assert!(pgn.contains(r#"[Red "Player1"]"#));
    assert!(pgn.contains(r#"[Black "Player2"]"#));
    assert!(pgn.contains(r#"[Result "1-0"]"#));
    assert!(pgn.contains("1. h2e2 h9g7"));
    assert!(pgn.contains("2. h3g3"));
    assert!(pgn.ends_with("1-0\n"));
}

#[test]
fn test_pgn_game_roundtrip() {
    let original_pgn = r#"[Event "Test Game"]
[Site "Online"]
[Date "2023.01.15"]
[Red "Player1"]
[Black "Player2"]
[Result "1-0"]

h2e2 h9g7 h3g3 i9h9
"#;

    let game = PgnGame::parse(original_pgn).unwrap();
    let output_pgn = game.to_pgn();

    // Parse again to verify roundtrip
    let game2 = PgnGame::parse(&output_pgn).unwrap();
    assert_eq!(game.tags, game2.tags);
    assert_eq!(game.moves.len(), game2.moves.len());
    assert_eq!(game.result, game2.result);
}

#[test]
fn test_pgn_game_get_tag() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Championship");
    game.set_tag("Red", "Player1");

    assert_eq!(game.get_tag("Event"), Some(&"Championship".to_string()));
    assert_eq!(game.get_tag("Red"), Some(&"Player1".to_string()));
    assert_eq!(game.get_tag("NonExistent"), None);
}

#[test]
fn test_pgn_game_set_tag_update() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Championship");
    game.set_tag("Event", "World Championship");

    assert_eq!(
        game.get_tag("Event"),
        Some(&"World Championship".to_string())
    );
    assert_eq!(game.tags.len(), 1); // Should not duplicate
}

#[test]
fn test_pgn_game_add_move() {
    let mut game = PgnGame::new();
    game.add_move("h2e2");
    game.add_move("h9g7");
    game.add_move("h3g3");

    assert_eq!(game.moves.len(), 3);
    assert_eq!(game.moves[0].notation, "h2e2");
    assert_eq!(game.moves[1].notation, "h9g7");
    assert_eq!(game.moves[2].notation, "h3g3");
}

#[test]
fn test_pgn_move_with_comment() {
    let mut move_record = PgnMove::new("h2e2");
    move_record = move_record.with_comment("Good opening");

    assert_eq!(move_record.notation, "h2e2");
    assert_eq!(move_record.comment, Some("Good opening".to_string()));
}

#[test]
fn test_pgn_move_with_move_number() {
    let move_record = PgnMove::new("h2e2").with_move_number(1);

    assert_eq!(move_record.notation, "h2e2");
    assert_eq!(move_record.move_number, Some(1));
}

#[test]
fn test_pgn_move_display() {
    let mv = PgnMove::new("h2e2");
    assert_eq!(mv.to_string(), "h2e2");

    let mv_with_num = PgnMove::new("h2e2").with_move_number(1);
    assert_eq!(mv_with_num.to_string(), "1. h2e2");

    let mv_with_comment = PgnMove::new("h2e2").with_comment("Good move");
    assert!(mv_with_comment.to_string().contains("h2e2"));
    assert!(mv_with_comment.to_string().contains("Good move"));
}

#[test]
fn test_split_quoted_simple() {
    let parts = split_quoted(r#"Event "Test Game""#, ' ').unwrap();
    assert_eq!(parts, vec!["Event", "\"Test Game\""]);
}

#[test]
fn test_split_quoted_multiple() {
    let parts = split_quoted(r#"key "value with spaces" another "more values""#, ' ').unwrap();
    assert_eq!(
        parts,
        vec!["key", "\"value with spaces\"", "another", "\"more values\""]
    );
}

#[test]
fn test_split_quoted_unmatched_quotes() {
    assert!(split_quoted(r#"key "value"#, ' ').is_none());
}

#[test]
fn test_pgn_game_standard_tags() {
    let tags = PgnGame::standard_tags();
    assert!(!tags.is_empty());

    let tag_keys: Vec<_> = tags.iter().map(|t| t.key.clone()).collect();
    assert!(tag_keys.contains(&"Event".to_string()));
    assert!(tag_keys.contains(&"Red".to_string()));
    assert!(tag_keys.contains(&"Black".to_string()));
    assert!(tag_keys.contains(&"Result".to_string()));
    assert!(tag_keys.contains(&"FEN".to_string()));
}

#[test]
fn test_pgn_game_empty() {
    let game = PgnGame::new();
    assert_eq!(game.tags.len(), 0);
    assert_eq!(game.moves.len(), 0);
    assert_eq!(game.result, PgnGameResult::Unknown);
}

#[test]
fn test_pgn_game_default() {
    let game = PgnGame::default();
    assert_eq!(game.tags.len(), 0);
    assert_eq!(game.moves.len(), 0);
    assert_eq!(game.result, PgnGameResult::Unknown);
}

#[test]
fn test_pgn_game_parse_no_moves() {
    let pgn = r#"[Event "Test"]
[Red "Player1"]
[Black "Player2"]
[Result "*"]"#;

    let game = PgnGame::parse(pgn).unwrap();
    assert_eq!(game.tags.len(), 4);
    assert_eq!(game.moves.len(), 0);
    assert_eq!(game.result, PgnGameResult::Unknown);
}

#[test]
fn test_pgn_game_parse_multiple_games() {
    let pgn1 = r#"[Event "Game 1"]
[Red "Player1"]
[Black "Player2"]
[Result "1-0"]

h2e2 h9g7
"#;

    let pgn2 = r#"[Event "Game 2"]
[Red "Player3"]
[Black "Player4"]
[Result "0-1"]

h3e3 h9g8
"#;

    let game1 = PgnGame::parse(pgn1).unwrap();
    let game2 = PgnGame::parse(pgn2).unwrap();

    assert_eq!(game1.get_tag("Event"), Some(&"Game 1".to_string()));
    assert_eq!(game1.result, PgnGameResult::RedWins);

    assert_eq!(game2.get_tag("Event"), Some(&"Game 2".to_string()));
    assert_eq!(game2.result, PgnGameResult::BlackWins);
}

#[test]
fn test_pgn_game_with_various_results() {
    let results = vec![
        ("1-0", PgnGameResult::RedWins),
        ("0-1", PgnGameResult::BlackWins),
        ("1/2-1/2", PgnGameResult::Draw),
        ("*", PgnGameResult::Unknown),
    ];

    for (result_str, expected_result) in results {
        let pgn = format!(
            r#"[Event "Test"]
[Result "{}"]

h2e2"#,
            result_str
        );

        let game = PgnGame::parse(&pgn).unwrap();
        assert_eq!(game.result, expected_result);
    }
}

#[test]
fn test_pgn_move_numbering() {
    let mut game = PgnGame::new();
    for i in 1..=6 {
        game.add_move(format!("move{}", i));
    }

    // Check move numbers are assigned correctly
    assert_eq!(game.moves[0].move_number, Some(1)); // Move 1
    assert_eq!(game.moves[1].move_number, Some(1)); // Move 1 (Black's response)
    assert_eq!(game.moves[2].move_number, Some(2)); // Move 2
    assert_eq!(game.moves[3].move_number, Some(2)); // Move 2
    assert_eq!(game.moves[4].move_number, Some(3)); // Move 3
    assert_eq!(game.moves[5].move_number, Some(3)); // Move 3
}

#[test]
fn test_pgn_game_display() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Test");
    game.add_move("h2e2");
    game.result = PgnGameResult::RedWins;

    let display = format!("{}", game);
    assert!(display.contains("[Event \"Test\"]"));
    assert!(display.contains("h2e2"));
    assert!(display.contains("1-0"));
}
