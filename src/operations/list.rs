use ignore::WalkBuilder;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::path_guards::safe_path;

#[derive(Serialize)]
struct Node {
    name: String,
    item_type: String, // file, directory
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Node>>,
}

/*
 * Generates a JSON tree of the given relative path
 *
 * Parameters:
 *   - path: Path relative to working directory
 *                Examples: "", "src", "src/dir"
 *   - recursive: If true, child directories are listed recursively
 *                Special behavior with itemtype = 'file': directories are included only if they contain files
 *   - itemtype: Item type for listing:
 *             folder - list directories only
 *             file   - list files only
 *             all    - list both files and directories
 *   - ignore_file: Optional path to an ignore file
 *                  All paths in this file are excluded from the listing.
 *
 * Returns:
 *   - JSON representing the directory tree, relative to working directory
 */
pub fn list(queries: &HashMap<String, Value>, ignore_file: Option<&str>) -> Value {
    // Extract query params
    let recursive = queries.get("recursive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let item_type = queries.get("itemtype")
        .and_then(|v| v.as_str())
        .unwrap_or("all");

    let path = queries.get("path")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty()) // convert empty path
        .unwrap_or(".");

    let path = safe_path(PathBuf::from(path));

    // Builder
    let mut builder = WalkBuilder::new(&path);

    // Add ignore file
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
    let root_node = build_node(&path, &paths, item_type);

    // Serialize to JSON
    serde_json::to_value(&root_node).unwrap()

}


fn build_node(root: &Path, paths: &[PathBuf], item_type: &str) -> Node {
    let mut children = Vec::new();

    for path in paths {
        if let Ok(relative) = path.strip_prefix(root) {
            let components: Vec<_> = relative.components().collect();

            if components.len() == 1 {
                if path.is_dir() {
                    let child_node = build_node(path, paths, item_type);

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

    Node {
        name: root
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| root.to_string_lossy().to_string()),
        item_type: if is_dir { "folder".to_string() } else { "file".to_string() },
        children: if is_dir { Some(children) } else { None },
    }
}
