use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::path_guards::{PathType, safe_path};

pub fn copy(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
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


    let dest_path = match safe_path(PathBuf::from(dest_path), PathType::Any, true, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    if !path.exists() {
        return json!({ "status": false, "message": "Source path does not exist" });
    }

    if path.is_dir() {
        match copy_dir(&path, &dest_path) {
            Ok(_) => json!({ "status": true, "message": format!("Folder copied to {}", dest_path.display()) }),
            Err(e) => json!({ "status": false, "message": format!("Failed to copy folder: {}", e) }),
        }
    } else {
        match fs::copy(&path, &dest_path) {
            Ok(_) => json!({ "status": true, "message": format!("File copied to {}", dest_path.display()) }),
            Err(e) => json!({ "status": false, "message": format!("Failed to copy file: {}", e) }),
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
