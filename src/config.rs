//! Configuration file support for AI engine settings

use std::path::PathBuf;

/// Get AI engine path from config file
pub fn get_engine_path_from_config() -> Option<PathBuf> {
    let config_dir = dirs::config_dir()?.join("cn_chess_tui");
    let config_path = config_dir.join("config.toml");

    let contents = std::fs::read_to_string(config_path).ok()?;

    // Simple TOML parsing for engine_path
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("engine_path") {
            let path = line.split('=').nth(1)?.trim().trim_matches('"');
            return Some(PathBuf::from(path));
        }
    }

    None
}

/// Get show_thinking setting from config
pub fn get_show_thinking_from_config() -> bool {
    false // Default
}
