//! Registry management for tool definitions
//!
//! Handles cloning, updating, and parsing tool registries.
//! Supports multiple registries with priority-based resolution.

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{AuthType, RegistryConfig, ToolConfig};
use crate::error::TuxBoxError;

/// Registry tools.toml structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    #[serde(default)]
    pub tools: HashMap<String, RegistryTool>,
}

/// Tool definition in registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTool {
    pub name: String,
    pub repo: String,
    pub branch: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub description: Option<String>,
    pub commands: Option<RegistryCommands>,
    pub dependencies: Option<RegistryDependencies>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryCommands {
    pub run: String,
    pub setup: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDependencies {
    pub python: Option<String>,
    pub requirements: Option<String>,
}

/// Clone or update a registry
pub fn sync_registry(registry_config: &RegistryConfig, registry_base_dir: &Path) -> Result<PathBuf> {
    let registry_dir = registry_base_dir.join(&registry_config.name);

    if registry_dir.exists() {
        // Already cloned, do git pull
        println!("  {} Updating registry '{}'...", "→".cyan(), registry_config.name);
        update_registry(&registry_dir)?;
    } else {
        // Clone registry
        println!("  {} Cloning registry '{}'...", "→".cyan(), registry_config.name);
        clone_registry(registry_config, &registry_dir)?;
    }

    Ok(registry_dir)
}

/// Clone a registry repository
fn clone_registry(registry_config: &RegistryConfig, dest: &Path) -> Result<()> {
    use git2::build::RepoBuilder;
    use std::process::Command;

    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).context("Failed to create registry directory")?;
    }

    // Try git2 first (native Rust implementation)
    let git2_result = match registry_config.auth_type {
        AuthType::Ssh => {
            RepoBuilder::new()
                .clone(&registry_config.url, dest)
                .context("git2 SSH clone failed")
        }
        AuthType::Https => {
            RepoBuilder::new()
                .clone(&registry_config.url, dest)
                .context("git2 HTTPS clone failed")
        }
    };

    // If git2 succeeds, we're done
    if git2_result.is_ok() {
        println!("  {} Registry cloned successfully", "✓".green());
        return Ok(());
    }

    // Fallback: use system git command (works better with SSH configs)
    println!("  {} git2 failed, trying system git command...", "→".yellow());

    let status = Command::new("git")
        .args(["clone", &registry_config.url, dest.to_str().unwrap()])
        .status()
        .context("Failed to execute git command")?;

    if !status.success() {
        anyhow::bail!(
            "Failed to clone registry. Both git2 and system git failed.\n\
             URL: {}\n\
             Ensure SSH keys are configured for SSH URLs.",
            registry_config.url
        );
    }

    println!("  {} Registry cloned successfully (via git command)", "✓".green());
    Ok(())
}

/// Update a registry repository (git pull)
fn update_registry(registry_dir: &Path) -> Result<()> {
    use git2::Repository;

    let repo = Repository::open(registry_dir).context("Failed to open registry repository")?;

    // Fetch from origin
    let mut remote = repo.find_remote("origin").context("Failed to find remote 'origin'")?;
    remote
        .fetch(&["main"], None, None)
        .context("Failed to fetch from remote")?;

    // Merge origin/main into current branch
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_up_to_date() {
        println!("  {} Registry already up to date", "✓".green());
    } else if analysis.0.is_fast_forward() {
        // Fast-forward merge
        let refname = "refs/heads/main";
        let mut reference = repo.find_reference(refname)?;
        reference.set_target(fetch_commit.id(), "Fast-forward merge")?;
        repo.set_head(refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        println!("  {} Registry updated successfully", "✓".green());
    } else {
        println!("  {} Registry update requires manual merge", "⚠".yellow());
    }

    Ok(())
}

/// Load tools.toml from a registry directory
pub fn load_registry_tools(registry_dir: &Path) -> Result<Registry> {
    let tools_file = registry_dir.join("tools.toml");

    if !tools_file.exists() {
        return Ok(Registry {
            tools: HashMap::new(),
        });
    }

    let content = fs::read_to_string(&tools_file)
        .with_context(|| format!("Failed to read {}", tools_file.display()))?;

    let registry: Registry = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", tools_file.display()))?;

    Ok(registry)
}

/// Find a tool across all configured registries (priority-based)
pub fn find_tool_in_registries(
    tool_name: &str,
    registries: &[RegistryConfig],
    registry_base_dir: &Path,
) -> Result<(RegistryTool, String)> {
    // Registries are already sorted by priority (highest first)
    for registry_config in registries {
        let registry_dir = registry_base_dir.join(&registry_config.name);

        // Skip if registry not cloned yet
        if !registry_dir.exists() {
            continue;
        }

        // Load tools from registry
        let registry = load_registry_tools(&registry_dir)?;

        // Check if tool exists in this registry
        if let Some(tool) = registry.tools.get(tool_name) {
            return Ok((tool.clone(), registry_config.name.clone()));
        }
    }

    Err(TuxBoxError::ToolNotFound(tool_name.to_string()).into())
}

/// Convert RegistryTool to ToolConfig
pub fn registry_tool_to_config(tool: &RegistryTool) -> ToolConfig {
    ToolConfig {
        name: tool.name.clone(),
        repo: tool.repo.clone(),
        branch: tool.branch.clone(),
        version: tool.version.clone(),
        tool_type: tool.tool_type.clone(),
        isolation: None, // Will be determined by execution strategy
        commands: tool.commands.as_ref().map(|c| crate::config::Commands {
            run: c.run.clone(),
            setup: c.setup.clone(),
        }),
    }
}

/// Sync all configured registries (clone if needed, otherwise update)
pub fn sync_all_registries() -> Result<()> {
    use crate::config;

    let config_result = config::load_config();
    if config_result.is_err() {
        println!("{}", "No registries configured. Use 'tbox init <url>' to add one.".yellow());
        return Ok(());
    }

    let config_data = config_result?;
    if config_data.registries.is_empty() {
        println!("{}", "No registries configured. Use 'tbox init <url>' to add one.".yellow());
        return Ok(());
    }

    let registry_base_dir = config::registry_dir()?;

    for registry_config in &config_data.registries {
        println!();
        println!("{} Registry: {}", "→".cyan(), registry_config.name.bold());
        match sync_registry(registry_config, &registry_base_dir) {
            Ok(_) => {
                // Load and show tool count
                let registry_dir = registry_base_dir.join(&registry_config.name);
                match load_registry_tools(&registry_dir) {
                    Ok(registry) => {
                        println!("  {} {} tools available", "✓".green(), registry.tools.len());
                    }
                    Err(e) => {
                        println!("  {} Failed to load tools: {}", "⚠".yellow(), e);
                    }
                }
            }
            Err(e) => {
                println!("  {} Failed to sync: {}", "✗".red(), e);
            }
        }
    }

    println!();
    println!("{} All registries synced!", "✓".green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_registry_tools_empty() {
        // Should handle non-existent tools.toml gracefully
        let result = load_registry_tools(Path::new("/nonexistent"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().tools.len(), 0);
    }
}
