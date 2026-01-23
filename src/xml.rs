//! XML conversion for PGN (Portable Game Notation)
//!
//! This module provides conversion between PGN format and XML format
//! for Chinese Chess games. The XML format follows Chinese Chess standards
//! with a `<pgn>` root element, tag elements, and move elements.
//!
//! Example XML output:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <pgn>
//!   <tags>
//!     <Event>World Championship</Event>
//!     <Red>Hu Ronghua</Red>
//!     <Black>Liu Dahua</Black>
//!     <Result>1-0</Result>
//!   </tags>
//!   <moves>
//!     <move>h2e2</move>
//!     <move>h9g7</move>
//!     <move>h3g3</move>
//!   </moves>
//! </pgn>
//! ```

use crate::pgn::{PgnGame, PgnGameResult};
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText, BytesDecl};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::fs::File;
use std::io::{Cursor, Write};

/// Convert a PgnGame to XML string format
///
/// This function uses quick-xml's Writer for proper XML serialization.
///
/// # Examples
/// ```
/// use cn_chess_tui::pgn::{PgnGame, PgnGameResult};
/// use cn_chess_tui::xml::pgn_to_xml;
///
/// let mut game = PgnGame::new();
/// game.set_tag("Event", "Test Game");
/// game.set_tag("Red", "Player1");
/// game.add_move("h2e2");
/// game.add_move("h9g7");
/// game.result = PgnGameResult::RedWins;
///
/// let xml = pgn_to_xml(&game);
/// assert!(xml.contains("<Event>Test Game</Event>"));
/// assert!(xml.contains("<move>h2e2</move>"));
/// assert!(xml.contains("<move>h9g7</move>"));
/// ```
pub fn pgn_to_xml(game: &PgnGame) -> String {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    // Write XML declaration
    let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(decl)).unwrap();

    // Write root element <pgn>
    let pgn_start = BytesStart::new("pgn");
    writer.write_event(Event::Start(pgn_start)).unwrap();

    // Write <tags> section
    let tags_start = BytesStart::new("tags");
    writer.write_event(Event::Start(tags_start)).unwrap();

    for tag in &game.tags {
        let tag_start = BytesStart::new(tag.key.as_str());
        writer.write_event(Event::Start(tag_start)).unwrap();

        let text = BytesText::new(tag.value.as_str());
        writer.write_event(Event::Text(text)).unwrap();

        let tag_end = BytesEnd::new(tag.key.as_str());
        writer.write_event(Event::End(tag_end)).unwrap();
    }

    let tags_end = BytesEnd::new("tags");
    writer.write_event(Event::End(tags_end)).unwrap();

    // Write <moves> section if there are moves
    if !game.moves.is_empty() {
        let moves_start = BytesStart::new("moves");
        writer.write_event(Event::Start(moves_start)).unwrap();

        for mv in &game.moves {
            let move_start = BytesStart::new("move");
            writer.write_event(Event::Start(move_start)).unwrap();

            let text = BytesText::new(mv.notation.as_str());
            writer.write_event(Event::Text(text)).unwrap();

            let move_end = BytesEnd::new("move");
            writer.write_event(Event::End(move_end)).unwrap();
        }

        let moves_end = BytesEnd::new("moves");
        writer.write_event(Event::End(moves_end)).unwrap();
    }

    // Write <result>
    let result_start = BytesStart::new("result");
    writer.write_event(Event::Start(result_start)).unwrap();

    let result_text = BytesText::new(game.result.to_pgn_string());
    writer.write_event(Event::Text(result_text)).unwrap();

    let result_end = BytesEnd::new("result");
    writer.write_event(Event::End(result_end)).unwrap();

    // Write root end element </pgn>
    let pgn_end = BytesEnd::new("pgn");
    writer.write_event(Event::End(pgn_end)).unwrap();

    // Extract the written XML
    let result = writer.into_inner();
    String::from_utf8(result.into_inner()).unwrap()
}

/// Convert an XML string to a PgnGame using quick-xml parser
///
/// # Examples
/// ```
/// use cn_chess_tui::pgn::{PgnGame, PgnGameResult};
/// use cn_chess_tui::xml::{pgn_to_xml, xml_to_pgn};
///
/// let mut game = PgnGame::new();
/// game.set_tag("Event", "Test Game");
/// game.set_tag("Red", "Player1");
/// game.add_move("h2e2");
/// game.add_move("h9g7");
/// game.result = PgnGameResult::RedWins;
///
/// let xml = pgn_to_xml(&game);
/// let parsed_game = xml_to_pgn(&xml).unwrap();
///
/// assert_eq!(parsed_game.get_tag("Event"), game.get_tag("Event"));
/// assert_eq!(parsed_game.get_tag("Red"), game.get_tag("Red"));
/// assert_eq!(parsed_game.moves.len(), game.moves.len());
/// ```
pub fn xml_to_pgn(xml: &str) -> Option<PgnGame> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut game = PgnGame::new();
    let mut in_tags = false;
    let mut in_moves = false;
    let mut in_result = false;
    let mut current_tag_name: Option<String> = None;
    let mut current_content = String::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"tags" => in_tags = true,
                    b"moves" => in_moves = true,
                    b"move" => {
                        current_content.clear();
                    }
                    b"result" => {
                        current_content.clear();
                        in_result = true;
                    }
                    _ => {
                        if in_tags {
                            current_tag_name = Some(std::str::from_utf8(e.name().as_ref()).ok()?.to_string());
                            current_content.clear();
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"tags" => in_tags = false,
                    b"moves" => in_moves = false,
                    b"move" => {
                        if in_moves {
                            game.add_move(current_content.trim());
                        }
                        current_content.clear();
                    }
                    b"result" => {
                        game.result = PgnGameResult::parse(current_content.trim())
                            .unwrap_or(PgnGameResult::Unknown);
                        current_content.clear();
                        in_result = false;
                    }
                    b"pgn" => break,
                    _ => {
                        if in_tags {
                            if let (Some(tag_name), false) = (&current_tag_name, current_content.is_empty()) {
                                game.set_tag(tag_name.clone(), current_content.trim().to_string());
                            }
                            current_tag_name = None;
                            current_content.clear();
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if in_tags || in_moves || in_result {
                    current_content.push_str(e.unescape().ok()?.as_ref());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("XML parsing error: {}", e);
                return None;
            }
            _ => {}
        }
        buf.clear();
    }

    Some(game)
}

/// Save content to a file
///
/// # Examples
/// ```no_run
/// use cn_chess_tui::xml::save_content;
///
/// let content = "Hello, World!";
/// save_content("test.txt", content).unwrap();
/// ```
pub fn save_content(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn_to_xml_simple() {
        let mut game = PgnGame::new();
        game.set_tag("Event", "Test Game");
        game.set_tag("Red", "Player1");
        game.add_move("h2e2");
        game.add_move("h9g7");
        game.result = PgnGameResult::RedWins;

        let xml = pgn_to_xml(&game);

        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<pgn>"));
        assert!(xml.contains("<Event>Test Game</Event>"));
        assert!(xml.contains("<Red>Player1</Red>"));
        assert!(xml.contains("<move>h2e2</move>"));
        assert!(xml.contains("<move>h9g7</move>"));
        assert!(xml.contains("<result>1-0</result>"));
        assert!(xml.contains("</pgn>"));
    }

    #[test]
    fn test_pgn_to_xml_with_special_chars() {
        let mut game = PgnGame::new();
        game.set_tag("Event", "Tom & Jerry <Championship>");
        game.set_tag("Site", "\"Beijing\"");
        game.add_move("h2e2");
        game.result = PgnGameResult::RedWins;

        let xml = pgn_to_xml(&game);

        // Check that special characters are escaped
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
        assert!(xml.contains("&quot;"));
    }

    #[test]
    fn test_pgn_to_xml_empty_game() {
        let game = PgnGame::new();
        let xml = pgn_to_xml(&game);

        assert!(xml.contains("<pgn>"));
        assert!(xml.contains("<tags>"));
        assert!(xml.contains("</tags>"));
        assert!(!xml.contains("<moves>")); // No moves section
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

        let game = xml_to_pgn(xml).unwrap();

        assert_eq!(game.get_tag("Event"), Some(&"Test Game".to_string()));
        assert_eq!(game.get_tag("Red"), Some(&"Player1".to_string()));
        assert_eq!(game.get_tag("Black"), Some(&"Player2".to_string()));
        assert_eq!(game.moves.len(), 2);
        assert_eq!(game.moves[0].notation, "h2e2");
        assert_eq!(game.moves[1].notation, "h9g7");
        assert_eq!(game.result, PgnGameResult::RedWins);
    }

    #[test]
    fn test_xml_to_pgn_with_escaped_chars() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Tom &amp; Jerry</Event>
    <Site>&quot;Beijing&quot;</Site>
  </tags>
  <result>*</result>
</pgn>"#;

        let game = xml_to_pgn(xml).unwrap();

        assert_eq!(game.get_tag("Event"), Some(&"Tom & Jerry".to_string()));
        assert_eq!(game.get_tag("Site"), Some(&"\"Beijing\"".to_string()));
    }

    #[test]
    fn test_xml_to_pgn_no_moves() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test</Event>
  </tags>
  <result>*</result>
</pgn>"#;

        let game = xml_to_pgn(xml).unwrap();

        assert_eq!(game.get_tag("Event"), Some(&"Test".to_string()));
        assert_eq!(game.moves.len(), 0);
    }

    #[test]
    fn test_pgn_xml_roundtrip() {
        let mut original = PgnGame::new();
        original.set_tag("Event", "World Championship");
        original.set_tag("Red", "Hu Ronghua");
        original.set_tag("Black", "Liu Dahua");
        original.add_move("h2e2");
        original.add_move("h9g7");
        original.add_move("h3g3");
        original.result = PgnGameResult::RedWins;

        let xml = pgn_to_xml(&original);
        let parsed = xml_to_pgn(&xml).unwrap();

        assert_eq!(original.tags.len(), parsed.tags.len());
        for tag in &original.tags {
            assert_eq!(parsed.get_tag(&tag.key), Some(&tag.value));
        }
        assert_eq!(original.moves.len(), parsed.moves.len());
        assert_eq!(original.result, parsed.result);
    }

    #[test]
    fn test_save_content() {
        use std::fs;
        use std::path::Path;

        let test_path = "/tmp/test_xml_save.txt";
        let content = "Hello, XML!";

        save_content(test_path, content).unwrap();

        assert!(Path::new(test_path).exists());
        let read_content = fs::read_to_string(test_path).unwrap();
        assert_eq!(read_content, content);

        // Cleanup
        fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_xml_to_pgn_all_results() {
        let results = vec![
            ("1-0", PgnGameResult::RedWins),
            ("0-1", PgnGameResult::BlackWins),
            ("1/2-1/2", PgnGameResult::Draw),
            ("*", PgnGameResult::Unknown),
        ];

        for (result_str, expected_result) in results {
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

            let game = xml_to_pgn(&xml).unwrap();
            assert_eq!(game.result, expected_result);
        }
    }

    #[test]
    fn test_xml_to_pgn_multiple_moves() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<pgn>
  <tags>
    <Event>Test</Event>
  </tags>
  <moves>
    <move>h2e2</move>
    <move>h9g7</move>
    <move>h3g3</move>
    <move>i9h9</move>
    <move>b0c2</move>
    <move>b9a7</move>
  </moves>
  <result>*</result>
</pgn>"#;

        let game = xml_to_pgn(xml).unwrap();

        assert_eq!(game.moves.len(), 6);
        assert_eq!(game.moves[0].notation, "h2e2");
        assert_eq!(game.moves[5].notation, "b9a7");
    }
}
