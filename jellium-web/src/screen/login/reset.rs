use iced::Element;

use crate::app::Message;
use crate::style::{Viewport, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis, Secrecy};

use super::{Action, Edit};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    pub username: String,
    pub pin: String,
    /// Which of Jellyfin's three answers came back.
    pub answered: Option<jellium_model::login::Reset>,
    /// The path the pin was written to, shown as quoted server output.
    pub pin_file: Option<String>,
    /// Milliseconds since the unix epoch on the local server's clock.
    pub expires: Option<i64>,
}

/// The reference's forgot-password form, and its reset form once the server
/// has written a pin: a heading, the field with its description, the submit
/// block and the cancel block.
// reference: forgot-password-page
// reference: reset-password-page
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    let held = &state.reset;
    let mut rows = vec![
        widget::heading(strings::lookup(Text::LoginResetTitle)),
        widget::field(
            Text::LoginResetUsername,
            &held.username,
            Some(Text::LoginResetHelp),
            |value| Message::LoginAction(Action::Edited(Edit::ResetUsername(value))),
            Message::LoginAction(Action::ResetSubmit),
            Secrecy::Shown,
        ),
        widget::block(
            strings::lookup(Text::LoginResetSubmit),
            Some(Message::LoginAction(Action::ResetSubmit)),
            Emphasis::Submit,
        ),
    ];

    match held.answered {
        Some(jellium_model::login::Reset::PinWritten) => {
            rows.push(widget::prose(
                strings::lookup(Text::LoginResetPinWritten),
                typeface::BODY,
            ));
            if let Some(file) = &held.pin_file {
                rows.push(widget::prose(
                    strings::lookup(Text::LoginResetPinFile),
                    typeface::BODY,
                ));
                rows.push(widget::prose(format!("> {file}"), typeface::SECONDARY));
            }
            if let Some(expires) = held.expires {
                rows.push(widget::prose(
                    strings::format(Text::LoginResetExpires, &[&stamped(expires)]),
                    typeface::BODY,
                ));
            }
            rows.push(widget::field(
                Text::LoginResetPin,
                &held.pin,
                None,
                |value| Message::LoginAction(Action::Edited(Edit::Pin(value))),
                Message::LoginAction(Action::PinSubmit),
                Secrecy::Shown,
            ));
            rows.push(widget::block(
                strings::lookup(Text::LoginResetPinSubmit),
                Some(Message::LoginAction(Action::PinSubmit)),
                Emphasis::Submit,
            ));
        }
        Some(jellium_model::login::Reset::ContactAdministrator) => {
            rows.push(widget::prose(
                strings::lookup(Text::LoginResetContactAdministrator),
                typeface::BODY,
            ));
        }
        Some(jellium_model::login::Reset::InNetworkRequired) => {
            rows.push(widget::prose(
                strings::lookup(Text::LoginResetInNetwork),
                typeface::BODY,
            ));
        }
        None => {}
    }

    rows.push(widget::block(
        strings::lookup(Text::LoginBack),
        Some(Message::LoginAction(Action::Back)),
        Emphasis::Raised,
    ));

    widget::form(viewport, rows)
}

/// Milliseconds since the unix epoch as a local timestamp.
fn stamped(millis: i64) -> String {
    chrono::DateTime::from_timestamp_millis(millis)
        .map(|when| when.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
