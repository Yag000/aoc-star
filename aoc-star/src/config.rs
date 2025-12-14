//! Configuration management.
//! We have a config file where the user can put their session cookie for Advent of Code,
//! and also specify which year to use by default : in any other cas, when a year is not specified,
//! we default to the current year.
//!
//! ! The config file is searched in the current directory first (aoc-star.yml),
//! ! then in the global config directory (usually ~/.config/aoc-star/config.yml).
//! ! If no config file is found, we create one in the global config directory
//! ! using environment variables (AOC_TOKEN for the session cookie).

use chrono::Datelike;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Config {
    pub token: String,
    pub year: Option<i32>,
}

/// Get the default token from the environment variable AOC_TOKEN
fn default_token() -> String {
    std::env::var("AOC_TOKEN").unwrap_or_default()
}

impl Config {
    /// Create a Config from environment variables :
    ///     - AOC_TOKEN for the session cookie
    ///     - current year for the year
    pub fn from_env() -> Self {
        let token = default_token();
        let year = Some(chrono::Utc::now().year());

        Config { token, year }
    }
}

#[allow(dead_code)]
/// Get the session token from the config file or global config
/// If not found, return an empty string
///
/// This function is only used when the "aoc-client" feature is enabled
/// and the user wants to fetch input or submit answers.
pub(crate) fn get_config_token() -> String {
    let token = get_config().token;
    if !token.is_empty() {
        token
    } else {
        get_global_config().token
    }
}

/// Get the global config from the config directory
/// If not found, create a new config file from environment variables
/// and return the config
fn get_global_config() -> Config {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("aoc-star");
    std::fs::create_dir_all(&config_dir).ok();
    let config_file = config_dir.join("config.yml");

    if config_file.exists() {
        let contents = std::fs::read_to_string(config_file).unwrap_or_default();
        serde_yaml::from_str(&contents).unwrap_or_else(|_| Config::from_env())
    } else {
        // Create a new config from environment variables
        let config = Config::from_env();
        // We make sure to save the config file for future use
        let contents = serde_yaml::to_string(&config).unwrap_or_default();
        std::fs::write(config_file, contents).ok();
        config
    }
}

/// Get the config from the current directory or global config.
pub(crate) fn get_config() -> Config {
    // Check current directory for a config file first
    let current_dir_config = std::path::PathBuf::from("aoc-star.yml");
    if current_dir_config.exists() {
        let contents = std::fs::read_to_string(current_dir_config).unwrap_or_default();
        return serde_yaml::from_str(&contents).unwrap_or_else(|_| Config::from_env());
    }
    // Fallback to global config
    get_global_config()
}
