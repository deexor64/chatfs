use std::{collections::HashMap, path::PathBuf};

use super::utils::safe_path::{ExpectedType, SafePath};

pub fn validator(queries: &HashMap<String, String>) -> Result<PathBuf, String> {
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
        .and_then(|p| p.expect_type(ExpectedType::AnyExist))
        .and_then(|p| p.ignore_rules())
        .map_err(|e| format!("path: {}", e))?
        .build();

    Ok(path)
}
