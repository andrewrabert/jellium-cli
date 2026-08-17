//! The scheduled tasks the server holds, and the one task a task screen shows.

use iced::Element;
use iced::widget::{button, row};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};

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
                .filter_map(shaped)
                .collect(),
        })
    })
    .await
}

/// One task as the dashboard takes it; a task the server named no id for is
/// dropped.
fn shaped(info: jellyfin_api::types::TaskInfo) -> Option<jellium_protocol::TaskState> {
    use jellyfin_api::types::TaskState as Upstream;
    Some(jellium_protocol::TaskState {
        id: info.id?,
        name: info.name.unwrap_or_default(),
        category: info.category.unwrap_or_default(),
        description: info.description.unwrap_or_default(),
        state: match info.state {
            Some(Upstream::Cancelling) => jellium_protocol::TaskRunState::Cancelling,
            Some(Upstream::Running) => jellium_protocol::TaskRunState::Running,
            Some(Upstream::Idle) | None => jellium_protocol::TaskRunState::Idle,
        },
        progress: info.current_progress_percentage,
    })
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

fn state_text(state: jellium_protocol::TaskRunState) -> Text {
    match state {
        jellium_protocol::TaskRunState::Idle => Text::TasksIdle,
        jellium_protocol::TaskRunState::Running => Text::TasksRunning,
        jellium_protocol::TaskRunState::Cancelling => Text::TasksCancelling,
    }
}

/// Every task with its state and its running progress.
pub fn view<'a>(state: &'a State, read_only: bool) -> Vec<Element<'a, Message>> {
    let mut page: Vec<Element<'a, Message>> = Vec::new();

    for task in &state.tasks {
        let progress = task
            .progress
            .map(|progress| format!(" {progress:.0}%"))
            .unwrap_or_default();
        let mut held = row![
            button(line(
                task.name.clone(),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ))
            .style(style::link)
            .on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::Task {
                    id: task.id.clone(),
                }
            ))),
            line(
                format!("{}{progress}", strings::lookup(state_text(task.state))),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()));

        if !read_only {
            held = held.push(match task.state {
                jellium_protocol::TaskRunState::Running => {
                    button(prose(strings::lookup(Text::TasksStop), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::DashboardAction(super::Action::Ask(
                            crate::screen::confirm::Pending::of(
                                crate::screen::confirm::Destructive::StopTask {
                                    id: task.id.clone(),
                                },
                                task.name.clone(),
                            ),
                        )))
                }
                _ => button(prose(strings::lookup(Text::TasksStart), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Write(
                        super::Written::StartTask {
                            id: task.id.clone(),
                            name: task.name.clone(),
                        },
                    ))),
            });
        }
        page.push(held.into());
    }

    page
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
                Text::TasksLastRun,
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
                strings::format(Text::TasksDuration, &[&format!("{}", ran.num_seconds())]),
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
