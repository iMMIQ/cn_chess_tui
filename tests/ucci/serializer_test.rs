use cn_chess_tui::ucci::{GoMode, UcciCommand};

fn serialize(cmd: &UcciCommand) -> String {
    cmd.serialize()
}

#[test]
fn test_serialize_ucci() {
    let cmd = UcciCommand::Ucci;
    assert_eq!(serialize(&cmd), "ucci");
}

#[test]
fn test_serialize_set_option_with_value() {
    let cmd = UcciCommand::SetOption {
        name: "hashsize".to_string(),
        value: Some("256".to_string()),
    };
    assert_eq!(serialize(&cmd), "setoption hashsize 256");
}

#[test]
fn test_serialize_set_option_no_value() {
    let cmd = UcciCommand::SetOption {
        name: "newgame".to_string(),
        value: None,
    };
    assert_eq!(serialize(&cmd), "setoption newgame");
}

#[test]
fn test_serialize_position_no_moves() {
    let cmd = UcciCommand::Position {
        fen: "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1".to_string(),
        moves: vec![],
    };
    assert_eq!(
        serialize(&cmd),
        "position fen rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1"
    );
}

#[test]
fn test_serialize_position_with_moves() {
    let cmd = UcciCommand::Position {
        fen: "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1".to_string(),
        moves: vec!["h2e2".to_string(), "h9g7".to_string()],
    };
    assert_eq!(
        serialize(&cmd),
        "position fen rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1 moves h2e2 h9g7"
    );
}

#[test]
fn test_serialize_go_depth() {
    let cmd = UcciCommand::Go {
        mode: GoMode::Depth(10),
        ponder: false,
        draw: false,
    };
    assert_eq!(serialize(&cmd), "depth 10");
}

#[test]
fn test_serialize_go_time() {
    let cmd = UcciCommand::Go {
        mode: GoMode::Time {
            time: 300000,
            movestogo: None,
            increment: Some(0),
            opptime: None,
            oppmovestogo: None,
            oppincrement: None,
        },
        ponder: false,
        draw: false,
    };
    assert_eq!(serialize(&cmd), "time 300000 increment 0");
}

#[test]
fn test_serialize_go_ponder() {
    let cmd = UcciCommand::Go {
        mode: GoMode::Time {
            time: 295000,
            movestogo: None,
            increment: None,
            opptime: None,
            oppmovestogo: None,
            oppincrement: None,
        },
        ponder: true,
        draw: false,
    };
    assert_eq!(serialize(&cmd), "ponder time 295000");
}

#[test]
fn test_serialize_banmoves() {
    let cmd = UcciCommand::BanMoves {
        moves: vec!["h6i4".to_string()],
    };
    assert_eq!(serialize(&cmd), "banmoves h6i4");
}

#[test]
fn test_serialize_stop() {
    let cmd = UcciCommand::Stop;
    assert_eq!(serialize(&cmd), "stop");
}

#[test]
fn test_serialize_ponderhit() {
    let cmd = UcciCommand::PonderHit { draw: false };
    assert_eq!(serialize(&cmd), "ponderhit");
}

#[test]
fn test_serialize_ponderhit_draw() {
    let cmd = UcciCommand::PonderHit { draw: true };
    assert_eq!(serialize(&cmd), "ponderhit draw");
}

#[test]
fn test_serialize_quit() {
    let cmd = UcciCommand::Quit;
    assert_eq!(serialize(&cmd), "quit");
}

#[test]
fn test_serialize_probe_no_moves() {
    let cmd = UcciCommand::Probe {
        fen: "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1".to_string(),
        moves: vec![],
    };
    assert_eq!(serialize(&cmd), "probe fen rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1");
}

#[test]
fn test_serialize_probe_with_moves() {
    let cmd = UcciCommand::Probe {
        fen: "4k4/9/9/9/9/9/9/9/4K4 w - - 0 1".to_string(),
        moves: vec!["h0e3".to_string()],
    };
    assert_eq!(serialize(&cmd), "probe fen 4k4/9/9/9/9/9/9/9/4K4 w - - 0 1 moves h0e3");
}

#[test]
fn test_serialize_go_infinite() {
    let cmd = UcciCommand::Go {
        mode: GoMode::Infinite,
        ponder: false,
        draw: false,
    };
    assert_eq!(serialize(&cmd), "infinite");
}

#[test]
fn test_serialize_go_nodes() {
    let cmd = UcciCommand::Go {
        mode: GoMode::Nodes(1000000),
        ponder: false,
        draw: false,
    };
    assert_eq!(serialize(&cmd), "nodes 1000000");
}
