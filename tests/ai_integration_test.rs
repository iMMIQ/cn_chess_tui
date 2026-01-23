use cn_chess_tui::game::GameController;
use std::env;
use std::fs;
use std::io::Write;

#[test]
#[cfg(unix)]
fn test_ai_engine_init() {
    use std::os::unix::fs::PermissionsExt;

    // Create a temporary script file
    let temp_dir = env::temp_dir();
    let script_path = temp_dir.join("mock_ucci_engine_test.sh");

    // Write script content
    let script_content = r#"#!/bin/bash
while read line; do
  case "$line" in
    ucci)
      echo "id name MockEngine"
      echo "ucciok"
      ;;
    isready)
      echo "readyok"
      ;;
    position*)
      ;;
    go*depth*)
      echo "bestmove h2e2"
      ;;
    stop)
      echo "bestmove h2e2"
      ;;
    quit)
      exit 0
      ;;
  esac
done
"#;

    fs::write(&script_path, script_content).unwrap();

    // Make executable
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();

    // Test with controller
    let mut controller = GameController::new();
    let result = controller.init_engine(script_path.to_str().unwrap());

    // Clean up
    let _ = fs::remove_file(&script_path);

    match result {
        Ok(_) => {
            assert!(controller.has_engine());
        }
        Err(e) => {
            panic!("Failed to init engine: {}", e);
        }
    }
}
