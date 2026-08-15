use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{Space, button, column, container, image, row, text};
use iced::{Element, Fill};
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// In-progress recordings first, then the rest newest first.
    pub recordings: Vec<BaseItemDto>,
    pub window: window::Window,
}

/// True while the Jellyfin server is still writing this recording.
pub fn in_progress(item: &BaseItemDto) -> bool {
    item.status.as_deref() == Some("InProgress") || item.timer_id.is_some()
}

/// The timer writing an in-progress recording.
pub fn writing(item: &BaseItemDto) -> Option<&str> {
    in_progress(item).then_some(item.timer_id.as_deref())?
}

pub async fn load(api: Rc<Api>, height: f32) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            recordings: api.recordings().await.bubbled()?,
            window: window::Window::new(window::Id::Recordings, theme::ROW_HEIGHT, height),
        })
    })
    .await
}

fn key(item: &BaseItemDto) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: images::Kind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    })
}

fn entry<'a>(
    item: &'a BaseItemDto,
    confirming: Option<Uuid>,
    art: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let poster: Element<'a, Message> = match art {
        Some(handle) => image(handle).width(theme::BAR_ART_WIDTH).into(),
        None => Space::new().width(theme::BAR_ART_WIDTH).into(),
    };

    let Some(id) = item.id else {
        return Space::new().into();
    };

    let mut named = column![text(item.name.clone().unwrap_or_default()).size(15)]
        .spacing(2)
        .width(Fill);
    if in_progress(item) {
        named = named.push(text(strings::lookup(Text::RecordingsInProgress)).size(13));
    }

    let mut controls = row![
        button(text(strings::lookup(Text::ProgramPlay)))
            .on_press(Message::LiveTvAction(Action::PlayRecording(id))),
    ]
    .spacing(theme::CARD_SPACING);

    if let Some(timer) = writing(item) {
        controls = controls.push(
            button(text(strings::lookup(Text::RecordingsStop))).on_press(Message::LiveTvAction(
                Action::StopRecording(timer.to_string()),
            )),
        );
    } else if confirming == Some(id) {
        controls = controls.push(
            button(text(strings::lookup(Text::RecordingsDeleteConfirm)))
                .on_press(Message::LiveTvAction(Action::ConfirmDelete(id))),
        );
        controls = controls.push(
            button(text(strings::lookup(Text::RecordingsDeleteCancel)))
                .on_press(Message::LiveTvAction(Action::CloseDelete)),
        );
    } else {
        controls = controls.push(
            button(text(strings::lookup(Text::RecordingsDelete)))
                .on_press(Message::LiveTvAction(Action::Delete(id))),
        );
    }

    container(
        row![poster, named, controls]
            .spacing(theme::CARD_SPACING)
            .align_y(iced::Center),
    )
    .height(theme::ROW_HEIGHT)
    .into()
}

/// A windowed list: an in-progress recording marked as in progress and
/// offering Stop Recording and no delete, a completed one offering Delete
/// behind a confirmation and no stop, and every row offering Play.
pub fn view<'a>(
    state: &'a State,
    confirming: Option<Uuid>,
    images: &'a Cache,
) -> Element<'a, Message> {
    if state.recordings.is_empty() {
        return widget::banner(strings::lookup(Text::RecordingsEmpty).to_string());
    }

    window::list(state.window, state.recordings.len(), move |index| {
        let item = &state.recordings[index];
        entry(
            item,
            confirming,
            key(item).and_then(|key| images.handle(key)),
        )
    })
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .window
        .shown(state.recordings.len())
        .filter_map(|index| state.recordings.get(index))
        .filter_map(key)
        .collect()
}
