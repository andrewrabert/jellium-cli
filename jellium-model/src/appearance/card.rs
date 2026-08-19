//! A card is a shape in a flow, and the flow picks the ladder that sizes it.
//!
//! Both ladders below are read in the source order the stylesheet writes them,
//! because that is the order the cascade resolves them in: a later block that
//! matches wins over an earlier one, and the rail ladder's orientation blocks
//! sit between its width blocks rather than beside them.

use jellyfin_api::types::{CollectionType, MediaType};

use super::space::{self, Room};
use super::typeface;
use super::{
    Across, Breakpoint, Css, Drawn, Layout, Length, Orientation, Query, Ratio, Screen, Share,
    Viewport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Portrait,
    Backdrop,
    SmallBackdrop,
    Square,
    Banner,
    Mixed(Mixed),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// One line a card's footer writes, named by the option in
/// `getCardFooterText` that pushes it.
// reference: card-footer-lines
// reference: card-footer-options
// reference: card-live-tv-naming
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    // `showParentTitle` and `showParentTitleOrTitle`, which the reference
    // pushes before the name and which an item of a live-tv type answers with
    // its own name
    ParentTitle,
    // the display name, which an item whose name the line above already
    // carries answers with nothing
    Name,
    // the parent title `parentTitleUnderneath` pushes under the name instead,
    // which an album, a track and a music video answer with their artists
    ParentTitleUnder,
    // `showYear`
    Year,
    // `showAirTime` with `showAirEndTime`
    AirTime,
    // `showChannelName`
    ChannelName,
    // `showCurrentProgram`
    CurrentProgram,
    // `showCurrentProgramTime`
    CurrentProgramTime,
    // `showSeriesTimerTime`, which an any-time series timer answers with
    // `Anytime`
    SeriesTimerTime,
    // `showSeriesTimerChannel`, which an any-channel series timer answers with
    // `All channels`
    SeriesTimerChannel,
}

/// One line a card's footer writes: text the reference's own `if (text)` test
/// passes.
// reference: card-text-lines
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caption(String);

impl Caption {
    // none where the text is blank, which is the line the reference drops
    pub fn of(text: String) -> Option<Caption> {
        match text.is_empty() {
            true => None,
            false => Some(Caption(text)),
        }
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

/// What a card writes under its image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Footer {
    /// No line, which is what an image-only card draws.
    Bare,
    /// `showTitle` alone, which is the login picker's `singleCardText` and a
    /// library tile's.
    Name,
    /// `showParentTitle`, `showTitle` and `showYear`, capped at the two lines
    /// `lines` writes, which is what a home rail, a library grid, a search
    /// result and the recordings tab each ask for.
    ParentNameAndYear,
    /// The channels tab's own: `showTitle`, `showCurrentProgram` and
    /// `showCurrentProgramTime`.
    Channel,
    /// A scheduled timer's: `showParentTitleOrTitle`, `showTitle`, and
    /// `showAirTime` with `showAirEndTime`.
    Timer,
    /// An active recording's: a timer's lines with `showChannelName` under
    /// them.
    ActiveRecording,
    /// A series timer's: `showTitle`, `showSeriesTimerTime` and
    /// `showSeriesTimerChannel`, which is the three lines `lines` writes.
    SeriesTimer,
}

impl Footer {
    /// The lines this footer pushes, in the order `getCardFooterText` pushes
    /// them.
    // reference: card-footer-lines
    // reference: card-footer-options
    pub fn pushed(self) -> &'static [Line] {
        match self {
            Footer::Bare => &[],
            Footer::Name => &[Line::Name],
            Footer::ParentNameAndYear => &[
                Line::ParentTitle,
                Line::Name,
                Line::ParentTitleUnder,
                Line::Year,
            ],
            Footer::Channel => &[Line::Name, Line::CurrentProgram, Line::CurrentProgramTime],
            Footer::Timer => &[Line::ParentTitle, Line::Name, Line::AirTime],
            Footer::ActiveRecording => &[
                Line::ParentTitle,
                Line::Name,
                Line::AirTime,
                Line::ChannelName,
            ],
            Footer::SeriesTimer => &[Line::Name, Line::SeriesTimerTime, Line::SeriesTimerChannel],
        }
    }

    /// The lines this footer writes: the pushes, capped where `options.lines`
    /// caps them. A card writes this many lines whatever its own item answers,
    /// where the reference drops a line no option pushed and lets two cards in
    /// one run differ in height.
    // reference: card-text-lines
    pub fn written(self) -> usize {
        let capped = match self {
            Footer::ParentNameAndYear => Some(2),
            Footer::SeriesTimer => Some(3),
            Footer::Bare
            | Footer::Name
            | Footer::Channel
            | Footer::Timer
            | Footer::ActiveRecording => None,
        };
        let pushed = self.pushed().len();
        match capped {
            Some(cap) => pushed.min(cap),
            None => pushed,
        }
    }
}

/// Whether a card's lines stand inside `.cardFooter`'s own padding, which
/// `getCardFooterText` writes for a card standing on the paper and which the
/// login pages write by hand.
// reference: card-footer-element
// reference: card-footer-outer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Footing {
    Padded,
    Bare,
}

/// How a wrapping run of cards lays a row that does not fill it:
/// `.vertical-wrap` leaves it at the leading edge and `.vertical-wrap.centered`
/// centres it.
// reference: card-container
// reference: page-centering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrap {
    Leading,
    Centred,
}

/// Whether the shadow and the radius fall on the card's whole box, which
/// `.visualCardBox` stands on the scheme's own paper behind image and footer
/// alike, or on the frame its image stands in alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backing {
    Padder,
    Paper,
}

/// Where a card's footer sets its lines: `.cardTextCentered` centres them and
/// `.cardText` alone leaves them at the leading edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    Centred,
    Leading,
}

/// One block of a ladder: the query it answers, and the value it sets.
struct Step<T> {
    at: Option<Query>,
    orientation: Option<Orientation>,
    held: T,
}

/// A ladder: what it sets before any block matches, the blocks in the source
/// order the stylesheet writes them, and what `.itemsContainer-tv`'s own
/// selector sets, which outranks every block by specificity.
struct Ladder<T: Copy + 'static> {
    base: T,
    steps: &'static [Step<T>],
    televised: Option<T>,
}

impl<T: Copy> Ladder<T> {
    /// The `.itemsContainer-tv` value where the browser is drawn in the
    /// television layout, and the last step whose query the viewport answers
    /// otherwise.
    fn resolved(&self, viewport: Viewport) -> T {
        if let Some(held) = self.televised
            && viewport.layout() == Layout::Television
        {
            return held;
        }
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

const fn step<T>(at: Breakpoint, held: T) -> Step<T> {
    Step {
        at: Some(Query::MinWidth(at)),
        orientation: None,
        held,
    }
}

const fn landscape<T>(at: Option<Breakpoint>, held: T) -> Step<T> {
    Step {
        at: match at {
            Some(breakpoint) => Some(Query::MinWidth(breakpoint)),
            None => None,
        },
        orientation: Some(Orientation::Landscape),
        held,
    }
}

// reference: card-width
// reference: card-width-ladder
const BANNER: Ladder<Across> = Ladder {
    base: Across::cards(1),
    steps: &[
        step(Breakpoint::em(50.0), Across::cards(2)),
        step(Breakpoint::em(75.0), Across::cards(3)),
        step(Breakpoint::em(131.25), Across::cards(4)),
    ],
    televised: None,
};

// reference: card-width
// reference: card-width-ladder
// reference: card-width-televised
const BACKDROP: Ladder<Across> = Ladder {
    base: Across::cards(1),
    steps: &[
        step(Breakpoint::em(25.0), Across::cards(2)),
        step(Breakpoint::em(48.125), Across::cards(3)),
        step(Breakpoint::em(75.0), Across::cards(4)),
        step(Breakpoint::em(100.0), Across::cards(5)),
        step(Breakpoint::em(156.25), Across::cards(6)),
    ],
    televised: Some(Across::cards(4)),
};

// reference: card-width
// reference: card-width-ladder
const SMALL_BACKDROP: Ladder<Across> = Ladder {
    base: Across::cards(2),
    steps: &[
        step(Breakpoint::em(31.25), Across::cards(3)),
        step(Breakpoint::em(50.0), Across::cards(4)),
        step(Breakpoint::em(62.5), Across::cards(5)),
        step(Breakpoint::em(75.0), Across::cards(6)),
        step(Breakpoint::em(87.5), Across::cards(7)),
        step(Breakpoint::em(100.0), Across::cards(8)),
    ],
    televised: None,
};

// reference: card-width
// reference: card-width-ladder
// reference: card-width-televised
const SQUARE: Ladder<Across> = Ladder {
    base: Across::cards(2),
    steps: &[
        step(Breakpoint::em(31.25), Across::cards(3)),
        step(Breakpoint::em(43.75), Across::cards(4)),
        step(Breakpoint::em(50.0), Across::cards(5)),
        step(Breakpoint::em(75.0), Across::cards(6)),
        step(Breakpoint::em(87.5), Across::cards(7)),
        step(Breakpoint::em(100.0), Across::cards(8)),
        step(Breakpoint::em(120.0), Across::cards(9)),
        step(Breakpoint::em(131.25), Across::cards(10)),
    ],
    televised: Some(Across::cards(6)),
};

// reference: card-width
// reference: card-width-ladder
// reference: card-width-televised
const PORTRAIT: Ladder<Across> = Ladder {
    base: Across::cards(3),
    steps: &[
        step(Breakpoint::em(31.25), Across::cards(3)),
        step(Breakpoint::em(43.75), Across::cards(4)),
        step(Breakpoint::em(50.0), Across::cards(5)),
        step(Breakpoint::em(75.0), Across::cards(6)),
        step(Breakpoint::em(87.5), Across::cards(7)),
        step(Breakpoint::em(100.0), Across::cards(8)),
        step(Breakpoint::em(120.0), Across::cards(9)),
        step(Breakpoint::em(131.25), Across::cards(10)),
    ],
    televised: Some(Across::cards(6)),
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

/// The box a card's own ladder measures against: a wall card's share is of the
/// box the page lays it in, and a rail card's is of the viewport, the
/// reference writing its rail ladder in `vw`.
// reference: card-width-ladder
// reference: card-rail-ladder
fn measured(card: Card, room: Room) -> Drawn {
    match card {
        Card::Wall(_) => room.width(),
        Card::Rail(_) => room.viewport().canvas().width(),
    }
}

// reference: card-rail-ladder
// reference: card-rail-overrides
const RAIL_BACKDROP: Ladder<Share> = Ladder {
    base: Share::units(72.0),
    steps: &[
        step(Breakpoint::em(35.0), Share::units(45.5)),
        step(Breakpoint::em(48.125), Share::units(30.0)),
        landscape(None, Share::units(30.0)),
        landscape(Some(Breakpoint::em(48.125)), Share::units(23.1)),
        step(Breakpoint::em(75.0), Share::units(23.1)),
        step(Breakpoint::em(100.0), Share::units(18.7)),
        step(Breakpoint::em(156.25), Share::units(15.6)),
    ],
    televised: Some(Share::units(23.5)),
};

// reference: card-rail-ladder
// reference: card-rail-overrides
const RAIL_SMALL_BACKDROP: Ladder<Share> = Ladder {
    base: Share::units(72.0),
    steps: &[
        step(Breakpoint::em(35.0), Share::units(30.0)),
        step(Breakpoint::em(48.125), Share::units(30.0)),
        landscape(None, Share::units(30.0)),
        landscape(Some(Breakpoint::em(48.125)), Share::units(23.1)),
        landscape(Some(Breakpoint::em(50.0)), Share::units(15.5)),
        step(Breakpoint::em(75.0), Share::units(23.1)),
        step(Breakpoint::em(100.0), Share::units(18.7)),
        step(Breakpoint::em(156.25), Share::units(15.6)),
    ],
    televised: Some(Share::units(18.8)),
};

// reference: card-rail-ladder
// reference: card-rail-overrides
const RAIL_SQUARE: Ladder<Share> = Ladder {
    base: Share::units(40.0),
    steps: &[
        step(Breakpoint::em(35.0), Share::units(31.2)),
        step(Breakpoint::em(43.75), Share::units(23.1)),
        landscape(None, Share::units(23.1)),
        step(Breakpoint::em(50.0), Share::units(18.5)),
        step(Breakpoint::em(75.0), Share::units(15.5)),
        step(Breakpoint::em(87.5), Share::units(13.3)),
        step(Breakpoint::em(100.0), Share::units(11.6)),
        step(Breakpoint::em(120.0), Share::units(10.41)),
        step(Breakpoint::em(131.25), Share::units(9.3)),
    ],
    televised: Some(Share::units(15.6)),
};

// reference: card-rail-ladder
// reference: card-rail-overrides
const RAIL_PORTRAIT: Ladder<Share> = Ladder {
    base: Share::units(40.0),
    steps: &[
        step(Breakpoint::em(25.0), Share::units(31.2)),
        step(Breakpoint::em(43.75), Share::units(23.1)),
        landscape(None, Share::units(23.1)),
        step(Breakpoint::em(50.0), Share::units(18.5)),
        step(Breakpoint::em(75.0), Share::units(15.5)),
        step(Breakpoint::em(87.5), Share::units(13.3)),
        step(Breakpoint::em(100.0), Share::units(11.6)),
        step(Breakpoint::em(120.0), Share::units(10.41)),
        step(Breakpoint::em(131.25), Share::units(9.3)),
    ],
    televised: Some(Share::units(15.6)),
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

/// An image's width over its height, as the server reports it.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Aspect {
    ratio: f64,
}

/// How far a median may stand from a ratio and still snap onto it.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Tolerance {
    apart: f64,
}

impl Tolerance {
    const fn of(apart: f64) -> Tolerance {
        Tolerance { apart }
    }

    fn holds(self, one: Aspect, other: Aspect) -> bool {
        (one.ratio - other.ratio).abs() <= self.apart
    }
}

/// The ratios the reference snaps a median onto, each with the distance it
/// snaps from: 2:3, 16:9, 1:1, 4:3, in the order the reference tests them.
// reference: primary-aspect
const SNAPPED: [(Aspect, Tolerance); 4] = [
    (Aspect::over(2.0, 3.0), Tolerance::of(0.15)),
    (Aspect::over(16.0, 9.0), Tolerance::of(0.2)),
    (Aspect::of(1.0), Tolerance::of(0.15)),
    (Aspect::over(4.0, 3.0), Tolerance::of(0.15)),
];

impl Aspect {
    pub const fn of(ratio: f64) -> Aspect {
        Aspect { ratio }
    }

    /// An aspect the reference writes as one count over another.
    const fn over(wide: f64, tall: f64) -> Aspect {
        Aspect { ratio: wide / tall }
    }

    pub fn ratio(self) -> f64 {
        self.ratio
    }

    /// The aspect a set of items shares: the median of those that report one,
    /// snapped to 2:3, 16:9, 1:1 or 4:3 inside the reference's own tolerance.
    /// `None` where no item reports one.
    // reference: primary-aspect
    pub fn shared(reported: impl Iterator<Item = Aspect>) -> Option<Aspect> {
        let mut held: Vec<f64> = reported
            .map(Aspect::ratio)
            .filter(|ratio| *ratio != 0.0)
            .collect();
        held.sort_by(|one, other| one.total_cmp(other));
        let half = held.len() / 2;
        let middle = match held.len() % 2 {
            0 => (held.get(half.checked_sub(1)?)? + held.get(half)?) / 2.0,
            _ => *held.get(half)?,
        };
        let middle = Aspect::of(middle);
        Some(
            SNAPPED
                .into_iter()
                .find(|(onto, apart)| apart.holds(*onto, middle))
                .map_or(middle, |(onto, _)| onto),
        )
    }
}

/// The aspect at or above which items ask for a banner.
// reference: card-auto-shape
const BANNER_AT: Aspect = Aspect::of(3.0);

/// The aspect at or above which items ask for a backdrop.
// reference: card-auto-shape
const BACKDROP_AT: Aspect = Aspect::of(1.33);

/// The aspect above which items ask for a square and at or below which they ask
/// for a portrait.
// reference: card-auto-shape
const SQUARE_ABOVE: Aspect = Aspect::of(0.8);

impl Shape {
    /// The shape items of this aspect ask for: banner at 3 and wider, backdrop
    /// at 1.33, square above 0.8, portrait below it.
    // reference: card-auto-shape
    fn of(aspect: Aspect) -> Shape {
        match aspect {
            held if held >= BANNER_AT => Shape::Banner,
            held if held >= BACKDROP_AT => Shape::Backdrop,
            held if held > SQUARE_ABOVE => Shape::Square,
            _ => Shape::Portrait,
        }
    }

    /// The shape items of this aspect ask for, and `default` where they share
    /// no aspect.
    // reference: card-auto-shape
    pub fn fitting(aspect: Option<Aspect>, default: Shape) -> Shape {
        aspect.map_or(default, Shape::of)
    }

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
    /// Cards wrap into rows, sized by the stylesheet ladder.
    Wall(Shape),
    /// Cards scroll sideways, sized by the viewport-unit ladder.
    Rail(Rail),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Every card this client draws.
    pub fn all() -> impl Iterator<Item = Card> {
        [
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
        ]
        .into_iter()
    }

    /// Every page width this card's own `getPostersPerRow` switch compares a
    /// page width against, in the source order the reference tests them.
    // reference: card-count-ladder
    pub fn tested(self) -> impl Iterator<Item = Css> {
        request(self).arms.iter().filter_map(|arm| arm.at)
    }

    /// The card a resumed item draws on, which its own media type decides: a
    /// resumed book is a portrait rail and every other resumed item a backdrop
    /// one.
    // reference: home-resume
    pub fn resumed(media: Option<MediaType>) -> Card {
        match media {
            Some(MediaType::Book) => Card::Rail(Rail::Portrait),
            Some(MediaType::Unknown | MediaType::Video | MediaType::Audio | MediaType::Photo)
            | None => Card::Rail(Rail::Backdrop),
        }
    }

    /// The card `UserCardBox` is built on.
    // reference: user-card
    pub const USER: Card = Card::Wall(Shape::Square);

    /// The card the next-up section draws on.
    // reference: home-next-up
    pub const NEXT_UP: Card = Card::Rail(Rail::Backdrop);

    /// The card a library tile draws on, the My Media section scrolling its
    /// tiles sideways because `enableScrollX` answers true and nothing else.
    // reference: home-library-tiles
    // reference: home-scroll-x
    pub const LIBRARY: Card = Card::Rail(Rail::Backdrop);

    /// The card a library's latest row draws on, which the library's own
    /// collection type decides.
    // reference: home-latest
    // the same rule's fourth disjunct reaches portrait for an item type of
    // `Channel` rather than for a collection type, and this client's latest
    // rows carry no such type, so a channels view draws backdrop here where the
    // reference draws portrait
    pub fn latest(collection: Option<CollectionType>) -> Card {
        match collection {
            Some(CollectionType::Movies | CollectionType::Books | CollectionType::Tvshows) => {
                Card::Rail(Rail::Portrait)
            }
            Some(CollectionType::Music | CollectionType::Homevideos) => Card::Rail(Rail::Square),
            Some(
                CollectionType::Unknown
                | CollectionType::Musicvideos
                | CollectionType::Trailers
                | CollectionType::Boxsets
                | CollectionType::Photos
                | CollectionType::Livetv
                | CollectionType::Playlists
                | CollectionType::Folders,
            )
            | None => Card::Rail(Rail::Backdrop),
        }
    }

    /// The card a library's grid draws: the shape that library's own
    /// controller writes, and for a library with no controller of its own the
    /// shape its items ask for.
    // reference: grid-card
    // reference: grid-card-series
    // reference: grid-card-album
    // reference: grid-card-auto
    pub fn grid(collection: Option<CollectionType>, aspect: Option<Aspect>) -> Card {
        match collection {
            Some(CollectionType::Movies | CollectionType::Tvshows) => Card::Wall(Shape::Portrait),
            Some(CollectionType::Music) => Card::Wall(Shape::Square),
            Some(
                CollectionType::Unknown
                | CollectionType::Musicvideos
                | CollectionType::Trailers
                | CollectionType::Books
                | CollectionType::Homevideos
                | CollectionType::Boxsets
                | CollectionType::Photos
                | CollectionType::Livetv
                | CollectionType::Playlists
                | CollectionType::Folders,
            )
            | None => Card::Wall(Shape::fitting(aspect, Shape::Square)),
        }
    }

    /// The card a scrolling row draws for items of this aspect: the
    /// reference's overflow shapes, the banner card it falls to where they are
    /// three times as wide as they are tall, and `default` where they share no
    /// aspect.
    // reference: card-auto-shape
    pub fn overflowing(aspect: Option<Aspect>, default: Card) -> Card {
        match aspect.map(Shape::of) {
            Some(Shape::Portrait) => Card::Rail(Rail::Portrait),
            Some(Shape::Backdrop) => Card::Rail(Rail::Backdrop),
            Some(Shape::SmallBackdrop) => Card::Rail(Rail::SmallBackdrop),
            Some(Shape::Banner) => Card::Wall(Shape::Banner),
            Some(Shape::Square | Shape::Mixed(_)) => Card::Rail(Rail::Square),
            None => default,
        }
    }

    pub fn shape(self) -> Shape {
        match self {
            Card::Wall(shape) => shape,
            Card::Rail(rail) => rail.shape(),
        }
    }

    /// The pitch one card occupies, gutter included.
    // a wall card's pitch is a share of the box the page lays it in
    // a rail card's pitch is a share of the viewport, whatever box holds it
    pub fn width(self, room: Room) -> Drawn {
        match self {
            Card::Wall(shape) => match shape.fixed() {
                Some(width) => width.drawn(),
                None => wall(shape).resolved(room.viewport()).pitch(room.width()),
            },
            Card::Rail(rail) => rail_ladder(rail)
                .resolved(room.viewport())
                .of(measured(self, room)),
        }
    }

    /// The card's own width inside its pitch, the pitch reserving the gutter
    /// its `.cardBox` margin makes.
    pub fn inside(self, room: Room) -> Drawn {
        self.width(room).less(space::GUTTER.drawn())
    }

    /// Cards a row holds.
    // a wall card counts across the box the page lays it in
    // a rail card counts across the viewport, which is `Math.floor(100 / vw)`
    pub fn across(self, room: Room) -> Across {
        if let Card::Wall(shape) = self
            && shape.fixed().is_none()
        {
            return wall(shape).resolved(room.viewport());
        }
        let width = self.width(room).count();
        if width <= 0.0 {
            return Across::cards(1);
        }
        Across::cards((measured(self, room).count() / width) as u32)
    }

    /// The arm `getPostersPerRow` answers, which a television takes before
    /// every width it tests, which steps at 2200px and 420px where the wall
    /// ladder steps at 2100px and 400px, and which is not a whole number.
    // reference: card-count-ladder
    pub fn requested(self, viewport: Viewport) -> PerRow {
        request(self).resolved(viewport)
    }

    /// The width an image is asked for, which `getImageWidth` computes as the
    /// page width over the arm itself, rounded to nearest, after `setCardData`
    /// has floored a resizable page's width to a hundred. A page reporting no
    /// display is not resizable and so is not rounded, which is what the
    /// reference's own `if (screen)` guard answers.
    // reference: card-image-width
    pub fn image_width(self, viewport: Viewport, screen: Option<Screen>) -> Fill {
        let resizable = screen.is_some_and(|screen| screen.resizable(viewport));
        let asked = match resizable {
            true => Viewport::new(
                rounded(viewport.width()),
                viewport.height(),
                viewport.layout(),
            ),
            false => viewport,
        };
        self.requested(asked).fill(asked.width())
    }
}

/// The hundred a resizable page's width is floored to before it asks for an
/// image.
// reference: card-width-request
const ROUND_SCREEN_TO: Css = Css::unitless(100.0);

/// A resizable page's width as the reference asks with it.
// reference: card-width-request
fn rounded(width: Css) -> Css {
    Css::of((width.count() / ROUND_SCREEN_TO.count()).floor() * ROUND_SCREEN_TO.count())
}

/// One arm of the request ladder: cards per row as the reference computes it,
/// which is `100 / percent` and stays unrounded, because `getImageWidth`
/// divides a width by it whole and only the layout ever wants a count.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerRow {
    rate: f64,
}

/// The decimals a written percent can go to before it stops naming a double.
// standard: ieee-754 — a binary64 round-trips through seventeen decimal digits,
// which is what the request ladder's arms are rendered to when they are
// compared against the oracle's own text
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

    /// `Math.round(width / rate)`, which is `getImageWidth`, on the very double
    /// the reference divides, with nothing narrower standing between the
    /// division and the rounding.
    // reference: card-image-width
    pub fn fill(self, width: Css) -> Fill {
        Fill::of(Css::of(width.count() / self.rate))
    }
}

/// How wide an image the server is asked for; it crosses into a query string,
/// so nothing above the request carries it as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fill {
    css: u32,
}

impl Fill {
    /// A request width is a page measurement, so a `Fill` is built from a `Css`
    /// and from nothing else: `PerRow::fill` builds one for an image and
    /// `space::preview` for a trickplay tile, and no canvas length reaches it,
    /// there being no crossing from `Drawn` back to `Css`.
    pub fn of(width: Css) -> Fill {
        Fill {
            css: width.count().round() as u32,
        }
    }

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
// reference: card-width-request
const WIDER_THAN_TALL: Ratio = Ratio::thousandths(1300);

/// One arm of a `getPostersPerRow` switch, in the source order the reference
/// tests them.
struct Arm {
    turned: Turned,
    at: Option<Css>,
    held: PerRow,
}

/// One shape's cards-per-row switch: the arm the reference answers a
/// television with before it tests anything else, its arms in the source order
/// it tests them, and the `default` it falls to.
struct Request {
    televised: Option<PerRow>,
    arms: &'static [Arm],
    otherwise: PerRow,
}

impl Request {
    /// The `isTV` arm where the browser is drawn in the television layout, the
    /// first arm whose condition the viewport answers otherwise, and the
    /// `default` under them both.
    fn resolved(&self, viewport: Viewport) -> PerRow {
        if let Some(held) = self.televised
            && viewport.layout() == Layout::Television
        {
            return held;
        }
        let landscape = viewport.width() > viewport.height().times(WIDER_THAN_TALL);
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

const fn arm(at: Css, held: PerRow) -> Arm {
    Arm {
        turned: Turned::Either,
        at: Some(at),
        held,
    }
}

const fn turned(at: Option<Css>, held: PerRow) -> Arm {
    Arm {
        turned: Turned::Landscape,
        at,
        held,
    }
}

// reference: card-count-ladder
const REQUEST_PORTRAIT: Request = Request {
    televised: Some(PerRow::percent(16.66666667)),
    arms: &[
        arm(Css::unitless(2200.0), PerRow::cards(10)),
        arm(Css::unitless(1920.0), PerRow::percent(11.1111111111)),
        arm(Css::unitless(1600.0), PerRow::cards(8)),
        arm(Css::unitless(1400.0), PerRow::percent(14.28571428571)),
        arm(Css::unitless(1200.0), PerRow::percent(16.66666667)),
        arm(Css::unitless(800.0), PerRow::cards(5)),
        arm(Css::unitless(700.0), PerRow::cards(4)),
        arm(Css::unitless(500.0), PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::percent(33.33333333),
};

// reference: card-count-ladder
const REQUEST_SQUARE: Request = Request {
    televised: Some(PerRow::percent(16.66666667)),
    arms: &[
        arm(Css::unitless(2200.0), PerRow::cards(10)),
        arm(Css::unitless(1920.0), PerRow::percent(11.1111111111)),
        arm(Css::unitless(1600.0), PerRow::cards(8)),
        arm(Css::unitless(1400.0), PerRow::percent(14.28571428571)),
        arm(Css::unitless(1200.0), PerRow::percent(16.66666667)),
        arm(Css::unitless(800.0), PerRow::cards(5)),
        arm(Css::unitless(700.0), PerRow::cards(4)),
        arm(Css::unitless(500.0), PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::cards(2),
};

// reference: card-count-ladder
const REQUEST_BANNER: Request = Request {
    televised: None,
    arms: &[
        arm(Css::unitless(2200.0), PerRow::cards(4)),
        arm(Css::unitless(1200.0), PerRow::percent(33.33333333)),
        arm(Css::unitless(800.0), PerRow::cards(2)),
    ],
    otherwise: PerRow::cards(1),
};

// reference: card-count-ladder
const REQUEST_BACKDROP: Request = Request {
    televised: Some(PerRow::cards(4)),
    arms: &[
        arm(Css::unitless(2500.0), PerRow::cards(6)),
        arm(Css::unitless(1600.0), PerRow::cards(5)),
        arm(Css::unitless(1200.0), PerRow::cards(4)),
        arm(Css::unitless(770.0), PerRow::cards(3)),
        arm(Css::unitless(420.0), PerRow::cards(2)),
    ],
    otherwise: PerRow::cards(1),
};

// reference: card-count-ladder
const REQUEST_SMALL_BACKDROP: Request = Request {
    televised: None,
    arms: &[
        arm(Css::unitless(1600.0), PerRow::cards(8)),
        arm(Css::unitless(1400.0), PerRow::percent(14.2857142857)),
        arm(Css::unitless(1200.0), PerRow::percent(16.66666667)),
        arm(Css::unitless(1000.0), PerRow::cards(5)),
        arm(Css::unitless(800.0), PerRow::cards(4)),
        arm(Css::unitless(500.0), PerRow::percent(33.33333333)),
    ],
    otherwise: PerRow::cards(2),
};

/// A shape the reference's own switch does not name, which is every mixed one,
/// takes its `default`.
// reference: card-count-ladder
const REQUEST_MIXED: Request = Request {
    televised: None,
    arms: &[],
    otherwise: PerRow::cards(4),
};

// reference: card-count-ladder
const REQUEST_RAIL_PORTRAIT: Request = Request {
    televised: Some(PerRow::percent(15.5)),
    arms: &[
        turned(Some(Css::unitless(1700.0)), PerRow::percent(11.6)),
        turned(None, PerRow::percent(15.5)),
        arm(Css::unitless(1400.0), PerRow::percent(15.0)),
        arm(Css::unitless(1200.0), PerRow::percent(18.0)),
        arm(Css::unitless(760.0), PerRow::percent(23.0)),
        arm(Css::unitless(400.0), PerRow::percent(31.5)),
    ],
    otherwise: PerRow::percent(42.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_SQUARE: Request = Request {
    televised: Some(PerRow::percent(15.5)),
    arms: &[
        turned(Some(Css::unitless(1700.0)), PerRow::percent(11.6)),
        turned(None, PerRow::percent(15.5)),
        arm(Css::unitless(1400.0), PerRow::percent(15.0)),
        arm(Css::unitless(1200.0), PerRow::percent(18.0)),
        arm(Css::unitless(760.0), PerRow::percent(23.0)),
        arm(Css::unitless(540.0), PerRow::percent(31.5)),
    ],
    otherwise: PerRow::percent(42.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_BACKDROP: Request = Request {
    televised: Some(PerRow::percent(23.3)),
    arms: &[
        turned(Some(Css::unitless(1700.0)), PerRow::percent(18.5)),
        turned(None, PerRow::percent(23.3)),
        arm(Css::unitless(1800.0), PerRow::percent(23.5)),
        arm(Css::unitless(1400.0), PerRow::percent(30.0)),
        arm(Css::unitless(760.0), PerRow::percent(40.0)),
        arm(Css::unitless(640.0), PerRow::percent(56.0)),
    ],
    otherwise: PerRow::percent(72.0),
};

// reference: card-count-ladder
const REQUEST_RAIL_SMALL_BACKDROP: Request = Request {
    televised: Some(PerRow::percent(18.9)),
    arms: &[
        turned(Some(Css::unitless(800.0)), PerRow::percent(15.5)),
        turned(None, PerRow::percent(23.3)),
        arm(Css::unitless(540.0), PerRow::percent(30.0)),
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

/// What a card's footer takes down the page: its first line over the top
/// `.cardText-first` gives it and every line after it in the secondary size,
/// each counting its own `.cardText` padding, which the reference writes in the
/// em of the size that line is set in, and the footer's own padding where it
/// stands inside it.
// reference: card-footer
// reference: card-text
// reference: card-text-first
// reference: card-text-lines
fn written(footer: Footer, footing: Footing) -> Drawn {
    let line = |size: Length, top: Length| {
        top.plus(typeface::LINE_HEIGHT.of(size))
            .plus(space::card_text(size).bottom)
    };
    let lines = footer.written();
    if lines == 0 {
        return Drawn::ZERO;
    }
    let mut stacked = line(typeface::BODY, space::CARD_TEXT_FIRST_TOP);
    for _ in 1..lines {
        let size = typeface::SECONDARY;
        stacked = stacked.plus(line(size, space::card_text(size).top));
    }
    match footing {
        Footing::Bare => stacked.drawn(),
        Footing::Padded => space::CARD_FOOTER_PAD
            .top
            .plus(stacked)
            .plus(space::CARD_FOOTER_PAD.bottom)
            .drawn(),
    }
}

/// What a card offers on a mobile layout, which is what the section's own
/// `getCardsHtml` call sets.
// reference: card-overlay-buttons
// reference: card-footer-menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Touch {
    /// `overlayPlayButton: true`: the control that plays, whatever the item's
    /// own media type.
    Plays,
    /// Every overlay option left unset, which the reference answers with the
    /// control that plays where the item's own media type is video and the
    /// card stands off the paper.
    Unset,
    /// `overlayMoreButton: true` on a card standing off the paper, and a
    /// `cardLayout` card leaving `cardFooterAside` unset: the control that
    /// opens the item's menu.
    Menu,
    /// `overlayPlayButton: false`, and `cardFooterAside: 'none'`: neither.
    Withheld,
}

/// One `getCardsHtml` call's own card: the shape it draws at, the lines its
/// footer writes, whether `cardLayout` stands its box on the scheme's paper,
/// where `centerText` sets its footer's lines, whether its box reserves the
/// margin under itself, and what it offers under a finger.
// reference: card-box-classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Drawing {
    pub card: Card,
    pub footer: Footer,
    pub backing: Backing,
    pub footing: Footing,
    pub setting: Setting,
    pub bottom: Bottom,
    pub touch: Touch,
}

impl Drawing {
    /// The pitch one card occupies down the page, gutter included.
    pub fn row(self, room: Room) -> Drawn {
        let gutter = space::GUTTER.drawn();
        self.card
            .shape()
            .aspect()
            .of(self.card.inside(room))
            .plus(written(self.footer, self.footing))
            .plus(reserved(self.bottom, room.viewport()))
            .plus(gutter)
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
