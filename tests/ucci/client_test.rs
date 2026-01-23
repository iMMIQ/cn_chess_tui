//! Integration tests for the high-level UCCI client API

use cn_chess_tui::ucci::UcciClient;
use std::io::Write;
use tempfile::NamedTempFile;

/// Create a mock engine script that responds to UCCI commands
#[cfg(unix)]
fn create_mock_engine() -> tempfile::TempPath {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"#!/bin/bash
# Mock UCCI engine for testing

while read line; do
    case "$line" in
        "ucci")
            echo "id name MockEngine"
            echo "id author TestAuthor"
            echo "option hashsize type spin min 1 max 512 default 32"
            echo "ucciok"
            ;;
        "isready")
            echo "readyok"
            ;;
        "setoption "*)
            # Accept any setoption command
            ;;
        "position "*)
            # Accept any position command
            ;;
        "banmoves "*)
            # Accept any banmoves command
            ;;
        "go depth "*)
            echo "info depth 10 score 100 nodes 1000 time 100"
            echo "bestmove h2e2"
            ;;
        "go time "*)
            echo "info depth 8 score 50 nodes 500 time 50"
            echo "bestmove h2e2"
            ;;
        "go infinite")
            echo "info depth 1 score 10"
            ;;
        "stop")
            echo "bestmove h2e2"
            ;;
        "quit")
            echo "bye"
            exit 0
            ;;
    esac
done
"#
    )
    .unwrap();

    // Flush and sync to ensure all data is written
    file.as_file().flush().unwrap();
    file.as_file().sync_all().unwrap();

    // Make executable
    use std::os::unix::fs::PermissionsExt;
    let mut perm = file.as_file().metadata().unwrap().permissions();
    perm.set_mode(0o755);
    file.as_file().set_permissions(perm).unwrap();

    // Convert to TempPath to release the file handle
    let path = file.into_temp_path();

    // Small delay to ensure the file is fully written and executable
    std::thread::sleep(std::time::Duration::from_millis(10));

    path
}

#[test]
#[cfg(unix)]
fn test_client_initialize() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    // Initialize the client
    client.initialize().unwrap();

    // Verify engine info was collected
    let info = client.engine_info();
    assert_eq!(info.name, "MockEngine");
    assert_eq!(info.author.as_ref().unwrap(), "TestAuthor");

    // Verify options were collected
    let options = client.options();
    assert!(options.contains_key("hashsize"));
    let hashsize_opt = options.get("hashsize").unwrap();
    assert_eq!(hashsize_opt.name, "hashsize");
    assert_eq!(hashsize_opt.default, Some("32".to_string()));

    // Verify state is idle after initialization
    assert!(client.is_idle());

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_set_option() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Set an option
    client.set_option("hashsize", "128").unwrap();

    // Verify state is still idle
    assert!(client.is_idle());

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_set_position() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Set a position
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    let moves = vec!["h2e2".to_string()];
    client.set_position(fen, &moves).unwrap();

    // Verify state is still idle
    assert!(client.is_idle());

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_go_depth_stop() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Set a position first
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    client.set_position(fen, &[]).unwrap();

    // Start depth search
    client.go_depth(10).unwrap();

    // Verify state is thinking
    assert!(client.is_thinking());

    // Stop and get result
    let result = client.stop().unwrap();

    // Verify we got a move
    match result {
        cn_chess_tui::ucci::MoveResult::Move(mv, ponder) => {
            assert_eq!(mv, "h2e2");
            assert!(ponder.is_none());
        }
        _ => panic!("Expected Move result, got {:?}", result),
    }

    // Verify state is idle again
    assert!(client.is_idle());

    // Note: Info messages are collected during stop() but the mock engine
    // sends them immediately after "go depth", not after "stop".
    // This is expected behavior - in real engines, info messages come
    // during the search, which our client properly collects.

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_go_time_stop() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Set a position first
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    client.set_position(fen, &[]).unwrap();

    // Start time search
    client.go_time(1000).unwrap();

    // Stop and get result
    let result = client.stop().unwrap();

    // Verify we got a move
    match result {
        cn_chess_tui::ucci::MoveResult::Move(mv, ponder) => {
            assert_eq!(mv, "h2e2");
            assert!(ponder.is_none());
        }
        _ => panic!("Expected Move result, got {:?}", result),
    }

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_ban_moves() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Ban some moves
    let banned_moves = vec!["h2e2".to_string(), "h9g7".to_string()];
    client.ban_moves(&banned_moves).unwrap();

    // Verify state is still idle
    assert!(client.is_idle());

    client.shutdown().unwrap();
}

#[test]
#[cfg(unix)]
fn test_client_is_ready() {
    let mock = create_mock_engine();
    let mut client = UcciClient::new(mock.to_str().unwrap()).unwrap();

    client.initialize().unwrap();

    // Check if ready
    assert!(client.is_ready().unwrap());

    client.shutdown().unwrap();
}
