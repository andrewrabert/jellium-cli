use iced::Element;
use iced::widget::{button, column, container, text_input};

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    /// The typed text, left on screen by a failed add.
    pub url: String,
}

pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let typed = text_input(strings::lookup(Text::LoginAddTitle), &state.add.url)
        .style(style::input)
        .on_input(|value| Message::LoginAction(Action::Edited(Edit::Url(value))))
        .on_submit(Message::LoginAction(Action::AddSubmit))
        .padding(style::drawn(space::CONTROL_GAP.drawn()));

    let submit = if state.working {
        button(prose(
            strings::lookup(Text::LoginAddWorking).to_owned(),
            typeface::BODY,
        ))
    } else {
        button(prose(
            strings::lookup(Text::LoginAddSubmit).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::LoginAction(Action::AddSubmit))
    };

    let mut form = column![
        prose(
            strings::lookup(Text::LoginAddTitle).to_owned(),
            typeface::HEADING_1
        ),
        typed,
        submit,
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .max_width(420);

    if !state.servers.is_empty() {
        form = form.push(
            button(prose(
                strings::lookup(Text::LoginBack).to_owned(),
                typeface::BODY,
            ))
            .on_press(Message::LoginAction(Action::Back)),
        );
    }

    container(form)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
