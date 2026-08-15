use iced::Element;
use iced::widget::{button, column, container, text, text_input};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::{Action, Edit};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    /// The typed text, left on screen by a failed add.
    pub url: String,
}

pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let typed = text_input(strings::lookup(Text::LoginAddTitle), &state.add.url)
        .on_input(|value| Message::LoginAction(Action::Edited(Edit::Url(value))))
        .on_submit(Message::LoginAction(Action::AddSubmit))
        .padding(8);

    let submit = if state.working {
        button(text(strings::lookup(Text::LoginAddWorking)))
    } else {
        button(text(strings::lookup(Text::LoginAddSubmit)))
            .on_press(Message::LoginAction(Action::AddSubmit))
    };

    let mut form = column![
        text(strings::lookup(Text::LoginAddTitle)).size(28),
        typed,
        submit,
    ]
    .spacing(theme::CARD_SPACING)
    .max_width(420);

    if !state.servers.is_empty() {
        form = form.push(
            button(text(strings::lookup(Text::LoginBack)))
                .on_press(Message::LoginAction(Action::Back)),
        );
    }

    container(form)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
