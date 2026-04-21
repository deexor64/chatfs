use std::{env, process::exit};
use clap::{Parser};

use crate::data::ensure_data_dir;
use super::types::{Cli, Commands};
use crate::config::manager::{get_config, load_config_cache, save_default_config, set_config};
use crate::config::types::ConfigKey;
use crate::transport::socket::set_gateway;
use crate::utils::logger;


pub fn cli_handler() -> Result<(), String>{
    // CLI instance
    let cli = Cli::parse();

    // Disable logging if requested
    // Subsequent log messages won't show up if logging is disabled
    if cli.no_logging {
        logger::toggle_logging(false);
    }

    logger::log_info("Logging is enabled (run with `--no-logging` to disable it)".to_string());

    // Check debug logging early
    // Subsequent debug log messages won't show up if logging or debug logging is kept disabled
    if cli.debug {
        logger::toggle_debug(true);
    }

    logger::log_info("Debug logging is enabled".to_string());

    // Detecting environment
    let mode = env::var("MODE").unwrap_or_else(|_| "".to_string());
    if mode == "dev" {
        logger::log_debug("Running in dev mode".to_string());
    }

    // Intializing data directory
    logger::log_debug("Initializing data directory...".to_string());
    _ = ensure_data_dir()?;

    // Load config and save it to cache
    let config = load_config_cache();

    match config {
        Ok(_) => {
            logger::log_debug("Config loaded from cache".to_string());
        }
        Err(e) => {
            logger::log_warn(e);
            logger::log_debug("Creating a new config...".to_string());
            save_default_config()?;
        }
    }

    // Process commands
    match cli.command {
        Some(Commands::SetConfig { key, value }) => {
            match set_config(key, value) {
                Ok(_) => {
                    logger::log_info("Config updated".to_string());
                    exit(0);
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        },
        Some(Commands::GetConfig { key }) => {
            match get_config(key) {
                Ok(value) => {
                    println!("{}", value);
                    exit(0);
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        },
        Some(Commands::Run { workspace, gateway }) => {
            // Use workspace directory if provided, otherwise use current directory
            match workspace {
                Some(workspace) => {
                    if !workspace.exists() {
                        return Err(format!("Workspace directory '{}' does not exist", workspace.display()));
                    }

                    if let Err(_) = std::env::set_current_dir(&workspace) {
                        return Err(format!("Failed to set workspace directory to '{}'", workspace.display()));
                    }

                    logger::log_info(format!("Using '{}' as workspace directory", workspace.display()));
                },
                None => {
                    logger::log_info("Using current directory as workspace directory".to_string());
                }
            }

            // Connect to gateway if provided, otherwise use config value
            match gateway {
                Some(gateway) => {
                    logger::log_info(format!("Using '{}' as gateway", gateway));
                    set_gateway(gateway);
                },
                None => {
                    let gateway = get_config(ConfigKey::Gateway)?;

                    logger::log_info(format!("Using '{}' as gateway", gateway));
                    set_gateway(gateway);
                }
            }

            return Ok(());
        },
        None => {
            // TODO: Update tool notice
            println!("Tool notice: use 'run' command");
            exit(0);
        }
    }
}
