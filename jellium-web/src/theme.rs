use iced::theme::{Custom, Palette};
use iced::{Color, Theme};

pub const CARD_WIDTH: f32 = 176.0;
pub const CARD_SPACING: f32 = 16.0;
pub const RAIL_HEIGHT: f32 = 288.0;
pub const IMAGE_WIDTH: u16 = 352;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// Jellium Web's own palette: deep neutral ground, warm accent, muted surfaces.
pub fn theme() -> Theme {
    Theme::Custom(std::sync::Arc::new(Custom::new(
        "Jellium".to_string(),
        Palette {
            background: rgb(0x12, 0x13, 0x1a),
            text: rgb(0xe8, 0xe9, 0xef),
            primary: rgb(0xd8, 0x8c, 0x4a),
            success: rgb(0x6f, 0xbf, 0x73),
            warning: rgb(0xd8, 0xb4, 0x4a),
            danger: rgb(0xff, 0xb4, 0xa8),
        },
    )))
}
