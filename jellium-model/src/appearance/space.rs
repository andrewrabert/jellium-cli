//! Every padding, gap, radius, border and shadow the client draws, and the page
//! padding that is a share of the viewport.

use chrono::TimeDelta;

use super::scheme::{self, Color};
use super::typeface;
use super::{Breakpoint, Canvas, Css, Drawn, Length, Orientation, Query, Share, Viewport};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub drop: Length,
    pub blur: Length,
    pub spread: Length,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slot {
    pub width: Length,
    pub height: Length,
}

/// The margin a `.cardBox` carries on every side.
// reference: card-box
pub const CARD_MARGIN: Length = Length::em(0.6);

pub const GUTTER: Length = CARD_MARGIN.plus(CARD_MARGIN);

// reference: card-box-bottom
pub const CARD_BOTTOM: Length = Length::em(1.8);

// reference: card-box-bottom
pub const CARD_BOTTOM_NARROW: Length = Length::em(1.2);

// reference: card-box-bottom
pub const CARD_BOTTOM_AT: Query = Query::MaxWidth(Breakpoint::em(50.0));

// reference: control-button
pub const CONTROL_MARGIN: Length = Length::em(0.3);

pub const CONTROL_GAP: Length = CONTROL_MARGIN.plus(CONTROL_MARGIN);

// reference: control-button-block
pub const BLOCK_MARGIN: Length = Length::em(0.25);

pub const BLOCK_GAP: Length = BLOCK_MARGIN.plus(BLOCK_MARGIN);

// reference: control-field
pub const FIELD_GAP: Length = Length::em(1.8);

// reference: section-title
pub const SECTION_GAP: Length = Length::em(1.25);

// reference: section-title-cards
pub const SECTION_TITLE_TOP: Length = Length::em(1.25);

// reference: section-title-cards
pub const SECTION_TITLE_BOTTOM: Length = Length::em(0.2);

/// The page's own padding, `.padded-top` and `.padded-bottom`, rather than the
/// `1em` standing inside a button's `padding: 0.9em 1em`.
// reference: page-padded
pub const PAD: Length = Length::em(1.0);

// reference: login-disclaimer
pub const DISCLAIMER_GAP: Length = Length::em(2.0);

// reference: page-bottom
pub const PAGE_BOTTOM: Length = Length::em(5.0);

// reference: page-standalone
pub const PAGE_TOP: Length = Length::em(4.5);

// reference: page-side
pub const PAGE_SIDE: Share = Share::per_ten_thousand(330);

// reference: page-bottom
pub const FORM_WIDTH: Length = Length::em(54.0);

// reference: page-bottom
pub const FORM_WIDTH_AT: Query = Query::MinWidth(Breakpoint::em(50.0));

// reference: control-button
pub const BUTTON_PAD: Padding = Padding {
    top: Length::em(0.9),
    right: Length::em(1.0),
    bottom: Length::em(0.9),
    left: Length::em(1.0),
};

// reference: control-input
pub const INPUT_PAD: Padding = Padding {
    top: Length::em(0.4),
    right: Length::em(0.25),
    bottom: Length::em(0.4),
    left: Length::em(0.25),
};

// reference: card-text
pub const CARD_TEXT_PAD: Padding = Padding {
    top: Length::em(0.06),
    right: Length::em(0.5),
    bottom: Length::em(0.06),
    left: Length::em(0.5),
};

// reference: card-footer
pub const CARD_FOOTER_PAD: Padding = Padding {
    top: Length::em(0.3),
    right: Length::em(0.3),
    bottom: Length::em(0.5),
    left: Length::em(0.3),
};

// reference: control-button
pub const RADIUS: Length = Length::em(0.2);

// reference: scheme-input
pub const INPUT_BORDER: Length = Length::em(0.16);

// reference: card-shadow
pub const CARD_SHADOW: Shadow = Shadow {
    drop: Length::em(0.0725),
    blur: Length::em(0.29),
    spread: Length::em(0.0),
    color: scheme::SHADOW,
};

// reference: header-logo — the slot's width
// reference: header-title — its height, which the title rule carries
pub const LOGO: Slot = Slot {
    width: Length::em(13.2),
    height: Length::em(1.7),
};

// reference: header-top
pub const HEADER_PAD: Length = Length::em(0.54);

// reference: list-body
pub const LIST_BODY_PAD: Padding = Padding {
    top: Length::em(0.85),
    right: Length::em(0.75),
    bottom: Length::em(0.85),
    left: Length::em(0.75),
};

/// A list row: a body line over a secondary line, inside the body padding.
pub const LIST_ROW: Length = LIST_BODY_PAD
    .top
    .plus(LIST_BODY_PAD.bottom)
    .plus(typeface::BODY.times(typeface::LINE_HEIGHT))
    .plus(typeface::SECONDARY.times(typeface::LINE_HEIGHT));

/// The now-playing bar, which stands as tall as this.
// reference: bar-top
pub const BAR: Length = Length::em(4.2);

/// The now-playing bar's cover art, which is as wide as the bar is tall.
// reference: bar-image
pub const BAR_ART: Length = Length::em(4.2);

/// The slider's own track, `.mdl-slider-background-flex`, rather than the
/// `::-ms-track` pseudo-element beside it that no browser here renders.
// reference: slider-track
pub const SLIDER_TRACK: Length = Length::em(0.2);

/// The slider's thumb, as both engines this client runs on are given it: the
/// WebKit thumb, the Gecko thumb and their hover and focus states all declare
/// this, and the `::-ms-thumb` arm beside them reaches neither engine.
// reference: slider-thumb
pub const SLIDER_THUMB: Length = Length::em(1.08);

/// The one appearance value the reference writes in css pixels, which do not
/// scale with the root and so cross to the canvas through `Css::drawn`.
// reference: slider-marker
pub const SLIDER_MARKER_WIDTH: Css = Css::of(2.0);

// reference: slider-marker
pub const SLIDER_MARKER_HEIGHT: Css = Css::of(12.0);

// reference: control-icon-button
pub const ICON_MARGIN: Length = Length::em(0.29);

pub const ICON_GAP: Length = ICON_MARGIN.plus(ICON_MARGIN);

// reference: progress-bar
pub const PROGRESS: Length = Length::em(0.28);

// reference: guide-row
pub const GUIDE_ROW: Length = Length::em(4.42);

/// The last step of a viewport-unit ladder the page reaches, and `base` where
/// it reaches none, which is the order the cascade resolves them in.
fn stepped(viewport: Viewport, base: Share, steps: &[(Query, Share)]) -> Share {
    let mut standing = base;
    for (at, share) in steps {
        if viewport.matches(*at) {
            standing = *share;
        }
    }
    standing
}

// reference: guide-strip
const GUIDE_STRIP: Share = Share::units(1800.0);

// reference: guide-strip
const GUIDE_STRIP_STEPS: [(Query, Share); 3] = [
    (Query::MinWidth(Breakpoint::em(37.5)), Share::units(1400.0)),
    (Query::MinWidth(Breakpoint::em(50.0)), Share::units(1200.0)),
    (Query::MinWidth(Breakpoint::em(80.0)), Share::units(810.0)),
];

// reference: guide-channel
const GUIDE_CHANNEL: Share = Share::units(24.0);

// reference: guide-channel
const GUIDE_CHANNEL_STEPS: [(Query, Share); 4] = [
    (Query::MinWidth(Breakpoint::em(31.25)), Share::units(16.0)),
    (Query::MinWidth(Breakpoint::em(37.5)), Share::units(16.0)),
    (Query::MinWidth(Breakpoint::em(50.0)), Share::units(14.0)),
    (Query::MinWidth(Breakpoint::em(80.0)), Share::units(12.0)),
];

/// The stretch of time the guide's strip spans.
// reference: guide-strip
const GUIDE_SPAN: TimeDelta = TimeDelta::days(1);

/// The guide's program strip, which is 1800vw of the page and steps to 1400vw,
/// 1200vw and 810vw.
// reference: guide-strip
pub fn guide_strip(viewport: Viewport) -> Drawn {
    stepped(viewport, GUIDE_STRIP, &GUIDE_STRIP_STEPS).of(viewport.canvas().width())
}

/// One minute of the guide, which is its strip over the day it spans.
// reference: guide-strip
pub fn guide_minute(viewport: Viewport) -> Drawn {
    Drawn::of(guide_strip(viewport).count() / GUIDE_SPAN.num_minutes() as f32)
}

/// The guide's channel column, 24vw stepping to 16vw, 14vw and 12vw.
// reference: guide-channel
pub fn guide_channel(viewport: Viewport) -> Drawn {
    stepped(viewport, GUIDE_CHANNEL, &GUIDE_CHANNEL_STEPS).of(viewport.canvas().width())
}

/// The page a tall dialog leaves above itself.
// reference: filter-dialog
const FILTER_TOP: Share = Share::per_ten_thousand(1000);

/// The page a tall dialog leaves below itself.
// reference: filter-dialog
const FILTER_BOTTOM: Share = Share::per_ten_thousand(2500);

/// The page a dialog leaves at each end of a page that is not tall.
// reference: filter-dialog
const FILTER_END: Share = Share::per_ten_thousand(500);

/// The page a dialog takes the first pair of ends on.
// reference: filter-dialog
const FILTER_TALL: Query = Query::MinHeight(Breakpoint::pixels(600));

/// The filter surface stands as tall as the reference's own `.dynamicFilterDialog`:
/// the page less a tenth at the top and a quarter at the bottom where
/// `FILTER_TALL` matches, and less a twentieth at each end where it does not.
// reference: filter-dialog
pub fn filter_surface(viewport: Viewport) -> Drawn {
    let height = viewport.canvas().height();
    let (top, bottom) = match viewport.matches(FILTER_TALL) {
        true => (FILTER_TOP, FILTER_BOTTOM),
        false => (FILTER_END, FILTER_END),
    };
    Drawn::of(height.count() - top.of(height).count() - bottom.of(height).count())
}

/// The frame the scrub preview draws in, as tall as the page allows it.
// reference: osd-preview
const PREVIEW: Share = Share::units(20.0);

/// A portrait page gives the frame a share of its width instead.
// reference: osd-preview
const PREVIEW_PORTRAIT: Share = Share::units(30.0);

/// A landscape page no taller than this gives the frame a larger share of its
/// height.
// reference: osd-preview
const PREVIEW_SHORT: Query = Query::MaxHeight(Breakpoint::em(50.0));

// reference: osd-preview
const PREVIEW_SHORT_SIDE: Share = Share::units(30.0);

/// The scrub preview's frame, `.chapterThumb`, which is a share of the viewport
/// rather than a card: 20vh, 30vw in portrait, and 30vh in a landscape page no
/// taller than 50em. It is a page measurement because it is both drawn and
/// asked for: the frame draws at `preview(viewport).drawn(viewport.band())` and
/// the tile is asked for at `Fill::of(preview(viewport))`.
// reference: osd-preview
pub fn preview(viewport: Viewport) -> Css {
    match (viewport.orientation(), viewport.matches(PREVIEW_SHORT)) {
        (Orientation::Portrait, _) => PREVIEW_PORTRAIT.of(viewport.width()),
        (Orientation::Landscape, true) => PREVIEW_SHORT_SIDE.of(viewport.height()),
        (Orientation::Landscape, false) => PREVIEW.of(viewport.height()),
    }
}

/// The page padding, which is a share of the page rather than an em, and a
/// share of the page the canvas draws because it is a length the layout uses.
// reference: page-side
pub fn page_side(canvas: Canvas) -> Drawn {
    PAGE_SIDE.of(canvas.width())
}
