use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::path_guards::{PathType, safe_path};

pub fn create(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    let item_type = queries.get("item_type")
        .and_then(|v| v.as_str())
        .unwrap();

    let path = queries.get("path")
        .and_then(|v| v.as_str()).unwrap();

    let path = match safe_path(PathBuf::from(path), PathType::Any, false, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    if path.exists() {
        return json!({ "status": false, "message": "Path already exists" })
    }

    match item_type {
        "folder" => {
            match fs::create_dir_all(&path) {
                Ok(_) => json!({ "status": true, "message": format!("Folder created ({})", path.display()) }),
                Err(e) => json!({ "status": false, "message": format!("Failed to create folder ({})", e) }),
            }
        }
        "file" => {
            match File::create(&path) {
                Ok(_) => json!({ "status": true, "message": format!("File created ({})", path.display()) }),
                Err(e) => json!({ "status": false, "message": format!("Failed to create file ({})", e) }),
            }
        }
        _ => json!({ "status": false, "message": "Invalid item_type" })
    }
}
