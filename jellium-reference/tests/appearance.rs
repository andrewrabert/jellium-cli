//! The gate over `jellium-model/src/appearance`: every value it holds names the
//! rule it came from, and the rule it names carries that value.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package directory sits inside the workspace")
        .to_path_buf()
}

/// The oracle a value is allowed to cite in place of a ported rule.
const ORACLE: &str = "reference/breakpoints.tsv";

/// The standards a value of the measurement system's own machinery is allowed
/// to rest on. A value enters that bucket only by someone adding its standard
/// here, which is what keeps the bucket from meaning "not otherwise
/// classifiable".
const STANDARDS: &[&str] = &["ieee-754", "css-initial-font-size"];

/// The types a bare scalar is written in. Every ported appearance value carries
/// a measurement type instead, so a number taken from a stylesheet cannot reach
/// the machinery bucket without first shedding its type.
const SCALARS: &[&str] = &["f32", "f64", "u8", "u16", "u32", "u64", "usize", "i32"];

/// A value that is its unit's identity or its zero, written as its own
/// initializer: the unit rather than anything taken from the reference.
const UNITS: &[&str] = &[
    "Length::em(0.0)",
    "Length::em(1.0)",
    "Drawn::of(0.0)",
    "Css::of(0.0)",
    "Share::per_ten_thousand(0)",
    "Share::per_ten_thousand(10_000)",
    "Alpha::thousandths(1000)",
    "Ratio::thousandths(1000)",
];

fn identity_or_zero(initializer: &str) -> bool {
    UNITS.iter().any(|unit| initializer.trim() == *unit)
}

/// Which of the four buckets a value falls in. A value falling in none of them
/// is what the residue check fails on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bucket {
    /// It cites a ported row or the oracle.
    Cited,
    /// It is built from constants that cite.
    Derived,
    /// It is its unit's identity or its zero.
    Identity,
    /// It is the measurement system's own machinery, resting on a standard
    /// outside the reference.
    Machinery,
}

/// One `const` item of the appearance module.
#[derive(Debug)]
struct Value {
    name: String,
    at: String,
    written: String,
    initializer: String,
    cited: Vec<String>,
    oracle: bool,
    standard: Option<String>,
    exported: bool,
}

impl Value {
    /// The bucket this falls in, and None where the four do not account for it.
    fn bucket(&self) -> Option<Bucket> {
        if let Some(standard) = &self.standard {
            let named = STANDARDS.contains(&standard.as_str());
            let bare = SCALARS.contains(&self.written.as_str());
            return (named && bare).then_some(Bucket::Machinery);
        }
        if !self.cited.is_empty() || self.oracle {
            return Some(Bucket::Cited);
        }
        if self.derived() {
            return Some(Bucket::Derived);
        }
        identity_or_zero(&self.initializer).then_some(Bucket::Identity)
    }

    /// Whether the value is built from other constants, and so inherits their
    /// citations.
    fn derived(&self) -> bool {
        self.initializer
            .split(|value: char| !value.is_alphanumeric() && value != '_')
            .any(|word| {
                word.len() > 1
                    && word.contains('_')
                    && word.chars().all(|value| {
                        value.is_ascii_uppercase() || value.is_ascii_digit() || value == '_'
                    })
                    && word != "ZERO"
                    && word != "WHOLE"
                    && word != "OPAQUE"
            })
    }
}

fn appearance(root: &Path) -> Vec<Value> {
    let directory = root.join("jellium-model/src/appearance");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&directory)
        .expect("jellium-model/src/appearance is readable")
        .map(|entry| entry.expect("the entry is readable").path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("rs"))
        .collect();
    files.sort();

    let mut values = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("the module is readable");
        let lines: Vec<&str> = text.lines().collect();
        let named = file
            .strip_prefix(root)
            .expect("the module sits under the workspace")
            .display()
            .to_string();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let exported = trimmed.starts_with("pub const ");
            if !(exported || trimmed.starts_with("const ")) {
                continue;
            }
            let declaration = trimmed
                .trim_start_matches("pub ")
                .trim_start_matches("const ");
            if declaration.starts_with("fn ") {
                continue;
            }
            let Some((name, typed)) = declaration.split_once(':') else {
                continue;
            };
            let written = typed
                .split_once('=')
                .map_or(typed, |(written, _)| written)
                .trim()
                .to_owned();

            let mut initializer = String::new();
            let mut at = index;
            loop {
                initializer.push_str(lines[at].trim());
                if lines[at].trim_end().ends_with(';') {
                    break;
                }
                at += 1;
                assert!(at < lines.len(), "{named}:{} never ends", index + 1);
            }
            let initializer = initializer
                .split_once('=')
                .expect("a const declares its value")
                .1
                .trim_end_matches(';')
                .trim()
                .to_owned();

            let mut cited = Vec::new();
            let mut oracle = false;
            let mut standard = None;
            let mut above = index;
            while above > 0 && lines[above - 1].trim_start().starts_with("//") {
                above -= 1;
                let comment = lines[above].trim_start();
                if let Some(rest) = comment.strip_prefix("// reference:") {
                    cited.push(
                        rest.trim_start()
                            .chars()
                            .take_while(|value| {
                                value.is_ascii_lowercase()
                                    || value.is_ascii_digit()
                                    || *value == '-'
                            })
                            .collect(),
                    );
                }
                if let Some(rest) = comment.strip_prefix("// standard:") {
                    standard = Some(
                        rest.trim_start()
                            .chars()
                            .take_while(|value| {
                                value.is_ascii_lowercase()
                                    || value.is_ascii_digit()
                                    || *value == '-'
                            })
                            .collect(),
                    );
                }
                if comment
                    .strip_prefix("// oracle:")
                    .is_some_and(|rest| rest.contains(ORACLE))
                {
                    oracle = true;
                }
            }

            values.push(Value {
                name: name.trim().to_owned(),
                at: format!("{named}:{}", index + 1),
                written,
                initializer,
                cited,
                oracle,
                standard,
                exported,
            });
        }
    }
    values
}

/// The `construct` and span of every ported row of `reference/provenance.tsv`.
fn ported(root: &Path) -> BTreeMap<String, (PathBuf, usize, usize)> {
    let text = std::fs::read_to_string(root.join("reference/provenance.tsv"))
        .expect("reference/provenance.tsv is readable");
    text.lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.get(5) != Some(&"ported") {
                return None;
            }
            Some((
                fields[0].to_owned(),
                (
                    PathBuf::from(fields[1]),
                    fields[2].parse().ok()?,
                    fields[3].parse().ok()?,
                ),
            ))
        })
        .collect()
}

#[test]
fn every_appearance_value_carries_a_provenance_row() {
    let root = workspace_root();
    let rows = ported(&root);
    let values = appearance(&root);
    assert!(
        values.iter().any(|value| value.exported),
        "no value was read out of jellium-model/src/appearance"
    );

    let mut uncited = Vec::new();
    let mut unknown = Vec::new();
    for value in &values {
        for construct in &value.cited {
            if !rows.contains_key(construct) {
                unknown.push(format!(
                    "{} cites {construct}, which no ported row names",
                    value.at
                ));
            }
        }
        if value.bucket().is_none() {
            uncited.push(format!("{} ({})", value.at, value.name));
        }
    }
    assert!(unknown.is_empty(), "citations with no row: {unknown:#?}");
    assert!(
        uncited.is_empty(),
        "appearance values no bucket accounts for: {uncited:#?}"
    );
}

/// The checkout named by `JELLYFIN_WEB_REFERENCE`, and None where it is unset.
fn checkout() -> Option<PathBuf> {
    let named = std::env::var("JELLYFIN_WEB_REFERENCE").ok()?;
    if named.is_empty() {
        return None;
    }
    Some(PathBuf::from(named))
}

/// Text with its spacing and its leading zeros gone, which is the form a css
/// declaration and a Rust literal can be compared in.
fn flattened(text: &str) -> String {
    let packed: Vec<char> = text
        .chars()
        .filter(|value| !value.is_whitespace())
        .flat_map(|value| value.to_lowercase())
        .collect();
    let mut written = String::with_capacity(packed.len());
    for (index, value) in packed.iter().enumerate() {
        let dropped = *value == '0'
            && packed.get(index + 1) == Some(&'.')
            && !index
                .checked_sub(1)
                .and_then(|before| packed.get(before))
                .is_some_and(|before| before.is_ascii_digit() || *before == '.');
        if !dropped {
            written.push(*value);
        }
    }
    written
}

/// A number as css writes it: the fewest decimals that carry it.
fn trimmed(count: f64) -> String {
    format!("{count:.4}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

/// Every literal value an initializer takes, each with the spellings a css
/// declaration is allowed to carry it in. A value that is its unit's identity
/// or its zero yields no spelling, being the unit itself.
fn spellings(initializer: &str) -> Vec<(String, Vec<String>)> {
    let mut found = Vec::new();
    let flat = flattened(initializer);
    let mut rest = flat.as_str();
    while let Some(at) = rest.find("::") {
        let before = &rest[..at];
        let name: String = before
            .chars()
            .rev()
            .take_while(|value| value.is_alphanumeric() || *value == '_')
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect();
        let after = &rest[at + 2..];
        let Some((call, tail)) = after.split_once('(') else {
            break;
        };
        let Some(end) = closing(tail) else {
            rest = after;
            continue;
        };
        let arguments = &tail[..end];
        match (name.as_str(), call) {
            ("length", "em") | ("breakpoint", "em") => {
                if let Ok(em) = arguments.parse::<f64>()
                    && em != 0.0
                    && em != 1.0
                {
                    found.push((
                        format!("{}em", trimmed(em)),
                        vec![
                            format!("{}em", trimmed(em)),
                            format!("{}%", trimmed(em * 100.0)),
                        ],
                    ));
                }
            }
            ("breakpoint", "pixels") | ("css", "of") => {
                if let Ok(px) = arguments.parse::<f64>()
                    && px != 0.0
                {
                    found.push((
                        format!("{}px", trimmed(px)),
                        vec![format!("{}px", trimmed(px))],
                    ));
                }
            }
            ("share", "per_ten_thousand") => {
                if let Ok(share) = arguments.replace('_', "").parse::<f64>()
                    && share != 0.0
                    && share != 10_000.0
                {
                    let percent = trimmed(share / 100.0);
                    found.push((
                        format!("{percent}%"),
                        vec![format!("{percent}%"), format!("{percent}vw")],
                    ));
                }
            }
            ("share", "units") => {
                if let Ok(count) = arguments.parse::<f64>()
                    && count != 0.0
                {
                    let count = trimmed(count);
                    found.push((
                        format!("{count}vw"),
                        vec![format!("{count}vw"), format!("{count}vh")],
                    ));
                }
            }
            ("ratio", "thousandths") => {
                if let Ok(thousandths) = arguments.parse::<f64>()
                    && thousandths != 1000.0
                {
                    let factor = trimmed(thousandths / 1000.0);
                    found.push((
                        factor.clone(),
                        vec![factor, format!("{}%", trimmed(thousandths / 10.0))],
                    ));
                }
            }
            ("color", "rgb" | "rgba") => {
                if let Some(color) = color(arguments) {
                    found.push((color.clone(), vec![color]));
                }
            }
            _ => {}
        }
        rest = after;
    }
    found
}

/// The offset of the parenthesis that closes the one just opened.
fn closing(tail: &str) -> Option<usize> {
    let mut depth = 1usize;
    for (at, value) in tail.char_indices() {
        match value {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// A `Color::rgb` or `Color::rgba` argument list in its shortest css form.
fn color(arguments: &str) -> Option<String> {
    let fields: Vec<&str> = arguments.split(',').collect();
    let channels: Vec<u8> = fields
        .iter()
        .take(3)
        .map(|field| {
            let field = field.trim();
            match field.strip_prefix("0x") {
                Some(hex) => u8::from_str_radix(hex, 16).ok(),
                None => field.parse().ok(),
            }
        })
        .collect::<Option<Vec<u8>>>()?;
    if channels.len() != 3 {
        return None;
    }
    if let Some(alpha) = fields.get(3) {
        let thousandths: f64 = alpha
            .split_once("thousandths(")?
            .1
            .trim_end_matches(')')
            .parse()
            .ok()?;
        return Some(format!(
            "rgba({}, {}, {}, {})",
            channels[0],
            channels[1],
            channels[2],
            trimmed(thousandths / 1000.0)
        ));
    }
    if channels.iter().all(|channel| channel >> 4 == channel & 0xf) {
        let [red, green, blue] = [channels[0] & 0xf, channels[1] & 0xf, channels[2] & 0xf];
        return Some(format!("#{red:x}{green:x}{blue:x}"));
    }
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        channels[0], channels[1], channels[2]
    ))
}

#[test]
fn every_cited_span_holds_the_value_that_cites_it() {
    let root = workspace_root();
    assert_eq!(flattened("padding: 0.4em .25em"), "padding:.4em.25em");
    assert_eq!(
        spellings("Length::em(0.6)")
            .into_iter()
            .map(|(named, _)| named)
            .collect::<Vec<String>>(),
        vec!["0.6em".to_owned()]
    );
    assert!(spellings("Length::em(1.0)").is_empty());

    let Some(checkout) = checkout() else {
        return;
    };
    let rows = ported(&root);
    let mut spans: BTreeMap<String, String> = BTreeMap::new();
    let mut absent = Vec::new();

    for value in &appearance(&root) {
        let Some(bucket) = value.bucket() else {
            absent.push(format!("{} ({}) falls in no bucket", value.at, value.name));
            continue;
        };
        if bucket == Bucket::Machinery || value.derived() || value.cited.is_empty() {
            continue;
        }
        let mut held = String::new();
        for construct in &value.cited {
            let text = spans.entry(construct.clone()).or_insert_with(|| {
                let (path, first, last) = rows
                    .get(construct)
                    .unwrap_or_else(|| panic!("{construct} has a ported row"));
                let source = checkout.join(path);
                let text = std::fs::read_to_string(&source)
                    .unwrap_or_else(|_| panic!("{} is readable", source.display()));
                let lines: Vec<&str> = text.split('\n').collect();
                flattened(&lines[first - 1..*last].join("\n"))
            });
            held.push_str(text);
        }
        for (named, allowed) in spellings(&value.initializer) {
            if !allowed
                .iter()
                .any(|spelling| held.contains(&flattened(spelling)))
            {
                absent.push(format!(
                    "{} ({}) takes {named}, which {:?} does not carry",
                    value.at, value.name, value.cited
                ));
            }
        }
    }
    assert!(
        absent.is_empty(),
        "values whose cited spans do not carry them: {absent:#?}"
    );
}

/// The three spellings a text size and a gap reach iced through, named rather
/// than matched on a word boundary, so this gate's scope is its own.
const GAPS: &[&str] = &[".size(", ".spacing(", ".padding("];

/// Every `.rs` file under a directory, sorted.
fn sources(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![directory.to_path_buf()];
    while let Some(at) = pending.pop() {
        let entries = match std::fs::read_dir(&at) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// The argument a call opens at `at` begins with, with the whitespace and the
/// line breaks before it gone.
fn argument(text: &str, at: usize) -> Option<char> {
    text[at..].chars().find(|value| !value.is_whitespace())
}

#[test]
fn no_literal_reaches_a_text_size_or_a_gap() {
    let root = workspace_root();
    let directory = root.join("jellium-web/src");
    let files = sources(&directory);
    assert!(
        !files.is_empty(),
        "no source was read out of jellium-web/src"
    );

    let mut spelled = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("the source is readable");
        let named = file
            .strip_prefix(&root)
            .expect("the source sits under the workspace")
            .display()
            .to_string();
        for call in GAPS {
            let mut at = 0;
            while let Some(found) = text[at..].find(call) {
                let opened = at + found + call.len();
                at = opened;
                let Some(first) = argument(&text, opened) else {
                    continue;
                };
                if first.is_ascii_digit() || first == '-' {
                    let line = text[..opened].lines().count();
                    spelled.push(format!("{named}:{line} spells a number in {call}"));
                }
            }
        }
    }
    assert!(
        spelled.is_empty(),
        "numbers reaching a text size or a gap: {spelled:#?}"
    );
}

/// The six spellings a drawn length reaches iced through, named rather than
/// matched on a word boundary, so `max_width` is inside this gate's scope
/// rather than caught by the `width` that ends it.
const LENGTHS: &[&str] = &[
    ".width(",
    ".max_width(",
    ".height(",
    ".max_height(",
    ".radius(",
    ".margin(",
];

#[test]
fn no_literal_reaches_a_drawn_length() {
    let root = workspace_root();
    let directory = root.join("jellium-web/src");
    let files = sources(&directory);
    assert!(
        !files.is_empty(),
        "no source was read out of jellium-web/src"
    );

    let mut spelled = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("the source is readable");
        let named = file
            .strip_prefix(&root)
            .expect("the source sits under the workspace")
            .display()
            .to_string();
        for call in LENGTHS {
            let mut at = 0;
            while let Some(found) = text[at..].find(call) {
                let opened = at + found + call.len();
                at = opened;
                let Some(first) = argument(&text, opened) else {
                    continue;
                };
                if first.is_ascii_digit() || first == '-' {
                    let line = text[..opened].lines().count();
                    spelled.push(format!("{named}:{line} spells a number in {call}"));
                }
            }
        }
    }
    assert!(
        spelled.is_empty(),
        "numbers reaching a drawn length: {spelled:#?}"
    );
}

/// The call this gate reads, which is iced's button constructor and not a
/// function whose own name ends in it.
const CONTROL: &str = "button(";

/// The end of the method chain hanging off the call that opens at `at`, which
/// is where the statement holding that call closes.
fn chained(text: &str, at: usize) -> Option<usize> {
    let mut end = at + closing(&text[at..])? + 1;
    loop {
        let rest = &text[end..];
        let skipped = rest.len() - rest.trim_start().len();
        if !rest[skipped..].starts_with('.') {
            return Some(end);
        }
        let named = end + skipped + 1;
        let read: usize = text[named..]
            .chars()
            .take_while(|value| value.is_alphanumeric() || *value == '_')
            .map(char::len_utf8)
            .sum();
        if read == 0 {
            return Some(end);
        }
        let after = named + read;
        end = match text[after..].starts_with('(') {
            true => after + closing(&text[after + 1..])? + 2,
            false => after,
        };
    }
}

#[test]
fn every_button_carries_a_style() {
    let root = workspace_root();
    let directory = root.join("jellium-web/src");
    let files = sources(&directory);
    assert!(
        !files.is_empty(),
        "no source was read out of jellium-web/src"
    );

    let mut bare = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("the source is readable");
        let named = file
            .strip_prefix(&root)
            .expect("the source sits under the workspace")
            .display()
            .to_string();
        let mut at = 0;
        while let Some(found) = text[at..].find(CONTROL) {
            let opened = at + found;
            at = opened + CONTROL.len();
            let held = text[..opened]
                .chars()
                .next_back()
                .is_some_and(|value| value.is_alphanumeric() || value == '_');
            if held {
                continue;
            }
            let Some(end) = chained(&text, at) else {
                continue;
            };
            if !text[opened..end].contains(".style(") {
                let line = text[..opened].lines().count();
                bare.push(format!("{named}:{line} draws a button with no style"));
            }
        }
    }
    assert!(
        bare.is_empty(),
        "buttons drawing iced's own face: {bare:#?}"
    );
}
