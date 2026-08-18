//! The construct gate: what the reference draws against what this client draws.
//!
//! `reference/constructs.tsv` names every construct each page of the pinned
//! reference draws, and `jellium-web/src` names what it draws through the doors
//! of its own `construct.rs`. These tests hold the two to each other, and
//! `reference/exemptions.tsv` is the only thing that licenses a difference.

use std::collections::{BTreeMap, BTreeSet};

use jellium_reference::construct::{Construct, Page, Role, Sentence, Table};
use jellium_reference::drawn::{self, Names, Site};
use jellium_reference::exemption::Exemptions;
use jellium_reference::tree;

/// The table, the exemptions and what the client says it draws, read once.
fn read() -> (
    Table,
    Exemptions,
    Vec<Site>,
    BTreeMap<drawn::Variant, Option<Sentence>>,
) {
    let root = tree::workspace_root();
    let table = Table::read(&root).unwrap_or_else(|trouble| panic!("{trouble}"));
    let exemptions = Exemptions::read(&root).unwrap_or_else(|trouble| panic!("{trouble}"));
    (
        table,
        exemptions,
        drawn::sites(&root),
        drawn::wording(&root),
    )
}

/// The reference pages one site names, and nothing for a site naming none.
fn paged(site: &Site) -> &[Page] {
    match &site.names {
        Names::Pages(pages) => pages,
        Names::Own(_) | Names::Caller => &[],
    }
}

#[test]
/// Every construct `reference/constructs.tsv` names for a page some module
/// draws is drawn by that module, unless a `loss` row of
/// `reference/exemptions.tsv` names it.
fn every_construct_the_reference_draws_is_drawn() {
    let (table, exemptions, sites, _) = read();
    let losses = exemptions.losses();
    let mut missing: Vec<String> = Vec::new();
    for site in &sites {
        let held: BTreeSet<&Construct> = site.drawn.iter().map(|drawn| &drawn.construct).collect();
        for page in paged(site) {
            let Some(constructs) = table.drawn(page) else {
                continue;
            };
            for row in constructs.rows() {
                if held.contains(&row.construct) || losses.contains(&row.construct) {
                    continue;
                }
                missing.push(format!("{} draws no {} of {page}", site.at, row.construct));
            }
        }
    }
    missing.sort_unstable();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "constructs the reference draws that this client does not: {missing:#?}"
    );
}

#[test]
/// Every construct a module draws stands in the table under a page that module
/// names, unless an `own` row of `reference/exemptions.tsv` names it.
fn no_construct_is_drawn_that_the_reference_does_not_draw() {
    let (table, exemptions, sites, _) = read();
    let own = exemptions.own();
    let mut stray: Vec<String> = Vec::new();
    for site in &sites {
        for held in &site.drawn {
            if own.contains(&held.construct) {
                continue;
            }
            let named = paged(site).iter().any(|page| {
                table.drawn(page).is_some_and(|constructs| {
                    constructs
                        .rows()
                        .iter()
                        .any(|row| row.construct == held.construct)
                })
            });
            if !named {
                stray.push(format!("{} draws {}", site.at, held.construct));
            }
        }
    }
    stray.sort_unstable();
    stray.dedup();
    assert!(
        stray.is_empty(),
        "constructs this client draws that the reference does not: {stray:#?}"
    );
}

#[test]
/// A construct the reference wraps in a link or a button is drawn through the
/// navigation door, and one it does not is not.
fn every_construct_is_drawn_in_the_role_the_reference_gives_it() {
    let (table, _, sites, _) = read();
    let mut wrong: Vec<String> = Vec::new();
    for site in &sites {
        for held in &site.drawn {
            for page in paged(site) {
                let Some(constructs) = table.drawn(page) else {
                    continue;
                };
                let roles: Vec<Role> = constructs
                    .rows()
                    .iter()
                    .filter(|row| row.construct == held.construct)
                    .map(|row| row.role)
                    .collect();
                if roles.is_empty() || roles.contains(&held.role) {
                    continue;
                }
                wrong.push(format!(
                    "{} draws {} through {} where {page} gives it {}",
                    site.at,
                    held.construct,
                    held.role.door(),
                    roles
                        .iter()
                        .map(|role| role.door())
                        .collect::<Vec<&str>>()
                        .join(" or ")
                ));
            }
        }
    }
    wrong.sort_unstable();
    wrong.dedup();
    assert!(
        wrong.is_empty(),
        "constructs drawn in a role the reference does not give them: {wrong:#?}"
    );
}

#[test]
/// A construct the reference gives a string key is drawn with a `Text` variant
/// whose own declaration names that key.
/// Rendered English is not compared, so `strings/en-us.json` is not under test.
fn every_drawn_construct_carries_the_reference_s_own_key() {
    let (table, _, sites, wording) = read();
    let mut wrong: Vec<String> = Vec::new();
    for site in &sites {
        for held in &site.drawn {
            for page in paged(site) {
                let Some(constructs) = table.drawn(page) else {
                    continue;
                };
                let keys: BTreeSet<&Sentence> = constructs
                    .rows()
                    .iter()
                    .filter(|row| row.construct == held.construct)
                    .filter_map(|row| row.sentence.as_ref())
                    .collect();
                if keys.is_empty() {
                    continue;
                }
                let said = held
                    .said
                    .as_ref()
                    .and_then(|variant| wording.get(variant))
                    .and_then(|key| key.as_ref());
                if said.is_some_and(|key| keys.contains(key)) {
                    continue;
                }
                wrong.push(format!(
                    "{} draws {} of {page} saying {}",
                    site.at,
                    held.construct,
                    said.map_or_else(
                        || "nothing of the reference".to_owned(),
                        Sentence::to_string
                    )
                ));
            }
        }
    }
    wrong.sort_unstable();
    wrong.dedup();
    assert!(
        wrong.is_empty(),
        "constructs drawn without the reference's own key: {wrong:#?}"
    );
}

#[test]
/// Every module of `jellium-web/src` answering an `Element` names the reference
/// pages it draws or the own constructs it draws.
fn every_drawing_site_names_what_it_draws() {
    let (_, _, sites, _) = read();
    assert!(
        !sites.is_empty(),
        "no drawing site was read out of the tree"
    );
    let unnamed: Vec<&str> = sites
        .iter()
        .filter(|site| matches!(site.names, Names::Caller) && !site.drawn.is_empty())
        .map(|site| site.at.as_str())
        .collect();
    assert!(
        unnamed.is_empty(),
        "modules drawing constructs that name neither a page nor an own construct: {unnamed:#?}"
    );
}

#[test]
/// Every exemption row cites a document that exists and names the row's
/// construct, and every `own` row states a purpose.
fn every_exemption_rests_on_a_document_that_names_it() {
    let root = tree::workspace_root();
    let exemptions = Exemptions::read(&root).unwrap_or_else(|trouble| panic!("{trouble}"));
    assert!(
        !exemptions.rows().is_empty(),
        "no exemption was read out of reference/exemptions.tsv"
    );
    let mut unrested: Vec<String> = Vec::new();
    for row in exemptions.rows() {
        let path = root.join(row.cited.as_str());
        let Ok(text) = std::fs::read_to_string(&path) else {
            unrested.push(format!(
                "{} cites {}, which does not read",
                row.construct,
                row.cited.as_str()
            ));
            continue;
        };
        if !text.contains(row.serves.as_str()) && !names(&text, &row.construct) {
            unrested.push(format!(
                "{} cites {}, which names neither it nor what it serves",
                row.construct,
                row.cited.as_str()
            ));
        }
    }
    unrested.sort_unstable();
    assert!(
        unrested.is_empty(),
        "exemptions resting on no document that names them: {unrested:#?}"
    );
}

/// Whether a document names a construct, under its own name or under the words
/// its name is made of.
fn names(text: &str, construct: &Construct) -> bool {
    let lowered = text.to_lowercase();
    lowered.contains(construct.as_str())
        || construct
            .as_str()
            .split('-')
            .all(|word| lowered.contains(word))
}

#[test]
/// Every own construct the exemption table licenses is drawn, and every own
/// construct drawn has a row.
fn every_own_construct_is_licensed_and_drawn() {
    let (_, exemptions, sites, _) = read();
    let licensed = exemptions.own();
    let drawn: BTreeSet<&Construct> = sites
        .iter()
        .flat_map(|site| match &site.names {
            Names::Own(own) => own.iter().collect::<Vec<&Construct>>(),
            Names::Pages(_) | Names::Caller => Vec::new(),
        })
        .collect();
    let undrawn: Vec<&Construct> = licensed.difference(&drawn).copied().collect();
    let unlicensed: Vec<&Construct> = drawn.difference(&licensed).copied().collect();
    assert!(
        undrawn.is_empty() && unlicensed.is_empty(),
        "own constructs licensed and not drawn: {undrawn:#?}; drawn and not licensed: {unlicensed:#?}"
    );
}

#[test]
/// The table names at least one construct for every page some module names, so
/// a page whose generator arm answered nothing cannot pass by emptiness.
fn no_page_a_module_names_stands_empty() {
    let (table, _, sites, _) = read();
    let mut empty: Vec<String> = sites
        .iter()
        .flat_map(paged)
        .filter(|page| {
            table
                .drawn(page)
                .is_none_or(|constructs| constructs.rows().is_empty())
        })
        .map(Page::to_string)
        .collect();
    empty.sort_unstable();
    empty.dedup();
    assert!(
        empty.is_empty(),
        "pages a module names that the table draws nothing for: {empty:#?}"
    );
}
