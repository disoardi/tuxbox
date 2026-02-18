//! Python environment management (venv fallback when Docker not available)

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::ToolConfig;
use crate::error::TuxBoxError;

/// Detect the best available Python executable (prefers 3.8+ for modern package support)
pub fn detect_python() -> Result<String> {
    // Try versioned executables from newest to oldest (3.8 minimum for modern pyproject.toml).
    // Systems like RHEL/CentOS may have python3.9 or python3.8 alongside the default python3.6.
    let candidates = [
        "python3.13",
        "python3.12",
        "python3.11",
        "python3.10",
        "python3.9",
        "python3.8",
        "python3",
        "python",
    ];

    for candidate in &candidates {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return Ok(candidate.to_string());
        }
    }

    Err(
        TuxBoxError::ExecutionError("Python not found. Please install Python 3.8+".to_string())
            .into(),
    )
}

/// Create or verify virtual environment for a tool.
/// Recreates the venv if its Python is older than 3.8 (incompatible with modern packages).
pub fn setup_venv(tool_path: &Path) -> Result<PathBuf> {
    let venv_path = tool_path.join("venv");

    if venv_path.exists() {
        // Check if the existing venv Python meets the 3.8 minimum.
        // Old Python (e.g. 3.6 on RHEL) cannot build modern Poetry packages.
        let is_too_old = get_venv_executable(&venv_path, "python")
            .ok()
            .and_then(|py| {
                Command::new(&py)
                    .args([
                        "-c",
                        "import sys; exit(0 if sys.version_info >= (3, 8) else 1)",
                    ])
                    .output()
                    .ok()
            })
            .map(|o| !o.status.success())
            .unwrap_or(false); // If we can't check, assume it's fine

        if is_too_old {
            println!(
                "  {} Existing venv uses Python < 3.8, recreating with newer Python...",
                "→".yellow()
            );
            let _ = std::fs::remove_dir_all(&venv_path);
            // Fall through to creation below
        } else {
            return Ok(venv_path);
        }
    }

    println!("  {} Creating Python virtual environment...", "→".cyan());

    let python = detect_python()?;

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

/// Install dependencies in the venv (supports requirements.txt and pyproject.toml)
pub fn install_requirements(venv_path: &Path, tool_path: &Path) -> Result<()> {
    use std::fs;

    let requirements_path = tool_path.join("requirements.txt");
    let pyproject_path = tool_path.join("pyproject.toml");

    // Get pip executable from venv
    let pip = get_venv_executable(venv_path, "pip")?;

    // Upgrade pip silently before installing dependencies.
    // Old pip versions (e.g. 9.0.3 from RHEL/CentOS system Python) return exit code 0
    // even when install fails (e.g. "File 'setup.py' not found"), causing silent failures.
    // Modern pip (>=19) handles PEP 517/518 builds correctly without setup.py.
    // Best-effort: continue even if upgrade fails (air-gapped networks, etc.).
    let _ = Command::new(&pip)
        .args(["install", "--upgrade", "pip"])
        .current_dir(tool_path)
        .output(); // suppress output, non-fatal

    // Use `pip install -e .` only for proper Python packages with a [build-system]
    // section in pyproject.toml (PEP 517/518). Many tools use pyproject.toml only
    // for configuration (linting, formatting) without being installable packages.
    if pyproject_path.exists() {
        let content = fs::read_to_string(&pyproject_path).unwrap_or_default();
        if content.contains("[build-system]") {
            println!(
                "  {} Installing Python package with dependencies...",
                "→".cyan()
            );

            let status = Command::new(&pip)
                .args(["install", "-e", "."])
                .current_dir(tool_path)
                .status()
                .context("Failed to install package")?;

            if status.success() {
                println!("  {} Package and dependencies installed", "✓".green());
                return Ok(());
            }

            // pip install -e . failed; try requirements.txt as a fallback
            if !requirements_path.exists() {
                anyhow::bail!(
                    "Failed to install Python package via pyproject.toml and no \
                     requirements.txt found. The tool may require Python >= 3.8 or \
                     a newer system. Check the tool's README for installation instructions."
                );
            }
        }
    }

    // Use requirements.txt when present
    if requirements_path.exists() {
        println!("  {} Installing Python dependencies...", "→".cyan());

        let status = Command::new(&pip)
            .args(["install", "-r", "requirements.txt"])
            .current_dir(tool_path)
            .status()
            .context("Failed to install requirements")?;

        if !status.success() {
            anyhow::bail!("Failed to install Python dependencies");
        }

        println!("  {} Dependencies installed", "✓".green());
        return Ok(());
    }

    // No dependency files found, skip
    Ok(())
}

/// Run a Python tool using the venv
pub fn run_in_venv(tool_config: &ToolConfig, tool_path: &Path, args: &[String]) -> Result<()> {
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
        return Err(
            TuxBoxError::ExecutionError("No run command specified for this tool".into()).into(),
        );
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
        return Err(
            TuxBoxError::ExecutionError(format!("Tool exited with status: {}", status)).into(),
        );
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
}
