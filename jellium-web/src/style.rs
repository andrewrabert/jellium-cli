//! The one place the ported appearance values cross into iced.
//!
//! Every length below is a canvas length, because the canvas carries the band's
//! root size as its scale and so resolves every em once for the whole surface.

pub use jellium_model::appearance::{
    Band, Css, Drawn, Length, Share, Viewport, scheme, space, typeface,
};

/// The one site a ported length becomes a number iced takes.
pub fn drawn(length: Drawn) -> f32 {
    length.count()
}

pub fn color(color: scheme::Color) -> iced::Color {
    iced::Color::from_rgba8(
        color.red(),
        color.green(),
        color.blue(),
        color.alpha().fraction(),
    )
}

pub fn radius() -> iced::border::Radius {
    iced::border::Radius::new(drawn(space::RADIUS.drawn()))
}

/// The dark scheme as iced's own palette, whose six slots the reference fills
/// five of: it declares no success color, so the accent stands in that slot and
/// nothing this client draws reads it.
pub fn theme() -> iced::Theme {
    iced::Theme::Custom(std::sync::Arc::new(iced::theme::Custom::new(
        crate::text::lookup(crate::text::Text::AppName).to_owned(),
        iced::theme::Palette {
            background: color(scheme::BACKGROUND),
            text: color(scheme::TEXT),
            primary: color(scheme::ACCENT),
            success: color(scheme::ACCENT),
            warning: color(scheme::STAR),
            danger: color(scheme::ERROR),
        },
    )))
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

/// The background a screen drawn over the video element carries.
pub fn over_video(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::BACKGROUND))
}

/// Whether a control is drawing its resting face or the one the reference gives
/// it under the pointer, under the keyboard focus ring, or pressed.
fn lit(status: iced::widget::button::Status) -> bool {
    match status {
        iced::widget::button::Status::Active | iced::widget::button::Status::Disabled => false,
        iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => true,
    }
}

/// A face in the scheme's own colors, rounded the way every control here is.
fn faced(background: scheme::Color, text: scheme::Color) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(iced::Background::Color(color(background))),
        text_color: color(text),
        border: iced::Border {
            radius: radius(),
            ..iced::Border::default()
        },
        ..iced::widget::button::Style::default()
    }
}

/// The reference's `.raised`, whose focus face is the accent.
// reference: scheme-raised
// reference: scheme-focus
pub fn raised(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::ACCENT, scheme::ON_ACCENT);
    }
    faced(scheme::RAISED, scheme::ON_RAISED)
}

/// The reference's `.button-submit`.
// reference: scheme-submit
pub fn submit(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::ACCENT_FOCUS, scheme::ON_ACCENT);
    }
    faced(scheme::ACCENT, scheme::ON_ACCENT)
}

/// A control carrying no face of its own until it is reached, which is what a
/// card, a row and an icon button are.
// reference: scheme-list-state
pub fn flat(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::LIST_HOVER, scheme::TEXT);
    }
    iced::widget::button::Style {
        text_color: color(scheme::TEXT),
        ..iced::widget::button::Style::default()
    }
}

// reference: scheme-input
pub fn input(
    _theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let edge = match status {
        iced::widget::text_input::Status::Focused { .. } => scheme::ACCENT,
        iced::widget::text_input::Status::Active
        | iced::widget::text_input::Status::Hovered
        | iced::widget::text_input::Status::Disabled => scheme::INPUT,
    };
    iced::widget::text_input::Style {
        background: iced::Background::Color(color(scheme::INPUT)),
        border: iced::Border {
            color: color(edge),
            width: drawn(space::INPUT_BORDER.drawn()),
            radius: radius(),
        },
        icon: color(scheme::TEXT_SECONDARY),
        placeholder: color(scheme::TEXT_SECONDARY),
        value: color(scheme::TEXT),
        selection: color(scheme::ACCENT),
    }
}
