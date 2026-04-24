use std::{collections::HashMap, path::PathBuf};

use super::utils::safe_path::{ExpectedType, SafePath};

pub fn validator(queries: &HashMap<String, String>) -> Result<(PathBuf, String), String> {
    let item_type = queries
        .get("item_type")
        .map(|value| value.as_str())
        .ok_or_else(|| "item_type: Missing or invalid 'item_type' parameter".to_string())?;

    if item_type != "folder" && item_type != "file" {
        return Err("item_type: Item type must be 'folder' or 'file'".to_string());
    }

    let _path = queries
        .get("path")
        .map(|value| value.as_str())
        .ok_or_else(|| "path: Missing or invalid 'path' parameter".to_string())?;

    if _path.is_empty() {
        return Err("path: Path cannot be empty (e.g. 'path=src/ui', 'path=src/file.txt')".to_string());
    }

    let path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyNonExist))
        .and_then(|p| p.ignore_rules())
        .map_err(|e| format!("path: {}", e))?
        .build();

    Ok((path, item_type.to_string()))
}
