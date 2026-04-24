use std::path::PathBuf;


/*
 * Search for ignore file in the working directory
 * By default, use .chatignore if it exists, otherwise copy .gitignore to .chatignore
 * If neither exists, return None
 */
const IGNORE_FILE_NAME: &str = ".chatignore";

pub fn ensure_ignore_file() -> Option<PathBuf> {
    let path = PathBuf::from(IGNORE_FILE_NAME);

    // Return existing ignore file if it exists
    if path.exists() {
        return Some(path);
    }

    None
}
