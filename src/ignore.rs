use std::path::PathBuf;


/*
 * Search for ignore file in the working directory
 * By default, use .chatignore if it exists, otherwise copy .gitignore to .chatignore
 * If neither exists, return None
 */
const IGNORE_FILE_NAME: &str = ".chatignore";

pub fn get_ignore_file() -> Result<PathBuf, String> {
    let path = PathBuf::from(IGNORE_FILE_NAME);

    // Return existing ignore file if it exists
    if !path.exists() {
        return Err("Ignore file not found".to_string());
    }

    return Ok(path);
}
