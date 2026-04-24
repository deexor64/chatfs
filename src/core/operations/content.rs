use serde_json::{Value, json};
use std::{ collections::HashMap, fs::File, io::{BufRead, BufReader}};

use super::super::validators::content::validator;
use super::super::validators::utils::parse_lines::parse_lines;

const MAX_LINES: usize = 500;

pub fn content(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (path, lines) = validator(queries)?;

    // Open file
    let file = File::open(&path)
        .map_err(|_| format!("Failed to open file '{}'", path.display()))?;

    let reader = BufReader::new(file);
    let mut content_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => content_lines.push(l),
            Err(_) => {
                return Err(format!(
                    "File '{}' is a non-readable or binary file",
                    path.display()
                ));
            }
        }
    }

    // Parse line string
    let (start, end, note) = parse_lines(&lines, content_lines.len(), MAX_LINES);

    let selected = content_lines[start..end].to_vec();

    Ok(json!({
        "notes": note,
        "content": selected.join("\n")
    }))
}
