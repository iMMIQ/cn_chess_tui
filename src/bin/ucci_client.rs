use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cn_chess_tui::ucci::UcciClient;

#[derive(Parser)]
#[command(name = "ucci_client")]
#[command(about = "UCCI (Universal Chinese Chess Protocol) client", long_about = None)]
struct Cli {
    /// Path to UCCI engine executable
    #[arg(short, long)]
    engine: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize engine and show info
    Info,

    /// Analyze a position
    Analyze {
        /// FEN string of the position
        #[arg(short, long)]
        fen: String,

        /// Search depth
        #[arg(short, long, default_value_t = 10)]
        depth: u32,

        /// Show thinking output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Play a quick game
    Play {
        /// Time in milliseconds per move
        #[arg(short, long, default_value_t = 5000)]
        time: u64,

        /// Number of moves to play
        #[arg(short, long, default_value_t = 10)]
        moves: u32,
    },

    /// Interactive mode
    Interactive,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !cli.engine.exists() {
        eprintln!("Error: Engine not found: {}", cli.engine.display());
        return Err("Engine not found".into());
    }

    let engine_path = cli
        .engine
        .to_str()
        .ok_or_else(|| Box::<dyn std::error::Error>::from("Engine path contains invalid UTF-8"))?;
    let mut client = UcciClient::new(engine_path)?;
    client.initialize()?;

    match cli.command {
        Commands::Info => {
            show_engine_info(&client);
        }
        Commands::Analyze {
            fen,
            depth,
            verbose,
        } => {
            analyze_position(&mut client, &fen, depth, verbose)?;
        }
        Commands::Play { time, moves } => {
            play_game(&mut client, time, moves)?;
        }
        Commands::Interactive => {
            interactive_mode(&mut client)?;
        }
    }

    client.shutdown()?;
    Ok(())
}

fn show_engine_info(client: &UcciClient) {
    let info = client.engine_info();
    println!("=== UCCI Engine Information ===");
    println!("Name: {}", info.name);
    if let Some(author) = &info.author {
        println!("Author: {}", author);
    }
    if let Some(copyright) = &info.copyright {
        println!("Copyright: {}", copyright);
    }

    println!("\n=== Supported Options ===");
    for (name, opt) in client.options() {
        println!("{}: {:?}", name, opt.type_);
        if let Some(default) = &opt.default {
            println!("  Default: {}", default);
        }
        if !opt.vars.is_empty() {
            println!("  Values: {}", opt.vars.join(", "));
        }
    }
}

fn analyze_position(
    client: &mut UcciClient,
    fen: &str,
    depth: u32,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Analyzing position: {}", fen);
    println!("Depth: {}", depth);
    println!();

    client.set_position(fen, &[])?;
    client.go_depth(depth)?;

    let result = client.stop()?;

    // Display thinking info if verbose
    if verbose {
        let infos = client.read_info();
        if !infos.is_empty() {
            println!("=== Thinking Output ===");
            for info in &infos {
                if let Some(d) = info.depth {
                    print!("depth {}: ", d);
                }
                if let Some(score) = info.score {
                    print!("score: {} ", score);
                }
                if let Some(time) = info.time_ms {
                    print!("time: {}ms ", time);
                }
                if let Some(nodes) = info.nodes {
                    print!("nodes: {} ", nodes);
                }
                if !info.pv.is_empty() {
                    println!("pv: {}", info.pv.join(" "));
                } else {
                    println!();
                }
                if let Some(msg) = &info.message {
                    println!("  {}", msg);
                }
            }
            println!();
        }
    }

    match result {
        cn_chess_tui::ucci::MoveResult::Move(mv, ponder) => {
            println!("Best move: {}", mv);
            if let Some(p) = ponder {
                println!("Ponder: {}", p);
            }
        }
        cn_chess_tui::ucci::MoveResult::NoMove => {
            println!("No move found");
        }
        cn_chess_tui::ucci::MoveResult::Draw => {
            println!("Engine suggests draw");
        }
        cn_chess_tui::ucci::MoveResult::Resign => {
            println!("Engine resigns");
        }
    }

    Ok(())
}

fn play_game(
    client: &mut UcciClient,
    time_ms: u64,
    num_moves: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Playing {} moves at {}ms per move", num_moves, time_ms);
    println!();

    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    client.set_position(fen, &[])?;

    for i in 0..num_moves {
        println!("Move {}:", i + 1);
        client.go_time(time_ms)?;

        let result = client.stop()?;

        match result {
            cn_chess_tui::ucci::MoveResult::Move(mv, ponder) => {
                println!("  Engine plays: {}", mv);
                if let Some(p) = ponder {
                    println!("  (Ponder: {})", p);
                }
            }
            cn_chess_tui::ucci::MoveResult::NoMove => {
                println!("  No move found");
                break;
            }
            cn_chess_tui::ucci::MoveResult::Draw => {
                println!("  Engine offers draw");
                break;
            }
            cn_chess_tui::ucci::MoveResult::Resign => {
                println!("  Engine resigns");
                break;
            }
        }
    }

    Ok(())
}

fn interactive_mode(client: &mut UcciClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UCCI Interactive Mode ===");
    println!("Type 'help' for commands, 'quit' to exit");
    println!();

    #[cfg(feature = "ucci-cli")]
    {
        use rustyline::error::ReadlineError;
        use rustyline::DefaultEditor;

        let mut rl = DefaultEditor::new()?;

        loop {
            let readline = rl.readline("ucci> ");
            match readline {
                Ok(line) => {
                    let _ = rl.add_history_entry(line.as_str());
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.is_empty() {
                        continue;
                    }

                    match parts[0] {
                        "help" => {
                            println!("Available commands:");
                            println!("  info              - Show engine information");
                            println!("  fen <FEN>         - Set position");
                            println!("  go depth <N>      - Search to depth N");
                            println!("  go time <MS>      - Search for MS milliseconds");
                            println!("  stop              - Stop search");
                            println!("  setopt <N> <V>    - Set option");
                            println!("  quit              - Exit");
                        }
                        "info" => {
                            show_engine_info(client);
                        }
                        "fen" => {
                            if parts.len() >= 2 {
                                client.set_position(parts[1], &[])?;
                                println!("Position set");
                            }
                        }
                        "go" => {
                            if parts.len() >= 3 {
                                match parts[1] {
                                    "depth" => {
                                        if let Ok(depth) = parts[2].parse::<u32>() {
                                            client.go_depth(depth)?;
                                            println!("Searching to depth {}...", depth);

                                            let result = client.stop()?;
                                            match result {
                                                cn_chess_tui::ucci::MoveResult::Move(mv, _) => {
                                                    println!("Best move: {}", mv);
                                                }
                                                cn_chess_tui::ucci::MoveResult::NoMove => {
                                                    println!("No move");
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    "time" => {
                                        if let Ok(time) = parts[2].parse::<u64>() {
                                            client.go_time(time)?;
                                            println!("Searching for {}ms...", time);

                                            let result = client.stop()?;
                                            match result {
                                                cn_chess_tui::ucci::MoveResult::Move(mv, _) => {
                                                    println!("Best move: {}", mv);
                                                }
                                                cn_chess_tui::ucci::MoveResult::NoMove => {
                                                    println!("No move");
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("Unknown mode: {}", parts[1]);
                                    }
                                }
                            }
                        }
                        "setopt" => {
                            if parts.len() >= 3 {
                                client.set_option(parts[1], parts[2])?;
                                println!("Option set");
                            }
                        }
                        "quit" => {
                            println!("Exiting...");
                            break;
                        }
                        _ => {
                            println!("Unknown command: {}", parts[0]);
                            println!("Type 'help' for available commands");
                        }
                    }
                }
                Err(ReadlineError::Eof) => {
                    println!("Exiting...");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    #[cfg(not(feature = "ucci-cli"))]
    {
        println!("Error: Interactive mode requires 'ucci-cli' feature");
        println!("Build with: cargo build --features ucci-cli");
    }

    Ok(())
}
