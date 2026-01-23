use crate::game::MoveError;
use crate::types::Position;

/// Parse ICCS move string (e.g., "h2e2") into positions
pub fn parse_iccs_move(iccs: &str) -> Result<(Position, Position), MoveError> {
    let chars: Vec<char> = iccs.chars().collect();
    if chars.len() < 4 {
        return Err(MoveError::InvalidMove);
    }

    // Parse file (a-i = 0-8)
    let from_file = (chars[0] as i8) - (b'a' as i8);
    let to_file = (chars[2] as i8) - (b'a' as i8);

    // Parse rank (0-9)
    let from_rank = (chars[1] as i8) - (b'0' as i8) - 1;
    let to_rank = (chars[3] as i8) - (b'0' as i8) - 1;

    if !(0..9).contains(&from_file)
        || !(0..10).contains(&from_rank)
        || !(0..9).contains(&to_file)
        || !(0..10).contains(&to_rank)
    {
        return Err(MoveError::InvalidMove);
    }

    Ok((
        Position::from_xy(from_file as usize, from_rank as usize),
        Position::from_xy(to_file as usize, to_rank as usize),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iccs_move() {
        let (from, to) = parse_iccs_move("h2e2").unwrap();
        assert_eq!(from, Position::from_xy(7, 1));
        assert_eq!(to, Position::from_xy(4, 1));
    }

    #[test]
    fn test_parse_iccs_move_invalid() {
        assert!(parse_iccs_move("abc").is_err());
        assert!(parse_iccs_move("zzzz").is_err());
    }
}
