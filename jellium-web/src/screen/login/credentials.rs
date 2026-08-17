use iced::Element;
use iced::widget::{button, column, container, row, scrollable, text_input};
use uuid::Uuid;

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    pub username: String,
    pub password: String,
    /// The user the picker filled the name from.
    pub picked: Option<Uuid>,
}

/// The users whose images the picker wants now.
pub fn images(state: &super::State) -> std::collections::HashSet<Uuid> {
    let Some(screen) = &state.target else {
        return std::collections::HashSet::new();
    };
    screen
        .users
        .iter()
        .filter(|user| user.has_image && !state.images.contains_key(&user.id))
        .map(|user| user.id)
        .collect()
}

fn picker<'a>(state: &'a super::State) -> Option<Element<'a, Message>> {
    let screen = state.target.as_ref()?;
    if screen.users.is_empty() {
        return None;
    }
    let cards = screen.users.iter().map(|user| {
        let mut card = column![]
            .spacing(style::drawn(space::BLOCK_GAP.drawn()))
            .align_x(iced::Center);
        if let Some(handle) = state.images.get(&user.id) {
            card = card.push(iced::widget::image(handle.clone()).width(96.0));
        }
        card = card.push(prose(user.name.clone(), typeface::BODY));
        button(card)
            .on_press(Message::LoginAction(Action::Pick {
                user: user.id,
                name: user.name.clone(),
            }))
            .into()
    });
    Some(
        column![
            prose(strings::lookup(Text::LoginPickUser), typeface::BODY),
            scrollable(row(cards).spacing(style::drawn(space::GUTTER.drawn()))).direction(
                scrollable::Direction::Horizontal(scrollable::Scrollbar::default(),)
            ),
            prose(strings::lookup(Text::LoginTypeName), typeface::SECONDARY),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .into(),
    )
}

pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let named = state
        .target
        .as_ref()
        .map(|screen| {
            if screen.name.is_empty() {
                screen.server.clone()
            } else {
                screen.name.clone()
            }
        })
        .unwrap_or_default();

    let mut form =
        column![prose(named, typeface::HEADING_1)].spacing(style::drawn(space::GUTTER.drawn()));
    if let Some(picker) = picker(state) {
        form = form.push(picker);
    }

    form = form
        .push(
            text_input(
                strings::lookup(Text::LoginUsername),
                &state.credentials.username,
            )
            .style(style::input)
            .on_input(|value| Message::LoginAction(Action::Edited(Edit::Username(value))))
            .on_submit(Message::LoginAction(Action::Submit))
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
        )
        .push(
            text_input(
                strings::lookup(Text::LoginPassword),
                &state.credentials.password,
            )
            .style(style::input)
            .on_input(|value| Message::LoginAction(Action::Edited(Edit::Password(value))))
            .on_submit(Message::LoginAction(Action::Submit))
            .secure(true)
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
        )
        .push(if state.working {
            button(prose(strings::lookup(Text::LoginWorking), typeface::BODY))
        } else {
            button(prose(strings::lookup(Text::LoginSubmit), typeface::BODY))
                .on_press(Message::LoginAction(Action::Submit))
        });

    if state
        .target
        .as_ref()
        .is_some_and(|screen| screen.quick_connect)
    {
        form = form.push(
            button(prose(
                strings::lookup(Text::LoginQuickConnect),
                typeface::BODY,
            ))
            .on_press(Message::LoginAction(Action::QuickConnect)),
        );
    }
    if !state.read_only {
        form = form.push(
            button(prose(
                strings::lookup(Text::LoginForgotPassword),
                typeface::BODY,
            ))
            .on_press(Message::LoginAction(Action::Reset)),
        );
    }
    form = form.push(
        button(prose(strings::lookup(Text::LoginBack), typeface::BODY))
            .on_press(Message::LoginAction(Action::Back)),
    );

    container(form.max_width(520))
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
