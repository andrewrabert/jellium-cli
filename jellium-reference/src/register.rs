//! `reference/provenance.tsv`, read.

use sha2::Sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

/// The name a `// reference: <construct>` comment cites and one row carries.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Construct(String);

impl Construct {
    /// A name of lowercase letters, digits and hyphens, and None for any other
    /// text.
    pub fn read(text: &str) -> Option<Construct> {
        let named = !text.is_empty()
            && text
                .chars()
                .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == '-');
        named.then(|| Construct(text.to_owned()))
    }

    /// The name a `// reference:` comment carries, read up to the first
    /// character no construct carries.
    pub fn cited(line: &str) -> Option<Construct> {
        let rest = line.split_once("// reference:")?.1.trim_start();
        let name: String = rest
            .chars()
            .take_while(|value| {
                value.is_ascii_lowercase() || value.is_ascii_digit() || *value == '-'
            })
            .collect();
        Construct::read(&name)
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

/// A construct this client reaches, or one the reference reaches and this
/// client never observes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ported,
    Dead,
}

impl Kind {
    pub fn read(word: &str) -> Option<Kind> {
        match word {
            "ported" => Some(Kind::Ported),
            "dead" => Some(Kind::Dead),
            _ => None,
        }
    }
}

/// The lines of one file of the pinned reference a construct was taken from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    path: PathBuf,
    first: usize,
    last: usize,
}

impl Span {
    /// A one-based line pair that ascends, and None for any other text.
    pub fn read(path: &str, first: &str, last: &str) -> Option<Span> {
        let first: usize = first.parse().ok()?;
        let last: usize = last.parse().ok()?;
        if path.is_empty() || first < 1 || first > last {
            return None;
        }
        Some(Span {
            path: PathBuf::from(path),
            first,
            last,
        })
    }

    /// How many lines the span covers.
    pub fn lines(&self) -> usize {
        self.last - self.first + 1
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}-{}",
            self.path.display(),
            self.first,
            self.last
        )
    }
}

/// A sha256 digest as the register writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest(String);

impl Digest {
    /// Sixty-four hexadecimal digits, and None for any other text.
    pub fn read(text: &str) -> Option<Digest> {
        let digits = text.len() == 64 && text.chars().all(|value| value.is_ascii_hexdigit());
        digits.then(|| Digest(text.to_owned()))
    }

    pub fn of(bytes: &[u8]) -> Digest {
        Digest(format!("{:x}", <Sha256 as sha2::Digest>::digest(bytes)))
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One row of `reference/provenance.tsv`.
#[derive(Debug, Clone)]
pub struct Row {
    pub construct: Construct,
    pub span: Span,
    pub digest: Digest,
    pub kind: Kind,
}

/// `reference/provenance.tsv`, parsed.
#[derive(Debug)]
pub struct Register {
    rows: Vec<Row>,
}

/// The columns the register opens with.
const HEADER: &str = "construct\tpath\tfirst\tlast\tsha256\tkind";

/// How many fields one row holds.
const FIELDS: usize = 6;

impl Register {
    pub fn read(root: &Path) -> Result<Register, Malformed> {
        let path = root.join("reference/provenance.tsv");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Err(Malformed::Unreadable { path });
        };
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) {
            return Err(Malformed::Header);
        }

        let mut rows: Vec<Row> = Vec::new();
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
            let Some(construct) = Construct::read(fields[0]) else {
                return Err(Malformed::Named { line: number });
            };
            let Some(span) = Span::read(fields[1], fields[2], fields[3]) else {
                return Err(Malformed::Span { line: number });
            };
            let Some(digest) = Digest::read(fields[4]) else {
                return Err(Malformed::Digest { line: number });
            };
            let Some(kind) = Kind::read(fields[5]) else {
                return Err(Malformed::Kind { line: number });
            };
            if let Some(first) = seen.insert(construct.clone(), number) {
                return Err(Malformed::Twice {
                    construct,
                    first,
                    again: number,
                });
            }
            rows.push(Row {
                construct,
                span,
                digest,
                kind,
            });
        }
        Ok(Register { rows })
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn row(&self, construct: &Construct) -> Option<&Row> {
        self.rows.iter().find(|row| &row.construct == construct)
    }

    /// Every construct of one kind.
    pub fn constructs(&self, kind: Kind) -> BTreeSet<&Construct> {
        self.rows
            .iter()
            .filter(|row| row.kind == kind)
            .map(|row| &row.construct)
            .collect()
    }
}

/// What refuses `reference/provenance.tsv`, naming the row it refuses.
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
    Span {
        line: usize,
    },
    Digest {
        line: usize,
    },
    Kind {
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
                "reference/provenance.tsv does not open with its header"
            ),
            Malformed::Fields { line, held } => write!(
                formatter,
                "reference/provenance.tsv:{line} holds {held} fields, and a row holds {FIELDS}"
            ),
            Malformed::Named { line } => write!(
                formatter,
                "reference/provenance.tsv:{line} names no construct"
            ),
            Malformed::Span { line } => write!(
                formatter,
                "reference/provenance.tsv:{line} spans lines that do not ascend"
            ),
            Malformed::Digest { line } => write!(
                formatter,
                "reference/provenance.tsv:{line} has no sha256 digest"
            ),
            Malformed::Kind { line } => write!(
                formatter,
                "reference/provenance.tsv:{line} is neither ported nor dead"
            ),
            Malformed::Twice {
                construct,
                first,
                again,
            } => write!(
                formatter,
                "reference/provenance.tsv names {construct} twice, at {first} and {again}"
            ),
        }
    }
}

impl std::error::Error for Malformed {}

#[cfg(test)]
mod tests {
    use super::{Construct, Digest, Span};

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
    fn a_span_that_does_not_ascend_is_refused() {
        assert_eq!(Span::read("src/scripts/browser.js", "9", "3"), None);
        assert_eq!(Span::read("src/scripts/browser.js", "0", "3"), None);
        assert_eq!(
            Span::read("src/scripts/browser.js", "3", "9").map(|span| span.lines()),
            Some(7)
        );
    }

    #[test]
    fn a_field_of_sixty_four_hexadecimal_digits_is_a_digest_and_nothing_else_is() {
        let empty = Digest::of(b"");
        assert_eq!(
            empty.to_string(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(Digest::read(&empty.to_string()), Some(empty));
        assert_eq!(Digest::read("abc"), None);
        assert_eq!(Digest::read(&"g".repeat(64)), None);
    }
}
