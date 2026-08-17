//! The first-run wizard: six steps in jellyfin-web's order, reached by Next and
//! left by Back, whose values live on the Jellyfin server.

pub mod finish;
pub mod language;
pub mod libraries;
pub mod metadata;
pub mod remote;
pub mod user;

use iced::widget::{button, center, row};
use iced::{Element, Task};
use jellium_model::setup::Step;

use crate::app::Message;
use crate::error::{Answer, Operation};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Choice, prose};

pub struct State {
    pub startup: jellium_protocol::Startup,
    pub api: std::rc::Rc<crate::api::Api>,
    pub step: Step,
    pub body: Body,
    /// True while a step load or a write is in flight.
    pub working: bool,
}

/// The step shown, holding what it read from the Jellyfin server on entry.
#[derive(Debug, Clone)]
pub enum Body {
    Loading,
    Language(language::State),
    User(user::State),
    Libraries(libraries::State),
    Metadata(metadata::State),
    RemoteAccess(remote::State),
    Finish,
}

/// One value the shown step holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edit {
    ServerName(String),
    Culture(Choice<String>),
    UserName(String),
    Password(String),
    Confirmation(String),
    LibraryName(String),
    ContentType(Choice<String>),
    Renaming(String),
    MetadataLanguage(Choice<String>),
    MetadataCountry(Choice<String>),
    RemoteAccess(bool),
    PortMapping(bool),
}

/// Every control the wizard resolves to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Edited(Edit),
    /// Writes the shown step and moves to the next one.
    Next,
    /// Moves to the previous step, and on the first one leaves the wizard.
    Back,
    /// Opens the add-library dialog, and abandons it.
    Adding(bool),
    /// Browses the server's own filesystem at `path`; the empty path opens
    /// where the server says its browser opens.
    Browse(String),
    /// Browses the parent of the directory the browser stands in.
    BrowseUp,
    /// Adds and removes a media path of the library being added.
    AddPath(String),
    RemovePath(usize),
    CreateLibrary,
    RenameLibrary {
        name: String,
    },
    RemoveLibrary {
        name: String,
    },
    /// Begins renaming a library.
    Renaming {
        name: String,
    },
    /// Posts `Startup/Complete`.
    Complete,
}

/// The wizard as it opens: the first step and the task that reads it.
pub fn enter(startup: jellium_protocol::Startup) -> (State, Task<Message>) {
    let api = std::rc::Rc::new(crate::api::Api::anonymous());
    let state = State {
        startup,
        api: api.clone(),
        step: Step::Language,
        body: Body::Loading,
        working: true,
    };
    (state, reading(api, Step::Language))
}

fn reading(api: std::rc::Rc<crate::api::Api>, step: Step) -> Task<Message> {
    Task::perform(load(api, step), Message::SetupLoaded)
}

/// Reads `step`'s stored values from the Jellyfin server.
pub async fn load(api: std::rc::Rc<crate::api::Api>, step: Step) -> Answer<(Step, Body)> {
    Answer::of(async {
        let body = match step {
            Step::Language => Body::Language(language::load(api).await.bubbled()?),
            Step::User => Body::User(user::load(api).await.bubbled()?),
            Step::Libraries => Body::Libraries(libraries::load(api).await.bubbled()?),
            Step::Metadata => Body::Metadata(metadata::load(api).await.bubbled()?),
            Step::RemoteAccess => Body::RemoteAccess(remote::load().await.bubbled()?),
            Step::Finish => Body::Finish,
        };
        Ok((step, body))
    })
    .await
}

/// True when the shown step admits Next.
fn ready(state: &State) -> bool {
    match &state.body {
        Body::User(user) => jellium_model::setup::user_ready(
            &user.user.name,
            &user.user.password,
            &user.confirmation,
        ),
        Body::Loading | Body::Finish => false,
        _ => true,
    }
}

/// Writes the shown step, so the step after it reads what this one stored.
fn written(state: &State) -> Task<Message> {
    let advanced = |outcome| Message::SetupAdvanced(outcome);
    match &state.body {
        Body::Language(language) => Task::perform(
            crate::control::set_setup_configuration(language.configuration.clone()),
            advanced,
        ),
        Body::Metadata(metadata) => Task::perform(
            crate::control::set_setup_configuration(metadata.configuration.clone()),
            advanced,
        ),
        Body::User(user) => {
            Task::perform(crate::control::set_setup_user(user.user.clone()), advanced)
        }
        Body::RemoteAccess(remote) => Task::perform(
            crate::control::set_setup_remote_access(remote.access),
            advanced,
        ),
        Body::Libraries(_) | Body::Loading | Body::Finish => {
            Task::perform(Answer::of(async { Ok(()) }), Message::SetupAdvanced)
        }
    }
}

fn edited(state: &mut State, edit: Edit) {
    match (&mut state.body, edit) {
        (Body::Language(language), Edit::ServerName(typed)) => {
            language.configuration.server_name = typed;
        }
        (Body::Language(language), Edit::Culture(choice)) => {
            language.configuration.ui_culture = choice.value;
        }
        (Body::User(user), Edit::UserName(typed)) => user.user.name = typed,
        (Body::User(user), Edit::Password(typed)) => user.user.password = typed,
        (Body::User(user), Edit::Confirmation(typed)) => user.confirmation = typed,
        (Body::Metadata(metadata), Edit::MetadataLanguage(choice)) => {
            metadata.configuration.preferred_metadata_language = choice.value;
        }
        (Body::Metadata(metadata), Edit::MetadataCountry(choice)) => {
            metadata.configuration.metadata_country_code = choice.value;
        }
        (Body::RemoteAccess(remote), Edit::RemoteAccess(on)) => {
            remote.access.enable_remote_access = on;
        }
        (Body::RemoteAccess(remote), Edit::PortMapping(on)) => {
            remote.access.enable_automatic_port_mapping = on;
        }
        (Body::Libraries(libraries), Edit::LibraryName(typed)) => {
            if let Some(adding) = &mut libraries.adding {
                adding.name = typed;
            }
        }
        (Body::Libraries(libraries), Edit::ContentType(choice)) => {
            if let Some(adding) = &mut libraries.adding {
                adding.content_type = choice;
            }
        }
        (Body::Libraries(libraries), Edit::Renaming(typed)) => {
            if let Some((_, held)) = &mut libraries.renaming {
                *held = typed;
            }
        }
        _ => {}
    }
}

/// The library being added on the libraries step, and `None` on every other.
fn adding(state: &mut State) -> Option<&mut libraries::Adding> {
    match &mut state.body {
        Body::Libraries(libraries) => libraries.adding.as_mut(),
        _ => None,
    }
}

/// A library write, which re-reads the step it landed on.
fn wrote(state: &mut State, task: Task<Message>) -> Task<Message> {
    state.working = true;
    task
}

/// Applies one control.
pub fn act(state: &mut State, action: Action) -> Task<Message> {
    let api = state.api.clone();
    match action {
        Action::Edited(edit) => {
            edited(state, edit);
            Task::none()
        }
        Action::Next => {
            if !ready(state) {
                return Task::none();
            }
            state.working = true;
            written(state)
        }
        Action::Back => match state.step.back() {
            Some(step) => {
                state.step = step;
                state.body = Body::Loading;
                state.working = true;
                reading(api, step)
            }
            None => Task::perform(crate::control::leave_setup(), Message::SetupLeft),
        },
        Action::Adding(open) => {
            if let Body::Libraries(libraries) = &mut state.body {
                libraries::adding(libraries, open);
            }
            Task::none()
        }
        Action::Renaming { name } => {
            if let Body::Libraries(libraries) = &mut state.body {
                libraries.renaming = Some((name.clone(), name));
            }
            Task::none()
        }
        Action::Browse(path) => {
            state.working = true;
            Task::perform(browse(api, path), Message::SetupBrowsed)
        }
        Action::BrowseUp => {
            let Some(standing) = adding(state).and_then(|adding| adding.browsing.clone()) else {
                return Task::none();
            };
            state.working = true;
            Task::perform(up(api, standing), Message::SetupBrowsed)
        }
        Action::AddPath(path) => {
            if let Some(adding) = adding(state)
                && !adding.paths.contains(&path)
            {
                adding.paths.push(path);
            }
            Task::none()
        }
        Action::RemovePath(index) => {
            if let Some(adding) = adding(state)
                && index < adding.paths.len()
            {
                adding.paths.remove(index);
            }
            Task::none()
        }
        Action::CreateLibrary => {
            let Some(adding) = adding(state) else {
                return Task::none();
            };
            let name = adding.name.clone();
            let content_type = adding.content_type.value.clone();
            let paths = adding.paths.clone();
            wrote(
                state,
                Task::perform(
                    Answer::of(create(api, name, content_type, paths)),
                    Message::SetupWrote,
                ),
            )
        }
        Action::RenameLibrary { name } => {
            let Some(held) = renaming_from(state) else {
                return Task::none();
            };
            wrote(
                state,
                Task::perform(Answer::of(rename(api, held, name)), Message::SetupWrote),
            )
        }
        Action::RemoveLibrary { name } => wrote(
            state,
            Task::perform(Answer::of(remove(api, name)), Message::SetupWrote),
        ),
        Action::Complete => {
            state.working = true;
            Task::perform(crate::control::complete_setup(), Message::SetupCompleted)
        }
    }
}

/// The library a rename is being typed for.
fn renaming_from(state: &State) -> Option<String> {
    match &state.body {
        Body::Libraries(libraries) => libraries.renaming.as_ref().map(|(held, _)| held.clone()),
        _ => None,
    }
}

async fn create(
    api: std::rc::Rc<crate::api::Api>,
    name: String,
    content_type: String,
    paths: Vec<String>,
) -> Result<(), crate::error::Bubble> {
    api.create_library(&name, &content_type, &serde_json::json!({}))
        .await
        .bubbled()?;
    for path in paths {
        api.add_path(&name, &path).await.bubbled()?;
    }
    Ok(())
}

async fn rename(
    api: std::rc::Rc<crate::api::Api>,
    name: String,
    renamed: String,
) -> Result<(), crate::error::Bubble> {
    api.rename_library(&name, &renamed).await.bubbled()
}

async fn remove(
    api: std::rc::Rc<crate::api::Api>,
    name: String,
) -> Result<(), crate::error::Bubble> {
    api.delete_library(&name).await.bubbled()
}

async fn browse(
    api: std::rc::Rc<crate::api::Api>,
    path: String,
) -> Answer<(String, Vec<jellyfin_api::types::FileSystemEntryInfo>)> {
    Answer::of(async {
        let path = match path.is_empty() {
            true => api.default_directory().await.bubbled()?,
            false => path,
        };
        let entries = api.directory(&path).await.bubbled()?;
        Ok((path, entries))
    })
    .await
}

async fn up(
    api: std::rc::Rc<crate::api::Api>,
    standing: String,
) -> Answer<(String, Vec<jellyfin_api::types::FileSystemEntryInfo>)> {
    Answer::of(async {
        let parent = api.parent_path(&standing).await.bubbled()?;
        let entries = api.directory(&parent).await.bubbled()?;
        Ok((parent, entries))
    })
    .await
}

/// Installs a loaded step.
pub fn stepped(state: &mut State, step: Step, body: Body) {
    state.step = step;
    state.body = body;
    state.working = false;
}

/// The write every setup step names its refusal as, which is what the step's
/// answer is read through.
pub fn step_write() -> crate::error::Wrote {
    crate::error::Wrote {
        operation: Operation::SetupStep,
        object: String::new(),
    }
}

/// Clears `working` after a refused step; the step keeps its values and does
/// not advance, and the refusal was raised by the door its answer was read
/// through.
pub fn refused(state: &mut State) {
    state.working = false;
}

/// Takes what the server's filesystem browser answered.
pub fn browsed(
    state: &mut State,
    path: String,
    entries: Vec<jellyfin_api::types::FileSystemEntryInfo>,
) {
    state.working = false;
    if let Some(adding) = adding(state) {
        adding.browsing = Some(path);
        adding.entries = entries;
    }
}

/// The step the wizard shows once the shown one was written.
pub fn advanced(state: &mut State) -> Task<Message> {
    let api = state.api.clone();
    match state.step.next() {
        Some(step) => {
            state.step = step;
            state.body = Body::Loading;
            state.working = true;
            reading(api, step)
        }
        None => {
            state.working = false;
            Task::none()
        }
    }
}

/// Re-reads the shown step, which is what a library write inside it does.
pub fn reread(state: &mut State) -> Task<Message> {
    let api = state.api.clone();
    state.working = true;
    reading(api, state.step)
}

fn body(state: &State) -> Element<'_, Message> {
    match &state.body {
        Body::Loading => prose(strings::lookup(Text::LoginWorking), typeface::BODY),
        Body::Language(language) => language::view(language),
        Body::User(user) => user::view(user),
        Body::Libraries(libraries) => libraries::view(libraries),
        Body::Metadata(metadata) => metadata::view(metadata),
        Body::RemoteAccess(remote) => remote::view(remote),
        Body::Finish => finish::view(),
    }
}

/// The chrome and the step: the wizard's title, the step's position stated as
/// text and never as a control, the off-snapshot warning when the server's
/// version and the snapshot's differ, the sentence naming a resumed saved
/// server, Back, Next, and the refusal over the server's own message.
pub fn view(state: &State, viewport: Viewport) -> Element<'_, Message> {
    let rows = [
        prose(strings::lookup(Text::SetupTitle), typeface::HEADING_2),
        prose(
            strings::format(
                Text::SetupPosition,
                &[
                    &state.step.position().to_string(),
                    &Step::ORDER.len().to_string(),
                ],
            ),
            typeface::SECONDARY,
        ),
    ]
    .into_iter()
    .chain(state.startup.off_snapshot().then(|| {
        prose(
            strings::format(
                Text::WarningOffSnapshot,
                &[
                    &state.startup.server_version,
                    &state.startup.snapshot_version,
                ],
            ),
            typeface::SECONDARY,
        )
    }))
    .chain(
        state
            .startup
            .resumed
            .then(|| prose(strings::lookup(Text::SetupResumed), typeface::SECONDARY)),
    );

    let mut controls = row![
        button(prose(strings::lookup(Text::SetupBack), typeface::BODY))
            .style(style::raised)
            .on_press(Message::SetupAction(Action::Back)),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));
    if ready(state) && !state.working {
        controls = controls.push(
            button(prose(strings::lookup(Text::SetupNext), typeface::BODY))
                .style(style::submit)
                .on_press(Message::SetupAction(Action::Next)),
        );
    }
    center(crate::widget::scrolled(
        iced::widget::container(widget::capped(
            viewport,
            space::FIELD_GAP,
            rows.chain([body(state), controls.into()]),
        ))
        .padding(style::padding(space::PAGE_PAD)),
    ))
    .into()
}
