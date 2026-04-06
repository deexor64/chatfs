use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};


pub fn build_matcher(ignorefile: &str) -> Gitignore {
    let mut builder = GitignoreBuilder::new(".");
    builder.add(ignorefile);

    builder.build().expect("Failed to build ignore matcher")
}

pub fn is_ignored(path: &Path, matcher: &Gitignore) -> bool {
    let is_dir = path.is_dir();
    matcher.matched(path, is_dir).is_ignore()
}

fn main() {
    let matcher = build_matcher(".myignore");

    let test_paths = vec![
        PathBuf::from("target/debug/app"),
        PathBuf::from("src/main.rs"),
        PathBuf::from("logs/error.log"),
    ];

    for path in test_paths {
        if is_ignored(&path, &matcher) {
            println!("IGNORED: {}", path.display());
        } else {
            println!("NOT IGNORED: {}", path.display());
        }
    }
}
