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

/// List available tools (Phase 0/1: hardcoded, Phase 2: from registry)
pub fn list_tools() -> Result<()> {
    use colored::Colorize;

    // Try to load registry config (Phase 2)
    if let Ok(config) = load_config() {
        println!("Registry: {}", config.registry_url);
        println!("\n(Registry fetching not implemented yet - coming in Phase 2)");
    } else {
        // Phase 0/1: Show hardcoded tools
        println!("Available tools (hardcoded):");
        println!("  {} sshmenuc - SSH connection manager (Python)", "•".cyan());
        println!("  {} test-tool - Test tool (Bash)", "•".cyan());
    }

    // Show installed tools
    let tools_dir = tools_dir()?;
    if tools_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&tools_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        if !entries.is_empty() {
            println!("\nInstalled tools:");
            for entry in entries {
                let tool_name = entry.file_name().to_string_lossy().to_string();
                println!("  {} {} (installed)", "✓".green(), tool_name.bold());
            }
        }
    }

    Ok(())
}

/// Show TuxBox status (Phase 0/1: local info, Phase 2: with registry)
pub fn show_status() -> Result<()> {
    use colored::Colorize;

    println!("TuxBox Status");
    println!("=============\n");

    // Base directory
    let base_dir = tuxbox_home()?;
    println!("Base directory: {}", base_dir.display());

    // Tools directory
    let tools_dir = tools_dir()?;
    println!("Tools directory: {}", tools_dir.display());

    // Registry status
    if let Ok(config) = load_config() {
        println!("Registry: {} (initialized)", config.registry_url.green());
    } else {
        println!("Registry: {} (use 'tbox init <url>' to configure)", "not initialized".yellow());
    }

    // Installed tools
    if tools_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&tools_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        if !entries.is_empty() {
            println!("\n{} installed tools:", entries.len());
            for entry in entries {
                let tool_name = entry.file_name().to_string_lossy().to_string();
                println!("  {} {}", "•".cyan(), tool_name.bold());
            }
        } else {
            println!("\n{}", "No tools installed yet.".yellow());
        }
    } else {
        println!("\n{}", "No tools installed yet.".yellow());
    }

    Ok(())
}
