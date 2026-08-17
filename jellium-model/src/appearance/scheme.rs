//! The dark scheme's colors, each cited to its rule in the pinned stylesheet.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Alpha {
    thousandths: u16,
}

impl Alpha {
    pub const OPAQUE: Alpha = Alpha::thousandths(1000);

    pub const fn thousandths(thousandths: u16) -> Alpha {
        Alpha { thousandths }
    }

    pub fn fraction(self) -> f32 {
        self.thousandths as f32 / 1000.0
    }

    /// The alpha as css writes it, to the fewest decimals that carry it.
    fn written(self) -> String {
        super::trimmed(self.fraction())
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

// reference: scheme-dialog-backdrop
pub const DIALOG_BACKDROP: Color = Color::rgb(0x00, 0x00, 0x00);

// reference: scheme-scrollbar
pub const SCROLLBAR_THUMB: Color = Color::rgb(0x3b, 0x3b, 0x3b);

// reference: scheme-scrollbar
pub const SCROLLBAR_TRACK: Color = Color::rgb(0x20, 0x20, 0x20);

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
