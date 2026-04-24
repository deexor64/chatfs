use ignore::WalkBuilder;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, path::{Path, PathBuf}};

use super::super::validators::{list::validator, utils::ignore_file::ensure_ignore_file};


#[derive(Serialize)]
struct Node {
    name: String,
    item_type: String, // file, directory
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Node>>,
}

pub fn list(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (recursive, item_type, path) = validator(queries)?;

    // Walk builder
    let mut builder = WalkBuilder::new(&path);

    // Add ignore file
    let ignore_file = ensure_ignore_file();

    if let Some(ignore) = ignore_file {
        builder.add_ignore(ignore);
    }

    // Set max depth based on recursive toggle
    builder.max_depth(if recursive { None } else { Some(1) });

    // Generate walker
    let walker = builder.build();

    // Collect valid paths
    let mut paths: Vec<PathBuf> = Vec::new();

    for entry in walker {
        if let Ok(e) = entry {
            let path = e.path().to_path_buf();

            // Apply filtering
            match item_type {
                "folder" if path.is_dir() => paths.push(path),
                "file" if path.is_file() => paths.push(path),
                _ => paths.push(path)
            }
        }
    }

    // Sort for deterministic output
    paths.sort();

    // Build JSON tree recursively
    let root_node = build_node(&path, &paths, item_type, true);

    // Return result
    Ok(serde_json::to_value(&root_node).unwrap())
}

fn build_node(root: &Path, paths: &[PathBuf], item_type: &str, is_root: bool) -> Node {
    let mut children = Vec::new();

    for path in paths {
        if let Ok(relative) = path.strip_prefix(root) {
            let components: Vec<_> = relative.components().collect();

            if components.len() == 1 {
                if path.is_dir() {
                    let child_node = build_node(path, paths, item_type, false);

                    if item_type != "file" || child_node.children.as_ref().map_or(false, |v| !v.is_empty()) {
                        children.push(child_node);
                    }
                } else if path.is_file() && (item_type != "folder") {
                    children.push(Node {
                        name: path
                            .file_name()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default(),
                        item_type: "file".to_string(),
                        children: None,
                    });
                }
            }
        }
    }

    // Determine node type for this root
    let is_dir = root.is_dir();

    // The worksapce root must be named "."
    let name = if is_root {
        ".".to_string()
    } else {
        root
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default()
    };

    Node {
        name: name,
        item_type: if is_dir { "folder".to_string() } else { "file".to_string() },
        children: if is_dir { Some(children) } else { None },
    }
}
