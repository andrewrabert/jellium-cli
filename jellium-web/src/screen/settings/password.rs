//! The password screen: the current password, the replacement, and what
//! changing it does to every other device.

use iced::Element;

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::widget;

use super::Action;

/// What has been typed into the two password fields.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub current: String,
    pub replacement: String,
}

/// The two password fields over the sentence stating that the server signs out
/// every other device signed in as this account, in their own section, and the
/// control that writes, which is absent under read-only.
// reference: settings-password-form
pub fn sections<'a>(state: &'a State, read_only: bool) -> Vec<Element<'a, Message>> {
    let submits = match read_only {
        true => Message::Unchanged,
        false => Message::SettingsAction(Action::ChangePassword),
    };

    let mut sections = vec![widget::fields(
        Text::PasswordChange,
        [
            widget::field(
                Text::PasswordCurrent,
                &state.current,
                None,
                |typed| Message::SettingsAction(Action::TypedCurrentPassword(typed)),
                submits.clone(),
                widget::Secrecy::Hidden,
            ),
            widget::field(
                Text::PasswordNew,
                &state.replacement,
                Some(Text::PasswordOtherDevices),
                |typed| Message::SettingsAction(Action::TypedNewPassword(typed)),
                submits,
                widget::Secrecy::Hidden,
            ),
        ],
    )];

    if !read_only {
        sections.push(widget::block(
            strings::lookup(Text::SettingsSave),
            Some(Message::SettingsAction(Action::ChangePassword)),
            widget::Emphasis::Submit,
        ));
    }

    sections
}
