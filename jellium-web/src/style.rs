//! The one place the ported appearance values cross into iced.
//!
//! Every length below is a canvas length, because the canvas carries the band's
//! root size as its scale and so resolves every em once for the whole surface.

pub use jellium_model::appearance::{Band, Css, Drawn, Length, Share, Viewport, typeface};

/// The one site a ported length becomes a number iced takes.
pub fn drawn(length: Drawn) -> f32 {
    length.count()
}

/// The family the reference's own base faces register under. Private because
/// `iced::font::Family::Name` is the foreign boundary that can carry only a
/// string and `font` is the one site that crosses it.
const FAMILY: &str = "Noto Sans";

pub fn font(weight: typeface::Weight) -> iced::Font {
    iced::Font {
        weight: match weight {
            typeface::Weight::Regular => iced::font::Weight::Normal,
            typeface::Weight::Bold => iced::font::Weight::Bold,
        },
        ..iced::Font::with_name(FAMILY)
    }
}

/// The canvas scale the band draws at, which is what resolves every em.
pub fn scale(band: Band) -> f32 {
    band.root().factor()
}
