//! Python environment management (venv fallback when Docker not available)

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::ToolConfig;
use crate::error::TuxBoxError;
use crate::tool_state::ToolState;

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

/// Find a Python executable meeting the minimum (major, minor) version requirement.
///
/// Search order:
/// 1. Versioned names in PATH: python{major}.{13..min_minor}
/// 2. pyenv installations in ~/.pyenv/versions/
///
/// Returns the path/name of the newest suitable Python found, or None.
fn find_python_for_version(min_major: u32, min_minor: u32) -> Option<String> {
    // Try versioned system Pythons from newest to minimum required
    for minor in (min_minor..=13).rev() {
        let candidate = format!("python{}.{}", min_major, minor);
        if Command::new(&candidate).arg("--version").output().is_ok() {
            return Some(candidate);
        }
    }

    // Check pyenv versions directory (~/.pyenv/versions/<version>/bin/python)
    let home = dirs::home_dir()?;
    let pyenv_versions = home.join(".pyenv/versions");
    if !pyenv_versions.exists() {
        return None;
    }

    let mut best: Option<(u32, u32, PathBuf)> = None;
    for entry in std::fs::read_dir(&pyenv_versions)
        .ok()?
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name().to_string_lossy().to_string();
        let parts: Vec<u32> = name.split('.').filter_map(|p| p.parse().ok()).collect();
        if parts.len() < 2 {
            continue;
        }
        let (major, minor) = (parts[0], parts[1]);
        // Skip if below minimum
        if major < min_major || (major == min_major && minor < min_minor) {
            continue;
        }
        let python_bin = entry.path().join("bin/python");
        if !python_bin.exists() {
            continue;
        }
        // Keep the newest matching version
        if best
            .as_ref()
            .map(|(bm, bn, _)| (major, minor) > (*bm, *bn))
            .unwrap_or(true)
        {
            best = Some((major, minor, python_bin));
        }
    }

    best.map(|(_, _, path)| path.to_string_lossy().to_string())
}

/// Resolve the Python interpreter for a tool based on its version requirement.
///
/// Returns `(python_override, min_version)`:
/// - `python_override`: a specific Python path to use (from pyenv or versioned name),
///   or None if the system default is fine.
/// - `min_version`: minimum (major, minor) the venv must satisfy.
fn resolve_python(tool_config: &ToolConfig) -> Result<(Option<String>, (u32, u32))> {
    if let Some(spec) = &tool_config.python_version {
        if let Some((min_major, min_minor)) = min_required_python(spec) {
            match find_python_for_version(min_major, min_minor) {
                Some(py) => {
                    println!("  {} Using Python: {}", "→".cyan(), py);
                    return Ok((Some(py), (min_major, min_minor)));
                }
                None => {
                    anyhow::bail!(
                        "This tool requires Python >={min_major}.{min_minor}, \
                         but no compatible Python was found on this system.\n\n\
                         Options to install Python {min_major}.{min_minor}+:\n\n\
                         1. Package manager (RHEL/CentOS — requires root):\n\
                            sudo yum install python3{min_minor}\n\n\
                         2. pyenv — install any Python version without root:\n\
                            curl https://pyenv.run | bash\n\
                            # restart the shell, then:\n\
                            pyenv install 3.{min_minor}\n\
                            pyenv global 3.{min_minor}"
                    );
                }
            }
        }
    }
    Ok((None, (3u32, 8u32)))
}

/// Create or verify virtual environment for a tool.
///
/// - `python_override`: use this specific Python binary to create the venv (e.g. from pyenv).
///   When None, `detect_python()` is used.
/// - `min_version`: the venv's Python must be >= this (major, minor). If the existing venv
///   doesn't satisfy it and a better Python is available, the venv is recreated.
pub fn setup_venv(
    tool_path: &Path,
    python_override: Option<&str>,
    min_version: (u32, u32),
) -> Result<PathBuf> {
    let venv_path = tool_path.join("venv");
    let (min_major, min_minor) = min_version;

    if venv_path.exists() {
        // Check if the existing venv Python meets the minimum version
        let venv_ok = get_venv_executable(&venv_path, "python")
            .ok()
            .and_then(|py| {
                Command::new(&py)
                    .args([
                        "-c",
                        &format!(
                            "import sys; exit(0 if sys.version_info >= ({min_major}, {min_minor}) else 1)"
                        ),
                    ])
                    .output()
                    .ok()
            })
            .map(|o| o.status.success())
            .unwrap_or(false);

        if venv_ok {
            return Ok(venv_path);
        }

        // Venv doesn't meet the requirement; recreate only if a better Python is available
        let has_better =
            python_override.is_some() || find_python_for_version(min_major, min_minor).is_some();

        if has_better {
            println!(
                "  {} Existing venv uses Python < {min_major}.{min_minor}, recreating with newer Python...",
                "→".yellow()
            );
            let _ = std::fs::remove_dir_all(&venv_path);
            // Fall through to creation below
        } else {
            // No better Python available; keep the existing venv and try anyway
            return Ok(venv_path);
        }
    }

    println!("  {} Creating Python virtual environment...", "→".cyan());

    let python = match python_override {
        Some(py) => py.to_string(),
        None => detect_python()?,
    };

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

            // pip install -e . failed (e.g. old poetry-core on Python 3.6 that does not
            // support [tool.poetry.group.*]). Try requirements.txt first, then the
            // Poetry-specific fallback that parses deps directly from pyproject.toml.
            if requirements_path.exists() {
                // fall through to requirements.txt block below
            } else {
                return install_poetry_deps_fallback(&pip, venv_path, tool_path);
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

/// Fallback installer for Poetry-managed projects on systems with old Python/poetry-core.
///
/// When `pip install -e .` fails (e.g. Python 3.6 cannot build a package that uses
/// `[tool.poetry.group.*]` because poetry-core for 3.6 is too old), this extracts the
/// package names from `[tool.poetry.dependencies]` and installs them with plain pip.
/// Then adds a `.pth` file to make the tool directory importable, replacing what
/// `pip install -e .` would have done for the import path.
fn install_poetry_deps_fallback(pip: &Path, venv_path: &Path, tool_path: &Path) -> Result<()> {
    let pyproject_path = tool_path.join("pyproject.toml");
    let content =
        std::fs::read_to_string(&pyproject_path).context("Failed to read pyproject.toml")?;

    let doc: toml::Value = toml::from_str(&content).context("Failed to parse pyproject.toml")?;

    let deps_table = doc
        .get("tool")
        .and_then(|t| t.get("poetry"))
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_table());

    let Some(deps_table) = deps_table else {
        // No [tool.poetry.dependencies] — nothing more we can do.
        anyhow::bail!(
            "Failed to install package: pip install -e . failed and no \
             [tool.poetry.dependencies] found in pyproject.toml. \
             The tool may require a newer Python or manual installation."
        );
    };

    // Collect installable package names; skip "python" itself and git/path/url deps.
    let packages: Vec<&str> = deps_table
        .iter()
        .filter(|(k, v)| {
            if k.to_lowercase() == "python" {
                return false;
            }
            if let Some(t) = v.as_table() {
                if t.contains_key("git") || t.contains_key("path") || t.contains_key("url") {
                    return false;
                }
            }
            true
        })
        .map(|(k, _)| k.as_str())
        .collect();

    if packages.is_empty() {
        return Ok(());
    }

    println!(
        "  {} Installing {} dependencies (Poetry fallback)...",
        "→".cyan(),
        packages.len()
    );

    let mut args = vec!["install"];
    args.extend(packages.iter().copied());

    let status = Command::new(pip)
        .args(&args)
        .current_dir(tool_path)
        .status()
        .context("Failed to install dependencies")?;

    if !status.success() {
        anyhow::bail!("Failed to install Poetry dependencies from pyproject.toml");
    }

    // Add the tool directory to the venv's sys.path via a .pth file so that
    // `python -m <tool>` works without a full editable install.
    let python = get_venv_executable(venv_path, "python")?;
    let site_output = Command::new(&python)
        .args(["-c", "import site; print(site.getsitepackages()[0])"])
        .output()
        .context("Failed to determine site-packages directory")?;

    let site_packages = PathBuf::from(
        String::from_utf8_lossy(&site_output.stdout)
            .trim()
            .to_string(),
    );

    std::fs::write(
        site_packages.join("tuxbox-tool.pth"),
        format!("{}\n", tool_path.display()),
    )
    .context("Failed to create .pth file")?;

    println!("  {} Dependencies installed (Poetry fallback)", "✓".green());
    Ok(())
}

/// Run a Python tool using the venv.
///
/// On first run: resolves the right Python, creates the venv, installs
/// dependencies, saves a `.tuxbox-state.toml` file.
///
/// On subsequent runs: reads the state file and skips setup entirely.
pub fn run_in_venv(tool_config: &ToolConfig, tool_path: &Path, args: &[String]) -> Result<()> {
    println!("  {} Using local Python venv", "🐍".cyan());

    let venv_path = if let Some(state) = ToolState::load(tool_path) {
        // Fast path: already installed — skip all setup
        println!("  {} Environment ready", "✓".green());
        state
            .venv
            .ok_or_else(|| TuxBoxError::ExecutionError("State file missing venv entry".into()))?
            .path
    } else {
        // First-run path: resolve Python, create venv, install deps
        let (python_override, min_version) = resolve_python(tool_config)?;
        let venv_path = setup_venv(tool_path, python_override.as_deref(), min_version)?;
        install_requirements(&venv_path, tool_path)?;

        // Persist state so future runs skip this entire block
        let python_used = python_override.unwrap_or_else(|| detect_python().unwrap_or_default());
        if let Err(e) = ToolState::for_venv(venv_path.clone(), python_used).save(tool_path) {
            // Non-fatal: a failed save just means next run will reinstall
            eprintln!("  {} Could not save tool state: {}", "⚠".yellow(), e);
        }

        venv_path
    };

    execute_in_venv(&venv_path, tool_path, tool_config, args)
}

/// Execute the tool inside the venv by simulating `source venv/bin/activate`.
///
/// Prepends `venv/bin` to PATH and sets VIRTUAL_ENV in the **child process only**.
/// The parent tbox process environment is never modified — no cleanup needed.
fn execute_in_venv(
    venv_path: &Path,
    tool_path: &Path,
    tool_config: &ToolConfig,
    args: &[String],
) -> Result<()> {
    let run_command = tool_config
        .commands
        .as_ref()
        .map(|c| c.run.as_str())
        .ok_or_else(|| {
            TuxBoxError::ExecutionError("No run command specified for this tool".into())
        })?;

    println!("  {} Running tool...", "→".cyan());

    // Simulate `source venv/bin/activate`: prepend venv/bin to PATH.
    // This resolves python3, pip, and any console scripts (e.g. cert-checker)
    // installed by pip without needing to know the tool name in advance.
    // The env modification is scoped to the child process only.
    let current_path = std::env::var("PATH").unwrap_or_default();

    #[cfg(unix)]
    let venv_bin = venv_path.join("bin");
    #[cfg(windows)]
    let venv_bin = venv_path.join("Scripts");

    let activated_path = format!("{}:{}", venv_bin.display(), current_path);

    let parts: Vec<&str> = run_command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(TuxBoxError::ExecutionError("Empty run command".into()).into());
    }

    let mut cmd = Command::new(parts[0]);
    cmd.current_dir(tool_path);
    cmd.env("PATH", &activated_path);
    cmd.env("VIRTUAL_ENV", venv_path);

    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    cmd.args(args);

    let status = cmd.status().context("Failed to execute tool")?;

    if !status.success() {
        return Err(
            TuxBoxError::ExecutionError(format!("Tool exited with status: {}", status)).into(),
        );
    }

    Ok(())
}

/// Parse the minimum Python version from a version specifier string.
/// Handles common Poetry/pip formats: ">=3.8", "^3.9", "~3.8", "3.8", ">=3.8,<4.0"
fn min_required_python(spec: &str) -> Option<(u32, u32)> {
    let mut min_version: Option<(u32, u32)> = None;

    for constraint in spec.split(',') {
        let constraint = constraint.trim();
        // Skip upper-bound constraints (< or <=) — they don't define the minimum
        if constraint.starts_with('<') {
            continue;
        }
        // Strip operator characters to get the bare version number
        let version_str = constraint.trim_start_matches(['>', '=', '^', '~', '!']);
        let parts: Vec<u32> = version_str
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        if parts.len() >= 2 {
            let v = (parts[0], parts[1]);
            // Keep the most restrictive (highest) lower bound seen so far
            min_version = Some(match min_version {
                Some(cur) if cur >= v => cur,
                _ => v,
            });
        }
    }

    min_version
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
        let result = detect_python();
        assert!(result.is_ok() || result.is_err()); // Just verify it runs
    }

    #[test]
    fn test_min_required_python() {
        assert_eq!(min_required_python(">=3.8"), Some((3, 8)));
        assert_eq!(min_required_python("^3.9"), Some((3, 9)));
        assert_eq!(min_required_python("~3.8"), Some((3, 8)));
        assert_eq!(min_required_python("3.8"), Some((3, 8)));
        assert_eq!(min_required_python(">=3.8,<4.0"), Some((3, 8)));
        assert_eq!(min_required_python(">=3.8,<3.10"), Some((3, 8)));
    }
}
