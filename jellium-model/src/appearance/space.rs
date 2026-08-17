//! Every padding, gap, radius, border and shadow the client draws, and the page
//! padding that is a share of the viewport.

use chrono::TimeDelta;

use super::scheme::{self, Color};
use super::typeface;
use super::{
    Breakpoint, Canvas, Css, Drawn, Length, Letters, Orientation, Query, Ratio, Share, Viewport,
};

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

/// The width a page lays its content inside, with the viewport that width was
/// measured in, so a card is never a share of anything wider than the box it
/// is drawn in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Room {
    viewport: Viewport,
    width: Drawn,
}

impl Room {
    /// The canvas less `page_side` at each edge.
    // reference: page-side
    pub fn content(viewport: Viewport) -> Room {
        let side = page_side(viewport.canvas());
        Room {
            viewport,
            width: Drawn::of((viewport.canvas().width().count() - side.count() * 2.0).max(0.0)),
        }
    }

    /// `content` less the letter picker's own reserve.
    /// `content` itself where the page is too short to draw the picker.
    // reference: alpha-picker-reserve
    // reference: letter-jump
    pub fn lettered(viewport: Viewport) -> Room {
        let content = Room::content(viewport);
        match viewport.letters() {
            Letters::Hidden => content,
            Letters::Shown => Room {
                viewport,
                width: Drawn::of(
                    (content.width.count() - LETTERS_RESERVE.of(content.width).count()).max(0.0),
                ),
            },
        }
    }

    pub fn viewport(self) -> Viewport {
        self.viewport
    }

    pub fn width(self) -> Drawn {
        self.width
    }
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
// reference: bar-art
pub const BAR_ART: Length = Length::em(4.2);

/// `.nowPlayingImage`'s share of the bar's height.
// reference: bar-art
pub const BAR_ART_HEIGHT: Share = Share::per_ten_thousand(7000);

/// `.nowPlayingBarText`'s own margins.
// reference: bar-text
pub const BAR_TEXT_MARGIN: Padding = Padding {
    top: Length::em(0.0),
    right: Length::em(1.0),
    bottom: Length::em(0.0),
    left: Length::em(0.5),
};

/// `.nowPlayingBarRight`'s margin against the edge of the bar.
// reference: bar-right
pub const BAR_RIGHT_MARGIN: Length = Length::em(0.5);

/// `.nowPlayingBarCurrentTime`'s inset from the controls it follows.
// reference: bar-time
pub const BAR_TIME_INSET: Length = Length::em(1.5);

/// `.nowPlayingBarVolumeSliderContainer`'s margin against what follows it.
// reference: bar-volume
pub const BAR_VOLUME_MARGIN: Length = Length::em(2.0);

/// That slider's own width.
// reference: bar-markup-right
pub const BAR_VOLUME_SLIDER: Length = Length::em(9.0);

/// `.nowPlayingBarInfoContainer`'s share of the bar at rest.
// reference: bar-info
const BAR_INFO: Share = Share::per_ten_thousand(4000);

/// Where the bar drops the control that favourites what is playing.
// reference: bar-shed-rating
pub const BAR_RATING_AT: Query = Query::MaxWidth(Breakpoint::em(70.0));

/// Where it drops the repeat and shuffle controls.
// reference: bar-shed-queue
pub const BAR_QUEUE_AT: Query = Query::MaxWidth(Breakpoint::em(66.0));

/// Where it drops the reading and the control that stops playback.
// reference: bar-shed-time
pub const BAR_TIME_AT: Query = Query::MaxWidth(Breakpoint::em(80.0));

/// Where the whole centred group goes.
// reference: bar-shed-centre
pub const BAR_CENTRE_AT: Query = Query::MaxWidth(Breakpoint::em(56.0));

/// Where the volume slider goes.
// reference: bar-shed-volume
pub const BAR_VOLUME_AT: Query = Query::MaxWidth(Breakpoint::em(60.0));

/// Where the control that mutes goes.
// reference: bar-shed-mute
pub const BAR_MUTE_AT: Query = Query::MaxWidth(Breakpoint::em(24.0));

/// The steps `.nowPlayingBarInfoContainer` takes as the page narrows, in the
/// order the cascade resolves them: the rule that sheds the reading is written
/// before the one that sheds the volume slider, so the whole of the bar stands
/// where both apply.
const BAR_INFO_STEPS: [(Query, Share); 2] = [
    (BAR_TIME_AT, Share::per_ten_thousand(4500)),
    (BAR_VOLUME_AT, Share::WHOLE),
];

/// `.nowPlayingBarInfoContainer`'s share of the bar: two fifths, widening to
/// nine twentieths and then to the whole of it as the page narrows.
// reference: bar-info
pub fn bar_info(viewport: Viewport) -> Share {
    stepped(viewport, BAR_INFO, &BAR_INFO_STEPS)
}

/// The slider's own track, `.mdl-slider-background-flex`, rather than the
/// `::-ms-track` pseudo-element beside it that no browser here renders.
// reference: slider-track
pub const SLIDER_TRACK: Length = Length::em(0.2);

/// The slider's thumb, as both engines this client runs on are given it: the
/// WebKit thumb, the Gecko thumb and their hover and focus states all declare
/// this, and the `::-ms-thumb` arm beside them reaches neither engine.
// reference: slider-thumb
pub const SLIDER_THUMB: Length = Length::em(1.08);

/// Written in css pixels, which do not scale with the root and so cross to the
/// canvas through `Css::drawn`.
// reference: slider-marker
pub const SLIDER_MARKER_WIDTH: Css = Css::of(2.0);

// reference: slider-marker
pub const SLIDER_MARKER_HEIGHT: Css = Css::of(12.0);

/// `.filterIndicator`'s circle, 1.8 of its own lettering.
// reference: filter-indicator
pub const INDICATOR: Length = typeface::INDICATOR.times(Ratio::thousandths(1800));

/// The indicator's inset from the top and the trailing edge of the control it
/// sits on, which the reference writes in css pixels and no type scale moves.
// reference: filter-indicator
pub const INDICATOR_INSET: Css = Css::of(2.0);

/// `.alphaPicker-fixed`'s inset from the foot of the page.
// reference: alpha-picker
pub const LETTERS_BOTTOM: Length = Length::em(5.5);

/// The same inset on a short page.
// reference: alpha-picker-size
pub const LETTERS_BOTTOM_SHORT: Length = Length::em(5.0);

// reference: alpha-picker-size
pub const LETTERS_SHORT: Query = Query::MaxHeight(Breakpoint::em(50.0));

/// `.alphaPicker-fixed-right`'s inset from the edge of the page.
// reference: alpha-picker-right
pub const LETTERS_RIGHT: Length = Length::em(0.4);

/// The same inset on a wide page.
// reference: alpha-picker-right
pub const LETTERS_RIGHT_ROOMY: Length = Length::em(1.0);

// reference: alpha-picker-right
pub const LETTERS_ROOMY: Query = Query::MinWidth(Breakpoint::em(62.5));

/// The share of the page `.padded-right-withalphapicker` keeps clear for the
/// letter picker.
// reference: alpha-picker-reserve
pub const LETTERS_RESERVE: Share = Share::per_ten_thousand(750);

/// `.searchFieldsInner`, which the page centres in what it is given.
// reference: search-field
pub const SEARCH_FIELD: Length = Length::em(60.0);

/// `.searchfields-icon`'s gap to the field it stands before.
// reference: search-icon
pub const SEARCH_ICON_GAP: Length = Length::em(0.25);

/// The same icon's drop from the field's own baseline.
// reference: search-icon
pub const SEARCH_ICON_DROP: Length = Length::em(0.1);

/// `.centerMessage`'s share of the page.
// reference: center-message
pub const CENTER_MESSAGE: Share = Share::per_ten_thousand(3000);

/// Its padding above and below.
// reference: center-message
pub const CENTER_MESSAGE_PAD: Length = Length::em(5.0);

/// One suggestion's padding, which is what spaces the column.
// reference: search-suggestions
pub const SUGGESTION_PAD: Padding = Padding {
    top: Length::em(0.5),
    right: Length::em(1.0),
    bottom: Length::em(0.5),
    left: Length::em(1.0),
};

// reference: control-icon-button
pub const ICON_MARGIN: Length = Length::em(0.29);

pub const ICON_GAP: Length = ICON_MARGIN.plus(ICON_MARGIN);

/// `.paper-icon-button-light`'s padding, which is what rounds it into a disc.
// reference: control-icon-button
pub const ICON_BUTTON_PAD: Length = Length::em(0.556);

// reference: progress-bar
pub const PROGRESS: Length = Length::em(0.28);

// reference: guide-row
pub const GUIDE_ROW: Length = Length::em(4.42);

/// `.itemBackdrop`'s height.
// reference: detail-backdrop — 40vh
pub const BACKDROP: Share = Share::units(40.0);

/// What the stacked arrangement leaves above the backdrop.
// reference: detail-backdrop
pub const BACKDROP_TOP: Length = Length::em(3.0);

/// `.detailRibbon`'s height, which is also how far it stands over the
/// backdrop.
// reference: detail-ribbon
pub const RIBBON: Length = Length::em(7.2);

/// The room the ribbon and the page's own content leave for the poster beside
/// them.
// reference: detail-ribbon — 32.45vw
// reference: detail-content
pub const RIBBON_INSET: Share = Share::units(32.45);

/// The ribbon's padding in the stacked arrangement.
// reference: detail-ribbon
pub const RIBBON_PAD: Padding = Padding {
    top: Length::em(0.5),
    right: Length::em(0.0),
    bottom: Length::em(0.5),
    left: Length::em(0.0),
};

/// The share of the page the stacked arrangement insets by.
// reference: detail-ribbon
// reference: detail-content
pub const DETAIL_SIDE: Share = Share::per_ten_thousand(500);

/// The trailing inset the ribbon arrangement leaves.
// reference: detail-content
pub const DETAIL_TRAIL: Share = Share::per_ten_thousand(200);

/// The poster's width beside the ribbon.
// reference: detail-poster — 25vw
pub const DETAIL_POSTER: Share = Share::units(25.0);

/// Its width over the backdrop in the stacked arrangement.
// reference: detail-poster — 30vw
pub const DETAIL_POSTER_STACKED: Share = Share::units(30.0);

/// How far the poster rises over the ribbon, which is the ribbon's own height
/// and four fifths again.
// reference: detail-poster
pub const DETAIL_POSTER_RISE: Length = RIBBON.times(Ratio::thousandths(1800));

/// The poster's inset from the leading edge beside the ribbon.
// reference: detail-poster
pub const DETAIL_POSTER_INSET: Share = PAGE_SIDE;

/// The gap above and below the row of detail buttons.
// reference: detail-buttons
pub const DETAIL_BUTTONS: Length = Length::em(1.0);

/// The gap under that row in the stacked arrangement.
// reference: detail-centred
pub const DETAIL_BUTTONS_BOTTOM: Length = Length::em(0.5);

/// One detail button's padding.
// reference: detail-button
pub const DETAIL_BUTTON_PAD: Padding = Padding {
    top: Length::em(0.7),
    right: Length::em(0.7),
    bottom: Length::em(0.7),
    left: Length::em(0.7),
};

/// The steps `.detailButton`'s sides take as the page widens, in the order the
/// cascade resolves them.
// reference: detail-button-pad
const DETAIL_BUTTON_SIDES: [(Query, Length); 3] = [
    (Query::MinWidth(Breakpoint::em(29.0)), Length::em(0.75)),
    (Query::MinWidth(Breakpoint::em(32.0)), Length::em(0.8)),
    (Query::MinWidth(Breakpoint::em(35.0)), Length::em(0.85)),
];

/// The gap under the page's own head.
// reference: detail-secondary
pub const DETAIL_BODY_TOP: Length = Length::em(1.25);

/// The same gap in the stacked arrangement.
// reference: detail-secondary
pub const DETAIL_BODY_TOP_STACKED: Length = Length::em(1.0);

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

/// The scrim standing above the bottom panel's own controls.
// reference: osd-bottom
pub const OSD_SCRIM: Length = Length::em(7.5);

/// The panel's own footing.
// reference: osd-bottom
pub const OSD_BOTTOM: Length = Length::em(1.75);

/// `.osdHeader`, the scrim standing over the top of the video.
// reference: osd-header
pub const OSD_HEADER: Length = Length::em(7.5);

/// The controls inside that header.
// reference: osd-header-buttons
pub const OSD_HEADER_TOP: Length = Length::em(3.5);

/// `.osdControls`' inset from the edges of the panel.
// reference: osd-controls
pub const OSD_CONTROLS: Length = Length::em(0.8);

/// The gap under one line of the panel's text.
// reference: osd-text
pub const OSD_TEXT_GAP: Length = Length::em(0.7);

/// That text's own inset.
// reference: osd-text
pub const OSD_TEXT_INSET: Length = Length::em(0.5);

/// The secondary line's deeper inset.
// reference: osd-secondary
pub const OSD_SECONDARY_INSET: Length = Length::em(0.6);

/// The gap between the slider and the clock at either end of it.
// reference: osd-markup
pub const OSD_TIME_GAP: Length = Length::em(0.25);

/// The slider's own room in that row.
// reference: osd-markup
pub const OSD_SLIDER_PAD: Padding = Padding {
    top: Length::em(0.5),
    right: Length::em(0.0),
    bottom: Length::em(0.25),
    left: Length::em(0.0),
};

/// The gap above the row of controls.
// reference: osd-controls
pub const OSD_BUTTONS_TOP: Length = Length::em(0.25);

/// `.osdTimeText`'s gap to the controls it follows.
// reference: osd-time
pub const OSD_ENDS_GAP: Length = Length::em(1.0);

/// `.osdTitle`'s gap to whatever follows it on its line.
// reference: osd-title
pub const OSD_TITLE_GAP: Length = Length::em(1.0);

/// The gap between the status glyph and the word beside it.
// reference: osd-status
pub const OSD_STATUS_GAP: Length = Length::em(0.125);

/// `.volumeButtons`' own margins, leading and trailing.
// reference: osd-volume
pub const OSD_VOLUME_MARGIN: Padding = Padding {
    top: Length::em(0.0),
    right: Length::em(1.0),
    bottom: Length::em(0.0),
    left: ICON_MARGIN,
};

/// `.osdVolumeSliderContainer`.
// reference: osd-volume
pub const OSD_VOLUME_SLIDER: Length = Length::em(9.0);

/// Where the panel drops the secondary media line.
// reference: osd-shed-rating
pub const OSD_INFO_AT: Query = Query::MaxWidth(Breakpoint::em(30.0));

/// Where its controls stand shoulder to shoulder.
// reference: osd-shed-margin
pub const OSD_MARGINS_AT: Query = Query::MaxWidth(Breakpoint::em(33.75));

/// Where the volume control and the status word go.
// reference: osd-shed-volume
pub const OSD_VOLUME_AT: Query = Query::MaxWidth(Breakpoint::em(43.0));

/// Where the two seek controls go.
// reference: osd-shed-seek
pub const OSD_SEEK_AT: Query = Query::MaxWidth(Breakpoint::em(50.0));

/// Where the ending time goes.
// reference: osd-shed-ends
pub const OSD_ENDS_AT: Query = Query::MaxWidth(Breakpoint::em(75.0));

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

/// The sides of that padding as the page widens.
// reference: detail-button-pad
pub fn detail_button_side(viewport: Viewport) -> Drawn {
    let mut standing = DETAIL_BUTTON_PAD.right;
    for (at, side) in DETAIL_BUTTON_SIDES {
        if viewport.matches(at) {
            standing = side;
        }
    }
    standing.drawn()
}

/// The page padding, which is a share of the page rather than an em, and a
/// share of the page the canvas draws because it is a length the layout uses.
// reference: page-side
pub fn page_side(canvas: Canvas) -> Drawn {
    PAGE_SIDE.of(canvas.width())
}

/// The letter picker's inset from the foot of the page at this viewport.
// reference: alpha-picker-size
pub fn letters_bottom(viewport: Viewport) -> Drawn {
    match viewport.matches(LETTERS_SHORT) {
        true => LETTERS_BOTTOM_SHORT.drawn(),
        false => LETTERS_BOTTOM.drawn(),
    }
}

/// Its inset from the edge of the page at this viewport.
// reference: alpha-picker-right
pub fn letters_right(viewport: Viewport) -> Drawn {
    match viewport.matches(LETTERS_ROOMY) {
        true => LETTERS_RIGHT_ROOMY.drawn(),
        false => LETTERS_RIGHT.drawn(),
    }
}
