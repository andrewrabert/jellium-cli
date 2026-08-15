use iced::theme::{Custom, Palette};
use iced::{Color, Theme};

use std::time::Duration;

/// The display and the cursor hide after this long without input.
pub const IDLE_HIDE: Duration = Duration::from_secs(3);

/// How often the idle timer ages.
pub const TICK: Duration = Duration::from_millis(250);

/// The pass that fires a due group command and corrects drift.
pub const GROUP_TICK: Duration = Duration::from_millis(50);

pub const BAR_HEIGHT: f32 = 84.0;

pub const BAR_ART_WIDTH: f32 = 56.0;

pub const SCRUB_HEIGHT: f32 = 24.0;

pub const CHAPTER_TICK_WIDTH: f32 = 2.0;

pub const OSD_SPACING: f32 = 12.0;

pub const CARD_WIDTH: f32 = 176.0;
pub const CARD_SPACING: f32 = 16.0;
pub const RAIL_HEIGHT: f32 = 288.0;

/// How tall one card stands: its poster, its name and its subtitle, with the
/// spacing beneath it. It is the row height every browse grid is windowed by.
pub const CARD_HEIGHT: f32 = CARD_WIDTH * 1.5 + 48.0 + CARD_SPACING;
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

/// The background a screen drawn over the video element carries.
pub fn over_video(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(theme.palette().background)
}

/// The pass that advances the guide's present-instant marker, the elapsed
/// bars, and a paused live playback's timer.
pub const LIVE_TICK: Duration = Duration::from_secs(1);

/// One row of the guide, the channel list, the queue, the recordings, the
/// schedule and the series lists.
pub const ROW_HEIGHT: f32 = 64.0;

/// How tall one line of a log file stands.
pub const LOG_LINE: f32 = 20.0;

/// How tall one activity entry stands.
pub const ENTRY_HEIGHT: f32 = 48.0;

/// The width the guide's time axis gives one minute.
pub const GUIDE_MINUTE: f32 = 6.0;

/// The column the guide's channel names occupy.
pub const GUIDE_CHANNEL_WIDTH: f32 = 176.0;

/// The width of the guide's present-instant marker.
pub const GUIDE_MARKER_WIDTH: f32 = 2.0;

/// The height of an elapsed bar.
pub const ELAPSED_HEIGHT: f32 = 4.0;

/// A message another client sent stops showing after this long.
pub const NOTICE_HIDE: Duration = Duration::from_secs(6);
