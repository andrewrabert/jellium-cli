//! Dashboard home: what the server is, who is on it, and what it is doing.

use iced::Element;
use iced::widget::{button, row};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Template, Text};
use crate::widget::prose;

/// What dashboard home shows, all of it updating without the user acting.
#[derive(Debug, Clone)]
pub struct State {
    pub info: jellyfin_api::types::SystemInfo,
    /// Every session on the server, this client's own among them.
    pub sessions: Vec<jellium_protocol::ServerSession>,
    /// The scheduled tasks, of which the running ones are shown.
    pub tasks: Vec<jellium_protocol::TaskState>,
    /// How far the library scan has got, and nothing while none runs.
    pub scanning: Option<f64>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            info: api.system_info().await.bubbled()?,
            sessions: Vec::new(),
            tasks: Vec::new(),
            scanning: None,
        })
    })
    .await
}

/// Takes the session listing one push carried.
pub fn sessions(state: &mut State, sessions: Vec<jellium_protocol::ServerSession>) {
    state.sessions = sessions;
}

/// Takes the tasks one push carried; the scan indicator is the library scan's
/// own progress, told apart from every other task.
pub fn tasks(state: &mut State, tasks: Vec<jellium_protocol::TaskState>) {
    state.scanning = tasks
        .iter()
        .find(|task| task.id.eq_ignore_ascii_case("RefreshLibrary"))
        .and_then(|task| task.progress);
    state.tasks = tasks;
}

fn running(state: &State) -> impl Iterator<Item = &jellium_protocol::TaskState> {
    state
        .tasks
        .iter()
        .filter(|task| task.state == jellium_protocol::TaskRunState::Running)
}

/// The server's name and version, every session with what it plays, each
/// running task with its progress, the scan indicator, and the global scan,
/// restart and shutdown controls.
pub fn view<'a>(state: &'a State, read_only: bool) -> Vec<Element<'a, Message>> {
    let mut page: Vec<Element<'a, Message>> = vec![
        prose(
            strings::format(
                Template::DashboardServer,
                &[&state.info.server_name.clone().unwrap_or_default()],
            ),
            typeface::BODY,
        ),
        prose(
            strings::format(
                Template::DashboardVersion,
                &[&state.info.version.clone().unwrap_or_default()],
            ),
            typeface::BODY,
        ),
    ];

    if let Some(progress) = state.scanning {
        page.push(prose(
            strings::format(Template::DashboardScanning, &[&format!("{progress:.0}")]),
            typeface::BODY,
        ));
    }

    page.push(prose(
        strings::lookup(Text::DashboardSessions),
        typeface::BODY,
    ));
    for session in &state.sessions {
        let playing = session
            .playing
            .clone()
            .unwrap_or_else(|| strings::lookup(Text::DashboardSessionNothing).to_string());
        page.push(prose(
            format!(
                "{} · {} · {} · {}",
                session.device_name, session.client_name, session.user_name, playing
            ),
            typeface::BODY,
        ));
    }

    page.push(prose(
        strings::lookup(Text::DashboardRunningTasks),
        typeface::BODY,
    ));
    for task in running(state) {
        let progress = task
            .progress
            .map(|progress| format!("{progress:.0}%"))
            .unwrap_or_default();
        page.push(prose(format!("{} {progress}", task.name), typeface::BODY));
    }

    if !read_only {
        page.push(
            row![
                button(prose(
                    strings::lookup(Text::DashboardScanAll),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::ScanAll
                ))),
                button(prose(
                    strings::lookup(Text::DashboardRestart),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::Restart,
                        state.info.server_name.clone().unwrap_or_default(),
                    )
                ))),
                button(prose(
                    strings::lookup(Text::DashboardShutdown),
                    typeface::BODY
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::Shutdown,
                        state.info.server_name.clone().unwrap_or_default(),
                    )
                ))),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .into(),
        );
    }

    page
}
