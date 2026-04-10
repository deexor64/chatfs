use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};
use std::env;

/*
 * Checks whether a given path should be ignored
 * Create the builder using build_matcher and pass it to is_ignored
 */
pub fn build_matcher(ignorefile: &PathBuf, work_dir: &PathBuf) -> Gitignore {
    let mut builder = GitignoreBuilder::new(work_dir);
    builder.add(ignorefile);
    builder.build().expect("Failed to build ignore matcher")
}

pub fn is_ignored(path: &Path, matcher: &Gitignore) -> bool {
    let is_dir = path.is_dir();
    matcher.matched(path, is_dir).is_ignore()
}

/*
 * Resolves and validates a user-provided path against the current working directory
 * Accepts any path string the user sends (absolute, relative, `../..`, `/`, etc)
 * Both the input path and work dir are canonicalized to eliminate '..', '.', and symlinks
 * Ensures the final path is within the cwd (or its subdirectories)
 * Folder paths can be equal to or under work dir while file paths must be under work dir
 * Prevents directory traversal and access to paths outside the project scope
 * Provides a safe base path for all subsequent file operations
 * ISSUE: Symlinked directories can still give access to outside via canonicalization
 * TODO: Add ignore rules here
 */
 #[derive(PartialEq)]
 pub enum PathType { // Path is supposed to be this type
     File,
     Any
 }

pub fn safe_path(path: PathBuf, item_type: PathType, allow_root: bool, _ignorefile: Option<&str>) -> Result<PathBuf, String> {
    // Get working dir
    let work_dir = env::current_dir()
        .map_err(|_| "Failed to determine current working directory (client side error)".to_string())?;

    // Check if work dir ("", ".", "./") is requested directly
    let root_requested = path.as_os_str().is_empty()
            || path == Path::new(".")
            || path == Path::new("./");

    // Check if requesting root directly is allowed
    if root_requested {
        if !allow_root {
            return Err("Access to project root is rejected for some path of this operation (check which one doesn't need or problematic to have root access)".to_string());
        }
        return Ok(work_dir.canonicalize().unwrap_or(work_dir));
    }

    // Get full path of requested path (including "../..", "/", "src/..", etc.)
    let full_path = if path.is_absolute() {
        path.clone()
    } else {
        work_dir.join(&path)
    };

    // Resolved working dir
    let resolved_cwd = work_dir.canonicalize().unwrap_or(work_dir.clone());

    // Resolved requested path
    let resolved_path = if full_path.exists() {
        full_path.canonicalize().map_err(|_| "Failed to canonicalize some path (client side error)")?
    } else {
        // Canonicalize the nearest existing ancestor
        let mut current = full_path.as_path();

        while !current.exists() {
            current = current.parent()
                .ok_or("Failed to canonicalize some path (client side error)")?;
        }

        let canonical_parent = current.canonicalize()
            .map_err(|_| "Failed to canonicalize some path (client side error)")?;

        // Rebuild the path from canonical parent
        let stripped = full_path.strip_prefix(current)
            .map_err(|_| "Failed to canonicalize some path (client side error)")?;

        canonical_parent.join(stripped)
    };

    // Reject paths outside of work dir
    if !resolved_path.starts_with(&resolved_cwd) {
        return Err(format!("Path '{}' is outside the project root", path.display()));
    }

    // Prevent using project root as a file
    if item_type == PathType::File && resolved_path == resolved_cwd {
        return Err("Cannot use project root as a file path".to_string());
    }

    Ok(resolved_path)
}
