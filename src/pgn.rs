//! PGN (Portable Game Notation) for Chinese Chess
//!
//! PGN format consists of two sections:
//! - Tag section: Key-value pairs in [key "value"] format
//! - Move section: Sequence of moves in ICCS notation
//!
//! Example:
//! ```text
//! [Event "World Championship"]
//! [Site "Beijing"]
//! [Date "2023.01.15"]
//! [Red "Hu Ronghua"]
//! [Black "Liu Dahua"]
//! [Result "1-0"]
//!
//! h2e2 h9g7 h3g3 i9h9
//! ```

use std::fmt::{self, Display, Formatter};

/// A PGN tag pair in the format [key "value"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnTag {
    pub key: String,
    pub value: String,
}

impl PgnTag {
    /// Parse a tag line in the format [key "value"]
    ///
    /// # Examples
    /// ```
    /// use cn_chess_tui::pgn::PgnTag;
    ///
    /// let tag = PgnTag::parse(r#"[Event "World Championship"]"#).unwrap();
    /// assert_eq!(tag.key, "Event");
    /// assert_eq!(tag.value, "World Championship");
    /// ```
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if !line.starts_with('[') || !line.ends_with(']') {
            return None;
        }

        let inner = &line[1..line.len() - 1];
        let parts = split_quoted(inner, ' ')?;

        if parts.len() != 2 {
            return None;
        }

        let key = parts[0].to_string();
        let value = parts[1].to_string();

        // Remove quotes from value if present
        let value = if value.starts_with('"') && value.ends_with('"') {
            value[1..value.len() - 1].to_string()
        } else {
            value
        };

        Some(PgnTag { key, value })
    }

    /// Create a new PgnTag
    #[allow(dead_code)]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Display for PgnTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{} \"{}\"]", self.key, self.value)
    }
}

/// A single move in the PGN move section
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnMove {
    /// The move notation (e.g., "h2e2" in ICCS)
    pub notation: String,
    /// Optional comment after the move
    pub comment: Option<String>,
    /// Move number (for display purposes)
    pub move_number: Option<usize>,
}

impl PgnMove {
    /// Create a new PgnMove
    pub fn new(notation: impl Into<String>) -> Self {
        Self {
            notation: notation.into(),
            comment: None,
            move_number: None,
        }
    }

    /// Add a comment to this move
    #[allow(dead_code)]
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set the move number
    #[allow(dead_code)]
    pub fn with_move_number(mut self, number: usize) -> Self {
        self.move_number = Some(number);
        self
    }
}

impl Display for PgnMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(num) = self.move_number {
            write!(f, "{}. ", num)?;
        }
        write!(f, "{}", self.notation)?;
        if let Some(comment) = &self.comment {
            write!(f, " {{ {}}}", comment)?;
        }
        Ok(())
    }
}

/// Result of a game in PGN format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnGameResult {
    RedWins,
    BlackWins,
    Draw,
    Unknown,
}

impl PgnGameResult {
    /// Parse game result from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "1-0" => Some(PgnGameResult::RedWins),
            "0-1" => Some(PgnGameResult::BlackWins),
            "1/2-1/2" => Some(PgnGameResult::Draw),
            "*" => Some(PgnGameResult::Unknown),
            _ => None,
        }
    }

    /// Convert to PGN string representation
    pub fn to_pgn_string(self) -> &'static str {
        match self {
            PgnGameResult::RedWins => "1-0",
            PgnGameResult::BlackWins => "0-1",
            PgnGameResult::Draw => "1/2-1/2",
            PgnGameResult::Unknown => "*",
        }
    }
}

impl Display for PgnGameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pgn_string())
    }
}

/// A complete PGN game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnGame {
    /// Tag pairs from the tag section
    pub tags: Vec<PgnTag>,
    /// Moves from the move section
    pub moves: Vec<PgnMove>,
    /// Game result
    pub result: PgnGameResult,
}

impl PgnGame {
    /// Create a new empty PGN game
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            moves: Vec::new(),
            result: PgnGameResult::Unknown,
        }
    }

    /// Parse a complete PGN game from a string
    ///
    /// # Examples
    /// ```
    /// use cn_chess_tui::pgn::PgnGame;
    ///
    /// let pgn = r#"[Event "Test Game"]
    /// [Red "Player1"]
    /// [Black "Player2"]
    /// [Result "1-0"]
    ///
    /// h2e2 h9g7 h3g3"#;
    ///
    /// let game = PgnGame::parse(pgn).unwrap();
    /// assert_eq!(game.tags.len(), 4);
    /// assert_eq!(game.moves.len(), 3);
    /// ```
    pub fn parse(text: &str) -> Option<Self> {
        let mut tags = Vec::new();
        let mut moves = Vec::new();
        let mut result = PgnGameResult::Unknown;

        let mut in_tag_section = true;
        let mut move_text = String::new();

        for line in text.lines() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                if in_tag_section && !tags.is_empty() {
                    // Empty line after tags ends tag section
                    in_tag_section = false;
                }
                continue;
            }

            if in_tag_section {
                // Try to parse as a tag
                if let Some(tag) = PgnTag::parse(line) {
                    tags.push(tag);
                } else {
                    // Not a tag, so we're done with tag section
                    in_tag_section = false;
                    move_text.push_str(line);
                    move_text.push(' ');
                }
            } else {
                move_text.push_str(line);
                move_text.push(' ');
            }
        }

        // Extract result from tags or move text
        for tag in &tags {
            if tag.key == "Result" {
                if let Some(parsed_result) = PgnGameResult::parse(&tag.value) {
                    result = parsed_result;
                }
            }
        }

        // Parse moves from move text
        if !move_text.is_empty() {
            moves = parse_moves(&move_text);

            // Check if the last token is a result
            if let Some(last_move) = moves.last() {
                if let Some(parsed_result) = PgnGameResult::parse(&last_move.notation) {
                    result = parsed_result;
                    moves.pop();
                }
            }
        }

        Some(PgnGame { tags, moves, result })
    }

    /// Get a tag value by key
    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.tags.iter().find(|t| t.key == key).map(|t| &t.value)
    }

    /// Set a tag value
    #[allow(dead_code)]
    pub fn set_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value = value.into();

        // Update existing tag or add new one
        if let Some(tag) = self.tags.iter_mut().find(|t| t.key == key) {
            tag.value = value;
        } else {
            self.tags.push(PgnTag::new(key, value));
        }
    }

    /// Add a move to the game
    #[allow(dead_code)]
    pub fn add_move(&mut self, notation: impl Into<String>) {
        let move_num = (self.moves.len() / 2) + 1;
        let pgn_move = PgnMove::new(notation).with_move_number(move_num);
        self.moves.push(pgn_move);
    }

    /// Convert the game to PGN format
    ///
    /// # Examples
    /// ```
    /// use cn_chess_tui::pgn::{PgnGame, PgnGameResult};
    ///
    /// let mut game = PgnGame::new();
    /// game.set_tag("Event", "Test Game");
    /// game.add_move("h2e2");
    /// game.add_move("h9g7");
    /// game.result = PgnGameResult::RedWins;
    ///
    /// let pgn = game.to_pgn();
    /// assert!(pgn.contains("[Event \"Test Game\"]"));
    /// assert!(pgn.contains("h2e2"));
    /// ```
    pub fn to_pgn(&self) -> String {
        let mut output = String::new();

        // Write tags
        for tag in &self.tags {
            output.push_str(&tag.to_string());
            output.push('\n');
        }

        // Empty line between tag section and move section
        if !self.tags.is_empty() && !self.moves.is_empty() {
            output.push('\n');
        }

        // Write moves
        for (i, mv) in self.moves.iter().enumerate() {
            if i > 0 {
                output.push(' ');
            }

            // Add move numbers
            if i % 2 == 0 {
                let move_num = (i / 2) + 1;
                output.push_str(&format!("{}. ", move_num));
            }

            output.push_str(&mv.notation);

            if let Some(comment) = &mv.comment {
                output.push_str(&format!(" {{ {}}}", comment));
            }
        }

        // Write result
        if !self.moves.is_empty() {
            output.push(' ');
        }
        output.push_str(self.result.to_pgn_string());
        output.push('\n');

        output
    }

    /// Get standard Chinese Chess PGN tags
    #[allow(dead_code)]
    pub fn standard_tags() -> Vec<PgnTag> {
        vec![
            PgnTag::new("Event", "?"),
            PgnTag::new("Site", "?"),
            PgnTag::new("Date", "????.??.??"),
            PgnTag::new("Round", "?"),
            PgnTag::new("Red", "?"),
            PgnTag::new("Black", "?"),
            PgnTag::new("Result", "*"),
            PgnTag::new("Time", "?"),
            PgnTag::new("UTCDate", "?"),
            PgnTag::new("UTCTime", "?"),
            PgnTag::new("FEN", ""),
        ]
    }
}

impl Default for PgnGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PgnGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pgn())
    }
}

/// Helper function to split a string by a delimiter, respecting quoted sections
///
/// # Examples
/// ```
/// use cn_chess_tui::pgn::split_quoted;
///
/// let parts = split_quoted(r#"Event "Test Game""#, ' ').unwrap();
/// assert_eq!(parts, vec!["Event", "\"Test Game\""]);
/// ```
pub fn split_quoted(text: &str, delimiter: char) -> Option<Vec<&str>> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let chars = text.chars().enumerate();

    for (i, c) in chars {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == delimiter && !in_quotes {
            let part = text[start..i].trim();
            if !part.is_empty() {
                parts.push(part);
            }
            start = i + 1;
        }
    }

    // Add the last part
    if start < text.len() {
        let part = text[start..].trim();
        if !part.is_empty() {
            parts.push(part);
        }
    }

    if in_quotes {
        return None; // Unclosed quotes
    }

    Some(parts)
}

/// Parse moves from move text, handling comments and move numbers
fn parse_moves(text: &str) -> Vec<PgnMove> {
    let mut moves = Vec::new();
    let mut current_move = String::new();
    let mut in_comment = false;
    let mut current_comment = String::new();

    let chars = text.chars().collect::<Vec<_>>();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if in_comment {
            if c == '}' && (i == 0 || chars[i - 1] != '\\') {
                in_comment = false;
                // Close the comment
            } else {
                current_comment.push(c);
            }
        } else if c == '{' && (i == 0 || chars[i - 1] != '\\') {
            in_comment = true;
            // Save the current move if any
            if !current_move.trim().is_empty() {
                moves.push(PgnMove::new(current_move.trim().to_string()));
                current_move = String::new();
            }
        } else if c.is_whitespace() {
            if !current_move.trim().is_empty() {
                let trimmed = current_move.trim();
                // Skip move numbers (e.g., "1.", "2.")
                if !trimmed.ends_with('.') {
                    moves.push(PgnMove::new(trimmed.to_string()));
                }
                current_move = String::new();
            }
        } else {
            current_move.push(c);
        }

        i += 1;
    }

    // Don't forget the last move
    if !current_move.trim().is_empty() {
        let trimmed = current_move.trim();
        if !trimmed.ends_with('.') {
            moves.push(PgnMove::new(trimmed.to_string()));
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn_tag_parse() {
        let tag = PgnTag::parse(r#"[Event "World Championship"]"#).unwrap();
        assert_eq!(tag.key, "Event");
        assert_eq!(tag.value, "World Championship");

        let tag2 = PgnTag::parse(r#"[Red "Hu Ronghua"]"#).unwrap();
        assert_eq!(tag2.key, "Red");
        assert_eq!(tag2.value, "Hu Ronghua");
    }

    #[test]
    fn test_pgn_tag_display() {
        let tag = PgnTag::new("Event", "World Championship");
        assert_eq!(tag.to_string(), r#"[Event "World Championship"]"#);
    }

    #[test]
    fn test_pgn_game_result_parse() {
        assert_eq!(
            PgnGameResult::parse("1-0"),
            Some(PgnGameResult::RedWins)
        );
        assert_eq!(
            PgnGameResult::parse("0-1"),
            Some(PgnGameResult::BlackWins)
        );
        assert_eq!(
            PgnGameResult::parse("1/2-1/2"),
            Some(PgnGameResult::Draw)
        );
        assert_eq!(PgnGameResult::parse("*"), Some(PgnGameResult::Unknown));
    }

    #[test]
    fn test_pgn_game_parse_simple() {
        let pgn = r#"[Event "Test Game"]
[Red "Player1"]
[Black "Player2"]
[Result "1-0"]

h2e2 h9g7 h3g3"#;

        let game = PgnGame::parse(pgn).unwrap();
        assert_eq!(game.tags.len(), 4);
        assert_eq!(game.moves.len(), 3);
        assert_eq!(game.result, PgnGameResult::RedWins);
    }

    #[test]
    fn test_pgn_game_to_pgn() {
        let mut game = PgnGame::new();
        game.set_tag("Event", "Test Game");
        game.set_tag("Red", "Player1");
        game.set_tag("Black", "Player2");
        game.add_move("h2e2");
        game.add_move("h9g7");
        game.result = PgnGameResult::RedWins;

        let pgn = game.to_pgn();
        assert!(pgn.contains(r#"[Event "Test Game"]"#));
        assert!(pgn.contains(r#"[Red "Player1"]"#));
        assert!(pgn.contains("h2e2"));
        assert!(pgn.contains("h9g7"));
        assert!(pgn.contains("1-0"));
    }

    #[test]
    fn test_split_quoted() {
        let parts = split_quoted(r#"Event "Test Game""#, ' ').unwrap();
        assert_eq!(parts, vec!["Event", "\"Test Game\""]);
    }
}
