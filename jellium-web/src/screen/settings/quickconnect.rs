//! The Quick Connect screen: authorizing a code another device is showing.

use iced::Element;
use iced::widget::{button, column, text, text_input};
use jellium_model::quickconnect::Outcome;

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;

/// The code being typed, the codes this run authorized, and what the last
/// authorize answered.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub code: String,
    /// The codes this run authorized, which is what tells an expired code from
    /// an unknown one.
    pub authorized: Vec<String>,
    pub outcome: Option<Outcome>,
}

/// The description, the code field, the authorize control — absent under
/// read-only — and the outcome of the last authorize.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut shown = column![
        text(strings::lookup(Text::QuickConnectDescription)),
        text(strings::lookup(Text::QuickConnectCode)),
        text_input("", &state.code).on_input(|typed| Message::SettingsAction(Action::Typed(typed))),
    ]
    .spacing(theme::CARD_SPACING);

    if !read_only {
        shown = shown.push(
            button(text(strings::lookup(Text::QuickConnectAuthorize))).on_press(
                Message::SettingsAction(Action::Ask(crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::AuthorizeQuickConnect {
                        code: state.code.clone(),
                    },
                    state.code.clone(),
                ))),
            ),
        );
    }

    if let Some(Outcome::Authorized) = state.outcome {
        shown = shown.push(text(strings::lookup(Text::QuickConnectAuthorized)));
    }

    shown.into()
}
