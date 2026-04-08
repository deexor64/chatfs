use serde_json::{Value, json};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use crate::path_guards::{build_matcher, is_ignored, safe_path};

pub fn content(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    // Extract query params
    let lines = queries.get("lines")
        .and_then(|v| v.as_str())
        .unwrap_or("1-*");

    let path = queries.get("path")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty()) // convert empty path
        .unwrap_or(".");

    let path = safe_path(PathBuf::from(path));

    // Check ignore
    if let Some(ignore) = ignore_file {
        let matcher = build_matcher(ignore);

        if is_ignored(&path, &matcher) {
            return json!({
                "status": false,
                "message": "Path is ignored".to_string()
            });
        }
    }

    // Check if path is a directory
    if path.is_dir() {
        return json!({
            "status": false,
            "message": "Path is a directory".to_string()
        });
    }

    // Open the file
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Value::String("File not found or inaccessible".to_string()),
    };

    // Read lines
    let reader = BufReader::new(file);
    let mut content_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => content_lines.push(l),
            Err(_) => {
                return Value::String("Non-readable or binary file".to_string());
            }
        }
    }

    // Parse lines parameter
    let (start, end, note) = parse_lines(lines, content_lines.len(), 200);

    // Extract and send selected lines
    let selected: Vec<String> = content_lines[start..end].to_vec();

    json!({
        "status": true,
        "note": note,
        "content": selected.join("\n")
    })
}

// Parses a lines string like "1-5", "3-*", "*-10"
// Returns a 0-based start index and end index (exclusive)
// Also max number of lines sent at once is limited
fn parse_lines(lines: &str, total_lines: usize, max_lines: usize) -> (usize, usize, String) {
    let parts: Vec<&str> = lines.split('-').collect();

    // Parse start and end line
    let start = if parts[0] == "*" {
        0
    } else {
        parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1)
    };

    let mut end = if parts[1] == "*" {
        total_lines
    } else {
        parts[1].parse::<usize>().unwrap_or(total_lines)
    };

    // Adjustment notes
    let mut note = String::new();

    // Clamp end to total lines
    if end > total_lines {
        end = total_lines;
        note.push_str("End line exceeded total lines - clamped; ");
    }

    // Clamp range to max_lines
    if end.saturating_sub(start) > max_lines {
        end = start + max_lines;
        note.push_str(format!("Requested range exceeded max lines - truncated; ").as_str());
    }

    // Ensure end is greater than or equal to start
    if end < start {
        end = start;
        note.push_str("End line less than start - adjusted; ");
    }

    (start, end, note)
}
