//! Self-update functionality for TuxBox
//!
//! Checks GitHub releases for newer versions and updates the binary.

use anyhow::{Context, Result};
use colored::Colorize;
use serde::Deserialize;
use std::env;
use std::fs;

use crate::error::TuxBoxError;

const GITHUB_API_URL: &str = "https://api.github.com/repos/disoardi/tuxbox/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub API Release response
#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

/// Check for updates and optionally install them
pub fn check_for_update(auto_install: bool) -> Result<()> {
    println!("{}", "Checking for updates...".cyan());

    // Fetch latest release from GitHub
    let release = fetch_latest_release()?;
    let latest_version = parse_version(&release.tag_name)?;
    let current_version = parse_version(&format!("v{}", CURRENT_VERSION))?;

    println!(
        "  {} Current version: {}",
        "→".cyan(),
        CURRENT_VERSION.bold()
    );
    println!(
        "  {} Latest version:  {}",
        "→".cyan(),
        latest_version.bold()
    );

    // Compare versions
    let current = semver::Version::parse(&current_version)?;
    let latest = semver::Version::parse(&latest_version)?;

    if latest <= current {
        println!("{}", "✓ You are running the latest version!".green());
        return Ok(());
    }

    println!(
        "{}",
        format!("🎉 New version available: {}", release.name).yellow()
    );

    if !auto_install {
        println!("\nTo update, run:");
        println!("  {}", "tbox self-update --install".cyan());
        return Ok(());
    }

    // Install update
    install_update(&release)?;

    Ok(())
}

/// Fetch latest release from GitHub API
fn fetch_latest_release() -> Result<GithubRelease> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("tuxbox")
        .build()?;

    let response = client
        .get(GITHUB_API_URL)
        .send()
        .context("Failed to fetch latest release from GitHub")?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API returned error: {}", response.status());
    }

    let release: GithubRelease = response
        .json()
        .context("Failed to parse GitHub API response")?;

    Ok(release)
}

/// Parse version string (removes 'v' prefix if present)
fn parse_version(version: &str) -> Result<String> {
    Ok(version.strip_prefix('v').unwrap_or(version).to_string())
}

/// Install update by downloading and replacing the binary
fn install_update(release: &GithubRelease) -> Result<()> {
    println!("{}", "\nInstalling update...".cyan());

    // Detect current platform
    let asset_name = get_platform_asset_name()?;
    println!(
        "  {} Detecting platform: {}",
        "→".cyan(),
        asset_name.dimmed()
    );

    // Find matching asset
    let asset = release
        .assets
        .iter()
        .find(|a| a.name.starts_with(&asset_name))
        .ok_or_else(|| {
            TuxBoxError::UpdateError(format!("No binary found for platform: {}", asset_name))
        })?;

    println!("  {} Downloading: {}", "→".cyan(), asset.name.bold());

    // Download binary
    let binary_data = download_asset(&asset.browser_download_url)?;

    // Extract from tarball and replace current binary
    replace_current_binary(&binary_data)?;

    println!("{}", "✓ Update installed successfully!".green());
    println!(
        "\n{}",
        "Please restart tbox to use the new version.".yellow()
    );

    Ok(())
}

/// Download asset from GitHub
fn download_asset(url: &str) -> Result<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("tuxbox")
        .build()?;

    let response = client
        .get(url)
        .send()
        .context("Failed to download update")?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed: {}", response.status());
    }

    let bytes = response.bytes()?.to_vec();
    println!("  {} Downloaded {} bytes", "✓".green(), bytes.len());

    Ok(bytes)
}

/// Replace current binary with the new one
fn replace_current_binary(tarball_data: &[u8]) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    println!("  {} Extracting binary...", "→".cyan());

    // Get current binary path
    let current_exe = env::current_exe().context("Failed to get current executable path")?;

    // Extract tarball
    let decoder = GzDecoder::new(tarball_data);
    let mut archive = Archive::new(decoder);

    // Find tbox binary in archive
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if path.file_name() == Some(std::ffi::OsStr::new("tbox")) {
            println!("  {} Replacing binary...", "→".cyan());

            // Create backup
            let backup_path = current_exe.with_extension("bak");
            fs::copy(&current_exe, &backup_path).context("Failed to create backup")?;

            // Extract to temp location first
            let temp_path = current_exe.with_extension("tmp");
            let mut temp_file = fs::File::create(&temp_path)?;
            std::io::copy(&mut entry, &mut temp_file)?;

            // Make executable (Unix)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&temp_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&temp_path, perms)?;
            }

            // Replace current binary
            fs::rename(&temp_path, &current_exe).context("Failed to replace binary")?;

            println!("  {} Binary replaced successfully", "✓".green());
            println!(
                "  {} Backup saved to: {}",
                "→".dimmed(),
                backup_path.display().to_string().dimmed()
            );

            return Ok(());
        }
    }

    Err(TuxBoxError::UpdateError("Binary not found in archive".into()).into())
}

/// Get platform-specific asset name
fn get_platform_asset_name() -> Result<String> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let platform = match (os, arch) {
        ("linux", "x86_64") => "tbox-linux-amd64",
        ("macos", "x86_64") => "tbox-macos-amd64",
        ("macos", "aarch64") => "tbox-macos-arm64",
        _ => {
            return Err(
                TuxBoxError::UpdateError(format!("Unsupported platform: {} {}", os, arch)).into(),
            );
        }
    };

    Ok(platform.to_string())
}

/// Show current version
pub fn show_version() -> Result<()> {
    println!("TuxBox version: {}", CURRENT_VERSION.bold());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("v1.2.3").unwrap(), "1.2.3");
        assert_eq!(parse_version("1.2.3").unwrap(), "1.2.3");
    }

    #[test]
    fn test_platform_detection() {
        let platform = get_platform_asset_name().unwrap();
        assert!(platform.starts_with("tbox-"));
    }
}
