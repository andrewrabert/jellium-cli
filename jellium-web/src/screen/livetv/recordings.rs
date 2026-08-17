use std::borrow::Cow;
use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::button;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
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

/// Every row of the recordings list: its poster over two lines.
const ROW: space::ListRow = space::ListRow::art(space::Lines::Two);

pub async fn load(api: Rc<Api>, height: Drawn) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            recordings: api.recordings().await.bubbled()?,
            window: window::Window::new(window::Id::Recordings, ROW.height().drawn(), height),
        })
    })
    .await
}

fn key(item: &BaseItemDto) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: images::Kind::Primary,
        index: None,
    })
}

/// The control that stops the timer writing an in-progress recording.
fn stop<'a>(timer: &str) -> Element<'a, Message> {
    button(prose(strings::lookup(Text::RecordingsStop), typeface::BODY))
        .style(style::flat)
        .on_press(Message::LiveTvAction(Action::StopRecording(
            timer.to_string(),
        )))
        .into()
}

/// The control that carries out a delete already asked for.
fn confirm<'a>(id: Uuid) -> Element<'a, Message> {
    button(prose(
        strings::lookup(Text::RecordingsDeleteConfirm),
        typeface::BODY,
    ))
    .style(style::flat)
    .on_press(Message::LiveTvAction(Action::ConfirmDelete(id)))
    .into()
}

/// The control that abandons a delete already asked for.
fn cancel<'a>() -> Element<'a, Message> {
    button(prose(
        strings::lookup(Text::RecordingsDeleteCancel),
        typeface::BODY,
    ))
    .style(style::flat)
    .on_press(Message::LiveTvAction(Action::CloseDelete))
    .into()
}

/// The control that asks for a delete.
fn delete<'a>(id: Uuid) -> Element<'a, Message> {
    button(prose(
        strings::lookup(Text::RecordingsDelete),
        typeface::BODY,
    ))
    .style(style::flat)
    .on_press(Message::LiveTvAction(Action::Delete(id)))
    .into()
}

// reference: list-markup
fn entry<'a>(
    item: &'a BaseItemDto,
    confirming: Option<Uuid>,
    art: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let mut controls: Vec<Element<'a, Message>> = Vec::new();
    if let Some(id) = item.id {
        controls.push(
            button(prose(strings::lookup(Text::ProgramPlay), typeface::BODY))
                .style(style::flat)
                .on_press(Message::LiveTvAction(Action::PlayRecording(id)))
                .into(),
        );
    }
    match (writing(item), item.id) {
        (Some(timer), _) => controls.push(stop(timer)),
        (None, Some(id)) if confirming == Some(id) => {
            controls.push(confirm(id));
            controls.push(cancel());
        }
        (None, Some(id)) => controls.push(delete(id)),
        (None, None) => {}
    }

    widget::list::row(
        ROW,
        widget::list::Row {
            face: Some(widget::list::Face::Art {
                image: art,
                elapsed: None,
            }),
            index: None,
            title: item.name.clone().unwrap_or_default().into(),
            secondary: in_progress(item)
                .then(|| Cow::from(strings::lookup(Text::RecordingsInProgress)))
                .into_iter()
                .collect(),
            press: widget::list::Press::Inert,
            controls,
        },
    )
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
        return widget::centered(strings::lookup(Text::RecordingsEmpty).to_string());
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
