use clap::{Parser};

use crate::cli_handler::cli_types::{Cli, Commands};
use crate::message_handler::socket_loop::socket_loop;
use crate::tool_config::config::{create_config, ensure_config, get_config, save_config_cache, set_config};
use crate::tool_config::config_types::ConfigKey;
use crate::logger;


pub fn cli_handler() -> Result<(), String>{
    let cli = Cli::parse();

    // Ensure config exists
    let config = ensure_config();

    match config {
        Ok(config) => {
            save_config_cache(config)?;
        },
        Err(e) => {
            logger::log_warn(e);
            logger::log_info("Creating a new config...".to_string());

            let config = create_config();

            match config {
                Ok(config) => {
                    save_config_cache(config)?;
                },
                Err(e) => {
                    logger::log_error(e.clone());
                    return Err(e);
                }
            }
        }
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
        Some(Commands::Run { no_logging, gateway }) => {
            if no_logging {
                logger::disable_logging();
            }

            logger::log_info("Logging is enabled (use '--no-logging' to disable)".to_string());

            if let Some(gateway) = gateway {
                socket_loop(gateway)?;
            } else {
                let gateway = get_config(ConfigKey::Gateway)?;
                socket_loop(gateway)?;
            }

            return Ok(());
        },
        None => {
            // TODO: Add tool notice
            println!("Tool notice: use 'run' command");
            return Ok(())
        }
    }
}
