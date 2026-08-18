//! What `jellium-web/src` says it draws, read out of its own source.

use std::collections::BTreeMap;
use std::path::Path;

use crate::construct::{Construct, Page, Role, Sentence};
use crate::tree::{self, Extension};

/// The name of one variant of `jellium-web/src/text.rs`'s `Text`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variant(String);

impl Variant {
    /// A name under a leading capital carrying letters and digits alone, and
    /// None for any other text.
    pub fn read(text: &str) -> Option<Variant> {
        let named = text.starts_with(|value: char| value.is_ascii_uppercase())
            && text.chars().all(|value| value.is_ascii_alphanumeric());
        named.then(|| Variant(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One construct one module of `jellium-web/src` draws, as one call of a door
/// of `jellium-web/src/construct.rs` names it.
#[derive(Debug, Clone)]
pub struct Drawn {
    pub at: String,
    pub construct: Construct,
    pub role: Role,
    /// The `Text` variant the door carries, and None for a door that carries
    /// none.
    pub said: Option<Variant>,
}

/// What one module of `jellium-web/src` names itself as.
#[derive(Debug, Clone)]
pub enum Names {
    /// The reference pages its own `DRAWS` names.
    Pages(Vec<Page>),
    /// This client's own constructs alone, each a row of
    /// `reference/exemptions.tsv`.
    Own(Vec<Construct>),
    /// Nothing of its own: every construct it draws is named by the module that
    /// calls it.
    Caller,
}

/// One module of `jellium-web/src` that builds an `Element`.
#[derive(Debug, Clone)]
pub struct Site {
    pub at: String,
    pub names: Names,
    pub drawn: Vec<Drawn>,
}

/// The four doors of `jellium-web/src/construct.rs` a construct is drawn
/// through, and the role each stands for.
const DOORS: [(&str, Role); 3] = [
    ("navigation", Role::Navigation),
    ("stated", Role::Stated),
    ("silent", Role::Silent),
];

/// The door this client's own constructs are drawn through, which names no
/// reference construct and so carries no reference role of its own.
const OWN: &str = "own";

/// The module that declares the doors, which names every construct in
/// declaring them and draws none.
const DOORWAY: &str = "jellium-web/src/construct.rs";

/// What a module writing its own `DRAWS` calls it.
const DRAWS: &str = "DRAWS";

/// Every module of `jellium-web/src` holding a function that answers an
/// `Element`, and what each names itself as.
/// A module naming neither a page nor an own construct answers `Names::Caller`
/// only where it names no construct at all.
pub fn sites(root: &Path) -> Vec<Site> {
    let client = root.join("jellium-web").join("src");
    if !client.is_dir() {
        return Vec::new();
    }
    tree::files_under(&client, &[Extension::RUST])
        .into_iter()
        .filter_map(|path| {
            let at = under(root, &path);
            if at == DOORWAY {
                return None;
            }
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{at} does not read: {error}"));
            if !text.contains("-> Element<") {
                return None;
            }
            let drawn = drawn(&at, &text);
            let own = own(&text);
            let pages = pages(&text, root);
            let names = if pages.is_empty() && own.is_empty() {
                Names::Caller
            } else if pages.is_empty() {
                Names::Own(own)
            } else {
                Names::Pages(pages)
            };
            Some(Site { at, names, drawn })
        })
        .collect()
}

/// The reference string key each `Text` variant stands for, read out of
/// `jellium-web/src/text.rs`'s own `text!` list.
pub fn wording(root: &Path) -> BTreeMap<Variant, Option<Sentence>> {
    let path = root.join("jellium-web").join("src").join("text.rs");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return BTreeMap::new();
    };
    let Some(list) = listed(&text) else {
        return BTreeMap::new();
    };
    let mut held = BTreeMap::new();
    let mut rest = list;
    while let Some(at) = rest.find("=>") {
        let (head, tail) = rest.split_at(at);
        let entry = match tail[2..].find("=>") {
            Some(next) => &tail[2..2 + next],
            None => &tail[2..],
        };
        rest = &tail[2..];
        let Some(variant) = last_word(head).and_then(|word| Variant::read(&word)) else {
            continue;
        };
        let key = entry
            .split("Sentence::")
            .nth(1)
            .and_then(named)
            .and_then(|word| Sentence::read(&word));
        held.insert(variant, key);
    }
    held
}

/// The `text!` list's own body.
fn listed(text: &str) -> Option<&str> {
    let at = text.rfind("text! {")? + "text! {".len();
    let end = text[at..].rfind('}')? + at;
    Some(&text[at..end])
}

/// What a module writes where it draws every page of the reference rather than
/// a list of its own.
const EVERY: &str = "ALL";

/// Where the generated vocabulary declares every page.
const VOCABULARY: &str = "jellium-model/src/construct.rs";

/// The reference pages a module's own `DRAWS` names, `Page::ALL` naming every
/// page the generated vocabulary declares.
fn pages(text: &str, root: &Path) -> Vec<Page> {
    let Some(at) = text.find(DRAWS) else {
        return Vec::new();
    };
    let Some(end) = text[at..].find(';') else {
        return Vec::new();
    };
    let named: Vec<String> = text[at..at + end]
        .split("Page::")
        .skip(1)
        .filter_map(named)
        .collect();
    if named.iter().any(|word| word == EVERY) {
        return every(root);
    }
    named
        .into_iter()
        .filter_map(|word| Page::named(&word))
        .collect()
}

/// Every page the generated vocabulary declares.
fn every(root: &Path) -> Vec<Page> {
    let Ok(text) = std::fs::read_to_string(root.join(VOCABULARY)) else {
        return Vec::new();
    };
    let Some(at) = text.find("pub enum Page {") else {
        return Vec::new();
    };
    let Some(end) = text[at..].find('}') else {
        return Vec::new();
    };
    text[at..at + end]
        .lines()
        .skip(1)
        .filter_map(|line| named(line.trim()))
        .filter_map(|word| Page::named(&word))
        .collect()
}

/// This client's own constructs a module draws, one per call of the `own` door.
fn own(text: &str) -> Vec<Construct> {
    calls(text, OWN)
        .into_iter()
        .filter_map(|args| args.first()?.split("Own::").nth(1).and_then(named))
        .filter_map(|word| Construct::named(&word))
        .collect()
}

/// One row per call of a door that names a construct of the reference.
fn drawn(at: &str, text: &str) -> Vec<Drawn> {
    let mut held: Vec<Drawn> = DOORS
        .iter()
        .flat_map(|(door, role)| {
            calls(text, door).into_iter().filter_map(move |args| {
                let construct = Construct::named(args.first()?.strip_prefix("Construct::")?)?;
                Some(Drawn {
                    at: at.to_owned(),
                    construct,
                    role: *role,
                    said: args.get(1).and_then(|word| said(word)),
                })
            })
        })
        .collect();
    held.extend(own(text).into_iter().map(|construct| Drawn {
        at: at.to_owned(),
        construct,
        role: Role::Silent,
        said: None,
    }));
    held
}

/// The `Text` variant one argument carries, whether the door takes it bare or
/// under an `Option`.
fn said(word: &str) -> Option<Variant> {
    Variant::read(word.split("Text::").nth(1).and_then(named)?.as_str())
}

/// Every call of `door` in the text, as its arguments split at the commas
/// standing outside a bracket.
fn calls(text: &str, door: &str) -> Vec<Vec<String>> {
    let mut held = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find(door) {
        let (head, tail) = rest.split_at(at);
        rest = &tail[door.len()..];
        if head.ends_with(|value: char| value.is_ascii_alphanumeric() || value == '_') {
            continue;
        }
        if !rest.starts_with('(') {
            continue;
        }
        let Some(end) = through(rest) else {
            continue;
        };
        held.push(split(&rest[1..end - 1]));
        rest = &rest[end..];
    }
    held
}

/// Where the bracket opening the text shuts, one past its own.
fn through(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (at, value) in text.char_indices() {
        match value {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// One argument list split at the commas standing outside a bracket.
fn split(text: &str) -> Vec<String> {
    let mut held = Vec::new();
    let mut depth = 0usize;
    let mut word = String::new();
    for value in text.chars() {
        match value {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                held.push(word.trim().to_owned());
                word = String::new();
                continue;
            }
            _ => {}
        }
        word.push(value);
    }
    let last = word.trim();
    if !last.is_empty() {
        held.push(last.to_owned());
    }
    held
}

/// The name standing at the head of the text, read up to the first character no
/// name carries.
fn named(text: &str) -> Option<String> {
    let held: String = text
        .chars()
        .take_while(|value| value.is_ascii_alphanumeric() || *value == '_')
        .collect();
    (!held.is_empty()).then_some(held)
}

/// The name standing at the end of the text, which is the variant a `=>` gives
/// its key.
fn last_word(text: &str) -> Option<String> {
    let held: String = text
        .chars()
        .rev()
        .skip_while(|value| value.is_whitespace())
        .take_while(|value| value.is_ascii_alphanumeric() || *value == '_')
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();
    (!held.is_empty()).then_some(held)
}

/// One path as the tree names it, which is the path under the workspace root.
fn under(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
