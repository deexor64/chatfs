use std::{fs::File, io::BufWriter, sync::OnceLock};

use super::config_path::get_config_path;
use super::types::{ConfigKey, Config};


// In memory cache for the config
static CONFIG: OnceLock<Config> = OnceLock::new();

/*
 * Ensures a valid config file exists and returns the full config
 */
fn read_config() -> Result<Config, String> {
    let config_file = get_config_path()?;

    if !config_file.exists() {
        return Err("Config not found".to_string())
    }

    // Attempt to read config file
    let content = std::fs::read_to_string(&config_file)
        .map_err(|_| "Failed to read config".to_string())?;

    let config: Config = serde_json::from_str::<Config>(&content)
        .map_err(|_| "Corrupted or invalid config".to_string())?;

    Ok(config)
}

/*
 * Creates a new config file with a given config
 * Any existing config file will be overwritten
 * Therefore full config must be provided
 */
fn save_config(config: Config) -> Result<(), String> {
    let config_file = get_config_path()?;

    // Attempt to create a new config file
    let file = File::create(config_file).map_err(|_| "Failed to create config".to_string())?;

    // Write config
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
        .map_err(|_| "Failed to create config".to_string())?;

    Ok(())
}

// Save default config to the config file
pub fn save_default_config() -> Result<(), String> {
    let config = Config {
        gateway: "".to_string()
    };

    save_config(config)
}

// Update config cache from the config file
pub fn save_config_cache() -> Result<(), String> {
    let config = read_config()?;

    CONFIG.set(config.clone()).map_err(|_| "Failed to save config cache".to_string())?;
    Ok(())
}



// Retrieve the value for the given config key from the cache
pub fn get_config(key: ConfigKey) -> Result<String, String> {
    let config = CONFIG.get().ok_or("Config not initialized".to_string())?;

    match key {
        ConfigKey::Gateway => Ok(config.gateway.clone())
    }
}


// Set the value for the given config key to the config file
// Config cache is not updated here, becuase run time mutation is not allowed
// This must only be invoked from a CLI command
pub fn set_config(key: ConfigKey, value: String) -> Result<(), String> {
    let mut config = read_config()?;

    match key {
        ConfigKey::Gateway => config.gateway = value,
    }

    save_config(config)?;

    Ok(())
}
