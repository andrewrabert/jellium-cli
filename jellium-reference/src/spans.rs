//! `reference/spans`, read.

use crate::register::Construct;
use crate::tree::Extension;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

/// The text of every span the register records, committed under
/// `reference/spans` so a row is measured with no checkout in hand.
#[derive(Debug)]
pub struct Spans {
    held: BTreeMap<Construct, String>,
}

impl Spans {
    /// One entry per `.txt` file of `reference/spans`, keyed by the construct
    /// its stem names, holding the file's bytes unchanged.
    pub fn read(root: &Path) -> Result<Spans, Unread> {
        let directory = root.join("reference/spans");
        let Ok(entries) = std::fs::read_dir(&directory) else {
            return Err(Unread::Directory { path: directory });
        };
        let mut held = BTreeMap::new();
        for entry in entries {
            let Ok(entry) = entry else {
                return Err(Unread::Directory { path: directory });
            };
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some(Extension::TEXT.as_str()) {
                continue;
            }
            let named = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(Construct::read);
            let Some(construct) = named else {
                return Err(Unread::Unnamed { path });
            };
            let Ok(text) = std::fs::read_to_string(&path) else {
                return Err(Unread::File { path });
            };
            held.insert(construct, text);
        }
        Ok(Spans { held })
    }

    pub fn text(&self, construct: &Construct) -> Option<&str> {
        self.held.get(construct).map(String::as_str)
    }

    pub fn constructs(&self) -> BTreeSet<&Construct> {
        self.held.keys().collect()
    }
}

/// What refuses `reference/spans`.
#[derive(Debug)]
pub enum Unread {
    Directory { path: PathBuf },
    File { path: PathBuf },
    Unnamed { path: PathBuf },
}

impl fmt::Display for Unread {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unread::Directory { path } => {
                write!(formatter, "{} does not read", path.display())
            }
            Unread::File { path } => {
                write!(formatter, "{} does not read as text", path.display())
            }
            Unread::Unnamed { path } => {
                write!(formatter, "{} is named for no construct", path.display())
            }
        }
    }
}

impl std::error::Error for Unread {}
