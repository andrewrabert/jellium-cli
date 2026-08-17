pub mod channels;
pub mod guide;
pub mod recordings;
pub mod schedule;
pub mod series;

use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{button, column, row};
use iced::{Element, Fill, Subscription, Task};
use jellium_protocol::TimerChanged;
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache};
use crate::style::{self, Drawn, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;
use crate::window;

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
    /// The recording a Delete has asked about.
    pub confirming: Option<Uuid>,
}

/// Every control the Live TV screen and the on-now row resolve to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Selected(Tab),
    /// Plays a channel here, or sends it to a bound target.
    PlayChannel(Uuid),
    Favorited(Uuid, bool),
    /// Filters the Channels tab.
    Kind(jellyfin_api::types::ChannelType),
    /// Opens a program's detail.
    Show(String),
    /// Creates a timer for a program from the server's defaults.
    Record(String),
    /// Opens the series options for a program, prefilled from the server's
    /// defaults.
    RecordSeries(String),
    /// Opens the series options of a series timer already created.
    EditSeries(String),
    Edited(series::Field),
    /// Creates or updates the series timer the options describe.
    ConfirmSeries,
    CloseSeries,
    CancelTimer(String),
    CancelSeriesTimer(String),
    PlayRecording(Uuid),
    /// Asks about deleting a completed recording, and deletes it.
    Delete(Uuid),
    ConfirmDelete(Uuid),
    CloseDelete,
    /// Cancels the timer writing an in-progress recording.
    StopRecording(String),
    /// Moves the guide one screen in time.
    Step(guide::Step),
    /// Moves the guide to a date.
    Date(chrono::NaiveDate),
    /// Moves the guide's focus.
    Focus(guide::Move),
    /// Opens the guide's focused cell.
    Open,
}

/// Loads the tab `tab` names, against a page `height` pixels tall.
pub async fn load(api: Rc<Api>, tab: Tab, height: Drawn) -> Answer<State> {
    Answer::of(async {
        let body = match tab {
            Tab::Guide => Body::Guide(guide::load(api, height).await?),
            Tab::Channels => Body::Channels(
                channels::load(api, jellyfin_api::types::ChannelType::Tv, height)
                    .await
                    .bubbled()?,
            ),
            Tab::Recordings => Body::Recordings(recordings::load(api, height).await.bubbled()?),
            Tab::Schedule => Body::Schedule(schedule::load(api, height).await.bubbled()?),
            Tab::Series => Body::Series(series::load(api, height).await.bubbled()?),
        };
        Ok(State {
            tab,
            body,
            editing: None,
            confirming: None,
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
) -> Element<'a, Message> {
    if let Some(editing) = &state.editing {
        return series::options(editing, viewport);
    }

    let tabs = row(Tab::ALL.iter().map(|tab| {
        button(prose(
            strings::lookup(tab.label()).to_owned(),
            typeface::BODY,
        ))
        .style(if *tab == state.tab {
            style::submit
        } else {
            style::raised
        })
        .on_press(Message::LiveTvAction(Action::Selected(*tab)))
        .into()
    }))
    .spacing(style::drawn(space::GUTTER.drawn()));

    let body = match &state.body {
        Body::Guide(guide) => guide::view(guide, now, images, viewport),
        Body::Channels(channels) => channels::view(channels, now, images),
        Body::Recordings(held) => recordings::view(held, state.confirming, images),
        Body::Schedule(schedule) => schedule::view(schedule),
        Body::Series(series) => series::view(series),
    };

    column![
        prose(
            strings::lookup(Text::LiveTvTitle).to_owned(),
            typeface::HEADING_2
        ),
        tabs,
        body,
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .width(Fill)
    .height(Fill)
    .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    match &state.body {
        Body::Guide(held) => guide::images(held),
        Body::Channels(held) => channels::images(held),
        Body::Recordings(held) => recordings::images(held),
        Body::Schedule(_) | Body::Series(_) => HashSet::new(),
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
        crate::app::View::LiveTv(state) => Some(state),
        _ => None,
    }
}

/// Reloads the tab shown, which is how a write's effect is taken up without
/// moving the scroll position or the order.
fn reload(signed: &Signed, tab: Tab, viewport: Viewport) -> Task<Message> {
    Task::perform(
        load(signed.api.clone(), tab, viewport.canvas().height()),
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
        Action::Favorited(channel, wanted) => Task::perform(
            async move { api.set_favorite(channel, wanted).await.map(|_| ()) },
            move |outcome| Message::Wrote(Operation::Timer, outcome),
        ),
        Action::Kind(kind) => {
            let height = viewport.canvas().height();
            Task::perform(
                async move { channels::load(api, kind, height).await },
                |loaded| {
                    Message::LiveTvLoaded(loaded.map(|channels| State {
                        tab: Tab::Channels,
                        body: Body::Channels(channels),
                        editing: None,
                        confirming: None,
                    }))
                },
            )
        }
        Action::Show(program) => Task::done(Message::Navigated(crate::route::Route::Program {
            id: program,
        })),
        Action::Record(program) => {
            Task::perform(async move { api.record(&program).await }, |outcome| {
                Message::Wrote(Operation::Timer, outcome)
            })
        }
        Action::RecordSeries(program) => Task::perform(
            async move {
                api.timer_defaults(&program)
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
        Action::CancelTimer(timer) | Action::StopRecording(timer) => {
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
        Action::Delete(recording) => {
            if let Some(state) = showing(signed) {
                state.confirming = Some(recording);
            }
            Task::none()
        }
        Action::CloseDelete => {
            if let Some(state) = showing(signed) {
                state.confirming = None;
            }
            Task::none()
        }
        Action::ConfirmDelete(recording) => {
            if let Some(state) = showing(signed) {
                state.confirming = None;
            }
            Task::perform(
                async move { api.delete_recording(recording).await },
                |outcome| Message::Wrote(Operation::Recording, outcome),
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
                    let id = program.id.clone();
                    act(signed, Action::Show(id), viewport)
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
