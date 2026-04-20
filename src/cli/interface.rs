use clap::{Parser};

use super::types::{Cli, Commands};
use crate::transport::socket::socket_loop;
use crate::config::loader::{save_config_cache, set_config, get_config};
use crate::config::types::ConfigKey;
use crate::utils::logger;


pub fn cli_handler() -> Result<(), String>{
    // CLI instance
    let cli = Cli::parse();

    // Load config and save it to cache
    let config = save_config_cache();

    if let Err(e) = config {
        logger::log_warn(e);
        logger::log_info("Creating a new config...".to_string());

        crate::config::loader::save_default_config()?;
    }

    match cli.command {
        Some(Commands::SetConfig { key, value }) => {
            return set_config(key, value);
        }
        Some(Commands::GetConfig { key }) => {
            let value = get_config(key)?;
            println!("{}", value);

            return Ok(());
        }
        Some(Commands::Run { path, gateway, no_logging, debug }) => {
            // Disable logging if requested
            if no_logging {
                logger::toggle_logging(false);
            }

            // Enable debug logging if requested
            if debug {
                logger::toggle_debug(true);
            }

            // This and subsequent log messages won't show up if logging is disabled
            logger::log_info("Logging is enabled (run with `--no-logging` to disable it)".to_string());

            // Set workspace directory if provided
            if let Some(path) = path {
                if let Err(_) = std::env::set_current_dir(&path) {
                    return Err(format!("Failed to set workspace directory to '{}'", path.display()));
                }
            }

            // Connect to gateway if provided, otherwise use config value
            match gateway {
                Some(gateway) => {
                    socket_loop(gateway)?;
                },
                None => {
                    let gateway = get_config(ConfigKey::Gateway)?;
                    socket_loop(gateway)?;
                }
            }

            return Ok(());
        },
        None => {
            // TODO: Update tool notice
            println!("Tool notice: use 'run' command");
            return Ok(())
        }
    }
}
