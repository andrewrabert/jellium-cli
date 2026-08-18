use iced::Element;
use iced::widget::column;

use crate::app::Message;
use crate::icon::Icon;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis, Face};

use super::Action;

/// One saved server's card: the reference's own square server card, with what
/// this client knows about the server written under it and the control that
/// forgets it.
// reference: select-server-card
fn saved<'a>(
    saved: &'a jellium_protocol::SavedServer,
    viewport: Viewport,
    read_only: bool,
) -> Element<'a, Message> {
    let named = match saved.name.is_empty() {
        true => saved.server.clone(),
        false => saved.name.clone(),
    };
    let mut entry = column![widget::card(
        card::Card::Rail(card::Rail::Square),
        Room::content(viewport),
        Face::Icon(Icon::Storage),
        named,
        card::Bottom::Flush,
        Message::LoginAction(Action::Select {
            server: saved.server.clone(),
        }),
    )]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    if saved.active {
        entry = entry.push(widget::line(
            strings::lookup(Text::LoginServersActive),
            typeface::SECONDARY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
        ));
    }
    if saved.credentialed {
        entry = entry.push(widget::line(
            strings::lookup(Text::LoginServersSignedIn),
            typeface::SECONDARY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
        ));
    }
    if !(read_only && saved.credentialed) {
        entry = entry.push(widget::block(
            strings::lookup(Text::LoginServersRemove),
            Some(Message::LoginAction(Action::Remove {
                server: saved.server.clone(),
            })),
            Emphasis::Raised,
        ));
    }
    entry.into()
}

/// The reference's select-server page: its title over the saved servers as
/// cards, with the control that adds one beneath them.
// reference: select-server-page
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    widget::page(
        viewport,
        column![
            widget::heading(
                typeface::Rank::First,
                strings::lookup(Text::LoginServersTitle)
            ),
            widget::picker(
                card::Card::Rail(card::Rail::Square),
                Room::content(viewport),
                state
                    .servers
                    .iter()
                    .map(|entry| saved(entry, viewport, state.read_only)),
            ),
            widget::block(
                strings::lookup(Text::LoginServersAdd),
                Some(Message::LoginAction(Action::Add)),
                Emphasis::Raised,
            ),
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into(),
    )
}
