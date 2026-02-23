//! Command-line interface definitions using Clap derive API (2026 pattern)

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tbox")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize TuxBox with a tool registry
    Init {
        /// URL of the tool registry repository
        registry_url: String,
    },

    /// List available tools from the registry
    List,

    /// Run a tool (clones automatically if needed)
    Run {
        /// Name of the tool to run
        tool: String,

        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Update a tool (or all tools with --all)
    Update {
        /// Name of the tool to update (omit to update all)
        tool: Option<String>,
    },

    /// Show TuxBox status and installed tools
    Status,

    /// Manage registries (add, remove, list, sync)
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },

    /// Check for updates and optionally install them
    SelfUpdate {
        /// Automatically install the update without prompting
        #[arg(short, long)]
        install: bool,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand)]
pub enum RegistryAction {
    /// List all configured registries
    List,

    /// Add a new registry
    Add {
        /// Name for the registry
        name: String,

        /// Registry repository URL
        url: String,

        /// Priority (higher = checked first, default: 100)
        #[arg(short, long)]
        priority: Option<u32>,
    },

    /// Remove a registry (also deletes local cache unless --keep-cache)
    Remove {
        /// Name of the registry to remove
        name: String,

        /// Keep local cache directory (do not delete ~/.tuxbox/registry/<name>)
        #[arg(long)]
        keep_cache: bool,
    },

    /// Rename a registry (updates config and moves local cache)
    Rename {
        /// Current name of the registry
        old_name: String,

        /// New name for the registry
        new_name: String,
    },

    /// Change the priority of a registry (higher = checked first)
    SetPriority {
        /// Name of the registry
        name: String,

        /// New priority value
        priority: u32,
    },

    /// Sync (clone/update) all registries
    Sync,
}
