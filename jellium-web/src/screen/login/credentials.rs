use iced::Element;
use iced::widget::column;
use uuid::Uuid;

use crate::app::Message;
use crate::icon::Icon;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
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

/// The card the login picker writes by hand: the user's image or the person
/// glyph over the centred name inside `.cardFooter`'s own padding.
// reference: login-user-card
const PICKED: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Square),
    footer: card::Footer::Name,
    backing: card::Backing::Padder,
    footing: card::Footing::Padded,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

/// The users the server offers, each on the square card the reference draws,
/// with the person glyph where the user carries no image.
// reference: login-user-card
fn picker<'a>(state: &'a super::State, viewport: Viewport) -> Option<Element<'a, Message>> {
    let screen = state.target.as_ref()?;
    if screen.users.is_empty() {
        return None;
    }
    Some(widget::wall(
        PICKED.card,
        Room::content(viewport),
        card::Wrap::Centred,
        screen.users.iter().map(|user| {
            let said = user.name.clone();
            widget::card(
                PICKED,
                Room::content(viewport),
                widget::Poster {
                    face: Some(match state.images.get(&user.id) {
                        Some(handle) => Face::Image(handle.clone()),
                        None => Face::Icon(Icon::Person),
                    }),
                    name: user.name.clone(),
                    logo: None,
                    timer: None,
                    press: Some(Message::LoginAction(Action::Pick {
                        user: user.id,
                        name: user.name.clone(),
                    })),
                    hovered: widget::Hovered::default(),
                },
                move |line| match line {
                    card::Line::Name => said.clone(),
                    _ => String::new(),
                },
            )
        }),
    ))
}

/// The reference's login page: the user picker over the manual form, then the
/// block controls.
// reference: login-page
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    let mut page = column![widget::heading(
        typeface::Rank::First,
        strings::lookup(Text::LoginHeader)
    )]
    .spacing(style::drawn(space::SECTION_GAP.drawn()));
    if let Some(picker) = picker(state, viewport) {
        page = page.push(picker);
    }
    page = page.push(widget::capped(
        viewport,
        space::FIELD_GAP,
        [
            widget::field(
                strings::lookup(Text::LoginUsername),
                &state.credentials.username,
                None,
                |value| Message::LoginAction(Action::Edited(Edit::Username(value))),
                Message::LoginAction(Action::Submit),
                Secrecy::Shown,
            ),
            widget::field(
                strings::lookup(Text::LoginPassword),
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
