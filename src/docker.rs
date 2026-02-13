//! Docker container management for tool isolation

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

use crate::config::ToolConfig;

/// Run a Python tool inside a Docker container
pub fn run_in_docker(tool_config: &ToolConfig, tool_path: &Path, args: &[String]) -> Result<()> {
    println!("  {} Using Docker for isolated execution", "🐳".cyan());

    // Determine Python version from tool or use default
    let python_version = "3.14"; // TODO: detect from tool config or pyproject.toml

    // Build container image name
    let image_name = format!("tuxbox-{}", tool_config.name);

    // Check if we need to build the image
    if !image_exists(&image_name)? {
        build_image(&image_name, tool_path, python_version)?;
    }

    // Run the tool in the container
    run_container(&image_name, tool_config, tool_path, args)?;

    Ok(())
}

/// Check if a Docker image exists locally
fn image_exists(image_name: &str) -> Result<bool> {
    let output = Command::new("docker")
        .args(["images", "-q", image_name])
        .output()
        .context("Failed to check Docker image")?;

    Ok(!output.stdout.is_empty())
}

/// Build Docker image for the tool
fn build_image(image_name: &str, tool_path: &Path, python_version: &str) -> Result<()> {
    println!("  {} Building Docker image...", "→".cyan());

    // Check if tool has a Dockerfile
    let dockerfile_path = tool_path.join("Dockerfile");

    if dockerfile_path.exists() {
        // Use tool's Dockerfile
        build_from_dockerfile(image_name, tool_path)?;
    } else {
        // Generate a standard Dockerfile for Python tools
        build_standard_python_image(image_name, tool_path, python_version)?;
    }

    println!("  {} Image built successfully", "✓".green());
    Ok(())
}

/// Build from existing Dockerfile
fn build_from_dockerfile(image_name: &str, tool_path: &Path) -> Result<()> {
    let status = Command::new("docker")
        .args(["build", "-t", image_name, "."])
        .current_dir(tool_path)
        .status()
        .context("Failed to build Docker image from Dockerfile")?;

    if !status.success() {
        anyhow::bail!("Docker build failed");
    }

    Ok(())
}

/// Build standard Python image with auto-install requirements
fn build_standard_python_image(
    image_name: &str,
    tool_path: &Path,
    python_version: &str,
) -> Result<()> {
    // Create temporary Dockerfile
    let dockerfile_content = format!(
        r#"FROM python:{}-slim

WORKDIR /app

# Copy tool files
COPY . /app

# Install dependencies if requirements.txt exists
RUN if [ -f requirements.txt ]; then \
        pip install --no-cache-dir -r requirements.txt; \
    fi

# Install the tool if it's a package
RUN if [ -f pyproject.toml ]; then \
        pip install --no-cache-dir -e .; \
    fi

# Set entrypoint
CMD ["python3"]
"#,
        python_version
    );

    // Write temporary Dockerfile
    let dockerfile_path = tool_path.join("Dockerfile.tuxbox");
    std::fs::write(&dockerfile_path, dockerfile_content).context("Failed to write Dockerfile")?;

    // Build image
    let status = Command::new("docker")
        .args(["build", "-f", "Dockerfile.tuxbox", "-t", image_name, "."])
        .current_dir(tool_path)
        .status()
        .context("Failed to build Docker image")?;

    // Clean up temporary Dockerfile
    let _ = std::fs::remove_file(dockerfile_path);

    if !status.success() {
        anyhow::bail!("Docker build failed");
    }

    Ok(())
}

/// Run the tool in a Docker container
fn run_container(
    image_name: &str,
    tool_config: &ToolConfig,
    _tool_path: &Path,
    args: &[String],
) -> Result<()> {
    println!("  {} Running in container...", "→".cyan());

    // Prepare command to run inside container
    // For Python modules: python3 -m <module_name>
    let container_cmd = format!("python3 -m {}", tool_config.name);

    // Build container name: <tool>_<version> (e.g., sshmenuc_1.1.0)
    let container_name = if let Some(version) = &tool_config.version {
        format!("{}_{}", tool_config.name, version)
    } else {
        tool_config.name.clone()
    };

    // Build docker run command
    let mut cmd = Command::new("docker");
    cmd.arg("run");
    cmd.arg("--rm"); // Remove container after exit
    cmd.arg("--name");
    cmd.arg(&container_name);

    // Check if stdout is a TTY - if yes, use -it for interactive tools
    use std::io::IsTerminal;
    if std::io::stdout().is_terminal() {
        cmd.arg("-it"); // Interactive + TTY (for TUI tools)
    } else {
        cmd.arg("-i"); // Interactive only (for pipes/redirects)
    }

    // Get user's home directory and UID/GID
    let home = std::env::var("HOME").unwrap_or_default();

    // Get current user's UID and GID
    #[cfg(unix)]
    let user_id = {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::metadata(&home).ok();
        metadata.map(|m| format!("{}:{}", m.uid(), m.gid()))
    };

    // Run container as current user (preserves permissions and home structure)
    #[cfg(unix)]
    if let Some(uid_gid) = user_id {
        cmd.arg("--user");
        cmd.arg(&uid_gid);
    }

    // Set HOME environment variable in container
    cmd.arg("-e");
    cmd.arg(format!("HOME={}", home));

    // Mount volumes for tool access to configs and data
    cmd.args([
        "-v",
        &format!("{}/.ssh:{}/.ssh:ro", home, home), // SSH configs (read-only)
        "-v",
        &format!("{}/.config:{}/.config", home, home), // App configs (read-write)
        "-v",
        &format!("{}:{}", home, home), // Home directory (preserves paths)
        "-w",
        "/app",
    ]);

    // Add the image
    cmd.arg(image_name);

    // Add the command to run
    cmd.args(container_cmd.split_whitespace());

    // Add user arguments
    cmd.args(args);

    // Execute with inherited stdio (for interactive TUI tools)
    use std::process::Stdio;
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status().context("Failed to run Docker container")?;

    if !status.success() {
        anyhow::bail!("Tool exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_exists_check() {
        // Just verify the function runs
        let _ = image_exists("nonexistent-image-xyz");
    }
}
