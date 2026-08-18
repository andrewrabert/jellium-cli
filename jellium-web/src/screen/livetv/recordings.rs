use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::style::card;
use crate::style::card::Aspect;
use crate::style::space::Room;
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Face, Hovered, Poster};
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// In-progress recordings first, then the rest newest first.
    pub recordings: Vec<BaseItemDto>,
    pub grid: window::Grid,
}

/// True while the Jellyfin server is still writing this recording.
pub fn in_progress(item: &BaseItemDto) -> bool {
    item.status.as_deref() == Some("InProgress") || item.timer_id.is_some()
}

/// The timer writing an in-progress recording.
pub fn writing(item: &BaseItemDto) -> Option<&str> {
    in_progress(item).then_some(item.timer_id.as_deref())?
}

/// The card the recordings tab draws, at the shape `shape: 'auto'` resolves to
/// for these items.
// reference: livetv-recordings-cards
// reference: livetv-recordings-latest
// reference: card-auto-shape
pub fn card(recordings: &[BaseItemDto]) -> card::Drawing {
    let shared = Aspect::shared(
        recordings
            .iter()
            .filter_map(|item| item.primary_image_aspect_ratio)
            .map(Aspect::of),
    );
    card::Drawing {
        card: card::Card::Wall(card::Shape::fitting(shared, card::Shape::Backdrop)),
        footer: card::Footer::Recording,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
    }
}

pub async fn load(api: Rc<Api>, room: Room) -> Answer<State> {
    Answer::of(async {
        let recordings = api.recordings().await.bubbled()?;
        let drawn = card(&recordings);
        Ok(State {
            grid: window::Grid::new(
                window::Id::Recordings,
                drawn.card.width(room),
                drawn.row(room),
                room,
            ),
            recordings,
        })
    })
    .await
}

fn key(item: &BaseItemDto, card: card::Card) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: images::Kind::Primary,
        index: None,
        card,
    })
}

/// The control that stops the timer writing an in-progress recording.
fn stop(timer: &str) -> Control {
    Control {
        glyph: Icon::Stop,
        label: Text::RecordingsStop,
        press: Message::LiveTvAction(Action::StopRecording(timer.to_string())),
    }
}

/// The control that carries out a delete already asked for.
fn confirm(id: Uuid) -> Control {
    Control {
        glyph: Icon::Check,
        label: Text::RecordingsDeleteConfirm,
        press: Message::LiveTvAction(Action::ConfirmDelete(id)),
    }
}

/// The control that abandons a delete already asked for.
fn keep() -> Control {
    Control {
        glyph: Icon::Close,
        label: Text::RecordingsDeleteCancel,
        press: Message::LiveTvAction(Action::CloseDelete),
    }
}

/// The control that asks for a delete.
fn delete(id: Uuid) -> Control {
    Control {
        glyph: Icon::Delete,
        label: Text::RecordingsDelete,
        press: Message::LiveTvAction(Action::Delete(id)),
    }
}

/// One recording's card: its poster over its name, its episode title and the
/// year it was made.
// reference: livetv-recordings-cards
fn entry<'a>(
    item: &'a BaseItemDto,
    drawing: card::Drawing,
    room: Room,
    confirming: Option<Uuid>,
    art: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let name = item.name.clone().unwrap_or_default();
    widget::card(
        drawing,
        room,
        Poster {
            face: art.map(Face::Image),
            name: name.clone(),
            logo: None,
            timer: item
                .timer_id
                .as_ref()
                .map(|_| crate::livetv::Recording::Once),
            elapsed: None,
            press: None,
            hovered: Hovered {
                plays: item
                    .id
                    .map(|id| Message::LiveTvAction(Action::PlayRecording(id))),
                controls: match (writing(item), item.id) {
                    (Some(timer), _) => vec![stop(timer)],
                    (None, Some(id)) if confirming == Some(id) => vec![confirm(id), keep()],
                    (None, Some(id)) => vec![delete(id)],
                    (None, None) => Vec::new(),
                },
            },
        },
        move |line| match line {
            card::Line::ParentTitle => name.clone(),
            // reference: card-display-name
            card::Line::Name => item.episode_title.clone().unwrap_or_default(),
            card::Line::Year => item
                .production_year
                .map(|year| year.to_string())
                .unwrap_or_default(),
            _ => String::new(),
        },
    )
}

/// The reference's own section title over a windowed wall of cards, each
/// carrying the recording's poster, its name and either its episode title or
/// the year it was made.
// reference: livetv-recordings-latest
pub fn view<'a>(
    state: &'a State,
    confirming: Option<Uuid>,
    images: &'a Cache,
    room: Room,
) -> Element<'a, Message> {
    if state.recordings.is_empty() {
        return widget::centered(strings::lookup(Text::RecordingsEmpty).to_string());
    }
    let drawing = card(&state.recordings);
    widget::section(
        strings::lookup(Text::RecordingsLatest),
        window::grid(
            state.grid,
            card::Wrap::Centred,
            state.recordings.len(),
            move |index| {
                let item = &state.recordings[index];
                entry(
                    item,
                    drawing,
                    room,
                    confirming,
                    key(item, drawing.card).and_then(|key| images.handle(key)),
                )
            },
        ),
    )
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let drawing = card(&state.recordings);
    state
        .grid
        .shown(state.recordings.len())
        .filter_map(|index| state.recordings.get(index))
        .filter_map(|item| key(item, drawing.card))
        .collect()
}
