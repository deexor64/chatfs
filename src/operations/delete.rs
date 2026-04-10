use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_validation::operational_path::{ExpectedType, OperationalPath};

pub fn delete(queries: &HashMap<String, Value>, _ignore_file: Option<&str>) -> Value {
    let _path = queries.get("path")
        .and_then(|v| v.as_str()).unwrap();

    let _op_path = OperationalPath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyExist));

    let path = match _op_path {
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
