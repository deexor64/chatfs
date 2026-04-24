use serde_json::{Value, json};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use super::super::ignore::{build_matcher};
use super::super::safe_path::{ExpectedType, SafePath};


pub fn content(queries: &HashMap<String, String>) -> Result<ExecutionResult, String> {
    let lines = match queries.get("lines").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'lines' parameter"}),
    };

    let _path = match queries.get("path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return json!({"status": false, "error": "Missing or invalid 'path' parameter"}),
    };

    let mut op_path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.expect_type(ExpectedType::File));

    if let Some(ignore) = ignore_file {
        let matcher = build_matcher(ignore, &std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        op_path = op_path.and_then(|p| p.ignore_rules(&matcher));
    }

    let path = match op_path {
        Ok(op) => op.build(),
        Err(e) => return json!({"status": false, "error": format!("path: {}", e)})
    };

    // Open the file
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return json!({"status": false, "error": format!("Failed to open file '{}'", _path)}),
    };

    // Read lines
    let reader = BufReader::new(file);
    let mut content_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => content_lines.push(l),
            Err(_) => {
                return json!({"status": false, "error": format!("File '{}' is a non-readable or binary file", _path)});
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
        note.push_str("Requested range exceeded max lines - truncated; ");
    }

    // Ensure end is greater than or equal to start
    if end < start {
        end = start;
        note.push_str("End line less than start - adjusted; ");
    }

    (start, end, note)
}
