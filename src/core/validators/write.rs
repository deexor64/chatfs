use regex::Regex;
use std::{collections::HashMap, path::PathBuf};

use super::utils::safe_path::{ExpectedType, SafePath};

pub fn validator(
    queries: &HashMap<String, String>,
) -> Result<(PathBuf, String, String, String), String> {
    let lines = queries
        .get("lines")
        .cloned()
        .unwrap_or_else(|| "1-*".to_string());

    let re = Regex::new(r"^(\d+|\*)-(\d+|\*)$").unwrap();
    if !re.is_match(&lines) {
        return Err("lines: Lines must follow 'start-end' (e.g. 2-2, 1-5, 1-* or *-10)".to_string());
    }

    let lines = if lines == "*-*" { "1-*".to_string() } else { lines };

    let mode = queries
        .get("mode")
        .map(|value| value.as_str())
        .unwrap_or("replace");

    if mode != "shift" && mode != "replace" {
        return Err("mode: Mode must be 'shift' or 'replace'".to_string());
    }

    let content = queries
        .get("content")
        .map(|value| value.as_str())
        .unwrap_or("");

    if content.contains('\n') {
        return Err("content: Content must be a single line (no newline characters are allowed)".to_string());
    }

    if content.len() >= 200 {
        return Err("content: Line length cannot exceed 200 characters".to_string());
    }

    let _path = queries
        .get("path")
        .map(|value| value.as_str())
        .ok_or_else(|| "path: Missing or invalid 'path' parameter".to_string())?;

    if _path.is_empty() {
        return Err("path: File name cannot be empty (e.g. 'path=src/file.txt')".to_string());
    }

    let path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ExpectedType::File))
        .and_then(|p| p.ignore_rules())
        .map_err(|e| format!("path: {}", e))?
        .build();

    Ok((path, lines, mode.to_string(), content.to_string()))
}
