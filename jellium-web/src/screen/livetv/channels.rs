use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{Space, button, column, container, image, row, text};
use iced::{Element, Fill};

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
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
    height: f32,
) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            channels: api.channels(kind, None).await.bubbled()?,
            kind,
            window: window::Window::new(window::Id::Channels, theme::ROW_HEIGHT, height),
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
        Some(handle) => image(handle).width(theme::BAR_ART_WIDTH).into(),
        None => Space::new().width(theme::BAR_ART_WIDTH).into(),
    };

    let mut named = column![text(format!("{} {}", channel.number, channel.name)).size(15)]
        .spacing(2)
        .width(Fill);
    if let Some(program) = &channel.current {
        named = named.push(text(program.title.clone()).size(13));
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
                .style(button::text)
                .on_press(Message::LiveTvAction(Action::PlayChannel(channel.id))),
            button(text(strings::lookup(favourite))).on_press(Message::LiveTvAction(
                Action::Favorited(channel.id, !channel.favorite)
            )),
        ]
        .spacing(theme::CARD_SPACING)
        .align_y(iced::Center),
    )
    .height(theme::ROW_HEIGHT)
    .into()
}

/// The TV and radio filter above a windowed list of rows, each carrying the
/// channel's number, name, logo, favourite mark and current program with an
/// elapsed bar, and offering no sort control.
pub fn view<'a>(state: &'a State, now: DateTime<Utc>, images: &'a Cache) -> Element<'a, Message> {
    use jellyfin_api::types::ChannelType;

    let filter = row![
        button(text(strings::lookup(Text::ChannelsTv)))
            .style(if state.kind == ChannelType::Tv {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Tv))),
        button(text(strings::lookup(Text::ChannelsRadio)))
            .style(if state.kind == ChannelType::Radio {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::LiveTvAction(Action::Kind(ChannelType::Radio))),
    ]
    .spacing(theme::CARD_SPACING);

    if state.channels.is_empty() {
        return column![
            filter,
            widget::banner(strings::lookup(Text::ChannelsEmpty).to_string()),
        ]
        .spacing(theme::CARD_SPACING)
        .into();
    }

    let rows = window::list(state.window, state.channels.len(), move |index| {
        let channel = &state.channels[index];
        entry(channel, now, images.handle(key(channel)))
    });

    column![filter, rows]
        .spacing(theme::CARD_SPACING)
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
