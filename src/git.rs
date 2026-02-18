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

    // For SSH URLs, use system git (handles SSH keys automatically)
    if repo_url.starts_with("git@") || repo_url.starts_with("ssh://") {
        return clone_with_system_git(repo_url, &tool_path, branch);
    }

    // For HTTPS URLs, try git2 with proxy auto-detection first
    let mut builder = git2::build::RepoBuilder::new();

    // Auto-detect proxy from environment variables (http_proxy, https_proxy, etc.)
    let mut fetch_options = git2::FetchOptions::new();
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    fetch_options.proxy_options(proxy_opts);
    builder.fetch_options(fetch_options);

    if let Some(branch_name) = branch {
        builder.branch(branch_name);
    }

    if builder.clone(repo_url, &tool_path).is_ok() {
        println!("  ✓ Cloned successfully");
        return Ok(());
    }

    // Fallback: system git handles proxy env vars natively
    clone_with_system_git(repo_url, &tool_path, branch)
}

/// Clone using system git command (for SSH URLs)
fn clone_with_system_git(
    repo_url: &str,
    dest: &std::path::Path,
    branch: Option<&str>,
) -> Result<()> {
    use std::process::Command;

    let mut cmd = Command::new("git");
    cmd.arg("clone");

    if let Some(branch_name) = branch {
        cmd.args(["--branch", branch_name]);
    }

    cmd.arg(repo_url);
    cmd.arg(dest);

    let output = cmd
        .output()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to execute git command: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TuxBoxError::GitError(format!("Git clone failed: {}", stderr.trim())).into());
    }

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

/// Update a tool (git pull with safety checks)
pub fn update_tool(tool_name: &str) -> Result<()> {
    use colored::Colorize;

    let tool_path = tool_path(tool_name)?;

    if !tool_path.exists() {
        return Err(TuxBoxError::ToolNotFound(tool_name.to_string()).into());
    }

    let repo = git2::Repository::open(&tool_path)
        .map_err(|e| TuxBoxError::GitError(format!("Failed to open repository: {}", e)))?;

    // Check if working directory is clean (tracked files only — excludes venv/, etc.)
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(false);
    status_opts.include_ignored(false);

    let statuses = repo
        .statuses(Some(&mut status_opts))
        .map_err(|e| TuxBoxError::GitError(format!("Failed to check repository status: {}", e)))?;

    if !statuses.is_empty() {
        eprintln!("{}", "  ⚠ Warning: Tool has uncommitted changes".yellow());
        eprintln!("  {} Update skipped to prevent data loss", "→".cyan());
        eprintln!("  {} To force update, remove and reinstall:", "→".cyan());
        eprintln!(
            "    {}",
            format!("rm -rf {}", tool_path.display()).bright_black()
        );
        eprintln!("    {}", format!("tbox run {}", tool_name).bright_black());
        return Ok(());
    }

    // Get current HEAD
    let head = repo
        .head()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to get HEAD reference: {}", e)))?;
    let current_commit = head
        .peel_to_commit()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to get current commit: {}", e)))?;
    let current_oid = current_commit.id();

    // Fetch from remote
    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| TuxBoxError::GitError(format!("Remote 'origin' not found: {}", e)))?;

    let mut fetch_options = git2::FetchOptions::new();
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    fetch_options.proxy_options(proxy_opts);

    remote
        .fetch(
            &["refs/heads/*:refs/remotes/origin/*"],
            Some(&mut fetch_options),
            None,
        )
        .map_err(|e| TuxBoxError::GitError(format!("Fetch failed: {}", e)))?;

    // Get remote HEAD (assuming main/master branch)
    let branch_name = head
        .shorthand()
        .ok_or_else(|| TuxBoxError::GitError("Failed to get branch name".to_string()))?;
    let remote_branch_name = format!("origin/{}", branch_name);
    let remote_ref = repo
        .find_reference(&format!("refs/remotes/{}", remote_branch_name))
        .map_err(|e| {
            TuxBoxError::GitError(format!(
                "Remote branch '{}' not found: {}",
                remote_branch_name, e
            ))
        })?;
    let remote_commit = remote_ref
        .peel_to_commit()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to get remote commit: {}", e)))?;
    let remote_oid = remote_commit.id();

    // Check if already up to date
    if current_oid == remote_oid {
        println!("  {} Already up to date", "✓".green());
        return Ok(());
    }

    // Count commits difference
    let (_ahead, behind) = repo
        .graph_ahead_behind(current_oid, remote_oid)
        .map_err(|e| TuxBoxError::GitError(format!("Failed to compare commits: {}", e)))?;

    if behind == 0 {
        println!(
            "  {} Local is ahead of remote (no update needed)",
            "✓".green()
        );
        return Ok(());
    }

    // Attempt fast-forward merge
    let fetch_commit = remote_commit;
    let refname = format!("refs/heads/{}", branch_name);
    let mut reference = repo.find_reference(&refname).map_err(|e| {
        TuxBoxError::GitError(format!("Failed to find reference '{}': {}", refname, e))
    })?;

    // Try fast-forward
    let ff_result = reference.set_target(fetch_commit.id(), "Fast-forward merge");

    match ff_result {
        Ok(_) => {
            // Update working directory
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .map_err(|e| TuxBoxError::GitError(format!("Failed to checkout HEAD: {}", e)))?;

            println!(
                "  {} Updated {} ({} {} behind)",
                "✓".green(),
                tool_name,
                behind,
                if behind == 1 { "commit" } else { "commits" }
            );
            // Invalidate tool state so the next `tbox run` re-installs dependencies
            // against the updated source.
            crate::tool_state::ToolState::invalidate(&tool_path);
        }
        Err(e) => {
            eprintln!(
                "{}",
                "  ✗ Update failed: merge conflict or non-fast-forward".red()
            );
            eprintln!("  {} Error: {}", "→".cyan(), e);
            eprintln!("  {} To force clean reinstall:", "→".cyan());
            eprintln!(
                "    {}",
                format!("rm -rf {}", tool_path.display()).bright_black()
            );
            eprintln!("    {}", format!("tbox run {}", tool_name).bright_black());
            return Err(
                TuxBoxError::GitError(format!("Update failed for '{}': {}", tool_name, e)).into(),
            );
        }
    }

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
