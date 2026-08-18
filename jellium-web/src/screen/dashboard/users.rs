//! The users the server holds, and the one user a user screen edits.

use iced::Element;
use iced::widget::{button, column, text_input};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
use jellium_model::form::{Field, Form};

/// Every user on the server, what a new one is being named, and the account
/// whose card menu is open.
#[derive(Debug, Clone)]
pub struct State {
    pub users: Vec<jellyfin_api::types::UserDto>,
    /// The name and password typed for a new user.
    pub naming: String,
    pub password: String,
    pub menu: Option<Uuid>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            users: api.users().await.bubbled()?,
            naming: String::new(),
            password: String::new(),
            menu: None,
        })
    })
    .await
}

/// One user's screen: its four panels, its policy and its configuration, each
/// read whole and written whole.
#[derive(Debug, Clone)]
pub struct One {
    pub id: Uuid,
    pub name: String,
    pub tab: super::UserTab,
    pub policy: Form,
    pub configuration: Form,
    /// What has been typed into the two password fields.
    pub current: String,
    pub replacement: String,
}

/// The fields a user's policy exposes; every key outside them survives a save.
pub const POLICY: &[Field] = &[
    Field::Flag {
        key: "IsAdministrator",
    },
    Field::Flag { key: "IsDisabled" },
    Field::Flag { key: "IsHidden" },
    Field::Flag {
        key: "EnableAllFolders",
    },
    Field::Lines {
        key: "EnabledFolders",
    },
    Field::Flag {
        key: "EnableAllDevices",
    },
    Field::Lines {
        key: "EnabledDevices",
    },
    Field::Flag {
        key: "EnableRemoteAccess",
    },
    Field::Flag {
        key: "EnableMediaPlayback",
    },
    Field::Flag {
        key: "EnableAudioPlaybackTranscoding",
    },
    Field::Flag {
        key: "EnableVideoPlaybackTranscoding",
    },
    Field::Flag {
        key: "EnablePlaybackRemuxing",
    },
    Field::Flag {
        key: "EnableContentDeletion",
    },
    Field::Flag {
        key: "EnableContentDownloading",
    },
    Field::Flag {
        key: "EnableLiveTvAccess",
    },
    Field::Flag {
        key: "EnableLiveTvManagement",
    },
    Field::Number {
        key: "MaxParentalRating",
    },
    Field::Lines { key: "BlockedTags" },
    Field::Lines { key: "AllowedTags" },
    Field::Number {
        key: "LoginAttemptsBeforeLockout",
    },
    Field::Number {
        key: "MaxActiveSessions",
    },
    Field::Number {
        key: "RemoteClientBitrateLimit",
    },
    Field::Choice {
        key: "SyncPlayAccess",
        options: &["CreateAndJoinGroups", "JoinGroups", "None"],
    },
];

/// The fields a user's own configuration exposes.
pub const CONFIGURATION: &[Field] = &[
    Field::Text {
        key: "AudioLanguagePreference",
    },
    Field::Text {
        key: "SubtitleLanguagePreference",
    },
    Field::Flag {
        key: "PlayDefaultAudioTrack",
    },
    Field::Flag {
        key: "DisplayMissingEpisodes",
    },
    Field::Choice {
        key: "SubtitleMode",
        options: &["Default", "Always", "OnlyForced", "None", "Smart"],
    },
    Field::Flag {
        key: "EnableNextEpisodeAutoPlay",
    },
    Field::Flag {
        key: "RememberAudioSelections",
    },
    Field::Flag {
        key: "RememberSubtitleSelections",
    },
];

pub async fn open(api: std::rc::Rc<crate::api::Api>, id: Uuid, tab: super::UserTab) -> Answer<One> {
    Answer::of(async {
        let user = api.user(id).await.bubbled()?;
        Ok(One {
            id,
            name: user.name.clone().unwrap_or_default(),
            tab,
            policy: Form::of(api.policy(id).await.bubbled()?),
            configuration: Form::of(api.user_configuration(id).await.bubbled()?),
            current: String::new(),
            replacement: String::new(),
        })
    })
    .await
}

/// The form that creates a user: its name, its password and the control.
pub fn new<'a>(state: &'a State) -> Vec<Element<'a, Message>> {
    vec![
        text_input(strings::lookup(Text::UsersName), &state.naming)
            .style(style::input)
            .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed)))
            .into(),
        text_input(strings::lookup(Text::UsersPassword), &state.password)
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::DashboardAction(super::Action::TypedPassword(typed)))
            .into(),
        button(prose(strings::lookup(Text::UsersCreate), typeface::BODY))
            .style(style::submit)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::CreateUser {
                    name: state.naming.clone(),
                    password: state.password.clone(),
                },
            )))
            .into(),
    ]
}

/// The image key one account's card draws.
// reference: user-card
fn image_key(id: Uuid) -> crate::images::Key {
    crate::images::Key {
        item: id,
        kind: crate::images::Kind::User,
        index: None,
    }
}

/// The image every account with one draws on its card.
// reference: user-card
pub fn images(state: &State) -> std::collections::HashSet<crate::images::Key> {
    state
        .users
        .iter()
        .filter(|user| user.primary_image_tag.is_some())
        .filter_map(|user| user.id)
        .map(image_key)
        .collect()
}

/// The four commands `showUserMenu` offers for one account: the three screens
/// it reaches, each behind the sheet's own glyph, and the deletion, which is
/// absent under read-only.
// the reference floats these rows over the card in an action sheet; here they
// stand in the page under the title
// reference: user-menu
fn menu<'a>(state: &'a State, open: Uuid, read_only: bool) -> Element<'a, Message> {
    let name = state
        .users
        .iter()
        .find(|user| user.id == Some(open))
        .and_then(|user| user.name.clone())
        .unwrap_or_default();
    let reaches = |glyph, label: Text, tab| widget::list::Row {
        face: Some(widget::list::Face::Glyph(glyph)),
        index: None,
        title: strings::lookup(label).into(),
        secondary: Vec::new(),
        press: widget::list::Press::Whole(Message::DashboardAction(super::Action::Open(
            super::Screen::User { id: open, tab },
        ))),
        controls: Vec::new(),
    };
    let mut rows = vec![
        reaches(
            crate::icon::Icon::ModeEdit,
            Text::UsersProfile,
            super::UserTab::Profile,
        ),
        reaches(
            crate::icon::Icon::Lock,
            Text::UsersAccess,
            super::UserTab::Access,
        ),
        reaches(
            crate::icon::Icon::Person,
            Text::UsersParental,
            super::UserTab::Parental,
        ),
    ];
    if !read_only {
        rows.push(widget::list::Row {
            face: Some(widget::list::Face::Glyph(crate::icon::Icon::Delete)),
            index: None,
            title: strings::lookup(Text::UsersDelete).into(),
            secondary: Vec::new(),
            press: widget::list::Press::Whole(Message::DashboardAction(super::Action::Ask(
                crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::DeleteUser { id: open },
                    name,
                ),
            ))),
            controls: Vec::new(),
        });
    }
    widget::list::listed(space::ListRow::glyph(space::Lines::One), rows)
}

/// The section title with the control that adds an account, the commands the
/// open card's menu offers, and the accounts as `UserCardBox` draws them.
// reference: users-grid
pub fn view<'a>(
    state: &'a State,
    read_only: bool,
    images: &'a crate::images::Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    let room = space::Room::dashboard(viewport);
    let adds = match read_only {
        true => None,
        false => Some(widget::fab(
            crate::icon::Icon::Add,
            Text::UsersCreate,
            Message::DashboardAction(super::Action::Open(super::Screen::UserNew)),
        )),
    };

    let mut page: Vec<Element<'a, Message>> =
        vec![widget::titled(strings::lookup(Text::UsersTitle), adds)];
    if let Some(open) = state.menu {
        page.push(menu(state, open, read_only));
    }

    let cards = state.users.iter().filter_map(|user| {
        let id = user.id?;
        Some(widget::user_card(
            room,
            user.name.clone().unwrap_or_default(),
            user.last_activity_date
                .map(|at| strings::format(Text::UsersLastSeen, &[&widget::table::stamped(at)])),
            user.primary_image_tag
                .as_ref()
                .and_then(|_| images.handle(image_key(id))),
            Message::DashboardAction(super::Action::Open(super::Screen::User {
                id,
                tab: super::UserTab::Profile,
            })),
            Message::DashboardAction(super::Action::UserMenu(Some(id))),
        ))
    });
    page.push(widget::wall(
        card::Card::USER,
        room,
        card::Wrap::Leading,
        cards,
    ));

    widget::scrolled(column(page)).into()
}

/// One user's four panels: profile, library and device access, parental
/// control, and password.
pub fn one<'a>(state: &'a One, read_only: bool, own: Uuid) -> Vec<Element<'a, Message>> {
    let panels = widget::localnav(super::UserTab::ALL.into_iter().map(|tab| widget::Entry {
        label: tab.label(),
        showing: match tab == state.tab {
            true => widget::Showing::Shown,
            false => widget::Showing::Offered(Message::DashboardAction(super::Action::Open(
                super::Screen::User { id: state.id, tab },
            ))),
        },
    }));

    let mut page: Vec<Element<'a, Message>> = Vec::new();

    match state.tab {
        super::UserTab::Password => {
            page.push(
                text_input(strings::lookup(Text::UsersPassword), &state.current)
                    .style(style::input)
                    .secure(true)
                    .on_input(|typed| {
                        Message::DashboardAction(super::Action::TypedCurrentPassword(typed))
                    })
                    .into(),
            );
            page.push(
                text_input(strings::lookup(Text::UsersPassword), &state.replacement)
                    .style(style::input)
                    .secure(true)
                    .on_input(|typed| Message::DashboardAction(super::Action::TypedPassword(typed)))
                    .into(),
            );
            if !read_only {
                page.push(
                    button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                        .style(style::submit)
                        .on_press(Message::DashboardAction(super::Action::Write(
                            super::Written::SetPassword { id: state.id },
                        )))
                        .into(),
                );
                page.push(
                    button(prose(
                        strings::lookup(Text::UsersImageUpload),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::ChooseImage))
                    .into(),
                );
                page.push(
                    button(prose(
                        strings::lookup(Text::UsersImageRemove),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::RemoveUserImage { id: state.id },
                            state.name.clone(),
                        ),
                    )))
                    .into(),
                );
            }
        }
        tab => {
            let (form, fields) = match tab {
                super::UserTab::Profile => (&state.configuration, CONFIGURATION),
                _ => (&state.policy, POLICY),
            };
            let policy = !matches!(tab, super::UserTab::Profile);
            for field in fields.iter().filter(|field| shown(tab, **field)) {
                if policy && state.id == own && field.key() == "IsAdministrator" {
                    page.push(prose(
                        strings::lookup(Text::UsersOwnAdministrator),
                        typeface::BODY,
                    ));
                    continue;
                }
                page.push(super::control(*field, form.value(*field)));
            }
            if !read_only {
                page.push(
                    button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                        .style(style::submit)
                        .on_press(Message::DashboardAction(super::Action::Save))
                        .into(),
                );
            }
        }
    }

    let mut shown: Vec<Element<'a, Message>> = vec![panels];
    shown.append(&mut page);
    shown
}

/// Which panel a policy field stands in.
fn shown(tab: super::UserTab, field: Field) -> bool {
    let parental = matches!(
        field.key(),
        "MaxParentalRating" | "BlockedTags" | "AllowedTags"
    );
    match tab {
        super::UserTab::Parental => parental,
        super::UserTab::Access => !parental,
        _ => true,
    }
}
