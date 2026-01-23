//! Tests for XML conversion functionality

use cn_chess_tui::pgn::{PgnGame, PgnGameResult};
use cn_chess_tui::xml::{escape_xml, pgn_to_xml, save_content, unescape_xml, xml_to_pgn};
use std::fs;

#[test]
fn test_escape_xml_characters() {
    // Test ampersand
    assert_eq!(escape_xml("Tom & Jerry"), "Tom &amp; Jerry");

    // Test less than
    assert_eq!(escape_xml("a < b"), "a &lt; b");

    // Test greater than
    assert_eq!(escape_xml("a > b"), "a &gt; b");

    // Test double quote
    assert_eq!(escape_xml("\"Hello\""), "&quot;Hello&quot;");

    // Test single quote
    assert_eq!(escape_xml("'World'"), "&apos;World&apos;");

    // Test combination
    assert_eq!(
        escape_xml("if (a < b && c > d)"),
        "if (a &lt; b &amp;&amp; c &gt; d)"
    );
}

#[test]
fn test_escape_xml_empty_string() {
    assert_eq!(escape_xml(""), "");
}

#[test]
fn test_escape_xml_no_special_chars() {
    assert_eq!(escape_xml("Hello World"), "Hello World");
}

#[test]
fn test_unescape_xml_entities() {
    // Test ampersand
    assert_eq!(unescape_xml("Tom &amp; Jerry"), "Tom & Jerry");

    // Test less than
    assert_eq!(unescape_xml("a &lt; b"), "a < b");

    // Test greater than
    assert_eq!(unescape_xml("a &gt; b"), "a > b");

    // Test double quote
    assert_eq!(unescape_xml("&quot;Hello&quot;"), "\"Hello\"");

    // Test single quote
    assert_eq!(unescape_xml("&apos;World&apos;"), "'World'");

    // Test combination
    assert_eq!(
        unescape_xml("if (a &lt; b &amp;&amp; c &gt; d)"),
        "if (a < b && c > d)"
    );
}

#[test]
fn test_escape_unescape_roundtrip() {
    let test_strings = vec![
        "Simple text",
        "Tom & Jerry",
        "a < b > c",
        "\"quoted\"",
        "'apostrophe'",
        "if (a < b && c > d) { return \"test\"; }",
        "<tag>content</tag>",
        "A & B < C > D",
    ];

    for original in test_strings {
        let escaped = escape_xml(original);
        let unescaped = unescape_xml(&escaped);
        assert_eq!(
            original, unescaped,
            "Roundtrip failed for: {}",
            original
        );
    }
}

#[test]
fn test_pgn_to_xml_minimal() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Test Game");
    game.add_move("h2e2");
    game.result = PgnGameResult::RedWins;

    let xml = pgn_to_xml(&game);

    // Check XML declaration
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

    // Check root element
    assert!(xml.contains("<pgn>"));
    assert!(xml.contains("</pgn>"));

    // Check tags section
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("</tags>"));
    assert!(xml.contains("<Event>Test Game</Event>"));

    // Check moves section
    assert!(xml.contains("<moves>"));
    assert!(xml.contains("</moves>"));
    assert!(xml.contains("<move>h2e2</move>"));

    // Check result
    assert!(xml.contains("<result>1-0</result>"));
}

#[test]
fn test_pgn_to_xml_complete_game() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "World Championship");
    game.set_tag("Site", "Beijing");
    game.set_tag("Date", "2023.01.15");
    game.set_tag("Round", "1");
    game.set_tag("Red", "Hu Ronghua");
    game.set_tag("Black", "Liu Dahua");
    game.set_tag("Result", "1-0");

    game.add_move("h2e2");
    game.add_move("h9g7");
    game.add_move("h3g3");
    game.add_move("i9h9");

    game.result = PgnGameResult::RedWins;

    let xml = pgn_to_xml(&game);

    // Verify all tags are present
    assert!(xml.contains("<Event>World Championship</Event>"));
    assert!(xml.contains("<Site>Beijing</Site>"));
    assert!(xml.contains("<Date>2023.01.15</Date>"));
    assert!(xml.contains("<Round>1</Round>"));
    assert!(xml.contains("<Red>Hu Ronghua</Red>"));
    assert!(xml.contains("<Black>Liu Dahua</Black>"));
    assert!(xml.contains("<Result>1-0</Result>"));

    // Verify all moves are present
    assert!(xml.contains("<move>h2e2</move>"));
    assert!(xml.contains("<move>h9g7</move>"));
    assert!(xml.contains("<move>h3g3</move>"));
    assert!(xml.contains("<move>i9h9</move>"));

    // Verify result
    assert!(xml.contains("<result>1-0</result>"));
}

#[test]
fn test_pgn_to_xml_with_special_characters() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Tom & Jerry <Championship> 2023");
    game.set_tag("Site", "\"Beijing\" - 'China'");
    game.add_move("h2e2");
    game.result = PgnGameResult::RedWins;

    let xml = pgn_to_xml(&game);

    // Verify special characters are escaped in tags
    assert!(xml.contains("Tom &amp; Jerry &lt;Championship&gt; 2023"));
    assert!(xml.contains("&quot;Beijing&quot; - &apos;China&apos;"));
}

#[test]
fn test_pgn_to_xml_empty_game() {
    let game = PgnGame::new();
    let xml = pgn_to_xml(&game);

    // Should have tags section even if empty
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("</tags>"));

    // Should not have moves section if no moves
    assert!(!xml.contains("<moves>"));

    // Should have unknown result
    assert!(xml.contains("<result>*</result>"));
}

#[test]
fn test_xml_to_pgn_simple() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test Game</Event>
    <Red>Player1</Red>
    <Black>Player2</Black>
  </tags>
  <moves>
    <move>h2e2</move>
    <move>h9g7</move>
  </moves>
  <result>1-0</result>
</pgn>"#;

    let game = xml_to_pgn(xml).expect("Failed to parse XML");

    assert_eq!(game.get_tag("Event"), Some(&"Test Game".to_string()));
    assert_eq!(game.get_tag("Red"), Some(&"Player1".to_string()));
    assert_eq!(game.get_tag("Black"), Some(&"Player2".to_string()));

    assert_eq!(game.moves.len(), 2);
    assert_eq!(game.moves[0].notation, "h2e2");
    assert_eq!(game.moves[1].notation, "h9g7");

    assert_eq!(game.result, PgnGameResult::RedWins);
}

#[test]
fn test_xml_to_pgn_with_escaped_characters() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Tom &amp; Jerry Championship</Event>
    <Site>&quot;Beijing&quot;</Site>
  </tags>
  <moves>
    <move>h2e2</move>
  </moves>
  <result>*</result>
</pgn>"#;

    let game = xml_to_pgn(xml).expect("Failed to parse XML");

    assert_eq!(
        game.get_tag("Event"),
        Some(&"Tom & Jerry Championship".to_string())
    );
    assert_eq!(game.get_tag("Site"), Some(&"\"Beijing\"".to_string()));
}

#[test]
fn test_xml_to_pgn_no_moves() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test</Event>
    <Red>Player1</Red>
  </tags>
  <result>*</result>
</pgn>"#;

    let game = xml_to_pgn(xml).expect("Failed to parse XML");

    assert_eq!(game.get_tag("Event"), Some(&"Test".to_string()));
    assert_eq!(game.get_tag("Red"), Some(&"Player1".to_string()));
    assert_eq!(game.moves.len(), 0);
    assert_eq!(game.result, PgnGameResult::Unknown);
}

#[test]
fn test_xml_to_pgn_all_results() {
    let test_cases = vec![
        ("1-0", PgnGameResult::RedWins),
        ("0-1", PgnGameResult::BlackWins),
        ("1/2-1/2", PgnGameResult::Draw),
        ("*", PgnGameResult::Unknown),
    ];

    for (result_str, expected_result) in test_cases {
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test</Event>
  </tags>
  <result>{}</result>
</pgn>"#,
            result_str
        );

        let game = xml_to_pgn(&xml).expect("Failed to parse XML");
        assert_eq!(
            game.result, expected_result,
            "Failed for result: {}",
            result_str
        );
    }
}

#[test]
fn test_xml_to_pgn_multiple_moves() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test Game</Event>
  </tags>
  <moves>
    <move>h2e2</move>
    <move>h9g7</move>
    <move>h3g3</move>
    <move>i9h9</move>
    <move>b0c2</move>
    <move>b9a7</move>
    <move>c2e4</move>
    <move>c7e5</move>
  </moves>
  <result>*</result>
</pgn>"#;

    let game = xml_to_pgn(xml).expect("Failed to parse XML");

    assert_eq!(game.moves.len(), 8);
    assert_eq!(game.moves[0].notation, "h2e2");
    assert_eq!(game.moves[3].notation, "i9h9");
    assert_eq!(game.moves[7].notation, "c7e5");
}

#[test]
fn test_pgn_xml_roundtrip() {
    let mut original = PgnGame::new();
    original.set_tag("Event", "World Championship");
    original.set_tag("Site", "Beijing");
    original.set_tag("Date", "2023.01.15");
    original.set_tag("Red", "Hu Ronghua");
    original.set_tag("Black", "Liu Dahua");
    original.set_tag("Result", "1-0");

    original.add_move("h2e2");
    original.add_move("h9g7");
    original.add_move("h3g3");
    original.add_move("i9h9");
    original.add_move("b0c2");
    original.add_move("b9a7");

    original.result = PgnGameResult::RedWins;

    // Convert to XML
    let xml = pgn_to_xml(&original);

    // Parse back from XML
    let parsed = xml_to_pgn(&xml).expect("Failed to parse roundtrip XML");

    // Verify all tags match
    assert_eq!(original.tags.len(), parsed.tags.len());
    for tag in &original.tags {
        assert_eq!(
            parsed.get_tag(&tag.key),
            Some(&tag.value),
            "Tag mismatch for key: {}",
            tag.key
        );
    }

    // Verify moves match
    assert_eq!(original.moves.len(), parsed.moves.len());
    for (i, mv) in original.moves.iter().enumerate() {
        assert_eq!(
            parsed.moves[i].notation, mv.notation,
            "Move mismatch at index {}",
            i
        );
    }

    // Verify result matches
    assert_eq!(original.result, parsed.result);
}

#[test]
fn test_pgn_xml_roundtrip_with_special_chars() {
    let mut original = PgnGame::new();
    original.set_tag("Event", "Tom & Jerry <Championship>");
    original.set_tag("Site", "\"Beijing\" - 'China'");
    original.add_move("h2e2");
    original.result = PgnGameResult::RedWins;

    let xml = pgn_to_xml(&original);
    let parsed = xml_to_pgn(&xml).expect("Failed to parse roundtrip XML");

    assert_eq!(
        parsed.get_tag("Event"),
        Some(&"Tom & Jerry <Championship>".to_string())
    );
    assert_eq!(
        parsed.get_tag("Site"),
        Some(&"\"Beijing\" - 'China'".to_string())
    );
}

#[test]
fn test_save_content_to_file() {
    let test_path = "/tmp/test_xml_conversion.txt";
    let content = "Hello, XML conversion test!";

    // Save content
    save_content(test_path, content).expect("Failed to save content");

    // Verify file exists
    assert!(std::path::Path::new(test_path).exists());

    // Verify content
    let read_content = fs::read_to_string(test_path).expect("Failed to read file");
    assert_eq!(read_content, content);

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_save_and_load_xml_roundtrip() {
    let test_path = "/tmp/test_xml_game.xml";

    let mut original = PgnGame::new();
    original.set_tag("Event", "Test Game");
    original.set_tag("Red", "Player1");
    original.set_tag("Black", "Player2");
    original.add_move("h2e2");
    original.add_move("h9g7");
    original.result = PgnGameResult::RedWins;

    // Save to XML file
    let xml = pgn_to_xml(&original);
    save_content(test_path, &xml).expect("Failed to save XML");

    // Load from file
    let loaded_xml = fs::read_to_string(test_path).expect("Failed to read XML");
    let parsed = xml_to_pgn(&loaded_xml).expect("Failed to parse loaded XML");

    // Verify
    assert_eq!(parsed.get_tag("Event"), Some(&"Test Game".to_string()));
    assert_eq!(parsed.get_tag("Red"), Some(&"Player1".to_string()));
    assert_eq!(parsed.get_tag("Black"), Some(&"Player2".to_string()));
    assert_eq!(parsed.moves.len(), 2);
    assert_eq!(parsed.result, PgnGameResult::RedWins);

    // Cleanup
    fs::remove_file(test_path).ok();
}

#[test]
fn test_xml_structure_chinese_chess_format() {
    let mut game = PgnGame::new();
    game.set_tag("Event", "Chinese Chess Championship");
    game.set_tag("Red", "Hu Ronghua");
    game.set_tag("Black", "Liu Dahua");
    game.add_move("h2e2");
    game.add_move("h9g7");
    game.result = PgnGameResult::RedWins;

    let xml = pgn_to_xml(&game);

    // Verify structure follows Chinese Chess standards
    // Root element should be <pgn>
    assert!(xml.contains("<pgn>"));

    // Tags should be in <tags> section
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("</tags>"));

    // Moves should be in <moves> section
    assert!(xml.contains("<moves>"));
    assert!(xml.contains("</moves>"));

    // Individual moves in <move> elements
    assert!(xml.contains("<move>h2e2</move>"));
    assert!(xml.contains("<move>h9g7</move>"));

    // Result in <result> element
    assert!(xml.contains("<result>1-0</result>"));
}

#[test]
fn test_xml_to_pgn_preserves_all_tags() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>World Championship</Event>
    <Site>Beijing</Site>
    <Date>2023.01.15</Date>
    <Round>1</Round>
    <Red>Hu Ronghua</Red>
    <Black>Liu Dahua</Black>
    <Result>1-0</Result>
    <Time>14:30:00</Time>
    <UTCDate>2023.01.15</UTCDate>
    <UTCTime>06:30:00</UTCTime>
  </tags>
  <moves>
    <move>h2e2</move>
  </moves>
  <result>1-0</result>
</pgn>"#;

    let game = xml_to_pgn(xml).expect("Failed to parse XML");

    assert_eq!(game.get_tag("Event"), Some(&"World Championship".to_string()));
    assert_eq!(game.get_tag("Site"), Some(&"Beijing".to_string()));
    assert_eq!(game.get_tag("Date"), Some(&"2023.01.15".to_string()));
    assert_eq!(game.get_tag("Round"), Some(&"1".to_string()));
    assert_eq!(game.get_tag("Red"), Some(&"Hu Ronghua".to_string()));
    assert_eq!(game.get_tag("Black"), Some(&"Liu Dahua".to_string()));
    assert_eq!(game.get_tag("Result"), Some(&"1-0".to_string()));
    assert_eq!(game.get_tag("Time"), Some(&"14:30:00".to_string()));
    assert_eq!(game.get_tag("UTCDate"), Some(&"2023.01.15".to_string()));
    assert_eq!(game.get_tag("UTCTime"), Some(&"06:30:00".to_string()));
}

#[test]
fn test_xml_malformed_missing_closing_tag() {
    // This tests that the parser handles malformed XML gracefully
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test
  </tags>
  <result>*</result>
</pgn>"#;

    // The parser should either return None or parse what it can
    let result = xml_to_pgn(xml);
    // We don't enforce strict error handling for the simplified parser
    // Just verify it doesn't panic
    let _ = result;
}

#[test]
fn test_xml_empty_document() {
    let xml = "";

    let result = xml_to_pgn(xml);
    // Empty document should return None or an empty game
    assert!(result.is_none() || result.unwrap().tags.is_empty());
}
