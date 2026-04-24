use serde_json::{Value, json};
use std::{collections::HashMap, fs};
use std::fs::File;

use super::super::validators::create::validator;

pub fn create(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, item_type) = validator(queries)?;

    match item_type.as_str() {
        "folder" => {
            fs::create_dir_all(&path)
                .map_err(|e| format!("Failed to create folder '{}' ({})", path.display(), e))?;
            Ok(json!({"message": format!("Folder '{}' created", path.display())}))
        }
        "file" => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directories for '{}' ({})", path.display(), e))?;
            }

            File::create(&path)
                .map_err(|e| format!("Failed to create file '{}' ({})", path.display(), e))?;

            Ok(json!({"message": format!("File '{}' created", path.display())}))
        }
        _ => unreachable!(),
    }
}
