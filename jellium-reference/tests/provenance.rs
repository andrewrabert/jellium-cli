//! Every ported construct is one row of `reference/provenance.tsv` cited by a
//! `// reference: <construct>` comment,
//! every construct the reference reaches and this client does not draw is a
//! `dead` row cited by nothing, every construct this client is to draw and
//! does not yet is an `owed` row cited by nothing,
//! and no construct is both. A row whose committed span no longer digests to the
//! recorded hash is a failure, so the pinned reference cannot move under the
//! port without saying so, and none of it asks for a checkout.

use jellium_reference::register::{Construct, Digest, Kind, Register};
use jellium_reference::spans::Spans;
use jellium_reference::tree::{self, Extension};
use std::collections::BTreeSet;
use std::path::Path;

fn register(root: &Path) -> Register {
    Register::read(root).unwrap_or_else(|error| panic!("{error}"))
}

fn spans(root: &Path) -> Spans {
    Spans::read(root).unwrap_or_else(|error| panic!("{error}"))
}

/// Every construct cited by a `// reference: <construct>` comment in the tree,
/// over a walk that fails where it reads no source.
fn cited(root: &Path) -> BTreeSet<Construct> {
    let guard = tree::guard(root);
    let mut sources = tree::files_under(root, &[Extension::RUST]);
    sources.extend(tree::files_under(
        &root.join("jellium-web/js"),
        &[Extension::JAVASCRIPT],
    ));
    assert!(!sources.is_empty(), "no source was read out of the tree");
    sources
        .into_iter()
        .filter(|path| !path.starts_with(&guard))
        .flat_map(|path| {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} does not read: {error}", path.display()));
            text.lines()
                .filter_map(Construct::cited)
                .collect::<Vec<Construct>>()
        })
        .collect()
}

#[test]
fn every_cited_construct_has_exactly_one_ported_row() {
    let root = tree::workspace_root();
    let register = register(&root);
    let ported = register.constructs(Kind::Ported);
    let missing: Vec<String> = cited(&root)
        .into_iter()
        .filter(|construct| !ported.contains(construct))
        .map(|construct| construct.to_string())
        .collect();
    assert!(
        missing.is_empty(),
        "cited with no ported row in reference/provenance.tsv: {missing:?}"
    );
}

#[test]
fn every_ported_row_is_cited() {
    let root = tree::workspace_root();
    let cited = cited(&root);
    let register = register(&root);
    let uncited: Vec<String> = register
        .constructs(Kind::Ported)
        .into_iter()
        .filter(|construct| !cited.contains(*construct))
        .map(Construct::to_string)
        .collect();
    assert!(
        uncited.is_empty(),
        "ported rows no comment cites: {uncited:?}"
    );
}

#[test]
fn no_undrawn_row_is_cited() {
    let root = tree::workspace_root();
    let cited = cited(&root);
    let register = register(&root);
    let reached: Vec<String> = [Kind::Dead, Kind::Owed]
        .into_iter()
        .flat_map(|kind| register.constructs(kind))
        .filter(|construct| cited.contains(construct))
        .map(Construct::to_string)
        .collect();
    assert!(
        reached.is_empty(),
        "rows nothing draws that a comment cites: {reached:?}"
    );
}

#[test]
fn every_row_digests_to_the_committed_span() {
    let root = tree::workspace_root();
    let register = register(&root);
    assert!(
        !register.rows().is_empty(),
        "reference/provenance.tsv records no construct"
    );
    let spans = spans(&root);

    let mut drifted = Vec::new();
    for row in register.rows() {
        let Some(text) = spans.text(&row.construct) else {
            drifted.push(format!(
                "{} ({}) stands under no reference/spans file",
                row.construct, row.span
            ));
            continue;
        };
        let lines = text.split('\n').count();
        if lines != row.span.lines() {
            drifted.push(format!(
                "{} ({}) records {} lines and its span holds {lines}",
                row.construct,
                row.span,
                row.span.lines()
            ));
            continue;
        }
        let digest = Digest::of(text.as_bytes());
        if digest != row.digest {
            drifted.push(format!(
                "{} ({}) records {} and its span digests to {digest}",
                row.construct, row.span, row.digest
            ));
        }
    }
    assert!(
        drifted.is_empty(),
        "rows the committed spans no longer hold: {drifted:#?}"
    );
}

#[test]
fn no_committed_span_stands_without_a_row() {
    let root = tree::workspace_root();
    let register = register(&root);
    let spans = spans(&root);
    assert!(
        !spans.constructs().is_empty(),
        "reference/spans holds no span"
    );
    let stray: Vec<String> = spans
        .constructs()
        .into_iter()
        .filter(|construct| register.row(construct).is_none())
        .map(Construct::to_string)
        .collect();
    assert!(
        stray.is_empty(),
        "spans under reference/spans no row names: {stray:?}"
    );
}
