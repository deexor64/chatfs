use std::{env, fs, path::PathBuf};
use directories::BaseDirs;

/*
 * Get the data directory for the application.
 * Directory location is determined by the environment mode.
 *   - `dev`: current directory
 *   - `prod` or else: user's home directory
 * New directory is created if it does not exist.
 * Always returns a canonical path.
 */
const DATA_DIR_NAME: &str = ".chatfs";

pub fn get_data_dir() -> Result<PathBuf, String> {
    // Determine the environment mode
    let mode = env::var("MODE").unwrap_or_else(|_| "".to_string());

    // Directory location
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

    Ok(data_dir)
}
