use iced::Element;

use crate::app::Message;
use crate::style::{Viewport, space, typeface};
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
    let told = match held.answered {
        Some(jellium_model::login::Reset::ContactAdministrator) => {
            Some(Text::LoginResetContactAdministrator)
        }
        Some(jellium_model::login::Reset::InNetworkRequired) => Some(Text::LoginResetInNetwork),
        Some(jellium_model::login::Reset::PinWritten) | None => None,
    };
    let written = matches!(held.answered, Some(jellium_model::login::Reset::PinWritten))
        .then(|| redeeming(held));
    widget::capped(
        viewport,
        space::FIELD_GAP,
        [
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
        ]
        .into_iter()
        .chain(written.into_iter().flatten())
        .chain(told.map(|text| widget::prose(strings::lookup(text), typeface::BODY)))
        .chain([widget::block(
            strings::lookup(Text::LoginBack),
            Some(Message::LoginAction(Action::Back)),
            Emphasis::Raised,
        )]),
    )
}

/// The rows the form gains once the server has written a pin: what it says,
/// the file it was written to, when it expires, and the field and the control
/// that redeem it.
fn redeeming<'a>(held: &'a State) -> impl Iterator<Item = Element<'a, Message>> {
    [widget::prose(
        strings::lookup(Text::LoginResetPinWritten),
        typeface::BODY,
    )]
    .into_iter()
    .chain(held.pin_file.iter().flat_map(|file| {
        [
            widget::prose(strings::lookup(Text::LoginResetPinFile), typeface::BODY),
            widget::prose(format!("> {file}"), typeface::SECONDARY),
        ]
    }))
    .chain(held.expires.map(|expires| {
        widget::prose(
            strings::format(Text::LoginResetExpires, &[&stamped(expires)]),
            typeface::BODY,
        )
    }))
    .chain([
        widget::field(
            Text::LoginResetPin,
            &held.pin,
            None,
            |value| Message::LoginAction(Action::Edited(Edit::Pin(value))),
            Message::LoginAction(Action::PinSubmit),
            Secrecy::Shown,
        ),
        widget::block(
            strings::lookup(Text::LoginResetPinSubmit),
            Some(Message::LoginAction(Action::PinSubmit)),
            Emphasis::Submit,
        ),
    ])
}

/// Milliseconds since the unix epoch as a local timestamp.
fn stamped(millis: i64) -> String {
    chrono::DateTime::from_timestamp_millis(millis)
        .map(|when| when.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
