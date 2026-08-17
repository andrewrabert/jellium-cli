//! The settings region: one route stack, one menu page, one save rule.
//! No control here writes on change; only Save writes, so leaving a screen
//! holding edits warns before they are lost.

use std::rc::Rc;

use iced::widget::{column, container};
use iced::{Element, Task};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::Answer;
use crate::icon::Icon;
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};

pub mod controls;
pub mod display;
pub mod home;
pub mod password;
pub mod playback;
pub mod profile;
pub mod quickconnect;
pub mod subtitles;

/// One settings screen, and everything that addresses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// The page the region opens on, which lists the screens and shows none of
    /// them.
    Menu,
    Profile,
    Display,
    Home,
    Playback,
    Subtitles,
    Controls,
    QuickConnect,
}

impl Screen {
    /// Every screen the menu lists, in the order the reference lists them.
    // reference: settings-menu-rows
    pub const ALL: [Screen; 7] = [
        Screen::Profile,
        Screen::QuickConnect,
        Screen::Display,
        Screen::Home,
        Screen::Playback,
        Screen::Subtitles,
        Screen::Controls,
    ];

    pub fn label(self) -> Text {
        match self {
            Screen::Menu => Text::SettingsTitle,
            Screen::Profile => Text::SettingsProfile,
            Screen::Display => Text::SettingsDisplay,
            Screen::Home => Text::SettingsHome,
            Screen::Playback => Text::SettingsPlayback,
            Screen::Subtitles => Text::SettingsSubtitles,
            Screen::Controls => Text::SettingsControls,
            Screen::QuickConnect => Text::SettingsQuickConnect,
        }
    }

    /// True for a screen only a user whose policy carries
    /// `EnableUserPreferenceAccess` reaches.
    pub fn preference_access(self) -> bool {
        matches!(
            self,
            Screen::Display
                | Screen::Home
                | Screen::Playback
                | Screen::Subtitles
                | Screen::Controls
        )
    }

    // the reference's own form ends with the block submit control on these
    // four pages and on no other
    // reference: settings-display-save
    // reference: settings-home-save
    // reference: settings-playback-save
    // reference: settings-subtitles-save
    pub fn saves(self) -> bool {
        matches!(
            self,
            Screen::Display | Screen::Home | Screen::Playback | Screen::Subtitles
        )
    }
}

/// The glyph the reference's own menu puts on the row that opens `screen`, and
/// none for the menu itself, which is no row of it.
// reference: settings-menu-rows
fn glyph(screen: Screen) -> Option<Icon> {
    match screen {
        Screen::Menu => None,
        Screen::Profile => Some(Icon::Person),
        Screen::QuickConnect => Some(Icon::PhonelinkLock),
        Screen::Display => Some(Icon::Tv),
        Screen::Home => Some(Icon::Home),
        Screen::Playback => Some(Icon::PlayCircleFilled),
        Screen::Subtitles => Some(Icon::ClosedCaption),
        Screen::Controls => Some(Icon::Keyboard),
    }
}

/// Every settings screen `session` reaches, in the order the reference's own
/// menu lists them.
// reference: settings-menu-rows
pub fn reached(session: &jellium_protocol::Session) -> impl Iterator<Item = Screen> {
    Screen::ALL.into_iter().filter(|screen| match screen {
        Screen::QuickConnect => session.quick_connect,
        screen if screen.preference_access() => session.preference_access,
        _ => true,
    })
}

/// Whether `session` reaches `screen`; the menu itself always is.
pub fn reaches(session: &jellium_protocol::Session, screen: Screen) -> bool {
    screen == Screen::Menu || reached(session).any(|held| held == screen)
}

/// What the shown screen holds.
#[derive(Debug)]
pub enum Body {
    Menu,
    Profile(Box<profile::State>),
    Display,
    Home(Box<home::State>),
    Playback(Box<playback::State>),
    Subtitles,
    Controls,
    QuickConnect(quickconnect::State),
}

/// What one settings screen answered with.
#[derive(Debug, Clone)]
pub enum Loaded {
    Menu,
    Profile(Box<profile::State>),
    Display,
    Home(Box<home::State>),
    Playback(Box<playback::State>),
    Subtitles,
    Controls,
    QuickConnect,
}

#[derive(Debug)]
pub struct State {
    pub screen: Screen,
    pub body: Body,
    /// The action awaiting its confirmation.
    pub confirming: Option<crate::screen::confirm::Pending>,
}

impl State {
    pub fn of(screen: Screen, loaded: Loaded) -> State {
        State {
            screen,
            body: match loaded {
                Loaded::Menu => Body::Menu,
                Loaded::Profile(state) => Body::Profile(state),
                Loaded::Display => Body::Display,
                Loaded::Home(state) => Body::Home(state),
                Loaded::Playback(state) => Body::Playback(state),
                Loaded::Subtitles => Body::Subtitles,
                Loaded::Controls => Body::Controls,
                Loaded::QuickConnect => Body::QuickConnect(quickconnect::State::default()),
            },
            confirming: None,
        }
    }
}

/// One preference a control sets, applied to the bag the session holds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Setting {
    Quality(jellium_protocol::Quality),
    SkipBack(i64),
    SkipForward(i64),
    SubtitleSize(jellium_model::prefs::SubtitleSize),
    SubtitleColour(jellium_model::prefs::SubtitleColour),
    SubtitleBackground(jellium_model::prefs::SubtitleColour),
    SubtitleOpacity(i32),
    SubtitleShadow(jellium_model::prefs::SubtitleShadow),
    ContinueWatchingRow(bool),
    NextUpRow(bool),
    SyncExtraOffset(i64),
    SyncMethod(jellium_protocol::sync::SyncMethod),
    SyncRateAttempts(u32),
    SyncSeekAttempts(u32),
}

impl Setting {
    /// `held` with this one preference set, which is what an edit records.
    fn applied(self, held: jellium_model::prefs::Held) -> jellium_model::prefs::Held {
        let mut held = held;
        match self {
            Setting::Quality(quality) => held.quality = quality,
            Setting::SkipBack(seconds) => held.skip_back_seconds = seconds,
            Setting::SkipForward(seconds) => held.skip_forward_seconds = seconds,
            Setting::SubtitleSize(size) => held.subtitle_size = size,
            Setting::SubtitleColour(colour) => held.subtitle_colour = colour,
            Setting::SubtitleBackground(colour) => held.subtitle_background = colour,
            Setting::SubtitleOpacity(opacity) => held.subtitle_opacity = opacity,
            Setting::SubtitleShadow(shadow) => held.subtitle_shadow = shadow,
            Setting::ContinueWatchingRow(on) => held.continue_watching = on,
            Setting::NextUpRow(on) => held.next_up = on,
            Setting::SyncExtraOffset(ms) => held.sync.extra_offset_ms = ms,
            Setting::SyncMethod(method) => held.sync.method = method,
            Setting::SyncRateAttempts(attempts) => held.sync.rate_attempts = attempts,
            Setting::SyncSeekAttempts(attempts) => held.sync.seek_attempts = attempts,
        }
        held
    }
}

/// Every control the settings region resolves to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Open(Screen),
    /// Edits one field of the user configuration the session holds.
    Edited(jellium_model::form::Field, String),
    /// Sets one flag of the user configuration the session holds.
    Flagged(jellium_model::form::Field, bool),
    /// Sets one preference of the bag the session holds.
    Set(Setting),
    /// Writes what the shown screen edits, which is the only thing that writes.
    Save,
    /// Types into the field the shown screen offers: the display name, the two
    /// passwords, the Quick Connect code, and a confirmation's name.
    Typed(String),
    TypedCurrentPassword(String),
    TypedNewPassword(String),
    /// Moves one library in the home screen's own order.
    MoveLibrary {
        id: Uuid,
        toward: jellium_model::user::Toward,
    },
    /// Hides one home library, and shows it.
    HideLibrary {
        id: Uuid,
        hidden: bool,
    },
    /// Opens the file input's picker.
    ChooseImage,
    /// Writes the typed display name.
    SaveName,
    /// Changes the signed-in user's own password.
    ChangePassword,
    Ask(crate::screen::confirm::Pending),
    Close,
    Confirm,
}

/// Loads the screen `screen` names for `user`, whose preference bag is read
/// under `client`.
pub async fn load(
    api: Rc<Api>,
    screen: Screen,
    user: Uuid,
    client: String,
) -> Answer<(Screen, Loaded)> {
    Answer::of(async {
        drop(client);
        let loaded = match screen {
            Screen::Menu => Loaded::Menu,
            Screen::Profile => Loaded::Profile(Box::new(profile::load(api, user).await.bubbled()?)),
            Screen::Display => Loaded::Display,
            Screen::Home => Loaded::Home(Box::new(home::load(api).await.bubbled()?)),
            Screen::Playback => Loaded::Playback(Box::new(playback::load(api).await.bubbled()?)),
            Screen::Subtitles => Loaded::Subtitles,
            Screen::Controls => Loaded::Controls,
            Screen::QuickConnect => Loaded::QuickConnect,
        };
        Ok((screen, loaded))
    })
    .await
}

/// The options one dropdown offers: each value under the name this region
/// gives it, carrying the control that chooses it.
pub fn choices<T: Clone>(
    values: impl IntoIterator<Item = T>,
    naming: impl Fn(T) -> String,
    chooses: impl Fn(T) -> Action,
) -> Vec<widget::Choice<Action>> {
    values
        .into_iter()
        .map(|value| widget::Choice {
            label: naming(value.clone()),
            value: chooses(value),
        })
        .collect()
}

/// The signed-in user's own name over the rows that open every screen they
/// reach, which is the one section the menu page draws.
// reference: settings-menu
// reference: settings-menu-rows
fn menu<'a>(signed: &'a Signed) -> Element<'a, Message> {
    column![
        container(prose(signed.session.user_name.clone(), typeface::HEADING_2))
            .padding(iced::Padding::ZERO.left(style::drawn(space::SECTION_TITLE_INSET.drawn()))),
        widget::list::listed(
            space::ListRow::glyph(space::Lines::One),
            reached(&signed.session).filter_map(|screen| {
                Some(widget::list::Row {
                    face: Some(widget::list::Face::Glyph(glyph(screen)?)),
                    index: None,
                    title: strings::lookup(screen.label()).into(),
                    secondary: Vec::new(),
                    press: widget::list::Press::Whole(Message::SettingsAction(Action::Open(
                        screen,
                    ))),
                    controls: Vec::new(),
                })
            }),
        ),
    ]
    .spacing(style::drawn(space::SECTION_GAP.drawn()))
    .into()
}

/// The shown screen's sections, or the menu's own, the save control under a
/// screen that carries one and the confirmation in their place, every one of
/// them held to the form's own width, under the read-only indicator and inside
/// the page's padding.
pub fn view<'a>(
    state: &'a State,
    signed: &'a Signed,
    images: &'a crate::images::Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    let read_only = signed.session.read_only;

    let mut sections: Vec<Element<'a, Message>> = match &state.body {
        Body::Menu => vec![menu(signed)],
        Body::Profile(profile) => profile::sections(profile, read_only, images),
        Body::Display => display::sections(&signed.configuration),
        Body::Home(home) => home::sections(home, signed.held, &signed.configuration),
        Body::Playback(playback) => playback::sections(
            playback,
            signed.held,
            &signed.configuration,
            signed.session.sync_play,
        ),
        Body::Subtitles => subtitles::sections(signed.held),
        Body::Controls => controls::sections(),
        Body::QuickConnect(quick) => quickconnect::sections(quick, read_only),
    };

    if state.screen.saves() && !read_only {
        sections.push(widget::block(
            strings::lookup(Text::SettingsSave),
            Some(Message::SettingsAction(Action::Save)),
            widget::Emphasis::Submit,
        ));
    }

    let rows: Vec<Element<'a, Message>> = match &state.confirming {
        Some(pending) => vec![crate::screen::confirm::view(
            pending,
            crate::screen::confirm::Region::Settings,
        )],
        None => sections,
    };

    let mut page = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if signed.server_changed {
        page = page.push(prose(
            strings::lookup(Text::SettingsServerChanged),
            typeface::BODY,
        ));
    }
    page = page.push(widget::capped(
        viewport,
        space::section_bottom(viewport.band()),
        rows,
    ));

    container(page)
        .padding(style::padding(space::PAGE_PAD))
        .into()
}

/// The bag with one preference set, taken back into the session.
fn set(signed: &mut Signed, setting: Setting) {
    let held = setting.applied(signed.held);
    signed.preferences.edit(held);
    signed.held = signed.preferences.held();
}

/// Applies a control.
/// A write the mode forecloses is never reachable, because `--read-only` leaves
/// its control out of the view.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    match action {
        Action::Open(screen) => {
            Task::done(Message::Navigated(crate::route::Route::Settings { screen }))
        }
        Action::Edited(field, value) => {
            signed.configuration.edit(field, value);
            Task::none()
        }
        Action::Flagged(field, on) => {
            signed.configuration.flag(field, on);
            Task::none()
        }
        Action::Set(setting) => {
            set(signed, setting);
            Task::none()
        }
        Action::Save => saving(signed),
        Action::Typed(typed) => {
            let crate::app::View::Settings(state) = &mut signed.view else {
                return Task::none();
            };
            if let Some(pending) = state.confirming.as_mut() {
                pending.typed = typed;
                return Task::none();
            }
            match &mut state.body {
                Body::Profile(profile) => profile.naming = typed,
                Body::QuickConnect(quick) => quick.code = typed,
                _ => {}
            }
            Task::none()
        }
        Action::TypedCurrentPassword(typed) => {
            if let crate::app::View::Settings(state) = &mut signed.view
                && let Body::Profile(profile) = &mut state.body
            {
                profile.password.current = typed;
            }
            Task::none()
        }
        Action::TypedNewPassword(typed) => {
            if let crate::app::View::Settings(state) = &mut signed.view
                && let Body::Profile(profile) = &mut state.body
            {
                profile.password.replacement = typed;
            }
            Task::none()
        }
        Action::MoveLibrary { id, toward } => {
            let Some(libraries) = library_ids(signed) else {
                return Task::none();
            };
            let order =
                jellium_model::user::ids(&signed.configuration, jellium_model::user::ORDERED_VIEWS);
            let moved = jellium_model::user::moved(&libraries, &order, id, toward);
            signed.configuration.edit(
                jellium_model::user::ORDERED_VIEWS,
                moved
                    .iter()
                    .map(Uuid::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            Task::none()
        }
        Action::HideLibrary { id, hidden } => {
            let mut excluded = jellium_model::user::ids(
                &signed.configuration,
                jellium_model::user::MY_MEDIA_EXCLUDES,
            );
            excluded.retain(|held| *held != id);
            if hidden {
                excluded.push(id);
            }
            signed.configuration.edit(
                jellium_model::user::MY_MEDIA_EXCLUDES,
                excluded
                    .iter()
                    .map(Uuid::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            Task::none()
        }
        Action::ChooseImage => {
            if let Some(picker) = signed.picker.as_ref() {
                picker.choose();
            }
            Task::none()
        }
        Action::SaveName => {
            let crate::app::View::Settings(state) = &signed.view else {
                return Task::none();
            };
            let Body::Profile(profile) = &state.body else {
                return Task::none();
            };
            let api = signed.api.clone();
            let id = profile.id;
            let body = profile::renamed(profile);
            let wrote = crate::error::Wrote {
                operation: crate::error::Operation::UserName,
                object: profile.naming.clone(),
            };
            Task::perform(
                async move { api.save_user(id, &body).await },
                move |result| Message::SettingsWrote(wrote.clone(), result),
            )
        }
        Action::ChangePassword => {
            let crate::app::View::Settings(state) = &signed.view else {
                return Task::none();
            };
            let Body::Profile(profile) = &state.body else {
                return Task::none();
            };
            let current = profile.password.current.clone();
            let replacement = profile.password.replacement.clone();
            let api = signed.api.clone();
            let id = signed.session.user_id;
            let wrote = crate::error::Wrote {
                operation: crate::error::Operation::UserPassword,
                object: signed.session.user_name.clone(),
            };
            Task::perform(
                async move { api.set_password(id, Some(&current), &replacement).await },
                move |result| Message::SettingsWrote(wrote.clone(), result),
            )
        }
        Action::Ask(pending) => {
            if let crate::app::View::Settings(state) = &mut signed.view {
                state.confirming = Some(pending);
            }
            Task::none()
        }
        Action::Close => {
            if let crate::app::View::Settings(state) = &mut signed.view {
                state.confirming = None;
            }
            Task::none()
        }
        Action::Confirm => confirmed(signed),
    }
}

fn library_ids(signed: &Signed) -> Option<Vec<Uuid>> {
    let crate::app::View::Settings(state) = &signed.view else {
        return None;
    };
    let Body::Home(home) = &state.body else {
        return None;
    };
    Some(home.libraries.iter().filter_map(|it| it.id).collect())
}

fn confirmed(signed: &mut Signed) -> Task<Message> {
    let crate::app::View::Settings(state) = &mut signed.view else {
        return Task::none();
    };
    let Some(pending) = state
        .confirming
        .take()
        .filter(crate::screen::confirm::Pending::ready)
    else {
        return Task::none();
    };
    match pending.action {
        crate::screen::confirm::Destructive::AuthorizeQuickConnect { code } => {
            if !jellium_model::quickconnect::shaped(&code) {
                crate::failure::raise(crate::error::told(Text::FailureQuickConnectShape));
                return Task::none();
            }
            let authorized_here = match &state.body {
                Body::QuickConnect(quick) => quick.authorized.contains(&code),
                _ => false,
            };
            let api = signed.api.clone();
            Task::perform(
                async move { api.authorize_quick_connect(&code, authorized_here).await },
                Message::QuickConnected,
            )
        }
        crate::screen::confirm::Destructive::RemoveUserImage { id } => {
            let api = signed.api.clone();
            let wrote = crate::error::Wrote {
                operation: crate::error::Operation::UserImage,
                object: signed.session.user_name.clone(),
            };
            Task::perform(
                async move { api.remove_user_image(id).await },
                move |result| Message::SettingsWrote(wrote.clone(), result),
            )
        }
        _ => Task::none(),
    }
}

/// The one write a Save makes, named by the screen it was made from.
fn saving(signed: &mut Signed) -> Task<Message> {
    let crate::app::View::Settings(state) = &signed.view else {
        return Task::none();
    };
    let object = strings::lookup(state.screen.label()).to_owned();
    let mut tasks = Vec::new();

    if signed.configuration.dirty() {
        let api = signed.api.clone();
        let id = signed.session.user_id;
        let body = signed.configuration.written();
        let wrote = crate::error::Wrote {
            operation: crate::error::Operation::UserConfiguration,
            object: object.clone(),
        };
        tasks.push(Task::perform(
            async move { api.save_user_configuration(id, &body).await },
            move |result| Message::SettingsSaved(wrote.clone(), result),
        ));
    }

    if signed.preferences.dirty() {
        let api = signed.api.clone();
        let client = signed.session.client.clone();
        let record = signed.preferences.written();
        let wrote = crate::error::Wrote {
            operation: crate::error::Operation::Preferences,
            object,
        };
        tasks.push(Task::perform(
            async move { api.save_preferences(&client, &record).await },
            move |result| Message::SettingsSaved(wrote.clone(), result),
        ));
    }

    Task::batch(tasks)
}

/// Takes the file the input reported: a type outside `upload::TYPES` and a file
/// over `upload::LIMIT` are refused here, naming the type and naming the size
/// and the cap, and neither is sent.
pub fn chosen(signed: &mut Signed, chosen: &crate::overlay::Chosen) -> Task<Message> {
    let refused = jellium_model::upload::refused(&chosen.mime, chosen.size);
    if let Some(refused) = &refused {
        crate::failure::raise(crate::error::upload_refused(refused));
        return Task::none();
    }
    let api = signed.api.clone();
    let id = signed.session.user_id;
    let mime = chosen.mime.clone();
    let bytes = chosen.bytes();
    let wrote = crate::error::Wrote {
        operation: crate::error::Operation::UserImage,
        object: signed.session.user_name.clone(),
    };
    Task::perform(
        async move { api.upload_user_image(id, &mime, bytes).await },
        move |result| Message::SettingsWrote(wrote.clone(), result),
    )
}

/// True while a settings screen is shown; whether it holds edits a save has not
/// taken is what the session's bag and configuration answer, because that is
/// where a settings screen's edits live.
pub fn dirty(view: &crate::app::View) -> bool {
    matches!(view, crate::app::View::Settings(_))
}

/// Takes the values a save wrote as read, clearing the edits.
pub fn saved(signed: &mut Signed) {
    let record = signed.preferences.written();
    signed.preferences.saved(record);
    let configuration = signed.configuration.written();
    signed.configuration.saved(configuration);
    signed.held = signed.preferences.held();
    crate::prefs::Device::settle(jellium_model::prefs::Parked::of(
        &signed.preferences,
        signed.session.read_only,
    ));
}

/// Discards the edits the shown screen holds, which is what leaving anyway
/// does.
pub fn abandoned(signed: &mut Signed) {
    signed.preferences.discard();
    signed.configuration.discard();
    signed.held = signed.preferences.held();
}
