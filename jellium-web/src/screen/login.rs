use iced::Element;
use iced::widget::{button, column, container, text, text_input};
use jellium_protocol::Credentials;

use crate::app::Message;
use crate::error::Trouble;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

#[derive(Debug, Default)]
pub struct State {
    pub server: String,
    pub username: String,
    pub password: String,
    pub submitting: bool,
    /// Shown under the form; the typed fields are never cleared with it.
    pub notice: Option<Trouble>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Server,
    Username,
    Password,
}

impl State {
    pub fn edit(&mut self, field: Field, value: String) {
        match field {
            Field::Server => self.server = value,
            Field::Username => self.username = value,
            Field::Password => self.password = value,
        }
    }

    pub fn credentials(&self) -> Credentials {
        Credentials {
            server: self.server.trim().to_string(),
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

fn field<'a>(label: Text, value: &'a str, which: Field, secure: bool) -> Element<'a, Message> {
    let input = text_input(strings::lookup(label), value)
        .on_input(move |value| Message::LoginEdited(which, value))
        .on_submit(Message::LoginSubmitted)
        .secure(secure)
        .padding(8);

    column![text(strings::lookup(label)).size(14), input]
        .spacing(4)
        .into()
}

pub fn view(state: &State) -> Element<'_, Message> {
    let submit = if state.submitting {
        button(text(strings::lookup(Text::LoginWorking)))
    } else {
        button(text(strings::lookup(Text::LoginSubmit))).on_press(Message::LoginSubmitted)
    };

    let mut form = column![
        text(strings::lookup(Text::LoginTitle)).size(28),
        field(Text::LoginServer, &state.server, Field::Server, false),
        field(Text::LoginUsername, &state.username, Field::Username, false),
        field(Text::LoginPassword, &state.password, Field::Password, true),
        submit,
    ]
    .spacing(theme::CARD_SPACING)
    .max_width(420);

    if let Some(notice) = &state.notice {
        form = form.push(widget::notice(notice.message()));
    }

    container(form)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
