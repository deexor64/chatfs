use serde_json::{Value, json};
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use std::fs::File;

use crate::core::types::{ItemType, OpPath};
use crate::core::utils::safe_path::SafePath;


pub fn create(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, item_type) = parse_queries(queries)?;

    match item_type {
        ItemType::Folder => {
            fs::create_dir_all(&path.resolved)
                .map_err(|e| format!("Failed to create folder '{}' ({})", path.original.display(), e))?;

            Ok(json!({"message": format!("Folder '{}' created", path.original.display())}))
        },
        ItemType::File => {
            if let Some(parent) = path.resolved.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directories for '{}' ({})", path.original.display(), e))?;
            }

            File::create(&path.resolved)
                .map_err(|e| format!("Failed to create file '{}' ({})", path.original.display(), e))?;

            Ok(json!({"message": format!("File '{}' created", path.original.display())}))
        }
        _ => unreachable!(),
    }
}


fn parse_queries(queries: &HashMap<String, String>) -> Result<(OpPath, ItemType), String> {
    // Item type
    let item_type: ItemType = match queries.get("item_type") {
        Some(value) => match value.as_str() {
            "file" => ItemType::File,
            "folder" => ItemType::Folder,
            _ => return Err("item_type: Item type must be 'file' or 'folder' (literally)".into()),
        },
        None => return Err("item_type: Missing or invalid 'item_type' parameter".to_string()),
    };

    // Path
    let path = match queries.get("path") {
        Some(value) => value,
        None => ".",
    };

    let safe_path = SafePath::from(PathBuf::from(path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ItemType::AnyNonExist))
        .and_then(|p| p.ignore_rules());

    let path: OpPath = match safe_path {
        Ok(p) => p.build(),
        Err(e) => return Err(format!("path: {}", e)),
    };

    Ok((path, item_type))
}
