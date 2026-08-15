//! The completion message and the one action that posts `Startup/Complete`.

use iced::Element;
use iced::widget::{button, column, text};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;

/// The completion message and the action; no restart is offered, expected or
/// reported.
pub fn view<'a>() -> Element<'a, Message> {
    column![
        text(strings::lookup(Text::SetupFinish)).size(20),
        text(strings::lookup(Text::SetupFinishMessage)),
        button(text(strings::lookup(Text::SetupFinishAction)))
            .on_press(Message::SetupAction(Action::Complete)),
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}
