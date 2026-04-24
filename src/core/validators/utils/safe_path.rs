use std::{env, path::{Path, PathBuf}};
use ignore::gitignore::{GitignoreBuilder};

use crate::core::validators::utils::ignore_file::ensure_ignore_file;

pub struct SafePath {
    original_path: PathBuf,
    resolved_path: PathBuf,
    workspace: PathBuf,
}

#[derive(PartialEq)]
pub enum ExpectedType {
    File, // Imply exists
    Dir,  // Imply exists
    AnyExist,
    AnyNonExist
}

/*
 * Resolves and validates a user-provided path against the current workspace directory
 * Does not care about path existance as long as path string is valid and within the scope
 * Accepts any path string the user sends (absolute, relative, `../..`, `/`, etc)
 * Ensures the canonical path is within the workspace dir (or its subdirectories)
 * Directory paths can be equal to or under workspace dir while file paths must be under workspace dir
 * Prevents directory traversal and access to paths outside the workspace scope
 * ISSUE: Symlinked directories can still give access to outside via canonicalization
 */
impl SafePath {
    pub fn from(path: PathBuf) -> Result<Self, String> {
        let work_dir = env::current_dir()
            .map_err(|_| "Failed to determine current working directory (client error)".to_string())?;

        let resolved_cwd = work_dir
            .canonicalize()
            .unwrap_or(work_dir.clone());

        // Build full path
        let full_path = if path.is_absolute() {
            path.clone()
        } else {
            resolved_cwd.join(&path)
        };

        // Resolve path (handles non-existent paths)
        let resolved_path = if full_path.exists() {
            full_path
                .canonicalize()
                .map_err(|_| "Failed to canonicalize path (client error)".to_string())?
        } else {
            let mut current = full_path.as_path();

            while !current.exists() {
                current = current.parent()
                    .ok_or("Failed to canonicalize path (client error)".to_string())?;
            }

            let canonical_parent = current
                .canonicalize()
                .map_err(|_| "Failed to canonicalize path (client error)".to_string())?;

            let stripped = full_path
                .strip_prefix(current)
                .map_err(|_| "Failed to canonicalize path (client error)".to_string())?;

            canonical_parent.join(stripped)
        };

        Ok(Self {
            original_path: path,
            resolved_path: resolved_path,
            workspace: resolved_cwd,
        })
    }

    // Ensure path is inside workspace
    pub fn within_workspace(self) -> Result<Self, String> {
        if !self.resolved_path.starts_with(&self.workspace) {
            return Err(format!(
                "Accessing path '{}' is not allowed",
                self.original_path.display()
            ));
        }
        
        Ok(self)
    }

    // Reject or allow root access
    // Is not effective with ExpectedType::File
    pub fn no_direct_root(self) -> Result<Self, String> {
        let root_requested =
            self.original_path.as_os_str().is_empty()
            || self.original_path == Path::new(".")
            || self.original_path == Path::new("./");

        if root_requested {
            return Err(format!(
                "Direct reference to workspace root '{}' is not required",
                self.original_path.display()));
        }

        Ok(self)
    }

    // Validate expected type
    pub fn expect_type(self, expected: ExpectedType) -> Result<Self, String> {
        match expected {
            ExpectedType::File => {
                if self.resolved_path == self.workspace {
                    return Err(format!(
                        "Cannot use workspace root '{}' as a file",
                        self.original_path.display()
                    ));
                }

                if !self.resolved_path.is_file() {
                    return Err(format!(
                        "Path '{}' does not exists or is not a file",
                        self.original_path.display()
                    ));
                }
            },
            ExpectedType::Dir => {
                if !self.resolved_path.is_dir() {
                    return Err(format!(
                        "Path '{}' does not exists or is not a folder",
                        self.original_path.display()
                    ));
                }
            },
            ExpectedType::AnyExist => {
                if !self.resolved_path.exists() {
                    return Err(format!(
                        "Path '{}' does not exists",
                        self.original_path.display()
                    ));
                }
            }
            ExpectedType::AnyNonExist => {
                if self.resolved_path.exists() {
                    return Err(format!(
                        "Path '{}' already exists",
                        self.original_path.display()
                    ));
                }
            }
        }

        Ok(self)
    }

    // Apply ignore rules
    pub fn ignore_rules(self) -> Result<Self, String> {
        // Build the ignore matcher
        let mut builder = GitignoreBuilder::new(&self.workspace);
        let ignore_file = ensure_ignore_file();

        if let Some(ignore_file) = ignore_file {
            builder.add(&ignore_file);
        }

        let matcher = builder.build().ok();

        // Apply ignore rules
        match matcher {
            Some(matcher) => {
                let is_dir = self.resolved_path.is_dir();

                if matcher.matched(&self.resolved_path, is_dir).is_ignore() {
                    // Output error should not hint the existance of the file
                    return Err(format!(
                        "Path '{}' does not exists",
                        self.original_path.display()
                    ));
                }

                Ok(self)
            },
            None => Ok(self),
        }
    }

    // Extract final usable path
    pub fn build(self) -> PathBuf {
        self.resolved_path
    }
}
