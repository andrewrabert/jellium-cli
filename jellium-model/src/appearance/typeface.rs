//! The reference's type scale over the 16px base, and the two weights the
//! client draws.

use super::{Breakpoint, Length, Query, Ratio, Viewport};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weight {
    Regular,
    Bold,
}

// reference: type-line-height
pub const LINE_HEIGHT: Ratio = Ratio::thousandths(1350);

// reference: type-root
pub const DESKTOP_ROOT: Ratio = Ratio::thousandths(930);

// reference: type-mobile-root
pub const MOBILE_ROOT: Ratio = Ratio::thousandths(900);

// reference: type-root
pub const HEADING_1: Length = Length::em(1.8);

// reference: type-root
pub const HEADING_2: Length = Length::em(1.5);

// reference: type-root
pub const HEADING_3: Length = Length::em(1.17);

/// The root itself, which every other size is written against; one em is the
/// unit rather than a value taken from the reference, so it cites nothing.
pub const BODY: Length = Length::em(1.0);

// reference: card-text
pub const SECONDARY: Length = Length::em(0.86);

// reference: control-input
pub const FIELD: Length = Length::em(1.1);

// reference: type-button-icon
pub const BUTTON_ICON: Length = Length::em(1.36);

// reference: card-icon
pub const CARD_ICON: Length = Length::em(5.0);

/// `.filterIndicator`'s lettering, 60% of the control it sits on.
// reference: filter-indicator
pub const INDICATOR: Length = BODY.times(Ratio::thousandths(600));

/// The steps `.alphaPicker-vertical` takes as the page shortens, in the order
/// the cascade resolves them.
// reference: alpha-picker-size
const LETTERS_STEPS: [(Query, Length); 4] = [
    (
        Query::MaxHeight(Breakpoint::em(49.0)),
        BODY.times(Ratio::thousandths(940)),
    ),
    (
        Query::MaxHeight(Breakpoint::em(44.0)),
        BODY.times(Ratio::thousandths(900)),
    ),
    (
        Query::MaxHeight(Breakpoint::em(37.0)),
        BODY.times(Ratio::thousandths(820)),
    ),
    (
        Query::MaxHeight(Breakpoint::em(32.0)),
        BODY.times(Ratio::thousandths(740)),
    ),
];

/// The size the letter picker's letters draw at.
/// 94% of the body at 49em of height and shorter, 90% at 44em, 82% at 37em,
/// 74% at 32em.
// reference: alpha-picker-size
pub fn letters(viewport: Viewport) -> Length {
    let mut standing = BODY;
    for (at, size) in LETTERS_STEPS {
        if viewport.matches(at) {
            standing = size;
        }
    }
    standing
}
