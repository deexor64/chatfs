use std::{collections::HashMap, path::PathBuf, sync::LazyLock};
use regex::Regex;

use super::utils::safe_path::{ExpectedType, SafePath};


static LINE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d+|\*)-(\d+|\*)$").unwrap()
});

pub fn validator(queries: &HashMap<String, String>) -> Result<(PathBuf, String), String> {
    let _lines = queries
        .get("lines")
        .cloned()
        .unwrap_or_else(|| "1-*".to_string());

    if !LINE_REGEX.is_match(&_lines) {
        return Err(
            "lines: Lines must follow 'start-end' (e.g. 2-2, 1-5, 1-*, *-10)"
                .to_string(),
        );
    }

    let lines = if _lines == "*-*" {
        "1-*".to_string()
    } else {
        _lines
    };

    let _path = match queries.get("path") {
        Some(value) => value,
        None => ".",
    };

    let _safe_path = SafePath::from(PathBuf::from(_path))
        .and_then(|p| p.no_direct_root())
        .and_then(|p| p.within_workspace())
        .and_then(|p| p.expect_type(ExpectedType::File))
        .and_then(|p| p.ignore_rules());

    let path = match _safe_path {
        Ok(p) => p.build(),
        Err(e) => return Err(format!("path: {}", e)),
    };

    Ok((path, lines))
}
