//! UCCI protocol command and response types

/// Engine state in UCCI protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Boot,   // Before ucci command
    Idle,   // Waiting for commands
    Thinking, // Searching for a move
}

/// UCCI commands sent from interface to engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UcciCommand {
    Ucci,
    SetOption { name: String, value: Option<String> },
    Position { fen: String, moves: Vec<String> },
    BanMoves { moves: Vec<String> },
    Go { mode: GoMode, ponder: bool, draw: bool },
    Stop,
    PonderHit { draw: bool },
    IsReady,
    Quit,
    Probe { fen: String, moves: Vec<String> },
}

/// Go command modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoMode {
    Depth(u32),
    Infinite,
    Nodes(u64),
    Time {
        time: u64,
        movestogo: Option<u32>,
        increment: Option<u64>,
        opptime: Option<u64>,
        oppmovestogo: Option<u32>,
        oppincrement: Option<u64>,
    },
}

/// UCCI responses from engine to interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UcciResponse {
    Id { field: String, value: String },
    Option {
        name: String,
        type_: OptionType,
        min: Option<i32>,
        max: Option<i32>,
        vars: Vec<String>,
        default: Option<String>,
    },
    UcciOk,
    ReadyOk,
    BestMove {
        mv: String,
        ponder: Option<String>,
        draw: bool,
        resign: bool,
    },
    NoBestMove,
    Info {
        time: Option<u64>,
        nodes: Option<u64>,
        depth: Option<u32>,
        score: Option<i32>,
        pv: Vec<String>,
        currmove: Option<String>,
        message: Option<String>,
    },
    PopHash {
        bestmove: Option<String>,
        lowerbound: Option<(i32, u32)>,
        upperbound: Option<(i32, u32)>,
    },
    Bye,
}

/// Option types in UCCI protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
    Label,
}

/// Result of a search
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveResult {
    Move(String, Option<String>), // bestmove, ponder
    NoMove,
    Draw,
    Resign,
}

impl UcciCommand {
    /// Serialize this command to UCCI protocol string format
    pub fn serialize(&self) -> String {
        crate::ucci::serializer::serialize_command(self)
    }
}
