//! Configuration management and Context struct (2026 pattern)

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::TuxBoxError;

/// TuxBox home directory (~/.tuxbox)
pub fn tuxbox_home() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| TuxBoxError::ConfigError("Home directory not found".into()))?;
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

/// TuxBox configuration with multi-registry support
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub registries: Vec<RegistryConfig>,

    // Legacy support (Phase 1 compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
}

/// Individual registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
    #[serde(default = "default_priority")]
    pub priority: u32,
    #[serde(default)]
    pub auth_type: AuthType,
}

fn default_priority() -> u32 {
    100
}

/// Authentication type for registry
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    #[default]
    Ssh,
    Https,
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

/// Initialize TuxBox configuration with a registry
pub fn init_config(registry_url: &str) -> Result<()> {
    let tuxbox_home = tuxbox_home()?;

    // Create directories
    fs::create_dir_all(&tuxbox_home).context("Failed to create TuxBox home directory")?;
    fs::create_dir_all(tools_dir()?).context("Failed to create tools directory")?;
    fs::create_dir_all(registry_dir()?).context("Failed to create registry directory")?;

    // Detect auth type from URL
    let auth_type = if registry_url.starts_with("git@") || registry_url.starts_with("ssh://") {
        AuthType::Ssh
    } else {
        AuthType::Https
    };

    // Extract registry name from URL (use last component or "default")
    let name = extract_registry_name(registry_url);

    // Load existing config or create new one
    let mut config = load_config().unwrap_or_else(|_| Config {
        registries: Vec::new(),
        registry_url: None,
    });

    // Add or update registry
    if let Some(existing) = config.registries.iter_mut().find(|r| r.name == name) {
        existing.url = registry_url.to_string();
        existing.auth_type = auth_type;
    } else {
        config.registries.push(RegistryConfig {
            name,
            url: registry_url.to_string(),
            priority: 100,
            auth_type,
        });
    }

    // Sort by priority (higher priority first)
    config
        .registries
        .sort_by(|a, b| b.priority.cmp(&a.priority));

    // Write config
    let config_toml = toml::to_string_pretty(&config)?;
    fs::write(config_file()?, config_toml).context("Failed to write config file")?;

    Ok(())
}

/// Extract a registry name from its URL
fn extract_registry_name(url: &str) -> String {
    // Extract from URLs like:
    // - git@github.com:user/repo.git -> repo
    // - https://github.com/user/repo -> repo
    // - https://github.com/user/repo.git -> repo

    let cleaned = url.trim_end_matches(".git").trim_end_matches('/');

    if let Some(name) = cleaned.rsplit('/').next() {
        name.to_string()
    } else if let Some(name) = cleaned.rsplit(':').next() {
        name.rsplit('/').next().unwrap_or("default").to_string()
    } else {
        "default".to_string()
    }
}

/// Load configuration (with backward compatibility)
pub fn load_config() -> Result<Config> {
    let config_path = config_file()?;

    if !config_path.exists() {
        return Err(TuxBoxError::NotInitialized.into());
    }

    let config_str = fs::read_to_string(config_path)?;
    let mut config: Config = toml::from_str(&config_str)?;

    // Backward compatibility: migrate old registry_url to new format
    #[allow(clippy::collapsible_if)]
    if let Some(ref legacy_url) = config.registry_url {
        if config.registries.is_empty() {
            let auth_type = if legacy_url.starts_with("git@") || legacy_url.starts_with("ssh://") {
                AuthType::Ssh
            } else {
                AuthType::Https
            };

            config.registries.push(RegistryConfig {
                name: "default".to_string(),
                url: legacy_url.clone(),
                priority: 100,
                auth_type,
            });
        }
    }

    Ok(config)
}

/// List available tools (Phase 0/1: hardcoded, Phase 2: from registry)
pub fn list_tools() -> Result<()> {
    use colored::Colorize;

    // Try to load registry config (Phase 2)
    if let Ok(config) = load_config() {
        if !config.registries.is_empty() {
            println!("{}", "Configured registries:".bold());
            for registry in &config.registries {
                let auth_icon = match registry.auth_type {
                    AuthType::Ssh => "🔐",
                    AuthType::Https => "🌐",
                };
                println!(
                    "  {} {} (priority: {}) - {}",
                    auth_icon,
                    registry.name.cyan().bold(),
                    registry.priority,
                    registry.url.dimmed()
                );
            }

            // Load and display tools from all registries
            use crate::registry;
            let registry_base_dir = registry_dir()?;
            let mut all_tools = std::collections::HashMap::new();

            for registry_config in &config.registries {
                let registry_dir_path = registry_base_dir.join(&registry_config.name);
                #[allow(clippy::collapsible_if)]
                if registry_dir_path.exists() {
                    if let Ok(registry_data) = registry::load_registry_tools(&registry_dir_path) {
                        for (tool_name, tool_info) in registry_data.tools {
                            all_tools
                                .entry(tool_name.clone())
                                .or_insert((tool_info, registry_config.name.clone()));
                        }
                    }
                }
            }

            if !all_tools.is_empty() {
                println!("\n{}", "Available tools from registries:".bold());
                for (tool_name, (tool_info, registry_name)) in all_tools {
                    let desc = tool_info.description.as_deref().unwrap_or("No description");
                    println!(
                        "  {} {} - {} {}",
                        "•".cyan(),
                        tool_name.green().bold(),
                        desc,
                        format!("(from {})", registry_name).dimmed()
                    );
                }
            } else {
                println!(
                    "\n{}",
                    "No tools found in registries. Run 'tbox registry sync' to fetch.".yellow()
                );
            }
        }
    } else {
        // Not initialized - guide user to init
        println!("{}", "TuxBox not initialized.".yellow());
        println!("\nTo get started:");
        println!("  {} Initialize with a registry:", "1.".cyan());
        println!("     tbox init <registry-url>");
        println!("\n  {} Example:", "2.".cyan());
        println!("     tbox init git@github.com:user/tuxbox-registry.git");
        println!("\nFor help creating a registry, see: docs/QUICK_START.md");
        return Ok(());
    }

    // Show installed tools
    let tools_dir = tools_dir()?;
    if tools_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&tools_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        if !entries.is_empty() {
            println!("\n{}", "Installed tools:".bold());
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
        if !config.registries.is_empty() {
            println!(
                "\n{} ({} configured):",
                "Registries:".bold(),
                config.registries.len()
            );
            for registry in &config.registries {
                let auth_icon = match registry.auth_type {
                    AuthType::Ssh => "🔐",
                    AuthType::Https => "🌐",
                };
                println!(
                    "  {} {} (priority: {}) - {}",
                    auth_icon,
                    registry.name.green(),
                    registry.priority,
                    registry.url.dimmed()
                );
            }
        } else {
            println!("\n{} {}", "Registries:".bold(), "none configured".yellow());
        }
    } else {
        println!(
            "\n{} {} (use 'tbox init <url>' to configure)",
            "Registries:".bold(),
            "not initialized".yellow()
        );
    }

    // Installed tools
    if tools_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&tools_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        if !entries.is_empty() {
            println!("\n{} {} installed tools:", "Tools:".bold(), entries.len());
            for entry in entries {
                let tool_name = entry.file_name().to_string_lossy().to_string();
                println!("  {} {}", "•".cyan(), tool_name.bold());
            }
        } else {
            println!(
                "\n{} {}",
                "Tools:".bold(),
                "No tools installed yet.".yellow()
            );
        }
    } else {
        println!(
            "\n{} {}",
            "Tools:".bold(),
            "No tools installed yet.".yellow()
        );
    }

    Ok(())
}

/// Add a new registry to configuration
pub fn add_registry(name: &str, url: &str, priority: Option<u32>) -> Result<()> {
    use colored::Colorize;

    let mut config = load_config().unwrap_or_else(|_| Config {
        registries: Vec::new(),
        registry_url: None,
    });

    // Check if registry with this name already exists
    if config.registries.iter().any(|r| r.name == name) {
        anyhow::bail!(
            "Registry '{}' already exists. Use a different name or remove it first.",
            name
        );
    }

    // Detect auth type
    let auth_type = if url.starts_with("git@") || url.starts_with("ssh://") {
        AuthType::Ssh
    } else {
        AuthType::Https
    };

    // Add registry
    config.registries.push(RegistryConfig {
        name: name.to_string(),
        url: url.to_string(),
        priority: priority.unwrap_or(100),
        auth_type,
    });

    // Sort by priority
    config
        .registries
        .sort_by(|a, b| b.priority.cmp(&a.priority));

    // Save config
    let config_toml = toml::to_string_pretty(&config)?;
    fs::write(config_file()?, config_toml)?;

    println!(
        "{} Registry '{}' added successfully!",
        "✓".green(),
        name.green().bold()
    );
    Ok(())
}

/// Remove a registry from configuration
pub fn remove_registry(name: &str) -> Result<()> {
    use colored::Colorize;

    let mut config = load_config()?;

    let initial_len = config.registries.len();
    config.registries.retain(|r| r.name != name);

    if config.registries.len() == initial_len {
        anyhow::bail!("Registry '{}' not found", name);
    }

    // Save config
    let config_toml = toml::to_string_pretty(&config)?;
    fs::write(config_file()?, config_toml)?;

    println!(
        "{} Registry '{}' removed successfully!",
        "✓".green(),
        name.yellow()
    );
    Ok(())
}

/// List all configured registries
pub fn list_registries() -> Result<()> {
    use colored::Colorize;

    let config = load_config()?;

    if config.registries.is_empty() {
        println!(
            "{}",
            "No registries configured. Use 'tbox init <url>' to add one.".yellow()
        );
        return Ok(());
    }

    println!("{}", "Configured registries:".bold());
    println!();

    for registry in &config.registries {
        let auth_icon = match registry.auth_type {
            AuthType::Ssh => "🔐",
            AuthType::Https => "🌐",
        };
        let auth_label = match registry.auth_type {
            AuthType::Ssh => "SSH",
            AuthType::Https => "HTTPS",
        };

        println!(
            "{} {} (priority: {})",
            auth_icon,
            registry.name.cyan().bold(),
            registry.priority
        );
        println!("  {} URL: {}", "→".dimmed(), registry.url.dimmed());
        println!("  {} Auth: {}", "→".dimmed(), auth_label.dimmed());
        println!();
    }

    Ok(())
}
