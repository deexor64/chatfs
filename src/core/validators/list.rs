use std::{collections::HashMap, path::PathBuf};

use super::utils::safe_path::{ExpectedType, SafePath};


pub fn validator(queries: &HashMap<String, String>) -> Result<(bool, &str, PathBuf), String> {
    // Validation
    let recursive = match queries.get("recursive") {
        Some(value) => match value.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err("recursive: Recursion must be 'true' or 'false' (literally)".into()),
        },
        None => false,
    };

    let item_type = match queries.get("item_type") {
        Some(value) => {
            let v = value.as_str();
            if v == "file" || v == "folder" || v == "all" {
                v
            } else {
                return Err("item_type: Item type must be 'file' or 'folder' or 'all' (literally)".into());
            }
        },
        None => "all",
    };

    let _path = match queries.get("path") {
        Some(value) => value,
        None => ".",
    };

    let _safe_path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.ignore_rules())
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.expect_type(ExpectedType::Dir));

    let path = match _safe_path {
        Ok(p) => p.build(),
        Err(e) => return Err(e)
    };

    Ok((recursive, item_type, path))
}
