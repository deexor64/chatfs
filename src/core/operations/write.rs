use serde_json::{Value, json};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::core::{types::{ItemType, Line, OpPath, WriteMode}, utils::{parse_lines::parse_lines, safe_path::SafePath}};


pub fn write(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, lines, mode, content) = parse_queries(queries)?;
    
    // Opening file
    let file = File::open(path.resolved)
        .map_err(|_| "Error opening file")?;

    let reader = BufReader::new(file);
    let mut line_content: Vec<String> = reader.lines().map(|l| l.unwrap_or_default()).collect();

    let (start, end) = parse_line_range(&lines, line_content.len());
    let insert_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let insert_count = insert_lines.len();

    match mode.as_str() {
        "replace" => {
            let end_idx = end.min(line_content.len());
            if start < line_content.len() {
                line_content.splice(start..end_idx, insert_lines);
            } else {
                line_content.extend(insert_lines);
            }
        }
        "shift" => {
            if start <= line_content.len() {
                line_content.splice(start..start, insert_lines);
            } else {
                line_content.extend(insert_lines);
            }
        }
        _ => {
            return Err("mode: Invalid mode".to_string());
        }
    }

    let mut file = File::create(&path)
        .map_err(|e| format!("Failed to write file '{}' ({})", path.display(), e))?;

    write!(file, "{}", line_content.join("\n"))
        .map_err(|e| format!("Failed to write file '{}' ({})", path.display(), e))?;

    Ok(json!({
        "message": format!("Successfully wrote {} line(s) at position '{}' of '{}'", insert_count, lines, path.display())
    }))
}

fn parse_line_range(lines_param: &str, total_lines: usize) -> (usize, usize) {
    let parts: Vec<&str> = lines_param.split('-').collect();

    let start = if parts[0] == "*" || parts[0].is_empty() {
        0
    } else {
        parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1)
    };

    let end = if parts.len() > 1 {
        if parts[1] == "*" || parts[1].is_empty() {
            total_lines
        } else {
            parts[1].parse::<usize>().unwrap_or(total_lines)
        }
    } else {
        start + 1
    };

    let end = end.min(total_lines);
    let start = start.min(end);

    (start, end)
}



pub fn parse_queries(queries: &HashMap<String, String>) -> Result<(OpPath, (Line, Line), WriteMode, String), String> {
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
        None => "*-*".to_string(),
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
    let content = queries
        .get("content")
        .map(|value| value.as_str())
        .unwrap_or("");

    Ok((path, lines, mode, content.to_string()))
}
