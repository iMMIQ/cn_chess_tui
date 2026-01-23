//! Parse UCCI responses from engine output

use crate::ucci::protocol::{OptionType, UcciResponse};

/// Error type for parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat(String),
    UnknownCommand(String),
    InvalidParameter(String),
    MissingRequiredField(String),
}

/// Parse a single line of engine output into a UCCI response
pub fn parse_response(line: &str) -> Result<UcciResponse, ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::InvalidFormat("Empty line".to_string()));
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err(ParseError::InvalidFormat("No parts".to_string()));
    }

    match parts[0] {
        "id" => parse_id(line),
        "option" => parse_option(line),
        "ucciok" => Ok(UcciResponse::UcciOk),
        "readyok" => Ok(UcciResponse::ReadyOk),
        "bestmove" => parse_bestmove(line),
        "nobestmove" => Ok(UcciResponse::NoBestMove),
        "info" => parse_info(line),
        "pophash" => parse_pophash(line),
        "bye" => Ok(UcciResponse::Bye),
        _ => Err(ParseError::UnknownCommand(parts[0].to_string())),
    }
}

fn parse_id(line: &str) -> Result<UcciResponse, ParseError> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err(ParseError::MissingRequiredField("id value".to_string()));
    }
    Ok(UcciResponse::Id {
        field: parts[1].to_string(),
        value: parts[2].to_string(),
    })
}

fn parse_option(line: &str) -> Result<UcciResponse, ParseError> {
    // Format: option <name> type <type> [min <min>] [max <max>] [var <var>...] [default <default>]
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 || parts[2] != "type" {
        return Err(ParseError::InvalidFormat(
            "Invalid option format".to_string(),
        ));
    }

    let name = parts[1].to_string();
    let type_str = parts[3];
    let type_ = match type_str {
        "check" => OptionType::Check,
        "spin" => OptionType::Spin,
        "combo" => OptionType::Combo,
        "button" => OptionType::Button,
        "string" => OptionType::String,
        "label" => OptionType::Label,
        _ => {
            return Err(ParseError::InvalidParameter(format!(
                "Unknown option type: {}",
                type_str
            )))
        }
    };

    let mut min = None;
    let mut max = None;
    let mut vars = Vec::new();
    let mut default = None;

    let mut i = 4;
    while i < parts.len() {
        match parts[i] {
            "min" => {
                if i + 1 < parts.len() {
                    min = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "max" => {
                if i + 1 < parts.len() {
                    max = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "var" => {
                if i + 1 < parts.len() {
                    vars.push(parts[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "default" => {
                if i + 1 < parts.len() {
                    default = Some(parts[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(UcciResponse::Option {
        name,
        type_,
        min,
        max,
        vars,
        default,
    })
}

fn parse_bestmove(line: &str) -> Result<UcciResponse, ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(ParseError::MissingRequiredField("move".to_string()));
    }

    let mv = parts[1].to_string();
    let mut ponder = None;
    let mut draw = false;
    let mut resign = false;

    let mut i = 2;
    while i < parts.len() {
        match parts[i] {
            "ponder" => {
                if i + 1 < parts.len() {
                    ponder = Some(parts[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "draw" => {
                draw = true;
                i += 1;
            }
            "resign" => {
                resign = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(UcciResponse::BestMove {
        mv,
        ponder,
        draw,
        resign,
    })
}

fn parse_info(line: &str) -> Result<UcciResponse, ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let mut time = None;
    let mut nodes = None;
    let mut depth = None;
    let mut score = None;
    let mut pv = Vec::new();
    let mut currmove = None;
    let mut message = None;

    let mut i = 1;
    while i < parts.len() {
        match parts[i] {
            "time" => {
                if i + 1 < parts.len() {
                    time = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "nodes" => {
                if i + 1 < parts.len() {
                    nodes = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "depth" => {
                if i + 1 < parts.len() {
                    depth = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "score" => {
                if i + 1 < parts.len() {
                    score = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "pv" => {
                // Collect remaining parts as PV
                i += 1;
                while i < parts.len() {
                    pv.push(parts[i].to_string());
                    i += 1;
                }
            }
            "currmove" => {
                if i + 1 < parts.len() {
                    currmove = Some(parts[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "message" => {
                // Collect remaining as message
                i += 1;
                if i < parts.len() {
                    message = Some(parts[i..].join(" "));
                    i = parts.len();
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(UcciResponse::Info {
        time,
        nodes,
        depth,
        score,
        pv,
        currmove,
        message,
    })
}

fn parse_pophash(line: &str) -> Result<UcciResponse, ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let mut bestmove = None;
    let mut lowerbound = None;
    let mut upperbound = None;

    let mut i = 1;
    while i < parts.len() {
        match parts[i] {
            "bestmove" => {
                if i + 1 < parts.len() {
                    bestmove = Some(parts[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "lowerbound" => {
                if i + 2 < parts.len() {
                    if let Ok(beta) = parts[i + 1].parse::<i32>() {
                        if let Ok(depth) = parts[i + 2].parse::<u32>() {
                            lowerbound = Some((beta, depth));
                            i += 3;
                            continue;
                        }
                    }
                    i += 1;
                } else {
                    i += 1;
                }
            }
            "upperbound" => {
                if i + 2 < parts.len() {
                    if let Ok(alpha) = parts[i + 1].parse::<i32>() {
                        if let Ok(depth) = parts[i + 2].parse::<u32>() {
                            upperbound = Some((alpha, depth));
                            i += 3;
                            continue;
                        }
                    }
                    i += 1;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(UcciResponse::PopHash {
        bestmove,
        lowerbound,
        upperbound,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ucciok() {
        let resp = parse_response("ucciok").unwrap();
        assert_eq!(resp, UcciResponse::UcciOk);
    }

    #[test]
    fn test_parse_readyok() {
        let resp = parse_response("readyok").unwrap();
        assert_eq!(resp, UcciResponse::ReadyOk);
    }

    #[test]
    fn test_parse_nobestmove() {
        let resp = parse_response("nobestmove").unwrap();
        assert_eq!(resp, UcciResponse::NoBestMove);
    }

    #[test]
    fn test_parse_bye() {
        let resp = parse_response("bye").unwrap();
        assert_eq!(resp, UcciResponse::Bye);
    }

    #[test]
    fn test_parse_id_name() {
        let resp = parse_response("id name ElephantEye 1.6 Beta").unwrap();
        match resp {
            UcciResponse::Id { field, value } => {
                assert_eq!(field, "name");
                assert_eq!(value, "ElephantEye 1.6 Beta");
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_bestmove_simple() {
        let resp = parse_response("bestmove h2e2").unwrap();
        match resp {
            UcciResponse::BestMove {
                mv,
                ponder,
                draw,
                resign,
            } => {
                assert_eq!(mv, "h2e2");
                assert_eq!(ponder, None);
                assert_eq!(draw, false);
                assert_eq!(resign, false);
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_bestmove_with_ponder() {
        let resp = parse_response("bestmove h2e2 ponder h9g7").unwrap();
        match resp {
            UcciResponse::BestMove {
                mv,
                ponder,
                draw,
                resign,
            } => {
                assert_eq!(mv, "h2e2");
                assert_eq!(ponder, Some("h9g7".to_string()));
                assert_eq!(draw, false);
                assert_eq!(resign, false);
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_bestmove_draw() {
        let resp = parse_response("bestmove h2e2 draw").unwrap();
        match resp {
            UcciResponse::BestMove {
                mv, draw, resign, ..
            } => {
                assert_eq!(mv, "h2e2");
                assert_eq!(draw, true);
                assert_eq!(resign, false);
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_info_depth() {
        let resp = parse_response("info depth 6 score 4 pv b0c2 b9c7").unwrap();
        match resp {
            UcciResponse::Info {
                depth, score, pv, ..
            } => {
                assert_eq!(depth, Some(6));
                assert_eq!(score, Some(4));
                assert_eq!(pv, vec!["b0c2", "b9c7"]);
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_info_time_nodes() {
        let resp = parse_response("info time 5000 nodes 5000000").unwrap();
        match resp {
            UcciResponse::Info { time, nodes, .. } => {
                assert_eq!(time, Some(5000));
                assert_eq!(nodes, Some(5000000));
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_option_check() {
        let resp = parse_response("option usemillisec type check default false").unwrap();
        match resp {
            UcciResponse::Option {
                name,
                type_,
                default,
                ..
            } => {
                assert_eq!(name, "usemillisec");
                assert_eq!(type_, OptionType::Check);
                assert_eq!(default, Some("false".to_string()));
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_option_spin() {
        let resp = parse_response("option hashsize type spin min 0 max 2048 default 256").unwrap();
        match resp {
            UcciResponse::Option {
                name,
                type_,
                min,
                max,
                default,
                ..
            } => {
                assert_eq!(name, "hashsize");
                assert_eq!(type_, OptionType::Spin);
                assert_eq!(min, Some(0));
                assert_eq!(max, Some(2048));
                assert_eq!(default, Some("256".to_string()));
            }
            _ => panic!("Wrong response type"),
        }
    }

    #[test]
    fn test_parse_option_combo() {
        let resp = parse_response(
            "option idle type combo var none var small var medium var large default large",
        )
        .unwrap();
        match resp {
            UcciResponse::Option {
                name,
                type_,
                vars,
                default,
                ..
            } => {
                assert_eq!(name, "idle");
                assert_eq!(type_, OptionType::Combo);
                assert_eq!(vars, vec!["none", "small", "medium", "large"]);
                assert_eq!(default, Some("large".to_string()));
            }
            _ => panic!("Wrong response type"),
        }
    }
}
