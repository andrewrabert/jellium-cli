//! The scheduled tasks the server holds, and the one task a task screen shows.

use iced::widget::{button, column, row, text};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;

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
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![text(strings::lookup(Text::TasksTitle)).size(22)]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    for task in &state.tasks {
        let progress = task
            .progress
            .map(|progress| format!(" {progress:.0}%"))
            .unwrap_or_default();
        let mut held = row![
            button(text(task.name.clone())).on_press(Message::DashboardAction(
                super::Action::Open(super::Screen::Task {
                    id: task.id.clone()
                })
            )),
            text(format!(
                "{}{progress}",
                strings::lookup(state_text(task.state))
            )),
        ]
        .spacing(theme::CARD_SPACING);

        if !read_only {
            held = held.push(match task.state {
                jellium_protocol::TaskRunState::Running => button(text(strings::lookup(
                    Text::TasksStop,
                )))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::StopTask {
                            id: task.id.clone(),
                        },
                        task.name.clone(),
                    ),
                ))),
                _ => button(text(strings::lookup(Text::TasksStart))).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::StartTask {
                        id: task.id.clone(),
                        name: task.name.clone(),
                    })),
                ),
            });
        }
        page = page.push(held);
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}

/// One task: its description, its last execution's status and duration, and
/// its triggers.
pub fn one<'a>(state: &'a One, read_only: bool) -> Element<'a, Message> {
    let mut page = column![
        text(state.info.name.clone().unwrap_or_default()).size(22),
        text(state.info.description.clone().unwrap_or_default()),
    ]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING);

    if let Some(last) = state.info.last_execution_result.as_ref() {
        page = page.push(text(strings::format(
            Text::TasksLastRun,
            &[&last
                .status
                .map(|status| status.to_string())
                .unwrap_or_default()],
        )));
        if let (Some(start), Some(end)) = (last.start_time_utc, last.end_time_utc) {
            let ran = end - start;
            page = page.push(text(strings::format(
                Text::TasksDuration,
                &[&format!("{}", ran.num_seconds())],
            )));
        }
    }

    page = page.push(text(strings::lookup(Text::TasksTriggers)));
    for (index, trigger) in state.triggers.iter().enumerate() {
        let mut held = row![text(
            trigger
                .type_
                .map(|kind| kind.to_string())
                .unwrap_or_default()
        )]
        .spacing(theme::CARD_SPACING);
        if !read_only {
            held = held.push(
                button(text(strings::lookup(Text::TasksTriggerRemove))).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::RemoveTrigger {
                        index,
                    })),
                ),
            );
        }
        page = page.push(held);
    }

    if !read_only {
        page = page.push(
            row![
                button(text(strings::lookup(Text::TasksTriggerDaily))).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::DailyTrigger,
                    }))
                ),
                button(text(strings::lookup(Text::TasksTriggerInterval))).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::IntervalTrigger,
                    }))
                ),
                button(text(strings::lookup(Text::TasksTriggerStartup))).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::AddTrigger {
                        kind: jellyfin_api::types::TaskTriggerInfoType::StartupTrigger,
                    }))
                ),
            ]
            .spacing(theme::CARD_SPACING),
        );
        page = page.push(button(text(strings::lookup(Text::DashboardSave))).on_press(
            Message::DashboardAction(super::Action::Write(super::Written::SetTriggers {
                id: state.info.id.clone().unwrap_or_default(),
                name: state.info.name.clone().unwrap_or_default(),
            })),
        ));
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}
