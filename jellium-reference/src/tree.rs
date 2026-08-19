//! The one walk of the repository the gates read the tree by.

use std::path::{Path, PathBuf};

/// A file extension the gates read the tree by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extension(&'static str);

impl Extension {
    pub const JAVASCRIPT: Extension = Extension("js");
    pub const RUST: Extension = Extension("rs");
    pub const TEXT: Extension = Extension("txt");
    pub const TOML: Extension = Extension("toml");
    pub const YAML: Extension = Extension("yaml");
    pub const YML: Extension = Extension("yml");

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

/// The workspace this package sits in.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package directory sits inside the workspace")
        .to_path_buf()
}

/// The package the gates live in, which guards the port and takes part in none
/// of it, so the names its own text spells cite nothing.
pub fn guard(root: &Path) -> PathBuf {
    root.join("jellium-reference")
}

/// The vendored third-party sources, which take no part in the port, so the
/// names their text spells count for nothing.
pub fn vendored(root: &Path) -> PathBuf {
    root.join("jellium-web").join("vendor")
}

/// The build output and the history, which carry no source of this tree's own.
fn unread(name: &str) -> bool {
    matches!(name, "target" | ".git")
}

/// Every file under `root` carrying one of `extensions`, sorted. A directory
/// that does not read is a failure rather than an empty answer, so a gate
/// reading this walk cannot pass by finding nothing.
pub fn files_under(root: &Path, extensions: &[Extension]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("{} does not read: {error}", directory.display()));
        for entry in entries {
            let path = entry
                .unwrap_or_else(|error| {
                    panic!("an entry of {} does not read: {error}", directory.display())
                })
                .path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if unread(name) {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| extensions.iter().any(|held| held.0 == value))
            {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}
