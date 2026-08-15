//! The keyboard controls screen: every key the player honours, read from the
//! one table the player matches against.

use iced::Element;
use iced::widget::{column, row, text};

use crate::app::Message;
use crate::player::binding::BINDINGS;
use crate::text::{self as strings, Text};
use crate::theme;

/// Every entry of `player::binding::BINDINGS`, each naming its key and what it
/// does, and no control that changes one.
pub fn view<'a>() -> Element<'a, Message> {
    let mut shown = column![
        row![
            text(strings::lookup(Text::ControlsKey)),
            text(strings::lookup(Text::ControlsDoes)),
        ]
        .spacing(theme::CARD_SPACING)
    ]
    .spacing(theme::CARD_SPACING);

    for binding in BINDINGS {
        shown = shown.push(
            row![
                text(strings::lookup(binding.named)),
                text(strings::lookup(binding.does.text())),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    shown.into()
}
