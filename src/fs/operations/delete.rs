use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::super::ignore::{build_matcher};
use super::super::safe_path::{ExpectedType, SafePath};

pub fn delete(queries: &HashMap<String, Value>, ignore_file: Option<&PathBuf>) -> Value {
    let _path = match queries.get("path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'path' parameter"}),
    };

    let mut op_path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyExist));

    if let Some(ignore) = ignore_file {
        let matcher = build_matcher(ignore, &std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        op_path = op_path.and_then(|p| p.ignore_rules(&matcher));
    }

    let path = match op_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    if path.is_dir() {
        match fs::remove_dir_all(&path) {
            Ok(_) => json!({ "status": true, "message": format!("Folder '{}' deleted", _path) }),
            Err(e) => json!({ "status": false, "error": format!("Failed to delete folder '{}' ({})",_path,  e) }),
        }
    } else {
        match fs::remove_file(&path) {
            Ok(_) => json!({ "status": true, "message": format!("File '{}' deleted", _path) }),
            Err(e) => json!({ "status": false, "error": format!("Failed to delete file '{}' ({})",_path,  e) }),
        }
    }
}
