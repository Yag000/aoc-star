//! Configuration management.
//! We have a config file where the user can put their session cookie for Advent of Code,
//! and also specify which year to use by default : in any other cas, when a year is not specified,
//! we default to the current year.
//!
//! ! The config file is searched in the current directory first (aoc-star.yml),
//! ! then in the global config directory (usually ~/.config/aoc-star/config.yml).
//! ! If no config file is found, we create one in the global config directory
//! ! using environment variables (AOC_TOKEN for the session cookie).

use std::path::PathBuf;

use chrono::Datelike;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Config {
    pub token: String,
    pub year: Option<i32>,
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

impl Default for Config {
    fn default() -> Self {
        Config::from_env()
    }
}

enum ConfigFileLocation {
    CurrentDir,
    GlobalDir,
}

impl From<ConfigFileLocation> for PathBuf {
    fn from(val: ConfigFileLocation) -> Self {
        match val {
            ConfigFileLocation::CurrentDir => std::path::PathBuf::from("aoc-star.yml"),
            ConfigFileLocation::GlobalDir => {
                let config_dir = dirs::config_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join("aoc-star");
                config_dir.join("config.yml")
            }
        }
    }
}

/// Get the default token from the environment variable AOC_TOKEN
fn default_token() -> String {
    std::env::var("AOC_TOKEN").unwrap_or_default()
}

#[allow(dead_code)]
/// Get the session token from the config file or global config
/// If not found, return an empty string
///
/// This function is only used when the "aoc-client" feature is enabled
/// and the user wants to fetch input or submit answers.
pub(crate) fn get_config_token() -> Result<String, Box<dyn std::error::Error>> {
    let token = get_config()?.token;
    if !token.is_empty() {
        Ok(token)
    } else {
        let token = default_token();
        update_token(token.clone(), ConfigFileLocation::GlobalDir).ok();
        Ok(token)
    }
}

/// Get the config from the current directory or global config
pub(crate) fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    match config_path {
        Some(loc) => {
            let path: PathBuf = loc.into();
            let contents = std::fs::read_to_string(path)?;
            serde_yaml::from_str(&contents).map_err(|e| e.into())
        }
        None => setup_config(),
    }
}

/// Finds the current config file. First it looks for aoc-star.yml on the current directory
/// and if nto it defaults to  ~/.config/aoc-star/config.yml.
fn get_config_path() -> Option<ConfigFileLocation> {
    let current_dir_config = std::path::PathBuf::from("aoc-star.yml");
    if current_dir_config.exists() {
        Some(ConfigFileLocation::CurrentDir)
    } else {
        let config_file = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("aoc-star")
            .join("config.yml");
        if config_file.exists() {
            Some(ConfigFileLocation::GlobalDir)
        } else {
            None
        }
    }
}
// Sets up the config file in the global config directory if there is no config file yet
pub(crate) fn setup_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path: PathBuf = ConfigFileLocation::GlobalDir.into();

    if path.exists() {
        return Err("Config file already exists.".into());
    }
    let config = Config::from_env();
    write_config(&config, ConfigFileLocation::GlobalDir)?;
    Ok(config)
}

fn write_config(
    config: &Config,
    location: ConfigFileLocation,
) -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = location.into();
    let contents = serde_yaml::to_string(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}

fn update_token(
    token: String,
    location: ConfigFileLocation,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = get_config()?;
    config.token = token;
    let path: PathBuf = location.into();
    let contents = serde_yaml::to_string(&config)?;
    std::fs::write(path, contents)?;
    Ok(())
}
