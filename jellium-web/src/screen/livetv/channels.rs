use std::borrow::Cow;
use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{button, column, row};
use iced::{Element, Fill};

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// Favourites first, then channel-number order.
    pub channels: Vec<Channel>,
    pub kind: jellyfin_api::types::ChannelType,
    pub window: window::Window,
}

/// Every row of the channels list: its logo over two lines.
const ROW: space::ListRow = space::ListRow::art(space::Lines::Two);

pub async fn load(
    api: Rc<Api>,
    kind: jellyfin_api::types::ChannelType,
    height: Drawn,
) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            channels: api.channels(kind, None).await.bubbled()?,
            kind,
            window: window::Window::new(window::Id::Channels, ROW.height().drawn(), height),
        })
    })
    .await
}

fn key(channel: &Channel) -> images::Key {
    images::Key {
        item: channel.id,
        kind: images::Kind::Primary,
        index: None,
    }
}

fn entry<'a>(
    channel: &'a Channel,
    now: DateTime<Utc>,
    logo: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let favourite = match channel.favorite {
        true => Text::ChannelUnfavorite,
        false => Text::ChannelFavorite,
    };
    widget::list::row(
        ROW,
        widget::list::Row {
            face: Some(widget::list::Face::Art {
                image: logo,
                elapsed: channel.current.as_ref().map(|program| program.elapsed(now)),
            }),
            index: None,
            title: format!("{} {}", channel.number, channel.name).into(),
            secondary: channel
                .current
                .iter()
                .map(|program| Cow::from(program.title.clone()))
                .collect(),
            press: widget::list::Press::Body(Message::LiveTvAction(Action::PlayChannel(
                channel.id,
            ))),
            controls: vec![
                button(prose(strings::lookup(favourite), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::LiveTvAction(Action::Favorited(
                        channel.id,
                        !channel.favorite,
                    )))
                    .into(),
            ],
        },
    )
}

/// The TV and radio filter above a windowed list of rows, each carrying the
/// channel's number, name, logo, favourite mark and current program with an
/// elapsed bar, and offering no sort control.
pub fn view<'a>(state: &'a State, now: DateTime<Utc>, images: &'a Cache) -> Element<'a, Message> {
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

    let rows = window::list(state.window, state.channels.len(), move |index| {
        let channel = &state.channels[index];
        entry(channel, now, images.handle(key(channel)))
    });

    column![filter, rows]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
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
