use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::path_guards::{PathType, safe_path};

pub fn write(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    let lines = queries.get("lines")
        .and_then(|v| v.as_str())
        .unwrap_or("1-*");

    let mode = queries.get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("replace");

    let content = queries.get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let path = queries.get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let path = match safe_path(PathBuf::from(path), PathType::Any, false, ignore_file) {
        Ok(p) => p,
        Err(e) => return json!({ "status": false, "message": e })
    };

    // Path must be a file
    if !path.is_file() {
        return json!({ "status": false, "message": "Path is not a file or does not exist" });
    }

    // Read existing file lines
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return json!({ "status": false, "message": "Cannot open file for reading" }),
    };

    let reader = BufReader::new(file);
    let mut line_content: Vec<String> = reader.lines()
        .map(|l| l.unwrap_or_default())
        .collect();

    // Parse line range (start and end are 0-based internally)
    let (start, end) = parse_line_range(lines, line_content.len());

    // Prepare the new lines (we clone here so we can use the count later)
    let insert_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let insert_count = insert_lines.len();

    // Apply the change based on mode
    match mode {
        "replace" => {
            let end_idx = end.min(lines.len());
            if start < lines.len() {
                line_content.splice(start..end_idx, insert_lines);
            } else {
                // Append if start is beyond current length
                line_content.extend(insert_lines);
            }
        }
        "shift" => {
            if start <= lines.len() {
                line_content.splice(start..start, insert_lines);
            } else {
                line_content.extend(insert_lines);
            }
        }
        _ => {
            return json!({
                "status": false,
                "message": "Invalid mode. Use 'replace' or 'shift'"
            });
        }
    }

    // Write the updated content back to file
    match File::create(&path) {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", line_content.join("\n")) {
                return json!({ "status": false, "message": format!("Failed to write file: {}", e) });
            }
            json!({
                "status": true,
                "message": format!("Successfully wrote {} line(s) at position {}", insert_count, lines)
            })
        }
        Err(e) => json!({
            "status": false,
            "message": format!("Failed to open file for writing: {}", e)
        }),
    }
}

/// Parse lines parameter like "3-5", "1-*", "5-5", etc. Returns (start, end) as 0-based indices
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
        start + 1   // single line case: e.g. "5" → "5-5"
    };

    let end = end.min(total_lines);
    let start = start.min(end);

    (start, end)
}
