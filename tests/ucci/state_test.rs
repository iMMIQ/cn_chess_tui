use cn_chess_tui::ucci::state::UcciStateMachine;
use cn_chess_tui::ucci::{GoMode, UcciCommand, UcciResponse};

#[test]
fn test_state_machine_full_cycle() {
    let mut sm = UcciStateMachine::new();

    // Boot -> ucci -> ucciok -> Idle
    assert!(sm.is_boot());
    sm.transition(&UcciCommand::Ucci).unwrap();
    sm.on_response(&UcciResponse::UcciOk).unwrap();
    assert!(sm.is_idle());

    // Idle -> go -> Thinking
    sm.transition(&UcciCommand::Go {
        mode: GoMode::Time {
            time: 5000,
            movestogo: None,
            increment: None,
            opptime: None,
            oppmovestogo: None,
            oppincrement: None,
        },
        ponder: false,
        draw: false,
    })
    .unwrap();
    assert!(sm.is_thinking());

    // Thinking -> bestmove -> Idle
    sm.on_response(&UcciResponse::BestMove {
        mv: "h2e2".to_string(),
        ponder: None,
        draw: false,
        resign: false,
    })
    .unwrap();
    assert!(sm.is_idle());
}

#[test]
fn test_pondering_mode() {
    let mut sm = UcciStateMachine::new();
    sm.transition(&UcciCommand::Ucci).unwrap();
    sm.on_response(&UcciResponse::UcciOk).unwrap();

    // Go with ponder
    sm.transition(&UcciCommand::Go {
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
    })
    .unwrap();
    assert!(sm.is_thinking());

    // PonderHit keeps us in thinking state
    assert!(sm.can_send(&UcciCommand::PonderHit { draw: false }));
}

#[test]
fn test_nobestmove_transitions_to_idle() {
    let mut sm = UcciStateMachine::new();
    sm.transition(&UcciCommand::Ucci).unwrap();
    sm.on_response(&UcciResponse::UcciOk).unwrap();
    sm.transition(&UcciCommand::Go {
        mode: GoMode::Depth(0),
        ponder: false,
        draw: false,
    })
    .unwrap();
    sm.on_response(&UcciResponse::NoBestMove).unwrap();
    assert!(sm.is_idle());
}

#[test]
fn test_setoption_in_idle() {
    let mut sm = UcciStateMachine::new();
    sm.transition(&UcciCommand::Ucci).unwrap();
    sm.on_response(&UcciResponse::UcciOk).unwrap();

    // Can send setoption in idle
    assert!(sm.can_send(&UcciCommand::SetOption {
        name: "hashsize".to_string(),
        value: Some("256".to_string()),
    }));

    // State doesn't change
    sm.transition(&UcciCommand::SetOption {
        name: "hashsize".to_string(),
        value: Some("256".to_string()),
    })
    .unwrap();
    assert!(sm.is_idle());
}
