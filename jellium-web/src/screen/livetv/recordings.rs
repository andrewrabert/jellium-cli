use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use jellium_model::item;
use jellium_protocol::Session;
use jellyfin_api::types::BaseItemDto;

use super::Action;
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::screen::overflow;
use crate::style::card::Aspect;
use crate::style::space::Room;
use crate::style::{self, card};
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Face, Hovered, Overlaid, Poster};
use crate::window;

#[derive(Debug, Clone)]
pub struct State {
    /// In-progress recordings first, then the rest newest first.
    pub recordings: Vec<BaseItemDto>,
    // the aspect the recordings share settles this, and the window is measured
    // against it
    pub drawing: card::Drawing,
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
fn card(recordings: &[BaseItemDto]) -> card::Drawing {
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
        touch: card::Touch::Unset,
    }
}

pub async fn load(api: Rc<Api>, room: Room) -> Answer<State> {
    Answer::of(async {
        let recordings = api.recordings().await.bubbled()?;
        let drawing = card(&recordings);
        Ok(State {
            grid: window::Grid::new(
                window::Id::Recordings,
                drawing.card.width(room),
                drawing.row(room),
                room,
            ),
            drawing,
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

/// One recording's card: its poster over its name, its episode title and the
/// year it was made.
// reference: livetv-recordings-cards
fn entry<'a>(
    item: &'a BaseItemDto,
    drawing: card::Drawing,
    room: Room,
    session: &Session,
    now: chrono::DateTime<chrono::Utc>,
    art: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let name = item.name.clone().unwrap_or_default();
    let offered = overflow::commands(overflow::Subject::Item(item), session, None, now);
    let menu = (!offered.is_empty())
        .then_some(Message::OverflowAction(overflow::Action::Open { offered }));
    widget::card(
        drawing,
        room,
        Poster {
            face: art.map(Face::Image),
            name: name.clone(),
            logo: None,
            timer: crate::livetv::Recording::covering(item),
            elapsed: None,
            press: item
                .id
                .map(|id| Message::Navigated(crate::route::Route::Detail { id })),
            hovered: Hovered {
                plays: item
                    .id
                    .map(|id| Message::LiveTvAction(Action::PlayRecording(id))),
                controls: menu
                    .clone()
                    .map(|press| Control {
                        glyph: Icon::MoreVert,
                        tint: style::Tint::Plain,
                        label: Text::OverflowOpen,
                        press,
                    })
                    .into_iter()
                    .collect(),
            },
            overlaid: Overlaid {
                plays: item
                    .id
                    .filter(|_| item::overlay_playable(item))
                    .map(|id| Message::LiveTvAction(Action::PlayRecording(id))),
                menu,
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
    images: &'a Cache,
    room: Room,
    session: &'a Session,
    now: chrono::DateTime<chrono::Utc>,
) -> Element<'a, Message> {
    if state.recordings.is_empty() {
        return widget::centered(strings::lookup(Text::RecordingsEmpty).to_string());
    }
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
                    state.drawing,
                    room,
                    session,
                    now,
                    key(item, state.drawing.card).and_then(|key| images.handle(key)),
                )
            },
        ),
    )
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .grid
        .shown(state.recordings.len())
        .filter_map(|index| state.recordings.get(index))
        .filter_map(|item| key(item, state.drawing.card))
        .collect()
}
