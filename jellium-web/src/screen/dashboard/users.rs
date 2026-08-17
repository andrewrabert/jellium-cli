//! The users the server holds, and the one user a user screen edits.

use iced::widget::{button, column, row, text_input};
use iced::{Element, Fill};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};
use jellium_model::form::{Field, Form};

/// Every user on the server, and what a new one is being named.
#[derive(Debug, Clone)]
pub struct State {
    pub users: Vec<jellyfin_api::types::UserDto>,
    /// The name and password typed for a new user.
    pub naming: String,
    pub password: String,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            users: api.users().await.bubbled()?,
            naming: String::new(),
            password: String::new(),
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
pub fn new<'a>(state: &'a State) -> Element<'a, Message> {
    column![
        prose(strings::lookup(Text::UsersCreate), typeface::HEADING_2),
        text_input(strings::lookup(Text::UsersName), &state.naming)
            .style(style::input)
            .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
        text_input(strings::lookup(Text::UsersPassword), &state.password)
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::DashboardAction(super::Action::TypedPassword(typed))),
        button(prose(strings::lookup(Text::UsersCreate), typeface::BODY))
            .style(style::submit)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::CreateUser {
                    name: state.naming.clone(),
                    password: state.password.clone(),
                }
            ))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()))
    .into()
}

/// The user list, and the control that creates one.
pub fn view<'a>(state: &'a State, read_only: bool, own: Uuid) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::UsersTitle),
        typeface::HEADING_2
    )]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()));

    if !read_only {
        page = page.push(
            button(prose(strings::lookup(Text::UsersCreate), typeface::BODY))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Open(
                    super::Screen::UserNew,
                ))),
        );
    }

    for user in &state.users {
        let Some(id) = user.id else {
            continue;
        };
        let name = user.name.clone().unwrap_or_default();
        let mut held = row![
            button(line(
                name.clone(),
                typeface::BODY,
                typeface::Weight::Regular
            ))
            .style(style::link)
            .on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::User {
                    id,
                    tab: super::UserTab::Profile,
                }
            ))),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()));

        if id == own {
            held = held.push(prose(
                strings::lookup(Text::UsersOwnAccount),
                typeface::BODY,
            ));
        } else if !read_only {
            held = held.push(
                button(prose(strings::lookup(Text::UsersDelete), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::DeleteUser { id },
                            name,
                        ),
                    ))),
            );
        }
        page = page.push(held);
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}

/// One user's four panels: profile, library and device access, parental
/// control, and password.
pub fn one<'a>(state: &'a One, read_only: bool, own: Uuid) -> Element<'a, Message> {
    let mut tabs = row![].spacing(style::drawn(space::GUTTER.drawn()));
    for tab in super::UserTab::ALL {
        let control =
            button(prose(strings::lookup(tab.label()), typeface::BODY)).style(style::flat);
        tabs = tabs.push(if tab == state.tab {
            control
        } else {
            control.on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::User { id: state.id, tab },
            )))
        });
    }

    let mut page = column![tabs, prose(state.name.clone(), typeface::HEADING_2)]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    match state.tab {
        super::UserTab::Password => {
            page = page.push(
                text_input(strings::lookup(Text::UsersPassword), &state.current)
                    .style(style::input)
                    .secure(true)
                    .on_input(|typed| {
                        Message::DashboardAction(super::Action::TypedCurrentPassword(typed))
                    }),
            );
            page = page.push(
                text_input(strings::lookup(Text::UsersPassword), &state.replacement)
                    .style(style::input)
                    .secure(true)
                    .on_input(|typed| {
                        Message::DashboardAction(super::Action::TypedPassword(typed))
                    }),
            );
            if !read_only {
                page = page.push(
                    button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                        .style(style::submit)
                        .on_press(Message::DashboardAction(super::Action::Write(
                            super::Written::SetPassword { id: state.id },
                        ))),
                );
                page = page.push(
                    button(prose(
                        strings::lookup(Text::UsersImageUpload),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::ChooseImage)),
                );
                page = page.push(
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
                    ))),
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
                    page = page.push(prose(
                        strings::lookup(Text::UsersOwnAdministrator),
                        typeface::BODY,
                    ));
                    continue;
                }
                page = page.push(super::control(*field, form.value(*field), policy));
            }
            if !read_only {
                page = page.push(
                    button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                        .style(style::submit)
                        .on_press(Message::DashboardAction(super::Action::Save)),
                );
            }
        }
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
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
