use ignore::WalkBuilder;
use serde::Serialize;
use std::path::{Path, PathBuf};

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
 *                Special behavior with filter = 'f': directories are included only if they contain files
 *   - filter: Optional filter for listing:
 *             folder - list directories only
 *             file   - list files only
 *             all    - list both files and directories
 *   - ignore_file: Optional path to an ignore file
 *                  All paths in this file are excluded from the listing.
 *
 * Returns:
 *   - JSON string representing the directory tree, relative to working directory
 */
pub fn list(path: &str, recursive: bool, item_type: Option<&str>, ignore_file: Option<&str>) -> String {
    // TODO: Add a guard to reject paths above the working directory

    let path = PathBuf::from(path);

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
                Some("folder") if path.is_dir() => paths.push(path),
                Some("file") if path.is_file() => paths.push(path),
                _ => paths.push(path)
            }
        }
    }

    // Sort for deterministic output
    paths.sort();

    // Build JSON tree recursively
    let root_node = build_node(&path, &paths, item_type);

    // Serialize to JSON string
    serde_json::to_string(&root_node).unwrap()
}


fn build_node(root: &Path, paths: &[PathBuf], filter: Option<&str>) -> Node {
    let mut children = Vec::new();

    for path in paths {
        if let Ok(relative) = path.strip_prefix(root) {
            let components: Vec<_> = relative.components().collect();

            if components.len() == 1 {
                if path.is_dir() {
                    let child_node = build_node(path, paths, filter);

                    if filter != Some("file") || child_node.children.as_ref().map_or(false, |v| !v.is_empty()) {
                        children.push(child_node);
                    }
                } else if path.is_file() && (filter != Some("folder")) {
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
