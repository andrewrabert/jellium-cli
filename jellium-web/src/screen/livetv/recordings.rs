use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{Space, button, column, container, image, row};
use iced::{Element, Fill};
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::widget::{line, prose};
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

pub async fn load(api: Rc<Api>, height: Drawn) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            recordings: api.recordings().await.bubbled()?,
            window: window::Window::new(
                window::Id::Recordings,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                height,
            ),
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
        Some(handle) => image(handle)
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
        None => Space::new()
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
    };

    let Some(id) = item.id else {
        return Space::new().into();
    };

    let mut named = column![line(
        item.name.clone().unwrap_or_default(),
        typeface::BODY,
        typeface::Weight::Regular,
    )]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .width(Fill);
    if in_progress(item) {
        named = named.push(line(
            strings::lookup(Text::RecordingsInProgress),
            typeface::SECONDARY,
            typeface::Weight::Regular,
        ));
    }

    let mut controls = row![
        button(prose(strings::lookup(Text::ProgramPlay), typeface::BODY))
            .on_press(Message::LiveTvAction(Action::PlayRecording(id))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if let Some(timer) = writing(item) {
        controls = controls.push(
            button(prose(strings::lookup(Text::RecordingsStop), typeface::BODY)).on_press(
                Message::LiveTvAction(Action::StopRecording(timer.to_string())),
            ),
        );
    } else if confirming == Some(id) {
        controls = controls.push(
            button(prose(
                strings::lookup(Text::RecordingsDeleteConfirm),
                typeface::BODY,
            ))
            .on_press(Message::LiveTvAction(Action::ConfirmDelete(id))),
        );
        controls = controls.push(
            button(prose(
                strings::lookup(Text::RecordingsDeleteCancel),
                typeface::BODY,
            ))
            .on_press(Message::LiveTvAction(Action::CloseDelete)),
        );
    } else {
        controls = controls.push(
            button(prose(
                strings::lookup(Text::RecordingsDelete),
                typeface::BODY,
            ))
            .on_press(Message::LiveTvAction(Action::Delete(id))),
        );
    }

    container(
        row![poster, named, controls]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .align_y(iced::Center),
    )
    .height(style::drawn(space::LIST_ROW.drawn()))
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
