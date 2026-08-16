//! The register that closes over what this client ported from jellyfin-web.
//!
//! Every ported construct is one row of `reference/provenance.tsv` cited by a
//! `// reference: <construct>` comment, every construct the reference reaches
//! and this client never observes is a `dead` row cited by nothing, and no
//! construct is both. A row whose cited lines no longer digest to the recorded
//! hash is a failure, so the pinned reference cannot move under the port
//! without saying so.

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// A construct this client reaches, or one the reference reaches and this
/// client never observes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Ported,
    Dead,
}

impl Kind {
    fn read(word: &str) -> Option<Kind> {
        match word {
            "ported" => Some(Kind::Ported),
            "dead" => Some(Kind::Dead),
            _ => None,
        }
    }
}

/// Every row of `reference/provenance.tsv`, parsed.
#[derive(Debug, Clone)]
struct Row {
    construct: String,
    path: PathBuf,
    first: usize,
    last: usize,
    sha256: String,
    kind: Kind,
}

const HEADER: &str = "construct\tpath\tfirst\tlast\tsha256\tkind";

/// The root a path names when it is not read from the jellyfin-web checkout.
const BUNDLED: &str = "jellyfin-apiclient:";

/// The entry of the bundle's `sourcesContent` a `BUNDLED` path names.
const SOURCE: &str = "src/apiClient.js";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package directory sits inside the workspace")
        .to_path_buf()
}

fn ignored(name: &str) -> bool {
    matches!(name, "target" | ".git" | "vendor" | "dist" | "reference")
}

fn files_under(root: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = match std::fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
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
            } else if path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| extensions.contains(&value))
            {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// The construct a `// reference: <construct>` comment names, reading the name
/// up to the first character no construct name carries.
fn construct_at(line: &str) -> Option<String> {
    let rest = line.split_once("// reference:")?.1.trim_start();
    let name: String = rest
        .chars()
        .take_while(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || *value == '-')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

/// Every construct cited by a `// reference: <construct>` comment in the tree.
fn cited(root: &Path) -> BTreeSet<String> {
    let itself = root.join("jellium-cli/tests/provenance.rs");
    let mut sources = files_under(root, &["rs"]);
    sources.extend(files_under(&root.join("jellium-web/js"), &["js"]));
    sources
        .into_iter()
        .filter(|path| path != &itself)
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .flat_map(|text| {
            text.lines()
                .filter_map(construct_at)
                .collect::<Vec<String>>()
        })
        .collect()
}

/// The table, refusing a malformed row, a duplicate construct and a line span
/// that is not ascending.
fn table(root: &Path) -> Vec<Row> {
    let path = root.join("reference/provenance.tsv");
    let text = std::fs::read_to_string(&path).expect("reference/provenance.tsv is readable");
    let mut lines = text.lines();
    assert_eq!(
        lines.next(),
        Some(HEADER),
        "reference/provenance.tsv does not open with its header"
    );

    let mut rows: Vec<Row> = Vec::new();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for (offset, line) in lines.enumerate() {
        let number = offset + 2;
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(
            fields.len(),
            6,
            "reference/provenance.tsv:{number} holds {} fields, and a row holds six",
            fields.len()
        );
        let first: usize = fields[2]
            .parse()
            .unwrap_or_else(|_| panic!("reference/provenance.tsv:{number} has no first line"));
        let last: usize = fields[3]
            .parse()
            .unwrap_or_else(|_| panic!("reference/provenance.tsv:{number} has no last line"));
        assert!(
            first >= 1 && first <= last,
            "reference/provenance.tsv:{number} spans {first}-{last}, which does not ascend"
        );
        assert!(
            fields[4].len() == 64 && fields[4].chars().all(|value| value.is_ascii_hexdigit()),
            "reference/provenance.tsv:{number} has no sha256 digest"
        );
        let kind = Kind::read(fields[5]).unwrap_or_else(|| {
            panic!("reference/provenance.tsv:{number} is neither ported nor dead")
        });
        if let Some(earlier) = seen.insert(fields[0].to_owned(), number) {
            panic!(
                "reference/provenance.tsv names {} twice, at {earlier} and {number}",
                fields[0]
            );
        }
        rows.push(Row {
            construct: fields[0].to_owned(),
            path: PathBuf::from(fields[1]),
            first,
            last,
            sha256: fields[4].to_owned(),
            kind,
        });
    }
    rows
}

/// The checkout named by `JELLYFIN_WEB_REFERENCE`, and None when it is unset.
fn checkout() -> Option<PathBuf> {
    let named = std::env::var("JELLYFIN_WEB_REFERENCE").ok()?;
    if named.is_empty() {
        return None;
    }
    Some(PathBuf::from(named))
}

/// `src/apiClient.js`, read out of the `sourcesContent` of the source map
/// under `JELLYFIN_APICLIENT_REFERENCE`, and None when it is unset.
fn apiclient() -> Option<String> {
    let named = std::env::var("JELLYFIN_APICLIENT_REFERENCE").ok()?;
    if named.is_empty() {
        return None;
    }
    let map = PathBuf::from(named).join("dist/jellyfin-apiclient.js.map");
    let text =
        std::fs::read_to_string(&map).unwrap_or_else(|_| panic!("{} is readable", map.display()));
    let document: serde_json::Value =
        serde_json::from_str(&text).expect("the source map is an object");
    let sources = document["sources"]
        .as_array()
        .expect("the source map names its sources");
    let at = sources
        .iter()
        .position(|source| {
            source
                .as_str()
                .is_some_and(|source| source.ends_with(SOURCE))
        })
        .unwrap_or_else(|| panic!("{} carries no {SOURCE}", map.display()));
    Some(
        document["sourcesContent"][at]
            .as_str()
            .expect("the source map carries the source")
            .to_owned(),
    )
}

fn rows_of(root: &Path, kind: Kind) -> BTreeSet<String> {
    table(root)
        .into_iter()
        .filter(|row| row.kind == kind)
        .map(|row| row.construct)
        .collect()
}

#[test]
fn every_cited_construct_has_exactly_one_ported_row() {
    assert_eq!(
        construct_at("    // reference: detect-browser — browser.js:245-346").as_deref(),
        Some("detect-browser")
    );
    assert_eq!(construct_at("    // a plain comment"), None);

    let root = workspace_root();
    let ported = rows_of(&root, Kind::Ported);
    let missing: Vec<String> = cited(&root)
        .into_iter()
        .filter(|construct| !ported.contains(construct))
        .collect();
    assert!(
        missing.is_empty(),
        "cited with no ported row in reference/provenance.tsv: {missing:?}"
    );
}

#[test]
fn every_ported_row_is_cited() {
    let root = workspace_root();
    let cited = cited(&root);
    let uncited: Vec<String> = rows_of(&root, Kind::Ported)
        .into_iter()
        .filter(|construct| !cited.contains(construct))
        .collect();
    assert!(
        uncited.is_empty(),
        "ported rows no comment cites: {uncited:?}"
    );
}

#[test]
fn no_dead_row_is_cited() {
    let root = workspace_root();
    let cited = cited(&root);
    let reached: Vec<String> = rows_of(&root, Kind::Dead)
        .into_iter()
        .filter(|construct| cited.contains(construct))
        .collect();
    assert!(reached.is_empty(), "dead rows a comment cites: {reached:?}");
}

#[test]
fn every_row_still_digests_to_what_the_pinned_reference_holds() {
    let root = workspace_root();
    let rows = table(&root);
    assert!(
        !rows.is_empty(),
        "reference/provenance.tsv records no construct"
    );

    let Some(checkout) = checkout() else {
        return;
    };

    let bundled = apiclient();
    let mut drifted = Vec::new();
    for row in rows {
        let named = row.path.to_str().expect("the path is text");
        let text = match named.strip_prefix(BUNDLED) {
            Some(_) => match bundled.clone() {
                Some(text) => text,
                None => {
                    drifted.push(format!(
                        "{} names {named} and JELLYFIN_APICLIENT_REFERENCE is unset",
                        row.construct
                    ));
                    continue;
                }
            },
            None => {
                let source = checkout.join(&row.path);
                std::fs::read_to_string(&source)
                    .unwrap_or_else(|_| panic!("{} is readable", source.display()))
            }
        };
        let lines: Vec<&str> = text.split('\n').collect();
        assert!(
            row.last <= lines.len(),
            "{} is shorter than line {}",
            row.path.display(),
            row.last
        );
        let span = lines[row.first - 1..row.last].join("\n");
        let digest = format!("{:x}", Sha256::digest(span.as_bytes()));
        if digest != row.sha256 {
            drifted.push(format!(
                "{} ({}:{}-{}) records {} and digests to {digest}",
                row.construct,
                row.path.display(),
                row.first,
                row.last,
                row.sha256
            ));
        }
    }
    assert!(
        drifted.is_empty(),
        "rows the pinned reference no longer holds: {drifted:#?}"
    );
}
