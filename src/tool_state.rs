//! Tool installation state — persists how a tool was set up so that
//! subsequent `tbox run` invocations skip the setup phase entirely.
//!
//! # State file location
//! `~/.tuxbox/tools/<tool>/.tuxbox-state.toml`
//!
//! # Lifecycle
//! - **Written** after a successful first installation (venv created + deps installed)
//! - **Read** at the start of every `tbox run` — if valid, setup is skipped
//! - **Invalidated** (file deleted) when `tbox update` pulls new commits, so the
//!   next run re-installs dependencies against the updated source

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const STATE_FILE: &str = ".tuxbox-state.toml";

/// Persisted installation state for a single tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolState {
    /// Schema version — bump if the format changes incompatibly.
    pub version: String,
    /// How the tool was installed: "venv", "docker", "bash"
    pub method: String,
    /// Venv details (populated when method = "venv").
    pub venv: Option<VenvState>,
}

/// Virtual-environment details saved after a successful `run_in_venv` install.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenvState {
    /// Absolute path to the venv directory.
    pub path: PathBuf,
    /// Python executable used to create the venv (may be a pyenv path).
    pub python: String,
}

impl ToolState {
    /// Construct a state record for a venv-based installation.
    pub fn for_venv(venv_path: PathBuf, python: String) -> Self {
        ToolState {
            version: "1".to_string(),
            method: "venv".to_string(),
            venv: Some(VenvState {
                path: venv_path,
                python,
            }),
        }
    }

    /// Load and validate the state file from the tool directory.
    ///
    /// Returns `None` when:
    /// - The state file does not exist (first run)
    /// - The file cannot be parsed (schema mismatch / corruption)
    /// - The recorded venv directory no longer exists (was deleted)
    pub fn load(tool_path: &Path) -> Option<Self> {
        let state_file = tool_path.join(STATE_FILE);
        if !state_file.exists() {
            return None;
        }

        let content = std::fs::read_to_string(&state_file).ok()?;
        let state: ToolState = toml::from_str(&content).ok()?;

        // Validate: venv directory must still exist
        if let Some(ref venv) = state.venv {
            if !venv.path.exists() {
                return None;
            }
        }

        Some(state)
    }

    /// Persist the state to the tool directory.
    pub fn save(&self, tool_path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(tool_path.join(STATE_FILE), content)?;
        Ok(())
    }

    /// Remove the state file, forcing a full reinstall on the next run.
    /// Called by `tbox update` after pulling new commits.
    pub fn invalidate(tool_path: &Path) {
        let _ = std::fs::remove_file(tool_path.join(STATE_FILE));
    }
}
