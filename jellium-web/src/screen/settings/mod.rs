//! The settings region: one route stack, one navigation column, one save rule.
//! No control here writes on change; only Save writes, so leaving a screen
//! holding edits warns before they are lost.

use std::rc::Rc;

use iced::widget::{button, checkbox, column, row};
use iced::{Element, Task};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::Answer;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;

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
    Profile,
    Password,
    Display,
    Home,
    Playback,
    Subtitles,
    Controls,
    QuickConnect,
}

impl Screen {
    /// Every screen, in the order the column shows them.
    pub const ALL: [Screen; 8] = [
        Screen::Profile,
        Screen::Password,
        Screen::Display,
        Screen::Home,
        Screen::Playback,
        Screen::Subtitles,
        Screen::Controls,
        Screen::QuickConnect,
    ];

    pub fn label(self) -> Text {
        match self {
            Screen::Profile => Text::SettingsProfile,
            Screen::Password => Text::SettingsPassword,
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
}

/// The screens `session` reaches, in column order: profile and password always,
/// the five preference screens under `preference_access`, and Quick Connect
/// while the server reports it enabled.
pub fn column_of(session: &jellium_protocol::Session) -> Vec<Screen> {
    Screen::ALL
        .into_iter()
        .filter(|screen| match screen {
            Screen::QuickConnect => session.quick_connect,
            screen if screen.preference_access() => session.preference_access,
            _ => true,
        })
        .collect()
}

/// What the shown screen holds.
#[derive(Debug)]
pub enum Body {
    Profile(Box<profile::State>),
    Password(password::State),
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
    Profile(Box<profile::State>),
    Password,
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
                Loaded::Profile(state) => Body::Profile(state),
                Loaded::Password => Body::Password(password::State::default()),
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
    /// Sets one preference of the bag the session holds.
    Set(Setting),
    /// Writes what the shown screen edits, which is the only thing that writes.
    Save,
    /// Types into the field the shown screen offers: the display name, the two
    /// passwords, the Quick Connect code, and a confirmation's name.
    Typed(String),
    TypedCurrentPassword(String),
    TypedNewPassword(String),
    /// Moves one home library, and hides or shows it.
    MoveLibrary {
        id: Uuid,
        down: bool,
    },
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
            Screen::Profile => Loaded::Profile(Box::new(profile::load(api, user).await.bubbled()?)),
            Screen::Password => Loaded::Password,
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

/// The one save control, which is the only control on a settings screen that
/// writes.
pub fn save<'a>() -> Element<'a, Message> {
    button(prose(
        strings::lookup(Text::SettingsSave).to_owned(),
        typeface::BODY,
    ))
    .on_press(Message::SettingsAction(Action::Save))
    .into()
}

/// One row of choices, the one held drawn without a press, so no control on a
/// settings screen writes on change.
pub fn choices<'a, T: Copy + PartialEq + 'static>(
    label: Text,
    offered: &'static [T],
    held: T,
    naming: impl Fn(T) -> String,
    setting: impl Fn(T) -> Setting,
) -> Element<'a, Message> {
    let mut controls = row![].spacing(theme::CARD_SPACING);
    for offer in offered.iter().copied() {
        let mut control = button(prose(naming(offer), typeface::BODY));
        if offer != held {
            control = control.on_press(Message::SettingsAction(Action::Set(setting(offer))));
        }
        controls = controls.push(control);
    }
    column![
        prose(strings::lookup(label).to_owned(), typeface::BODY),
        controls
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}

/// One flag of the user configuration, labelled by this region's own text
/// rather than by its json key.
pub fn flag<'a>(
    label: Text,
    field: jellium_model::form::Field,
    configuration: &jellium_model::form::Form,
) -> Element<'a, Message> {
    row![
        checkbox(configuration.value(field) == "true").on_toggle(move |on| {
            Message::SettingsAction(Action::Edited(field, on.to_string()))
        }),
        prose(strings::lookup(label).to_owned(), typeface::BODY),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center)
    .into()
}

/// One preference held as a flag in the bag rather than the configuration.
pub fn toggle<'a>(
    label: Text,
    held: bool,
    setting: impl Fn(bool) -> Setting + 'static,
) -> Element<'a, Message> {
    row![
        checkbox(held).on_toggle(move |on| Message::SettingsAction(Action::Set(setting(on)))),
        prose(strings::lookup(label).to_owned(), typeface::BODY),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center)
    .into()
}

fn mode_label(option: &str) -> Text {
    match option {
        "Always" => Text::PlaybackSubtitleModeAlways,
        "OnlyForced" => Text::PlaybackSubtitleModeOnlyForced,
        "None" => Text::PlaybackSubtitleModeNone,
        "Smart" => Text::PlaybackSubtitleModeSmart,
        _ => Text::PlaybackSubtitleModeDefault,
    }
}

/// One `Field::Choice` of the user configuration, its options named by this
/// region's own text.
pub fn choice<'a>(
    label: Text,
    field: jellium_model::form::Field,
    configuration: &jellium_model::form::Form,
) -> Element<'a, Message> {
    let jellium_model::form::Field::Choice { options, .. } = field else {
        return prose(strings::lookup(label).to_owned(), typeface::BODY);
    };
    let held = configuration.value(field);
    let mut controls = row![].spacing(theme::CARD_SPACING);
    for option in options {
        let mut control = button(prose(
            strings::lookup(mode_label(option)).to_owned(),
            typeface::BODY,
        ));
        if *option != held {
            control = control.on_press(Message::SettingsAction(Action::Edited(
                field,
                (*option).to_owned(),
            )));
        }
        controls = controls.push(control);
    }
    column![
        prose(strings::lookup(label).to_owned(), typeface::BODY),
        controls
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}

/// One `Field::Listed` of the user configuration, offered against the list the
/// screen supplies at runtime.
pub fn listed<'a>(
    label: Text,
    field: jellium_model::form::Field,
    configuration: &jellium_model::form::Form,
    cultures: &[jellyfin_api::types::CultureDto],
) -> Element<'a, Message> {
    let held = configuration.value(field);
    let mut controls = row![].spacing(theme::CARD_SPACING);
    let mut any = button(prose(
        strings::lookup(Text::PlaybackLanguageAny).to_owned(),
        typeface::BODY,
    ));
    if !held.is_empty() {
        any = any.on_press(Message::SettingsAction(Action::Edited(
            field,
            String::new(),
        )));
    }
    controls = controls.push(any);
    for culture in cultures {
        let Some(code) = culture.three_letter_iso_language_name.clone() else {
            continue;
        };
        let mut control = button(prose(
            culture.name.clone().unwrap_or_else(|| code.clone()),
            typeface::BODY,
        ));
        if code != held {
            control = control.on_press(Message::SettingsAction(Action::Edited(field, code)));
        }
        controls = controls.push(control);
    }
    column![
        prose(strings::lookup(label).to_owned(), typeface::BODY),
        controls
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}

/// The navigation column beside the screen shown, the read-only indicator above
/// both, and the confirmation in the acting control's place.
pub fn view<'a>(
    state: &'a State,
    signed: &'a Signed,
    images: &'a crate::images::Cache,
) -> Element<'a, Message> {
    let read_only = signed.session.read_only;

    let mut nav = column![prose(
        strings::lookup(Text::SettingsTitle).to_owned(),
        typeface::HEADING_3
    )]
    .spacing(theme::CARD_SPACING);
    for screen in column_of(&signed.session) {
        let mut control = button(prose(
            strings::lookup(screen.label()).to_owned(),
            typeface::BODY,
        ));
        if screen != state.screen {
            control = control.on_press(Message::SettingsAction(Action::Open(screen)));
        }
        nav = nav.push(control);
    }

    let body: Element<'a, Message> = match &state.body {
        Body::Profile(profile) => profile::view(profile, read_only, images),
        Body::Password(password) => password::view(password, read_only),
        Body::Display => display::view(&signed.configuration, read_only),
        Body::Home(home) => home::view(home, signed.held, &signed.configuration, read_only),
        Body::Playback(playback) => playback::view(
            playback,
            signed.held,
            &signed.configuration,
            signed.session.sync_play,
            read_only,
        ),
        Body::Subtitles => subtitles::view(signed.held, read_only),
        Body::Controls => controls::view(),
        Body::QuickConnect(quick) => quickconnect::view(quick, read_only),
    };

    let shown: Element<'a, Message> = match &state.confirming {
        Some(pending) => {
            crate::screen::confirm::view(pending, crate::screen::confirm::Region::Settings)
        }
        None => body,
    };

    let mut page = column![].spacing(theme::CARD_SPACING);
    if signed.server_changed {
        page = page.push(prose(
            strings::lookup(Text::SettingsServerChanged).to_owned(),
            typeface::BODY,
        ));
    }
    page = page.push(row![nav, shown].spacing(theme::CARD_SPACING));

    iced::widget::container(page)
        .padding(theme::CARD_SPACING)
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
            if let crate::app::View::Settings(State {
                body: Body::Password(password),
                ..
            }) = &mut signed.view
            {
                password.current = typed;
            }
            Task::none()
        }
        Action::TypedNewPassword(typed) => {
            if let crate::app::View::Settings(State {
                body: Body::Password(password),
                ..
            }) = &mut signed.view
            {
                password.replacement = typed;
            }
            Task::none()
        }
        Action::MoveLibrary { id, down } => {
            let Some(libraries) = library_ids(signed) else {
                return Task::none();
            };
            let order =
                jellium_model::user::ids(&signed.configuration, jellium_model::user::ORDERED_VIEWS);
            let moved = jellium_model::user::moved(&libraries, &order, id, down);
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
            let crate::app::View::Settings(State {
                body: Body::Profile(profile),
                ..
            }) = &signed.view
            else {
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
            let crate::app::View::Settings(State {
                body: Body::Password(password),
                ..
            }) = &signed.view
            else {
                return Task::none();
            };
            let api = signed.api.clone();
            let id = signed.session.user_id;
            let current = password.current.clone();
            let replacement = password.replacement.clone();
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
    let crate::app::View::Settings(State {
        body: Body::Home(home),
        ..
    }) = &signed.view
    else {
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
