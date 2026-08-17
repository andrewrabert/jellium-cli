use iced::Element;
use iced::widget::column;
use uuid::Uuid;

use crate::app::Message;
use crate::icon::Icon;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis, Face, Secrecy};

use super::{Action, Edit};

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

/// The users the server offers, each on the square card the reference draws,
/// with the person glyph where the user carries no image.
// reference: login-user-card
fn picker<'a>(state: &'a super::State, viewport: Viewport) -> Option<Element<'a, Message>> {
    let screen = state.target.as_ref()?;
    if screen.users.is_empty() {
        return None;
    }
    let wall = card::Card::Wall(card::Shape::Square);
    Some(widget::picker(
        wall,
        Room::content(viewport),
        screen
            .users
            .iter()
            .map(|user| {
                widget::card(
                    wall,
                    Room::content(viewport),
                    match state.images.get(&user.id) {
                        Some(handle) => Face::Image(handle.clone()),
                        None => Face::Icon(Icon::Person),
                    },
                    user.name.clone(),
                    card::Bottom::Padded,
                    Message::LoginAction(Action::Pick {
                        user: user.id,
                        name: user.name.clone(),
                    }),
                )
            })
            .collect(),
    ))
}

/// The reference's login page: the user picker over the manual form, then the
/// block controls.
// reference: login-page
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    let mut page = column![widget::heading(strings::lookup(Text::LoginHeader))]
        .spacing(style::drawn(space::SECTION_GAP.drawn()));
    if let Some(picker) = picker(state, viewport) {
        page = page.push(picker);
    }
    page = page.push(widget::form(
        viewport,
        vec![
            widget::field(
                Text::LoginUsername,
                &state.credentials.username,
                None,
                |value| Message::LoginAction(Action::Edited(Edit::Username(value))),
                Message::LoginAction(Action::Submit),
                Secrecy::Shown,
            ),
            widget::field(
                Text::LoginPassword,
                &state.credentials.password,
                None,
                |value| Message::LoginAction(Action::Edited(Edit::Password(value))),
                Message::LoginAction(Action::Submit),
                Secrecy::Hidden,
            ),
            widget::block(
                strings::lookup(match state.working {
                    true => Text::LoginWorking,
                    false => Text::LoginSubmit,
                }),
                (!state.working).then_some(Message::LoginAction(Action::Submit)),
                Emphasis::Submit,
            ),
        ],
    ));

    if state
        .target
        .as_ref()
        .is_some_and(|screen| screen.quick_connect)
    {
        page = page.push(widget::block(
            strings::lookup(Text::LoginQuickConnect),
            Some(Message::LoginAction(Action::QuickConnect)),
            Emphasis::Raised,
        ));
    }
    if !state.read_only {
        page = page.push(widget::block(
            strings::lookup(Text::LoginForgotPassword),
            Some(Message::LoginAction(Action::Reset)),
            Emphasis::Raised,
        ));
    }
    page = page.push(widget::block(
        strings::lookup(Text::LoginBack),
        Some(Message::LoginAction(Action::Back)),
        Emphasis::Raised,
    ));

    widget::page(viewport, page.into())
}
