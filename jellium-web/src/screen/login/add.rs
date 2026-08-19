use iced::Element;

use crate::app::Message;
use crate::style::{Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis, Secrecy};

use super::{Action, Edit};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    /// The typed text, left on screen by a failed add.
    pub url: String,
}

/// The reference's connect form: its title, the server field with its
/// description, and the two block controls.
// reference: add-server-page
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    widget::page(
        viewport,
        widget::capped(
            viewport,
            space::FIELD_GAP,
            [
                widget::heading(typeface::Rank::First, strings::lookup(Text::LoginAddTitle)),
                widget::field(
                    strings::lookup(Text::LoginAddUrl),
                    &state.add.url,
                    Some(Text::LoginAddUrlHelp),
                    None,
                    |value| Message::LoginAction(Action::Edited(Edit::Url(value))),
                    Message::LoginAction(Action::AddSubmit),
                    Secrecy::Shown,
                ),
                widget::block(
                    strings::lookup(match state.working {
                        true => Text::LoginAddWorking,
                        false => Text::LoginAddSubmit,
                    }),
                    (!state.working).then_some(Message::LoginAction(Action::AddSubmit)),
                    Emphasis::Submit,
                ),
                widget::block(
                    strings::lookup(Text::LoginBack),
                    (!state.servers.is_empty()).then_some(Message::LoginAction(Action::Back)),
                    Emphasis::Raised,
                ),
            ],
        ),
    )
}
