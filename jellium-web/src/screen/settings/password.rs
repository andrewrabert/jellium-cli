//! The password screen: the current password, the replacement, and what
//! changing it does to every other device.

use iced::Element;
use iced::widget::{button, column, text, text_input};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;

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
        text(strings::lookup(Text::PasswordOtherDevices)),
        text(strings::lookup(Text::PasswordCurrent)),
        text_input("", &state.current)
            .secure(true)
            .on_input(|typed| Message::SettingsAction(Action::TypedCurrentPassword(typed))),
        text(strings::lookup(Text::PasswordNew)),
        text_input("", &state.replacement)
            .secure(true)
            .on_input(|typed| Message::SettingsAction(Action::TypedNewPassword(typed))),
    ]
    .spacing(theme::CARD_SPACING);

    if !read_only {
        shown = shown.push(
            button(text(strings::lookup(Text::PasswordChange)))
                .on_press(Message::SettingsAction(Action::ChangePassword)),
        );
    }

    shown.into()
}
