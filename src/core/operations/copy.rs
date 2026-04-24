use serde_json::{Value, json};
use std::{collections::HashMap, fs, path::PathBuf};

use super::super::validators::copy_move::{resolve_destination, validator};

pub fn copy(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (source_path, dest_path) = validator(queries)?;
    let final_dest = resolve_destination(&source_path, dest_path)?;

    if source_path.is_dir() {
        copy_dir(&source_path, &final_dest)
            .map_err(|e| format!("Failed to copy '{}' to '{}' ({})", source_path.display(), final_dest.display(), e))?;
    } else {
        if let Some(parent) = final_dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directories for '{}' ({})", final_dest.display(), e))?;
        }

        fs::copy(&source_path, &final_dest)
            .map_err(|e| format!("Failed to copy '{}' to '{}' ({})", source_path.display(), final_dest.display(), e))?;
    }

    Ok(json!({"message": format!("Copied '{}' to '{}'", source_path.display(), final_dest.display())}))
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
