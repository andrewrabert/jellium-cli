use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package directory sits inside the workspace")
        .to_path_buf()
}

fn ignored(name: &str) -> bool {
    matches!(name, "target" | ".git")
}

fn files_under(root: &Path, extension: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory).expect("the directory is readable");
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            let name = match path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name,
                None => continue,
            };
            if ignored(name) {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some(extension) {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn named_under(root: &Path, file_name: &str) -> Vec<PathBuf> {
    files_under(root, "toml")
        .into_iter()
        .filter(|path| path.file_name().and_then(|name| name.to_str()) == Some(file_name))
        .collect()
}

/// Every `.rs` file in the repository this test can reach.
fn sources(root: &Path) -> Vec<PathBuf> {
    let itself = root.join("jellium-reference/tests/suppressions.rs");
    files_under(root, "rs")
        .into_iter()
        .filter(|path| path != &itself)
        .collect()
}

fn string_end(chars: &[char], at: usize) -> Option<usize> {
    let mut index = at + 1;
    while index < chars.len() {
        match chars[index] {
            '\\' => index += 2,
            '"' => return Some(index + 1),
            _ => index += 1,
        }
    }
    None
}

fn raw_string_end(chars: &[char], at: usize) -> Option<usize> {
    let mut hashes = 0usize;
    let mut index = at + 1;
    while chars.get(index) == Some(&'#') {
        hashes += 1;
        index += 1;
    }
    if chars.get(index) != Some(&'"') {
        return None;
    }
    index += 1;
    while index < chars.len() {
        if chars[index] == '"' {
            let closed = (1..=hashes).all(|offset| chars.get(index + offset) == Some(&'#'));
            if closed {
                return Some(index + hashes + 1);
            }
        }
        index += 1;
    }
    None
}

fn char_literal_end(chars: &[char], at: usize) -> Option<usize> {
    if chars.get(at + 1) == Some(&'\\') {
        let mut index = at + 2;
        while index < chars.len() {
            match chars[index] {
                '\\' => index += 2,
                '\'' => return Some(index + 1),
                _ => index += 1,
            }
        }
        return None;
    }
    if chars.get(at + 2) == Some(&'\'') {
        return Some(at + 3);
    }
    None
}

fn line_comment_end(chars: &[char], at: usize) -> usize {
    let mut index = at + 2;
    while index < chars.len() && chars[index] != '\n' {
        index += 1;
    }
    index
}

fn block_comment_end(chars: &[char], at: usize) -> usize {
    let mut index = at + 2;
    let mut depth = 1usize;
    while index < chars.len() && depth > 0 {
        if chars[index] == '/' && chars.get(index + 1) == Some(&'*') {
            depth += 1;
            index += 2;
        } else if chars[index] == '*' && chars.get(index + 1) == Some(&'/') {
            depth -= 1;
            index += 2;
        } else {
            index += 1;
        }
    }
    index
}

fn trivia_end(chars: &[char], at: usize) -> Option<usize> {
    match chars[at] {
        '/' if chars.get(at + 1) == Some(&'/') => Some(line_comment_end(chars, at)),
        '/' if chars.get(at + 1) == Some(&'*') => Some(block_comment_end(chars, at)),
        'r' if matches!(chars.get(at + 1), Some('"' | '#')) => raw_string_end(chars, at),
        '"' => string_end(chars, at),
        '\'' => char_literal_end(chars, at),
        _ => None,
    }
}

fn bracket_close(chars: &[char], from: usize) -> Option<usize> {
    let mut index = from;
    let mut depth = 1usize;
    while index < chars.len() {
        if let Some(next) = trivia_end(chars, index) {
            index = next;
            continue;
        }
        match chars[index] {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn attribute_body(chars: &[char], at: usize) -> Option<usize> {
    match chars.get(at + 1) {
        Some('[') => Some(at + 2),
        Some('!') if chars.get(at + 2) == Some(&'[') => Some(at + 3),
        _ => None,
    }
}

fn code_only(chars: &[char], from: usize, until: usize) -> String {
    let mut text = String::new();
    let mut index = from;
    while index < until {
        if let Some(next) = trivia_end(chars, index) {
            for skipped in &chars[index..next.min(until)] {
                text.push(if *skipped == '\n' { '\n' } else { ' ' });
            }
            index = next;
            continue;
        }
        text.push(chars[index]);
        index += 1;
    }
    text
}

fn names_a_level(body: &str) -> bool {
    ["allow", "expect"].iter().any(|level| {
        body.match_indices(level).any(|(at, _)| {
            let before = body[..at].chars().next_back();
            let after = body[at + level.len()..].trim_start();
            !before.is_some_and(|value| value.is_alphanumeric() || value == '_')
                && after.starts_with('(')
        })
    })
}

/// Every attribute in `text` that names a lint level of `allow` or `expect`, as
/// one-based line numbers.
/// An attribute is read from `#[` or `#![` to its balanced close, newlines
/// included, so a body spanning lines is one site.
/// A body nested inside `cfg_attr` is read the same way.
fn suppressions(text: &str) -> Vec<usize> {
    let chars: Vec<char> = text.chars().collect();
    let mut lines = Vec::new();
    let mut index = 0usize;
    let mut line = 1usize;
    while index < chars.len() {
        if let Some(next) = trivia_end(&chars, index) {
            line += chars[index..next.min(chars.len())]
                .iter()
                .filter(|value| **value == '\n')
                .count();
            index = next;
            continue;
        }
        if chars[index] == '#'
            && let Some(body) = attribute_body(&chars, index)
            && let Some(close) = bracket_close(&chars, body)
        {
            if names_a_level(&code_only(&chars, body, close)) {
                lines.push(line);
            }
            line += chars[index..close]
                .iter()
                .filter(|value| **value == '\n')
                .count();
            index = close + 1;
            continue;
        }
        if chars[index] == '\n' {
            line += 1;
        }
        index += 1;
    }
    lines
}

fn manifest_sections(manifest: &str) -> Vec<(String, Vec<String>)> {
    let mut sections: Vec<(String, Vec<String>)> = Vec::new();
    for line in manifest.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('[')
            && let Some(name) = rest.strip_suffix(']')
        {
            sections.push((name.trim().to_owned(), Vec::new()));
            continue;
        }
        if let Some(section) = sections.last_mut() {
            section.1.push(trimmed.to_owned());
        }
    }
    sections
}

fn lowered_to_allow(entry: &str) -> Option<String> {
    let (key, value) = entry.split_once('=')?;
    if value.contains("\"allow\"") || value.contains("'allow'") {
        return Some(key.trim().to_owned());
    }
    None
}

/// Every `[lints]` entry in a `Cargo.toml` whose level is `allow`.
fn allowances(manifest: &str) -> Vec<String> {
    manifest_sections(manifest)
        .into_iter()
        .filter(|(heading, _)| heading.split('.').any(|segment| segment == "lints"))
        .flat_map(|(heading, entries)| {
            entries.into_iter().filter_map(move |entry| {
                lowered_to_allow(&entry).map(|key| format!("{heading}.{key}"))
            })
        })
        .collect()
}

/// No lint attribute stands anywhere in the tree.
#[test]
fn every_lint_stands_at_the_level_the_toolchain_sets() {
    let multiline = "#[expect(\n    clippy::too_many_arguments,\n    reason = \"a body over several lines is one site\"\n)]\nfn wide() {}\n";
    assert_eq!(suppressions(multiline), vec![1]);

    let nested =
        "#[cfg_attr(\n    target_arch = \"wasm32\",\n    allow(dead_code)\n)]\nstruct Only;\n";
    assert_eq!(suppressions(nested), vec![1]);

    let crate_level = "#![allow(clippy::all)]\n#![expect(dead_code, reason = \"none\")]\n";
    assert_eq!(suppressions(crate_level), vec![1, 2]);

    let innocent = "// #[allow(dead_code)]\nconst QUOTED: &str = \"#[allow(dead_code)]\";\n#[derive(Debug)]\n#[deny(clippy::allow_attributes)]\nstruct Clean;\n";
    assert!(suppressions(innocent).is_empty());

    let root = workspace_root();
    let mut sites = Vec::new();
    for path in sources(&root) {
        let text = std::fs::read_to_string(&path).expect("the source is readable");
        for line in suppressions(&text) {
            let shown = path.strip_prefix(&root).unwrap_or(&path).display();
            sites.push(format!("{shown}:{line}"));
        }
    }
    assert!(sites.is_empty(), "lint attributes in tree: {sites:?}");
}

/// No `[lints]` table in any manifest lowers a lint to `allow`.
#[test]
fn no_manifest_allows_a_lint() {
    let lowered = "[lints.clippy]\ntoo_many_arguments = \"allow\"\n";
    assert_eq!(allowances(lowered), vec!["lints.clippy.too_many_arguments"]);

    let root = workspace_root();
    let mut sites = Vec::new();
    for path in named_under(&root, "Cargo.toml") {
        let manifest = std::fs::read_to_string(&path).expect("the manifest is readable");
        for key in allowances(&manifest) {
            let shown = path.strip_prefix(&root).unwrap_or(&path).display();
            sites.push(format!("{shown}: {key}"));
        }
    }
    assert!(sites.is_empty(), "manifests lowering a lint: {sites:?}");
}

/// The `disallowed-methods` entries `/CLAUDE.md` names, which stay; an entry
/// joining them raises strictness and is welcome.
const FORCED: [&str; 9] = [
    "core::result::Result::ok",
    "core::result::Result::unwrap_or",
    "core::result::Result::unwrap_or_default",
    "core::result::Result::unwrap_or_else",
    "serde_json::from_slice",
    "serde_json::from_str",
    "serde_json::from_value",
    "serde_json::to_string",
    "serde_json::to_value",
];

/// Every method path a clippy configuration disallows.
fn disallowed(configuration: &str) -> BTreeSet<&str> {
    configuration
        .split("path =")
        .skip(1)
        .filter_map(|entry| entry.split('"').nth(1))
        .collect()
}

/// True when `line` passes a flag that lowers a lint level: `--cap-lints`,
/// `--allow`, or `-A` attached to its lint or detached from it.
fn lowers_a_lint(line: &str) -> bool {
    if line.contains("--cap-lints") || line.contains("--allow") {
        return true;
    }
    let chars: Vec<char> = line.chars().collect();
    (0..chars.len().saturating_sub(1)).any(|at| {
        chars[at] == '-'
            && chars[at + 1] == 'A'
            && !at
                .checked_sub(1)
                .is_some_and(|before| chars[before].is_alphanumeric() || chars[before] == '-')
    })
}

/// `jellium-web/clippy.toml` is the only clippy configuration in the tree, its
/// only key is `disallowed-methods`, and the nine entries the policy names are
/// among the entries it holds.
#[test]
fn clippy_configuration_only_adds_obligations() {
    assert_eq!(
        disallowed("{ path = \"core::mem::swap\", reason = \"no\" }"),
        BTreeSet::from(["core::mem::swap"])
    );

    let root = workspace_root();
    let configurations = named_under(&root, "clippy.toml");
    let shown: Vec<String> = configurations
        .iter()
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap_or(path)
                .display()
                .to_string()
        })
        .collect();
    assert_eq!(shown, vec!["jellium-web/clippy.toml".to_owned()]);

    let text = std::fs::read_to_string(&configurations[0]).expect("the configuration is readable");
    let keys: BTreeSet<&str> = text
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, _)| key.trim())
        .filter(|key| !key.is_empty() && !key.starts_with('{'))
        .collect();
    assert_eq!(keys, BTreeSet::from(["disallowed-methods"]));
    assert!(disallowed(&text).is_superset(&BTreeSet::from(FORCED)));
}

/// Neither the justfile, nor any workflow, nor any cargo configuration passes
/// `-A`, `--allow` or `--cap-lints` to a compiler or to clippy.
#[test]
fn no_command_line_lowers_a_lint() {
    assert!(lowers_a_lint("    cargo clippy -- -Adead_code"));
    assert!(lowers_a_lint("    cargo clippy -- -A dead_code"));
    assert!(lowers_a_lint("rustflags = [\"-A\", \"dead_code\"]"));
    assert!(!lowers_a_lint(
        "    cargo clippy --all-targets -- -D warnings"
    ));
    assert!(!lowers_a_lint(
        "        run: Compress-Archive -Path release"
    ));

    let root = workspace_root();
    let mut commands = vec![root.join("justfile")];
    commands.extend(files_under(&root.join(".github"), "yml"));
    commands.extend(files_under(&root.join(".github"), "yaml"));
    commands.extend(named_under(&root, "config.toml"));

    let mut sites = Vec::new();
    for path in commands {
        let text = std::fs::read_to_string(&path).expect("the command file is readable");
        for (number, line) in text.lines().enumerate() {
            if lowers_a_lint(line) {
                let shown = path.strip_prefix(&root).unwrap_or(&path).display();
                sites.push(format!("{shown}:{}", number + 1));
            }
        }
    }
    assert!(sites.is_empty(), "command lines lowering a lint: {sites:?}");
}

/// Every site in `jellium-web/src` outside `failure.rs` that names `spelling`,
/// as repository-relative `path:line`.
fn spelled_outside_the_door(spelling: &str) -> Vec<String> {
    let root = workspace_root();
    let door = root.join("jellium-web/src/failure.rs");
    let mut sites = Vec::new();
    for path in sources(&root.join("jellium-web/src")) {
        if path == door {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("the source is readable");
        for (number, line) in text.lines().enumerate() {
            if line.contains(spelling) {
                let shown = path.strip_prefix(&root).unwrap_or(&path).display();
                sites.push(format!("{shown}:{}", number + 1));
            }
        }
    }
    sites
}

/// `jellium-web/src/failure.rs` holds every deserializer `jellium-web`
/// constructs, so the doors `/CLAUDE.md` names cannot be spelled around.
#[test]
fn only_the_doors_deserialize() {
    let sites = spelled_outside_the_door("Deserializer::from_");
    assert!(
        sites.is_empty(),
        "deserializers outside the doors: {sites:?}"
    );
}

/// `jellium-web/src/failure.rs` holds every woff2 decode `jellium-web` makes.
#[test]
fn the_woff2_decoder_is_named_only_behind_its_door() {
    let sites = spelled_outside_the_door("convert_woff2_to_ttf");
    assert!(
        sites.is_empty(),
        "woff2 decodes outside the door: {sites:?}"
    );
}
