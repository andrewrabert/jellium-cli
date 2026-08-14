use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{center, text};
use iced::{Element, Task, Theme};
use jellium_protocol::{Session, SessionStatus};
use jellyfin_api::types::UserItemDataDto;
use uuid::Uuid;

use crate::api::Api;
use crate::boot;
use crate::control;
use crate::error::Trouble;
use crate::images::{self, Cache};
use crate::route::Route;
use crate::screen::library::{PAGE_SIZE, Sort, Step};
use crate::screen::{detail, home, library, login, search};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

pub struct Jellium {
    pub stage: Stage,
    pub images: Cache,
}

#[allow(clippy::large_enum_variant)]
pub enum Stage {
    Booting,
    Login(login::State),
    Signed(Signed),
    Failed(Trouble),
}

pub struct Signed {
    pub session: Session,
    pub api: Rc<Api>,
    pub history: Vec<Route>,
    pub view: View,
    /// The last failure the view survived, shown above it.
    pub notice: Option<Trouble>,
}

pub enum View {
    Loading,
    Home(home::State),
    Library(library::State),
    Detail(detail::State),
    Search(search::State),
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "the message set is the app's declared control surface"
)]
pub enum Message {
    Ready,
    SessionChecked(Result<SessionStatus, Trouble>),
    LoginEdited(login::Field, String),
    LoginSubmitted,
    LoggedIn(Result<SessionStatus, Trouble>),
    LogoutPressed,
    LoggedOut(Result<(), Trouble>),
    Navigated(Route),
    WentBack,
    HomeLoaded(Result<home::State, Trouble>),
    LibraryLoaded(Result<library::State, Trouble>),
    DetailLoaded(Result<detail::State, Trouble>),
    SearchLoaded(Result<search::State, Trouble>),
    SortSelected(Sort),
    PageStepped(Step),
    SearchEdited(String),
    SearchSubmitted,
    PlayedToggled(Uuid, bool),
    FavoriteToggled(Uuid, bool),
    UserDataUpdated(Uuid, Result<UserItemDataDto, Trouble>),
    ImageWanted(images::Key),
    ImageLoaded(images::Key, Result<Vec<u8>, Trouble>),
}

impl Signed {
    fn route(&self) -> Option<&Route> {
        self.history.last()
    }

    fn wanted_images(&self) -> HashSet<images::Key> {
        match &self.view {
            View::Loading => HashSet::new(),
            View::Home(state) => home::images(state),
            View::Library(state) => library::images(state),
            View::Detail(state) => detail::images(state),
            View::Search(state) => search::images(state),
        }
    }

    fn items_mut(&mut self) -> Vec<&mut jellyfin_api::types::BaseItemDto> {
        match &mut self.view {
            View::Loading => Vec::new(),
            View::Home(state) => state
                .libraries
                .iter_mut()
                .chain(state.continue_watching.iter_mut())
                .chain(state.next_up.iter_mut())
                .collect(),
            View::Library(state) => state.page.items.iter_mut().collect(),
            View::Detail(state) => std::iter::once(&mut state.item)
                .chain(state.children.iter_mut())
                .collect(),
            View::Search(state) => state.results.items.iter_mut().collect(),
        }
    }
}

fn load(api: Rc<Api>, route: &Route) -> Task<Message> {
    match route.clone() {
        Route::Home => Task::perform(home::load(api), Message::HomeLoaded),
        Route::Library { id, sort, start } => {
            Task::perform(library::load(api, id, sort, start), Message::LibraryLoaded)
        }
        Route::Detail { id } => Task::perform(detail::load(api, id), Message::DetailLoaded),
        Route::Search { term, start } => {
            Task::perform(search::load(api, term, start), Message::SearchLoaded)
        }
    }
}

impl Jellium {
    pub fn boot() -> (Jellium, Task<Message>) {
        (
            Jellium {
                stage: Stage::Booting,
                images: Cache::new(),
            },
            Task::batch([
                Task::done(Message::Ready),
                Task::perform(control::status(), Message::SessionChecked),
            ]),
        )
    }

    fn signed(&mut self) -> Option<&mut Signed> {
        match &mut self.stage {
            Stage::Signed(signed) => Some(signed),
            _ => None,
        }
    }

    fn route(&self) -> Option<Route> {
        match &self.stage {
            Stage::Signed(signed) => signed.route().cloned(),
            _ => None,
        }
    }

    /// A rejected sign-in keeps the login screen with its typed server and
    /// username, clears `submitting`, and shows the reason under the form.
    fn enter(&mut self, status: SessionStatus) -> Task<Message> {
        match status {
            SessionStatus::Anonymous => {
                self.stage = Stage::Login(login::State::default());
                Task::none()
            }
            SessionStatus::Failed(failure) => {
                self.refuse(Trouble::Upstream(failure));
                Task::none()
            }
            SessionStatus::Authenticated(session) => {
                let api = Rc::new(Api::new(session.user_id));
                let task = load(api.clone(), &Route::Home);
                self.stage = Stage::Signed(Signed {
                    session,
                    api,
                    history: vec![Route::Home],
                    view: View::Loading,
                    notice: None,
                });
                task
            }
        }
    }

    fn refuse(&mut self, trouble: Trouble) {
        match &mut self.stage {
            Stage::Login(state) => {
                state.submitting = false;
                state.notice = Some(trouble);
            }
            _ => {
                self.stage = Stage::Login(login::State {
                    notice: Some(trouble),
                    ..login::State::default()
                });
            }
        }
    }

    /// A trouble saying the session is gone returns to the login screen with
    /// the reason under the form; every other trouble is terminal.
    fn lost(&mut self, trouble: Trouble) -> Task<Message> {
        if trouble.session_lost() {
            self.refuse(trouble);
        } else {
            self.stage = Stage::Failed(trouble);
        }
        Task::none()
    }

    /// Records a failure the signed-in view survives: a played or favorite
    /// toggle that was refused, and a logout whose revoke did not happen. A
    /// trouble saying the session is gone returns to the login screen instead.
    fn report(&mut self, trouble: Trouble) {
        if trouble.session_lost() {
            self.refuse(trouble);
            return;
        }
        if let Some(signed) = self.signed() {
            signed.notice = Some(trouble);
        }
    }

    /// Issues one image fetch; empty when no session holds an `Api`.
    fn fetch_image(&self, key: images::Key) -> Task<Message> {
        let Stage::Signed(signed) = &self.stage else {
            return Task::none();
        };
        let api = signed.api.clone();
        let url = api.image_url(key);
        Task::perform(async move { api.image(url).await }, move |result| {
            Message::ImageLoaded(key, result)
        })
    }

    fn navigate(&mut self, route: Route) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        if signed.route() == Some(&route) {
            return Task::none();
        }
        signed.history.push(route.clone());
        signed.view = View::Loading;
        let api = signed.api.clone();
        load(api, &route)
    }

    fn replace(&mut self, route: Route) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        signed.history.pop();
        signed.history.push(route.clone());
        signed.view = View::Loading;
        let api = signed.api.clone();
        load(api, &route)
    }

    fn settle(&mut self) {
        let Some(signed) = self.signed() else {
            return;
        };
        let wanted = signed.wanted_images();
        self.images.retain(&wanted);
    }

    fn fetch_images(&mut self) -> Task<Message> {
        let Some(signed) = self.signed() else {
            return Task::none();
        };
        let wanted = signed.wanted_images();
        let started: Vec<_> = wanted
            .into_iter()
            .filter(|key| self.images.begin(*key))
            .collect();

        Task::batch(started.into_iter().map(|key| self.fetch_image(key)))
    }

    fn loaded(&mut self, view: View) -> Task<Message> {
        if let Some(signed) = self.signed() {
            signed.view = view;
        }
        self.settle();
        self.fetch_images()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ready => {
                boot::hide_static_page();
                Task::none()
            }
            Message::SessionChecked(Ok(status)) | Message::LoggedIn(Ok(status)) => {
                self.enter(status)
            }
            Message::SessionChecked(Err(trouble)) => self.lost(trouble),
            Message::LoginEdited(field, value) => {
                if let Stage::Login(state) = &mut self.stage {
                    state.edit(field, value);
                }
                Task::none()
            }
            Message::LoginSubmitted => {
                let Stage::Login(state) = &mut self.stage else {
                    return Task::none();
                };
                if state.submitting {
                    return Task::none();
                }
                state.submitting = true;
                state.notice = None;
                Task::perform(control::login(state.credentials()), Message::LoggedIn)
            }
            Message::LoggedIn(Err(trouble)) => {
                self.refuse(trouble);
                Task::none()
            }
            Message::LogoutPressed => Task::perform(control::logout(), Message::LoggedOut),
            Message::LoggedOut(Ok(())) => {
                self.images.retain(&HashSet::new());
                self.stage = Stage::Login(login::State::default());
                Task::none()
            }
            Message::LoggedOut(Err(trouble)) => {
                self.report(trouble);
                Task::none()
            }
            Message::Navigated(route) => self.navigate(route),
            Message::WentBack => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                if signed.history.len() < 2 {
                    return Task::none();
                }
                signed.history.pop();
                let route = signed.route().cloned().unwrap_or(Route::Home);
                signed.view = View::Loading;
                let api = signed.api.clone();
                load(api, &route)
            }
            Message::HomeLoaded(Ok(state)) => self.loaded(View::Home(state)),
            Message::LibraryLoaded(Ok(state)) => self.loaded(View::Library(state)),
            Message::DetailLoaded(Ok(state)) => self.loaded(View::Detail(state)),
            Message::SearchLoaded(Ok(state)) => self.loaded(View::Search(state)),
            Message::HomeLoaded(Err(trouble))
            | Message::LibraryLoaded(Err(trouble))
            | Message::DetailLoaded(Err(trouble))
            | Message::SearchLoaded(Err(trouble)) => self.lost(trouble),
            Message::SortSelected(sort) => match self.route() {
                Some(Route::Library { id, .. }) => {
                    self.replace(Route::Library { id, sort, start: 0 })
                }
                _ => Task::none(),
            },
            Message::PageStepped(step) => self.step(step),
            Message::SearchEdited(term) => {
                if let Some(Signed {
                    view: View::Search(state),
                    ..
                }) = self.signed()
                {
                    state.term = term;
                }
                Task::none()
            }
            Message::SearchSubmitted => {
                let term = match self.signed() {
                    Some(Signed {
                        view: View::Search(state),
                        ..
                    }) => state.term.clone(),
                    _ => return Task::none(),
                };
                self.replace(Route::Search { term, start: 0 })
            }
            Message::PlayedToggled(id, played) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(async move { api.set_played(id, played).await }, move |r| {
                    Message::UserDataUpdated(id, r)
                })
            }
            Message::FavoriteToggled(id, favorite) => {
                let Some(signed) = self.signed() else {
                    return Task::none();
                };
                let api = signed.api.clone();
                Task::perform(
                    async move { api.set_favorite(id, favorite).await },
                    move |r| Message::UserDataUpdated(id, r),
                )
            }
            Message::UserDataUpdated(id, Ok(data)) => {
                if let Some(signed) = self.signed() {
                    for item in signed.items_mut() {
                        if item.id == Some(id) {
                            item.user_data = Some(data.clone());
                        }
                    }
                }
                Task::none()
            }
            Message::UserDataUpdated(_, Err(trouble)) => {
                self.report(trouble);
                Task::none()
            }
            Message::ImageWanted(key) => {
                if !self.images.begin(key) {
                    return Task::none();
                }
                self.fetch_image(key)
            }
            Message::ImageLoaded(key, Ok(bytes)) => {
                self.images.store(key, bytes);
                Task::none()
            }
            Message::ImageLoaded(key, Err(_)) => {
                if !self.images.fail(key) {
                    return Task::none();
                }
                if !self.images.begin(key) {
                    return Task::none();
                }
                self.fetch_image(key)
            }
        }
    }

    fn step(&mut self, step: Step) -> Task<Message> {
        let delta = match step {
            Step::Previous => -PAGE_SIZE,
            Step::Next => PAGE_SIZE,
        };
        match self.route() {
            Some(Route::Library { id, sort, start }) => self.replace(Route::Library {
                id,
                sort,
                start: (start + delta).max(0),
            }),
            Some(Route::Search { term, start }) => self.replace(Route::Search {
                term,
                start: (start + delta).max(0),
            }),
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.stage {
            Stage::Booting => center(text(strings::lookup(Text::StatusLoading))).into(),
            Stage::Login(state) => login::view(state),
            Stage::Failed(trouble) => center(widget::notice(trouble.message())).into(),
            Stage::Signed(signed) => {
                let body: Element<'_, Message> = match &signed.view {
                    View::Loading => center(text(strings::lookup(Text::StatusLoading))).into(),
                    View::Home(state) => home::view(state, &self.images),
                    View::Library(state) => library::view(state, &self.images),
                    View::Detail(state) => detail::view(state, &self.images),
                    View::Search(state) => search::view(state, &self.images),
                };
                widget::chrome(
                    &signed.session,
                    signed.history.len() > 1,
                    signed.notice.as_ref(),
                    body,
                )
            }
        }
    }

    pub fn title(&self) -> String {
        strings::lookup(Text::AppName).to_string()
    }

    pub fn theme(&self) -> Theme {
        theme::theme()
    }
}
