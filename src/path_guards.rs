use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};
use std::env;

/*
 * Checks whether a given path should be ignored
 * Create the builder using build_matcher and pass it to is_ignored
 */
pub fn build_matcher(ignorefile: &str) -> Gitignore {
    let mut builder = GitignoreBuilder::new(".");
    builder.add(ignorefile);
    builder.build().expect("Failed to build ignore matcher")
}

pub fn is_ignored(path: &Path, matcher: &Gitignore) -> bool {
    let is_dir = path.is_dir();
    matcher.matched(path, is_dir).is_ignore()
}

/*
 * Resolves and validates a user-provided path against the current working directory
 * Both the input path and cwd are canonicalized to eliminate '..', '.', and symlinks
 * Ensures the final path is within the cwd (or its subdirectories)
 * Prevents directory traversal and access to paths outside the project scope.
 * Falls back to the cwd if the input path is invalid or escapes the allowed root
 * Provides a safe base path for all subsequent file operations
 */
pub fn safe_path(path: PathBuf) -> PathBuf {
    // Current working directory
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Try to resolve both paths
    let safe_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => cwd.clone(), // fallback if invalid path
    };

    let safe_cwd = match cwd.canonicalize() {
        Ok(p) => p,
        Err(_) => cwd.clone(),
    };

    // Check if safe_path is inside cwd
    let final_path = if safe_path.starts_with(&safe_cwd) {
        safe_path
    } else {
        safe_cwd
    };

    // Return safe path
    final_path
}
