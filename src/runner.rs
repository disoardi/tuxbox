//! Tool execution logic

use anyhow::Result;
use colored::Colorize;

use crate::config::{IsolationStrategy, ToolConfig};
use crate::environment::{ExecutionEnvironment, detect_environment};
use crate::error::TuxBoxError;
use crate::{docker, git, native, python};

/// Run a tool (clone if needed, then execute)
///
/// Execution strategy (in order of precedence):
/// 1. `isolation` field in registry → honour it explicitly
/// 2. Docker available → run in container (default preference)
/// 3. Docker not available → run in Python venv (fallback)
pub fn run_tool(tool_name: &str, args: &[String]) -> Result<()> {
    // Get tool configuration
    let tool_config = get_tool_config(tool_name)?;

    // Native binaries: download from GitHub releases, no git clone needed
    if tool_config.tool_type.as_deref() == Some("native") {
        return native::run_native_tool(&tool_config, args);
    }

    // Clone if not present
    if !git::is_tool_cloned(tool_name)? {
        println!("  Tool not installed, cloning...");
        git::clone_tool(tool_name, &tool_config.repo, tool_config.branch.as_deref())?;
    }

    // Get tool path
    let tool_path = git::tool_path(tool_name)?;

    // Respect explicit isolation declared in the registry; fall back to auto-detect.
    // IsolationStrategy::None means "run directly without container isolation" → LocalVenv.
    let env = match tool_config.isolation {
        Some(IsolationStrategy::Venv) | Some(IsolationStrategy::None) => {
            ExecutionEnvironment::LocalVenv
        }
        Some(IsolationStrategy::Docker) => ExecutionEnvironment::Docker,
        None => detect_environment(),
    };

    // Execute based on environment
    match env {
        ExecutionEnvironment::Docker => {
            docker::run_in_docker(&tool_config, &tool_path, args)?;
        }
        ExecutionEnvironment::LocalVenv => match tool_config.tool_type.as_deref() {
            Some("python") => {
                python::run_in_venv(&tool_config, &tool_path, args)?;
            }
            Some("bash") | Some("script") => {
                run_bash_script(&tool_config, &tool_path, args)?;
            }
            _ => {
                let isolation_hint = match &tool_config.isolation {
                    Some(iso) => format!(
                        " (isolation = {:?} — only python and bash are supported without Docker)",
                        iso
                    ),
                    None => " — please install Docker".to_string(),
                };
                return Err(TuxBoxError::ExecutionError(format!(
                    "Tool type '{}' cannot run locally{}",
                    tool_config.tool_type.as_deref().unwrap_or("unknown"),
                    isolation_hint
                ))
                .into());
            }
        },
    }

    println!("  {} Tool executed successfully", "✓".green());
    Ok(())
}

/// Delete a tool installation completely (with confirmation)
pub fn delete_tool(tool_name: &str) -> Result<()> {
    let tool_path = git::tool_path(tool_name)?;

    if !tool_path.exists() {
        return Err(TuxBoxError::ToolNotFound(format!(
            "'{}' non è installato (directory non trovata)",
            tool_name
        ))
        .into());
    }

    println!(
        "  {} Verranno rimossi: {}",
        "⚠".yellow(),
        tool_path.display().to_string().bold()
    );
    print!("  Confermi? [y/N] ");
    use std::io::Write;
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("  {} Operazione annullata.", "✗".red());
        return Ok(());
    }

    std::fs::remove_dir_all(&tool_path)?;
    println!("  {} Tool '{}' rimosso.", "✓".green(), tool_name.bold());
    Ok(())
}

/// Remove a tool installation and re-setup from scratch (with confirmation)
pub fn reinstall_tool(tool_name: &str) -> Result<()> {
    let tool_path = git::tool_path(tool_name)?;

    if !tool_path.exists() {
        return Err(TuxBoxError::ToolNotFound(format!(
            "'{}' non è installato (directory non trovata)",
            tool_name
        ))
        .into());
    }

    println!(
        "  {} Verranno rimossi e reinstallati: {}",
        "⚠".yellow(),
        tool_path.display().to_string().bold()
    );
    print!("  Confermi? [y/N] ");
    use std::io::Write;
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("  {} Operazione annullata.", "✗".red());
        return Ok(());
    }

    std::fs::remove_dir_all(&tool_path)?;
    println!(
        "  {} Installazione rimossa. Re-setup in corso...",
        "→".cyan()
    );

    run_tool(tool_name, &[])
}

/// Get tool configuration from registry or fallback to hardcoded
///
/// Phase 2: Load from multi-registry with priority-based resolution
/// Fallback: Hardcoded tools for backward compatibility
fn get_tool_config(tool_name: &str) -> Result<ToolConfig> {
    use crate::{config, registry};

    // Try loading from registry first (Phase 2)
    #[allow(clippy::collapsible_if)]
    if let Ok(cfg) = config::load_config() {
        if !cfg.registries.is_empty() {
            let registry_base_dir = config::registry_dir()?;

            // Sync registries if needed (clone/update)
            for registry_config in &cfg.registries {
                let _ = registry::sync_registry(registry_config, &registry_base_dir);
            }

            // Find tool in registries (priority-based)
            match registry::find_tool_in_registries(tool_name, &cfg.registries, &registry_base_dir)
            {
                Ok((tool, registry_name)) => {
                    println!(
                        "  {} Found in registry: {}",
                        "→".cyan(),
                        registry_name.bold()
                    );
                    return Ok(registry::registry_tool_to_config(&tool));
                }
                Err(_) => {
                    // Not found in registry, try hardcoded fallback
                    println!(
                        "  {} Tool not in registry, trying hardcoded...",
                        "→".yellow()
                    );
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
            isolation: Some(IsolationStrategy::Venv),
            commands: Some(crate::config::Commands {
                setup: Some("pip3 install -r requirements.txt".to_string()),
                run: "python3 -m sshmenuc".to_string(),
            }),
            python_version: None,
            system_deps: Some(vec!["openssh-client".to_string()]),
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
            python_version: None,
            system_deps: None,
        }),
        _ => Err(TuxBoxError::ToolNotFound(tool_name.to_string()).into()),
    }
}

/// Run a bash script directly (no Docker, no venv)
fn run_bash_script(
    tool_config: &ToolConfig,
    tool_path: &std::path::Path,
    args: &[String],
) -> Result<()> {
    use std::process::Command;

    // Get run command from config
    let run_cmd = tool_config
        .commands
        .as_ref()
        .map(|c| c.run.as_str())
        .unwrap_or("bash");

    println!("  {} Executing: {}", "→".cyan(), run_cmd.bold());

    // Build command
    let mut cmd = Command::new("bash");
    cmd.current_dir(tool_path);
    cmd.arg("-c");

    // Build full command with args
    let full_cmd = if args.is_empty() {
        run_cmd.to_string()
    } else {
        format!("{} {}", run_cmd, args.join(" "))
    };

    cmd.arg(&full_cmd);

    // Execute
    let status = cmd.status().map_err(|e| {
        TuxBoxError::ExecutionError(format!("Failed to execute bash script: {}", e))
    })?;

    if !status.success() {
        return Err(TuxBoxError::ExecutionError(format!(
            "Script exited with code: {}",
            status.code().unwrap_or(-1)
        ))
        .into());
    }

    Ok(())
}
