use serde_json::{Value, json};
use std::{collections::HashMap, fs};

use super::super::validators::delete::validator;

pub fn delete(queries: &HashMap<String, String>) -> Result<Value, String> {
    let path = validator(queries)?;

    if path.is_dir() {
        fs::remove_dir_all(&path)
            .map_err(|e| format!("Failed to delete folder '{}' ({})", path.display(), e))?;
        Ok(json!({"message": format!("Folder '{}' deleted", path.display())}))
    } else {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete file '{}' ({})", path.display(), e))?;
        Ok(json!({"message": format!("File '{}' deleted", path.display())}))
    }
}
