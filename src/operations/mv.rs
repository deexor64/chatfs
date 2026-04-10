use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_validation::operational_path::{ExpectedType, OperationalPath};

// Function is name 'mv' becuase 'move' is a rust reserved keyword
pub fn mv(queries: &HashMap<String, Value>, _ignore_file: Option<&str>) -> Value {
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

    let _dest_path = queries.get("dest_path")
        .and_then(|v| v.as_str()).unwrap();

    let _op_dest_path = OperationalPath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace());

    let dest_path = match _op_dest_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    match fs::rename(&path, &dest_path) {
        Ok(_) => json!({ "status": true, "message": format!("Moved '{}' to '{}'", _path,  _dest_path) }),
        Err(e) => json!({ "status": false, "error": format!("Failed to move '{}' to '{}' ({})", _path,  _dest_path, e) }),
    }
}
