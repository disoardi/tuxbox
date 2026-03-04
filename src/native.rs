//! Native binary tool support
//!
//! Downloads pre-built binaries from GitHub releases and executes them directly.
//! No Docker, no Python, no compilation required.
//!
//! Asset naming convention expected: `{tool_name}-{os}-{arch}`
//!   e.g. `hfs-linux-x86_64`, `hfs-macos-arm64`

use anyhow::{Context, Result};
use colored::Colorize;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{ToolConfig, tools_dir};
use crate::error::TuxBoxError;

const VERSION_FILE: &str = ".tuxbox-native-version";

/// Run a native binary tool (download from GitHub releases if not present, then execute)
pub fn run_native_tool(tool_config: &ToolConfig, args: &[String]) -> Result<()> {
    let tool_dir = tools_dir()?.join(&tool_config.name);
    let binary_path = tool_dir.join(&tool_config.name);

    if !binary_path.exists() {
        println!(
            "  {} Downloading native binary for {}...",
            "→".cyan(),
            tool_config.name.bold()
        );
        download_binary(tool_config, &tool_dir, &binary_path)?;
    }

    run_binary(&binary_path, args)
}

/// Re-download binary if a newer release is available on GitHub
pub fn update_native_tool(tool_name: &str) -> Result<()> {
    let tool_dir = tools_dir()?.join(tool_name);
    let binary_path = tool_dir.join(tool_name);
    let version_file = tool_dir.join(VERSION_FILE);

    let installed_tag = fs::read_to_string(&version_file)
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    // We need the repo URL — read it from the version file's sibling or re-fetch via registry.
    // Simplest: always fetch latest release and compare tags.
    let repo_url = read_repo_url(&tool_dir)?;
    let (owner, repo) = parse_github_url(&repo_url)?;
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let release = fetch_release(&api_url)?;

    if release.tag_name == installed_tag {
        println!(
            "  {} {} — already up to date ({})",
            "✓".green(),
            tool_name.bold(),
            installed_tag.dimmed()
        );
        return Ok(());
    }

    println!(
        "  {} {} — updating {} → {}",
        "→".cyan(),
        tool_name.bold(),
        installed_tag.dimmed(),
        release.tag_name.green()
    );

    // Re-create a temporary ToolConfig with just what we need
    let tool_config = ToolConfig {
        name: tool_name.to_string(),
        repo: repo_url,
        branch: None,
        version: None,
        tool_type: Some("native".to_string()),
        isolation: None,
        commands: None,
        python_version: None,
    };

    download_binary(&tool_config, &tool_dir, &binary_path)?;
    Ok(())
}

/// Returns true if the tool directory looks like a native (non-git) tool
pub fn is_native_tool_dir(tool_dir: &Path) -> bool {
    tool_dir.join(VERSION_FILE).exists()
}

// ─── Internal ───────────────────────────────────────────────────────────────

fn download_binary(tool_config: &ToolConfig, tool_dir: &Path, binary_path: &Path) -> Result<()> {
    let (owner, repo) = parse_github_url(&tool_config.repo)?;
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    println!("  {} Fetching release info...", "→".cyan());
    let release = fetch_release(&api_url)?;

    let asset_name = get_platform_asset_name(&tool_config.name)?;
    println!("  {} Platform: {}", "→".cyan(), asset_name.dimmed());

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| {
            let available: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
            TuxBoxError::ExecutionError(format!(
                "No binary found for platform: {}\nAvailable assets: {}",
                asset_name,
                available.join(", ")
            ))
        })?;

    println!(
        "  {} Downloading: {} ({})",
        "→".cyan(),
        asset.name.bold(),
        release.tag_name.dimmed()
    );

    fs::create_dir_all(tool_dir).context("Failed to create tool directory")?;

    let data = download_asset(&asset.browser_download_url)?;
    fs::write(binary_path, &data).context("Failed to write binary")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(binary_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(binary_path, perms)?;
    }

    // Persist the repo URL and installed tag for future updates
    fs::write(tool_dir.join(VERSION_FILE), &release.tag_name)?;
    fs::write(tool_dir.join(".tuxbox-native-repo"), &tool_config.repo)?;

    println!(
        "  {} {} installed ({})",
        "✓".green(),
        asset_name.bold(),
        release.tag_name.green()
    );
    Ok(())
}

fn run_binary(binary_path: &Path, args: &[String]) -> Result<()> {
    let status = std::process::Command::new(binary_path)
        .args(args)
        .status()
        .map_err(|e| TuxBoxError::ExecutionError(format!("Failed to execute binary: {}", e)))?;

    if !status.success() {
        return Err(TuxBoxError::ExecutionError(format!(
            "Binary exited with code: {}",
            status.code().unwrap_or(-1)
        ))
        .into());
    }

    Ok(())
}

fn read_repo_url(tool_dir: &Path) -> Result<String> {
    let repo_file = tool_dir.join(".tuxbox-native-repo");
    fs::read_to_string(&repo_file)
        .map(|s| s.trim().to_string())
        .with_context(|| format!("Cannot read repo URL from {}", repo_file.display()))
}

fn parse_github_url(url: &str) -> Result<(String, String)> {
    let url = url.trim_end_matches(".git");
    let path = url
        .trim_start_matches("https://github.com/")
        .trim_start_matches("git@github.com:");
    let parts: Vec<&str> = path.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        anyhow::bail!("Cannot parse GitHub URL: {}", url);
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn get_platform_asset_name(tool_name: &str) -> Result<String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let suffix = match (os, arch) {
        ("linux", "x86_64") => "linux-x86_64",
        ("macos", "x86_64") => "macos-x86_64",
        ("macos", "aarch64") => "macos-arm64",
        _ => anyhow::bail!("Unsupported platform: {} {}", os, arch),
    };

    Ok(format!("{}-{}", tool_name, suffix))
}

fn fetch_release(api_url: &str) -> Result<GithubRelease> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("tuxbox")
        .build()?;

    let response = client
        .get(api_url)
        .send()
        .context("Failed to fetch release from GitHub")?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API returned error: {}", response.status());
    }

    response
        .json()
        .context("Failed to parse GitHub API response")
}

fn download_asset(url: &str) -> Result<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("tuxbox")
        .build()?;

    let response = client
        .get(url)
        .send()
        .context("Failed to download binary")?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed: {}", response.status());
    }

    let bytes = response.bytes()?.to_vec();
    println!("  {} Downloaded {} bytes", "✓".green(), bytes.len());
    Ok(bytes)
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

/// Return the tool directory path if the native tool is installed, or None
pub fn native_tool_dir(tool_name: &str) -> Result<Option<PathBuf>> {
    let dir = tools_dir()?.join(tool_name);
    if dir.join(VERSION_FILE).exists() {
        Ok(Some(dir))
    } else {
        Ok(None)
    }
}
