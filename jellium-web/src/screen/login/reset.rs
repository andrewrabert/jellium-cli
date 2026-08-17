use iced::Element;
use iced::widget::{button, column, container, text_input};

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

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

pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let held = &state.reset;
    let mut form = column![
        prose(strings::lookup(Text::LoginResetTitle), typeface::HEADING_1),
        text_input(strings::lookup(Text::LoginResetUsername), &held.username)
            .style(style::input)
            .on_input(|value| Message::LoginAction(Action::Edited(Edit::ResetUsername(value))))
            .on_submit(Message::LoginAction(Action::ResetSubmit))
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
        button(prose(
            strings::lookup(Text::LoginResetSubmit),
            typeface::BODY
        ))
        .on_press(Message::LoginAction(Action::ResetSubmit)),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .max_width(520);

    match held.answered {
        Some(jellium_model::login::Reset::PinWritten) => {
            form = form.push(prose(
                strings::lookup(Text::LoginResetPinWritten),
                typeface::BODY,
            ));
            if let Some(file) = &held.pin_file {
                form = form
                    .push(prose(
                        strings::lookup(Text::LoginResetPinFile),
                        typeface::BODY,
                    ))
                    .push(prose(format!("> {file}"), typeface::SECONDARY));
            }
            if let Some(expires) = held.expires {
                form = form.push(prose(
                    crate::text::format(Text::LoginResetExpires, &[&stamped(expires)]),
                    typeface::BODY,
                ));
            }
            form = form
                .push(
                    text_input(strings::lookup(Text::LoginResetPin), &held.pin)
                        .style(style::input)
                        .on_input(|value| Message::LoginAction(Action::Edited(Edit::Pin(value))))
                        .on_submit(Message::LoginAction(Action::PinSubmit))
                        .padding(style::drawn(space::CONTROL_GAP.drawn())),
                )
                .push(
                    button(prose(
                        strings::lookup(Text::LoginResetPinSubmit),
                        typeface::BODY,
                    ))
                    .on_press(Message::LoginAction(Action::PinSubmit)),
                );
        }
        Some(jellium_model::login::Reset::ContactAdministrator) => {
            form = form.push(prose(
                strings::lookup(Text::LoginResetContactAdministrator),
                typeface::BODY,
            ));
        }
        Some(jellium_model::login::Reset::InNetworkRequired) => {
            form = form.push(prose(
                strings::lookup(Text::LoginResetInNetwork),
                typeface::BODY,
            ));
        }
        None => {}
    }

    form = form.push(
        button(prose(strings::lookup(Text::LoginBack), typeface::BODY))
            .on_press(Message::LoginAction(Action::Back)),
    );

    container(form)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}

/// Milliseconds since the unix epoch as a local timestamp.
fn stamped(millis: i64) -> String {
    chrono::DateTime::from_timestamp_millis(millis)
        .map(|when| when.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
