use std::{env, fs, path::PathBuf};
use std::sync::OnceLock;
use directories::BaseDirs;

/*
 * Returns the data directory for the application depending on the environment mode
 * - `dev`: current directory
 * - `prod` or else: user's home directory
 */
const DATA_DIR_NAME: &str = ".chatfs";
static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn ensure_data_dir() -> Result<PathBuf, String> {
    // Data dir already initialized, return cached value
    if let Some(data_dir) = DATA_DIR.get() {
        return Ok(data_dir.into());
    }

    // Subsequent code will only be executed once

    // Determining directory location based on environment mode
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

    // Canonicalize the user directory path
    let user_dir = user_dir
        .canonicalize()
        .map_err(|_| "Failed to canonicalize user directory".to_string())?;

    // Obtain data directory path
    let data_dir = user_dir.join(DATA_DIR_NAME);

    // Create data directory if it does not exist
    if !data_dir.exists() {
        fs::create_dir(&data_dir)
            .map_err(|_| "Failed to create data directory".to_string())?;
    }

    DATA_DIR.set(PathBuf::from(data_dir.clone())).ok();
    Ok(data_dir)
}


/*
 * Returns the path to the config file with the data directory
 */
const CONFIG_FILE_NAME: &str = "config.json";

pub fn get_config_file() -> Result<PathBuf, String> {
    let data_dir = ensure_data_dir()?;
    Ok(data_dir.join(CONFIG_FILE_NAME))
}
