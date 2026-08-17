//! The login stage: the saved server list, add-server, one server's
//! credentials, Quick Connect sign-in and password reset.

pub mod add;
pub mod credentials;
pub mod quickconnect;
pub mod reset;
pub mod servers;

use iced::{Element, Subscription, Task};
use uuid::Uuid;

use crate::app::Message;
use crate::control;
use crate::style::{Viewport, space};
use crate::text::Text;

pub struct State {
    pub screen: jellium_model::login::Screen,
    /// The saved servers as the last read answered them, in file order.
    pub servers: Vec<jellium_protocol::SavedServer>,
    /// The login screen held while a server is chosen.
    pub target: Option<jellium_protocol::LoginScreen>,
    pub add: add::State,
    pub credentials: credentials::State,
    pub quick_connect: quickconnect::State,
    pub reset: reset::State,
    /// The images fetched for the picker, keyed by user.
    pub images: std::collections::HashMap<Uuid, iced::widget::image::Handle>,
    pub read_only: bool,
    /// True while a request is in flight.
    pub working: bool,
    /// The one thing the login stage has to say that is not a failure, shown
    /// above the form.
    pub told: Option<String>,
}

/// One value the shown screen holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edit {
    Url(String),
    Username(String),
    Password(String),
    ResetUsername(String),
    Pin(String),
}

/// Every control the login stage resolves to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Edited(Edit),
    /// Opens add-server.
    Add,
    /// Probes the typed url and opens the login screen of the server that
    /// answered.
    AddSubmit,
    Select {
        server: String,
    },
    Remove {
        server: String,
    },
    /// Fills the name from the picker and shows the password field.
    Pick {
        user: Uuid,
        name: String,
    },
    /// Signs in with the name and password shown.
    Submit,
    /// Opens the Quick Connect screen.
    QuickConnect,
    /// Initiates a fresh request, which is what an expired one offers.
    QuickConnectRetry,
    /// Opens password reset.
    Reset,
    ResetSubmit,
    PinSubmit,
    /// Leaves the shown screen for the one it opened from.
    Back,
}

impl State {
    fn empty(read_only: bool) -> State {
        State {
            screen: jellium_model::login::Screen::Servers,
            servers: Vec::new(),
            target: None,
            add: add::State::default(),
            credentials: credentials::State::default(),
            quick_connect: quickconnect::State::default(),
            reset: reset::State::default(),
            images: std::collections::HashMap::new(),
            read_only,
            working: false,
            told: None,
        }
    }

    /// The login target every login-stage request presents, and the empty
    /// string while none is held.
    pub fn target(&self) -> String {
        self.target
            .as_ref()
            .map(|screen| screen.target.clone())
            .unwrap_or_default()
    }
}

/// The stage as it opens, on the list when a server is saved and on
/// add-server when none is.
pub fn enter(
    servers: Vec<jellium_protocol::SavedServer>,
    read_only: bool,
) -> (State, Task<Message>) {
    let mut state = State::empty(read_only);
    state.screen = if servers.is_empty() {
        jellium_model::login::Screen::Add
    } else {
        jellium_model::login::Screen::Servers
    };
    state.servers = servers;
    (state, Task::none())
}

/// The stage on one server's login screen, which is where a rejected saved
/// credential lands.
pub fn entered(screen: jellium_protocol::LoginScreen) -> (State, Task<Message>) {
    let mut state = State::empty(screen.read_only);
    state.screen = jellium_model::login::Screen::Credentials;
    if screen.rejected {
        crate::failure::raise(crate::error::told(Text::FailureSavedSignInRejected));
    }
    let target = screen.target.clone();
    state.target = Some(screen);
    let images = credentials::images(&state).into_iter().map(|user| {
        let target = target.clone();
        Task::perform(
            async move { control::public_image(target, user).await },
            move |result| Message::PublicImageLoaded(user, result),
        )
    });
    (
        state,
        Task::batch(
            std::iter::once(Task::perform(control::servers(), Message::ServersListed))
                .chain(images),
        ),
    )
}

pub fn view(state: &State, viewport: Viewport) -> Element<'_, Message> {
    let shown = match state.screen {
        jellium_model::login::Screen::Servers => servers::view(state, viewport),
        jellium_model::login::Screen::Add => add::view(state, viewport),
        jellium_model::login::Screen::Credentials => credentials::view(state, viewport),
        jellium_model::login::Screen::QuickConnect => quickconnect::view(state),
        jellium_model::login::Screen::Reset => reset::view(state, viewport),
    };
    let Some(told) = &state.told else {
        return shown;
    };
    iced::widget::column![crate::widget::banner(told.clone()), shown]
        .spacing(crate::style::drawn(space::GUTTER.drawn()))
        .into()
}

/// The five-second poll, held only while the Quick Connect screen shows a
/// pending request.
pub fn subscription(state: &State) -> Subscription<Message> {
    let pending = state.screen == jellium_model::login::Screen::QuickConnect
        && state.quick_connect.code.is_some()
        && matches!(
            state.quick_connect.standing,
            None | Some(jellium_model::quickconnect::SignIn::Pending)
        );
    if !pending {
        return Subscription::none();
    }
    iced::time::every(jellium_model::quickconnect::POLL).map(|_| Message::QuickConnectTicked)
}
