use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{button, column, row};
use iced::{Element, Fill};
use jellium_model::item::Mark;

use super::{Action, clock};
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::style::space::Room;
use crate::style::{self, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Face, Hovered, Poster, prose};
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Favourites first, then channel-number order.
    pub channels: Vec<Channel>,
    pub kind: jellyfin_api::types::ChannelType,
    pub grid: window::Grid,
}

/// The channels tab's own card.
// reference: livetv-channels-cards
// reference: livetv-tab-markup
pub const CARD: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Square),
    footer: card::Footer::Channel,
    backing: card::Backing::Paper,
    footing: card::Footing::Padded,
    setting: card::Setting::Leading,
    bottom: card::Bottom::Flush,
};

pub async fn load(
    api: Rc<Api>,
    kind: jellyfin_api::types::ChannelType,
    room: Room,
) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            channels: api.live_tv_channels(kind, None).await.bubbled()?,
            kind,
            grid: window::Grid::new(
                window::Id::Channels,
                CARD.card.width(room),
                CARD.row(room),
                room,
            ),
        })
    })
    .await
}

fn key(channel: &Channel) -> images::Key {
    images::Key {
        item: channel.id,
        kind: images::Kind::Primary,
        index: None,
        card: CARD.card,
    }
}

/// One channel's card: its logo over its number and name, the programme on it
/// now, and that programme's own times.
// reference: livetv-channels-cards
fn entry<'a>(
    channel: &'a Channel,
    room: Room,
    logo: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    // reference: card-display-name
    let name = format!("{} {}", channel.number, channel.name);
    let current = channel.current.clone();
    widget::card(
        CARD,
        room,
        Poster {
            face: logo.map(Face::Image),
            name: name.clone(),
            logo: None,
            timer: None,
            elapsed: None,
            press: None,
            hovered: Hovered {
                plays: Some(Message::LiveTvAction(Action::PlayChannel(channel.id))),
                controls: vec![Control {
                    glyph: match channel.favorite {
                        Mark::Set => Icon::Favorite,
                        Mark::Cleared => Icon::FavoriteBorder,
                    },
                    tint: style::Tint::Plain,
                    label: match channel.favorite {
                        Mark::Set => Text::ChannelUnfavorite,
                        Mark::Cleared => Text::ChannelFavorite,
                    },
                    press: Message::LiveTvAction(Action::Favorited(
                        channel.id,
                        channel.favorite.flipped(),
                    )),
                }],
            },
        },
        move |line| match line {
            card::Line::Name => name.clone(),
            card::Line::CurrentProgram => current
                .as_ref()
                .map(|program| program.title.clone())
                .unwrap_or_default(),
            // reference: card-air-time
            card::Line::CurrentProgramTime => current
                .as_ref()
                .map(|program| {
                    strings::format(
                        Text::ProgramAirtime,
                        &[&clock(program.start), &clock(program.end)],
                    )
                })
                .unwrap_or_default(),
            _ => String::new(),
        },
    )
}

/// The TV and radio filter above a windowed wall of cards, each carrying the
/// channel's logo, number, name and current programme.
pub fn view<'a>(state: &'a State, images: &'a Cache, room: Room) -> Element<'a, Message> {
    use jellyfin_api::types::ChannelType;

    let filter = row![
        button(prose(strings::lookup(Text::ChannelsTv), typeface::BODY))
            .style(if state.kind == ChannelType::Tv {
                style::submit
            } else {
                style::raised
            })
            .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Tv))),
        button(prose(strings::lookup(Text::ChannelsRadio), typeface::BODY))
            .style(if state.kind == ChannelType::Radio {
                style::submit
            } else {
                style::raised
            })
            .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Radio))),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

    if state.channels.is_empty() {
        return column![
            filter,
            widget::centered(strings::lookup(Text::ChannelsEmpty).to_string()),
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into();
    }

    let cards = window::grid(
        state.grid,
        card::Wrap::Leading,
        state.channels.len(),
        move |index| {
            let channel = &state.channels[index];
            entry(channel, room, images.handle(key(channel)))
        },
    );

    column![filter, cards]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .width(Fill)
        .height(Fill)
        .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .grid
        .shown(state.channels.len())
        .filter_map(|index| state.channels.get(index))
        .map(key)
        .collect()
}
