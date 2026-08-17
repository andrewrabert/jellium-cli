//! The password screen: the current password, the replacement, and what
//! changing it does to every other device.

use iced::Element;
use iced::widget::{button, column, text_input};

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::Action;
use crate::style::{self, space, typeface};
use crate::widget::prose;

/// What has been typed into the two password fields.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub current: String,
    pub replacement: String,
}

/// The two fields, the sentence stating that the server signs out every other
/// device signed in as this account, and the control, which is absent under
/// read-only.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut shown = column![
        prose(
            strings::lookup(Text::PasswordOtherDevices).to_owned(),
            typeface::BODY
        ),
        prose(
            strings::lookup(Text::PasswordCurrent).to_owned(),
            typeface::BODY
        ),
        text_input("", &state.current)
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::SettingsAction(Action::TypedCurrentPassword(typed))),
        prose(
            strings::lookup(Text::PasswordNew).to_owned(),
            typeface::BODY
        ),
        text_input("", &state.replacement)
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::SettingsAction(Action::TypedNewPassword(typed))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if !read_only {
        shown = shown.push(
            button(prose(
                strings::lookup(Text::PasswordChange).to_owned(),
                typeface::BODY,
            ))
            .on_press(Message::SettingsAction(Action::ChangePassword)),
        );
    }

    shown.into()
}
