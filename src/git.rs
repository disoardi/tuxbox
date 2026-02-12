//! Git operations using git2 crate

use anyhow::Result;

use crate::config::tools_dir;
use crate::error::TuxBoxError;

/// Clone a Git repository to the tools directory
pub fn clone_tool(tool_name: &str, repo_url: &str, branch: Option<&str>) -> Result<()> {
    let tools_dir = tools_dir()?;
    let tool_path = tools_dir.join(tool_name);

    if tool_path.exists() {
        return Ok(()); // Already cloned
    }

    println!("  → Cloning {} from {}...", tool_name, repo_url);

    // Clone repository
    let mut builder = git2::build::RepoBuilder::new();

    if let Some(branch_name) = branch {
        builder.branch(branch_name);
    }

    builder
        .clone(repo_url, &tool_path)
        .map_err(|e| TuxBoxError::GitError(format!("Clone failed: {}", e)))?;

    println!("  ✓ Cloned successfully");

    Ok(())
}

/// Check if a tool is already cloned
pub fn is_tool_cloned(tool_name: &str) -> Result<bool> {
    let tools_dir = tools_dir()?;
    let tool_path = tools_dir.join(tool_name);

    Ok(tool_path.exists() && tool_path.is_dir())
}

/// Get the path to a tool's directory
pub fn tool_path(tool_name: &str) -> Result<std::path::PathBuf> {
    Ok(tools_dir()?.join(tool_name))
}

/// Update a tool (git pull)
pub fn update_tool(tool_name: &str) -> Result<()> {
    let tool_path = tool_path(tool_name)?;

    if !tool_path.exists() {
        return Err(TuxBoxError::ToolNotFound(tool_name.to_string()).into());
    }

    let repo = git2::Repository::open(&tool_path)
        .map_err(|e| TuxBoxError::GitError(format!("Failed to open repository: {}", e)))?;

    // Fetch origin
    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| TuxBoxError::GitError(format!("Remote 'origin' not found: {}", e)))?;

    remote
        .fetch(&["HEAD"], None, None)
        .map_err(|e| TuxBoxError::GitError(format!("Fetch failed: {}", e)))?;

    println!("  ✓ Updated {}", tool_name);

    Ok(())
}

/// Update all installed tools
pub fn update_all_tools() -> Result<()> {
    let tools_dir = tools_dir()?;

    if !tools_dir.exists() {
        println!("  No tools installed yet.");
        return Ok(());
    }

    for entry in std::fs::read_dir(tools_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let tool_name = entry.file_name().to_string_lossy().to_string();
            update_tool(&tool_name)?;
        }
    }

    Ok(())
}
