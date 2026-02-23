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
pub fn sync_registry(
    registry_config: &RegistryConfig,
    registry_base_dir: &Path,
) -> Result<PathBuf> {
    let registry_dir = registry_base_dir.join(&registry_config.name);

    if registry_dir.exists() {
        // Already cloned, do git pull
        println!(
            "  {} Updating registry '{}'...",
            "→".cyan(),
            registry_config.name
        );
        update_registry(registry_config, &registry_dir)?;
    } else {
        // Clone registry
        println!(
            "  {} Cloning registry '{}'...",
            "→".cyan(),
            registry_config.name
        );
        clone_registry(registry_config, &registry_dir)?;
    }

    Ok(registry_dir)
}

/// Run `git pull` via system git in the given directory
fn system_git_pull(dir: &Path) -> Result<()> {
    use std::process::Command;

    let status = Command::new("git")
        .args(["pull", "--ff-only"])
        .current_dir(dir)
        .status()
        .context("Failed to execute git pull")?;

    if !status.success() {
        anyhow::bail!("git pull failed in {}", dir.display());
    }
    Ok(())
}

/// Run `git clone` via system git
fn system_git_clone(url: &str, dest: &Path) -> Result<()> {
    use std::process::Command;

    let status = Command::new("git")
        .args(["clone", url, dest.to_str().unwrap()])
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!(
            "git clone failed.\nURL: {}\nEnsure SSH keys are configured for SSH URLs.",
            url
        );
    }
    Ok(())
}

/// Clone a registry repository
fn clone_registry(registry_config: &RegistryConfig, dest: &Path) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).context("Failed to create registry directory")?;
    }

    match registry_config.auth_type {
        AuthType::Ssh => {
            // SSH: always use system git — git2 has no SSH key callbacks
            system_git_clone(&registry_config.url, dest)?;
            println!("  {} Registry cloned successfully", "✓".green());
        }
        AuthType::Https => {
            // HTTPS: try git2 first (respects proxy settings), fall back to system git
            use git2::build::RepoBuilder;

            let mut builder = RepoBuilder::new();
            let mut fetch_options = git2::FetchOptions::new();
            let mut proxy_opts = git2::ProxyOptions::new();
            proxy_opts.auto();
            fetch_options.proxy_options(proxy_opts);
            builder.fetch_options(fetch_options);

            if builder.clone(&registry_config.url, dest).is_ok() {
                println!("  {} Registry cloned successfully", "✓".green());
            } else {
                println!(
                    "  {} git2 failed, trying system git command...",
                    "→".yellow()
                );
                system_git_clone(&registry_config.url, dest)?;
                println!(
                    "  {} Registry cloned successfully (via git command)",
                    "✓".green()
                );
            }
        }
    }

    Ok(())
}

/// Update a registry repository (git pull)
fn update_registry(registry_config: &RegistryConfig, registry_dir: &Path) -> Result<()> {
    match registry_config.auth_type {
        AuthType::Ssh => {
            // SSH: always use system git — git2 fetch has no SSH key callbacks
            system_git_pull(registry_dir)?;
            println!("  {} Registry updated successfully", "✓".green());
        }
        AuthType::Https => {
            // HTTPS: try git2 first (respects proxy), fall back to system git
            if update_registry_git2(registry_dir).is_err() {
                println!("  {} git2 failed, trying system git pull...", "→".yellow());
                system_git_pull(registry_dir)?;
                println!(
                    "  {} Registry updated successfully (via git command)",
                    "✓".green()
                );
            }
        }
    }

    Ok(())
}

/// Update registry via git2 (HTTPS only — no SSH support)
fn update_registry_git2(registry_dir: &Path) -> Result<()> {
    use git2::Repository;

    let repo = Repository::open(registry_dir).context("Failed to open registry repository")?;

    let mut remote = repo
        .find_remote("origin")
        .context("Failed to find remote 'origin'")?;
    let mut fetch_options = git2::FetchOptions::new();
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    fetch_options.proxy_options(proxy_opts);

    remote
        .fetch(&["main"], Some(&mut fetch_options), None)
        .context("Fetch failed")?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_up_to_date() {
        println!("  {} Registry already up to date", "✓".green());
    } else if analysis.0.is_fast_forward() {
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
        python_version: tool.dependencies.as_ref().and_then(|d| d.python.clone()),
    }
}

/// Sync all configured registries (clone if needed, otherwise update)
pub fn sync_all_registries() -> Result<()> {
    use crate::config;

    let config_result = config::load_config();
    if config_result.is_err() {
        println!(
            "{}",
            "No registries configured. Use 'tbox init <url>' to add one.".yellow()
        );
        return Ok(());
    }

    let config_data = config_result?;
    if config_data.registries.is_empty() {
        println!(
            "{}",
            "No registries configured. Use 'tbox init <url>' to add one.".yellow()
        );
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
