use clap::{Parser};

use crate::cli_handler::cli_types::{Cli, Commands};
use crate::tool_config::config::{create_config, ensure_config, get_config, save_config_cache, set_config};
use crate::tool_config::config_types::ConfigKey;
use crate::logger;


pub fn cli_handler() -> Result<(), String>{
    let cli = Cli::parse();
    logger::enable_logging();

    // Ensure config exists
    let config = ensure_config();

    match config {
        Ok(config) => {
            save_config_cache(config)?;
        },
        Err(e) => {
            logger::log_warn(e);
            logger::log_debug("Creating a fresh config...".to_string());

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

    // Disable logging until enabled by the cli argument
    // Any subsequent log messages will be suppressed
    logger::disable_logging();

    match cli.command {
        Some(Commands::SetConfig { key, value }) => {
            return set_config(key, value);
        }
        Some(Commands::GetConfig { key }) => {
            let value = get_config(key)?;
            println!("{}", value);

            return Ok(());
        }
        Some(Commands::Run { logging, gateway }) => {
            if logging {
                logger::enable_logging();
            }

            if let Some(gateway) = gateway {
                println!("Setting gateway '{}' ...", gateway);
            } else {
                let gateway = get_config(ConfigKey::Gateway)?;
                println!("Setting gateway '{}' ...", gateway);
            }

            return Ok(());
        },
        None => { return Ok(()) }
    }
}
