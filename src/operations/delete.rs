use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_guards::{PathType, safe_path};

pub fn delete(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    let path = queries.get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let path = match safe_path(PathBuf::from(path), PathType::Any, false, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    if path.is_dir() {
        match fs::remove_dir_all(&path) {
            Ok(_) => json!({ "status": true, "message": format!("Folder deleted: {}", path.display()) }),
            Err(e) => json!({ "status": false, "message": format!("Failed to delete folder: {}", e) }),
        }
    } else if path.is_file() {
        match fs::remove_file(&path) {
            Ok(_) => json!({ "status": true, "message": format!("File deleted: {}", path.display()) }),
            Err(e) => json!({ "status": false, "message": format!("Failed to delete file: {}", e) }),
        }
    } else {
        json!({ "status": false, "message": "Path not found" })
    }
}
