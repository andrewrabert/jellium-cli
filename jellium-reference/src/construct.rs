//! The names both registers stand on, and `reference/constructs.tsv`, read.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

/// The name of one construct of the pinned reference: lowercase letters,
/// digits and hyphens.
/// Two registers spell names in this form and neither shares the other's
/// namespace. `reference/provenance.tsv` names the span a ported value was
/// taken from, chosen by whoever ported it and as often naming a rule as an
/// element — `page-side`, `scheme-header-transparent`.
/// `reference/constructs.tsv` names an element the reference's markup writes,
/// derived from that element's own class. The same element carries different
/// names in the two: `header-back` against `header-back-button`,
/// `section-title-button` against `section-title-text-button`, `tab-strip`
/// against `header-tabs`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Construct(String);

impl Construct {
    /// A name of lowercase letters, digits and hyphens, and None for any other
    /// text.
    pub fn read(text: &str) -> Option<Construct> {
        hyphenated(text).then(|| Construct(text.to_owned()))
    }

    /// The name a `// reference:` comment carries, read up to the first
    /// character no construct carries.
    pub fn cited(line: &str) -> Option<Construct> {
        Construct::read(&cited(line, "// reference:")?)
    }

    /// The name whose Pascal form is `variant`, and None for a variant that is
    /// not the Pascal form of any name.
    pub fn named(variant: &str) -> Option<Construct> {
        let name = Construct::read(&hyphenate(variant)?)?;
        (name.variant() == variant).then_some(name)
    }

    /// The Rust variant `jellium-model`'s own `Construct` gives this name:
    /// each hyphen-separated word capitalised, the hyphens gone.
    pub fn variant(&self) -> String {
        capitalize(&self.0)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Construct {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One page of the pinned reference, named by the route its own route table
/// gives it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page(String);

impl Page {
    /// A name of lowercase letters, digits and hyphens, and None for any other
    /// text.
    pub fn read(text: &str) -> Option<Page> {
        hyphenated(text).then(|| Page(text.to_owned()))
    }

    /// The page whose Pascal form is `variant`, and None for a variant that is
    /// not the Pascal form of any page.
    pub fn named(variant: &str) -> Option<Page> {
        let page = Page::read(&hyphenate(variant)?)?;
        (page.variant() == variant).then_some(page)
    }

    /// The Rust variant `jellium-model`'s own `Page` gives this name.
    pub fn variant(&self) -> String {
        capitalize(&self.0)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Page {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The word the table writes where a construct carries no string key.
const SILENT: &str = "silent";

/// One string key of the reference's own string table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sentence(String);

impl Sentence {
    /// A key of ASCII letters and digits, and None for `silent`, which names a
    /// construct carrying no sentence.
    pub fn read(text: &str) -> Option<Sentence> {
        let keyed = text != SILENT
            && !text.is_empty()
            && text.chars().all(|value| value.is_ascii_alphanumeric());
        keyed.then(|| Sentence(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// What the reference's markup makes of a construct, which is a fact its
/// markup states and never a judgement about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The reference wraps it in a link or a button.
    Navigation,
    /// The reference writes a string key inside it and wraps it in neither.
    Stated,
    /// The reference's markup gives it no string key.
    Silent,
}

impl Role {
    pub fn read(word: &str) -> Option<Role> {
        match word {
            "navigation" => Some(Role::Navigation),
            "stated" => Some(Role::Stated),
            SILENT => Some(Role::Silent),
            _ => None,
        }
    }

    /// The door of `jellium-web/src/construct.rs` that draws this role.
    pub fn door(self) -> &'static str {
        match self {
            Role::Navigation => "navigation",
            Role::Stated => "stated",
            Role::Silent => SILENT,
        }
    }
}

/// One row of `reference/constructs.tsv`.
#[derive(Debug, Clone)]
pub struct Expected {
    pub page: Page,
    pub construct: Construct,
    pub role: Role,
    pub sentence: Option<Sentence>,
}

/// One page's constructs, in the order the reference draws them.
#[derive(Debug, Clone)]
pub struct Constructs {
    page: Page,
    rows: Vec<Expected>,
}

impl Constructs {
    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn rows(&self) -> &[Expected] {
        &self.rows
    }
}

/// `reference/constructs.tsv`, parsed.
#[derive(Debug)]
pub struct Table {
    pages: Vec<Constructs>,
}

/// The columns the table opens with.
const HEADER: &str = "page\tconstruct\trole\tkey";

/// How many fields one row holds.
const FIELDS: usize = 4;

impl Table {
    pub fn read(root: &Path) -> Result<Table, Malformed> {
        let path = root.join("reference/constructs.tsv");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Err(Malformed::Unreadable { path });
        };
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) {
            return Err(Malformed::Header);
        }

        let mut pages: Vec<Constructs> = Vec::new();
        let mut seen: BTreeMap<(Page, Construct), usize> = BTreeMap::new();
        for (offset, line) in lines.enumerate() {
            let number = offset + 2;
            if line.is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() != FIELDS {
                return Err(Malformed::Fields {
                    line: number,
                    held: fields.len(),
                });
            }
            let Some(page) = Page::read(fields[0]) else {
                return Err(Malformed::Paged { line: number });
            };
            let Some(construct) = Construct::read(fields[1]) else {
                return Err(Malformed::Named { line: number });
            };
            let Some(role) = Role::read(fields[2]) else {
                return Err(Malformed::Roled { line: number });
            };
            let sentence = Sentence::read(fields[3]);
            if sentence.is_none() && fields[3] != SILENT {
                return Err(Malformed::Keyed { line: number });
            }
            if let Some(first) = seen.insert((page.clone(), construct.clone()), number) {
                return Err(Malformed::Twice {
                    page,
                    construct,
                    first,
                    again: number,
                });
            }
            let drawn = Expected {
                page: page.clone(),
                construct,
                role,
                sentence,
            };
            match pages.iter_mut().find(|held| held.page == page) {
                Some(held) => held.rows.push(drawn),
                None => pages.push(Constructs {
                    page,
                    rows: vec![drawn],
                }),
            }
        }
        Ok(Table { pages })
    }

    pub fn pages(&self) -> &[Constructs] {
        &self.pages
    }

    pub fn drawn(&self, page: &Page) -> Option<&Constructs> {
        self.pages.iter().find(|held| &held.page == page)
    }
}

/// What refuses `reference/constructs.tsv`, naming the row it refuses.
#[derive(Debug)]
pub enum Malformed {
    Unreadable {
        path: PathBuf,
    },
    Header,
    Fields {
        line: usize,
        held: usize,
    },
    Named {
        line: usize,
    },
    Paged {
        line: usize,
    },
    Roled {
        line: usize,
    },
    Keyed {
        line: usize,
    },
    Twice {
        page: Page,
        construct: Construct,
        first: usize,
        again: usize,
    },
}

impl fmt::Display for Malformed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Malformed::Unreadable { path } => {
                write!(formatter, "{} does not read", path.display())
            }
            Malformed::Header => write!(
                formatter,
                "reference/constructs.tsv does not open with its header"
            ),
            Malformed::Fields { line, held } => write!(
                formatter,
                "reference/constructs.tsv:{line} holds {held} fields, and a row holds {FIELDS}"
            ),
            Malformed::Named { line } => write!(
                formatter,
                "reference/constructs.tsv:{line} names no construct"
            ),
            Malformed::Paged { line } => {
                write!(formatter, "reference/constructs.tsv:{line} names no page")
            }
            Malformed::Roled { line } => {
                write!(formatter, "reference/constructs.tsv:{line} names no role")
            }
            Malformed::Keyed { line } => {
                write!(formatter, "reference/constructs.tsv:{line} names no key")
            }
            Malformed::Twice {
                page,
                construct,
                first,
                again,
            } => write!(
                formatter,
                "reference/constructs.tsv draws {construct} on {page} twice, at {first} and {again}"
            ),
        }
    }
}

impl std::error::Error for Malformed {}

/// Whether text is lowercase letters, digits and hyphens, and holds something.
fn hyphenated(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == '-')
}

/// The name a comment carries after `marker`, read up to the first character
/// no name carries.
fn cited(line: &str, marker: &str) -> Option<String> {
    let rest = line.split_once(marker)?.1.trim_start();
    Some(
        rest.chars()
            .take_while(|value| {
                value.is_ascii_lowercase() || value.is_ascii_digit() || *value == '-'
            })
            .collect(),
    )
}

/// Each hyphen-separated word of `name` capitalised, the hyphens gone.
fn capitalize(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut letters = word.chars();
            match letters.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + letters.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// `variant` broken before each capital and joined with hyphens, and None for
/// text that is not a Pascal form at all.
fn hyphenate(variant: &str) -> Option<String> {
    if !variant.starts_with(|value: char| value.is_ascii_uppercase()) {
        return None;
    }
    let mut name = String::new();
    for letter in variant.chars() {
        if !letter.is_ascii_alphanumeric() {
            return None;
        }
        if letter.is_ascii_uppercase() && !name.is_empty() {
            name.push('-');
        }
        name.push(letter.to_ascii_lowercase());
    }
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::{Construct, Page, Role, Sentence};

    #[test]
    fn a_comment_cites_the_name_up_to_the_first_character_no_construct_carries() {
        assert_eq!(
            Construct::cited("    // reference: detect-browser — browser.js:245-346")
                .as_ref()
                .map(Construct::as_str),
            Some("detect-browser")
        );
        assert_eq!(Construct::cited("    // a plain comment"), None);
    }

    #[test]
    fn a_name_and_its_pascal_form_answer_each_other() {
        let construct = Construct::read("header-back-button").expect("a hyphenated name");
        assert_eq!(construct.variant(), "HeaderBackButton");
        assert_eq!(Construct::named("HeaderBackButton"), Some(construct));
        assert_eq!(
            Page::named("LiveTv").as_ref().map(Page::as_str),
            Some("live-tv")
        );
        assert_eq!(Construct::named("headerTabs"), None);
        assert_eq!(Construct::named("Header_Tabs"), None);
    }

    #[test]
    fn the_silent_key_names_no_sentence() {
        assert_eq!(Sentence::read("silent"), None);
        assert_eq!(
            Sentence::read("HeaderMyMedia")
                .as_ref()
                .map(Sentence::as_str),
            Some("HeaderMyMedia")
        );
    }

    #[test]
    fn each_role_names_the_door_that_draws_it() {
        assert_eq!(Role::read("navigation").map(Role::door), Some("navigation"));
        assert_eq!(Role::read("stated").map(Role::door), Some("stated"));
        assert_eq!(Role::read("silent").map(Role::door), Some("silent"));
        assert_eq!(Role::read("quiet"), None);
    }
}
