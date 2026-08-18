//! The gate over `jellium-model/src/appearance`: every value it holds names the
//! rule it came from, and the rule it names carries that value.

use jellium_reference::register::{Construct, Kind, Register};
use jellium_reference::spans::Spans;
use jellium_reference::tree::{self, Extension};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

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
    /// The module a reader qualifies the value with, which is the declaring
    /// file's stem and what tells `scheme::SECONDARY` from
    /// `typeface::SECONDARY`.
    module: String,
    at: String,
    written: String,
    initializer: String,
    measures: Vec<Measure>,
    cited: Vec<Construct>,
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
        let unit = !self.measures.is_empty()
            && self
                .measures
                .iter()
                .all(|measure| matches!(measure, Measure::Identity));
        unit.then_some(Bucket::Identity)
    }

    /// Whether the value is built from other constants, and so inherits their
    /// citations; a written number carries no name, however its digits are
    /// grouped.
    fn derived(&self) -> bool {
        self.initializer
            .split(|value: char| !value.is_alphanumeric() && value != '_')
            .any(|word| {
                word.len() > 1
                    && word.contains('_')
                    && word.chars().any(|value| value.is_ascii_uppercase())
                    && word.chars().all(|value| {
                        value.is_ascii_uppercase() || value.is_ascii_digit() || value == '_'
                    })
            })
    }
}

fn appearance(root: &Path) -> Vec<Value> {
    let directory = root.join("jellium-model/src/appearance");
    let files = tree::files_under(&directory, &[Extension::RUST]);

    let mut values = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("the module is readable");
        let lines: Vec<&str> = text.lines().collect();
        let module = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("the module is a named rust file")
            .to_owned();
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
                if let Some(construct) = Construct::cited(comment) {
                    cited.push(construct);
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
                module: module.clone(),
                at: format!("{named}:{}", index + 1),
                written,
                measures: measures(&initializer),
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

fn register(root: &Path) -> Register {
    Register::read(root).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn every_appearance_value_carries_a_provenance_row() {
    let root = tree::workspace_root();
    let register = register(&root);
    let ported = register.constructs(Kind::Ported);
    let values = appearance(&root);
    assert!(
        values.iter().any(|value| value.exported),
        "no value was read out of jellium-model/src/appearance"
    );

    let mut uncited = Vec::new();
    let mut unknown = Vec::new();
    for value in &values {
        for construct in &value.cited {
            if !ported.contains(construct) {
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

/// Whether a character stands inside a name.
fn inside(value: char) -> bool {
    value.is_alphanumeric() || value == '_'
}

/// Whether `text` names `value`: an occurrence of its name bounded by
/// characters no name carries, standing after neither the `pub const` that
/// declares it nor a path into one of `modules` other than its own.
fn names(text: &str, modules: &BTreeSet<&str>, value: &Value) -> bool {
    text.match_indices(&value.name).any(|(at, found)| {
        let before = text[..at].chars().next_back();
        let after = text[at + found.len()..].chars().next();
        if before.is_some_and(inside) || after.is_some_and(inside) {
            return false;
        }
        let head = text[..at].trim_end();
        if head.ends_with("const") {
            return false;
        }
        let Some(path) = head.strip_suffix("::") else {
            return true;
        };
        let qualifier: String = path
            .chars()
            .rev()
            .take_while(|value| inside(*value))
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect();
        !modules.contains(qualifier.as_str()) || qualifier == value.module
    })
}

/// Every exported appearance value is named by something other than its own
/// declaration.
/// rustc's `dead_code` never reaches an exported item, so without this a ported
/// value nothing draws stands in the tree unseen.
/// A mention qualified by another appearance module names that module's value
/// of the same name and not this one.
/// The guard package is not read, so the names this file spells count for
/// nothing.
#[test]
fn every_exported_appearance_value_is_read() {
    let root = tree::workspace_root();
    let values = appearance(&root);
    assert!(
        values.iter().any(|value| value.exported),
        "no value was read out of jellium-model/src/appearance"
    );
    let modules: BTreeSet<&str> = values.iter().map(|value| value.module.as_str()).collect();

    let guard = tree::guard(&root);
    let sources = tree::files_under(&root, &[Extension::RUST]);
    assert!(!sources.is_empty(), "no source was read out of the tree");
    let texts: Vec<String> = sources
        .into_iter()
        .filter(|path| !path.starts_with(&guard))
        .map(|path| {
            std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} does not read: {error}", path.display()))
        })
        .collect();

    let unread: Vec<String> = values
        .iter()
        .filter(|value| value.exported)
        .filter(|value| !texts.iter().any(|text| names(text, &modules, value)))
        .map(|value| format!("{} ({})", value.at, value.name))
        .collect();
    assert!(
        unread.is_empty(),
        "exported appearance values nothing reads: {unread:#?}"
    );
}

/// Whether two characters standing side by side belong to one value, which is
/// what a run of spacing is kept for and what a spelling must not be found
/// inside.
fn joined(before: char, after: char) -> bool {
    let value = |held: char| held.is_ascii_alphanumeric() || held == '_' || held == '#';
    let decimal =
        (before == '.' && after.is_ascii_digit()) || (before.is_ascii_digit() && after == '.');
    (value(before) && value(after)) || decimal
}

/// Text as a css declaration and a Rust literal can be compared in: lowered,
/// its leading zeros gone, and its spacing gone wherever the spacing parts two
/// characters that do not join.
fn flattened(text: &str) -> String {
    let lowered: Vec<char> = text.chars().flat_map(char::to_lowercase).collect();
    let mut kept: Vec<char> = Vec::with_capacity(lowered.len());
    for (index, value) in lowered.iter().enumerate() {
        let leading = *value == '0'
            && lowered.get(index + 1) == Some(&'.')
            && !index
                .checked_sub(1)
                .and_then(|before| lowered.get(before))
                .is_some_and(|before| before.is_ascii_digit() || *before == '.');
        if !leading {
            kept.push(*value);
        }
    }

    let mut written = String::with_capacity(kept.len());
    let mut at = 0;
    while at < kept.len() {
        let value = kept[at];
        if !value.is_whitespace() {
            written.push(value);
            at += 1;
            continue;
        }
        let mut end = at;
        while end < kept.len() && kept[end].is_whitespace() {
            end += 1;
        }
        let spanning = at
            .checked_sub(1)
            .and_then(|before| kept.get(before))
            .zip(kept.get(end))
            .is_some_and(|(before, after)| joined(*before, *after));
        if spanning {
            written.push(' ');
        }
        at = end;
    }
    written
}

/// The unit a css declaration writes a count in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Em,
    Rem,
    Percent,
    Pixels,
    ViewportWidth,
    ViewportHeight,
    /// No unit at all, which is what a line height and a MUI measure are
    /// written as.
    Bare,
}

impl Unit {
    fn read(suffix: &str) -> Option<Unit> {
        match suffix {
            "em" => Some(Unit::Em),
            "rem" => Some(Unit::Rem),
            "%" => Some(Unit::Percent),
            "px" => Some(Unit::Pixels),
            "vw" => Some(Unit::ViewportWidth),
            "vh" => Some(Unit::ViewportHeight),
            "" => Some(Unit::Bare),
            _ => None,
        }
    }
}

/// A count as a declaration carries it: the decimals that name it, and the
/// unit it stands in.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Count {
    written: String,
    unit: Unit,
}

impl Count {
    fn of(count: f64, unit: Unit) -> Count {
        Count {
            written: trimmed(count),
            unit,
        }
    }
}

/// A form a css declaration is allowed to carry a value in.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Spelling {
    /// A count in its unit, which a declaration carries by writing that count
    /// in that unit to the same decimals.
    Measured(Count),
    /// A color in the shortest form css writes it, which a declaration carries
    /// by writing it with no value character on either side.
    Color(String),
}

/// One value an initializer writes.
#[derive(Debug)]
enum Measure {
    /// A value taken from the reference, named as css writes it, with every
    /// spelling a declaration may carry it in.
    Spelt {
        named: String,
        spellings: Vec<Spelling>,
    },
    /// The unit's own zero or whole, which is the unit rather than anything
    /// taken from the reference.
    Identity,
}

/// A span of the pinned reference: its text flattened, and every count it
/// writes.
struct Span {
    text: String,
    counts: Vec<Count>,
}

impl Span {
    fn of(text: &str) -> Span {
        let text = flattened(text);
        Span {
            counts: numbered(&text),
            text,
        }
    }

    /// Whether this writes the spelling as a value of its own.
    fn carries(&self, spelling: &Spelling) -> bool {
        match spelling {
            Spelling::Measured(count) => self.counts.contains(count),
            Spelling::Color(written) => self
                .text
                .match_indices(written)
                .any(|(at, found)| self.bounded(at, found)),
        }
    }

    /// Whether the text found at `at` stands with no value character on either
    /// side of it.
    fn bounded(&self, at: usize, found: &str) -> bool {
        let opening = found.chars().next();
        let closing = found.chars().next_back();
        let before = self.text[..at].chars().next_back();
        let after = self.text[at + found.len()..].chars().next();
        !before
            .zip(opening)
            .is_some_and(|(before, opening)| joined(before, opening))
            && !closing
                .zip(after)
                .is_some_and(|(closing, after)| joined(closing, after))
    }
}

/// Every count a flattened span writes: a count opens where a digit, a `.` or
/// a `-` stands beside a character it does not join, runs through the digits
/// and the `.`, and takes the `%` or the letters that follow as its unit.
fn numbered(text: &str) -> Vec<Count> {
    let held: Vec<char> = text.chars().collect();
    let mut counts = Vec::new();
    let mut at = 0;
    while at < held.len() {
        let value = held[at];
        let opens = (value.is_ascii_digit() || value == '.' || value == '-')
            && at
                .checked_sub(1)
                .is_none_or(|before| !joined(held[before], value));
        if !opens {
            at += 1;
            continue;
        }
        let mut end = at + usize::from(value == '-');
        while end < held.len() && (held[end].is_ascii_digit() || held[end] == '.') {
            end += 1;
        }
        let mut suffix = end;
        if held.get(suffix) == Some(&'%') {
            suffix += 1;
        } else {
            while suffix < held.len() && held[suffix].is_ascii_alphabetic() {
                suffix += 1;
            }
        }
        let written: String = held[at..end].iter().collect();
        let unit: String = held[end..suffix].iter().collect();
        if let Ok(count) = written.parse::<f64>()
            && let Some(unit) = Unit::read(&unit)
        {
            counts.push(Count::of(count, unit));
        }
        at = suffix.max(at + 1);
    }
    counts
}

/// A number as css writes it: the fewest decimals that carry it.
fn trimmed(count: f64) -> String {
    format!("{count:.4}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

/// Every value an initializer writes, in the order it writes them; a
/// constructor no arm names writes none, so a value written in one reaches no
/// bucket of its own.
fn measures(initializer: &str) -> Vec<Measure> {
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
        rest = after;
        // every argument is read with its digit separators gone, so
        // `Length::em(1.669_565_2)` is a count and not a name
        let number = arguments.replace('_', "").parse::<f64>();
        let measure = match (name.as_str(), call) {
            // a design length is root-relative, the canvas applying the root
            // once for the whole surface, so a rule written in rem spells the
            // same value a rule written in em does
            ("length", "em") => {
                let Ok(em) = number else { continue };
                if em == 0.0 || em == 1.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: format!("{}em", trimmed(em)),
                        spellings: vec![
                            Spelling::Measured(Count::of(em, Unit::Em)),
                            Spelling::Measured(Count::of(em, Unit::Rem)),
                            Spelling::Measured(Count::of(em * 100.0, Unit::Percent)),
                        ],
                    }
                }
            }
            ("breakpoint", "em") => {
                let Ok(em) = number else { continue };
                Measure::Spelt {
                    named: format!("{}em", trimmed(em)),
                    spellings: vec![
                        Spelling::Measured(Count::of(em, Unit::Em)),
                        Spelling::Measured(Count::of(em, Unit::Rem)),
                    ],
                }
            }
            ("breakpoint", "pixels") => {
                let Ok(px) = number else { continue };
                Measure::Spelt {
                    named: format!("{}px", trimmed(px)),
                    spellings: vec![Spelling::Measured(Count::of(px, Unit::Pixels))],
                }
            }
            ("css", "of") => {
                let Ok(px) = number else { continue };
                if px == 0.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: format!("{}px", trimmed(px)),
                        spellings: vec![Spelling::Measured(Count::of(px, Unit::Pixels))],
                    }
                }
            }
            ("css", "unitless") => {
                let Ok(px) = number else { continue };
                if px == 0.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: trimmed(px),
                        spellings: vec![Spelling::Measured(Count::of(px, Unit::Bare))],
                    }
                }
            }
            ("columns", "twelfths") => {
                let Ok(count) = number else { continue };
                Measure::Spelt {
                    named: trimmed(count),
                    spellings: vec![Spelling::Measured(Count::of(count, Unit::Bare))],
                }
            }
            // a canvas length is not a css value: its zero is the unit, and the
            // reference writes no other
            ("drawn", "of") => {
                let Ok(count) = number else { continue };
                if count != 0.0 {
                    continue;
                }
                Measure::Identity
            }
            ("share", "per_ten_thousand") => {
                let Ok(share) = number else { continue };
                if share == 0.0 || share == 10_000.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: format!("{}%", trimmed(share / 100.0)),
                        spellings: vec![
                            Spelling::Measured(Count::of(share / 100.0, Unit::Percent)),
                            Spelling::Measured(Count::of(share / 100.0, Unit::ViewportWidth)),
                        ],
                    }
                }
            }
            ("share", "units") => {
                let Ok(count) = number else { continue };
                if count == 0.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: format!("{}vw", trimmed(count)),
                        spellings: vec![
                            Spelling::Measured(Count::of(count, Unit::ViewportWidth)),
                            Spelling::Measured(Count::of(count, Unit::ViewportHeight)),
                        ],
                    }
                }
            }
            // a ratio is a multiple of the lettering the rule is written in,
            // which css writes bare, in the element's own em, or as a
            // percentage
            ("ratio", "thousandths") => {
                let Ok(thousandths) = number else { continue };
                if thousandths == 0.0 || thousandths == 1000.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: trimmed(thousandths / 1000.0),
                        spellings: vec![
                            Spelling::Measured(Count::of(thousandths / 1000.0, Unit::Bare)),
                            Spelling::Measured(Count::of(thousandths / 1000.0, Unit::Em)),
                            Spelling::Measured(Count::of(thousandths / 10.0, Unit::Percent)),
                        ],
                    }
                }
            }
            ("alpha", "thousandths") => {
                let Ok(thousandths) = number else { continue };
                if thousandths == 0.0 || thousandths == 1000.0 {
                    Measure::Identity
                } else {
                    Measure::Spelt {
                        named: trimmed(thousandths / 1000.0),
                        spellings: vec![Spelling::Measured(Count::of(
                            thousandths / 1000.0,
                            Unit::Bare,
                        ))],
                    }
                }
            }
            ("color", "rgb" | "rgba") => {
                let Some(color) = color(arguments) else {
                    continue;
                };
                Measure::Spelt {
                    named: color.clone(),
                    spellings: vec![Spelling::Color(flattened(&color))],
                }
            }
            _ => continue,
        };
        found.push(measure);
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

/// The gate over the reading this file does before it measures anything.
#[test]
fn a_span_carries_a_value_only_where_it_writes_it() {
    assert_eq!(flattened("padding: 0.4em .25em"), "padding:.4em.25em");
    assert_eq!(
        flattened("0 0 0.29em rgba(0, 0, 0, 0.37)"),
        "0 0 .29em rgba(0,0,0,.37)"
    );

    let mui = Span::of("paddingTop: 0.08, paddingBottom: 128, minWidth: 8px");
    assert!(!mui.carries(&Spelling::Measured(Count::of(8.0, Unit::Bare))));
    assert!(mui.carries(&Spelling::Measured(Count::of(8.0, Unit::Pixels))));

    let sheet = Span::of("color: #ffffff; font-size: 1.66956521739130434em");
    assert!(!sheet.carries(&Spelling::Color("#fff".to_owned())));
    assert!(sheet.carries(&Spelling::Measured(Count::of(1.669_565_2, Unit::Em))));

    assert!(matches!(
        measures("Length::em(0.6)").as_slice(),
        [Measure::Spelt { named, .. }] if named == "0.6em"
    ));
    assert!(matches!(
        measures("Length::em(1.0)").as_slice(),
        [Measure::Identity]
    ));
    assert!(matches!(
        measures("Alpha::thousandths(0)").as_slice(),
        [Measure::Identity]
    ));
}

#[test]
fn every_cited_span_holds_the_value_that_cites_it() {
    let root = tree::workspace_root();
    let register = register(&root);
    let committed = Spans::read(&root).unwrap_or_else(|error| panic!("{error}"));
    let mut spans: BTreeMap<Construct, Span> = BTreeMap::new();
    let mut absent = Vec::new();
    let mut measured = 0usize;

    for value in &appearance(&root) {
        let Some(bucket) = value.bucket() else {
            absent.push(format!("{} ({}) falls in no bucket", value.at, value.name));
            continue;
        };
        if bucket == Bucket::Machinery || value.derived() || value.cited.is_empty() {
            continue;
        }
        for construct in &value.cited {
            if spans.contains_key(construct) {
                continue;
            }
            let row = register
                .row(construct)
                .unwrap_or_else(|| panic!("{construct} has a ported row"));
            let text = committed
                .text(&row.construct)
                .unwrap_or_else(|| panic!("{construct} stands under a reference/spans file"));
            spans.insert(construct.clone(), Span::of(text));
        }
        for measure in &value.measures {
            let Measure::Spelt { named, spellings } = measure else {
                continue;
            };
            let carried = spellings.iter().any(|spelling| {
                value
                    .cited
                    .iter()
                    .any(|construct| spans[construct].carries(spelling))
            });
            measured += 1;
            if !carried {
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
    assert!(
        measured > 0,
        "no appearance value was measured against a cited span"
    );
}

/// The four spellings a text size, a line box and a gap reach iced through,
/// named rather than matched on a word boundary, so this gate's scope is its
/// own.
const GAPS: &[&str] = &[".size(", ".line_height(", ".spacing(", ".padding("];

/// The argument a call opens at `at` begins with, with the whitespace and the
/// line breaks before it gone.
fn argument(text: &str, at: usize) -> Option<char> {
    text[at..].chars().find(|value| !value.is_whitespace())
}

#[test]
fn no_literal_reaches_a_text_size_or_a_gap() {
    let root = tree::workspace_root();
    let directory = root.join("jellium-web/src");
    let files = tree::files_under(&directory, &[Extension::RUST]);
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

/// The seven spellings a drawn length reaches iced through, named rather than
/// matched on a word boundary, so `max_width` is inside this gate's scope
/// rather than caught by the `width` that ends it.
const LENGTHS: &[&str] = &[
    ".width(",
    ".max_width(",
    ".scroller_width(",
    ".height(",
    ".max_height(",
    ".radius(",
    ".margin(",
];

#[test]
fn no_literal_reaches_a_drawn_length() {
    let root = tree::workspace_root();
    let directory = root.join("jellium-web/src");
    let files = tree::files_under(&directory, &[Extension::RUST]);
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
    let root = tree::workspace_root();
    let directory = root.join("jellium-web/src");
    let files = tree::files_under(&directory, &[Extension::RUST]);
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
