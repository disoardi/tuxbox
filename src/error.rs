//! Custom error types using thiserror (2026 pattern)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TuxBoxError {
    #[error("Tool '{0}' not found in registry")]
    ToolNotFound(String),

    #[error("TuxBox not initialized. Run 'tbox init <registry-url>' first")]
    NotInitialized,

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Tool execution failed: {0}")]
    ExecutionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
