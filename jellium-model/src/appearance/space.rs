//! Every padding, gap, radius, border and shadow the client draws, and the page
//! padding that is a share of the viewport. A construct carrying both a measure
//! and a colour is named the same here and in `scheme`; the module names which
//! of the two a constant is.

use chrono::TimeDelta;

use super::scheme::{self, Color};
use super::typeface;
use super::{
    Across, Breakpoint, Canvas, Cap, Columns, Css, Drawn, Layout, Length, Letters, Orientation,
    Query, Ratio, Share, Viewport,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

/// A padding written in css pixels, which is how MUI writes its own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inset {
    pub top: Css,
    pub right: Css,
    pub bottom: Css,
    pub left: Css,
}

/// What one `Grid item` takes of its row at each of MUI's own breakpoints, a
/// width carrying up from the key that names it until another key names one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub xs: Columns,
    pub sm: Option<Columns>,
    pub md: Option<Columns>,
    pub lg: Option<Columns>,
    pub xl: Option<Columns>,
}

impl Cell {
    /// The widest key the page reaches, and `xs` where it reaches none.
    pub fn columns(self, viewport: Viewport) -> Columns {
        for (at, named) in [
            (EXTRA_AT, self.xl),
            (LARGE_AT, self.lg),
            (MEDIUM_AT, self.md),
            (SMALL_AT, self.sm),
        ] {
            if viewport.matches(at)
                && let Some(columns) = named
            {
                return columns;
            }
        }
        self.xs
    }

    pub fn across(self, viewport: Viewport) -> Across {
        self.columns(viewport).across()
    }
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

/// One element drawn over the foot of another, which is the negative margin
/// and the relative offset the reference writes beside it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overlap {
    /// How far the covering element's own top stands over the covered
    /// element's foot.
    pub raised: Length,
    /// How much of its own height the covering element hands back to whatever
    /// follows the pair.
    pub shed: Length,
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

    /// The canvas less the drawer where the page is wide enough to stand one
    /// beside the content, and less `.content-primary`'s own side padding at
    /// each edge.
    // reference: dashboard-frame
    // reference: dashboard-content-side
    pub fn dashboard(viewport: Viewport) -> Room {
        let beside = match viewport.matches(DRAWER_BESIDE_AT) {
            true => DRAWER.drawn(viewport.layout()).count(),
            false => 0.0,
        };
        let side = DASHBOARD_SIDE.drawn().count();
        Room {
            viewport,
            width: Drawn::of((viewport.canvas().width().count() - beside - side * 2.0).max(0.0)),
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
const CARD_MARGIN: Length = Length::em(0.6);

pub const GUTTER: Length = CARD_MARGIN.abutting(CARD_MARGIN);

// reference: card-box-bottom
pub const CARD_BOTTOM: Length = Length::em(1.8);

// reference: card-box-bottom
pub const CARD_BOTTOM_NARROW: Length = Length::em(1.2);

// reference: card-box-bottom
pub const CARD_BOTTOM_AT: Query = Query::MaxWidth(Breakpoint::em(50.0));

// reference: control-button
const CONTROL_MARGIN: Length = Length::em(0.3);

pub const CONTROL_GAP: Length = CONTROL_MARGIN.abutting(CONTROL_MARGIN);

// reference: control-button-block
const BLOCK_MARGIN: Length = Length::em(0.25);

/// Two `.block` controls are block-level siblings in normal flow.
pub const BLOCK_GAP: Length = BLOCK_MARGIN.collapsing(BLOCK_MARGIN);

// reference: control-field
// reference: control-select-container
// reference: control-checkbox-container
pub const FIELD_GAP: Length = Length::em(1.8);

/// `.inputLabel`'s own margin under itself.
// reference: control-input-label
// reference: control-select-label
pub const LABEL_GAP: Length = Length::em(0.25);

/// `.fieldDescription`'s own margin over itself.
// reference: control-field-description
// reference: control-select-description
pub const DESCRIPTION_GAP: Length = Length::em(0.25);

/// `.emby-select`'s own padding, written in the em of the 110% it is set in,
/// its trailing side the reserve the chevron is laid over.
// reference: control-select
pub const SELECT_PAD: Padding = Padding {
    top: typeface::FIELD.times(Ratio::thousandths(500)),
    right: typeface::FIELD.times(Ratio::thousandths(1900)),
    bottom: typeface::FIELD.times(Ratio::thousandths(500)),
    left: typeface::FIELD.times(Ratio::thousandths(500)),
};

/// `.emby-select-withcolor`'s edge, which the reference draws thinner than an
/// input's.
// reference: scheme-select
pub const SELECT_BORDER: Length = Length::em(0.07);

/// `.selectArrowContainer`'s inset from the field's trailing edge.
// reference: control-select-arrow
pub const SELECT_ARROW_INSET: Length = Length::em(0.3);

/// `.checkboxOutline`, the box itself.
// reference: control-checkbox
pub const CHECKBOX: Length = Length::em(1.83);

// reference: control-checkbox
pub const CHECKBOX_BORDER: Length = Length::em(0.14);

// reference: control-checkbox
pub const CHECKBOX_RADIUS: Length = Length::em(0.14);

/// `.emby-checkbox-label`'s own inset, where the label and the description
/// under it both begin.
// reference: control-checkbox-label
// reference: control-checkbox-description
pub const CHECKBOX_INSET: Length = Length::em(2.4);

/// What that inset leaves between the box and the label.
pub const CHECKBOX_GAP: Length = CHECKBOX_INSET.less(CHECKBOX);

/// `.emby-checkbox-label`'s own height, which is what one checkbox row stands
/// in.
// reference: control-checkbox-label
pub const CHECKBOX_ROW: Length = Length::em(2.35);

// reference: control-checkbox-list
const CHECKBOX_LIST_MARGIN: Length = Length::em(0.5);

/// Two `.checkboxList` rows are block-level siblings in normal flow.
pub const CHECKBOX_LIST_GAP: Length = CHECKBOX_LIST_MARGIN.collapsing(CHECKBOX_LIST_MARGIN);

/// `.fieldDescription`'s own inset from the control it stands under.
// reference: field-description
pub const DESCRIPTION_INSET: Length = Length::em(0.15);

// reference: section-vertical
const SECTION_BOTTOM: Length = Length::em(2.7);

// reference: section-vertical
const SECTION_BOTTOM_MOBILE: Length = Length::em(1.0);

/// The margin `.verticalSection-extrabottompadding` leaves under a section,
/// which the reference cuts on a mobile page.
// the television layout writes no rule of its own and keeps the desktop margin
// reference: section-vertical
pub fn section_bottom(layout: Layout) -> Length {
    match layout {
        Layout::Mobile => SECTION_BOTTOM_MOBILE,
        Layout::Desktop | Layout::Television => SECTION_BOTTOM,
    }
}

// reference: section-title
pub const SECTION_GAP: Length = Length::em(1.25);

// reference: section-title-cards
pub const SECTION_TITLE_TOP: Length = Length::em(1.25);

/// `.sectionTitle-cards` standing in a div that is no
/// `.sectionTitleContainer-cards`, which is what a grouped list writes over
/// each of its groups.
// reference: section-title-cards
pub const GROUP_TITLE_PAD: Padding = Padding {
    top: Length::em(0.5),
    right: Length::em(0.0),
    bottom: Length::em(0.2),
    left: Length::em(0.0),
};

/// The page's own padding, `.padded-top` and `.padded-bottom`, rather than the
/// `1em` standing inside a button's `padding: 0.9em 1em`.
// reference: page-padded
const PAD: Length = Length::em(1.0);

/// `.padded-top` and `.padded-bottom`, and nothing at the sides, which
/// `page_side` carries instead.
// reference: page-padded
pub const PAGE_PAD: Padding = Padding {
    top: PAD,
    right: Length::em(0.0),
    bottom: PAD,
    left: Length::em(0.0),
};

// reference: login-disclaimer
pub const DISCLAIMER_GAP: Length = Length::em(2.0);

// reference: page-bottom
pub const PAGE_BOTTOM: Length = Length::em(5.0);

// reference: page-standalone
pub const PAGE_TOP: Length = Length::em(4.5);

// reference: page-side
const PAGE_SIDE: Share = Share::per_ten_thousand(330);

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

/// `.emby-input`'s own padding, written in the em of the 110% it is set in.
// reference: control-input
pub const INPUT_PAD: Padding = Padding {
    top: typeface::FIELD.times(Ratio::thousandths(400)),
    right: typeface::FIELD.times(Ratio::thousandths(250)),
    bottom: typeface::FIELD.times(Ratio::thousandths(400)),
    left: typeface::FIELD.times(Ratio::thousandths(250)),
};

// reference: card-text
const CARD_TEXT_ENDS: Ratio = Ratio::thousandths(60);

// reference: card-text
const CARD_TEXT_SIDE: Ratio = Ratio::thousandths(500);

/// `.cardText`'s own padding, in the em of the size the line it holds is set
/// in.
// reference: card-text
pub const fn card_text(size: Length) -> Padding {
    Padding {
        top: size.times(CARD_TEXT_ENDS),
        right: size.times(CARD_TEXT_SIDE),
        bottom: size.times(CARD_TEXT_ENDS),
        left: size.times(CARD_TEXT_SIDE),
    }
}

// reference: users-card-secondary
const SECONDARY_LINES: Ratio = Ratio::thousandths(3000);

/// The floor `.localUsers` puts under a card's secondary line, which is three
/// of that line's own em.
pub const USER_CARD_SECONDARY: Length = typeface::SECONDARY.times(SECONDARY_LINES);

/// The drop `UserCardBox` gives the control it floats on its footer's trailing
/// edge.
// reference: user-card-box
pub const USER_CARD_MENU_TOP: Css = Css::of(5.0);

/// `.fab`'s own padding, which is what rounds it into a disc.
// reference: control-fab
pub const FAB_PAD: Length = Length::em(0.6);

/// The margin `.sectionTitleButton` leaves between a section's title and the
/// control beside it.
// reference: section-title-button
pub const SECTION_TITLE_BUTTON: Length = Length::em(1.5);

/// The top `.cardText-first` gives a footer's first line, which stands where
/// `.cardText`'s own writes the rest.
// reference: card-text-first
pub const CARD_TEXT_FIRST_TOP: Length = Length::em(0.24);

/// `.cardFooterLogo`'s own box, which stands the whole height of the footer at
/// its leading edge.
// reference: card-footer-logo-face
pub const CARD_FOOTER_LOGO: Length = Length::em(4.5);

/// The share of that box the logo is drawn at, centred in it.
// reference: card-footer-logo-face
pub const CARD_FOOTER_LOGO_IMAGE: Share = Share::per_ten_thousand(7000);

/// The inset `.cardFooter-withlogo` leaves for it.
// reference: card-footer-logo-face
pub const CARD_FOOTER_LOGO_INSET: Length = Length::em(4.0);

/// `.cardIndicators`' own inset from the top trailing corner of a card's
/// image.
// reference: card-indicators
pub const CARD_INDICATORS_INSET: Length = Length::em(0.225);

/// `.cardOverlayButton-hover`'s own padding, in the em of the size that button
/// is set in.
// reference: card-overlay-hover
pub const CARD_OVERLAY_PAD: Length = typeface::CARD_OVERLAY_BUTTON.times(Ratio::thousandths(250));

/// `.cardOverlayButtonIcon`'s own box, in the em of the glyph it holds.
// reference: card-overlay-button-icon
pub const CARD_OVERLAY_GLYPH: Length = typeface::CARD_OVERLAY_ICON.times(Ratio::thousandths(1500));

/// `.cardOverlayFab-primary`'s own disc, in the em of the size that control is
/// set in.
// reference: card-overlay-fab
pub const CARD_OVERLAY_FAB: Length = typeface::CARD_OVERLAY_FAB.times(Ratio::thousandths(3000));

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

/// The shadow a raised surface carries, which the reference writes the same
/// under a card and under a toast.
// reference: card-shadow
// reference: toast-face
pub const SHADOW: Shadow = Shadow {
    drop: Length::em(0.0725),
    blur: Length::em(0.29),
    spread: Length::em(0.0),
    color: scheme::SHADOW,
};

/// The rail a scrollbar stands in, and the scroller filling it.
// reference: scrollbar-size
pub const SCROLLBAR: Length = Length::em(0.4);

/// `.toast`'s own width, which it is never drawn narrower than, across the
/// border box its own `box-sizing` measures.
// reference: toast-face
const TOAST_MIN: Length = Length::em(20.0);

// reference: toast-face
pub const TOAST_PAD: Padding = Padding {
    top: Length::em(1.0),
    right: Length::em(1.5),
    bottom: Length::em(1.0),
    left: Length::em(1.5),
};

/// That floor inside `TOAST_PAD`, which is what a notice's content is laid
/// against.
// reference: toast-face
pub const TOAST_MIN_INSIDE: Length = TOAST_MIN.less(TOAST_PAD.left).less(TOAST_PAD.right);

/// `.toast`'s own corners, which the reference rounds tighter than a control's.
// reference: toast-face
pub const TOAST_RADIUS: Length = Length::em(0.15);

/// The margin one toast carries above and below itself.
// reference: toast-face
const TOAST_MARGIN: Length = Length::em(0.25);

/// `.toastContainer` is a flex column, which collapses nothing.
pub const TOAST_GAP: Length = TOAST_MARGIN.abutting(TOAST_MARGIN);

/// `.toastContainer`'s inset from the foot and the leading edge of the page.
// reference: toast-container
pub const TOAST_INSET: Length = Length::em(1.0);

// reference: header-logo — the slot's width
// reference: header-title — its height, which the title rule carries
pub const LOGO: Slot = Slot {
    width: Length::em(13.2),
    height: Length::em(1.7),
};

// reference: header-top
pub const HEADER_PAD: Length = Length::em(0.54);

// reference: list-body
const LIST_BODY_PAD: Padding = Padding {
    top: Length::em(0.85),
    right: Length::em(0.75),
    bottom: Length::em(0.85),
    left: Length::em(0.75),
};

/// `.listItemBodyText`'s own padding, which is what stands between one line of
/// a row's body and the next.
// reference: list-body-text
const LIST_TEXT_PAD: Padding = Padding {
    top: Length::em(0.1),
    right: Length::em(0.0),
    bottom: Length::em(0.1),
    left: Length::em(0.0),
};

/// `.listItemImage`'s own square.
// reference: list-image
pub const LIST_IMAGE: Slot = Slot {
    width: Length::em(4.0),
    height: Length::em(4.0),
};

/// The gap `.listItem-indexnumberleft` leaves between a row's position and
/// what follows it.
// reference: list-index
pub const LIST_INDEX_GAP: Length = Length::em(1.0);

/// How many lines a list's rows stack in their bodies, which is what holds
/// every row of one list to one height: the first line is the row's title and
/// the rest are its secondary lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lines {
    One,
    Two,
    Three,
}

/// What a list stands before every row's body, which the row's height is the
/// taller of: `.listItemImage`'s square, `.listItemIcon`'s box, or nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Before {
    Image,
    Icon,
    Nothing,
}

/// One `.listItem`'s every vertical measure and the height a list of them
/// pitches at, which is those same measures summed: `.listItem`'s own padding
/// around the taller of what stands before the body and the body itself, over
/// the rule `.listItem-border` draws.
// reference: control-list-item
// reference: control-list-border
// reference: list-body
// reference: list-body-text
// reference: list-body-text-desktop
// reference: list-image
// reference: list-icon
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListRow {
    lines: Lines,
    before: Before,
}

impl ListRow {
    /// A list standing `.listItemImage`'s square before every row's body.
    pub const fn art(lines: Lines) -> ListRow {
        ListRow {
            lines,
            before: Before::Image,
        }
    }

    /// A list standing `.listItemIcon` there.
    pub const fn glyph(lines: Lines) -> ListRow {
        ListRow {
            lines,
            before: Before::Icon,
        }
    }

    /// A list whose rows start at their bodies.
    pub const fn bare(lines: Lines) -> ListRow {
        ListRow {
            lines,
            before: Before::Nothing,
        }
    }

    /// `.listItem`'s own padding.
    pub const fn padding(self) -> Padding {
        LIST_ITEM_PAD
    }

    /// `.listItemBody`'s own padding.
    pub const fn body(self) -> Padding {
        LIST_BODY_PAD
    }

    /// `.listItemBodyText`'s own padding, which one line of the body stands
    /// inside.
    pub const fn text(self) -> Padding {
        LIST_TEXT_PAD
    }

    /// The line box that text stands in.
    pub const fn leading(self) -> typeface::Leading {
        typeface::LIST_LEADING
    }

    /// The size a row's title is written at.
    pub const fn title(self) -> Length {
        typeface::BODY
    }

    /// The size every line under the title is written at.
    pub const fn secondary(self) -> Length {
        typeface::SECONDARY
    }

    /// The rule `.listItem-border` draws under the row and on no other side.
    pub const fn rule(self) -> Length {
        LIST_RULE
    }

    /// The row standing over that rule.
    pub const fn standing(self) -> Length {
        self.padding()
            .top
            .plus(self.padding().bottom)
            .plus(self.written().taller(self.faced()))
    }

    /// That row and its rule, which is the height a list pitches at.
    pub const fn height(self) -> Length {
        self.standing().plus(self.rule())
    }

    /// One line of the body in its line box inside `.listItemBodyText`'s own
    /// padding.
    const fn line(self, size: Length) -> Length {
        self.text()
            .top
            .plus(self.text().bottom)
            .plus(self.leading().of(size))
    }

    /// `.listItemBody` itself: its own padding around the lines it stacks.
    const fn written(self) -> Length {
        let secondary = self.line(self.secondary());
        let stacked = match self.lines {
            Lines::One => self.line(self.title()),
            Lines::Two => self.line(self.title()).plus(secondary),
            Lines::Three => self.line(self.title()).plus(secondary).plus(secondary),
        };
        self.body().top.plus(self.body().bottom).plus(stacked)
    }

    /// What stands before the body, as tall as it is drawn.
    const fn faced(self) -> Length {
        match self.before {
            Before::Image => LIST_IMAGE.height,
            Before::Icon => typeface::LIST_ICON,
            Before::Nothing => Length::em(0.0),
        }
    }
}

/// `a[data-role=button]`'s own padding.
// reference: control-localnav
pub const LOCALNAV_PAD: Padding = Padding {
    top: Length::em(0.8),
    right: Length::em(1.0),
    bottom: Length::em(0.8),
    left: Length::em(1.0),
};

/// The room the group takes back between adjacent controls, which lays each
/// one over the control before it.
// reference: control-localnav-group
pub const LOCALNAV_OVERLAP: Length = Length::em(-0.4);

/// The radius the group carries at its two ends and nowhere else.
// reference: control-localnav-group
pub const LOCALNAV_RADIUS: Length = Length::em(0.3125);

/// The room `.localnav` reserves under itself.
// reference: localnav-row
pub const LOCALNAV_BOTTOM: Length = Length::em(2.2);

/// MUI's own spacing step, which every measure the reference writes as
/// `spacing(n)` is a count of.
// reference: mui-spacing
const SPACING_STEP: Css = Css::of(8.0);

/// The rhythm a dashboard screen stacks its content at, which is three steps.
// reference: dashboard-content
pub const DASHBOARD_GAP: Css = SPACING_STEP.times(Ratio::thousandths(3000));

/// `.content-primary`'s own side padding on a dashboard page.
// reference: dashboard-content-side
pub const DASHBOARD_SIDE: Length = Length::em(1.0);

/// The room `.dashboardDocument .content-primary` leaves above itself.
// reference: dashboard-frame
pub const DASHBOARD_TOP: Length = Length::em(3.25);

/// `$drawer-width`, the column the navigation drawer stands in and the page's
/// own content begins after.
// reference: dashboard-frame
pub const DRAWER: Css = Css::of(240.0);

/// MUI's own breakpoints, which every `Grid item`'s ladder is written against
/// and which the dashboard's overrides declare.
// reference: dashboard-frame
const SMALL_AT: Query = Query::MinWidth(Breakpoint::pixels(600));

// reference: dashboard-frame
const MEDIUM_AT: Query = Query::MinWidth(Breakpoint::pixels(900));

// reference: dashboard-frame
const LARGE_AT: Query = Query::MinWidth(Breakpoint::pixels(1200));

// reference: dashboard-frame
const EXTRA_AT: Query = Query::MinWidth(Breakpoint::pixels(1536));

/// The page the drawer stands beside the content on rather than over it.
// reference: dashboard-frame
pub const DRAWER_BESIDE_AT: Query = MEDIUM_AT;

/// The ladder the libraries page's own grid puts one card on.
// reference: dashboard-libraries-grid
pub const LIBRARY_CELL: Cell = Cell {
    xs: Columns::twelfths(12.0),
    sm: Some(Columns::twelfths(6.0)),
    md: Some(Columns::twelfths(3.0)),
    lg: Some(Columns::twelfths(2.4)),
    xl: None,
};

/// The ladder the plugins page's own grid puts one card on.
// reference: dashboard-plugins-grid
pub const PLUGIN_CELL: Cell = Cell {
    xs: Columns::twelfths(12.0),
    sm: Some(Columns::twelfths(6.0)),
    md: Some(Columns::twelfths(4.0)),
    lg: Some(Columns::twelfths(3.0)),
    xl: Some(Columns::twelfths(2.0)),
};

/// The gutter a `Grid container` of cards leaves between two of them.
// reference: dashboard-libraries-grid
// reference: dashboard-plugins-grid
pub const CARD_GRID_GAP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The height `BaseCard` stands at where the screen drawing it names none.
// reference: base-card
pub const BASE_CARD: Css = Css::unitless(240.0);

/// The height the libraries page gives its own cards.
// reference: dashboard-library-card
pub const LIBRARY_CARD: Css = Css::unitless(260.0);

/// `MuiCardContent`'s own padding on every side it writes one.
// reference: mui-card-content
const CARD_CONTENT_INSET: Css = Css::unitless(16.0);

/// That padding as `BaseCard` writes it, being the last child
/// `MuiCardContent` rounds off: its bottom two steps and its trailing side one.
// reference: base-card
// reference: mui-spacing
pub const CARD_CONTENT_PAD: Inset = Inset {
    top: CARD_CONTENT_INSET,
    right: SPACING_STEP,
    bottom: SPACING_STEP.times(Ratio::thousandths(2000)),
    left: CARD_CONTENT_INSET,
};

/// The least `MuiCardContent` stands at, border box included.
// reference: base-card
const CARD_CONTENT_MIN: Css = Css::unitless(50.0);

/// The same least, inside that padding.
pub const CARD_CONTENT_MIN_INSIDE: Css = CARD_CONTENT_MIN
    .less(CARD_CONTENT_PAD.top)
    .less(CARD_CONTENT_PAD.bottom);

/// The margin `gutterBottom` leaves under a `MuiTypography` line, in the em of
/// the line it sits under.
// reference: mui-typography-gutter-bottom
pub const GUTTER_BOTTOM: Length = Length::em(0.35);

/// `.MuiDrawer-paper`'s own foot, which the reference leaves clear for the
/// now-playing bar.
// reference: drawer-paper
pub const DRAWER_BOTTOM: Length = Length::em(4.2);

/// `MuiListItemButton`'s own padding, which MUI writes as the bare numbers the
/// DOM reads as css pixels.
// reference: mui-list-item-button
pub const LIST_ROW_PAD: Inset = Inset {
    top: Css::unitless(8.0),
    right: Css::unitless(16.0),
    bottom: Css::unitless(8.0),
    left: Css::unitless(16.0),
};

/// The slot `MuiListItemIcon` stands a row's glyph in, which the reference
/// narrows from MUI's own and which `MuiListItemText`'s own `inset` matches.
// reference: dashboard-list-icon
// reference: dashboard-list-icon-slot
pub const LIST_ICON_SLOT: Css = Css::unitless(36.0);

/// `MuiList`'s own padding above and below the rows it holds.
// reference: mui-list
pub const LIST_PAD: Inset = Inset {
    top: Css::unitless(8.0),
    right: Css::unitless(0.0),
    bottom: Css::unitless(8.0),
    left: Css::unitless(0.0),
};

/// The room `MuiListItem` keeps clear at the trailing edge of the control it
/// stands a `secondaryAction` beside.
// reference: mui-list-item
pub const LIST_ROW_ACTION: Css = Css::unitless(48.0);

/// Where that action stands, measured from the row's own trailing edge.
// reference: mui-list-secondary-action
pub const LIST_ACTION_INSET: Css = Css::unitless(16.0);

/// `MuiListItemText`'s own margins around the one line it writes.
// reference: mui-list-item-text
pub const LIST_TEXT_MARGIN: Inset = Inset {
    top: Css::unitless(4.0),
    right: Css::unitless(0.0),
    bottom: Css::unitless(4.0),
    left: Css::unitless(0.0),
};

/// The same margins where a row writes a second line under its title.
// reference: mui-list-item-text
pub const LIST_TEXT_MARGIN_STACKED: Inset = Inset {
    top: Css::unitless(6.0),
    right: Css::unitless(0.0),
    bottom: Css::unitless(6.0),
    left: Css::unitless(0.0),
};

/// The slot `MuiListItemAvatar` stands a row's disc in.
// reference: mui-list-item-avatar
pub const LIST_AVATAR_SLOT: Css = Css::unitless(56.0);

/// `MuiAvatar`'s own disc.
// reference: mui-avatar
pub const AVATAR: Css = Css::unitless(40.0);

/// The corner that rounds it, which MUI writes as the whole of its own box.
// reference: mui-avatar
pub const AVATAR_RADIUS: Share = Share::per_ten_thousand(5000);

/// `MuiIconButton`'s own padding, which is what rounds it into a disc.
// reference: mui-icon-button
pub const ICON_BUTTON_PAD: Css = Css::unitless(8.0);

/// The corner MUI rounds that padding by.
// reference: mui-icon-button
pub const ICON_BUTTON_RADIUS: Share = Share::per_ten_thousand(5000);

/// `MuiLinearProgress`'s own track.
// reference: mui-linear-progress
pub const LINEAR_PROGRESS: Css = Css::unitless(4.0);

/// `TaskProgress`'s own row, which the reference holds to this height.
// reference: task-progress
pub const TASK_PROGRESS_ROW: Length = Length::em(1.2);

/// The least width it gives that row.
// reference: task-progress
pub const TASK_PROGRESS_MIN: Css = Css::of(170.0);

/// The gap it leaves between the bar and the reading beside it.
// reference: task-progress
pub const TASK_PROGRESS_GAP: Css = SPACING_STEP;

/// The room it leaves after that reading.
// reference: task-progress
pub const TASK_PROGRESS_TRAIL: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The room the tasks page leaves over its own categories.
// reference: tasks-page
pub const TASKS_TOP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The rhythm one category stacks its heading and its list at.
// reference: tasks-category
pub const CATEGORY_GAP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The padding the reference writes inside the paper a log file's body stands
/// on.
// reference: logs-viewer
pub const VIEWER_PAD: Css = Css::of(16.0);

/// The room it leaves over that paper.
// reference: logs-viewer
pub const VIEWER_GAP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The inset an entry held by a group stands at, which is four steps.
// reference: drawer-server
pub const DRAWER_NESTED: Css = SPACING_STEP.times(Ratio::thousandths(4000));

/// The width the reference caps the two sentences it writes where the server
/// holds no repository at.
// reference: repositories-page
pub const REPOSITORIES_EMPTY: Css = Css::of(500.0);

/// The rhythm it stacks those two sentences at.
// reference: repositories-page
pub const REPOSITORIES_EMPTY_GAP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// A body cell's own padding at MRT's compact density.
// reference: table-body-cell
pub const TABLE_CELL_PAD: Padding = Padding {
    top: Length::em(0.5),
    right: Length::em(0.5),
    bottom: Length::em(0.5),
    left: Length::em(0.5),
};

/// The same cell in a display column, which MRT pads at the sides alone.
// reference: table-body-cell
pub const TABLE_DISPLAY_PAD: Padding = Padding {
    top: Length::em(0.0),
    right: Length::em(0.5),
    bottom: Length::em(0.0),
    left: Length::em(0.5),
};

/// A head cell's own padding, which MRT writes shallower above than below.
// reference: table-head-cell
pub const TABLE_HEAD_PAD: Padding = Padding {
    top: Length::em(0.25),
    right: Length::em(0.5),
    bottom: Length::em(0.4),
    left: Length::em(0.5),
};

/// The rule every table cell draws under itself, written in css pixels.
// reference: mui-table-cell
pub const TABLE_CELL_RULE: Css = Css::of(1.0);

/// A toolbar's own least height; the reference keeps two of these clear of the
/// table's container, one for each toolbar the paper stands.
// reference: table-toolbar
// reference: table-container
pub const TABLE_TOOLBAR: Length = Length::em(3.5);

/// The padding the toolbar's own row carries.
// reference: table-toolbar-row
pub const TABLE_TOOLBAR_PAD: Length = Length::em(0.5);

/// The gap between the controls standing in that row.
// reference: table-toolbar-row
pub const TABLE_TOOLBAR_GAP: Length = Length::em(0.5);

/// The gap `TablePage` leaves between its title and the line under it, which
/// is two steps.
// reference: table-page
pub const TABLE_TITLE_GAP: Css = SPACING_STEP.times(Ratio::thousandths(2000));

/// The room it leaves under that stack, which is one.
// reference: table-page
pub const TABLE_TITLE_BOTTOM: Css = SPACING_STEP;

/// The width MRT gives a column whose definition declares none.
// reference: table-column-default
const TABLE_COLUMN: Css = Css::unitless(180.0);

/// The narrowest MRT draws any column, whatever its definition declares.
// reference: table-cell-width
pub const TABLE_COLUMN_FLOOR: Css = Css::unitless(30.0);

// reference: table-activity-columns
pub const ACTIVITY_TIME: Css = Css::unitless(160.0);

// reference: table-activity-columns
pub const ACTIVITY_LEVEL: Css = Css::unitless(90.0);

// reference: table-activity-columns
pub const ACTIVITY_USER: Css = Css::unitless(75.0);

// reference: table-activity-columns
pub const ACTIVITY_NAME: Css = Css::unitless(270.0);

// reference: table-activity-columns
pub const ACTIVITY_OVERVIEW: Css = Css::unitless(170.0);

// reference: table-activity-columns
pub const ACTIVITY_TYPE: Css = Css::unitless(150.0);

// reference: table-activity-columns
pub const ACTIVITY_ACTIONS: Css = Css::unitless(60.0);

// reference: table-devices-columns
pub const DEVICES_LAST_ACTIVE: Css = Css::unitless(160.0);

// reference: table-devices-columns
pub const DEVICES_DEVICE: Css = Css::unitless(200.0);

// reference: table-devices-columns
pub const DEVICES_APP: Css = Css::unitless(200.0);

// reference: table-devices-columns
pub const DEVICES_USER: Css = Css::unitless(120.0);

// reference: table-devices-actions
pub const DEVICES_ACTIONS: Css = Css::unitless(100.0);

// reference: table-keys-columns
pub const KEYS_TOKEN: Css = Css::unitless(300.0);

// reference: table-keys-columns
pub const KEYS_APP: Css = TABLE_COLUMN;

// reference: table-keys-columns
pub const KEYS_ISSUED: Css = TABLE_COLUMN;

/// The keys screen declares a narrower action column than MRT will draw, so it
/// stands at the floor instead.
// reference: table-keys-actions
pub const KEYS_ACTIONS: Css = TABLE_COLUMN_FLOOR;

/// MUI's own corner, which every MUI surface rounds by.
// reference: mui-shape
pub const SHAPE_RADIUS: Css = Css::unitless(4.0);

/// One segment of the toolbar's group: its own padding.
// reference: mui-toggle-button
pub const TOGGLE_PAD: Css = Css::unitless(7.0);

/// The edge that segment carries.
// reference: mui-toggle-button
pub const TOGGLE_BORDER: Css = Css::of(1.0);

/// The room the group takes back between adjacent segments.
// reference: mui-toggle-group
pub const TOGGLE_OVERLAP: Css = Css::unitless(-1.0);

/// The height a table pitches its body rows at: a cell's own padding around
/// one line of the table's lettering, over the rule the cell draws under
/// itself. Every row is one line, the reference setting `white-space: nowrap`
/// at this density.
// reference: table-body-cell
// reference: mui-table-cell
pub fn table_row(layout: Layout) -> Drawn {
    TABLE_CELL_PAD
        .top
        .plus(typeface::BODY_2_LEADING.of(typeface::BODY_2))
        .plus(TABLE_CELL_PAD.bottom)
        .drawn()
        .plus(TABLE_CELL_RULE.drawn(layout))
}

/// The head row's own height, which is the head cell's padding and that same
/// rule around the line box MUI writes for a head cell.
// reference: table-head-cell
// reference: mui-table-cell
pub fn table_head(layout: Layout) -> Drawn {
    TABLE_HEAD_PAD
        .top
        .plus(typeface::TABLE_HEAD_LEADING.of(typeface::BODY_2))
        .plus(TABLE_HEAD_PAD.bottom)
        .drawn()
        .plus(TABLE_CELL_RULE.drawn(layout))
}

/// `MuiFilledInput`'s own padding around its value, which MUI writes as the
/// bare numbers the DOM reads as css pixels.
// reference: mui-filled-input
pub const FILLED_PAD: Inset = Inset {
    top: Css::unitless(25.0),
    right: Css::unitless(12.0),
    bottom: Css::unitless(8.0),
    left: Css::unitless(12.0),
};

/// Where the field's own label stands once it has shrunk.
// reference: mui-input-label
pub const FILLED_LABEL_INSET: Inset = Inset {
    top: Css::of(7.0),
    right: Css::of(0.0),
    bottom: Css::of(0.0),
    left: Css::of(12.0),
};

/// The rule the field draws under itself at rest.
// reference: mui-filled-underline
pub const FILLED_RULE: Css = Css::of(1.0);

/// The room a filled select keeps clear at its trailing edge.
// reference: mui-select-filled
const FILLED_CHEVRON_ROOM: Css = Css::unitless(32.0);

/// The padding a filled select stands its value at, which is a filled field's
/// own with its trailing side widened to the room the chevron is laid over.
// reference: mui-filled-input
// reference: mui-select-filled
pub const FILLED_SELECT_PAD: Inset = Inset {
    top: FILLED_PAD.top,
    right: FILLED_CHEVRON_ROOM,
    bottom: FILLED_PAD.bottom,
    left: FILLED_PAD.left,
};

/// Its chevron's inset from that edge.
// reference: mui-select-icon
pub const FILLED_CHEVRON_INSET: Css = Css::unitless(7.0);

/// `MuiCheckbox`'s own padding around its box.
// reference: mui-switch-base
pub const CHECK_PAD: Css = Css::unitless(9.0);

/// `MuiSwitchBase`'s own corner, which rounds a box's padding into a disc.
// reference: mui-switch-base
pub const CHECK_RADIUS: Share = Share::per_ten_thousand(5000);

/// `MuiFormControlLabel`'s own margins, the leading one pulling the box back
/// over the edge of the page.
// reference: mui-form-control-label
pub const CHECK_LABEL_MARGIN: Inset = Inset {
    top: Css::unitless(0.0),
    right: Css::unitless(16.0),
    bottom: Css::unitless(0.0),
    left: Css::unitless(-11.0),
};

/// `MuiFormHelperText`'s own margins, which MUI writes as the bare numbers the
/// DOM reads as css pixels.
// reference: mui-form-helper-text
pub const HELPER_MARGIN: Inset = Inset {
    top: Css::unitless(3.0),
    right: Css::unitless(0.0),
    bottom: Css::unitless(0.0),
    left: Css::unitless(0.0),
};

/// The margins the `contained` variant takes, which is the variant every
/// helper under a filled control is drawn in.
// reference: mui-form-helper-text
pub const HELPER_CONTAINED_MARGIN: Inset = Inset {
    top: HELPER_MARGIN.top,
    right: Css::unitless(14.0),
    bottom: HELPER_MARGIN.bottom,
    left: Css::unitless(14.0),
};

/// A contained button's own padding at MUI's large size.
// reference: mui-button-large
pub const CONTAINED_PAD: Inset = Inset {
    top: Css::of(8.0),
    right: Css::of(22.0),
    bottom: Css::of(8.0),
    left: Css::of(22.0),
};

/// The least width MUI draws a button at.
// reference: mui-button
const CONTAINED_MIN: Css = Css::unitless(64.0);

/// That floor inside `CONTAINED_PAD`, which is what a contained button's label
/// is laid against.
// reference: mui-button
// reference: mui-button-large
pub const CONTAINED_MIN_INSIDE: Css = CONTAINED_MIN
    .less(CONTAINED_PAD.left)
    .less(CONTAINED_PAD.right);

/// An alert's own padding.
// reference: mui-alert
pub const ALERT_PAD: Inset = Inset {
    top: Css::of(6.0),
    right: Css::of(16.0),
    bottom: Css::of(6.0),
    left: Css::of(16.0),
};

/// The room its glyph takes before the sentence.
// reference: mui-alert-parts
pub const ALERT_GLYPH_PAD: Inset = Inset {
    top: Css::of(7.0),
    right: Css::unitless(12.0),
    bottom: Css::of(7.0),
    left: Css::of(0.0),
};

/// The room the sentence itself stands in.
// reference: mui-alert-parts
pub const ALERT_MESSAGE_PAD: Inset = Inset {
    top: Css::of(8.0),
    right: Css::of(0.0),
    bottom: Css::of(8.0),
    left: Css::of(0.0),
};

/// The cap MUI holds a menu's paper under: the page it stands in, and the
/// offset that leaves a row of that page tappable beyond the menu.
// reference: mui-menu-paper
const MENU_CAP: Cap = Cap {
    share: Share::WHOLE,
    offset: Css::of(-96.0),
};

/// The height a select's menu stands at: the height iced draws one option at,
/// which is the closed field's own padding around one line of its lettering
/// because iced gives a menu's options that padding, for each option the menu
/// holds, under the cap MUI writes over its paper.
// reference: mui-filled-input
// reference: mui-input-base
// reference: mui-menu-paper
pub fn menu_height(options: usize, viewport: Viewport) -> Drawn {
    let layout = viewport.layout();
    let stacked = Drawn::of(filled_row(layout).count() * options as f32);
    MENU_CAP.holds(stacked, viewport.canvas().height(), layout)
}

/// The height a filled field stands: its own padding around one line of the
/// lettering MUI writes inside it.
// reference: mui-filled-input
// reference: mui-input-base
fn filled_row(layout: Layout) -> Drawn {
    FILLED_PAD
        .top
        .drawn(layout)
        .plus(typeface::FILLED_LEADING.of(typeface::BODY).drawn())
        .plus(FILLED_PAD.bottom.drawn(layout))
}

/// The height a row of checkboxes pitches at: `MuiCheckbox`'s padding around
/// MUI's own medium glyph.
// reference: mui-switch-base
// reference: mui-svg-icon
pub fn check_row(layout: Layout) -> Drawn {
    CHECK_PAD
        .drawn(layout)
        .plus(typeface::CONTROL_GLYPH.drawn())
        .plus(CHECK_PAD.drawn(layout))
}

/// `.listItem`'s own padding, which the reference writes wider at the leading
/// edge than at the other three.
// reference: control-list-item
pub const LIST_ITEM_PAD: Padding = Padding {
    top: Length::em(0.25),
    right: Length::em(0.25),
    bottom: Length::em(0.25),
    left: Length::em(0.5),
};

/// The rule `.listItem-border` draws under a row and on no other side.
// reference: control-list-border
pub const LIST_RULE: Length = Length::em(0.1);

/// `.editPageSidebar`'s width, and `.editPageInnerContent`'s beside it.
// reference: metadata-sidebar
pub const EDITOR_SIDEBAR: Share = Share::per_ten_thousand(3000);

// reference: metadata-sidebar
pub const EDITOR_CONTENT: Share = Share::per_ten_thousand(6850);

/// The same two on the widest page the reference writes a step for.
// reference: metadata-sidebar-wide
pub const EDITOR_SIDEBAR_WIDE: Share = Share::per_ten_thousand(2500);

// reference: metadata-sidebar-wide
pub const EDITOR_CONTENT_WIDE: Share = Share::per_ten_thousand(7350);

/// The page the reference stands the sidebar beside the content on.
// reference: metadata-sidebar
pub const EDITOR_BESIDE_AT: Query = Query::MinWidth(Breakpoint::em(50.0));

// reference: metadata-sidebar-wide
pub const EDITOR_WIDE_AT: Query = Query::MinWidth(Breakpoint::em(112.5));

/// The gutter the reference leaves between the sidebar and the content beside
/// it, which the sidebar's own rule stands in.
// reference: metadata-sidebar
pub const EDITOR_GAP: Share = Share::WHOLE.less(EDITOR_SIDEBAR).less(EDITOR_CONTENT);

/// The same gutter on the widest page the reference writes a step for.
// reference: metadata-sidebar-wide
pub const EDITOR_GAP_WIDE: Share = Share::WHOLE
    .less(EDITOR_SIDEBAR_WIDE)
    .less(EDITOR_CONTENT_WIDE);

/// The rule down the sidebar's trailing edge, written in css pixels.
// reference: metadata-sidebar
pub const EDITOR_RULE: Css = Css::of(1.0);

/// The gap `.listItemIcon` leaves between a row's glyph and its body.
// reference: list-icon
pub const LIST_ICON_GAP: Length = Length::em(0.25);

/// The inset the settings menu writes on its own section title.
// reference: settings-menu
pub const SECTION_TITLE_INSET: Length = Length::em(0.25);

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
const LETTERS_BOTTOM: Length = Length::em(5.5);

/// The same inset on a short page.
// reference: alpha-picker-size
const LETTERS_BOTTOM_SHORT: Length = Length::em(5.0);

// reference: alpha-picker-size
const LETTERS_SHORT: Query = Query::MaxHeight(Breakpoint::em(50.0));

/// `.alphaPicker-fixed-right`'s inset from the edge of the page.
// reference: alpha-picker-right
const LETTERS_RIGHT: Length = Length::em(0.4);

/// The same inset on a wide page.
// reference: alpha-picker-right
const LETTERS_RIGHT_ROOMY: Length = Length::em(1.0);

// reference: alpha-picker-right
const LETTERS_ROOMY: Query = Query::MinWidth(Breakpoint::em(62.5));

/// The share of the page `.padded-right-withalphapicker` keeps clear for the
/// letter picker.
// reference: alpha-picker-reserve
const LETTERS_RESERVE: Share = Share::per_ten_thousand(750);

/// `.searchFieldsInner`, which the page centres in what it is given.
// reference: search-field
pub const SEARCH_FIELD: Length = Length::em(60.0);

/// `.searchfields-icon`'s gap to the field it stands before, in the glyph's own
/// em.
// reference: search-icon
pub const SEARCH_ICON_GAP: Length = typeface::SEARCH_ICON.times(Ratio::thousandths(250));

/// The same glyph's lift off the foot of the row, in that same em.
// reference: search-icon
pub const SEARCH_ICON_LIFT: Length = typeface::SEARCH_ICON.times(Ratio::thousandths(100));

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
const ICON_MARGIN: Length = Length::em(0.29);

pub const ICON_GAP: Length = ICON_MARGIN.abutting(ICON_MARGIN);

/// `.paper-icon-button-light`'s padding, which is what rounds it into a disc.
// reference: control-icon-button
pub const PAPER_ICON_BUTTON_PAD: Length = Length::em(0.556);

// reference: progress-bar
pub const PROGRESS: Length = Length::em(0.28);

// reference: guide-row
pub const GUIDE_ROW: Length = Length::em(4.42);

/// The rule between two of the guide's rows, down the trailing edge of its
/// channel column, and down the leading edge of a programme's cell.
// reference: guide-row
// reference: guide-channel-header
// reference: guide-program-cell
pub const GUIDE_RULE: Css = Css::of(1.0);

/// `.guide-channelTimeslotHeader` and `.timeslotHeader`'s own height.
// reference: guide-timeslot-height
pub const GUIDE_TIMESLOT: Length = Length::em(2.8);

/// The indent `.timeslotHeader` writes its time at.
// reference: guide-timeslot
pub const GUIDE_TIMESLOT_INDENT: Length = Length::em(0.25);

/// `.guideProgramName`'s own padding.
// reference: guide-program-name
pub const GUIDE_PROGRAM_PAD: Padding = Padding {
    top: Length::em(0.0),
    right: Length::em(0.7),
    bottom: Length::em(0.0),
    left: Length::em(0.7),
};

/// `.guideProgramIndicator`'s own padding.
// reference: guide-program-indicator
pub const GUIDE_BADGE_PAD: Padding = Padding {
    top: Length::em(0.2),
    right: Length::em(0.25),
    bottom: Length::em(0.2),
    left: Length::em(0.25),
};

/// Its radius.
// reference: guide-program-indicator
pub const GUIDE_BADGE_RADIUS: Length = Length::em(0.25);

/// The gap it leaves after the programme's name.
// reference: guide-program-indicator
pub const GUIDE_BADGE_LEADING: Length = Length::em(1.0);

/// The gap it leaves after itself.
// reference: guide-program-indicator
pub const GUIDE_BADGE_TRAILING: Length = Length::em(0.5);

/// The page the guide writes no badge on a cell at.
// reference: guide-indicators-narrow
pub const GUIDE_BADGE_AT: Query = Query::MaxWidth(Breakpoint::em(50.0));

/// The gap `.guideProgramSecondaryInfo` leaves over the episode title.
// reference: guide-secondary-info
pub const GUIDE_EPISODE_TOP: Length = Length::em(0.1);

/// The gap `.programIcon` leaves before a timer's glyph.
// reference: guide-program-icon
pub const GUIDE_MARK_GAP: Length = Length::em(0.5);

/// The page the guide draws its channel header narrow at: no number, and a
/// wider logo.
// reference: guide-channel-narrow
pub const GUIDE_CHANNEL_NARROW_AT: Query = Query::MaxWidth(Breakpoint::em(62.5));

/// The inset `.guideChannelNumber` writes at the header's leading edge, and
/// `.guideChannelName` at its trailing one.
// reference: guide-channel-number
// reference: guide-channel-name
pub const GUIDE_CHANNEL_INSET: Length = Length::em(1.0);

/// The inset `.guideChannelImage` leaves at the header's trailing edge.
// reference: guide-channel-image
pub const GUIDE_LOGO_INSET: Share = Share::per_ten_thousand(800);

// reference: guide-channel-image
const GUIDE_LOGO_TOP: Share = Share::per_ten_thousand(1500);

// reference: guide-channel-image
const GUIDE_LOGO: Share = Share::per_ten_thousand(4000);

// reference: guide-channel-narrow
const GUIDE_LOGO_NARROW: Share = Share::per_ten_thousand(7000);

// reference: guide-channel-number
const GUIDE_NUMBER: Share = Share::per_ten_thousand(3000);

// reference: guide-channel-name
const GUIDE_NAME: Share = Share::per_ten_thousand(7000);

// reference: guide-channel-name-wide
const GUIDE_NAME_WIDE: Share = Share::per_ten_thousand(4000);

// reference: guide-channel-name-wide
const GUIDE_NAME_WIDE_AT: Query = Query::MinWidth(Breakpoint::em(62.5));

/// `.itemBackdrop`'s height.
// reference: detail-backdrop — 40vh
pub const BACKDROP: Share = Share::units(40.0);

/// Its height on a short portrait page.
// reference: detail-backdrop — 30vh
const BACKDROP_PORTRAIT: Share = Share::units(30.0);

// reference: detail-backdrop
const BACKDROP_PORTRAIT_AT: Query = Query::MaxWidth(Breakpoint::em(40.0));

/// What the stacked arrangement leaves above the backdrop.
// reference: detail-backdrop
pub const BACKDROP_TOP: Length = Length::em(3.0);

/// `.detailRibbon`'s height, which is also how far it stands over the
/// backdrop.
// reference: detail-ribbon
pub const RIBBON: Length = Length::em(7.2);

/// The ribbon over the backdrop: its whole height above the backdrop's foot,
/// so its own foot lands on that foot and the pair keeps the backdrop's
/// height.
// reference: detail-ribbon
pub const RIBBON_OVERLAP: Overlap = Overlap {
    raised: RIBBON,
    shed: Length::em(0.0),
};

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

/// The ribbon's own content in the stacked arrangement, which is the page less
/// the two sides `.padded-left` and `.padded-right` write over the ribbon's
/// own shorthand.
// reference: page-side
const RIBBON_CONTENT: Share = Share::WHOLE.less(PAGE_SIDE).less(PAGE_SIDE);

/// The share of the page the primary content insets by in the stacked
/// arrangement.
// reference: detail-content
pub const DETAIL_SIDE: Share = Share::per_ten_thousand(500);

/// The trailing inset the ribbon arrangement leaves.
// reference: detail-content
pub const DETAIL_TRAIL: Share = Share::per_ten_thousand(200);

/// The poster's width beside the ribbon.
// reference: detail-poster-arms — 25vw
pub const DETAIL_POSTER: Share = Share::units(25.0);

/// Its width over the backdrop in the stacked arrangement.
// reference: detail-poster-arms — 30vw
pub const DETAIL_POSTER_STACKED: Share = Share::units(30.0);

/// The poster's inset from the page's leading edge in the televised
/// arrangement.
// reference: detail-poster-arms
pub const DETAIL_POSTER_TELEVISED: Share = Share::per_ten_thousand(500);

/// How far the poster rises over the ribbon, which is the ribbon's own height
/// and four fifths again.
// reference: detail-poster-arms
const DETAIL_POSTER_RISE: Length = RIBBON.times(Ratio::thousandths(1800));

/// The poster's inset from the leading edge beside the ribbon.
// reference: detail-poster-arms
pub const DETAIL_POSTER_INSET: Share = PAGE_SIDE;

/// Its inset in the stacked arrangement on a page wide enough to leave it one.
// reference: detail-poster-arms
const DETAIL_POSTER_STACKED_INSET: Share = Share::per_ten_thousand(500);

/// Where a page stops raising the poster over the ribbon, the raising and the
/// reference's own lowering being alternatives that never compose.
// reference: detail-narrow
const DETAIL_POSTER_LOWERED_AT: Query = Query::MaxWidth(Breakpoint::em(62.5));

/// Where the poster stands flush inside the ribbon and the row of buttons
/// stops leaving it room.
// reference: detail-poster-arms
// reference: detail-buttons-narrow
const DETAIL_NARROW: Query = Query::MaxWidth(Breakpoint::em(32.0));

/// The room the stacked head leaves the poster beside the item's name.
// reference: detail-head-inset
const DETAIL_HEAD_INSET: Share = Share::per_ten_thousand(3750);

/// The steps that room takes as the page widens, in the order the cascade
/// resolves them; the reference writes each as one card of the poster wall,
/// spelt out to thirty digits.
// reference: detail-head-inset
const DETAIL_HEAD_INSET_STEPS: [(Query, Across); 7] = [
    (Query::MinWidth(Breakpoint::em(43.75)), Across::cards(4)),
    (Query::MinWidth(Breakpoint::em(50.0)), Across::cards(5)),
    (Query::MinWidth(Breakpoint::em(75.0)), Across::cards(6)),
    (Query::MinWidth(Breakpoint::em(87.5)), Across::cards(7)),
    (Query::MinWidth(Breakpoint::em(100.0)), Across::cards(8)),
    (Query::MinWidth(Breakpoint::em(120.0)), Across::cards(9)),
    (Query::MinWidth(Breakpoint::em(131.25)), Across::cards(10)),
];

/// The gap above and below the row of detail buttons.
// reference: detail-buttons
pub const DETAIL_BUTTONS: Length = Length::em(1.0);

/// The gap under that row in the stacked arrangement.
// reference: detail-centred
const DETAIL_BUTTONS_BOTTOM: Length = Length::em(0.5);

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
fn guide_strip(viewport: Viewport) -> Drawn {
    stepped(viewport, GUIDE_STRIP, &GUIDE_STRIP_STEPS).of(viewport.canvas().width())
}

/// One minute of the guide, which is its strip over the day it spans.
// reference: guide-strip
fn guide_minute(viewport: Viewport) -> Drawn {
    Drawn::of(guide_strip(viewport).count() / GUIDE_SPAN.num_minutes() as f32)
}

/// How far across the guide's strip a stretch of time reaches.
// reference: guide-strip
pub fn guide_across(spanning: TimeDelta, viewport: Viewport) -> Drawn {
    Drawn::of(guide_minute(viewport).count() * spanning.num_minutes() as f32)
}

/// The guide's channel column, 24vw stepping to 16vw, 14vw and 12vw.
// reference: guide-channel
pub fn guide_channel(viewport: Viewport) -> Drawn {
    stepped(viewport, GUIDE_CHANNEL, &GUIDE_CHANNEL_STEPS).of(viewport.canvas().width())
}

/// What a guide row stands in over the rule at the foot of it.
// reference: guide-row
pub fn guide_standing(layout: Layout) -> Drawn {
    Drawn::of(GUIDE_ROW.drawn().count() - GUIDE_RULE.drawn(layout).count())
}

/// `.guideChannelImage`'s width: two fifths of the channel header, and seven
/// tenths on a narrow page.
// reference: guide-channel-image
// reference: guide-channel-narrow
pub fn guide_logo(viewport: Viewport) -> Drawn {
    let share = match viewport.matches(GUIDE_CHANNEL_NARROW_AT) {
        true => GUIDE_LOGO_NARROW,
        false => GUIDE_LOGO,
    };
    share.of(guide_channel(viewport))
}

/// Its height, which is the header's own less what it leaves over and under
/// itself.
// reference: guide-channel-image
// reference: guide-row
pub fn guide_logo_height() -> Drawn {
    let header = GUIDE_ROW.drawn();
    Drawn::of(header.count() - GUIDE_LOGO_TOP.of(header).count() * 2.0)
}

/// The most of the header a channel's name takes: seven tenths, and two fifths
/// above 62.5em.
// reference: guide-channel-name
// reference: guide-channel-name-wide
pub fn guide_name(viewport: Viewport) -> Drawn {
    let share = match viewport.matches(GUIDE_NAME_WIDE_AT) {
        true => GUIDE_NAME_WIDE,
        false => GUIDE_NAME,
    };
    share.of(guide_channel(viewport))
}

/// The most of it a channel's number takes.
// reference: guide-channel-number
pub fn guide_number(viewport: Viewport) -> Drawn {
    GUIDE_NUMBER.of(guide_channel(viewport))
}

// reference: control-tab
const TAB_SIDES: Ratio = Ratio::thousandths(1500);

/// A tab's own padding, which the reference writes as `1.5em` of the tab's own
/// lettering on every side.
// reference: control-tab
pub fn tab_pad(viewport: Viewport) -> Padding {
    let side = typeface::tab(viewport).times(TAB_SIDES);
    Padding {
        top: side,
        right: side,
        bottom: side,
        left: side,
    }
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
/// asked for: the frame draws at `preview(viewport).drawn(viewport.layout())` and
/// the tile is asked for at `Fill::of(preview(viewport))`.
// reference: osd-preview
pub fn preview(viewport: Viewport) -> Css {
    match (viewport.orientation(), viewport.matches(PREVIEW_SHORT)) {
        (Orientation::Portrait, _) => PREVIEW_PORTRAIT.of(viewport.width()),
        (Orientation::Landscape, true) => PREVIEW_SHORT_SIDE.of(viewport.height()),
        (Orientation::Landscape, false) => PREVIEW.of(viewport.height()),
    }
}

/// The backdrop's height at this page: two fifths of it, and three tenths on a
/// portrait page no wider than 40em.
// reference: detail-backdrop
// reference: detail-narrow
pub fn backdrop(viewport: Viewport) -> Drawn {
    let share = match (
        viewport.orientation(),
        viewport.matches(BACKDROP_PORTRAIT_AT),
    ) {
        (Orientation::Portrait, true) => BACKDROP_PORTRAIT,
        (Orientation::Portrait | Orientation::Landscape, _) => BACKDROP,
    };
    share.of(viewport.canvas().height())
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

/// The poster over the ribbon: the ribbon's height and four fifths again over
/// the ribbon's foot, and on a page the reference lowers the poster on
/// instead, that same length shed from the foot of the pair.
// reference: detail-poster-arms
// reference: detail-narrow
pub fn detail_poster_overlap(viewport: Viewport) -> Overlap {
    match viewport.matches(DETAIL_POSTER_LOWERED_AT) {
        true => Overlap {
            raised: Length::em(0.0),
            shed: DETAIL_POSTER_RISE,
        },
        false => Overlap {
            raised: DETAIL_POSTER_RISE,
            shed: Length::em(0.0),
        },
    }
}

/// The poster's inset from the leading edge of the page in the stacked
/// arrangement: the page's own side where the poster stands flush inside the
/// ribbon, and a twentieth of the page above that width.
// reference: detail-poster-arms
// reference: page-side
pub fn detail_poster_stacked_inset(viewport: Viewport) -> Drawn {
    match viewport.matches(DETAIL_NARROW) {
        true => page_side(viewport.canvas()),
        false => DETAIL_POSTER_STACKED_INSET.of(viewport.canvas().width()),
    }
}

/// The room the stacked head leaves the poster beside the item's name: three
/// eighths of the ribbon's own content, and one card of the poster wall as the
/// page widens.
// reference: detail-head-inset
pub fn detail_head_inset(viewport: Viewport) -> Drawn {
    let content = RIBBON_CONTENT.of(viewport.canvas().width());
    let mut standing = DETAIL_HEAD_INSET.of(content);
    for (at, across) in DETAIL_HEAD_INSET_STEPS {
        if viewport.matches(at) {
            standing = across.pitch(content);
        }
    }
    standing
}

/// That room under the row of buttons, which the narrowest pages give up so
/// the row stands the width of the ribbon.
// reference: detail-buttons-narrow
pub fn detail_buttons_inset(viewport: Viewport) -> Drawn {
    match viewport.matches(DETAIL_NARROW) {
        true => Drawn::ZERO,
        false => detail_head_inset(viewport),
    }
}

/// The gap under that row, which the narrowest pages close.
// reference: detail-centred
// reference: detail-buttons-narrow
pub fn detail_buttons_bottom(viewport: Viewport) -> Drawn {
    match viewport.matches(DETAIL_NARROW) {
        true => Drawn::ZERO,
        false => DETAIL_BUTTONS_BOTTOM.drawn(),
    }
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
