//! `reference/exemptions.tsv`, read.

use crate::construct::Construct;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

/// Which of the two kinds an exemption is. There is no third.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A construct the reference draws that this client cannot draw.
    Loss,
    /// A construct this client draws that the reference has no counterpart
    /// for.
    Own,
}

impl Kind {
    pub fn read(word: &str) -> Option<Kind> {
        match word {
            "loss" => Some(Kind::Loss),
            "own" => Some(Kind::Own),
            _ => None,
        }
    }
}

/// The document a row rests on: the ADR that accepted a loss, or the
/// requirement that introduced an own construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cited(String);

impl Cited {
    /// A path to a document, and None for text naming none.
    pub fn read(text: &str) -> Option<Cited> {
        let named = !text.trim().is_empty() && text.trim() == text && text.ends_with(".md");
        named.then(|| Cited(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Cited {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// What a construct is for. An own-construct row asserts that no construct of
/// the reference serves this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Purpose(String);

impl Purpose {
    /// A sentence stating what the construct serves, and None for text that
    /// states none.
    pub fn read(text: &str) -> Option<Purpose> {
        let stated = !text.trim().is_empty() && text.trim() == text;
        stated.then(|| Purpose(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Purpose {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One row of `reference/exemptions.tsv`.
#[derive(Debug, Clone)]
pub struct Exemption {
    pub kind: Kind,
    pub construct: Construct,
    pub cited: Cited,
    pub serves: Purpose,
}

/// `reference/exemptions.tsv`, parsed.
#[derive(Debug)]
pub struct Exemptions {
    rows: Vec<Exemption>,
}

/// The columns the table opens with.
const HEADER: &str = "kind\tconstruct\tcites\tserves";

/// How many fields one row holds.
const FIELDS: usize = 4;

impl Exemptions {
    pub fn read(root: &Path) -> Result<Exemptions, Malformed> {
        let path = root.join("reference/exemptions.tsv");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Err(Malformed::Unreadable { path });
        };
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) {
            return Err(Malformed::Header);
        }

        let mut rows: Vec<Exemption> = Vec::new();
        let mut seen: BTreeMap<Construct, usize> = BTreeMap::new();
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
            let Some(kind) = Kind::read(fields[0]) else {
                return Err(Malformed::Kinded { line: number });
            };
            let Some(construct) = Construct::read(fields[1]) else {
                return Err(Malformed::Named { line: number });
            };
            let Some(cited) = Cited::read(fields[2]) else {
                return Err(Malformed::Uncited { line: number });
            };
            let Some(serves) = Purpose::read(fields[3]) else {
                return Err(Malformed::Purposeless { line: number });
            };
            if let Some(first) = seen.insert(construct.clone(), number) {
                return Err(Malformed::Twice {
                    construct,
                    first,
                    again: number,
                });
            }
            rows.push(Exemption {
                kind,
                construct,
                cited,
                serves,
            });
        }
        Ok(Exemptions { rows })
    }

    pub fn rows(&self) -> &[Exemption] {
        &self.rows
    }

    /// Every construct the reference draws that this client does not.
    pub fn losses(&self) -> BTreeSet<&Construct> {
        self.of(Kind::Loss)
    }

    /// Every construct of this client's own.
    pub fn own(&self) -> BTreeSet<&Construct> {
        self.of(Kind::Own)
    }

    fn of(&self, kind: Kind) -> BTreeSet<&Construct> {
        self.rows
            .iter()
            .filter(|row| row.kind == kind)
            .map(|row| &row.construct)
            .collect()
    }
}

/// What refuses `reference/exemptions.tsv`, naming the row it refuses.
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
    Kinded {
        line: usize,
    },
    Uncited {
        line: usize,
    },
    Purposeless {
        line: usize,
    },
    Twice {
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
                "reference/exemptions.tsv does not open with its header"
            ),
            Malformed::Fields { line, held } => write!(
                formatter,
                "reference/exemptions.tsv:{line} holds {held} fields, and a row holds {FIELDS}"
            ),
            Malformed::Named { line } => write!(
                formatter,
                "reference/exemptions.tsv:{line} names no construct"
            ),
            Malformed::Kinded { line } => {
                write!(formatter, "reference/exemptions.tsv:{line} names no kind")
            }
            Malformed::Uncited { line } => write!(
                formatter,
                "reference/exemptions.tsv:{line} cites no document"
            ),
            Malformed::Purposeless { line } => write!(
                formatter,
                "reference/exemptions.tsv:{line} states no purpose"
            ),
            Malformed::Twice {
                construct,
                first,
                again,
            } => write!(
                formatter,
                "reference/exemptions.tsv names {construct} twice, at {first} and {again}"
            ),
        }
    }
}

impl std::error::Error for Malformed {}

#[cfg(test)]
mod tests {
    use super::{Cited, Exemptions, Kind, Purpose};
    use std::path::Path;

    #[test]
    fn a_citation_names_a_document_and_untrimmed_or_unsuffixed_text_names_none() {
        assert_eq!(
            Cited::read("adr/0050-the-references-motion-is-an-accepted-loss.md")
                .as_ref()
                .map(Cited::as_str),
            Some("adr/0050-the-references-motion-is-an-accepted-loss.md")
        );
        assert_eq!(Cited::read(""), None);
        assert_eq!(Cited::read("  adr/0050.md"), None);
        assert_eq!(Cited::read("adr/0050"), None);
    }

    #[test]
    fn a_purpose_is_stated_text_and_blank_text_states_none() {
        assert_eq!(
            Purpose::read("the mark a press leaves under the pointer")
                .as_ref()
                .map(Purpose::as_str),
            Some("the mark a press leaves under the pointer")
        );
        assert_eq!(Purpose::read(""), None);
        assert_eq!(Purpose::read("   "), None);
    }

    #[test]
    fn the_table_divides_its_rows_into_losses_and_own_constructs() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the workspace holds this package");
        let exemptions = Exemptions::read(root).expect("reference/exemptions.tsv reads");
        assert!(exemptions.rows().iter().any(|row| row.kind == Kind::Own));
        assert_eq!(
            exemptions.losses().len() + exemptions.own().len(),
            exemptions.rows().len()
        );
    }
}
