use iced::Element;
use iced::widget::{button, column, container, text};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    /// The code shown; the secret is never here and never arrives.
    pub code: Option<String>,
    /// Where the request stands, which is what decides the screen's text and
    /// whether it offers a retry.
    pub standing: Option<jellium_model::quickconnect::SignIn>,
}

pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let mut shown = column![
        text(strings::lookup(Text::LoginQuickConnectTitle)).size(28),
        text(strings::lookup(Text::LoginQuickConnectInstruction)),
    ]
    .spacing(theme::CARD_SPACING)
    .max_width(480);

    if let Some(code) = &state.quick_connect.code {
        shown = shown
            .push(text(strings::lookup(Text::LoginQuickConnectCode)).size(14))
            .push(text(code.clone()).size(36));
    }

    match state.quick_connect.standing {
        None | Some(jellium_model::quickconnect::SignIn::Pending) => {
            shown = shown.push(text(strings::lookup(Text::LoginQuickConnectWaiting)));
        }
        Some(jellium_model::quickconnect::SignIn::Expired) => {
            shown = shown.push(
                button(text(strings::lookup(Text::LoginQuickConnectRetry)))
                    .on_press(Message::LoginAction(Action::QuickConnectRetry)),
            );
        }
        Some(
            jellium_model::quickconnect::SignIn::Disabled
            | jellium_model::quickconnect::SignIn::Authorized,
        ) => {}
    }

    shown = shown.push(
        button(text(strings::lookup(Text::LoginBack))).on_press(Message::LoginAction(Action::Back)),
    );

    container(shown)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
