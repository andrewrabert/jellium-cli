use iced::Element;
use iced::widget::column;

use crate::app::Message;
use crate::icon::Icon;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis, Face};

use super::Action;

/// The card the select-server page writes by hand: its glyph over its centred
/// name inside `.cardFooter`'s own padding.
// reference: select-server-card
const SAVED: card::Drawing = card::Drawing {
    card: card::Card::Rail(card::Rail::Square),
    footer: card::Footer::Name,
    backing: card::Backing::Padder,
    footing: card::Footing::Padded,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Flush,
    touch: card::Touch::Withheld,
};

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
    let said = named.clone();
    let mut entry = column![widget::card(
        SAVED,
        Room::content(viewport),
        widget::Poster {
            face: Some(Face::Icon(Icon::Storage)),
            name: named,
            logo: None,
            timer: None,
            elapsed: None,
            press: Some(Message::LoginAction(Action::Select {
                server: saved.server.clone(),
            })),
            hovered: widget::Hovered::default(),
            overlaid: widget::Overlaid::default(),
        },
        move |line| match line {
            card::Line::Name => said.clone(),
            _ => String::new(),
        },
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
            widget::wall(
                card::Card::Rail(card::Rail::Square),
                Room::content(viewport),
                card::Wrap::Centred,
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
