//! A card is a shape in a flow, and the flow picks the ladder that sizes it.
//!
//! Both ladders below are read in the source order the stylesheet writes them,
//! because that is the order the cascade resolves them in: a later block that
//! matches wins over an earlier one, and the rail ladder's orientation blocks
//! sit between its width blocks rather than beside them.

use super::space::{self, GUTTER};
use super::typeface;
use super::{Across, Breakpoint, Css, Drawn, Length, Orientation, Query, Screen, Share, Viewport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Portrait,
    Backdrop,
    SmallBackdrop,
    Square,
    Banner,
    Mixed(Mixed),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mixed {
    Portrait,
    Square,
    Backdrop,
}

/// Whether the card reserves the reference's `.cardBox-bottompadded` margin,
/// which is a property of the card's box rather than of its footer: the login
/// picker's cards carry it and the select-server page's do not, in the
/// reference's own markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bottom {
    Padded,
    Flush,
}

/// What a card writes under its image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Footer {
    /// No footer, which is what an image-only card draws.
    Bare,
    /// One line, which is the login picker's `singleCardText`.
    Name,
    /// A name over a secondary line, which is a poster's.
    NameAndSubtitle,
}

/// One block of a ladder: the query it answers, and the value it sets.
struct Step<T> {
    at: Option<Query>,
    orientation: Option<Orientation>,
    held: T,
}

/// A ladder: what it sets before any block matches, and the blocks in the
/// source order the stylesheet writes them.
struct Ladder<T: Copy + 'static> {
    base: T,
    steps: &'static [Step<T>],
}

impl<T: Copy> Ladder<T> {
    /// The last step whose query the viewport answers, which is what the
    /// cascade leaves standing.
    fn resolved(&self, viewport: Viewport) -> T {
        let mut standing = self.base;
        for step in self.steps {
            let wide_enough = step.at.is_none_or(|at| viewport.matches(at));
            let turned = step
                .orientation
                .is_none_or(|orientation| viewport.orientation() == orientation);
            if wide_enough && turned {
                standing = step.held;
            }
        }
        standing
    }
}

const fn step<T>(at: f32, held: T) -> Step<T> {
    Step {
        at: Some(Query::MinWidth(Breakpoint::em(at))),
        orientation: None,
        held,
    }
}

const fn landscape<T>(at: Option<f32>, held: T) -> Step<T> {
    Step {
        at: match at {
            Some(em) => Some(Query::MinWidth(Breakpoint::em(em))),
            None => None,
        },
        orientation: Some(Orientation::Landscape),
        held,
    }
}

// reference: card-width-ladder
const BANNER: Ladder<Across> = Ladder {
    base: Across::cards(1),
    steps: &[
        step(50.0, Across::cards(2)),
        step(75.0, Across::cards(3)),
        step(131.25, Across::cards(4)),
    ],
};

// reference: card-width-ladder
const BACKDROP: Ladder<Across> = Ladder {
    base: Across::cards(1),
    steps: &[
        step(25.0, Across::cards(2)),
        step(48.125, Across::cards(3)),
        step(75.0, Across::cards(4)),
        step(100.0, Across::cards(5)),
        step(156.25, Across::cards(6)),
    ],
};

// reference: card-width-ladder
const SMALL_BACKDROP: Ladder<Across> = Ladder {
    base: Across::cards(2),
    steps: &[
        step(31.25, Across::cards(3)),
        step(50.0, Across::cards(4)),
        step(62.5, Across::cards(5)),
        step(75.0, Across::cards(6)),
        step(87.5, Across::cards(7)),
        step(100.0, Across::cards(8)),
    ],
};

// reference: card-width-ladder
const SQUARE: Ladder<Across> = Ladder {
    base: Across::cards(2),
    steps: &[
        step(31.25, Across::cards(3)),
        step(43.75, Across::cards(4)),
        step(50.0, Across::cards(5)),
        step(75.0, Across::cards(6)),
        step(87.5, Across::cards(7)),
        step(100.0, Across::cards(8)),
        step(120.0, Across::cards(9)),
        step(131.25, Across::cards(10)),
    ],
};

// reference: card-width-ladder
const PORTRAIT: Ladder<Across> = Ladder {
    base: Across::cards(3),
    steps: &[
        step(31.25, Across::cards(3)),
        step(43.75, Across::cards(4)),
        step(50.0, Across::cards(5)),
        step(75.0, Across::cards(6)),
        step(87.5, Across::cards(7)),
        step(100.0, Across::cards(8)),
        step(120.0, Across::cards(9)),
        step(131.25, Across::cards(10)),
    ],
};

/// The cards a row of `shape` holds, in the source order of `card-width` and
/// `card-width-ladder`.
// reference: card-width-ladder
fn wall(shape: Shape) -> &'static Ladder<Across> {
    match shape {
        Shape::Banner => &BANNER,
        Shape::Backdrop => &BACKDROP,
        Shape::SmallBackdrop => &SMALL_BACKDROP,
        Shape::Square | Shape::Mixed(Mixed::Square) => &SQUARE,
        Shape::Portrait | Shape::Mixed(Mixed::Portrait) => &PORTRAIT,
        Shape::Mixed(Mixed::Backdrop) => &BACKDROP,
    }
}

const fn share(at: f32, per_ten_thousand: u32) -> Step<Share> {
    step(at, Share::per_ten_thousand(per_ten_thousand))
}

const fn share_landscape(at: Option<f32>, per_ten_thousand: u32) -> Step<Share> {
    landscape(at, Share::per_ten_thousand(per_ten_thousand))
}

// reference: card-rail-ladder
const RAIL_BACKDROP: Ladder<Share> = Ladder {
    base: Share::per_ten_thousand(7200),
    steps: &[
        share(35.0, 4550),
        share(48.125, 3000),
        share_landscape(None, 3000),
        share_landscape(Some(48.125), 2310),
        share(75.0, 2310),
        share(100.0, 1870),
        share(156.25, 1560),
    ],
};

// reference: card-rail-ladder
const RAIL_SMALL_BACKDROP: Ladder<Share> = Ladder {
    base: Share::per_ten_thousand(7200),
    steps: &[
        share(35.0, 3000),
        share(48.125, 3000),
        share_landscape(None, 3000),
        share_landscape(Some(48.125), 2310),
        share_landscape(Some(50.0), 1550),
        share(75.0, 2310),
        share(100.0, 1870),
        share(156.25, 1560),
    ],
};

// reference: card-rail-ladder
const RAIL_SQUARE: Ladder<Share> = Ladder {
    base: Share::per_ten_thousand(4000),
    steps: &[
        share(35.0, 3120),
        share(43.75, 2310),
        share_landscape(None, 2310),
        share(50.0, 1850),
        share(75.0, 1550),
        share(87.5, 1330),
        share(100.0, 1160),
        share(120.0, 1041),
        share(131.25, 930),
    ],
};

// reference: card-rail-ladder
const RAIL_PORTRAIT: Ladder<Share> = Ladder {
    base: Share::per_ten_thousand(4000),
    steps: &[
        share(25.0, 3120),
        share(43.75, 2310),
        share_landscape(None, 2310),
        share(50.0, 1850),
        share(75.0, 1550),
        share(87.5, 1330),
        share(100.0, 1160),
        share(120.0, 1041),
        share(131.25, 930),
    ],
};

// reference: card-rail-ladder
fn rail_ladder(rail: Rail) -> &'static Ladder<Share> {
    match rail {
        Rail::Backdrop => &RAIL_BACKDROP,
        Rail::SmallBackdrop => &RAIL_SMALL_BACKDROP,
        Rail::Square => &RAIL_SQUARE,
        Rail::Portrait => &RAIL_PORTRAIT,
    }
}

impl Shape {
    /// The padder's share of the card's own width: 150%, 56.25%, 100%, 18.5%.
    // reference: card-aspect
    pub fn aspect(self) -> Share {
        match self {
            Shape::Portrait | Shape::Mixed(Mixed::Portrait) => Share::per_ten_thousand(15_000),
            Shape::Backdrop | Shape::SmallBackdrop | Shape::Mixed(Mixed::Backdrop) => {
                Share::per_ten_thousand(5625)
            }
            Shape::Square | Shape::Mixed(Mixed::Square) => Share::WHOLE,
            Shape::Banner => Share::per_ten_thousand(1850),
        }
    }

    /// The fixed width a mixed card is written at, which no ladder steps.
    // reference: card-width
    fn fixed(self) -> Option<Length> {
        match self {
            Shape::Mixed(Mixed::Portrait) => Some(Length::em(12.0)),
            Shape::Mixed(Mixed::Square) => Some(Length::em(18.0)),
            Shape::Mixed(Mixed::Backdrop) => Some(Length::em(32.0)),
            Shape::Portrait
            | Shape::Backdrop
            | Shape::SmallBackdrop
            | Shape::Square
            | Shape::Banner => None,
        }
    }
}

/// How a run of cards is laid out, which is what picks the ladder that sizes
/// them; the reference gives an overflow class to four shapes and none to the
/// rest, so no unreachable pairing is constructible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    /// Cards wrap into rows, sized by the stylesheet ladder.
    Wall(Shape),
    /// Cards scroll sideways, sized by the viewport-unit ladder.
    Rail(Rail),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rail {
    Portrait,
    Square,
    Backdrop,
    SmallBackdrop,
}

impl Rail {
    fn shape(self) -> Shape {
        match self {
            Rail::Portrait => Shape::Portrait,
            Rail::Square => Shape::Square,
            Rail::Backdrop => Shape::Backdrop,
            Rail::SmallBackdrop => Shape::SmallBackdrop,
        }
    }
}

impl Card {
    pub fn shape(self) -> Shape {
        match self {
            Card::Wall(shape) => shape,
            Card::Rail(rail) => rail.shape(),
        }
    }

    /// The pitch one card occupies, gutter included.
    pub fn width(self, viewport: Viewport) -> Drawn {
        let canvas = viewport.canvas().width();
        match self {
            Card::Wall(shape) => match shape.fixed() {
                Some(width) => width.drawn(),
                None => wall(shape).resolved(viewport).pitch(canvas),
            },
            Card::Rail(rail) => rail_ladder(rail).resolved(viewport).of(canvas),
        }
    }

    /// Cards a row holds.
    pub fn across(self, viewport: Viewport) -> Across {
        if let Card::Wall(shape) = self
            && shape.fixed().is_none()
        {
            return wall(shape).resolved(viewport);
        }
        let width = self.width(viewport).count();
        if width <= 0.0 {
            return Across::cards(1);
        }
        Across::cards((viewport.canvas().width().count() / width) as u32)
    }

    /// The pitch one card occupies down the page, gutter included.
    pub fn row(self, viewport: Viewport, footer: Footer, bottom: Bottom) -> Drawn {
        let gutter = GUTTER.drawn();
        let inside = Drawn::of(self.width(viewport).count() - gutter.count());
        self.shape()
            .aspect()
            .of(inside)
            .plus(written(footer))
            .plus(reserved(bottom, viewport))
            .plus(gutter)
    }

    /// The arm `getPostersPerRow` answers, which steps at 2200px and 420px
    /// where the wall ladder steps at 2100px and 400px, and which is not a
    /// whole number.
    // reference: card-count-ladder
    pub fn requested(self, viewport: Viewport) -> PerRow {
        request(self).resolved(viewport)
    }

    /// The width an image is asked for, which `getImageWidth` computes as the
    /// page width over the arm itself, rounded to nearest, after `setCardData`
    /// has floored a resizable page's width to a hundred.
    // reference: card-image-width
    pub fn image_width(self, viewport: Viewport, screen: Screen) -> Fill {
        let asked = match screen.resizable(viewport) {
            true => Viewport::new(rounded(viewport.width()), viewport.height()),
            false => viewport,
        };
        self.requested(asked).fill(asked.width())
    }
}

/// The hundred a resizable page's width is floored to before it asks for an
/// image.
// reference: card-screen-rounding
const ROUND_SCREEN_TO: f32 = 100.0;

/// A resizable page's width as the reference asks with it.
// reference: card-screen-rounding
fn rounded(width: Css) -> Css {
    Css::of((width.count() / ROUND_SCREEN_TO).floor() * ROUND_SCREEN_TO)
}

/// One arm of the request ladder: cards per row as the reference computes it,
/// which is `100 / percent` and stays unrounded, because `getImageWidth`
/// divides a width by it whole and only the layout ever wants a count.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerRow {
    rate: f64,
}

/// The decimals a double carries, which is as far as a written percent can go.
const WRITTEN_DIGITS: usize = 17;

/// The decimals `reference/breakpoints.tsv` writes its `requested` column to.
// oracle: reference/breakpoints.tsv
const REQUESTED_DECIMALS: usize = 11;

impl PerRow {
    /// An arm the reference writes as a whole number of cards.
    pub const fn cards(cards: u32) -> PerRow {
        PerRow { rate: cards as f64 }
    }

    /// An arm the reference writes as `100 / percent`, carrying the digits it
    /// writes the percent to.
    pub const fn percent(percent: f64) -> PerRow {
        PerRow {
            rate: 100.0 / percent,
        }
    }

    /// Whether this arm is the reference's own way of writing `cards` cards:
    /// `100 / cards`, rendered to some number of decimals and divided back
    /// into a hundred, is exactly this arm.
    fn names(self, cards: f64) -> bool {
        (0..=WRITTEN_DIGITS).any(|digits| {
            let percent = format!("{:.*}", digits, 100.0 / cards)
                .parse::<f64>()
                .unwrap_or_default();
            percent > 0.0 && 100.0 / percent == self.rate
        })
    }

    /// The count a row lays out.
    pub fn across(self) -> Across {
        let whole = self.rate.round();
        if whole >= 1.0 && self.names(whole) {
            return Across::cards(whole as u32);
        }
        Across::cards(self.rate.floor().max(1.0) as u32)
    }

    /// The arm rendered to eleven decimals, which is how
    /// `reference/breakpoints.tsv` writes its `requested` column and the only
    /// precision that separates `100 / 14.2857142857` from
    /// `100 / 14.28571428571`.
    pub fn written(self) -> String {
        format!("{:.*}", REQUESTED_DECIMALS, self.rate)
    }

    /// `Math.round(width / rate)`, which is `getImageWidth`.
    // reference: card-image-width
    pub fn fill(self, width: Css) -> Fill {
        Fill {
            css: (f64::from(width.count()) / self.rate).round() as u32,
        }
    }
}

/// How wide an image the server is asked for; it crosses into a query string,
/// so nothing above the request carries it as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fill {
    css: u32,
}

impl Fill {
    pub fn count(self) -> u32 {
        self.css
    }
}

/// Whether an arm asks the page to be turned the way `getImageWidth` tests it,
/// which is not the css orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turned {
    Landscape,
    Either,
}

/// How much wider than tall `getImageWidth` asks a page to be before it calls
/// it landscape.
// reference: card-image-width
const WIDER_THAN_TALL: f32 = 1.3;

/// One arm of a `getPostersPerRow` switch, in the source order the reference
/// tests them.
struct Arm {
    turned: Turned,
    at: Option<Css>,
    held: PerRow,
}

/// One shape's cards-per-row switch: its arms, and the `default` it falls to.
struct Request {
    arms: &'static [Arm],
    otherwise: PerRow,
}

impl Request {
    fn resolved(&self, viewport: Viewport) -> PerRow {
        let landscape = viewport.width().count() > viewport.height().count() * WIDER_THAN_TALL;
        for arm in self.arms {
            let turned = match arm.turned {
                Turned::Landscape => landscape,
                Turned::Either => true,
            };
            if turned && arm.at.is_none_or(|at| viewport.width() >= at) {
                return arm.held;
            }
        }
        self.otherwise
    }
}

const fn arm(at: f32, held: PerRow) -> Arm {
    Arm {
        turned: Turned::Either,
        at: Some(Css::of(at)),
        held,
    }
}

const fn turned(at: Option<f32>, held: PerRow) -> Arm {
    Arm {
        turned: Turned::Landscape,
        at: match at {
            Some(css) => Some(Css::of(css)),
            None => None,
        },
        held,
    }
}

// reference: card-count-ladder
const REQUEST_PORTRAIT: Request = Request {
    arms: &[
        arm(2200.0, PerRow::cards(10)),
        arm(1920.0, PerRow::percent(11.1111111111)),
        arm(1600.0, PerRow::cards(8)),
        arm(1400.0, PerRow::percent(14.28571428571)),
        arm(1200.0, PerRow::percent(16.66666667)),
        arm(800.0, PerRow::cards(5)),
        arm(700.0, PerRow::cards(4)),
        arm(500.0, PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::percent(33.33333333),
};

// reference: card-count-ladder
const REQUEST_SQUARE: Request = Request {
    arms: &[
        arm(2200.0, PerRow::cards(10)),
        arm(1920.0, PerRow::percent(11.1111111111)),
        arm(1600.0, PerRow::cards(8)),
        arm(1400.0, PerRow::percent(14.28571428571)),
        arm(1200.0, PerRow::percent(16.66666667)),
        arm(800.0, PerRow::cards(5)),
        arm(700.0, PerRow::cards(4)),
        arm(500.0, PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::cards(2),
};

// reference: card-count-ladder
const REQUEST_BANNER: Request = Request {
    arms: &[
        arm(2200.0, PerRow::cards(4)),
        arm(1200.0, PerRow::percent(33.33333333)),
        arm(800.0, PerRow::cards(2)),
    ],
    otherwise: PerRow::cards(1),
};

// reference: card-count-ladder
const REQUEST_BACKDROP: Request = Request {
    arms: &[
        arm(2500.0, PerRow::cards(6)),
        arm(1600.0, PerRow::cards(5)),
        arm(1200.0, PerRow::cards(4)),
        arm(770.0, PerRow::cards(3)),
        arm(420.0, PerRow::cards(2)),
    ],
    otherwise: PerRow::cards(1),
};

// reference: card-count-ladder
const REQUEST_SMALL_BACKDROP: Request = Request {
    arms: &[
        arm(1600.0, PerRow::cards(8)),
        arm(1400.0, PerRow::percent(14.2857142857)),
        arm(1200.0, PerRow::percent(16.66666667)),
        arm(1000.0, PerRow::cards(5)),
        arm(800.0, PerRow::cards(4)),
        arm(500.0, PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::cards(2),
};

/// A shape the reference's own switch does not name, which is every mixed one,
/// takes its `default`.
// reference: card-count-ladder
const REQUEST_MIXED: Request = Request {
    arms: &[],
    otherwise: PerRow::cards(4),
};

// reference: card-count-ladder
const REQUEST_RAIL_PORTRAIT: Request = Request {
    arms: &[
        turned(Some(1700.0), PerRow::percent(11.6)),
        turned(None, PerRow::percent(15.5)),
        arm(1400.0, PerRow::percent(15.0)),
        arm(1200.0, PerRow::percent(18.0)),
        arm(760.0, PerRow::percent(23.0)),
        arm(400.0, PerRow::percent(31.5)),
    ],
    otherwise: PerRow::percent(42.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_SQUARE: Request = Request {
    arms: &[
        turned(Some(1700.0), PerRow::percent(11.6)),
        turned(None, PerRow::percent(15.5)),
        arm(1400.0, PerRow::percent(15.0)),
        arm(1200.0, PerRow::percent(18.0)),
        arm(760.0, PerRow::percent(23.0)),
        arm(540.0, PerRow::percent(31.5)),
    ],
    otherwise: PerRow::percent(42.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_BACKDROP: Request = Request {
    arms: &[
        turned(Some(1700.0), PerRow::percent(18.5)),
        turned(None, PerRow::percent(23.3)),
        arm(1800.0, PerRow::percent(23.5)),
        arm(1400.0, PerRow::percent(30.0)),
        arm(760.0, PerRow::percent(40.0)),
        arm(640.0, PerRow::percent(56.0)),
    ],
    otherwise: PerRow::percent(72.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_SMALL_BACKDROP: Request = Request {
    arms: &[
        turned(Some(800.0), PerRow::percent(15.5)),
        turned(None, PerRow::percent(23.3)),
        arm(540.0, PerRow::percent(30.0)),
    ],
    otherwise: PerRow::percent(72.0),
};

// reference: card-count-ladder
fn request(card: Card) -> &'static Request {
    match card {
        Card::Wall(Shape::Portrait) => &REQUEST_PORTRAIT,
        Card::Wall(Shape::Square) => &REQUEST_SQUARE,
        Card::Wall(Shape::Banner) => &REQUEST_BANNER,
        Card::Wall(Shape::Backdrop) => &REQUEST_BACKDROP,
        Card::Wall(Shape::SmallBackdrop) => &REQUEST_SMALL_BACKDROP,
        Card::Wall(Shape::Mixed(_)) => &REQUEST_MIXED,
        Card::Rail(Rail::Portrait) => &REQUEST_RAIL_PORTRAIT,
        Card::Rail(Rail::Square) => &REQUEST_RAIL_SQUARE,
        Card::Rail(Rail::Backdrop) => &REQUEST_RAIL_BACKDROP,
        Card::Rail(Rail::SmallBackdrop) => &REQUEST_RAIL_SMALL_BACKDROP,
    }
}

/// What a card's footer takes down the page.
// reference: card-footer
fn written(footer: Footer) -> Drawn {
    let name = space::CARD_FOOTER_PAD
        .top
        .plus(typeface::BODY.times(typeface::LINE_HEIGHT))
        .plus(space::CARD_FOOTER_PAD.bottom);
    match footer {
        Footer::Bare => Drawn::ZERO,
        Footer::Name => name.drawn(),
        Footer::NameAndSubtitle => name
            .plus(typeface::SECONDARY.times(typeface::LINE_HEIGHT))
            .drawn(),
    }
}

/// What a card's box reserves below itself, which the reference narrows on a
/// narrow page.
// reference: card-box-bottom
fn reserved(bottom: Bottom, viewport: Viewport) -> Drawn {
    match bottom {
        Bottom::Flush => Drawn::ZERO,
        Bottom::Padded => match viewport.matches(space::CARD_BOTTOM_AT) {
            true => space::CARD_BOTTOM_NARROW.drawn(),
            false => space::CARD_BOTTOM.drawn(),
        },
    }
}
