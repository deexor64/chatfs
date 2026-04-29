use serde_json::Value;
use std::{ collections::HashMap, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use crate::core::types::{ItemType, Line, OpPath};
use crate::core::utils::{parse_lines::parse_lines, safe_lines::safe_lines, safe_path::SafePath};


pub fn content(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, lines) = parse_queries(queries)?;

    // Open file
    let file = File::open(path.resolved)
        .map_err(|_| format!("Failed to open file '{}'", path.original.display()))?;

    let reader = BufReader::new(file);

    // Read file content
    let mut content: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => content.push(l),
            Err(_) => {
                return Err(format!(
                    "File '{}' is a non-readable or binary file",
                    path.original.display()
                ));
            }
        }
    }

    // Empty file
    if content.is_empty() {
        return Ok(Value::String("".to_string()));
    }

    // Safe line range
    let (start, end) = safe_lines(lines, content.len())?;

    // Extract selected lines
    let selected = content[start..end].to_vec();

    Ok(Value::String(selected.join("\n")))
}


fn parse_queries(queries: &HashMap<String, String>) -> Result<(OpPath, (Line, Line)), String> {
    // Path
    let path = match queries.get("path") {
        Some(value) => value,
        None => return Err("path: Missing or invalid 'path' parameter".to_string()),
    };

    let safe_path = SafePath::from(PathBuf::from(path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.expect_type(ItemType::File))
        .and_then(|p| p.ignore_rules());

    let path: OpPath = match safe_path {
        Ok(p) => p.build(),
        Err(e) => return Err(format!("path: {}", e)),
    };

    // Lines
    let lines = match queries.get("lines") {
        Some(value) => value.clone(),
        None => "1-*".to_string(),
    };

    let lines: (Line, Line) = parse_lines(lines)?;

    Ok((path, lines))
}
