use std::{fs::File, io::BufWriter, path::PathBuf, sync::RwLock};

use crate::path_validation::data_dir::{get_data_dir};
use crate::tool_config::config_types::{ConfigKey, Config};


// In memory cache for the config
static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

/*
 * Ensures a valid config file exists and returns the config
 * Always results in an error if the config file does not exist or is invalid
 */
pub fn ensure_config() -> Result<Config, String> {
    let config_file = get_config_file()?;

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
 * Creates a new config file with default values
 * Any existing config file will be overwritten
 * This must only be called when a valid config file does not already exist
 */
pub fn create_config() -> Result<Config, String> {
    let config_file = get_config_file()?;

    // Attempt to create new config file
    let file = File::create(config_file).map_err(|_| "Failed to create config".to_string())?;

    // Write default config
    let writer = BufWriter::new(file);

    let config = Config {
        gateway: "".to_string()
    };

    serde_json::to_writer_pretty(writer, &config)
        .map_err(|_| "Failed to create config".to_string())?;

    Ok(config)
}

// Update config cache from the given config
pub fn save_config_cache(config: Config) -> Result<(), String> {
    let mut conf_w = CONFIG.write().map_err(|_| "Failed to save config cache".to_string())?;
    *conf_w = Some(config);
    Ok(())
}

// Retrieve the value for the given config key from the cache
pub fn get_config(key: ConfigKey) -> Result<String, String> {
    let guard = CONFIG.read().map_err(|_| "Failed to read config".to_string())?;
    let config = guard.as_ref().ok_or("Failed to read config".to_string())?;

    // Return value for the key
    match key {
        ConfigKey::Gateway => Ok(config.gateway.clone())
    }
}

// Set the value for the given config key to the cache and update config file
pub fn set_config(key: ConfigKey, value: String) -> Result<(), String> {
    // Write to cache
    let mut guard = CONFIG.write().map_err(|_| "Failed to write config".to_string())?;
    let config = guard.as_mut().ok_or("Failed to write config".to_string())?;

    match key {
        ConfigKey::Gateway => {
            config.gateway = value.to_string();
        },
    }

    // Write to config file
    let config_file = get_config_file()?;

    let file = File::create(config_file).map_err(|_| "Failed to create config file".to_string())?; // Fresh file
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
        .map_err(|_| "Failed to create config file".to_string())?;

    Ok(())
}

// Helper to get the path to the config file
fn get_config_file() -> Result<PathBuf, String> {
    let data_dir = get_data_dir()?;
    Ok(data_dir.join("config.json"))
}
