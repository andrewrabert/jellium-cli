//! The users the server holds, and the one user a user screen edits.

use iced::Element;
use iced::widget::{button, column, text_input};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Template, Text};
use crate::widget::{self, prose};
use jellium_model::appearance::typeface::Rank;
use jellium_model::form::{Field, Form};

use super::{Control, Group, Heading, Offered, Values};

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

/// One user's screen: its four panels and the policy every one of them but the
/// password panel edits, read whole and written whole.
#[derive(Debug, Clone)]
pub struct One {
    pub id: Uuid,
    pub name: String,
    pub tab: super::UserTab,
    pub policy: Form,
    /// The libraries, channels and devices the access panel's lists offer.
    pub sourced: Vec<super::Sourced>,
    /// What has been typed into the two password fields.
    pub current: String,
    pub replacement: String,
}

/// The SyncPlay levels the policy offers, in the order the reference stands
/// them.
// reference: user-profile-syncplay-options
pub const SYNC_PLAY_ACCESS: &[Offered] = &[
    Offered {
        value: "CreateAndJoinGroups",
        label: Text::UsersSyncPlayCreate,
    },
    Offered {
        value: "JoinGroups",
        label: Text::UsersSyncPlayJoin,
    },
    Offered {
        value: "None",
        label: Text::UsersSyncPlayNone,
    },
];

// reference: user-profile-remote-access
const REMOTE_ACCESS: Group = Group {
    heading: None,
    note: None,
    controls: &[Control {
        field: Field::Flag {
            key: "EnableRemoteAccess",
        },
        label: Text::UsersRemoteAccess,
        helper: &[Text::UsersRemoteAccessHelp],
        unit: None,
        offered: None,
    }],
    closing: None,
};

// reference: user-profile-administrator
const ADMINISTRATOR: Group = Group {
    heading: None,
    note: None,
    controls: &[Control {
        field: Field::Flag {
            key: "IsAdministrator",
        },
        label: Text::UsersAdministrator,
        helper: &[],
        unit: None,
        offered: None,
    }],
    closing: None,
};

/// What stands where the administrator control would, on the reader's own
/// account.
const OWN_ADMINISTRATOR: Group = Group {
    heading: None,
    note: Some(Text::UsersOwnAdministrator),
    controls: &[],
    closing: None,
};

// reference: user-profile-feature-access
const FEATURE_ACCESS: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersFeatureAccess,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableLiveTvAccess",
            },
            label: Text::UsersLiveTvAccess,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag {
                key: "EnableLiveTvManagement",
            },
            label: Text::UsersLiveTvManagement,
            helper: &[],
            unit: None,
            offered: None,
        },
    ],
    closing: None,
};

// reference: user-profile-playback
// reference: user-profile-playback-note
const PLAYBACK: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersPlayback,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableMediaPlayback",
            },
            label: Text::UsersMediaPlayback,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag {
                key: "EnableAudioPlaybackTranscoding",
            },
            label: Text::UsersAudioTranscoding,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag {
                key: "EnableVideoPlaybackTranscoding",
            },
            label: Text::UsersVideoTranscoding,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag {
                key: "EnablePlaybackRemuxing",
            },
            label: Text::UsersRemuxing,
            helper: &[],
            unit: None,
            offered: None,
        },
    ],
    closing: Some(Text::UsersPlaybackHelp),
};

// reference: user-profile-bitrate
const BITRATE: Group = Group {
    heading: None,
    note: None,
    controls: &[Control {
        field: Field::Megabits {
            key: "RemoteClientBitrateLimit",
        },
        label: Text::UsersBitrateLimit,
        helper: &[Text::UsersBitrateLimitHelp, Text::UsersBitrateLimitOverride],
        unit: None,
        offered: None,
    }],
    closing: None,
};

// reference: user-profile-syncplay
const SYNC_PLAY: Group = Group {
    heading: None,
    note: None,
    controls: &[Control {
        field: Field::Listed {
            key: "SyncPlayAccess",
        },
        label: Text::UsersSyncPlay,
        helper: &[Text::UsersSyncPlayHelp],
        unit: None,
        offered: Some(Values::Closed(SYNC_PLAY_ACCESS)),
    }],
    closing: None,
};

// reference: user-profile-deletion
const DELETION: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersDeletion,
    }),
    note: None,
    controls: &[Control {
        field: Field::Flag {
            key: "EnableContentDeletion",
        },
        label: Text::UsersDeletionAll,
        helper: &[],
        unit: None,
        offered: None,
    }],
    closing: None,
};

// reference: user-profile-other
const OTHER: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersOther,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableContentDownloading",
            },
            label: Text::UsersDownloads,
            helper: &[Text::UsersDownloadsHelp],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag { key: "IsDisabled" },
            label: Text::UsersDisabled,
            helper: &[Text::UsersDisabledHelp],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Flag { key: "IsHidden" },
            label: Text::UsersHidden,
            helper: &[Text::UsersHiddenHelp],
            unit: None,
            offered: None,
        },
    ],
    closing: None,
};

// reference: user-profile-lockout
// reference: user-profile-sessions
const LIMITS: Group = Group {
    heading: None,
    note: None,
    controls: &[
        Control {
            field: Field::Number {
                key: "LoginAttemptsBeforeLockout",
            },
            label: Text::UsersLockout,
            helper: &[Text::UsersLockoutHelp, Text::UsersLockoutZero],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Number {
                key: "MaxActiveSessions",
            },
            label: Text::UsersSessions,
            helper: &[Text::UsersSessionsHelp, Text::UsersSessionsZero],
            unit: None,
            offered: None,
        },
    ],
    closing: None,
};

// reference: user-access-container
// reference: user-access-folders
const LIBRARY_ACCESS: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersLibraryAccess,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableAllFolders",
            },
            label: Text::UsersAllLibraries,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Lines {
                key: "EnabledFolders",
            },
            label: Text::UsersLibraries,
            helper: &[Text::UsersLibrariesHelp],
            unit: None,
            offered: Some(Values::Checked {
                unless: Some(Field::Flag {
                    key: "EnableAllFolders",
                }),
            }),
        },
    ],
    closing: None,
};

// reference: user-access-container
// reference: user-access-channels
const CHANNEL_ACCESS: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersChannelAccess,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableAllChannels",
            },
            label: Text::UsersAllChannels,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Lines {
                key: "EnabledChannels",
            },
            label: Text::UsersChannels,
            helper: &[Text::UsersChannelsHelp],
            unit: None,
            offered: Some(Values::Checked {
                unless: Some(Field::Flag {
                    key: "EnableAllChannels",
                }),
            }),
        },
    ],
    closing: None,
};

// reference: user-access-container
// reference: user-access-devices
const DEVICE_ACCESS: Group = Group {
    heading: Some(Heading {
        rank: Rank::Second,
        title: Text::UsersDeviceAccess,
    }),
    note: None,
    controls: &[
        Control {
            field: Field::Flag {
                key: "EnableAllDevices",
            },
            label: Text::UsersAllDevices,
            helper: &[],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Lines {
                key: "EnabledDevices",
            },
            label: Text::UsersDevices,
            helper: &[Text::UsersDevicesHelp],
            unit: None,
            offered: Some(Values::Checked {
                unless: Some(Field::Flag {
                    key: "EnableAllDevices",
                }),
            }),
        },
    ],
    closing: None,
};

/// The user's policy as the reference's own profile page stands it.
pub const PROFILE: &[Group] = &[
    REMOTE_ACCESS,
    ADMINISTRATOR,
    FEATURE_ACCESS,
    PLAYBACK,
    BITRATE,
    SYNC_PLAY,
    DELETION,
    OTHER,
    LIMITS,
];

/// The same panel on the reader's own account.
pub const PROFILE_OWN: &[Group] = &[
    REMOTE_ACCESS,
    OWN_ADMINISTRATOR,
    FEATURE_ACCESS,
    PLAYBACK,
    BITRATE,
    SYNC_PLAY,
    DELETION,
    OTHER,
    LIMITS,
];

/// The libraries, channels and devices the user reaches, as the reference's
/// own access page stands them.
pub const ACCESS: &[Group] = &[LIBRARY_ACCESS, CHANNEL_ACCESS, DEVICE_ACCESS];

/// The parental controls, in the order the reference stands them.
// reference: user-parental-rating
// reference: user-parental-allowed-tags
// reference: user-parental-blocked-tags
pub const PARENTAL: &[Group] = &[Group {
    heading: None,
    note: None,
    controls: &[
        Control {
            field: Field::Number {
                key: "MaxParentalRating",
            },
            label: Text::UsersMaxRating,
            helper: &[Text::UsersMaxRatingHelp],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Lines { key: "AllowedTags" },
            label: Text::UsersAllowedTags,
            helper: &[Text::UsersAllowedTagsHelp],
            unit: None,
            offered: None,
        },
        Control {
            field: Field::Lines { key: "BlockedTags" },
            label: Text::UsersBlockedTags,
            helper: &[Text::UsersBlockedTagsHelp],
            unit: None,
            offered: None,
        },
    ],
    closing: None,
}];

/// One item the server names, as a row of a checkbox list.
fn item(item: &jellyfin_api::types::BaseItemDto) -> Option<crate::widget::Choice<String>> {
    Some(crate::widget::Choice {
        label: item.name.clone().unwrap_or_default(),
        value: item.id?.to_string(),
    })
}

/// One device as the reference's own access list writes it: its custom name or
/// its name, joined to the application it runs.
// reference: user-access-devices
fn device(held: &jellyfin_api::types::DeviceInfoDto) -> Option<crate::widget::Choice<String>> {
    let named = held
        .custom_name
        .clone()
        .or_else(|| held.name.clone())
        .unwrap_or_default();
    let written = match held.app_name.as_deref() {
        Some(app) => format!("{named} - {app}"),
        None => named,
    };
    Some(crate::widget::Choice {
        label: written,
        value: held.id.clone()?,
    })
}

pub async fn open(api: std::rc::Rc<crate::api::Api>, id: Uuid, tab: super::UserTab) -> Answer<One> {
    Answer::of(async {
        let user = api.user(id).await.bubbled()?;
        let folders = api.media_folders().await.bubbled()?;
        let channels = api.channels().await.bubbled()?;
        let devices = api.devices().await.bubbled()?;
        Ok(One {
            id,
            name: user.name.clone().unwrap_or_default(),
            tab,
            policy: Form::of(api.policy(id).await.bubbled()?),
            sourced: vec![
                super::Sourced {
                    field: Field::Lines {
                        key: "EnabledFolders",
                    },
                    rows: folders.iter().filter_map(item).collect(),
                },
                super::Sourced {
                    field: Field::Lines {
                        key: "EnabledChannels",
                    },
                    rows: channels.iter().filter_map(item).collect(),
                },
                super::Sourced {
                    field: Field::Lines {
                        key: "EnabledDevices",
                    },
                    rows: devices.iter().filter_map(device).collect(),
                },
            ],
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
        button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
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
        card: card::Card::USER,
    }
}

/// The image every account with one draws on its card.
// reference: user-card
pub fn images(state: &State) -> crate::images::Wanted {
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
            Text::UsersAdd,
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
                .map(|at| strings::format(Template::UsersLastSeen, &[&widget::table::stamped(at)])),
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

/// One user's four panels: profile, access, parental control, and password.
pub fn one<'a>(
    state: &'a One,
    read_only: bool,
    own: Uuid,
    viewport: Viewport,
) -> Vec<Element<'a, Message>> {
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
            let account = match state.id == own {
                true => super::Account::Own,
                false => super::Account::Other,
            };
            page.push(super::controls(
                tab.groups(account),
                &state.policy,
                &state.sourced,
                super::Controls::Emby,
                viewport,
            ));
            if !read_only {
                page.push(super::save(
                    super::Controls::Emby,
                    Some(Message::DashboardAction(super::Action::Save)),
                    viewport.layout(),
                ));
            }
        }
    }

    let mut shown: Vec<Element<'a, Message>> = vec![panels];
    shown.append(&mut page);
    shown
}
