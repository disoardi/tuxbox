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

/// Get hardcoded tool configuration (MVP Phase 0)
///
/// TODO Phase 3: Load from registry instead
fn get_tool_config(tool_name: &str) -> Result<ToolConfig> {
    match tool_name {
        "sshmenuc" => Ok(ToolConfig {
            name: "sshmenuc".to_string(),
            repo: "https://github.com/disoardi/sshmenuc".to_string(),
            branch: Some("main".to_string()),
            tool_type: Some("python".to_string()),
            isolation: None, // Phase 1: will add venv support
            commands: Some(crate::config::Commands {
                setup: Some("pip3 install -r requirements.txt".to_string()),
                run: "python3 -m sshmenuc".to_string(),
            }),
        }),
        "test-tool" => Ok(ToolConfig {
            name: "test-tool".to_string(),
            repo: "https://github.com/your-username/test-tool".to_string(),
            branch: None,
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
