use iced::Element;
use iced::widget::{button, column, container};

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::Action;
use crate::style::{self, space, typeface};
use crate::widget::prose;

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
        prose(
            strings::lookup(Text::LoginQuickConnectTitle),
            typeface::HEADING_1
        ),
        prose(
            strings::lookup(Text::LoginQuickConnectInstruction),
            typeface::BODY
        ),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .max_width(480);

    if let Some(code) = &state.quick_connect.code {
        shown = shown
            .push(prose(
                strings::lookup(Text::LoginQuickConnectCode),
                typeface::BODY,
            ))
            .push(prose(code.clone(), typeface::HEADING_1));
    }

    match state.quick_connect.standing {
        None | Some(jellium_model::quickconnect::SignIn::Pending) => {
            shown = shown.push(prose(
                strings::lookup(Text::LoginQuickConnectWaiting),
                typeface::BODY,
            ));
        }
        Some(jellium_model::quickconnect::SignIn::Expired) => {
            shown = shown.push(
                button(prose(
                    strings::lookup(Text::LoginQuickConnectRetry),
                    typeface::BODY,
                ))
                .on_press(Message::LoginAction(Action::QuickConnectRetry)),
            );
        }
        Some(
            jellium_model::quickconnect::SignIn::Disabled
            | jellium_model::quickconnect::SignIn::Authorized,
        ) => {}
    }

    shown = shown.push(
        button(prose(strings::lookup(Text::LoginBack), typeface::BODY))
            .on_press(Message::LoginAction(Action::Back)),
    );

    container(shown)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
