use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::path_validation::ignore_rules::{build_matcher};
use crate::path_validation::operational_path::{ExpectedType, OperationalPath};


pub fn create(queries: &HashMap<String, Value>, ignore_file: Option<&PathBuf>) -> Value {
    let item_type = match queries.get("item_type").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'item_type' parameter"}),
    };

    let _path = match queries.get("path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'path' parameter"}),
    };

    let mut op_path = OperationalPath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyNonExist));

    if let Some(ignore) = ignore_file {
        let matcher = build_matcher(ignore, &std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        op_path = op_path.and_then(|p| p.ignore_rules(&matcher));
    }

    let path = match op_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    match item_type {
        "folder" => {
            match fs::create_dir_all(&path) {
                Ok(_) => json!({ "status": true, "message": format!("Folder '{}' created", _path) }),
                Err(e) => json!({ "status": false, "message": format!("Failed to create folder '{}' ({})", _path, e) }),
            }
        }
        "file" => {
            if let Some(parent) = path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    return json!({ "status": false, "error": format!("Failed to create parent directories for '{}' ({})", _path, e) });
                }
            }

            match File::create(&path) {
                Ok(_) => json!({ "status": true, "message": format!("File '{}' created", _path) }),
                Err(e) => json!({ "status": false, "error": format!("Failed to create file '{}' ({})", _path, e) }),
            }
        }
        _ => json!({ "status": false, "message": "Invalid item_type" })
    }
}
