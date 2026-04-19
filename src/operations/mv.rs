use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_validation::ignore_rules::{build_matcher};
use crate::path_validation::operational_path::{ExpectedType, OperationalPath};

// Function is name 'mv' becuase 'move' is a rust reserved keyword
pub fn mv(queries: &HashMap<String, Value>, ignore_file: Option<&PathBuf>) -> Value {
    let _path = match queries.get("path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'path' parameter"}),
    };

    let op_path = OperationalPath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::AnyExist));

    let path = match op_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    let _dest_path = match queries.get("dest_path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'dest_path' parameter"}),
    };

    let mut op_dest_path = OperationalPath::from(PathBuf::from(_dest_path))
        .and_then(|p| p.within_workspace());

    if let Some(ignore) = ignore_file {
        let matcher = build_matcher(ignore, &std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        op_dest_path = op_dest_path.and_then(|p| p.ignore_rules(&matcher));
    }

    let dest_path = match op_dest_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    match fs::rename(&path, &dest_path) {
        Ok(_) => json!({ "status": true, "message": format!("Moved '{}' to '{}'", _path,  _dest_path) }),
        Err(e) => json!({ "status": false, "error": format!("Failed to move '{}' to '{}' ({})", _path,  _dest_path, e) }),
    }
}
