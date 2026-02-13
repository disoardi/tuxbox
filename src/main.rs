//! TuxBox - A meta-tool to manage and run personal tools from Git repositories
//!
//! # Architecture
//!
//! TuxBox is organized into modules:
//! - `cli`: Command-line interface definitions (Clap)
//! - `config`: Configuration management (TOML parsing, context struct)
//! - `git`: Git operations (clone, pull, status)
//! - `runner`: Tool execution logic
//! - `error`: Custom error types
//! - `environment`: Environment detection (Docker availability)
//! - `docker`: Docker container management
//! - `python`: Python venv management (fallback)
//! - `registry`: Registry management and tool resolution

mod cli;
mod config;
mod docker;
mod environment;
mod error;
mod git;
mod python;
mod registry;
mod runner;
mod selfupdate;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;

fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize colored output (respect NO_COLOR environment variable)
    colored::control::set_override(std::env::var("NO_COLOR").is_err());

    // Execute command
    match cli.command {
        cli::Commands::Init { registry_url } => {
            println!("{} Initializing TuxBox...", "→".cyan());
            config::init_config(&registry_url)?;
            println!("{} TuxBox initialized successfully!", "✓".green());
        }
        cli::Commands::List => {
            println!("{} Available tools:", "→".cyan());
            config::list_tools()?;
        }
        cli::Commands::Run { tool, args } => {
            println!("{} Running tool: {}", "→".cyan(), tool.bold());
            runner::run_tool(&tool, &args)?;
        }
        cli::Commands::Update { tool } => {
            if let Some(tool_name) = tool {
                println!("{} Updating tool: {}", "→".cyan(), tool_name.bold());
                git::update_tool(&tool_name)?;
            } else {
                println!("{} Updating all tools...", "→".cyan());
                git::update_all_tools()?;
            }
        }
        cli::Commands::Status => {
            config::show_status()?;
        }
        cli::Commands::Registry { action } => match action {
            cli::RegistryAction::List => {
                config::list_registries()?;
            }
            cli::RegistryAction::Add {
                name,
                url,
                priority,
            } => {
                println!("{} Adding registry '{}'...", "→".cyan(), name.bold());
                config::add_registry(&name, &url, priority)?;
            }
            cli::RegistryAction::Remove { name } => {
                println!("{} Removing registry '{}'...", "→".cyan(), name.bold());
                config::remove_registry(&name)?;
            }
            cli::RegistryAction::Sync => {
                println!("{} Syncing all registries...", "→".cyan());
                registry::sync_all_registries()?;
            }
        },
        cli::Commands::SelfUpdate { install } => {
            selfupdate::check_for_update(install)?;
        }
        cli::Commands::Version => {
            selfupdate::show_version()?;
        }
    }

    Ok(())
}
