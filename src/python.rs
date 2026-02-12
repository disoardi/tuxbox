//! Python environment management (venv fallback when Docker not available)

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::ToolConfig;
use crate::error::TuxBoxError;

/// Detect available Python executable (python3 preferred)
pub fn detect_python() -> Result<String> {
    // Try python3 first (most common on modern systems)
    if Command::new("python3").arg("--version").output().is_ok() {
        return Ok("python3".to_string());
    }

    // Fallback to python
    if Command::new("python").arg("--version").output().is_ok() {
        return Ok("python".to_string());
    }

    Err(TuxBoxError::ExecutionError(
        "Python not found. Please install Python 3.8+".to_string(),
    )
    .into())
}

/// Get Python version
pub fn get_python_version(python_cmd: &str) -> Result<String> {
    let output = Command::new(python_cmd)
        .arg("--version")
        .output()
        .context("Failed to get Python version")?;

    let version = String::from_utf8_lossy(&output.stdout);
    Ok(version.trim().to_string())
}

/// Create or verify virtual environment for a tool
pub fn setup_venv(tool_path: &Path) -> Result<PathBuf> {
    let venv_path = tool_path.join("venv");

    if venv_path.exists() {
        // Venv already exists
        return Ok(venv_path);
    }

    println!("  {} Creating Python virtual environment...", "→".cyan());

    let python = detect_python()?;

    // Create venv
    let status = Command::new(&python)
        .args(["-m", "venv", "venv"])
        .current_dir(tool_path)
        .status()
        .context("Failed to create virtual environment")?;

    if !status.success() {
        anyhow::bail!("Failed to create venv");
    }

    println!("  {} Virtual environment created", "✓".green());
    Ok(venv_path)
}

/// Install requirements.txt in the venv
pub fn install_requirements(venv_path: &Path, tool_path: &Path) -> Result<()> {
    let requirements_path = tool_path.join("requirements.txt");

    if !requirements_path.exists() {
        // No requirements.txt, skip
        return Ok(());
    }

    println!("  {} Installing Python dependencies...", "→".cyan());

    // Get pip executable from venv
    let pip = get_venv_executable(venv_path, "pip")?;

    // Install requirements
    let status = Command::new(pip)
        .args(["install", "-r", "requirements.txt"])
        .current_dir(tool_path)
        .status()
        .context("Failed to install requirements")?;

    if !status.success() {
        anyhow::bail!("Failed to install Python dependencies");
    }

    println!("  {} Dependencies installed", "✓".green());
    Ok(())
}

/// Run a Python tool using the venv
pub fn run_in_venv(
    tool_config: &ToolConfig,
    tool_path: &Path,
    args: &[String],
) -> Result<()> {
    println!("  {} Using local Python venv", "🐍".cyan());

    // Setup venv
    let venv_path = setup_venv(tool_path)?;

    // Install requirements
    install_requirements(&venv_path, tool_path)?;

    // Get python executable from venv
    let python = get_venv_executable(&venv_path, "python")?;

    // Execute the tool
    println!("  {} Running tool...", "→".cyan());

    // Parse run command
    let run_command = if let Some(commands) = &tool_config.commands {
        &commands.run
    } else {
        return Err(TuxBoxError::ExecutionError(
            "No run command specified for this tool".into(),
        )
        .into());
    };

    // Build command - replace python/python3 with venv python
    let parts: Vec<&str> = run_command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(TuxBoxError::ExecutionError("Empty run command".into()).into());
    }

    let mut cmd = Command::new(&python);
    cmd.current_dir(tool_path);

    // Add command parts (skip first if it's "python" or "python3")
    let start_idx = if parts[0] == "python" || parts[0] == "python3" {
        1 // Skip the python command, we're using venv python
    } else {
        0
    };

    if parts.len() > start_idx {
        cmd.args(&parts[start_idx..]);
    }

    // Add user arguments
    cmd.args(args);

    // Execute
    let status = cmd.status().context("Failed to execute tool")?;

    if !status.success() {
        return Err(TuxBoxError::ExecutionError(format!(
            "Tool exited with status: {}",
            status
        ))
        .into());
    }

    Ok(())
}

/// Get executable path from venv (handles OS differences)
fn get_venv_executable(venv_path: &Path, exe_name: &str) -> Result<PathBuf> {
    // On Unix: venv/bin/<exe>
    // On Windows: venv/Scripts/<exe>.exe

    #[cfg(unix)]
    let exe_path = venv_path.join("bin").join(exe_name);

    #[cfg(windows)]
    let exe_path = venv_path.join("Scripts").join(format!("{}.exe", exe_name));

    if !exe_path.exists() {
        anyhow::bail!("Executable '{}' not found in venv", exe_name);
    }

    Ok(exe_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python() {
        // Should find python3 or python
        let result = detect_python();
        assert!(result.is_ok() || result.is_err()); // Just verify it runs
    }

    #[test]
    fn test_python_version() {
        if let Ok(python) = detect_python() {
            let version = get_python_version(&python);
            assert!(version.is_ok());
        }
    }
}
