//! The scrub preview: the frame cropped out of a tile sheet, and where the
//! pointer settled.

use std::time::Duration;

pub use jellium_model::trickplay::{Description, Tile, Trickplay, chapter_at, tile};

/// One frame cropped out of a tile sheet.
pub fn cropped(
    bytes: &[u8],
    description: Description,
    tile: Tile,
) -> Option<iced::widget::image::Handle> {
    let sheet = crate::failure::decoded_image(crate::text::Text::FailureTrickplayUnread, bytes)?;
    #[expect(
        clippy::disallowed_methods,
        reason = "a width outside u32 carries no cause beyond the number itself"
    )]
    let width = u32::try_from(description.width).ok()?;
    #[expect(
        clippy::disallowed_methods,
        reason = "a height outside u32 carries no cause beyond the number itself"
    )]
    let height = u32::try_from(description.height).ok()?;
    if width == 0 || height == 0 {
        return None;
    }
    #[expect(
        clippy::disallowed_methods,
        reason = "a column outside u32 carries no cause beyond the number itself"
    )]
    let x = u32::try_from(tile.column).ok()?.checked_mul(width)?;
    #[expect(
        clippy::disallowed_methods,
        reason = "a row outside u32 carries no cause beyond the number itself"
    )]
    let y = u32::try_from(tile.row).ok()?.checked_mul(height)?;

    use image::GenericImageView;
    let (sheet_width, sheet_height) = sheet.dimensions();
    if x + width > sheet_width || y + height > sheet_height {
        return None;
    }

    let frame = sheet.view(x, y, width, height).to_image();
    Some(iced::widget::image::Handle::from_rgba(
        width,
        height,
        frame.into_raw(),
    ))
}

/// The preview shown now: the frame under the pointer, the sheet it is being
/// cropped from, and where the pointer settled.
#[derive(Debug, Clone)]
pub struct Preview {
    pub at: Duration,
    /// Where the pointer settled, so a preview is asked for only once it has.
    pub settled: Duration,
    pub frame: Option<iced::widget::image::Handle>,
}

/// How long the pointer must settle before a preview is asked for.
pub const SETTLE: Duration = Duration::from_millis(100);
