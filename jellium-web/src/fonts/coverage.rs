//! The reference's `unicode-range` declarations: the base ranges the bundle's
//! own faces draw, and the ranges each face the origin serves declares.
//!
//! Both tables are read once, on the first miss. Every bound goes through
//! `failure::unraised::read::<Codepoint>`, whose `FromStr` reads base sixteen,
//! which is how both files write one. A row holding a bound that does not read
//! is dropped, and a parse that dropped any row raises its sentence once at its
//! end rather than once per row.

use std::sync::OnceLock;

use crate::error;
use crate::failure::{self, unraised};
use crate::style::typeface::Weight;
use crate::text::Text;

use super::{Codepoint, Family};

const BASE: &str = include_str!("../../fonts/embedded.tsv");

const ORIGIN: &str = include_str!("../../fonts/coverage.tsv");

/// One served face: the file the reference's rule points at, and the ranges
/// that rule declares.
pub struct Row {
    pub family: Family,
    pub weight: Weight,
    pub path: &'static str,
    pub ranges: Vec<(Codepoint, Codepoint)>,
}

/// The ranges the sixteen base faces draw, sorted and merged, so a lookup
/// bisects them.
pub fn embedded() -> &'static [(Codepoint, Codepoint)] {
    static RANGES: OnceLock<Vec<(Codepoint, Codepoint)>> = OnceLock::new();
    RANGES.get_or_init(|| {
        let read: Vec<Vec<(Codepoint, Codepoint)>> = BASE
            .lines()
            .filter_map(|line| ranges(line.split('\t').skip(2)))
            .collect();
        if read.len() < BASE.lines().count() {
            failure::raise(error::told(Text::FailureFontCoverage));
        }
        merged(read.into_iter().flatten().collect())
    })
}

/// The served faces, in the family order the reference's own font stack puts
/// them for `en-us`: HK, JP, KR, SC, TC.
pub fn rows() -> &'static [Row] {
    static ROWS: OnceLock<Vec<Row>> = OnceLock::new();
    ROWS.get_or_init(|| {
        let read: Vec<Row> = ORIGIN.lines().filter_map(row).collect();
        if read.len() < ORIGIN.lines().count() {
            failure::raise(error::told(Text::FailureFontCoverage));
        }
        read
    })
}

fn row(line: &'static str) -> Option<Row> {
    let mut fields = line.split('\t');
    let family = family(fields.next()?)?;
    let weight = weight(fields.next()?)?;
    let path = fields.next()?;
    Some(Row {
        family,
        weight,
        path,
        ranges: ranges(fields)?,
    })
}

/// The family whose own `@font-face` rule writes `name`.
fn family(name: &str) -> Option<Family> {
    [
        Family::HongKong,
        Family::Japanese,
        Family::Korean,
        Family::Simplified,
        Family::Traditional,
    ]
    .into_iter()
    .find(|family| family.name() == name)
}

/// The weight the reference's own file names spell as a hundreds figure.
fn weight(field: &str) -> Option<Weight> {
    match field {
        "400" => Some(Weight::Regular),
        "700" => Some(Weight::Bold),
        _ => None,
    }
}

/// The ranges `fields` writes as `start-end` in hexadecimal, and nothing where
/// any bound does not read.
fn ranges<'a>(fields: impl Iterator<Item = &'a str>) -> Option<Vec<(Codepoint, Codepoint)>> {
    fields
        .map(|field| {
            let (start, end) = field.split_once('-')?;
            let (Ok(start), Ok(end)) = (
                unraised::read::<Codepoint>(start),
                unraised::read::<Codepoint>(end),
            ) else {
                return None;
            };
            Some((start, end))
        })
        .collect()
}

/// The same codepoints, as the fewest ranges that hold them, so a lookup can
/// bisect. A range whose start is past its end holds no codepoint and is left
/// out, which is what the reference's own bold cyrillic declaration is.
fn merged(mut ranges: Vec<(Codepoint, Codepoint)>) -> Vec<(Codepoint, Codepoint)> {
    ranges.retain(|(start, end)| start <= end);
    ranges.sort_unstable();
    let mut merged: Vec<(Codepoint, Codepoint)> = Vec::with_capacity(ranges.len());
    for (start, end) in ranges {
        match merged.last_mut() {
            Some(last) if start <= last.1.after() => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    merged
}
