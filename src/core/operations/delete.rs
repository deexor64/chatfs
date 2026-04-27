use serde_json::{Value, json};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::core::{types::{ItemType, OpPath}, utils::safe_path::SafePath};


pub fn delete(queries: &HashMap<String, String>) -> Result<Value, String> {
    let path = parse_queries(queries)?;

    if path.resolved.is_dir() {
        fs::remove_dir_all(&path.resolved)
            .map_err(|e| format!("Failed to delete folder '{}' ({})", path.resolved.display(), e))?;
        Ok(json!({"message": format!("Folder '{}' deleted", path.resolved.display())}))
    } else {
        fs::remove_file(&path.resolved)
            .map_err(|e| format!("Failed to delete file '{}' ({})", path.resolved.display(), e))?;
        Ok(json!({"message": format!("File '{}' deleted", path.resolved.display())}))
    }
}


fn parse_queries(queries: &HashMap<String, String>) -> Result<OpPath, String> {
    // Path
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

    Ok(path)
}
