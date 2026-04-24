use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use super::utils::safe_path::{ExpectedType, SafePath};

pub fn validator(queries: &HashMap<String, String>) -> Result<(PathBuf, PathBuf), String> {
    let _path = queries
        .get("path")
        .map(|value| value.as_str())
        .ok_or_else(|| "path: Missing or invalid 'path' parameter".to_string())?;

    if _path.is_empty() {
        return Err("path: Path cannot be empty (e.g. 'path=src/ui', 'path=src/file.txt')".to_string());
    }

    let _dest_path = queries
        .get("dest_path")
        .map(|value| value.as_str())
        .ok_or_else(|| "dest_path: Missing or invalid 'dest_path' parameter".to_string())?;

    if _dest_path.is_empty() {
        return Err("dest_path: Destination path cannot be empty (e.g. 'dest_path=src/components', 'dest_path=src/test.py')".to_string());
    }

    let source = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyExist))
        .and_then(|p| p.ignore_rules())
        .map_err(|e| format!("path: {}", e))?
        .build();

    let destination = SafePath::from(PathBuf::from(_dest_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.ignore_rules())
        .map_err(|e| format!("dest_path: {}", e))?
        .build();

    Ok((source, destination))
}

pub fn resolve_destination(src: &Path, dest: PathBuf) -> Result<PathBuf, String> {
    if dest.exists() {
        if dest.is_dir() {
            let file_name = src.file_name().ok_or_else(|| "path: Invalid source path".to_string())?;
            return Ok(dest.join(file_name));
        }

        if src.is_dir() {
            return Err("dest_path: Cannot copy or move a folder onto an existing file path".to_string());
        }

        return Ok(dest);
    }

    if !src.is_dir() {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("dest_path: Failed to create parent directories ({})", e))?;
        }
    }

    Ok(dest)
}
