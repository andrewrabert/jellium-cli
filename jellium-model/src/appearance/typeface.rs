//! The reference's type scale over the 16px base, and the two weights the
//! client draws.

use super::{Breakpoint, Css, Length, Query, Ratio, Viewport};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weight {
    Regular,
    Bold,
}

/// The line box a run of text stands in, which the reference writes either as
/// a factor of the size the run is set at or as a length of its own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Leading {
    Factor(Ratio),
    Length(Length),
}

impl Leading {
    /// The line box a run set at `size` stands in.
    pub const fn of(self, size: Length) -> Length {
        match self {
            Leading::Factor(factor) => size.times(factor),
            Leading::Length(length) => length,
        }
    }

    /// The factor to the fewest decimals that carry it, and the length in em.
    pub fn css(self) -> String {
        match self {
            Leading::Factor(factor) => super::trimmed(factor.factor()),
            Leading::Length(length) => {
                format!("{}em", super::trimmed(length.drawn().count() / super::BASE))
            }
        }
    }
}

// reference: type-line-height
pub const LINE_HEIGHT: Leading = Leading::Factor(Ratio::thousandths(1350));

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

// reference: mui-typography
pub const HEADING_1_LEADING: Leading = Leading::Factor(Ratio::thousandths(1167));

// reference: mui-typography
pub const HEADING_2_LEADING: Leading = Leading::Factor(Ratio::thousandths(1200));

// reference: mui-typography
pub const HEADING_3_LEADING: Leading = Leading::Factor(Ratio::thousandths(1167));

/// Which of the three heading levels a line is written at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    First,
    Second,
    Third,
}

impl Rank {
    /// The size the reference writes at this level.
    pub const fn size(self) -> Length {
        match self {
            Rank::First => HEADING_1,
            Rank::Second => HEADING_2,
            Rank::Third => HEADING_3,
        }
    }

    /// The line box MUI writes at it.
    pub const fn leading(self) -> Leading {
        match self {
            Rank::First => HEADING_1_LEADING,
            Rank::Second => HEADING_2_LEADING,
            Rank::Third => HEADING_3_LEADING,
        }
    }
}

// the reference writes 300 over MUI's h1 and h2 and 400 over its h3, and a
// browser resolves both against the two faces the client bundles as the
// lighter one
// reference: mui-typography
pub const HEADING_WEIGHT: Weight = Weight::Regular;

/// The root itself, which every other size is written against; one em is the
/// unit rather than a value taken from the reference, so it cites nothing.
pub const BODY: Length = Length::em(1.0);

// reference: card-text
pub const SECONDARY: Length = Length::em(0.86);

// reference: control-input
// reference: control-select
pub const FIELD: Length = Length::em(1.1);

/// `.selectArrow`, the chevron laid over a select's trailing edge.
// reference: control-select-arrow-glyph
pub const SELECT_ARROW: Length = Length::em(1.7);

/// `.checkboxIcon`, which the reference writes smaller than the box it stands
/// in.
// reference: control-checkbox-icon
pub const CHECKBOX_MARK: Length = Length::em(1.6);

// reference: type-button-icon
pub const BUTTON_ICON: Length = Length::em(1.36);

/// The glyph an icon control draws, which is larger than the one a labelled
/// button carries.
// reference: control-icon-glyph
pub const ICON_BUTTON: Length = Length::em(1.669_565_2);

// reference: card-icon
pub const CARD_ICON: Length = Length::em(5.0);

/// `.nowPlayingBarText`, which the bar writes smaller than the body.
// reference: bar-text
pub const BAR_TEXT: Length = BODY.times(Ratio::thousandths(920));

/// `.mediaButton`'s glyph, which the bar draws larger than a page's.
// reference: bar-button
pub const BAR_ICON: Length = ICON_BUTTON.times(Ratio::thousandths(1200));

/// `.filterIndicator`'s lettering, 60% of the control it sits on.
// reference: filter-indicator
pub const INDICATOR: Length = BODY.times(Ratio::thousandths(600));

/// `.listItemIcon`, which the reference writes larger than the body of the row
/// it stands before.
// reference: list-icon
pub const LIST_ICON: Length = BODY.times(Ratio::thousandths(1430));

/// The line box `.layout-desktop` gives `.listItemBodyText`, which the
/// reference writes tighter than the page's own.
// reference: list-body-text-desktop
pub const LIST_LEADING: Leading = Leading::Factor(Ratio::thousandths(1200));

/// The size MUI sets its own `body2` variant at, in the css pixels it writes
/// before `pxToRem` puts that size over the 16px base.
// reference: mui-typography
const MUI_BODY_2: Css = Css::unitless(14.0);

/// MUI's own `body2`, which a table cell and an alert are written in.
// reference: mui-typography
pub const BODY_2: Length = MUI_BODY_2.length();

// reference: mui-typography
pub const BODY_2_LEADING: Leading = Leading::Factor(Ratio::thousandths(1430));

/// The line box MUI writes inside a filled field, which it sets as a length
/// rather than as a factor.
// reference: mui-input-base
pub const FILLED_LEADING: Leading = Leading::Length(Length::em(1.4375));

/// The size a filled field's own label shrinks to.
// reference: mui-input-label
pub const FILLED_LABEL: Length = BODY.times(Ratio::thousandths(750));

/// The glyph size MUI writes before `pxToRem`.
// reference: mui-svg-icon
const MUI_GLYPH: Css = Css::unitless(24.0);

/// MUI's own medium glyph, which a select's chevron and a checkbox's box are
/// drawn at.
// reference: mui-svg-icon
pub const CONTROL_GLYPH: Length = MUI_GLYPH.length();

/// An alert's own glyph, which MUI writes as the bare number the DOM reads as
/// css pixels.
// reference: mui-alert-parts
const MUI_ALERT_GLYPH: Css = Css::unitless(22.0);

/// The glyph an alert stands before its sentence.
// reference: mui-alert-parts
pub const ALERT_GLYPH: Length = MUI_ALERT_GLYPH.length();

/// The size the reference writes over MUI's own large button, which is the
/// page's own body.
// reference: mui-theme-button
pub const CONTAINED: Length = Length::em(1.0);

/// The weight it writes over it.
// reference: mui-theme-button
pub const CONTAINED_WEIGHT: Weight = Weight::Bold;

/// The line box MUI writes for a head cell, in the css pixels it writes before
/// `pxToRem`.
// reference: mui-table-cell
const MUI_HEAD_LEADING: Css = Css::unitless(24.0);

/// The line box a head cell's lettering stands in, which MUI writes as a
/// length rather than as a factor.
// reference: mui-table-cell
pub const TABLE_HEAD_LEADING: Leading = Leading::Length(MUI_HEAD_LEADING.length());

/// MRT writes `bold` over MUI's own 500, and a browser resolves that against
/// the two faces the client bundles as the bolder one.
// reference: table-head-cell
pub const TABLE_HEAD_WEIGHT: Weight = Weight::Bold;

/// The size MUI sets a small toggle at, in the css pixels it writes before
/// `pxToRem`.
// reference: mui-toggle-button
const MUI_TOGGLE: Css = Css::unitless(13.0);

/// One segment of the top toolbar's group, which MUI writes smaller than its
/// own `button` variant.
// reference: mui-toggle-button
pub const TOGGLE: Length = MUI_TOGGLE.length();

/// `typography.button`'s own line box; the reference takes that variant's
/// uppercasing off and leaves its box.
// reference: mui-typography
// reference: mui-theme-typography
pub const BUTTON_LEADING: Leading = Leading::Factor(Ratio::thousandths(1750));

/// `.toast`'s own lettering, which the reference writes larger than the body.
// reference: toast-face
pub const TOAST: Length = Length::em(1.1);

/// `.detailButton-icon`.
// reference: detail-button-icon
pub const DETAIL_ICON: Length = Length::em(1.6);

/// `.searchfields-icon`, the one glyph the search field stands beside.
// reference: search-icon
pub const SEARCH_ICON: Length = Length::em(2.0);

// the reference writes 600, and a browser resolves that against the two faces
// the client bundles as the bolder one
// reference: control-tab
pub const TAB_WEIGHT: Weight = Weight::Bold;

/// The line box a tab's lettering stands in, which the reference writes
/// tighter than the page's own.
// reference: control-tab
pub const TAB_LEADING: Leading = Leading::Factor(Ratio::thousandths(1250));

// reference: tab-size
const TAB_NARROW: Length = BODY.times(Ratio::thousandths(835));

// reference: tab-size
const TAB_NARROW_AT: Query = Query::MaxWidth(Breakpoint::em(100.0));

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

/// The lettering a tab is written in.
// 83.5% of the body on a page no wider than 100em, and the body above it
// reference: tab-size
pub fn tab(viewport: Viewport) -> Length {
    match viewport.matches(TAB_NARROW_AT) {
        true => TAB_NARROW,
        false => BODY,
    }
}

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
