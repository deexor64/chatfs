use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::path_validation::operational_path::{ExpectedType, OperationalPath};


pub fn create(queries: &HashMap<String, Value>, _ignore_file: Option<&str>) -> Value {
    let item_type = queries.get("item_type")
        .and_then(|v| v.as_str())
        .unwrap();

    let _path = queries.get("path")
        .and_then(|v| v.as_str()).unwrap();

    let _op_path = OperationalPath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyNonExist));

    let path = match _op_path {
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
            match File::create(&path) {
                Ok(_) => json!({ "status": true, "message": format!("File '{}' created", _path) }),
                Err(e) => json!({ "status": false, "message": format!("Failed to create file '{}' ({})", _path, e) }),
            }
        }
        _ => json!({ "status": false, "message": "Invalid item_type" })
    }
}
