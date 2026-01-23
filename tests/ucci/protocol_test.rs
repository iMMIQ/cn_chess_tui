use cn_chess_tui::ucci::{GoMode, UcciCommand, UcciResponse};

#[test]
fn test_ucci_command_equality() {
    let cmd1 = UcciCommand::Ucci;
    let cmd2 = UcciCommand::Ucci;
    assert_eq!(cmd1, cmd2);
}

#[test]
fn test_go_mode_depth() {
    let mode = GoMode::Depth(10);
    assert_eq!(mode, GoMode::Depth(10));
    assert_ne!(mode, GoMode::Depth(5));
}

#[test]
fn test_set_option_command() {
    let cmd = UcciCommand::SetOption {
        name: "hashsize".to_string(),
        value: Some("256".to_string()),
    };
    match cmd {
        UcciCommand::SetOption { name, value } => {
            assert_eq!(name, "hashsize");
            assert_eq!(value, Some("256".to_string()));
        }
        _ => panic!("Wrong command type"),
    }
}

#[test]
fn test_bestmove_response() {
    let resp = UcciResponse::BestMove {
        mv: "h2e2".to_string(),
        ponder: Some("h9g7".to_string()),
        draw: false,
        resign: false,
    };
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
