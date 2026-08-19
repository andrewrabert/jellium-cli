//! The scheduled tasks the server holds, and the one task a task screen shows.

use iced::Element;
use iced::widget::{button, column, row};

use super::frame;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::style::{self, Layout, Share, space, typeface};
use crate::text::{self as strings, Template, Text};
use crate::widget::{self, Showing, prose};

/// Every scheduled task, with the state and progress each push carries.
#[derive(Debug, Clone)]
pub struct State {
    pub tasks: Vec<jellium_protocol::TaskState>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            tasks: api
                .tasks()
                .await
                .bubbled()?
                .into_iter()
                .filter_map(jellium_model::task::taken)
                .collect(),
        })
    })
    .await
}

/// One task's screen: what it is, how its last run ended, and its triggers.
#[derive(Debug, Clone)]
pub struct One {
    pub info: jellyfin_api::types::TaskInfo,
    /// The triggers as they are to be written, whole.
    pub triggers: Vec<jellyfin_api::types::TaskTriggerInfo>,
}

pub async fn open(api: std::rc::Rc<crate::api::Api>, id: String) -> Answer<One> {
    Answer::of(async {
        let info = api.task(&id).await.bubbled()?;
        Ok(One {
            triggers: info.triggers.clone().unwrap_or_default(),
            info,
        })
    })
    .await
}

/// Takes the tasks one push carried, in place.
pub fn tasks(state: &mut State, tasks: Vec<jellium_protocol::TaskState>) {
    state.tasks = tasks;
}

/// The phrase date-fns writes for a span, singular where it counts one.
// reference: date-fns-distance-en-us
fn phrase(distance: jellium_model::distance::Distance) -> String {
    use jellium_model::distance::Distance;
    let (alone, several): (Text, Template) = match distance {
        Distance::LessThanMinutes(_) => (
            Text::DistanceLessThanMinute,
            Template::DistanceLessThanMinutes,
        ),
        Distance::Minutes(_) => (Text::DistanceMinute, Template::DistanceMinutes),
        Distance::AboutHours(_) => (Text::DistanceAboutHour, Template::DistanceAboutHours),
        Distance::Days(_) => (Text::DistanceDay, Template::DistanceDays),
        Distance::AboutMonths(_) => (Text::DistanceAboutMonth, Template::DistanceAboutMonths),
        Distance::Months(_) => (Text::DistanceMonth, Template::DistanceMonths),
        Distance::AboutYears(_) => (Text::DistanceAboutYear, Template::DistanceAboutYears),
        Distance::OverYears(_) => (Text::DistanceOverYear, Template::DistanceOverYears),
        Distance::AlmostYears(_) => (Text::DistanceAlmostYear, Template::DistanceAlmostYears),
    };
    match distance.count() {
        1 => strings::lookup(alone).to_string(),
        count => strings::format(several, &[&count.to_string()]),
    }
}

/// How far a moment stands from now, in words, with the suffix its side of now
/// asks for.
// reference: date-fns-distance
fn since(at: chrono::DateTime<chrono::Utc>, now: chrono::DateTime<chrono::Utc>) -> String {
    let said = phrase(jellium_model::distance::Distance::between(at, now));
    match jellium_model::distance::Sense::of(at, now) {
        jellium_model::distance::Sense::Passed => {
            strings::format(Template::DistancePassed, &[&said])
        }
        jellium_model::distance::Sense::Ahead => strings::format(Template::DistanceAhead, &[&said]),
    }
}

/// The sentence saying how long ago a task last ran and how long that run took.
// reference: task-last-ran
fn last_ran(run: jellium_protocol::TaskRun, now: chrono::DateTime<chrono::Utc>) -> String {
    strings::format(
        Template::TasksLastRan,
        &[
            &since(run.ended, now),
            &phrase(jellium_model::distance::Distance::between(
                run.started,
                run.ended,
            )),
        ],
    )
}

/// The word the reference writes after the sentence for an ending it names.
// reference: task-last-ran
fn ending(ending: jellium_protocol::TaskEnding) -> Option<Text> {
    match ending {
        jellium_protocol::TaskEnding::Completed => None,
        jellium_protocol::TaskEnding::Failed => Some(Text::TasksFailed),
        jellium_protocol::TaskEnding::Cancelled => Some(Text::TasksCancelled),
        jellium_protocol::TaskEnding::Aborted => Some(Text::TasksAborted),
    }
}

/// What a task writes under its own name.
// reference: task-progress
// reference: task-last-ran
fn beneath<'a>(
    task: &'a jellium_protocol::TaskState,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<widget::modern::Beneath<'a>> {
    match task.state {
        jellium_protocol::TaskRunState::Running => Some(widget::modern::Beneath::Running(
            task.progress
                .map(|progress| Share::part((progress * 100.0) as i64, 10_000)),
        )),
        jellium_protocol::TaskRunState::Cancelling => Some(widget::modern::Beneath::Said(
            strings::lookup(Text::TasksStopping).into(),
        )),
        jellium_protocol::TaskRunState::Idle => {
            let run = task.last_ran?;
            let mut said = last_ran(run, now);
            if let Some(named) = ending(run.ending) {
                said.push_str(strings::lookup(named));
            }
            Some(widget::modern::Beneath::Ran(said.into()))
        }
    }
}

/// The control that starts a task or stops the one that is running.
// reference: tasks-row
fn control(task: &jellium_protocol::TaskState) -> widget::modern::Trailing {
    match task.state {
        jellium_protocol::TaskRunState::Running => widget::modern::Trailing {
            glyph: Icon::Stop,
            label: None,
            press: Message::DashboardAction(super::Action::Ask(
                crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::StopTask {
                        id: task.id.clone(),
                    },
                    task.name.clone(),
                ),
            )),
        },
        _ => widget::modern::Trailing {
            glyph: Icon::PlayArrow,
            label: None,
            press: Message::DashboardAction(super::Action::Write(super::Written::StartTask {
                id: task.id.clone(),
                name: task.name.clone(),
            })),
        },
    }
}

/// The heading over each category and the tasks it holds as MUI's own list,
/// the categories in name order and a category's tasks in name order, each row
/// reaching its own screen and carrying the control that starts it or stops it.
// the three endings the reference writes after the sentence are written in the
// same secondary lettering the sentence is, where the reference paints two of
// them in its own error colour and one in the css named blue
// reference: tasks-page
// reference: tasks-categories
// reference: tasks-category
// reference: tasks-row
// reference: task-progress
// reference: task-last-ran
pub fn view<'a>(
    state: &'a State,
    read_only: bool,
    now: chrono::DateTime<chrono::Utc>,
    layout: Layout,
) -> frame::Filling<'a> {
    let mut categories: std::collections::BTreeMap<&'a str, Vec<&'a jellium_protocol::TaskState>> =
        std::collections::BTreeMap::new();
    for task in &state.tasks {
        categories
            .entry(task.category.as_str())
            .or_default()
            .push(task);
    }

    let mut page: Vec<Element<'a, Message>> = Vec::new();
    for (category, mut held) in categories {
        held.sort_by(|one, other| one.name.cmp(&other.name));
        page.push(
            column![
                widget::modern::heading(typeface::Rank::Second, category),
                widget::modern::listed(
                    held.into_iter().map(|task| widget::modern::Row {
                        lead: Some(widget::modern::Lead::Avatar(Icon::AccessTime)),
                        primary: widget::modern::Primary::Headed(
                            typeface::Rank::Third,
                            task.name.as_str().into(),
                        ),
                        beneath: beneath(task, now),
                        within: None,
                        showing: Some(Showing::Offered(Message::DashboardAction(
                            super::Action::Open(super::Screen::Task {
                                id: task.id.clone(),
                            }),
                        ))),
                        trailing: (!read_only).then(|| control(task)),
                    }),
                    layout,
                ),
            ]
            .spacing(style::drawn(space::CATEGORY_GAP.drawn(layout)))
            .into(),
        );
    }

    frame::Filling::Capped {
        above: Some(space::TASKS_TOP),
        rows: page,
    }
}

/// One task: its description, its last execution's status and duration, and
/// its triggers.
pub fn one<'a>(state: &'a One, read_only: bool) -> Vec<Element<'a, Message>> {
    let mut page: Vec<Element<'a, Message>> = vec![
        prose(
            state.info.name.clone().unwrap_or_default(),
            typeface::HEADING_2,
        ),
        prose(
            state.info.description.clone().unwrap_or_default(),
            typeface::BODY,
        ),
    ];

    if let Some(last) = state.info.last_execution_result.as_ref() {
        page.push(prose(
            strings::format(
                Template::TasksLastRun,
                &[&last
                    .status
                    .map(|status| status.to_string())
                    .unwrap_or_default()],
            ),
            typeface::BODY,
        ));
        if let (Some(start), Some(end)) = (last.start_time_utc, last.end_time_utc) {
            let ran = end - start;
            page.push(prose(
                strings::format(
                    Template::TasksDuration,
                    &[&format!("{}", ran.num_seconds())],
                ),
                typeface::BODY,
            ));
        }
    }

    page.push(prose(strings::lookup(Text::TasksTriggers), typeface::BODY));
    for (index, trigger) in state.triggers.iter().enumerate() {
        let mut held = row![prose(
            trigger
                .type_
                .map(|kind| kind.to_string())
                .unwrap_or_default(),
            typeface::BODY
        )]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()));
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::TasksTriggerRemove),
                    typeface::BODY,
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::RemoveTrigger { index },
                ))),
            );
        }
        page.push(held.into());
    }

    if !read_only {
        page.push(
            row![
                button(prose(
                    strings::lookup(Text::TasksTriggerDaily),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::DailyTrigger,
                    }
                ))),
                button(prose(
                    strings::lookup(Text::TasksTriggerInterval),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::IntervalTrigger,
                    }
                ))),
                button(prose(
                    strings::lookup(Text::TasksTriggerStartup),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::StartupTrigger,
                    }
                ))),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .into(),
        );
        page.push(
            button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                .style(style::submit)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::SetTriggers {
                        id: state.info.id.clone().unwrap_or_default(),
                        name: state.info.name.clone().unwrap_or_default(),
                    },
                )))
                .into(),
        );
    }

    page
}
