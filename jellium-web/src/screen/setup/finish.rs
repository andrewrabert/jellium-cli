//! The completion message and the one action that posts `Startup/Complete`.

use iced::Element;
use iced::widget::{button, column};

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::Action;
use crate::style::{self, space, typeface};
use crate::widget::prose;

/// The completion message and the action; no restart is offered, expected or
/// reported.
pub fn view<'a>() -> Element<'a, Message> {
    column![
        prose(strings::lookup(Text::SetupFinish), typeface::HEADING_3),
        prose(strings::lookup(Text::SetupFinishMessage), typeface::BODY),
        button(prose(
            strings::lookup(Text::SetupFinishAction),
            typeface::BODY
        ))
        .on_press(Message::SetupAction(Action::Complete)),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .into()
}
