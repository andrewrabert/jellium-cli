use std::collections::HashSet;

use iced::widget::Space;
use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Fill};
use jellium_protocol::Repeat;

use crate::app::Message;
use crate::images::{self, Cache, Kind as ImageKind};
use crate::player::group::{self, Joined};
use crate::player::{Action, Playing};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::window;

fn key(item: &jellyfin_api::types::BaseItemDto) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: ImageKind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    })
}

/// The images the window shows, rather than the whole queue's.
pub fn images(
    playing: Option<&Playing>,
    group: Option<&Joined>,
    window: window::Window,
) -> HashSet<images::Key> {
    if let Some(joined) = group {
        let queued = joined.queued();
        return window
            .shown(queued.len())
            .filter_map(|index| queued.get(index).copied())
            .filter_map(key)
            .collect();
    }
    let Some(playing) = playing else {
        return HashSet::new();
    };
    let upcoming = playing.queue.upcoming().collect::<Vec<_>>();
    window
        .shown(upcoming.len())
        .filter_map(|index| upcoming.get(index))
        .filter_map(|(_, item)| key(item))
        .chain(playing.queue.current().and_then(key))
        .collect()
}

fn repeat_label(repeat: Repeat) -> Text {
    match repeat {
        Repeat::Off => Text::QueueRepeatOff,
        Repeat::One => Text::QueueRepeatOne,
        Repeat::All => Text::QueueRepeatAll,
    }
}

fn entry<'a>(
    art: Element<'a, Message>,
    name: String,
    play: Option<Message>,
    remove: Message,
) -> Element<'a, Message> {
    let title: Element<'a, Message> = match play {
        Some(play) => button(text(name)).style(button::text).on_press(play).into(),
        None => text(name).into(),
    };
    row![
        art,
        title,
        Space::new().width(Fill),
        button(text(strings::lookup(Text::QueueRemove))).on_press(remove),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center)
    .into()
}

fn art<'a>(item: &jellyfin_api::types::BaseItemDto, cache: &'a Cache) -> Element<'a, Message> {
    match key(item).and_then(|key| cache.handle(key)) {
        Some(handle) => image(handle).width(theme::BAR_ART_WIDTH).into(),
        None => Space::new().width(theme::BAR_ART_WIDTH).into(),
    }
}

/// The upcoming items, each removable, with the shuffle and repeat controls:
/// the group's playlist when this installation is in a group, and the local
/// queue otherwise, both windowed.
pub fn view<'a>(
    playing: Option<&'a Playing>,
    group: Option<&'a Joined>,
    window: window::Window,
    cache: &'a Cache,
) -> Element<'a, Message> {
    /// One queue row's data, so every row is built inside the window rather
    /// than ahead of it.
    struct Row<'a> {
        item: &'a jellyfin_api::types::BaseItemDto,
        play: Option<Message>,
        remove: Message,
    }

    let (repeat, rows): (Repeat, Vec<Row<'a>>) = match group {
        Some(joined) => (
            group::repeat(joined),
            joined
                .queue
                .items
                .iter()
                .filter_map(|queued| {
                    Some(Row {
                        item: joined.item(queued.playlist_item)?,
                        play: Some(Message::GroupAction(group::Action::Play(
                            queued.playlist_item,
                        ))),
                        remove: Message::GroupAction(group::Action::Remove(queued.playlist_item)),
                    })
                })
                .collect(),
        ),
        None => match playing {
            Some(playing) => (
                playing.queue.repeat(),
                playing
                    .queue
                    .upcoming()
                    .map(|(position, item)| Row {
                        item,
                        play: None,
                        remove: Message::PlayerAction(Action::RemoveQueued(position)),
                    })
                    .collect(),
            ),
            None => (Repeat::Off, Vec::new()),
        },
    };

    let controls = row![
        button(text(strings::lookup(Text::QueueBack))).on_press(Message::WentBack),
        button(text(strings::lookup(Text::QueueShuffle)))
            .on_press(Message::PlayerAction(Action::ToggleShuffle)),
        button(text(strings::lookup(repeat_label(repeat))))
            .on_press(Message::PlayerAction(Action::CycleRepeat)),
    ]
    .spacing(theme::CARD_SPACING);

    let body: Element<'a, Message> = if rows.is_empty() {
        text(strings::lookup(Text::QueueEmpty)).into()
    } else {
        window::list(window, rows.len(), move |index| {
            let row = &rows[index];
            container(entry(
                art(row.item, cache),
                row.item.name.clone().unwrap_or_default(),
                row.play.clone(),
                row.remove.clone(),
            ))
            .height(theme::ROW_HEIGHT)
            .into()
        })
    };

    container(
        column![
            text(strings::lookup(Text::QueueTitle)).size(22),
            controls,
            body,
        ]
        .spacing(theme::CARD_SPACING),
    )
    .style(theme::over_video)
    .padding(theme::CARD_SPACING)
    .width(Fill)
    .height(Fill)
    .into()
}
