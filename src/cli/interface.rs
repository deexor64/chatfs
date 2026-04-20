use clap::{Parser};

use super::types::{Cli, Commands};
use crate::transport::socket::socket_loop;
use crate::config::loader::{save_config_cache, set_config, get_config};
use crate::config::types::ConfigKey;
use crate::utils::logger;


pub fn cli_handler() -> Result<(), String>{
    let cli = Cli::parse();

    // Ensure config exists
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
            // TODO: Update tool notice
            println!("Tool notice: use 'run' command");
            return Ok(())
        }
    }
}
