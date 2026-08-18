//! The gate over README.md's recipe listing: it is the listing `just` prints.

use jellium_reference::tree;
use std::path::Path;
use std::process::Command;

/// The line the fenced listing opens with.
const OPENING: &str = "Available recipes:";

/// The fence a listing stands between.
const FENCE: &str = "```";

/// The listing README.md carries, taken from the fenced block that opens with
/// `Available recipes:`.
fn listed(readme: &str) -> String {
    let at = readme
        .find(OPENING)
        .unwrap_or_else(|| panic!("README.md carries no block opening with {OPENING}"));
    let rest = &readme[at..];
    let end = rest
        .find(&format!("\n{FENCE}"))
        .unwrap_or_else(|| panic!("README.md's listing stands under no closing fence"));
    rest[..end].to_owned()
}

/// The listing `just` prints for the justfile at `root`.
fn printed(root: &Path) -> String {
    let output = Command::new("just")
        .arg("--list")
        .current_dir(root)
        .output()
        .unwrap_or_else(|error| panic!("just does not run: {error}"));
    assert!(
        output.status.success(),
        "just --list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("just prints text")
        .trim_end()
        .to_owned()
}

#[test]
fn the_readme_lists_the_recipes_the_justfile_holds() {
    let root = tree::workspace_root();
    let readme = std::fs::read_to_string(root.join("README.md")).expect("README.md is readable");
    assert_eq!(
        listed(&readme),
        printed(&root),
        "README.md's listing is not the listing just prints"
    );
}
