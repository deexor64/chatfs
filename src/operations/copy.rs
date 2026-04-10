use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_validation::operational_path::{ExpectedType, OperationalPath};

pub fn copy(queries: &HashMap<String, Value>, _ignore_file: Option<&str>) -> Value {
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

    if path.is_dir() {
        match copy_dir(&path, &dest_path) {
            Ok(_) => json!({ "status": true, "message": format!("Copied '{}' to '{}'", _path,  _dest_path) }),
            Err(e) => json!({ "status": false, "error": format!("Failed to copy '{}' to '{}' ({})", _path,  _dest_path, e)}),
        }
    } else {
        match fs::copy(&path, &dest_path) {
            Ok(_) => json!({ "status": true, "message": format!("Copied '{}' to '{}'", _path,  _dest_path) }),
            Err(e) => json!({ "status": false, "error": format!("Failed to copy '{}' to '{}' ({})", _path,  _dest_path, e)}),
        }
    }
}

fn copy_dir(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
