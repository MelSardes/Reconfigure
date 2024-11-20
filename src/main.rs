use clap::{Parser, Subcommand};
use log::{error, info};
use std::path::PathBuf;

mod commands;
mod config;
mod system;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration from the current system
    Init {
        /// Include all installed packages, including dependencies
        #[arg(long)]
        hard: bool,
    },
    /// Apply configuration from a file
    Apply {
        /// Path to the configuration file
        #[arg(value_name = "FILE")]
        config_file: PathBuf,
        /// Only apply specific section (system, packages, themes)
        #[arg(short, long)]
        section: Option<String>,
    },
    /// Add a package to the configuration
    AddPackage {
        /// Package manager to use (pacman, yay)
        #[arg(value_name = "MANAGER")]
        package_manager: String,
        /// Name of the package to add
        #[arg(value_name = "PACKAGE")]
        package_name: String,
        /// Category to add the package to (system, development, graphics)
        #[arg(value_name = "CATEGORY")]
        category: Option<String>,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { hard } => {
            info!("Initializing new configuration (hard = {})", hard);
            commands::init::run(hard)
        }
        Commands::Apply {
            config_file,
            section,
        } => {
            info!(
                "Applying configuration from {:?} (section = {:?})",
                config_file, section
            );
            commands::apply::run(&config_file, section.as_deref())
        }
        Commands::AddPackage {
            package_manager,
            package_name,
            category,
        } => {
            info!(
                "Adding package {} using {} (category = {:?})",
                package_name, package_manager, category
            );
            commands::add_package::run(&package_manager, &package_name, category.as_deref())
        }
    };

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}
