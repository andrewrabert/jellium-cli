//! The display screen: whether missing episodes are drawn.

use iced::Element;

use crate::app::Message;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::widget;

use super::Action;

/// The `DisplayMissingEpisodes` flag over its help in the screen's own section.
// reference: settings-display-form
pub fn sections<'a>(configuration: &'a jellium_model::form::Form) -> Vec<Element<'a, Message>> {
    vec![widget::fields(
        typeface::Rank::Second,
        Text::SettingsDisplay,
        [widget::flag(
            strings::lookup(Text::DisplayMissingEpisodes),
            Some(Text::DisplayMissingEpisodesHelp),
            configuration.flagged(jellium_model::user::MISSING_EPISODES),
            |on| {
                Message::SettingsAction(Action::Flagged(jellium_model::user::MISSING_EPISODES, on))
            },
        )],
    )]
}
