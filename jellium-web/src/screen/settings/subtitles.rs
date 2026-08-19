//! The subtitle appearance screen; every control here changes how native text
//! cues are drawn and nothing the server burns into the picture.

use iced::Element;
use jellium_model::prefs::{Held, OPACITIES, SubtitleColour, SubtitleShadow, SubtitleSize};

use crate::app::Message;
use crate::style::{space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;

use super::{Action, Setting};

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

/// The sentence stating that burned-in subtitles ignore every setting here,
/// then text size, text colour, background colour with its opacity and drop
/// shadow, each a dropdown, in the screen's own section.
// reference: settings-subtitles-form
pub fn sections<'a>(held: Held) -> Vec<Element<'a, Message>> {
    vec![widget::fields(
        typeface::Rank::Second,
        Text::SettingsSubtitles,
        [
            widget::description(Text::SubtitlesBurnedIn, space::DESCRIPTION_INSET),
            widget::select(
                strings::lookup(Text::SubtitlesSize),
                None,
                super::choices(SubtitleSize::ALL, size_label, |size| {
                    Action::Set(Setting::SubtitleSize(size))
                }),
                &Action::Set(Setting::SubtitleSize(held.subtitle_size)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::SubtitlesColour),
                None,
                super::choices(SubtitleColour::ALL, colour_label, |colour| {
                    Action::Set(Setting::SubtitleColour(colour))
                }),
                &Action::Set(Setting::SubtitleColour(held.subtitle_colour)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::SubtitlesBackground),
                None,
                super::choices(SubtitleColour::ALL, colour_label, |colour| {
                    Action::Set(Setting::SubtitleBackground(colour))
                }),
                &Action::Set(Setting::SubtitleBackground(held.subtitle_background)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::SubtitlesOpacity),
                None,
                super::choices(OPACITIES, opacity_label, |opacity| {
                    Action::Set(Setting::SubtitleOpacity(opacity))
                }),
                &Action::Set(Setting::SubtitleOpacity(held.subtitle_opacity)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::SubtitlesShadow),
                None,
                super::choices(SubtitleShadow::ALL, shadow_label, |shadow| {
                    Action::Set(Setting::SubtitleShadow(shadow))
                }),
                &Action::Set(Setting::SubtitleShadow(held.subtitle_shadow)),
                Message::SettingsAction,
            ),
        ],
    )]
}
