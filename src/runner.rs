//! Tool execution logic

use anyhow::{Context, Result};
use std::process::Command;

use crate::config::ToolConfig;
use crate::error::TuxBoxError;
use crate::git;

/// Run a tool (clone if needed, then execute)
///
/// MVP Phase 0: Hardcoded tool config for sshmenuc + simple tools
/// TODO Phase 3: Load from registry
pub fn run_tool(tool_name: &str, args: &[String]) -> Result<()> {
    // MVP: Hardcoded tool configs
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

    // Execute tool
    println!("  → Executing {}...", tool_name);

    let run_command = if let Some(commands) = &tool_config.commands {
        &commands.run
    } else {
        // Default: try to run main script
        return Err(TuxBoxError::ExecutionError(
            "No run command specified for this tool".into(),
        )
        .into());
    };

    // Parse and execute command
    let parts: Vec<&str> = run_command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(TuxBoxError::ExecutionError("Empty run command".into()).into());
    }

    let mut cmd = Command::new(parts[0]);
    cmd.current_dir(&tool_path);

    // Add command parts
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }

    // Add user arguments
    cmd.args(args);

    // Execute
    let status = cmd
        .status()
        .context("Failed to execute tool")?;

    if !status.success() {
        return Err(TuxBoxError::ExecutionError(format!(
            "Tool exited with status: {}",
            status
        ))
        .into());
    }

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
                setup: Some("pip install -r requirements.txt".to_string()),
                run: "python sshmenuc.py".to_string(),
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
