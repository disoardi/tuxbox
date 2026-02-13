//! Environment detection for choosing execution strategy

use std::process::Command;

/// Check if Docker is available and running
pub fn is_docker_available() -> bool {
    // Try to run `docker version` to check if Docker daemon is available
    let result = Command::new("docker").arg("version").output();

    match result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Detect the best execution environment
pub enum ExecutionEnvironment {
    /// Docker is available - use containerized execution (PREFERRED)
    Docker,
    /// Docker not available - use local venv (FALLBACK)
    LocalVenv,
}

/// Determine the best execution environment
pub fn detect_environment() -> ExecutionEnvironment {
    // Allow forcing venv for testing purposes
    if std::env::var("TUXBOX_FORCE_VENV").is_ok() {
        return ExecutionEnvironment::LocalVenv;
    }

    if is_docker_available() {
        ExecutionEnvironment::Docker
    } else {
        ExecutionEnvironment::LocalVenv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_detection() {
        // Just verify the function runs without panic
        let _env = detect_environment();
    }
}
