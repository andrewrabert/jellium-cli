//! A browserless walk of every threshold the pinned stylesheets test and the
//! pixel below it.
//!
//! Every expected value here is read from `reference/breakpoints.tsv`, which
//! `tools/reference/assets.mjs` computed from the stylesheets themselves, so no
//! row of it is written by the same hand that writes the ladder.

use std::collections::BTreeSet;

use jellium_model::appearance::card::{Card, Mixed, PerRow, Rail, Shape};
use jellium_model::appearance::{
    Across, Band, Breakpoint, Css, Dialog, HEIGHTS, Letters, Orientation, Screen, Viewport, WIDTHS,
};

/// One row of the oracle, naming the whole viewport it was resolved at.
struct Row {
    kind: String,
    width: f32,
    height: f32,
    shape: String,
    orientation: String,
    percent: f64,
    across: usize,
    requested: String,
    fill: u32,
    band: String,
    root: f64,
    letters: String,
    dialog: String,
}

impl Row {
    fn viewport(&self) -> Viewport {
        Viewport::new(Css::of(self.width), Css::of(self.height))
    }

    /// The card the row's `kind` and `shape` columns name.
    fn card(&self) -> Card {
        match (self.kind.as_str(), self.shape.as_str()) {
            ("rail", "portrait") => Card::Rail(Rail::Portrait),
            ("rail", "square") => Card::Rail(Rail::Square),
            ("rail", "backdrop") => Card::Rail(Rail::Backdrop),
            ("rail", "smallBackdrop") => Card::Rail(Rail::SmallBackdrop),
            (_, "portrait") => Card::Wall(Shape::Portrait),
            (_, "square") => Card::Wall(Shape::Square),
            (_, "backdrop") => Card::Wall(Shape::Backdrop),
            (_, "smallBackdrop") => Card::Wall(Shape::SmallBackdrop),
            (_, "banner") => Card::Wall(Shape::Banner),
            (_, "mixedPortrait") => Card::Wall(Shape::Mixed(Mixed::Portrait)),
            (_, "mixedSquare") => Card::Wall(Shape::Mixed(Mixed::Square)),
            (_, "mixedBackdrop") => Card::Wall(Shape::Mixed(Mixed::Backdrop)),
            (kind, shape) => {
                panic!("the oracle names a card this port has no shape for: {kind} {shape}")
            }
        }
    }

    /// The share of the page one card's pitch takes, as a percentage, which is
    /// the quantity the `percent` column carries.
    fn share(&self) -> f64 {
        let viewport = self.viewport();
        let canvas = viewport.canvas().width().count() as f64;
        100.0 * self.card().width(viewport).count() as f64 / canvas
    }
}

fn oracle() -> Vec<Row> {
    let text = include_str!("../../reference/breakpoints.tsv");
    let mut lines = text.lines();
    let header = lines.next().expect("the oracle carries a header");
    assert_eq!(
        header,
        "kind\twidth\theight\tshape\torientation\tpercent\tacross\trequested\tfill\tband\troot\tletter_jump\tdialog"
    );
    lines
        .map(|line| {
            let field: Vec<&str> = line.split('\t').collect();
            assert_eq!(field.len(), 13, "a row of the oracle is short: {line}");
            Row {
                kind: field[0].to_owned(),
                width: field[1].parse().expect("a width"),
                height: field[2].parse().expect("a height"),
                shape: field[3].to_owned(),
                orientation: field[4].to_owned(),
                percent: field[5].parse().expect("a percentage"),
                across: field[6].parse().expect("a count"),
                requested: field[7].to_owned(),
                fill: field[8].parse().expect("a requested width"),
                band: field[9].to_owned(),
                root: field[10].parse().expect("a root size"),
                letters: field[11].to_owned(),
                dialog: field[12].to_owned(),
            }
        })
        .collect()
}

fn named(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Portrait => "portrait",
        Orientation::Landscape => "landscape",
    }
}

fn banded(band: Band) -> &'static str {
    match band {
        Band::Mobile => "mobile",
        Band::Desktop => "desktop",
    }
}

fn lettered(letters: Letters) -> &'static str {
    match letters {
        Letters::Shown => "shown",
        Letters::Hidden => "hidden",
    }
}

fn dialoged(dialog: Dialog) -> &'static str {
    match dialog {
        Dialog::Fixed => "fixed",
        Dialog::Fullscreen => "fullscreen",
    }
}

/// The relative distance between two shares, which is how a percentage read
/// from the oracle is compared with one the module computes.
fn apart(held: f64, want: f64) -> f64 {
    (held - want).abs() / want.max(1.0)
}

#[test]
fn every_width_threshold_lands_where_the_reference_puts_it() {
    let rows = oracle();
    let width: Vec<&Row> = rows.iter().filter(|row| row.kind == "width").collect();
    assert!(!width.is_empty());
    for row in width {
        let viewport = row.viewport();
        let at = format!("{}x{} {}", row.width, row.height, row.shape);
        assert_eq!(named(viewport.orientation()), row.orientation, "{at}");
        assert_eq!(banded(viewport.band()), row.band, "{at}");
        assert!(
            (viewport.band().root().factor() as f64 - row.root / 1000.0).abs() < 1e-6,
            "{at}"
        );
        assert_eq!(row.card().across(viewport).count(), row.across, "{at}");
        assert!(apart(row.share(), row.percent) < 1e-3, "{at}");
    }
}

#[test]
fn every_rail_width_lands_where_the_reference_puts_it() {
    let rows = oracle();
    let rail: Vec<&Row> = rows.iter().filter(|row| row.kind == "rail").collect();
    assert!(!rail.is_empty());
    for row in rail {
        let at = format!("{}x{} {}", row.width, row.height, row.shape);
        assert!(apart(row.share(), row.percent) < 1e-3, "{at}");
        assert_eq!(
            row.card().across(row.viewport()).count(),
            row.across,
            "{at}"
        );
    }
}

#[test]
fn the_oracle_covers_every_card() {
    let rows = oracle();
    let named: BTreeSet<(String, String)> = rows
        .iter()
        .map(|row| {
            let kind = match row.kind.as_str() {
                "rail" => "rail",
                _ => "wall",
            };
            (kind.to_owned(), row.shape.clone())
        })
        .collect();
    let every = [
        Card::Wall(Shape::Portrait),
        Card::Wall(Shape::Square),
        Card::Wall(Shape::Backdrop),
        Card::Wall(Shape::SmallBackdrop),
        Card::Wall(Shape::Banner),
        Card::Wall(Shape::Mixed(Mixed::Portrait)),
        Card::Wall(Shape::Mixed(Mixed::Square)),
        Card::Wall(Shape::Mixed(Mixed::Backdrop)),
        Card::Rail(Rail::Portrait),
        Card::Rail(Rail::Square),
        Card::Rail(Rail::Backdrop),
        Card::Rail(Rail::SmallBackdrop),
    ];
    for card in every {
        let held = rows
            .iter()
            .find(|row| row.card() == card)
            .map(|row| (row.kind.clone(), row.shape.clone()));
        assert!(
            held.is_some(),
            "the oracle has no row for {card:?}: {named:?}"
        );
    }
}

#[test]
fn a_wall_row_holds_the_count_the_percentage_names() {
    for row in oracle() {
        if row.kind != "width" || row.shape.starts_with("mixed") {
            continue;
        }
        let viewport = row.viewport();
        let across = row.card().across(viewport);
        let at = format!("{}x{} {}", row.width, row.height, row.shape);
        assert!(
            apart(row.percent * across.count() as f64, 100.0) < 1e-6,
            "{at}"
        );
        let canvas = viewport.canvas().width().count();
        let laid = row.card().width(viewport).count() * across.count() as f32;
        assert!((laid - canvas).abs() < canvas * 1e-5, "{at}");
    }
}

#[test]
fn a_mixed_card_counts_the_same_on_the_page_and_on_the_canvas() {
    for row in oracle() {
        if !row.shape.starts_with("mixed") {
            continue;
        }
        let at = format!("{}x{} {}", row.width, row.height, row.shape);
        let counted = (100.0 / row.percent).floor().max(1.0) as usize;
        assert_eq!(row.card().across(row.viewport()).count(), counted, "{at}");
        assert_eq!(counted, row.across, "{at}");
    }
}

/// Every row's `requested` and `fill` are what `Card::requested` and
/// `Card::image_width` answer, so no arm of the transcribed ladder is
/// unmeasured and no ladder that shares arms with another is folded into it.
/// The arm is compared as `PerRow::written` against the column's text,
/// rendering rather than parsing, because the decimal the emitter writes does
/// not read back as the double the division produces.
#[test]
fn every_request_asks_for_what_the_reference_asks_for() {
    let rows = oracle();
    assert!(!rows.is_empty());
    for row in rows {
        let viewport = row.viewport();
        let at = format!("{}x{} {} {}", row.width, row.height, row.kind, row.shape);
        let card = row.card();
        assert_eq!(card.requested(viewport).written(), row.requested, "{at}");
        let filling = Screen::new(Css::of(row.width));
        assert!(!filling.resizable(viewport), "{at}");
        assert_eq!(
            card.image_width(viewport, Some(filling)).count(),
            row.fill,
            "{at}"
        );
    }
}

/// An arm is a count only where the reference's own digits say so, which is
/// what keeps a rail card's request from being sized by a rounded count.
#[test]
fn an_arm_becomes_a_count_only_where_its_digits_allow_it() {
    for (arm, cards) in [
        (PerRow::cards(10), 10),
        (PerRow::percent(11.1111111111), 9),
        (PerRow::percent(14.2857142857), 7),
        (PerRow::percent(14.28571428571), 7),
        (PerRow::percent(16.66666667), 6),
        (PerRow::percent(33.33333333), 3),
    ] {
        assert_eq!(arm.across(), Across::cards(cards), "{arm:?}");
    }
    for (arm, cards) in [
        (PerRow::percent(11.6), 8),
        (PerRow::percent(15.0), 6),
        (PerRow::percent(15.5), 6),
        (PerRow::percent(18.0), 5),
        (PerRow::percent(18.5), 5),
        (PerRow::percent(23.0), 4),
        (PerRow::percent(23.3), 4),
        (PerRow::percent(23.5), 4),
        (PerRow::percent(30.0), 3),
        (PerRow::percent(31.5), 3),
        (PerRow::percent(40.0), 2),
        (PerRow::percent(42.0), 2),
        (PerRow::percent(56.0), 1),
        (PerRow::percent(72.0), 1),
    ] {
        assert_eq!(arm.across(), Across::cards(cards), "{arm:?}");
    }
}

/// A page inside a wider display rounds its width down to a hundred before the
/// request; one filling its display does not, and neither does one reporting no
/// display at all, which is what the reference's own `if (screen)` guard
/// answers.
#[test]
fn a_resizable_page_rounds_its_width_before_asking() {
    let viewport = Viewport::new(Css::of(1450.0), Css::of(900.0));
    let card = Card::Wall(Shape::Portrait);
    let inside = Screen::new(Css::of(1920.0));
    let filling = Screen::new(Css::of(1450.0));
    assert!(inside.resizable(viewport));
    assert!(!filling.resizable(viewport));
    assert_eq!(card.image_width(viewport, Some(inside)).count(), 200);
    assert_eq!(card.image_width(viewport, Some(filling)).count(), 207);
    assert_eq!(card.image_width(viewport, None).count(), 207);
}

#[test]
fn every_height_threshold_lands_where_the_reference_puts_it() {
    let rows = oracle();
    let height: Vec<&Row> = rows.iter().filter(|row| row.kind == "height").collect();
    assert!(!height.is_empty());
    for row in height {
        let viewport = row.viewport();
        let at = format!("{}x{}", row.width, row.height);
        assert_eq!(lettered(viewport.letters()), row.letters, "{at}");
        assert_eq!(dialoged(viewport.dialog()), row.dialog, "{at}");
    }
}

#[test]
fn a_boundary_pixel_falls_on_the_side_the_stylesheet_puts_it() {
    for at in WIDTHS {
        let bound = at.css().count();
        let exactly = Viewport::new(Css::of(bound), Css::of(bound));
        assert!(exactly.matches(jellium_model::appearance::Query::MaxWidth(*at)));
        assert!(exactly.matches(jellium_model::appearance::Query::MinWidth(*at)));
        let below = Viewport::new(Css::of(bound - 1.0), Css::of(bound));
        assert!(below.matches(jellium_model::appearance::Query::MaxWidth(*at)));
        assert!(!below.matches(jellium_model::appearance::Query::MinWidth(*at)));
    }
    for at in HEIGHTS {
        let bound = at.css().count();
        let exactly = Viewport::new(Css::of(bound), Css::of(bound));
        assert!(exactly.matches(jellium_model::appearance::Query::MaxHeight(*at)));
        assert!(exactly.matches(jellium_model::appearance::Query::MinHeight(*at)));
        let below = Viewport::new(Css::of(bound), Css::of(bound - 1.0));
        assert!(below.matches(jellium_model::appearance::Query::MaxHeight(*at)));
        assert!(!below.matches(jellium_model::appearance::Query::MinHeight(*at)));
    }
}

/// The thresholds the oracle walked, recovered from the pairs of rows it wrote
/// at each one: a threshold and the pixel below it.
fn walked(rows: &[Row], kind: &str, axis: impl Fn(&Row) -> f32) -> Vec<f32> {
    let every: BTreeSet<i64> = rows
        .iter()
        .filter(|row| row.kind == kind)
        .map(|row| axis(row) as i64)
        .collect();
    every
        .iter()
        .filter(|at| every.contains(&(*at - 1)))
        .map(|at| *at as f32)
        .collect()
}

#[test]
fn the_threshold_tables_are_the_ones_the_stylesheets_test() {
    let rows = oracle();
    let held: Vec<f32> = WIDTHS.iter().map(|at| at.css().count()).collect();
    assert_eq!(walked(&rows, "width", |row| row.width), held);
    let held: Vec<f32> = HEIGHTS.iter().map(|at| at.css().count()).collect();
    assert_eq!(walked(&rows, "height", |row| row.height), held);
    let ascending: Vec<Breakpoint> = {
        let mut sorted = WIDTHS.to_vec();
        sorted.sort_by(|one, two| one.css().count().total_cmp(&two.css().count()));
        sorted
    };
    assert_eq!(ascending, WIDTHS.to_vec());
}

#[test]
fn orientation_follows_the_viewports_own_aspect_ratio() {
    let wider = Viewport::new(Css::of(801.0), Css::of(800.0));
    let narrower = Viewport::new(Css::of(799.0), Css::of(800.0));
    assert_eq!(wider.orientation(), Orientation::Landscape);
    assert_eq!(narrower.orientation(), Orientation::Portrait);
}

#[test]
fn a_row_always_holds_a_card() {
    for row in oracle() {
        assert!(row.card().across(row.viewport()).count() >= 1);
    }
}
