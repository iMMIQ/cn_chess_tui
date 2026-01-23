//! Serialize UCCI commands to protocol format

use crate::ucci::protocol::{GoMode, UcciCommand};

/// Serialize a UCCI command to a protocol string
pub fn serialize_command(cmd: &UcciCommand) -> String {
    match cmd {
        UcciCommand::Ucci => "ucci".to_string(),

        UcciCommand::SetOption { name, value } => {
            match value {
                Some(v) => format!("setoption {} {}", name, v),
                None => format!("setoption {}", name),
            }
        }

        UcciCommand::Position { fen, moves } => {
            if moves.is_empty() {
                format!("position fen {}", fen)
            } else {
                format!("position fen {} moves {}", fen, moves.join(" "))
            }
        }

        UcciCommand::BanMoves { moves } => {
            if moves.is_empty() {
                "banmoves".to_string()
            } else {
                format!("banmoves {}", moves.join(" "))
            }
        }

        UcciCommand::Go { mode, ponder, draw } => {
            let mut parts = Vec::new();
            if *ponder {
                parts.push("ponder".to_string());
            }
            if *draw {
                parts.push("draw".to_string());
            }
            parts.push(serialize_go_mode(mode));
            parts.join(" ")
        }

        UcciCommand::Stop => "stop".to_string(),

        UcciCommand::PonderHit { draw } => {
            if *draw {
                "ponderhit draw".to_string()
            } else {
                "ponderhit".to_string()
            }
        }

        UcciCommand::IsReady => "isready".to_string(),

        UcciCommand::Quit => "quit".to_string(),

        UcciCommand::Probe { fen, moves } => {
            if moves.is_empty() {
                format!("probe fen {}", fen)
            } else {
                format!("probe fen {} moves {}", fen, moves.join(" "))
            }
        }
    }
}

fn serialize_go_mode(mode: &GoMode) -> String {
    match mode {
        GoMode::Depth(d) => format!("depth {}", d),
        GoMode::Infinite => "infinite".to_string(),
        GoMode::Nodes(n) => format!("nodes {}", n),
        GoMode::Time {
            time,
            movestogo,
            increment,
            opptime,
            oppmovestogo,
            oppincrement,
        } => {
            let mut parts = vec![format!("time {}", time)];
            if let Some(mtg) = movestogo {
                parts.push(format!("movestogo {}", mtg));
            }
            if let Some(inc) = increment {
                parts.push(format!("increment {}", inc));
            }
            if let Some(opt) = opptime {
                parts.push(format!("opptime {}", opt));
            }
            if let Some(omtg) = oppmovestogo {
                parts.push(format!("oppmovestogo {}", omtg));
            }
            if let Some(oinc) = oppincrement {
                parts.push(format!("oppincrement {}", oinc));
            }
            parts.join(" ")
        }
    }
}
