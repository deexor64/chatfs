use std::{env, fs::{self, File}, io::BufWriter, path::PathBuf, sync::RwLock};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

pub enum ConfigKey {
    Gateway
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    gateway: String,
}

// In mememory cache for the config
static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

// Get data directory
fn get_data_dir() -> Result<PathBuf, String> {
    let mode = env::var("MODE").unwrap_or_else(|_| "".to_string());

    let user_dir: PathBuf = if mode == "dev" {
        env::current_dir()
            .map_err(|_| "Failed to determine user directory".to_string())?
    } else {
        BaseDirs::new()
            .ok_or("Failed to determine user directory".to_string())?
            .home_dir()
            .to_path_buf()
    };

    let user_dir = user_dir
        .canonicalize()
        .map_err(|_| "Failed to canonicalize user directory".to_string())?;

    let data_dir = user_dir.join(".chatfs");

    if !data_dir.exists() {
        fs::create_dir(&data_dir)
            .map_err(|_| "Failed to create data directory".to_string())?;
    }

    println!("Datadir: {}", data_dir.display());

    Ok(data_dir)
}


/*
 * This function loads the user settings from the config file, or creates a new one if it doesn't exist
 * Must be called once at startup
 * Config is cached after loading
 * All config reads will be returned from cache
 * All writes will be written to the config file and the cache will be updated
 */
pub fn load_config() -> Result<(), String> {
    // Get data dir
    let data_dir = get_data_dir()?;

    // Read config
    let config_file = data_dir.join("config.json");

    if config_file.exists() {
        // Attempt to read an existing config file
        let content = std::fs::read_to_string(&config_file)
            .map_err(|_| "Failed to read config file".to_string())?;

        // Parse config
        let config = serde_json::from_str::<Config>(&content)
            .map_err(|_| "Failed to read config file".to_string())?;

        // Save to config cache
        let mut conf_w = CONFIG.write().map_err(|_| "Failed to save config cache".to_string())?;
        *conf_w = Some(config);

    } else {
        // Attempt to create new config file
        let file = File::create(config_file).map_err(|_| "Failed to create config file".to_string())?;
        let writer = BufWriter::new(file);

        // Write default config
        let config = Config {
            gateway: "".to_string()
        };

        serde_json::to_writer_pretty(writer, &config)
            .map_err(|_| "Failed to create config file".to_string())?;

        // Save to config cache
        let mut conf_w = CONFIG.write().map_err(|_| "Failed to save config cache".to_string())?;
        *conf_w = Some(config);
    }

    Ok(())
}

pub fn get_config(key: ConfigKey) -> Result<String, String> {
    let mode = env::var("MODE").unwrap_or_else(|_| "".to_string());

    let guard = CONFIG.read().map_err(|_| "Failed to read config".to_string())?;
    let config = guard.as_ref().ok_or("Failed to read config".to_string())?;

    match key {
        ConfigKey::Gateway => {
            if mode == "dev"{
                Ok("ws://127.0.0.1:8000/client/".to_string())
            } else if config.gateway.is_empty() {
                Err("Gateway config not set".to_string())
            } else {
                Ok(config.gateway.clone())
            }
        },
    }
}

pub fn set_config(key: ConfigKey, value: &str) -> Result<(), String> {
    // Write to cache
    let mut guard = CONFIG.write().map_err(|_| "Failed to write config".to_string())?;
    let config = guard.as_mut().ok_or("Failed to write config".to_string())?;

    match key {
        ConfigKey::Gateway => config.gateway = value.to_string(),
    }

    // Write to config file
    let data_dir = get_data_dir()?;
    let config_file = data_dir.join("config.json");

    let file = File::create(config_file).map_err(|_| "Failed to create config file".to_string())?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
        .map_err(|_| "Failed to create config file".to_string())?;

    Ok(())
}
