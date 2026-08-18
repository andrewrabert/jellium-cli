use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{column, container};
use iced::{Element, Fill};
use jellyfin_api::types::{BaseItemDto, TimerInfoDto};

use super::{Action, clock};
use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::style::card::Aspect;
use crate::style::space::Room;
use crate::style::{self, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Control, Face, Hovered, Poster, prose};

#[derive(Debug, Clone)]
pub struct State {
    /// The recordings the server is writing now.
    pub active: Vec<BaseItemDto>,
    /// Upcoming timers ordered by start time.
    pub timers: Vec<TimerInfoDto>,
}

/// The card the active-recordings section draws.
// reference: livetv-schedule-recordings
// reference: livetv-schedule-active
// reference: card-auto-shape
pub fn active_card(recordings: &[BaseItemDto]) -> card::Drawing {
    let shared = Aspect::shared(
        recordings
            .iter()
            .filter_map(|item| item.primary_image_aspect_ratio)
            .map(Aspect::of),
    );
    card::Drawing {
        card: card::Card::Wall(card::Shape::fitting(shared, card::Shape::Backdrop)),
        footer: card::Footer::ActiveRecording,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
    }
}

/// The card a day group draws.
// reference: livetv-timer-cards
pub const TIMER_CARD: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Backdrop),
    footer: card::Footer::Timer,
    backing: card::Backing::Paper,
    footing: card::Footing::Padded,
    setting: card::Setting::Leading,
    bottom: card::Bottom::Flush,
};

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            active: api.active_recordings().await.bubbled()?,
            timers: api.timers().await.bubbled()?,
        })
    })
    .await
}

/// One run of timers sharing a day: the heading the reference writes over the
/// run, and none where its timers carry no start.
struct Day<'a> {
    named: Option<String>,
    timers: &'a [TimerInfoDto],
}

/// The day a timer falls under, written as the reference writes it: the weekday
/// in full, the month short, the day of the month.
// reference: schedule-groups
fn named(at: chrono::DateTime<chrono::Utc>) -> String {
    chrono::DateTime::<chrono::Local>::from(at)
        .format("%A, %b %-d")
        .to_string()
}

/// The runs of consecutive timers sharing a day, in the order they arrive.
// reference: schedule-groups
fn days(timers: &[TimerInfoDto]) -> Vec<Day<'_>> {
    let mut days: Vec<Day<'_>> = Vec::new();
    let mut start = 0;
    for index in 0..timers.len() {
        let ends = timers
            .get(index + 1)
            .map(|next| next.start_date.map(named) != timers[index].start_date.map(named))
            .unwrap_or(true);
        if ends {
            days.push(Day {
                named: timers[start].start_date.map(named),
                timers: &timers[start..=index],
            });
            start = index + 1;
        }
    }
    days
}

/// The times a card writes for an airing that names both ends, and nothing
/// where it names neither.
// reference: card-air-time
fn airtime(
    start: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
) -> String {
    let Some(start) = start else {
        return String::new();
    };
    match end {
        Some(end) => strings::format(Text::ProgramAirtime, &[&clock(start), &clock(end)]),
        None => clock(start),
    }
}

/// The image the Jellyfin server holds for `item`.
fn key(id: uuid::Uuid) -> images::Key {
    images::Key {
        item: id,
        kind: images::Kind::Primary,
        index: None,
    }
}

/// One active recording's card: its own image over the programme's name, its
/// episode title, the time it runs and the channel it came from.
// reference: livetv-schedule-active
fn active<'a>(
    item: &'a BaseItemDto,
    drawing: card::Drawing,
    room: Room,
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
            timer: Some(crate::livetv::Recording::Once),
            press: None,
            hovered: Hovered::default(),
        },
        move |line| match line {
            card::Line::ParentTitle => name.clone(),
            // reference: card-display-name
            card::Line::Name => item.episode_title.clone().unwrap_or_default(),
            card::Line::AirTime => airtime(item.start_date, item.end_date),
            card::Line::ChannelName => item.channel_name.clone().unwrap_or_default(),
            _ => String::new(),
        },
    )
}

/// One scheduled timer's card: the programme's image over its name, its
/// episode title and the time it runs, with the channel's logo at the leading
/// edge of the footer.
// reference: livetv-timer-cards
fn timed<'a>(
    timer: &'a TimerInfoDto,
    room: Room,
    art: Option<iced::widget::image::Handle>,
    logo: Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    let program = timer.program_info.as_ref();
    let name = program
        .and_then(|held| held.name.clone())
        .or(timer.name.clone())
        .unwrap_or_default();
    let episode = program.and_then(|held| held.episode_title.clone());
    let times = airtime(timer.start_date, timer.end_date);
    widget::card(
        TIMER_CARD,
        room,
        Poster {
            face: art.map(Face::Image),
            name: name.clone(),
            logo,
            timer: Some(match timer.series_timer_id {
                Some(_) => crate::livetv::Recording::Series,
                None => crate::livetv::Recording::Once,
            }),
            press: None,
            hovered: Hovered {
                plays: None,
                controls: timer
                    .id
                    .clone()
                    .map(|id| Control {
                        glyph: Icon::Delete,
                        label: Text::ScheduleCancel,
                        press: Message::LiveTvAction(Action::CancelTimer(id)),
                    })
                    .into_iter()
                    .collect(),
            },
        },
        move |line| match line {
            card::Line::ParentTitle => name.clone(),
            // reference: card-display-name
            card::Line::Name => episode.clone().unwrap_or_default(),
            card::Line::AirTime => times.clone(),
            _ => String::new(),
        },
    )
}

/// One group's heading over its wall of cards, and its cards alone where the
/// group has no heading.
// reference: schedule-groups
// reference: section-title-cards
fn grouped<'a>(day: Day<'a>, room: Room, images: &'a Cache) -> Element<'a, Message> {
    let cards = widget::wall(
        TIMER_CARD.card,
        room,
        card::Wrap::Leading,
        day.timers
            .iter()
            .map(|timer| timed(timer, room, program_art(timer, images), logo(timer, images))),
    );
    match day.named {
        None => cards,
        Some(named) => column![
            container(prose(named, typeface::HEADING_2))
                .padding(style::padding(space::GROUP_TITLE_PAD)),
            cards,
        ]
        .into(),
    }
}

/// The programme image a timer's card draws.
fn program_art(timer: &TimerInfoDto, images: &Cache) -> Option<iced::widget::image::Handle> {
    program_key(timer).and_then(|key| images.handle(key))
}

/// The channel logo a timer's card carries in its footer.
// reference: card-footer-logo
fn logo(timer: &TimerInfoDto, images: &Cache) -> Option<iced::widget::image::Handle> {
    timer
        .channel_id
        .and_then(|channel| images.handle(key(channel)))
}

fn program_key(timer: &TimerInfoDto) -> Option<images::Key> {
    Some(key(timer.program_info.as_ref()?.id?))
}

/// The active recordings under their own title, then the timers grouped by the
/// day they start on, all in one scroll.
// reference: livetv-tab-markup
pub fn view<'a>(state: &'a State, images: &'a Cache, room: Room) -> Element<'a, Message> {
    if state.active.is_empty() && state.timers.is_empty() {
        return widget::centered(strings::lookup(Text::ScheduleEmpty).to_string());
    }
    let mut page = column![];
    if !state.active.is_empty() {
        let drawing = active_card(&state.active);
        page = page.push(widget::section(
            strings::lookup(Text::ScheduleActive),
            widget::wall(
                drawing.card,
                room,
                card::Wrap::Centred,
                state.active.iter().map(|item| {
                    active(
                        item,
                        drawing,
                        room,
                        item.id.and_then(|id| images.handle(key(id))),
                    )
                }),
            ),
        ));
    }
    for day in days(&state.timers) {
        page = page.push(grouped(day, room, images));
    }
    widget::scrolled(page).height(Fill).into()
}

/// Every active recording's own image, and every shown timer's programme image
/// and channel logo.
pub fn images(state: &State) -> HashSet<images::Key> {
    let active = state.active.iter().filter_map(|item| item.id).map(key);
    let programs = state.timers.iter().filter_map(program_key);
    let logos = state
        .timers
        .iter()
        .filter_map(|timer| timer.channel_id)
        .map(key);
    active.chain(programs).chain(logos).collect()
}
