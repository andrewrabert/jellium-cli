use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{Space, button, column, container, image, row};
use iced::{Element, Fill};

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::widget::prose;
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Favourites first, then channel-number order.
    pub channels: Vec<Channel>,
    pub kind: jellyfin_api::types::ChannelType,
    pub window: window::Window,
}

pub async fn load(
    api: Rc<Api>,
    kind: jellyfin_api::types::ChannelType,
    height: Drawn,
) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            channels: api.channels(kind, None).await.bubbled()?,
            kind,
            window: window::Window::new(
                window::Id::Channels,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                height,
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
        width: theme::IMAGE_WIDTH,
    }
}

fn entry<'a>(
    channel: &'a Channel,
    now: DateTime<Utc>,
    logo: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let art: Element<'a, Message> = match logo {
        Some(handle) => image(handle)
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
        None => Space::new()
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
    };

    let mut named = column![prose(
        format!("{} {}", channel.number, channel.name),
        typeface::BODY
    )]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .width(Fill);
    if let Some(program) = &channel.current {
        named = named.push(prose(program.title.clone(), typeface::SECONDARY));
        named = named.push(widget::elapsed_bar(program.elapsed(now)));
    }

    let favourite = if channel.favorite {
        Text::ChannelUnfavorite
    } else {
        Text::ChannelFavorite
    };

    container(
        row![
            art,
            button(named)
                .style(style::flat)
                .on_press(Message::LiveTvAction(Action::PlayChannel(channel.id))),
            button(prose(strings::lookup(favourite).to_owned(), typeface::BODY)).on_press(
                Message::LiveTvAction(Action::Favorited(channel.id, !channel.favorite))
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .align_y(iced::Center),
    )
    .height(style::drawn(space::LIST_ROW.drawn()))
    .into()
}

/// The TV and radio filter above a windowed list of rows, each carrying the
/// channel's number, name, logo, favourite mark and current program with an
/// elapsed bar, and offering no sort control.
pub fn view<'a>(state: &'a State, now: DateTime<Utc>, images: &'a Cache) -> Element<'a, Message> {
    use jellyfin_api::types::ChannelType;

    let filter = row![
        button(prose(
            strings::lookup(Text::ChannelsTv).to_owned(),
            typeface::BODY
        ))
        .style(if state.kind == ChannelType::Tv {
            style::submit
        } else {
            style::raised
        })
        .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Tv))),
        button(prose(
            strings::lookup(Text::ChannelsRadio).to_owned(),
            typeface::BODY
        ))
        .style(if state.kind == ChannelType::Radio {
            style::submit
        } else {
            style::raised
        })
        .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Radio))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if state.channels.is_empty() {
        return column![
            filter,
            widget::banner(strings::lookup(Text::ChannelsEmpty).to_string()),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .into();
    }

    let rows = window::list(state.window, state.channels.len(), move |index| {
        let channel = &state.channels[index];
        entry(channel, now, images.handle(key(channel)))
    });

    column![filter, rows]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .width(Fill)
        .height(Fill)
        .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .window
        .shown(state.channels.len())
        .filter_map(|index| state.channels.get(index))
        .map(key)
        .collect()
}
