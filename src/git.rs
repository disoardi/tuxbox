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

    // Detect remote URL to choose fetch strategy (before moving repo)
    let remote_url = repo
        .find_remote("origin")
        .ok()
        .and_then(|r| r.url().map(|u| u.to_string()))
        .unwrap_or_default();

    let is_ssh = remote_url.starts_with("git@") || remote_url.starts_with("ssh://");

    // Drop statuses (borrow of repo) before consuming repo
    drop(statuses);

    if is_ssh {
        update_tool_system_git(tool_name, &tool_path)
    } else {
        update_tool_git2(tool_name, &tool_path, repo)
    }
}

/// Update a tool via system `git pull --ff-only` (used for SSH remotes)
fn update_tool_system_git(tool_name: &str, tool_path: &std::path::Path) -> Result<()> {
    use colored::Colorize;
    use std::process::Command;

    // Record HEAD before pull to detect whether it advanced
    let before_oid = git2::Repository::open(tool_path).ok().and_then(|r| {
        r.head()
            .ok()
            .and_then(|h| h.peel_to_commit().ok())
            .map(|c| c.id())
    });

    let status = Command::new("git")
        .args(["pull", "--ff-only"])
        .current_dir(tool_path)
        .status()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to execute git pull: {}", e)))?;

    if !status.success() {
        eprintln!(
            "{}",
            "  ✗ Update failed (non-fast-forward or conflict)".red()
        );
        eprintln!("  {} To force clean reinstall:", "→".cyan());
        eprintln!(
            "    {}",
            format!("rm -rf {}", tool_path.display()).bright_black()
        );
        eprintln!("    {}", format!("tbox run {}", tool_name).bright_black());
        return Err(TuxBoxError::GitError(format!("git pull failed for '{}'", tool_name)).into());
    }

    // Check if HEAD advanced
    let after_oid = git2::Repository::open(tool_path).ok().and_then(|r| {
        r.head()
            .ok()
            .and_then(|h| h.peel_to_commit().ok())
            .map(|c| c.id())
    });

    if before_oid != after_oid {
        println!("  {} Updated {}", "✓".green(), tool_name);
        crate::tool_state::ToolState::invalidate(tool_path);
    } else {
        println!("  {} Already up to date", "✓".green());
    }

    Ok(())
}

/// Update a tool via git2 fetch + fast-forward (used for HTTPS remotes)
fn update_tool_git2(
    tool_name: &str,
    tool_path: &std::path::Path,
    repo: git2::Repository,
) -> Result<()> {
    use colored::Colorize;

    let head = repo
        .head()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to get HEAD reference: {}", e)))?;
    let current_oid = head
        .peel_to_commit()
        .map_err(|e| TuxBoxError::GitError(format!("Failed to get current commit: {}", e)))?
        .id();

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

    if current_oid == remote_oid {
        println!("  {} Already up to date", "✓".green());
        return Ok(());
    }

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

    let refname = format!("refs/heads/{}", branch_name);
    let mut reference = repo.find_reference(&refname).map_err(|e| {
        TuxBoxError::GitError(format!("Failed to find reference '{}': {}", refname, e))
    })?;

    match reference.set_target(remote_commit.id(), "Fast-forward merge") {
        Ok(_) => {
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .map_err(|e| TuxBoxError::GitError(format!("Failed to checkout HEAD: {}", e)))?;
            println!(
                "  {} Updated {} ({} {} behind)",
                "✓".green(),
                tool_name,
                behind,
                if behind == 1 { "commit" } else { "commits" }
            );
            crate::tool_state::ToolState::invalidate(tool_path);
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
