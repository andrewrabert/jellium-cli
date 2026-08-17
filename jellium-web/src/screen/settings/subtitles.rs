//! The subtitle appearance screen; every control here changes how native text
//! cues are drawn and nothing the server burns into the picture.

use iced::Element;
use iced::widget::column;
use jellium_model::prefs::{Held, OPACITIES, SubtitleColour, SubtitleShadow, SubtitleSize};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::{Setting, choices};
use crate::style::typeface;
use crate::widget::prose;

fn size_label(size: SubtitleSize) -> String {
    strings::lookup(match size {
        SubtitleSize::Small => Text::SubtitleSizeSmall,
        SubtitleSize::Medium => Text::SubtitleSizeMedium,
        SubtitleSize::Large => Text::SubtitleSizeLarge,
        SubtitleSize::Huge => Text::SubtitleSizeHuge,
    })
    .to_owned()
}

fn colour_label(colour: SubtitleColour) -> String {
    strings::lookup(match colour {
        SubtitleColour::White => Text::ColourWhite,
        SubtitleColour::Black => Text::ColourBlack,
        SubtitleColour::Red => Text::ColourRed,
        SubtitleColour::Green => Text::ColourGreen,
        SubtitleColour::Blue => Text::ColourBlue,
        SubtitleColour::Yellow => Text::ColourYellow,
        SubtitleColour::Magenta => Text::ColourMagenta,
        SubtitleColour::Cyan => Text::ColourCyan,
    })
    .to_owned()
}

fn shadow_label(shadow: SubtitleShadow) -> String {
    strings::lookup(match shadow {
        SubtitleShadow::None => Text::ShadowNone,
        SubtitleShadow::Drop => Text::ShadowDrop,
        SubtitleShadow::Outline => Text::ShadowOutline,
    })
    .to_owned()
}

fn opacity_label(opacity: i32) -> String {
    format!("{opacity}%")
}

/// Text size, text colour, background colour with its opacity and drop shadow,
/// the sentence stating that burned-in subtitles ignore every setting here, and
/// the save, which is absent under read-only.
pub fn view<'a>(held: Held, read_only: bool) -> Element<'a, Message> {
    let mut shown = column![
        prose(
            strings::lookup(Text::SubtitlesBurnedIn).to_owned(),
            typeface::BODY
        ),
        choices(
            Text::SubtitlesSize,
            &SubtitleSize::ALL,
            held.subtitle_size,
            size_label,
            Setting::SubtitleSize,
        ),
        choices(
            Text::SubtitlesColour,
            &SubtitleColour::ALL,
            held.subtitle_colour,
            colour_label,
            Setting::SubtitleColour,
        ),
        choices(
            Text::SubtitlesBackground,
            &SubtitleColour::ALL,
            held.subtitle_background,
            colour_label,
            Setting::SubtitleBackground,
        ),
        choices(
            Text::SubtitlesOpacity,
            &OPACITIES,
            held.subtitle_opacity,
            opacity_label,
            Setting::SubtitleOpacity,
        ),
        choices(
            Text::SubtitlesShadow,
            &SubtitleShadow::ALL,
            held.subtitle_shadow,
            shadow_label,
            Setting::SubtitleShadow,
        ),
    ]
    .spacing(theme::CARD_SPACING);

    if !read_only {
        shown = shown.push(super::save());
    }

    shown.into()
}
