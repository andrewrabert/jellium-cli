//! The appearance jellyfin-web draws, ported from the revision
//! `reference/PINNED` names. Every value here cites the rule it came from, and
//! `jellium-reference/tests/appearance.rs` fails when a citation stops
//! resolving.

pub mod blur;
pub mod card;
pub mod css;
pub mod document;
pub mod scheme;
pub mod scroll;
pub mod space;
pub mod typeface;

/// The count rounded to the nearest whole, which is the unit this module holds
/// a decimal the reference wrote in.
// every count this module scales is at or above nothing
const fn nearest(count: f64) -> u32 {
    (count + 0.5) as u32
}

/// A page measurement in css pixels, which is what the browser reports and what
/// every media query tests against a 16px em.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Css {
    count: f64,
}

impl Css {
    pub const fn of(count: f64) -> Css {
        Css { count }
    }

    /// A length a script or a MUI style declaration writes as a bare number,
    /// which the DOM reads as a count of css pixels.
    pub const fn unitless(count: f64) -> Css {
        Css { count }
    }

    pub fn count(self) -> f64 {
        self.count
    }

    pub const fn times(self, ratio: Ratio) -> Css {
        Css::of(self.count * ratio.factor())
    }

    /// Const, so a measure that is one written count less another is written as
    /// that difference rather than as its arithmetic result.
    pub const fn less(self, other: Css) -> Css {
        Css::of(self.count - other.count)
    }

    /// This measure held between `floor` and `ceiling`, which is what a script
    /// writes as a `Math.min` over a `Math.max`.
    pub fn held(self, floor: Css, ceiling: Css) -> Css {
        match self < floor {
            true => floor,
            false => match self > ceiling {
                true => ceiling,
                false => self,
            },
        }
    }

    /// MUI's `pxToRem`: this count of css pixels as the design length it is
    /// over the 16px base.
    pub const fn length(self) -> Length {
        Length::em(self.count / BASE)
    }

    /// The canvas length this measures, which is this over the layout's root.
    pub fn drawn(self, layout: Layout) -> Drawn {
        Drawn::of(self.count / layout.root().factor())
    }
}

/// A length the canvas draws in, which is every length a layout measures.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Drawn {
    count: f64,
}

impl Drawn {
    pub const ZERO: Drawn = Drawn::of(0.0);

    pub const fn of(count: f64) -> Drawn {
        Drawn { count }
    }

    pub fn count(self) -> f64 {
        self.count
    }

    pub fn plus(self, other: Drawn) -> Drawn {
        Drawn::of(self.count + other.count)
    }

    /// The plain difference, which stands below nothing where `other` is the
    /// longer.
    pub const fn less(self, other: Drawn) -> Drawn {
        Drawn::of(self.count - other.count)
    }

    pub fn times(self, ratio: Ratio) -> Drawn {
        Drawn::of(self.count * ratio.factor())
    }
}

/// A share of a containing length, which is how the reference pads a page,
/// proportions a card and fills a progress bar; a share the reference writes
/// as a repeating percentage is carried as an `Across` instead.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Share {
    per_ten_thousand: u32,
}

impl Share {
    pub const WHOLE: Share = Share::per_ten_thousand(10_000);

    pub const fn per_ten_thousand(share: u32) -> Share {
        Share {
            per_ten_thousand: share,
        }
    }

    /// The nearest ten-thousandth to the count of the reference's viewport
    /// units the stylesheet wrote, which are one share whichever axis it is
    /// taken of; the axis is the caller's, written as `of(canvas.width())` or
    /// `of(canvas.height())`.
    pub const fn units(count: f64) -> Share {
        Share::per_ten_thousand(nearest(count * 100.0))
    }

    /// The share `part` is of `whole`, clamped to the whole, and none of it
    /// where `whole` is zero.
    pub fn part(part: i64, whole: i64) -> Share {
        if whole <= 0 || part <= 0 {
            return Share::per_ten_thousand(0);
        }
        let share = part.saturating_mul(10_000) / whole;
        Share::per_ten_thousand(share.min(10_000) as u32)
    }

    /// What this leaves after `taken`, and none of it where `taken` is more
    /// than this.
    pub const fn less(self, taken: Share) -> Share {
        Share::per_ten_thousand(self.per_ten_thousand.saturating_sub(taken.per_ten_thousand))
    }

    /// A share is taken of either measurement and answers in the one it was
    /// given, so a share of the page is a page length and a share of the canvas
    /// a canvas length; it cannot mix them and it is not a way around
    /// `Css::drawn` being the only crossing.
    pub fn of<M: Measure>(self, length: M) -> M {
        length.scaled(self.per_ten_thousand as f64 / 10_000.0)
    }

    /// The nearest ten-thousandth to a count out of a hundred, which is how a
    /// server reports how far through an item a viewer is, held to the whole
    /// and to none of it.
    pub fn per_hundred(count: f64) -> Share {
        Share::per_ten_thousand(nearest(count.clamp(0.0, 100.0) * 100.0))
    }

    /// This as the count out of a hundred, which is what the reading beside a
    /// progress bar writes.
    pub fn percent(self) -> f64 {
        self.per_ten_thousand as f64 / 100.0
    }
}

/// The two measurements a page length can be, sealed so nothing else becomes
/// one.
pub trait Measure: Copy + sealed::Scaled {}

impl Measure for Css {}

impl Measure for Drawn {}

mod sealed {
    use super::{Css, Drawn};

    pub trait Scaled {
        /// This length multiplied by `factor`, in the measurement it already is.
        fn scaled(self, factor: f64) -> Self;
    }

    impl Scaled for Css {
        fn scaled(self, factor: f64) -> Css {
            Css::of(self.count * factor)
        }
    }

    impl Scaled for Drawn {
        fn scaled(self, factor: f64) -> Drawn {
            Drawn::of(self.count * factor)
        }
    }
}

/// A cap css measures against the box the capped thing stands in: a share of
/// that box, and the offset css sums with that share. `calc(100% - 96px)` is
/// that sum, and css reads the length written after the `-` as the sum's own
/// negative term.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cap {
    pub share: Share,
    pub offset: Css,
}

impl Cap {
    /// `held` under this cap of `whole`, which is what a `max-height` does to
    /// the height a box would otherwise stand at; nothing where the cap falls
    /// under nothing.
    pub fn holds(self, held: Drawn, whole: Drawn, layout: Layout) -> Drawn {
        let cap = self.share.of(whole).plus(self.offset.drawn(layout));
        Drawn::of(held.count().min(cap.count()).max(0.0))
    }
}

/// Cards a row holds, which is what the stylesheet ladder actually carries: its
/// every percentage is `100 / cards` written out to thirty digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Across {
    cards: u32,
}

impl Across {
    /// A count, never zero.
    pub const fn cards(cards: u32) -> Across {
        Across {
            cards: if cards == 0 { 1 } else { cards },
        }
    }

    pub fn count(self) -> usize {
        self.cards as usize
    }

    /// The pitch this count leaves each card in `width`.
    pub fn pitch(self, width: Drawn) -> Drawn {
        Drawn::of(width.count() / self.cards as f64)
    }
}

/// A count of the twelve columns MUI's grid divides a row into, held in tenths
/// because a `Grid item` names its width to one decimal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Columns {
    tenths: u32,
}

impl Columns {
    /// A count, never zero.
    pub const fn twelfths(count: f64) -> Columns {
        let tenths = nearest(count * 10.0);
        Columns {
            tenths: if tenths == 0 { 1 } else { tenths },
        }
    }

    /// How many items this wide stand across one row.
    pub fn across(self) -> Across {
        Across::cards(ROW.tenths / self.tenths)
    }
}

/// MUI's own twelve, which every `Grid item`'s width is a count of.
// reference: mui-grid
const ROW: Columns = Columns::twelfths(12.0);

/// One length to another, which is what a line height and a root size are.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio {
    thousandths: u16,
}

impl Ratio {
    pub const fn thousandths(thousandths: u16) -> Ratio {
        Ratio { thousandths }
    }

    /// A ratio the reference writes as a percentage, which is how a root font
    /// size is written.
    pub const fn percent(percent: f64) -> Ratio {
        Ratio {
            thousandths: nearest(percent * 10.0) as u16,
        }
    }

    /// The double nearest the decimal the reference wrote, which is the double
    /// the reference's own literal parses to.
    pub const fn factor(self) -> f64 {
        self.thousandths as f64 / 1000.0
    }

    /// This ratio taken `other` of, to the nearest thousandth, which the
    /// reference leaves unquantised.
    pub const fn times(self, other: Ratio) -> Ratio {
        Ratio::thousandths(nearest(self.thousandths as f64 * other.factor()) as u16)
    }
}

/// How far a MUI surface stands off the page, which is what decides the white
/// MUI lays over its own paper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elevation {
    steps: u8,
}

impl Elevation {
    pub const fn steps(steps: u8) -> Elevation {
        Elevation { steps }
    }

    pub fn count(self) -> f64 {
        self.steps as f64
    }
}

/// The base every design length and every breakpoint is written over.
// standard: css-initial-font-size — a media query resolves an em against the
// initial font size rather than the root's, and sixteen css pixels is that size
// in every engine this client runs on; it is the base the reference's own em
// values are written over and the reference never writes it
const BASE: f64 = 16.0;

/// A number as css writes it: the fewest decimals that carry it.
fn trimmed(count: f64) -> String {
    let written = format!("{count:.4}");
    written
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

/// A design length, written in the reference's em over a 16px base.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length {
    em: f64,
}

impl Length {
    pub const fn em(em: f64) -> Length {
        Length { em }
    }

    /// Sixteen to the em, with no root in it, because the canvas applies the
    /// root once for the whole surface.
    pub fn drawn(self) -> Drawn {
        Drawn::of(self.em * BASE)
    }

    /// Const, so a length that is the sum or the multiple of ported lengths is
    /// written as that sum rather than as its arithmetic result.
    pub const fn plus(self, other: Length) -> Length {
        Length::em(self.em + other.em)
    }

    /// Const, so a length that is one ported length less another is written as
    /// that difference rather than as its arithmetic result.
    pub const fn less(self, other: Length) -> Length {
        Length::em(self.em - other.em)
    }

    pub const fn times(self, ratio: Ratio) -> Length {
        Length::em(self.em * ratio.factor())
    }

    /// Two margins that meet where css does not collapse them, which is any
    /// pair side by side and any pair stacked inside a flex container.
    pub const fn abutting(self, other: Length) -> Length {
        Length::em(self.em + other.em)
    }

    /// Two vertical margins that meet as block-level siblings in normal flow,
    /// which css collapses into the larger of the two.
    pub const fn collapsing(self, other: Length) -> Length {
        match self.em > other.em {
            true => self,
            false => other,
        }
    }

    /// The taller of two boxes standing side by side, which is what a flex
    /// row's own height is.
    pub const fn taller(self, other: Length) -> Length {
        match self.em > other.em {
            true => self,
            false => other,
        }
    }
}

/// A viewport width or height the pinned stylesheets test against, fixed to a
/// 16px base and never scaled by the layout; every threshold the reference writes
/// is a whole number of css pixels, so it is held as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Breakpoint {
    css: u32,
}

impl Breakpoint {
    /// Sixteen to the em, whatever the layout's root is, because that is what a
    /// media query measures.
    pub const fn em(em: f64) -> Breakpoint {
        Breakpoint {
            css: nearest(em * BASE),
        }
    }

    /// The same threshold where the reference writes one in css pixels rather
    /// than in em, as `.dynamicFilterDialog`'s `min-height: 600px` does, so a
    /// threshold is spelt here the way the reference spells it.
    pub const fn pixels(css: u32) -> Breakpoint {
        Breakpoint { css }
    }

    pub fn css(self) -> Css {
        Css::of(self.css as f64)
    }
}

/// Every width the pinned client tests the viewport against, ascending.
// oracle: reference/breakpoints.tsv
pub const WIDTHS: &[Breakpoint] = &[
    Breakpoint::em(24.0),
    Breakpoint::em(25.0),
    Breakpoint::em(29.0),
    Breakpoint::em(30.0),
    Breakpoint::em(31.25),
    Breakpoint::em(32.0),
    Breakpoint::em(33.75),
    Breakpoint::em(34.375),
    Breakpoint::em(35.0),
    Breakpoint::em(37.5),
    Breakpoint::em(40.0),
    Breakpoint::em(43.0),
    Breakpoint::em(43.75),
    Breakpoint::em(48.125),
    Breakpoint::em(50.0),
    Breakpoint::em(56.0),
    Breakpoint::pixels(900),
    Breakpoint::em(60.0),
    Breakpoint::em(62.5),
    Breakpoint::em(63.0),
    Breakpoint::em(64.0),
    Breakpoint::em(66.0),
    Breakpoint::em(68.75),
    Breakpoint::em(70.0),
    Breakpoint::em(75.0),
    Breakpoint::em(80.0),
    Breakpoint::em(87.5),
    Breakpoint::pixels(1536),
    Breakpoint::em(100.0),
    Breakpoint::em(112.5),
    Breakpoint::em(120.0),
    Breakpoint::em(131.25),
    Breakpoint::em(156.25),
];

/// Every height the pinned stylesheets test the viewport against, ascending.
// oracle: reference/breakpoints.tsv
pub const HEIGHTS: &[Breakpoint] = &[
    Breakpoint::em(31.25),
    Breakpoint::em(32.0),
    Breakpoint::em(37.0),
    Breakpoint::em(37.5),
    Breakpoint::em(44.0),
    Breakpoint::em(45.0),
    Breakpoint::em(49.0),
    Breakpoint::em(50.0),
];

/// Which of the three layouts jellyfin-web draws in, which the reference
/// carries as a class on the root element and never as a media query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layout {
    Mobile,
    Desktop,
    Television,
}

impl Layout {
    /// The root size the layout draws at, as a ratio of the 16px base.
    pub fn root(self) -> Ratio {
        match self {
            Layout::Mobile => typeface::MOBILE_ROOT,
            Layout::Desktop => typeface::DESKTOP_ROOT,
            Layout::Television => typeface::TELEVISION_ROOT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// One media feature the pinned stylesheets test, carrying its own direction,
/// so a rule ported into a constant cannot be compared the wrong way round at a
/// call site; css makes both bounds inclusive and so does this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Query {
    MaxWidth(Breakpoint),
    MinWidth(Breakpoint),
    MaxHeight(Breakpoint),
    MinHeight(Breakpoint),
}

/// Whether the letter jump is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letters {
    Shown,
    Hidden,
}

/// How a dialog is sized, in the reference's own words: `.dialog-fixedSize`
/// keeps the size it was given until the viewport pins it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialog {
    Fixed,
    Fullscreen,
}

// reference: letter-jump
const LETTERS_HIDDEN: Query = Query::MaxHeight(Breakpoint::em(31.25));

// reference: dialog-fullscreen
const DIALOG_NARROW: Query = Query::MaxWidth(Breakpoint::em(80.0));

// reference: dialog-fullscreen
const DIALOG_SHORT: Query = Query::MaxHeight(Breakpoint::em(45.0));

/// The page as this client draws it: the size the page reports, and the layout
/// the browser showing it is drawn in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    width: Css,
    height: Css,
    layout: Layout,
}

impl Viewport {
    pub fn new(width: Css, height: Css, layout: Layout) -> Viewport {
        Viewport {
            width,
            height,
            layout,
        }
    }

    pub fn width(self) -> Css {
        self.width
    }

    pub fn height(self) -> Css {
        self.height
    }

    /// The same page as the canvas draws it, which is every layout's measure.
    pub fn canvas(self) -> Canvas {
        let layout = self.layout;
        Canvas {
            width: self.width.drawn(layout),
            height: self.height.drawn(layout),
        }
    }

    /// True exactly when a browser applies a rule under `query`.
    pub fn matches(self, query: Query) -> bool {
        match query {
            Query::MaxWidth(at) => self.width.count() <= at.css().count(),
            Query::MinWidth(at) => self.width.count() >= at.css().count(),
            Query::MaxHeight(at) => self.height.count() <= at.css().count(),
            Query::MinHeight(at) => self.height.count() >= at.css().count(),
        }
    }

    pub fn layout(self) -> Layout {
        self.layout
    }

    /// Landscape when the width is at least the height.
    pub fn orientation(self) -> Orientation {
        match self.width.count() >= self.height.count() {
            true => Orientation::Landscape,
            false => Orientation::Portrait,
        }
    }

    pub fn letters(self) -> Letters {
        match self.matches(LETTERS_HIDDEN) {
            true => Letters::Hidden,
            false => Letters::Shown,
        }
    }

    pub fn dialog(self) -> Dialog {
        match self.matches(DIALOG_NARROW) || self.matches(DIALOG_SHORT) {
            true => Dialog::Fullscreen,
            false => Dialog::Fixed,
        }
    }
}

/// What the display offers, which the drawer's width is taken from and the
/// page's resizability tested against.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Screen {
    available: Css,
}

/// The width the display has to offer beyond the page for the page to be
/// resizable inside it.
// reference: card-resizable
const RESIZABLE_BY: Css = Css::unitless(20.0);

/// The room the reference leaves beside the open navigation drawer, which the
/// drawer's width is the display less.
// reference: nav-drawer-width
const MAIN_DRAWER_BESIDE: Css = Css::unitless(50.0);

/// The narrowest the navigation drawer is drawn, which is also its width where
/// the page reports no display.
// reference: nav-drawer-width
const MAIN_DRAWER_NARROWEST: Css = Css::unitless(240.0);

/// The widest the navigation drawer is drawn.
// reference: nav-drawer-width
const MAIN_DRAWER_WIDEST: Css = Css::unitless(320.0);

impl Screen {
    pub fn new(available: Css) -> Screen {
        Screen { available }
    }

    /// The reference's `isResizable`: the display offers more than twenty css
    /// pixels of width beyond the page.
    // reference: card-resizable
    pub fn resizable(self, viewport: Viewport) -> bool {
        self.available.less(viewport.width()) > RESIZABLE_BY
    }

    /// `getNavDrawerOptions`: this display less the room left beside the
    /// drawer, held between the drawer's narrowest and its widest.
    // reference: nav-drawer-width
    pub fn main_drawer_width(self) -> Css {
        self.available
            .less(MAIN_DRAWER_BESIDE)
            .held(MAIN_DRAWER_NARROWEST, MAIN_DRAWER_WIDEST)
    }
}

/// The page as the canvas draws it: its css size over the layout's root.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Canvas {
    width: Drawn,
    height: Drawn,
}

impl Canvas {
    pub fn width(self) -> Drawn {
        self.width
    }

    pub fn height(self) -> Drawn {
        self.height
    }
}
