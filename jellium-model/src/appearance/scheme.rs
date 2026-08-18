//! The dark scheme's colors, each cited to its rule in the pinned stylesheet.
//! A construct carrying both a measure and a colour is named the same here and
//! in `space`; the module names which of the two a constant is.

use super::{Elevation, Ratio, nearest};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Alpha {
    thousandths: u16,
}

impl Alpha {
    pub const OPAQUE: Alpha = Alpha::thousandths(1000);

    pub const CLEAR: Alpha = Alpha::thousandths(0);

    /// A count of thousandths, the whole being a thousand; a count past the
    /// whole is refused where it is written.
    pub const fn thousandths(count: u16) -> Alpha {
        assert!(count <= 1000, "an alpha is at most a thousand thousandths");
        Alpha { thousandths: count }
    }

    pub fn fraction(self) -> f64 {
        self.thousandths as f64 / 1000.0
    }

    /// A sum past the whole is refused where it is written.
    pub const fn plus(self, other: Alpha) -> Alpha {
        Alpha::thousandths(self.thousandths + other.thousandths)
    }

    /// The alpha as css writes it, to the fewest decimals that carry it.
    fn written(self) -> String {
        let whole = self.thousandths / 1000;
        let fraction = self.thousandths % 1000;
        if fraction == 0 {
            return whole.to_string();
        }
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: Alpha,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::rgba(red, green, blue, Alpha::OPAQUE)
    }

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: Alpha) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// The same color at `alpha`, which is what MUI's `alpha()` answers.
    pub const fn at(self, alpha: Alpha) -> Color {
        Color::rgba(self.red, self.green, self.blue, alpha)
    }

    /// The nearest thousandth of the alpha this leaves, css compositing an
    /// opacity without truncating it.
    pub const fn faded(self, ratio: Ratio) -> Color {
        self.at(Alpha::thousandths(
            nearest(self.alpha.thousandths as f64 * ratio.factor()) as u16,
        ))
    }

    /// MUI's `darken()`: every channel taken down by `coefficient`, the product
    /// truncated the way `recomposeColor` truncates it.
    pub const fn darkened(self, coefficient: Ratio) -> Color {
        let kept = 1.0 - coefficient.factor();
        Color::rgba(
            (self.red as f64 * kept) as u8,
            (self.green as f64 * kept) as u8,
            (self.blue as f64 * kept) as u8,
            self.alpha,
        )
    }

    /// MUI's `lighten()`: every channel raised toward white by `coefficient`,
    /// the product truncated the way `recomposeColor` truncates it.
    // reference: mui-color-manipulator
    pub const fn lightened(self, coefficient: Ratio) -> Color {
        let raise = coefficient.factor();
        Color::rgba(
            (self.red as f64 + (255.0 - self.red as f64) * raise) as u8,
            (self.green as f64 + (255.0 - self.green as f64) * raise) as u8,
            (self.blue as f64 + (255.0 - self.blue as f64) * raise) as u8,
            self.alpha,
        )
    }

    /// This laid over `beneath`, which is what a background image does to the
    /// background color under it.
    pub fn over(self, beneath: Color) -> Color {
        let mixed = |over: u8, under: u8| {
            (over as f64 * self.alpha.fraction() + under as f64 * (1.0 - self.alpha.fraction()))
                as u8
        };
        Color::rgba(
            mixed(self.red, beneath.red),
            mixed(self.green, beneath.green),
            mixed(self.blue, beneath.blue),
            beneath.alpha,
        )
    }

    /// MUI's `getContrastText()`: white where white clears the threshold over
    /// this, and MUI's own near-black where it does not.
    // reference: mui-palette-contrast
    // reference: mui-color-manipulator
    pub fn contrast_text(self) -> Color {
        let over = CONTRAST_LIGHT.luminance();
        let under = self.luminance();
        let ratio = (over.max(under) + 0.05) / (over.min(under) + 0.05);
        match ratio >= CONTRAST_THRESHOLD.factor() {
            true => CONTRAST_LIGHT,
            false => CONTRAST_DARK,
        }
    }

    /// MUI's `getLuminance()`, to the three digits it truncates at.
    // reference: mui-color-manipulator
    fn luminance(self) -> f64 {
        let channel = |value: u8| {
            let value = value as f64 / 255.0;
            match value <= 0.03928 {
                true => value / 12.92,
                false => ((value + 0.055) / 1.055).powf(2.4),
            }
        };
        let sum =
            0.2126 * channel(self.red) + 0.7152 * channel(self.green) + 0.0722 * channel(self.blue);
        (sum * 1000.0).round() / 1000.0
    }

    pub fn red(self) -> u8 {
        self.red
    }

    pub fn green(self) -> u8 {
        self.green
    }

    pub fn blue(self) -> u8 {
        self.blue
    }

    pub fn alpha(self) -> Alpha {
        self.alpha
    }

    /// The color in its shortest css form, which is how the reference writes
    /// every one of its own: `#rgb` where each channel's two hex digits repeat,
    /// `#rrggbb` otherwise, and `rgba(r, g, b, a)` where the alpha is not
    /// opaque, its alpha written to the fewest decimals that carry it.
    pub fn css(self) -> String {
        if self.alpha != Alpha::OPAQUE {
            return format!(
                "rgba({}, {}, {}, {})",
                self.red,
                self.green,
                self.blue,
                self.alpha.written()
            );
        }
        let channels = [self.red, self.green, self.blue];
        if channels.iter().all(|channel| channel >> 4 == channel & 0xf) {
            let [red, green, blue] = channels.map(|channel| channel & 0xf);
            return format!("#{red:x}{green:x}{blue:x}");
        }
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

// reference: scheme-background
pub const BACKGROUND: Color = Color::rgb(0x10, 0x10, 0x10);

// reference: scheme-anchors
pub const SURFACE: Color = Color::rgb(0x20, 0x20, 0x20);

// reference: scheme-text
pub const TEXT: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(800));

// reference: scheme-secondary-text
pub const TEXT_SECONDARY: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(500));

// reference: scheme-label
pub const LABEL: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(700));

// reference: scheme-submit
pub const ACCENT: Color = Color::rgb(0x00, 0xa4, 0xdc);

// reference: scheme-submit
pub const ACCENT_FOCUS: Color = Color::rgb(0x0c, 0xb0, 0xe8);

/// `.button-link`'s own lettering, which the reference draws on no face. The
/// reference writes this hex in `.button-link` and in `.button-submit` as two
/// rules that stand apart, so neither constant is written as the other.
// reference: scheme-button-link
pub const ANCHOR: Color = Color::rgb(0x00, 0xa4, 0xdc);

// reference: scheme-anchors
pub const ERROR: Color = Color::rgb(0xc6, 0x28, 0x28);

// reference: scheme-delete
pub const DELETE: Color = Color::rgb(0xcb, 0x27, 0x2a);

/// `.button-delete`'s lettering.
// reference: scheme-delete
pub const ON_DELETE: Color = Color::rgb(0xff, 0xff, 0xff);

// reference: scheme-anchors
pub const STAR: Color = Color::rgb(0xf2, 0xb0, 0x1e);

// reference: scheme-raised
pub const RAISED: Color = Color::rgb(0x30, 0x30, 0x30);

// reference: scheme-raised
pub const RAISED_FOCUS: Color = Color::rgb(0x38, 0x38, 0x38);

// reference: scheme-submit
pub const ON_ACCENT: Color = Color::rgb(0xff, 0xff, 0xff);

// reference: scheme-raised
pub const ON_RAISED: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(870));

// reference: scheme-input
pub const INPUT: Color = Color::rgb(0x29, 0x29, 0x29);

/// `.emby-select-withcolor`'s own face.
// reference: scheme-select
pub const SELECT: Color = Color::rgb(0x29, 0x29, 0x29);

/// The face `.emby-select-withcolor > option` carries.
// reference: scheme-select-option
pub const SELECT_OPTION: Color = Color::rgb(0x22, 0x22, 0x22);

/// `.checkboxIcon`, the mark a checked box carries.
// reference: control-checkbox-icon
pub const ON_CHECKBOX: Color = Color::rgb(0xff, 0xff, 0xff);

/// The edge a checked box carries where it is reached.
// reference: scheme-checkbox
pub const CHECKBOX_FOCUS: Color = Color::rgb(0xff, 0xff, 0xff);

// reference: card-content
pub const CARD_PADDER: Color = Color::rgb(0x24, 0x24, 0x24);

// reference: card-shadow
pub const SHADOW: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(370));

// reference: scheme-header-transparent
pub const HEADER: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(400));

/// A list row under the pointer. The span's other rule paints a row that holds
/// focus, which is a state no control on this canvas carries.
// reference: scheme-list-state
pub const LIST_HOVER: Color = Color::rgb(0x24, 0x24, 0x24);

// reference: scheme-toast
pub const TOAST: Color = Color::rgb(0x30, 0x30, 0x30);

/// `.toast`'s lettering.
// reference: scheme-toast
pub const ON_TOAST: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(870));

/// The near end of a scrim's gradient, which is the page's own background at
/// three quarters.
// reference: osd-bottom
pub const SCRIM: Color = Color::rgba(0x10, 0x10, 0x10, Alpha::thousandths(750));

/// `.videoOsdBottom`'s lettering.
// reference: osd-bottom
pub const ON_OSD: Color = Color::rgb(0xff, 0xff, 0xff);

/// `.osdHeader`'s lettering, which is dimmer than the panel's.
// reference: osd-header
pub const ON_OSD_HEADER: Color = Color::rgb(0xee, 0xee, 0xee);

// reference: scheme-dialog-backdrop
pub const DIALOG_BACKDROP: Color = Color::rgb(0x00, 0x00, 0x00);

// reference: scheme-scrollbar
pub const SCROLLBAR_THUMB: Color = Color::rgb(0x3b, 0x3b, 0x3b);

// reference: scheme-scrollbar
pub const SCROLLBAR_TRACK: Color = Color::rgb(0x20, 0x20, 0x20);

/// A tab the strip is not showing.
// reference: scheme-tab
pub const TAB_OFFERED: Color = Color::rgb(0x99, 0x99, 0x99);

/// The tab whose body the strip is showing.
// reference: scheme-tab
pub const TAB_SHOWN: Color = Color::rgb(0xff, 0xff, 0xff);

/// A control of a `.localnav` group.
// reference: control-localnav
pub const LOCALNAV: Color = Color::rgb(0x29, 0x29, 0x29);

/// The control whose screen the group is showing.
// reference: scheme-localnav-active
pub const LOCALNAV_SHOWN: Color = Color::rgb(0x00, 0xa4, 0xdc);

/// That control's own lettering.
// reference: scheme-localnav-active
pub const ON_LOCALNAV_SHOWN: Color = Color::rgb(0x29, 0x29, 0x29);

/// The opacity MUI gives a selected control, which the reference raises from
/// MUI's own.
// reference: scheme-anchors
const SELECTED: Alpha = Alpha::thousandths(200);

/// The opacity MUI adds where a control is reached.
// reference: mui-dark-action
const HOVERED: Alpha = Alpha::thousandths(80);

/// A list row whose screen the dashboard is showing.
// reference: mui-list-item-button
pub const LIST_ROW_SELECTED: Color = ACCENT.at(SELECTED);

/// That row where it is reached.
// reference: mui-list-item-button
pub const LIST_ROW_SELECTED_HOVER: Color = ACCENT.at(SELECTED.plus(HOVERED));

/// The glyph an avatar carries, which the reference writes white on the accent.
// reference: tasks-row
pub const ON_AVATAR: Color = Color::rgb(0xff, 0xff, 0xff);

/// What MUI takes a palette colour down by to reach a progress bar's track on a
/// dark scheme.
// reference: mui-linear-progress-dark
const PROGRESS_DARKENED_BY: Ratio = Ratio::thousandths(500);

/// `MuiLinearProgress`'s own track, which MUI darkens out of the accent.
// reference: mui-linear-progress
pub const PROGRESS_TRACK: Color = ACCENT.darkened(PROGRESS_DARKENED_BY);

/// MUI's `action.hover`, the overlay any reached control carries.
// reference: mui-dark-action
pub const ACTION_HOVER: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(80));

/// MUI's `text.primary` on a dark scheme, which every MUI surface writes its
/// lettering in; the reference's own stylesheet lowers the page's to `TEXT`.
// reference: mui-common
// reference: mui-dark-action
pub const ON_SURFACE: Color = Color::rgb(0xff, 0xff, 0xff);

/// MUI's `text.secondary` on a dark scheme, which a filled field's own label
/// and an unticked box are written in.
// reference: mui-dark-action
pub const ON_SURFACE_SECONDARY: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(700));

/// MUI's `action.active`, which it draws a control's own glyph in.
// reference: mui-common
// reference: mui-dark-action
pub const ACTION_ACTIVE: Color = Color::rgb(0xff, 0xff, 0xff);

/// `MuiFilledInput`'s own face.
// reference: mui-filled-root
pub const FILLED: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(90));

/// The face it takes under the pointer.
// reference: mui-filled-root
pub const FILLED_HOVER: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(130));

/// The rule `MuiFilledInput` draws under itself at rest.
// reference: mui-filled-root
// reference: mui-filled-underline
pub const FILLED_RULE: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(700));

/// MUI's own green at its light shade, which its success severity is drawn
/// from on a dark scheme.
// reference: mui-color-green
// reference: mui-palette-success
const SUCCESS_LIGHT: Color = Color::rgb(0x81, 0xc7, 0x84);

/// What MUI lightens that shade by to reach an alert's lettering.
// reference: mui-alert-dark
const ALERT_LIGHTENED_BY: Ratio = Ratio::thousandths(600);

/// What it darkens it by to reach the alert's face.
// reference: mui-alert-dark
const ALERT_DARKENED_BY: Ratio = Ratio::thousandths(900);

/// A success alert's face, which MUI darkens out of the severity's own light
/// shade.
// reference: mui-alert
pub const ALERT_SUCCESS: Color = SUCCESS_LIGHT.darkened(ALERT_DARKENED_BY);

/// Its lettering, which MUI lightens out of that same shade.
// reference: mui-alert
pub const ON_ALERT_SUCCESS: Color = SUCCESS_LIGHT.lightened(ALERT_LIGHTENED_BY);

/// The glyph an alert stands before its sentence, which is the severity itself
/// at the opacity MUI gives that glyph.
// reference: mui-alert
// reference: mui-alert-parts
// reference: mui-color-green
// reference: mui-palette-success
pub const ALERT_SUCCESS_GLYPH: Color = Color::rgb(0x66, 0xbb, 0x6a).at(Alpha::thousandths(900));

/// The two lettering colors MUI chooses between for a filled face.
// reference: mui-common
const CONTRAST_LIGHT: Color = Color::rgb(0xff, 0xff, 0xff);

// reference: mui-light-text
const CONTRAST_DARK: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(870));

/// The ratio white must clear over a face for MUI to write on it in white.
// reference: mui-palette-contrast
const CONTRAST_THRESHOLD: Ratio = Ratio::thousandths(3000);

/// The theme's own tonal offset, which MUI steps a palette color by to reach
/// its light and its dark shade.
// reference: mui-palette-contrast
const TONAL_OFFSET: Ratio = Ratio::thousandths(200);

/// What MUI takes that offset of to reach a dark shade.
// reference: mui-palette-augment
const TONAL_DARK_STEP: Ratio = Ratio::thousandths(1500);

const CONTAINED_DARKENED_BY: Ratio = TONAL_OFFSET.times(TONAL_DARK_STEP);

/// The face a contained button takes under the pointer, which is the accent at
/// MUI's own dark shade.
// reference: mui-button-contained
// reference: mui-palette-augment
pub const CONTAINED_HOVER: Color = ACCENT.darkened(CONTAINED_DARKENED_BY);

/// The elevation MUI's own paper stands at where nothing raises it.
// reference: mui-paper-elevation
pub const PAPER_ELEVATION: Elevation = Elevation::steps(1);

/// The elevation MUI's own popover stands at.
// reference: mui-popover-elevation
pub const POPOVER_ELEVATION: Elevation = Elevation::steps(8);

/// The face a MUI paper draws at `elevation`: `background.paper` under the
/// white that elevation lays over it, which MUI reads off the square of the
/// elevation below one step and off its logarithm at one step and above.
// reference: mui-paper
// reference: mui-overlay
pub fn paper_face(elevation: Elevation) -> Color {
    let steps = elevation.count();
    let raised = match steps < 1.0 {
        true => 5.119_16 * steps * steps,
        false => 4.5 * (steps + 1.0).ln() + 2.0,
    };
    let thousandths = (raised * 10.0).round() as u16;
    CONTRAST_LIGHT
        .at(Alpha::thousandths(thousandths))
        .over(SURFACE)
}

/// MUI's `divider`, which is the edge one segment of the toolbar's group
/// carries.
// reference: mui-dark-action
pub const DIVIDER: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(120));

/// What MUI takes the divider down by to reach the rule a table cell draws.
// reference: mui-table-cell
// reference: mui-table-cell-border
const TABLE_RULE_DARKENED_BY: Ratio = Ratio::thousandths(680);

/// The rule a table cell draws under itself, which MUI darkens out of the
/// divider taken to full opacity.
// reference: mui-table-cell
// reference: mui-table-cell-border
pub const TABLE_RULE: Color = DIVIDER.at(Alpha::OPAQUE).darkened(TABLE_RULE_DARKENED_BY);

/// The segment of the toolbar's group whose view the screen is showing.
// reference: mui-toggle-button
pub const TOGGLE_SHOWN: Color = ON_SURFACE.at(SELECTED);

/// That segment where it is reached.
// reference: mui-toggle-button
pub const TOGGLE_SHOWN_HOVER: Color = ON_SURFACE.at(SELECTED.plus(HOVERED));

/// The rule `.listItem-border` draws under a row.
// reference: scheme-list-border
pub const LIST_RULE: Color = Color::rgba(0x22, 0x22, 0x22, Alpha::thousandths(900));

/// `.programCell-active`, the cell of the programme airing now.
// reference: scheme-program-active
pub const PROGRAM_AIRING: Color = Color::rgb(0x1e, 0x1e, 0x1e);

/// The rule the guide draws between its rows, beside its channel column and
/// down the leading edge of a cell.
// reference: scheme-guide-rule
pub const GUIDE_RULE: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(50));

/// `.cardOverlayContainer`'s own scrim.
// reference: card-overlay-container
pub const CARD_OVERLAY: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(500));

/// The disc `.cardOverlayFab-primary` stands on.
// reference: card-overlay-fab
pub const CARD_OVERLAY_FAB: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(700));

/// The lettering a control on that scrim is drawn in.
// reference: card-overlay-button
pub const ON_CARD_OVERLAY: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(760));

/// `.playstatebutton-icon-played`, the glyph a played mark draws once it is
/// set.
// reference: scheme-played-mark
pub const PLAYED_MARK: Color = Color::rgb(0xcc, 0x33, 0x33);

/// `.ratingbutton-icon-withrating`, the glyph a rating control draws once the
/// item is a favourite.
// reference: scheme-rating-mark
pub const FAVORITE_MARK: Color = Color::rgb(0xcc, 0x33, 0x33);

/// `.timerIcon` and `.seriesTimerIcon`, the glyph a timer covering a guide cell
/// draws.
// reference: guide-timer-icon
pub const TIMER: Color = Color::rgb(0xcc, 0x33, 0x33);

/// What `.seriesTimerIcon-inactive`'s own opacity leaves of the lettering it
/// inherits.
// reference: guide-timer-icon
const INACTIVE: Ratio = Ratio::thousandths(700);

/// `.seriesTimerIcon-inactive`, which is the cell's own lettering faded.
// reference: guide-timer-icon
// reference: scheme-text
pub const TIMER_CANCELLED: Color = TEXT.faded(INACTIVE);

/// `.timerIndicator`, the glyph a timer covering a card's image draws. The
/// reference writes this hex in `.timerIndicator` and in `.button-delete` as
/// two rules that stand apart, so neither constant is written as the other.
// reference: indicator-timer-face
pub const CARD_TIMER: Color = Color::rgb(0xcb, 0x27, 0x2a);

/// `.timerIndicator-inactive`, the glyph a cancelled series timer draws on that
/// image.
// reference: indicator-timer-face
pub const CARD_TIMER_CANCELLED: Color = Color::rgb(0x88, 0x88, 0x88);

/// `.liveTvProgram`.
// reference: guide-indicator-colors
pub const BADGE_LIVE: Color = Color::rgb(0xcc, 0x33, 0x33);

/// `.premiereTvProgram`.
// reference: guide-indicator-colors
pub const BADGE_PREMIERE: Color = Color::rgb(0xef, 0x6c, 0x00);

/// `.newTvProgram`.
// reference: guide-indicator-colors
pub const BADGE_NEW: Color = Color::rgb(0x33, 0x88, 0xcc);

/// The lettering all three write on themselves.
// reference: guide-indicator-colors
pub const ON_BADGE: Color = Color::rgb(0xff, 0xff, 0xff);

/// A node of the metadata manager's sidebar under the pointer.
// reference: metadata-tree
pub const TREE_HOVER: Color = Color::rgb(0x33, 0x88, 0xcc);

/// That node's lettering.
// reference: metadata-tree
pub const ON_TREE_HOVER: Color = Color::rgb(0xff, 0xff, 0xff);

/// The node whose part the manager is showing.
// reference: metadata-tree
pub const TREE_SHOWN: Color = Color::rgb(0x00, 0xa4, 0xdc);

/// The rule down the metadata sidebar's trailing edge.
// reference: metadata-sidebar
pub const EDITOR_RULE: Color = Color::rgb(0x55, 0x55, 0x55);

/// `.filterIndicator`'s face.
// reference: filter-indicator-face
pub const INDICATOR: Color = Color::rgb(0x03, 0xa9, 0xf4);

// reference: scheme-card-background
const CARD_BACKGROUNDS: [Color; 5] = [
    Color::rgb(0x00, 0x45, 0x5c),
    Color::rgb(0x44, 0xba, 0xe1),
    Color::rgb(0x00, 0xa4, 0xdb),
    Color::rgb(0x1c, 0x4c, 0x5c),
    Color::rgb(0x00, 0x7e, 0xa8),
];

/// The background a card with no image draws, taken from the character at the
/// middle of `name` the way the reference takes it: the decimal digits of that
/// character's code summed, the last digit of the sum taken, and that stepped
/// around the five backgrounds.
// reference: card-background-index
pub fn card_background(name: &str) -> Color {
    let middle = name.chars().count() / 2;
    let Some(character) = name.chars().nth(middle) else {
        return CARD_BACKGROUNDS[0];
    };
    let sum: u32 = (character as u32)
        .to_string()
        .chars()
        .filter_map(|digit| digit.to_digit(10))
        .sum();
    let last = sum % 10;
    CARD_BACKGROUNDS[(last % CARD_BACKGROUNDS.len() as u32) as usize]
}

/// `.actionSheetItemAsideText`, which the reference draws at the page's own
/// lettering under its own opacity.
// reference: action-sheet-item-aside
pub const SHEET_ASIDE: Color = TEXT.faded(Ratio::percent(70.0));

/// The rule `.actionsheetDivider` draws between two runs of a sheet.
// reference: scheme-action-sheet-divider
pub const SHEET_DIVIDER: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(140));
