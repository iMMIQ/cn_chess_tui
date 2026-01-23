//! Configuration file support for AI engine settings

use dirs::config_dir;
use serde::Deserialize;
use std::path::PathBuf;

/// Engine configuration from TOML file
#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    /// Path to the UCCI engine executable
    pub engine_path: Option<PathBuf>,
    /// Whether to show engine thinking output
    pub show_thinking: Option<bool>,
}

impl EngineConfig {
    /// Load configuration from the default config file location
    ///
    /// Config file is looked for in:
    /// - Linux: `~/.config/cn_chess_tui/config.toml`
    /// - macOS: `~/Library/Application Support/cn_chess_tui/config.toml`
    /// - Windows: `%APPDATA%\cn_chess_tui\config.toml`
    ///
    /// Returns None if config file doesn't exist or is invalid
    pub fn load() -> Option<Self> {
        let config_dir = config_dir()?.join("cn_chess_tui");
        let config_path = config_dir.join("config.toml");

        let contents = std::fs::read_to_string(config_path).ok()?;
        toml::from_str(&contents).ok()
    }

    /// Get AI engine path from config file
    pub fn get_engine_path(&self) -> Option<PathBuf> {
        self.engine_path.clone()
    }

    /// Get show_thinking setting from config
    ///
    /// Returns false if not set
    pub fn get_show_thinking(&self) -> bool {
        self.show_thinking.unwrap_or(false)
    }
}

/// Get AI engine path from config file
///
/// This is a convenience function that loads the config and extracts the engine path.
/// Returns None if config file doesn't exist or engine_path is not set.
pub fn get_engine_path_from_config() -> Option<PathBuf> {
    EngineConfig::load()?.get_engine_path()
}

/// Get show_thinking setting from config
///
/// Returns false if config file doesn't exist or show_thinking is not set.
pub fn get_show_thinking_from_config() -> bool {
    EngineConfig::load()
        .map(|cfg| cfg.get_show_thinking())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_config_with_all_fields() {
        let toml_content = r#"
            engine_path = "/usr/bin/pikafish"
            show_thinking = true
        "#;

        let config: EngineConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.engine_path, Some(PathBuf::from("/usr/bin/pikafish")));
        assert_eq!(config.show_thinking, Some(true));
    }

    #[test]
    fn test_parse_config_empty() {
        let toml_content = "";
        let config: EngineConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.engine_path, None);
        assert_eq!(config.show_thinking, None);
    }

    #[test]
    fn test_parse_config_partial() {
        let toml_content = r#"
            engine_path = "/usr/bin/pikafish"
        "#;
        let config: EngineConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.engine_path, Some(PathBuf::from("/usr/bin/pikafish")));
        assert_eq!(config.show_thinking, None);
    }

    #[test]
    fn test_get_engine_path() {
        let config = EngineConfig {
            engine_path: Some(PathBuf::from("/usr/bin/pikafish")),
            show_thinking: Some(true),
        };
        assert_eq!(config.get_engine_path(), Some(PathBuf::from("/usr/bin/pikafish")));
    }

    #[test]
    fn test_get_engine_path_none() {
        let config = EngineConfig {
            engine_path: None,
            show_thinking: None,
        };
        assert_eq!(config.get_engine_path(), None);
    }

    #[test]
    fn test_get_show_thinking() {
        let config = EngineConfig {
            engine_path: None,
            show_thinking: Some(true),
        };
        assert_eq!(config.get_show_thinking(), true);
    }

    #[test]
    fn test_get_show_thinking_default() {
        let config = EngineConfig {
            engine_path: None,
            show_thinking: None,
        };
        assert_eq!(config.get_show_thinking(), false);
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("cn_chess_tui");
        fs::create_dir(&config_dir).unwrap();
        let config_path = config_dir.join("config.toml");

        let toml_content = r#"
            engine_path = "/usr/bin/pikafish"
            show_thinking = true
        "#;
        fs::write(&config_path, toml_content).unwrap();

        // Note: This test documents the structure but can't fully test
        // due to dirs::config_dir() being a global function
    }
}
