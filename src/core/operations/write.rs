use serde_json::Value;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::core::types::{ItemType, Line, OpPath, WriteMode};
use crate::core::utils::{parse_lines::parse_lines, safe_lines::safe_lines, safe_path::SafePath};


pub fn write(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, lines, mode, content) = parse_queries(queries)?;

    // Open file
    let file = File::open(&path.resolved)
        .map_err(|_| "Error opening file")?;

    let reader = BufReader::new(file);
    let mut line_content: Vec<String> = reader.lines().map(|l| l.unwrap_or_default()).collect();

    let (start, end) = safe_lines(lines, line_content.len())?;
    let insert_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let insert_count = insert_lines.len();

    match mode {
        WriteMode::Replace => {
            let end_idx = end.min(line_content.len());
            if start < line_content.len() {
                line_content.splice(start..end_idx, insert_lines);
            } else {
                line_content.extend(insert_lines);
            }
        },
        WriteMode::Shift => {
            if start <= line_content.len() {
                line_content.splice(start..start, insert_lines);
            } else {
                line_content.extend(insert_lines);
            }
        }
    }

    let mut file = File::create(&path.resolved)
        .map_err(|e| format!("Failed to write file '{}' ({})", path.original.display(), e))?;

    write!(file, "{}", line_content.join("\n"))
        .map_err(|e| format!("Failed to write file '{}' ({})", path.original.display(), e))?;

    Ok(Value::String(format!("Successfully wrote {} line(s) of '{}'", insert_count, path.original.display())))
}

fn parse_queries(queries: &HashMap<String, String>) -> Result<(OpPath, (Line, Line), WriteMode, String), String> {
    // Path
    let path_str = match queries.get("path") {
        Some(value) => value,
        None => return Err("path: Missing or invalid 'path' parameter".to_string()),
    };

    let safe_path = SafePath::from(PathBuf::from(path_str))
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

    // Mode
    let mode: WriteMode = match queries.get("mode") {
        Some(value) => match value.as_str() {
            "shift" => WriteMode::Shift,
            "replace" => WriteMode::Replace,
            _ => return Err("mode: Mode must be 'shift' or 'replace'".to_string()),
        },
        None => WriteMode::Shift,
    };

    // Content
    let content: String = match queries.get("content") {
        Some(value) => value.clone(),
        None => return Err("content: Missing or invalid 'content' parameter".to_string()),
    };

    Ok((path, lines, mode, content))
}
