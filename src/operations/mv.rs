use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_guards::{PathType, safe_path};

// Function is name 'mv' becuase 'move' is a rust reserved keyword
pub fn mv(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    let path = queries.get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let path = match safe_path(PathBuf::from(path), PathType::Any, false, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    let dest_path = queries.get("dest_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let dest_path = match safe_path(PathBuf::from(dest_path), PathType::Any, false, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    if !path.exists() {
        return json!({ "status": false, "message": "Source path does not exist" });
    }

    match fs::rename(&path, &dest_path) {
        Ok(_) => json!({ "status": true, "message": format!("Moved to {}", dest_path.display()) }),
        Err(e) => json!({ "status": false, "message": format!("Failed to move: {}", e) }),
    }
}
