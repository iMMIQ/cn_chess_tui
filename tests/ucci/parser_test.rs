use cn_chess_tui::ucci::parser::parse_response;
use cn_chess_tui::ucci::UcciResponse;

#[test]
fn test_parse_empty_line() {
    let result = parse_response("");
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_command() {
    let result = parse_response("unknown_command");
    assert!(result.is_err());
}

#[test]
fn test_parse_incomplete_id() {
    let result = parse_response("id name");
    assert!(result.is_err());
}

#[test]
fn test_parse_incomplete_bestmove() {
    let result = parse_response("bestmove");
    assert!(result.is_err());
}

#[test]
fn test_parse_complex_info() {
    let resp =
        parse_response("info depth 7 score 4 pv c3c4 h9i7 c2d4 h7e7 h0g2 i9h9 i0h0").unwrap();
    match resp {
        UcciResponse::Info {
            depth, score, pv, ..
        } => {
            assert_eq!(depth, Some(7));
            assert_eq!(score, Some(4));
            assert_eq!(pv.len(), 7);
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn test_parse_info_currmove() {
    let resp = parse_response("info currmove h2e2").unwrap();
    match resp {
        UcciResponse::Info { currmove, .. } => {
            assert_eq!(currmove, Some("h2e2".to_string()));
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn test_parse_info_message() {
    let resp = parse_response("info message analysis complete").unwrap();
    match resp {
        UcciResponse::Info { message, .. } => {
            assert_eq!(message, Some("analysis complete".to_string()));
        }
        _ => panic!("Wrong response type"),
    }
}
