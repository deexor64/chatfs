use serde_json::{Value, json};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::core::{types::{ItemType, OpPath}, utils::safe_path::SafePath};

pub fn copy(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (source_path, dest_path) = parse_queries(queries)?;

    if source_path.resolved.is_dir() {
        copy_dir(&source_path.resolved, &dest_path.resolved)
            .map_err(|e| format!("Failed to copy '{}' to '{}' ({})", source_path.original.display(), dest_path.original.display(), e))?;
    } else {
        if let Some(parent) = dest_path.resolved.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directories for '{}' ({})", dest_path.original.display(), e))?;
        }

        fs::copy(&source_path.resolved, &dest_path.resolved)
            .map_err(|e| format!("Failed to copy '{}' to '{}' ({})", source_path.original.display(), dest_path.original.display(), e))?;
    }

    Ok(json!({"message": format!("Copied '{}' to '{}'", source_path.original.display(), dest_path.original.display())}))
}

fn copy_dir(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn parse_queries(queries: &HashMap<String, String>) -> Result<(OpPath, OpPath), String> {
    // Source path
    let path = match queries.get("path") {
        Some(value) => value,
        None => return Err("path: Missing or invalid 'path' parameter".to_string()),
    };

    let safe_path = SafePath::from(PathBuf::from(path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ItemType::AnyExist))
        .and_then(|p| p.ignore_rules());

    let path: OpPath = match safe_path {
        Ok(p) => p.build(),
        Err(e) => return Err(format!("path: {}", e)),
    };

    // Destination path
    let dest_path = match queries.get("dest_path") {
        Some(value) => value,
        None => return Err("dest_path: Missing or invalid 'dest_path' parameter".to_string()),
    };

    let safe_dest_path = SafePath::from(PathBuf::from(dest_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ItemType::AnyNonExist))
        .and_then(|p| p.ignore_rules());

    let dest_path: OpPath = match safe_dest_path {
        Ok(p) => p.build(),
        Err(e) => return Err(format!("dest_path: {}", e)),
    };

    Ok((path, dest_path))
}
