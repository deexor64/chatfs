use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::{fs, path::{Path, PathBuf}};

/*
 * Search for ignore file in the working directory
 * By default, use .chatignore if it exists, otherwise copy .gitignore to .chatignore
 * If neither exists, return None
 */
const IGNORE_FILE_NAME: &str = ".chatignore";

fn ensure_ignore_file() -> Option<PathBuf> {
    let path = PathBuf::from(IGNORE_FILE_NAME);

    // Return existing ignore file if it exists
    if path.exists() {
        return Some(path);
    }

    // Create a new ignore file if gitignore exists
    // Copy .gitignore to new ignore file
    if PathBuf::from(".gitignore").exists() {
        let gitignore_content = fs::read_to_string(&PathBuf::from(".gitignore")).ok();

        if let Some(content) = gitignore_content {
            fs::write(&path, content).ok();
        }
    }

    None
}

/*
 * Checks whether a given path should be ignored based on ignorefile
 * Create the builder using build_matcher and pass it to is_ignored
 */
pub fn is_ignored(path: &Path, matcher: &Gitignore) -> bool {
    let is_dir = path.is_dir();
    matcher.matched(path, is_dir).is_ignore()
}

// Helper to build a matcher
pub fn build_matcher(ignorefile: &PathBuf, work_dir: &PathBuf) -> Gitignore {
    let mut builder = GitignoreBuilder::new(work_dir);
    builder.add(ignorefile);
    builder.build().expect("Failed to build ignore matcher")
}
