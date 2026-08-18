pub mod channels;
pub mod guide;
pub mod recordings;
pub mod schedule;
pub mod series;

use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::column;
use iced::{Element, Fill, Subscription, Task};
use jellium_model::item::Mark;
use jellium_protocol::TimerChanged;
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache};
use crate::style::space::Room;
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
use crate::window;

/// A time of day as `getDisplayTime` writes one.
// reference: card-air-time
pub fn clock(at: DateTime<Utc>) -> String {
    DateTime::<chrono::Local>::from(at)
        .format("%H:%M")
        .to_string()
}

/// Which of the five tabs the Live TV screen shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Guide,
    Channels,
    Recordings,
    Schedule,
    Series,
}

impl Tab {
    pub const ALL: [Tab; 5] = [
        Tab::Guide,
        Tab::Channels,
        Tab::Recordings,
        Tab::Schedule,
        Tab::Series,
    ];

    pub fn label(self) -> Text {
        match self {
            Tab::Guide => Text::LiveTvTabGuide,
            Tab::Channels => Text::LiveTvTabChannels,
            Tab::Recordings => Text::LiveTvTabRecordings,
            Tab::Schedule => Text::LiveTvTabSchedule,
            Tab::Series => Text::LiveTvTabSeries,
        }
    }
}

/// What the shown tab holds.
#[derive(Debug, Clone)]
pub enum Body {
    Guide(guide::State),
    Channels(channels::State),
    Recordings(recordings::State),
    Schedule(schedule::State),
    Series(series::State),
}

/// The Live TV screen: the tab shown, what it holds, and what is drawn over
/// it.
#[derive(Debug, Clone)]
pub struct State {
    pub tab: Tab,
    pub body: Body,
    /// The series options being edited, drawn over the tab.
    pub editing: Option<series::Editing>,
    /// The recording options being edited, drawn over the tab.
    pub timing: Option<schedule::Editing>,
}

/// Every control the Live TV screen and the on-now row resolve to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Selected(Tab),
    /// Plays a channel here, or sends it to a bound target.
    PlayChannel(Uuid),
    Favorited(Uuid, Mark),
    /// Filters the Channels tab.
    Kind(jellyfin_api::types::ChannelType),
    /// Opens a program's detail.
    Show(Uuid),
    /// Creates a timer for a program from the server's defaults.
    Record(Uuid),
    /// Opens the series options for a program, prefilled from the server's
    /// defaults.
    RecordSeries(Uuid),
    /// Opens the series options of a series timer already created.
    EditSeries(String),
    Edited(series::Field),
    /// Creates or updates the series timer the options describe.
    ConfirmSeries,
    CloseSeries,
    /// Opens the recording options of a scheduled timer.
    EditTimer(String),
    /// Writes one field of those options.
    Timed(schedule::Field),
    /// Writes the timer the options describe back to the server.
    ConfirmTimer,
    CloseTimer,
    CancelTimer(String),
    CancelSeriesTimer(String),
    PlayRecording(Uuid),
    /// Moves the guide one screen in time.
    Step(guide::Step),
    /// Moves the guide to a date.
    Date(chrono::NaiveDate),
    /// Moves the guide's focus.
    Focus(guide::Move),
    /// Opens the guide's focused cell.
    Open,
}

/// Loads the tab `tab` names, against the room `room` lays its cards in.
pub async fn load(api: Rc<Api>, tab: Tab, room: Room) -> Answer<State> {
    Answer::of(async {
        let body = match tab {
            Tab::Guide => Body::Guide(guide::load(api, room.viewport().canvas().height()).await?),
            Tab::Channels => Body::Channels(
                channels::load(api, jellyfin_api::types::ChannelType::Tv, room)
                    .await
                    .bubbled()?,
            ),
            Tab::Recordings => Body::Recordings(recordings::load(api, room).await.bubbled()?),
            Tab::Schedule => Body::Schedule(schedule::load(api).await.bubbled()?),
            Tab::Series => Body::Series(series::load(api, room).await.bubbled()?),
        };
        Ok(State {
            tab,
            body,
            editing: None,
            timing: None,
        })
    })
    .await
}

/// The five tab controls above the tab shown.
pub fn view<'a>(
    state: &'a State,
    now: DateTime<Utc>,
    images: &'a Cache,
    viewport: Viewport,
    session: &'a jellium_protocol::Session,
) -> Element<'a, Message> {
    if let Some(editing) = &state.editing {
        return series::options(editing, viewport);
    }
    if let Some(timing) = &state.timing {
        return schedule::options(timing, viewport);
    }

    let tabs = widget::tabs(
        viewport,
        Tab::ALL.into_iter().map(|tab| widget::Entry {
            label: tab.label(),
            showing: match tab == state.tab {
                true => widget::Showing::Shown,
                false => widget::Showing::Offered(Message::LiveTvAction(Action::Selected(tab))),
            },
        }),
    );

    let room = Room::content(viewport);
    let body = match &state.body {
        Body::Guide(guide) => guide::view(guide, now, images, viewport),
        Body::Channels(channels) => channels::view(channels, images, room, session, now),
        Body::Recordings(held) => recordings::view(held, images, room, session, now),
        Body::Schedule(schedule) => schedule::view(schedule, images, room, session, now),
        Body::Series(series) => series::view(series, room, session, now),
    };

    column![
        prose(strings::lookup(Text::LiveTvTitle), typeface::HEADING_2),
        tabs,
        body,
    ]
    .spacing(style::drawn(space::SECTION_GAP.drawn()))
    .width(Fill)
    .height(Fill)
    .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    match &state.body {
        Body::Guide(held) => guide::images(held),
        Body::Channels(held) => channels::images(held),
        Body::Recordings(held) => recordings::images(held),
        Body::Schedule(held) => schedule::images(held),
        Body::Series(_) => HashSet::new(),
    }
}

/// The guide's keys while the Guide tab is open.
pub fn keys(state: &State) -> Subscription<Action> {
    match &state.body {
        Body::Guide(_) => guide::keys(),
        _ => Subscription::none(),
    }
}

/// The Live TV screen this tab is showing, and nothing while another screen is.
fn showing(signed: &mut Signed) -> Option<&mut State> {
    match &mut signed.view {
        crate::app::View::LiveTv(state) => Some(state.as_mut()),
        _ => None,
    }
}

/// Reloads the tab shown, which is how a write's effect is taken up without
/// moving the scroll position or the order.
fn reload(signed: &Signed, tab: Tab, viewport: Viewport) -> Task<Message> {
    Task::perform(
        load(signed.api.clone(), tab, Room::content(viewport)),
        Message::LiveTvLoaded,
    )
}

/// Applies a control.
/// A `PlayChannel` while this tab holds group membership plays nothing and
/// states why.
pub fn act(signed: &mut Signed, action: Action, viewport: Viewport) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Selected(tab) => {
            Task::done(Message::Navigated(crate::route::Route::LiveTv { tab }))
        }
        Action::PlayChannel(channel) => {
            if signed.group.is_some() {
                crate::failure::raise(crate::error::told(Text::FailureChannelInGroup));
                return Task::none();
            }
            crate::player::live::play(signed, channel)
        }
        Action::Favorited(channel, wanted) => {
            let kind = match showing(signed).map(|state| &state.body) {
                Some(Body::Channels(held)) => held.kind,
                _ => jellyfin_api::types::ChannelType::Tv,
            };
            let room = Room::content(viewport);
            Task::perform(
                Answer::of(async move {
                    api.set_favorite(channel, wanted).await.bubbled()?;
                    channels::load(api, kind, room).await.bubbled()
                }),
                |loaded| {
                    Message::LiveTvLoaded(loaded.map(|channels| State {
                        tab: Tab::Channels,
                        body: Body::Channels(channels),
                        editing: None,
                        timing: None,
                    }))
                },
            )
        }
        Action::Kind(kind) => {
            let room = Room::content(viewport);
            Task::perform(
                async move { channels::load(api, kind, room).await },
                |loaded| {
                    Message::LiveTvLoaded(loaded.map(|channels| State {
                        tab: Tab::Channels,
                        body: Body::Channels(channels),
                        editing: None,
                        timing: None,
                    }))
                },
            )
        }
        Action::Show(program) => Task::done(Message::Navigated(crate::route::Route::Detail {
            id: program,
        })),
        Action::Record(program) => Task::perform(
            async move { api.record(&program.to_string()).await },
            |outcome| Message::Wrote(Operation::Timer, outcome),
        ),
        Action::RecordSeries(program) => Task::perform(
            async move {
                api.timer_defaults(&program.to_string())
                    .await
                    .map(|options| (options, true))
            },
            Message::SeriesPrefilled,
        ),
        Action::EditSeries(timer) => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Series(held) = &state.body else {
                return Task::none();
            };
            let Some(options) = held
                .timers
                .iter()
                .find(|held| held.id.as_deref() == Some(timer.as_str()))
                .cloned()
            else {
                return Task::none();
            };
            state.editing = Some(series::Editing {
                options,
                creating: false,
            });
            Task::none()
        }
        Action::Edited(field) => {
            if let Some(state) = showing(signed)
                && let Some(editing) = &mut state.editing
            {
                editing.edited(field);
            }
            Task::none()
        }
        Action::ConfirmSeries => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Some(editing) = state.editing.take() else {
                return Task::none();
            };
            let options = editing.options;
            let creating = editing.creating;
            Task::perform(
                async move {
                    if creating {
                        api.record_series(&options).await
                    } else {
                        api.update_series_timer(&options).await
                    }
                },
                |outcome| Message::Wrote(Operation::SeriesTimer, outcome),
            )
        }
        Action::CloseSeries => {
            if let Some(state) = showing(signed) {
                state.editing = None;
            }
            Task::none()
        }
        Action::EditTimer(timer) => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Schedule(held) = &state.body else {
                return Task::none();
            };
            let Some(held) = held
                .timers
                .iter()
                .find(|held| held.id.as_deref() == Some(timer.as_str()))
                .cloned()
            else {
                return Task::none();
            };
            state.timing = Some(schedule::Editing { timer: held });
            Task::none()
        }
        Action::Timed(field) => {
            if let Some(state) = showing(signed)
                && let Some(timing) = &mut state.timing
            {
                timing.edited(field);
            }
            Task::none()
        }
        Action::ConfirmTimer => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Some(timing) = state.timing.take() else {
                return Task::none();
            };
            Task::perform(
                async move { api.update_timer(&timing.timer).await },
                |outcome| Message::Wrote(Operation::Timer, outcome),
            )
        }
        Action::CloseTimer => {
            if let Some(state) = showing(signed) {
                state.timing = None;
            }
            Task::none()
        }
        Action::CancelTimer(timer) => {
            Task::perform(async move { api.cancel_timer(&timer).await }, |outcome| {
                Message::Wrote(Operation::Timer, outcome)
            })
        }
        Action::CancelSeriesTimer(timer) => Task::perform(
            async move { api.cancel_series_timer(&timer).await },
            |outcome| Message::Wrote(Operation::SeriesTimer, outcome),
        ),
        Action::PlayRecording(recording) => {
            let api = signed.api.clone();
            Task::perform(
                crate::player::resolve(
                    api,
                    crate::player::Intent::Item {
                        item: recording,
                        resume: true,
                    },
                ),
                Message::Resolved,
            )
        }
        Action::Step(step) => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Guide(guide) = &mut state.body else {
                return Task::none();
            };
            guide.stepped(step);
            fetch_if_stale(signed)
        }
        Action::Date(date) => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Guide(guide) = &mut state.body else {
                return Task::none();
            };
            guide.dated(date);
            fetch_if_stale(signed)
        }
        Action::Focus(moved) => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Guide(guide) = &mut state.body else {
                return Task::none();
            };
            guide.moved(moved);
            let showing =
                window::showing(guide.window, guide.window.shown(guide.channels.len()).start);
            Task::batch([showing, fetch_if_stale(signed)])
        }
        Action::Open => {
            let Some(state) = showing(signed) else {
                return Task::none();
            };
            let Body::Guide(guide) = &state.body else {
                return Task::none();
            };
            let now = Utc::now();
            match guide.focused() {
                Some(program) if program.airing(now) => {
                    let channel = program.channel;
                    act(signed, Action::PlayChannel(channel), viewport)
                }
                Some(program) => {
                    let item = program.item;
                    act(signed, Action::Show(item), viewport)
                }
                None => Task::none(),
            }
        }
    }
}

/// One program query when the guide's wanted band has left what is held, and
/// nothing while it sits inside it.
pub fn fetch_if_stale(signed: &mut Signed) -> Task<Message> {
    let api = signed.api.clone();
    let Some(state) = showing(signed) else {
        return Task::none();
    };
    let Body::Guide(guide) = &state.body else {
        return Task::none();
    };
    if !guide.stale() {
        return Task::none();
    }
    let wanted = guide.wanted();
    let channels = guide.channels[wanted.channels.clone()]
        .iter()
        .map(|channel| channel.id)
        .collect::<Vec<_>>();
    Task::perform(guide::fetch(api, wanted, channels), Message::GuideFetched)
}

/// Applies the timer changes an event carried: the guide's record markers are
/// patched in place without refetching program data, and the Schedule and
/// Series tabs re-read their own list, so neither the scroll position nor the
/// order moves.
pub fn timed(signed: &mut Signed, changes: &[TimerChanged], viewport: Viewport) -> Task<Message> {
    let tab = match showing(signed) {
        Some(state) => match &mut state.body {
            Body::Guide(guide) => {
                for changed in changes {
                    guide.timed(changed);
                }
                return Task::none();
            }
            Body::Schedule(_) => Tab::Schedule,
            Body::Series(_) => Tab::Series,
            Body::Recordings(_) => Tab::Recordings,
            Body::Channels(_) => return Task::none(),
        },
        None => return Task::none(),
    };
    reload(signed, tab, viewport)
}
