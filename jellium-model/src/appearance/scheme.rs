//! The dark scheme's colors, each cited to its rule in the pinned stylesheet.
//! A construct carrying both a measure and a colour is named the same here and
//! in `space`; the module names which of the two a constant is.

use super::Ratio;

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

    pub fn fraction(self) -> f32 {
        self.thousandths as f32 / 1000.0
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

    /// MUI's `darken()`: every channel taken down by `coefficient`, the product
    /// truncated the way `recomposeColor` truncates it.
    pub const fn darkened(self, coefficient: Ratio) -> Color {
        let kept = 1.0 - coefficient.factor();
        Color::rgba(
            (self.red as f32 * kept) as u8,
            (self.green as f32 * kept) as u8,
            (self.blue as f32 * kept) as u8,
            self.alpha,
        )
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

// reference: scheme-anchors
pub const SECONDARY: Color = Color::rgb(0xaa, 0x5c, 0xc3);

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

// reference: scheme-backdrop-scrim
pub const BACKDROP_SCRIM: Color = Color::rgba(0x00, 0x00, 0x00, Alpha::thousandths(860));

// reference: scheme-list-state
pub const LIST_HOVER: Color = Color::rgb(0x24, 0x24, 0x24);

// reference: scheme-list-state
pub const LIST_FOCUS: Color = Color::rgb(0x33, 0x33, 0x33);

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

/// A drawer entry whose screen the dashboard is showing.
// reference: drawer-entry
pub const DRAWER_SHOWN: Color = ACCENT.at(SELECTED);

/// That entry where it is reached.
// reference: drawer-entry
pub const DRAWER_SHOWN_HOVER: Color = ACCENT.at(SELECTED.plus(HOVERED));

/// MUI's `action.hover`, the overlay any reached control carries.
// reference: mui-dark-action
pub const ACTION_HOVER: Color = Color::rgba(0xff, 0xff, 0xff, Alpha::thousandths(80));

/// MUI's `text.primary` on a dark scheme, which every MUI surface writes its
/// lettering in; the reference's own stylesheet lowers the page's to `TEXT`.
// reference: mui-common
// reference: mui-dark-action
pub const ON_SURFACE: Color = Color::rgb(0xff, 0xff, 0xff);

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
pub const CARD_BACKGROUNDS: [Color; 5] = [
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
