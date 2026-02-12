//! Configuration management and Context struct (2026 pattern)

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::TuxBoxError;

/// TuxBox home directory (~/.tuxbox)
pub fn tuxbox_home() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| TuxBoxError::ConfigError("Home directory not found".into()))?;
    Ok(home.join(".tuxbox"))
}

/// Tools storage directory (~/.tuxbox/tools)
pub fn tools_dir() -> Result<PathBuf> {
    Ok(tuxbox_home()?.join("tools"))
}

/// Registry cache directory (~/.tuxbox/registry)
pub fn registry_dir() -> Result<PathBuf> {
    Ok(tuxbox_home()?.join("registry"))
}

/// Config file path (~/.tuxbox/config.toml)
pub fn config_file() -> Result<PathBuf> {
    Ok(tuxbox_home()?.join("config.toml"))
}

/// TuxBox configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub registry_url: String,
}

/// Tool definition from registry
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    pub repo: String,
    pub branch: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub isolation: Option<IsolationStrategy>,
    pub commands: Option<Commands>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IsolationStrategy {
    Venv,
    Docker,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    pub setup: Option<String>,
    pub run: String,
}

/// Initialize TuxBox configuration
pub fn init_config(registry_url: &str) -> Result<()> {
    let tuxbox_home = tuxbox_home()?;

    // Create directories
    fs::create_dir_all(&tuxbox_home)
        .context("Failed to create TuxBox home directory")?;
    fs::create_dir_all(tools_dir()?)
        .context("Failed to create tools directory")?;
    fs::create_dir_all(registry_dir()?)
        .context("Failed to create registry directory")?;

    // Write config
    let config = Config {
        registry_url: registry_url.to_string(),
    };

    let config_toml = toml::to_string_pretty(&config)?;
    fs::write(config_file()?, config_toml)
        .context("Failed to write config file")?;

    Ok(())
}

/// Load configuration
pub fn load_config() -> Result<Config> {
    let config_path = config_file()?;

    if !config_path.exists() {
        return Err(TuxBoxError::NotInitialized.into());
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}

/// List available tools (stub for MVP)
pub fn list_tools() -> Result<()> {
    let config = load_config()?;
    println!("Registry: {}", config.registry_url);
    println!("\n(Registry fetching not implemented yet - coming in Phase 3)");

    // TODO: Fetch and parse registry
    Ok(())
}

/// Show TuxBox status (stub for MVP)
pub fn show_status() -> Result<()> {
    let config = load_config()?;
    println!("Registry: {}", config.registry_url);

    let tools_dir = tools_dir()?;
    if tools_dir.exists() {
        println!("\nInstalled tools:");
        for entry in fs::read_dir(tools_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                println!("  - {}", entry.file_name().to_string_lossy());
            }
        }
    } else {
        println!("\nNo tools installed yet.");
    }

    Ok(())
}
