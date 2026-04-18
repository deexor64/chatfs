use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};

/*
 * Checks whether a given path should be ignored based on ignorefile
 * Create the builder using build_matcher and pass it to is_ignored
 */
pub fn is_ignored(path: &Path, matcher: &Gitignore) -> bool {
    let is_dir = path.is_dir();
    matcher.matched(path, is_dir).is_ignore()
}

// Helper to build a matcher
pub fn build_matcher(ignorefile: &PathBuf, work_dir: &PathBuf) -> Gitignore {
    let mut builder = GitignoreBuilder::new(work_dir);
    builder.add(ignorefile);
    builder.build().expect("Failed to build ignore matcher")
}
