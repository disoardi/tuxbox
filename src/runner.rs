//! Tool execution logic

use anyhow::Result;
use colored::Colorize;

use crate::config::ToolConfig;
use crate::environment::{detect_environment, ExecutionEnvironment};
use crate::error::TuxBoxError;
use crate::{docker, git, python};

/// Run a tool (clone if needed, then execute)
///
/// Intelligent execution strategy:
/// 1. Always check for Docker first (PREFERRED)
/// 2. If Docker available → run in container (full isolation)
/// 3. If Docker not available → run in Python venv (fallback)
pub fn run_tool(tool_name: &str, args: &[String]) -> Result<()> {
    // Get tool configuration
    let tool_config = get_tool_config(tool_name)?;

    // Clone if not present
    if !git::is_tool_cloned(tool_name)? {
        println!("  Tool not installed, cloning...");
        git::clone_tool(
            tool_name,
            &tool_config.repo,
            tool_config.branch.as_deref(),
        )?;
    }

    // Get tool path
    let tool_path = git::tool_path(tool_name)?;

    // Detect execution environment
    let env = detect_environment();

    // Execute based on environment (Docker-first approach)
    match env {
        ExecutionEnvironment::Docker => {
            // PREFERRED: Use Docker for full isolation
            docker::run_in_docker(&tool_config, &tool_path, args)?;
        }
        ExecutionEnvironment::LocalVenv => {
            // FALLBACK: Use local Python venv
            if tool_config.tool_type.as_deref() == Some("python") {
                python::run_in_venv(&tool_config, &tool_path, args)?;
            } else {
                // Non-Python tools without Docker - direct execution
                return Err(TuxBoxError::ExecutionError(
                    format!(
                        "Tool type '{}' requires Docker for execution. Please install Docker.",
                        tool_config.tool_type.as_deref().unwrap_or("unknown")
                    )
                )
                .into());
            }
        }
    }

    println!("  {} Tool executed successfully", "✓".green());
    Ok(())
}

/// Get tool configuration from registry or fallback to hardcoded
///
/// Phase 2: Load from multi-registry with priority-based resolution
/// Fallback: Hardcoded tools for backward compatibility
fn get_tool_config(tool_name: &str) -> Result<ToolConfig> {
    use crate::{config, registry};

    // Try loading from registry first (Phase 2)
    if let Ok(cfg) = config::load_config() {
        if !cfg.registries.is_empty() {
            let registry_base_dir = config::registry_dir()?;

            // Sync registries if needed (clone/update)
            for registry_config in &cfg.registries {
                let _ = registry::sync_registry(registry_config, &registry_base_dir);
            }

            // Find tool in registries (priority-based)
            match registry::find_tool_in_registries(tool_name, &cfg.registries, &registry_base_dir) {
                Ok((tool, registry_name)) => {
                    println!("  {} Found in registry: {}", "→".cyan(), registry_name.bold());
                    return Ok(registry::registry_tool_to_config(&tool));
                }
                Err(_) => {
                    // Not found in registry, try hardcoded fallback
                    println!("  {} Tool not in registry, trying hardcoded...", "→".yellow());
                }
            }
        }
    }

    // Fallback to hardcoded tools (Phase 0/1 compatibility)
    get_hardcoded_tool_config(tool_name)
}

/// Get hardcoded tool configuration (backward compatibility)
fn get_hardcoded_tool_config(tool_name: &str) -> Result<ToolConfig> {
    match tool_name {
        "sshmenuc" => Ok(ToolConfig {
            name: "sshmenuc".to_string(),
            repo: "https://github.com/disoardi/sshmenuc".to_string(),
            branch: Some("main".to_string()),
            version: Some("1.1.0".to_string()),
            tool_type: Some("python".to_string()),
            isolation: None,
            commands: Some(crate::config::Commands {
                setup: Some("pip3 install -r requirements.txt".to_string()),
                run: "python3 -m sshmenuc".to_string(),
            }),
        }),
        "test-tool" => Ok(ToolConfig {
            name: "test-tool".to_string(),
            repo: "https://github.com/your-username/test-tool".to_string(),
            branch: None,
            version: Some("0.1.0".to_string()),
            tool_type: Some("bash".to_string()),
            isolation: None,
            commands: Some(crate::config::Commands {
                setup: None,
                run: "./run.sh".to_string(),
            }),
        }),
        _ => Err(TuxBoxError::ToolNotFound(tool_name.to_string()).into()),
    }
}
