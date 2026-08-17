//! The appearance jellyfin-web draws, ported from the revision
//! `reference/PINNED` names. Every value here cites the rule it came from, and
//! `jellium-reference/tests/appearance.rs` fails when a citation stops
//! resolving.

pub mod card;
pub mod css;
pub mod scheme;
pub mod space;
pub mod typeface;

/// A page measurement in css pixels, which is what the browser reports and what
/// every media query tests against a 16px em.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Css {
    count: f32,
}

impl Css {
    pub const fn of(count: f32) -> Css {
        Css { count }
    }

    pub fn count(self) -> f32 {
        self.count
    }

    /// The canvas length this measures, which is this over the band's root.
    pub fn drawn(self, band: Band) -> Drawn {
        Drawn::of(self.count / band.root().factor())
    }
}

/// A length the canvas draws in, which is every length a layout measures.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Drawn {
    count: f32,
}

impl Drawn {
    pub const ZERO: Drawn = Drawn::of(0.0);

    pub const fn of(count: f32) -> Drawn {
        Drawn { count }
    }

    pub fn count(self) -> f32 {
        self.count
    }

    pub fn plus(self, other: Drawn) -> Drawn {
        Drawn::of(self.count + other.count)
    }

    pub fn times(self, ratio: Ratio) -> Drawn {
        Drawn::of(self.count * ratio.factor())
    }
}

/// A share of a containing length, which is how the reference pads a page,
/// proportions a card and fills a progress bar; every share the reference
/// writes is exact in hundredths of a percent.
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

    /// The share `part` is of `whole`, clamped to the whole, and none of it
    /// where `whole` is zero.
    pub fn part(part: i64, whole: i64) -> Share {
        if whole <= 0 || part <= 0 {
            return Share::per_ten_thousand(0);
        }
        let share = part.saturating_mul(10_000) / whole;
        Share::per_ten_thousand(share.min(10_000) as u32)
    }

    pub fn of(self, length: Drawn) -> Drawn {
        Drawn::of(length.count() * self.per_ten_thousand as f32 / 10_000.0)
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
        Drawn::of(width.count() / self.cards as f32)
    }
}

/// One length to another, which is what a line height and a root size are.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio {
    thousandths: u16,
}

impl Ratio {
    pub const fn thousandths(thousandths: u16) -> Ratio {
        Ratio { thousandths }
    }

    pub const fn factor(self) -> f32 {
        self.thousandths as f32 / 1000.0
    }
}

/// The base every design length and every breakpoint is written over, which is
/// the root size a browser starts at.
const BASE: f32 = 16.0;

/// A number as css writes it: the fewest decimals that carry it.
fn trimmed(count: f32) -> String {
    let written = format!("{count:.4}");
    written
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

/// A design length, written in the reference's em over a 16px base.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length {
    em: f32,
}

impl Length {
    pub const fn em(em: f32) -> Length {
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

    pub const fn times(self, ratio: Ratio) -> Length {
        Length::em(self.em * ratio.factor())
    }
}

/// A viewport width or height the pinned stylesheets test against, fixed to a
/// 16px base and never scaled by the band; every threshold the reference writes
/// is a whole number of css pixels, so it is held as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Breakpoint {
    css: u32,
}

impl Breakpoint {
    /// Sixteen to the em, whatever the band's root is, because that is what a
    /// media query measures.
    pub const fn em(em: f32) -> Breakpoint {
        Breakpoint {
            css: (em * BASE) as u32,
        }
    }

    pub fn css(self) -> Css {
        Css::of(self.css as f32)
    }
}

/// Every width the pinned stylesheets test the viewport against, ascending.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    Mobile,
    Desktop,
}

impl Band {
    /// The root size the band draws at, as a ratio of the 16px base.
    pub fn root(self) -> Ratio {
        match self {
            Band::Mobile => typeface::MOBILE_ROOT,
            Band::Desktop => typeface::DESKTOP_ROOT,
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

// reference: band-threshold
pub const BAND: Query = Query::MaxWidth(Breakpoint::em(37.5));

// reference: letter-jump
pub const LETTERS_HIDDEN: Query = Query::MaxHeight(Breakpoint::em(31.25));

// reference: dialog-fullscreen
pub const DIALOG_NARROW: Query = Query::MaxWidth(Breakpoint::em(80.0));

// reference: dialog-fullscreen
pub const DIALOG_SHORT: Query = Query::MaxHeight(Breakpoint::em(45.0));

/// The page's own size, as the page reports it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    width: Css,
    height: Css,
}

impl Viewport {
    pub fn new(width: Css, height: Css) -> Viewport {
        Viewport { width, height }
    }

    pub fn width(self) -> Css {
        self.width
    }

    pub fn height(self) -> Css {
        self.height
    }

    /// The same page as the canvas draws it, which is every layout's measure.
    pub fn canvas(self) -> Canvas {
        let band = self.band();
        Canvas {
            width: self.width.drawn(band),
            height: self.height.drawn(band),
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

    pub fn band(self) -> Band {
        match self.matches(BAND) {
            true => Band::Mobile,
            false => Band::Desktop,
        }
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

/// What the display offers the page, which is only ever asked whether the page
/// can be resized inside it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Screen {
    available: Css,
}

/// The width the display has to offer beyond the page for the page to be
/// resizable inside it.
// reference: card-resizable
const RESIZABLE_BY: f32 = 20.0;

impl Screen {
    pub fn new(available: Css) -> Screen {
        Screen { available }
    }

    /// The reference's `isResizable`: the display offers more than twenty css
    /// pixels of width beyond the page.
    // reference: card-resizable
    pub fn resizable(self, viewport: Viewport) -> bool {
        self.available.count() - viewport.width().count() > RESIZABLE_BY
    }
}

/// The page as the canvas draws it: its css size over the band's root.
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
