//! The dashboard is one region of routes with its own navigation column, its
//! own feeds and one confirmation rule.

pub mod activity;
pub mod catalog;
pub mod devices;
pub mod frame;
pub mod home;
pub mod keys;
pub mod libraries;
pub mod livetv;
pub mod logs;
pub mod page;
pub mod plugins;
pub mod repositories;
pub mod settings;
pub mod tasks;
pub mod users;

use std::rc::Rc;

use iced::widget::{column, row};
use iced::{Element, Task};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation, Wrote};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// Which controls a settings section is drawn with: MUI's, on one of the
/// dashboard's react routes, or the reference's own, on one of its legacy
/// views.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controls {
    Mui,
    Emby,
}

/// The heading a group of a section's fields stands under: the level the
/// reference writes it at, and what it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Heading {
    pub rank: jellium_model::appearance::typeface::Rank,
    pub title: Text,
}

/// One option a control offers: the value it writes and the sentence naming
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offered {
    pub value: &'static str,
    pub label: Text,
}

/// One control of a section's form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Control {
    /// The key it edits, and how that key is read and written.
    pub field: jellium_model::form::Field,
    /// The sentence naming it, which a filled field shrinks into its own head
    /// and a box writes beside itself.
    pub label: Text,
    /// The sentence `MuiFormHelperText` writes under it, and nothing where the
    /// reference writes none.
    pub helper: Option<Text>,
    /// What each value it offers reads as, in the order the reference stands
    /// them, and nothing where it offers no closed set.
    pub offered: Option<&'static [Offered]>,
}

/// One group of a section's controls: the heading the reference writes over
/// them, the sentence it writes between that heading and them, and the
/// controls standing under it in the order it stands them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Group {
    pub heading: Option<Heading>,
    pub note: Option<Text>,
    pub controls: &'static [Control],
}

/// The acceleration methods the transcoding screen offers, in the order the
/// reference stands them.
// reference: dashboard-content
const ACCELERATION: &[Offered] = &[
    Offered {
        value: "none",
        label: Text::TranscodingAccelerationNone,
    },
    Offered {
        value: "amf",
        label: Text::TranscodingAccelerationAmf,
    },
    Offered {
        value: "nvenc",
        label: Text::TranscodingAccelerationNvenc,
    },
    Offered {
        value: "qsv",
        label: Text::TranscodingAccelerationQsv,
    },
    Offered {
        value: "vaapi",
        label: Text::TranscodingAccelerationVaapi,
    },
    Offered {
        value: "rkmpp",
        label: Text::TranscodingAccelerationRkmpp,
    },
    Offered {
        value: "videotoolbox",
        label: Text::TranscodingAccelerationVideoToolbox,
    },
    Offered {
        value: "v4l2m2m",
        label: Text::TranscodingAccelerationV4l2m2m,
    },
];

/// Which configuration section a settings screen edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    General,
    Networking,
    Branding,
    Resume,
    Streaming,
    Transcoding,
    Trickplay,
}

impl Section {
    pub fn label(self) -> Text {
        match self {
            Section::General => Text::DashboardSettings,
            Section::Networking => Text::DashboardNetworking,
            Section::Branding => Text::DashboardBranding,
            Section::Resume => Text::DashboardResume,
            Section::Streaming => Text::DashboardStreaming,
            Section::Transcoding => Text::DashboardTranscoding,
            Section::Trickplay => Text::DashboardTrickplay,
        }
    }

    /// The named configuration key this section reads and writes, and `None`
    /// for the sections the whole server configuration carries.
    pub fn key(self) -> Option<&'static str> {
        match self {
            Section::General | Section::Resume | Section::Streaming => None,
            Section::Networking => Some("network"),
            Section::Branding => Some("branding"),
            Section::Transcoding => Some("encoding"),
            Section::Trickplay => Some("trickplay"),
        }
    }

    // reference: dashboard-frame
    pub fn controls(self) -> Controls {
        match self {
            Section::Networking => Controls::Emby,
            Section::General
            | Section::Branding
            | Section::Resume
            | Section::Streaming
            | Section::Transcoding
            | Section::Trickplay => Controls::Mui,
        }
    }

    /// The groups this section's fields stand in; every key outside them
    /// survives a save.
    pub fn groups(self) -> &'static [Group] {
        use jellium_model::appearance::typeface::Rank;
        use jellium_model::form::Field;
        match self {
            Section::General => &[
                Group {
                    heading: None,
                    note: None,
                    controls: &[
                        Control {
                            field: Field::Text { key: "ServerName" },
                            label: Text::GeneralServerName,
                            helper: Some(Text::GeneralServerNameHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Text { key: "UICulture" },
                            label: Text::GeneralDisplayLanguage,
                            helper: Some(Text::GeneralDisplayLanguageHelp),
                            offered: None,
                        },
                    ],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Second,
                        title: Text::DashboardPaths,
                    }),
                    note: None,
                    controls: &[Control {
                        field: Field::Text {
                            key: "MetadataPath",
                        },
                        label: Text::GeneralMetadataPath,
                        helper: Some(Text::GeneralMetadataPathHelp),
                        offered: None,
                    }],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Second,
                        title: Text::DashboardQuickConnect,
                    }),
                    note: None,
                    controls: &[Control {
                        field: Field::Flag {
                            key: "QuickConnectAvailable",
                        },
                        label: Text::GeneralQuickConnect,
                        helper: None,
                        offered: None,
                    }],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Second,
                        title: Text::DashboardPerformance,
                    }),
                    note: None,
                    controls: &[Control {
                        field: Field::Number {
                            key: "LibraryScanFanoutConcurrency",
                        },
                        label: Text::GeneralScanConcurrency,
                        helper: Some(Text::GeneralScanConcurrencyHelp),
                        offered: None,
                    }],
                },
            ],
            Section::Networking => &[
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::NetworkingAddresses,
                    }),
                    note: None,
                    controls: &[
                        Control {
                            field: Field::Number {
                                key: "InternalHttpPort",
                            },
                            label: Text::NetworkingPort,
                            helper: Some(Text::NetworkingPortHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Text { key: "BaseUrl" },
                            label: Text::NetworkingBaseUrl,
                            helper: Some(Text::NetworkingBaseUrlHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Lines {
                                key: "LocalNetworkSubnets",
                            },
                            label: Text::NetworkingLanNetworks,
                            helper: Some(Text::NetworkingLanNetworksHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Lines {
                                key: "KnownProxies",
                            },
                            label: Text::NetworkingKnownProxies,
                            helper: Some(Text::NetworkingKnownProxiesHelp),
                            offered: None,
                        },
                    ],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::NetworkingRemote,
                    }),
                    note: None,
                    controls: &[
                        Control {
                            field: Field::Flag {
                                key: "EnableRemoteAccess",
                            },
                            label: Text::NetworkingRemoteAccess,
                            helper: Some(Text::NetworkingRemoteAccessHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Lines {
                                key: "RemoteIPFilter",
                            },
                            label: Text::NetworkingRemoteFilter,
                            helper: Some(Text::NetworkingRemoteFilterHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Number {
                                key: "PublicHttpPort",
                            },
                            label: Text::NetworkingPublicPort,
                            helper: Some(Text::NetworkingPublicPortHelp),
                            offered: None,
                        },
                    ],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::NetworkingProtocols,
                    }),
                    note: None,
                    controls: &[
                        Control {
                            field: Field::Flag { key: "EnableIPv4" },
                            label: Text::NetworkingIpv4,
                            helper: Some(Text::NetworkingIpv4Help),
                            offered: None,
                        },
                        Control {
                            field: Field::Flag { key: "EnableIPv6" },
                            label: Text::NetworkingIpv6,
                            helper: Some(Text::NetworkingIpv6Help),
                            offered: None,
                        },
                    ],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::NetworkingDiscovery,
                    }),
                    note: None,
                    controls: &[Control {
                        field: Field::Flag {
                            key: "AutoDiscovery",
                        },
                        label: Text::NetworkingAutoDiscovery,
                        helper: Some(Text::NetworkingAutoDiscoveryHelp),
                        offered: None,
                    }],
                },
            ],
            Section::Branding => &[Group {
                heading: None,
                note: None,
                controls: &[
                    Control {
                        field: Field::Flag {
                            key: "SplashscreenEnabled",
                        },
                        label: Text::BrandingSplashscreen,
                        helper: None,
                        offered: None,
                    },
                    Control {
                        field: Field::Text {
                            key: "LoginDisclaimer",
                        },
                        label: Text::BrandingLoginDisclaimer,
                        helper: Some(Text::BrandingLoginDisclaimerHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Text { key: "CustomCss" },
                        label: Text::BrandingCustomCss,
                        helper: Some(Text::BrandingCustomCssHelp),
                        offered: None,
                    },
                ],
            }],
            Section::Resume => &[Group {
                heading: None,
                note: None,
                controls: &[
                    Control {
                        field: Field::Number {
                            key: "MinResumePct",
                        },
                        label: Text::ResumeMinimumPercentage,
                        helper: Some(Text::ResumeMinimumPercentageHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number {
                            key: "MaxResumePct",
                        },
                        label: Text::ResumeMaximumPercentage,
                        helper: Some(Text::ResumeMaximumPercentageHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number {
                            key: "MinAudiobookResume",
                        },
                        label: Text::ResumeMinimumAudiobook,
                        helper: Some(Text::ResumeMinimumAudiobookHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number {
                            key: "MaxAudiobookResume",
                        },
                        label: Text::ResumeMaximumAudiobook,
                        helper: Some(Text::ResumeMaximumAudiobookHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number {
                            key: "MinResumeDurationSeconds",
                        },
                        label: Text::ResumeMinimumDuration,
                        helper: Some(Text::ResumeMinimumDurationHelp),
                        offered: None,
                    },
                ],
            }],
            Section::Streaming => &[Group {
                heading: None,
                note: None,
                controls: &[Control {
                    field: Field::Number {
                        key: "RemoteClientBitrateLimit",
                    },
                    label: Text::StreamingBitrateLimit,
                    helper: Some(Text::StreamingBitrateLimitHelp),
                    offered: None,
                }],
            }],
            Section::Transcoding => &[
                Group {
                    heading: None,
                    note: None,
                    controls: &[Control {
                        field: Field::Listed {
                            key: "HardwareAccelerationType",
                        },
                        label: Text::TranscodingHardwareAcceleration,
                        helper: Some(Text::TranscodingHardwareAccelerationHelp),
                        offered: Some(ACCELERATION),
                    }],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::TranscodingHardwareEncoding,
                    }),
                    note: None,
                    controls: &[Control {
                        field: Field::Flag {
                            key: "EnableHardwareEncoding",
                        },
                        label: Text::TranscodingEnableHardwareEncoding,
                        helper: None,
                        offered: None,
                    }],
                },
                Group {
                    heading: Some(Heading {
                        rank: Rank::Third,
                        title: Text::TranscodingEncodingFormat,
                    }),
                    note: Some(Text::TranscodingEncodingFormatHelp),
                    controls: &[Control {
                        field: Field::Flag {
                            key: "AllowHevcEncoding",
                        },
                        label: Text::TranscodingAllowHevc,
                        helper: None,
                        offered: None,
                    }],
                },
                Group {
                    heading: None,
                    note: None,
                    controls: &[
                        Control {
                            field: Field::Flag {
                                key: "EnableTonemapping",
                            },
                            label: Text::TranscodingTonemapping,
                            helper: Some(Text::TranscodingTonemappingHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Number {
                                key: "EncodingThreadCount",
                            },
                            label: Text::TranscodingThreadCount,
                            helper: Some(Text::TranscodingThreadCountHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Text {
                                key: "TranscodingTempPath",
                            },
                            label: Text::TranscodingPath,
                            helper: Some(Text::TranscodingPathHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Number {
                                key: "DownMixAudioBoost",
                            },
                            label: Text::TranscodingDownMixBoost,
                            helper: Some(Text::TranscodingDownMixBoostHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Flag {
                                key: "EnableThrottling",
                            },
                            label: Text::TranscodingThrottle,
                            helper: Some(Text::TranscodingThrottleHelp),
                            offered: None,
                        },
                        Control {
                            field: Field::Number {
                                key: "ThrottleDelaySeconds",
                            },
                            label: Text::TranscodingThrottleDelay,
                            helper: Some(Text::TranscodingThrottleDelayHelp),
                            offered: None,
                        },
                    ],
                },
            ],
            Section::Trickplay => &[Group {
                heading: None,
                note: None,
                controls: &[
                    Control {
                        field: Field::Flag {
                            key: "EnableHwAcceleration",
                        },
                        label: Text::TrickplayHardwareDecoding,
                        helper: None,
                        offered: None,
                    },
                    Control {
                        field: Field::Flag {
                            key: "EnableHwEncoding",
                        },
                        label: Text::TrickplayHardwareEncoding,
                        helper: Some(Text::TrickplayHardwareEncodingHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number { key: "Interval" },
                        label: Text::TrickplayInterval,
                        helper: Some(Text::TrickplayIntervalHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number { key: "TileWidth" },
                        label: Text::TrickplayTileWidth,
                        helper: Some(Text::TrickplayTileWidthHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number { key: "TileHeight" },
                        label: Text::TrickplayTileHeight,
                        helper: Some(Text::TrickplayTileHeightHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number { key: "JpegQuality" },
                        label: Text::TrickplayJpegQuality,
                        helper: Some(Text::TrickplayJpegQualityHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number { key: "Qscale" },
                        label: Text::TrickplayQscale,
                        helper: Some(Text::TrickplayQscaleHelp),
                        offered: None,
                    },
                    Control {
                        field: Field::Number {
                            key: "ProcessThreads",
                        },
                        label: Text::TrickplayThreads,
                        helper: Some(Text::TrickplayThreadsHelp),
                        offered: None,
                    },
                ],
            }],
        }
    }
}

/// Which of a user screen's four panels is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserTab {
    Profile,
    Access,
    Parental,
    Password,
}

impl UserTab {
    pub const ALL: [UserTab; 4] = [
        UserTab::Profile,
        UserTab::Access,
        UserTab::Parental,
        UserTab::Password,
    ];

    pub fn label(self) -> Text {
        match self {
            UserTab::Profile => Text::UsersProfile,
            UserTab::Access => Text::UsersAccess,
            UserTab::Parental => Text::UsersParental,
            UserTab::Password => Text::UsersPassword,
        }
    }
}

/// Which of the Live TV administration screens is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveTvTab {
    Tuners,
    Providers,
    Mapping,
    Dvr,
}

impl LiveTvTab {
    pub const ALL: [LiveTvTab; 4] = [
        LiveTvTab::Tuners,
        LiveTvTab::Providers,
        LiveTvTab::Mapping,
        LiveTvTab::Dvr,
    ];

    pub fn label(self) -> Text {
        match self {
            LiveTvTab::Tuners => Text::TunersTitle,
            LiveTvTab::Providers => Text::ProvidersTitle,
            LiveTvTab::Mapping => Text::MappingTitle,
            LiveTvTab::Dvr => Text::DvrTitle,
        }
    }
}

/// One control of a form, drawn by what its field edits.
pub fn control<'a>(field: jellium_model::form::Field, held: String) -> Element<'a, Message> {
    match field {
        jellium_model::form::Field::Flag { key } => Element::from(
            row![
                iced::widget::checkbox(held == "true").on_toggle(move |held| {
                    Message::DashboardAction(Action::Edited(field, held.to_string()))
                }),
                prose(key, typeface::BODY),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .align_y(iced::Center),
        ),
        _ => Element::from(
            column![
                prose(field.key(), typeface::BODY),
                iced::widget::text_input(field.key(), &held)
                    .style(style::input)
                    .on_input(move |held| {
                        Message::DashboardAction(Action::Edited(field, held))
                    }),
            ]
            .spacing(style::drawn(space::BLOCK_GAP.drawn())),
        ),
    }
}

/// One dashboard screen, and everything that addresses it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Home,
    Settings { section: Section },
    Users,
    User { id: Uuid, tab: UserTab },
    UserNew,
    Libraries,
    Library { name: String },
    Tasks,
    Task { id: String },
    Logs,
    Log { name: String },
    Activity,
    Plugins,
    Catalog,
    Repositories,
    Devices,
    Keys,
    LiveTv { tab: LiveTvTab },
    PluginPage { plugin: Uuid, name: String },
}

impl Screen {
    /// The drawer entry this screen stands under, which is the entry the
    /// drawer draws as shown while this screen is open.
    // reference: drawer-server
    // reference: drawer-devices
    // reference: drawer-livetv
    // reference: drawer-plugins
    // reference: drawer-advanced
    pub fn under(&self) -> Screen {
        match self {
            Screen::User { .. } | Screen::UserNew => Screen::Users,
            Screen::Library { .. } => Screen::Libraries,
            Screen::Task { .. } => Screen::Tasks,
            Screen::Log { .. } => Screen::Logs,
            Screen::Catalog | Screen::Repositories | Screen::PluginPage { .. } => Screen::Plugins,
            Screen::LiveTv {
                tab: LiveTvTab::Providers | LiveTvTab::Mapping,
            } => Screen::LiveTv {
                tab: LiveTvTab::Tuners,
            },
            other => other.clone(),
        }
    }

    /// The glyph the drawer stands behind this screen's entry.
    // reference: drawer-server
    // reference: drawer-devices
    // reference: drawer-livetv
    // reference: drawer-plugins
    // reference: drawer-advanced
    pub fn glyph(&self) -> crate::icon::Icon {
        use crate::icon::Icon;
        match self {
            Screen::Home => Icon::Dashboard,
            Screen::Settings {
                section: Section::General,
            } => Icon::Settings,
            Screen::Settings {
                section: Section::Branding,
            } => Icon::Palette,
            Screen::Settings {
                section: Section::Networking,
            } => Icon::Lan,
            Screen::Settings { .. } => Icon::PlayCircle,
            Screen::Users | Screen::User { .. } | Screen::UserNew => Icon::People,
            Screen::Libraries | Screen::Library { .. } => Icon::LibraryAdd,
            Screen::Devices => Icon::Devices,
            Screen::Activity => Icon::Analytics,
            Screen::LiveTv {
                tab: LiveTvTab::Dvr,
            } => Icon::Dvr,
            Screen::LiveTv { .. } => Icon::LiveTv,
            Screen::Plugins
            | Screen::Catalog
            | Screen::Repositories
            | Screen::PluginPage { .. } => Icon::Extension,
            Screen::Keys => Icon::VpnKey,
            Screen::Logs | Screen::Log { .. } => Icon::Article,
            Screen::Tasks | Screen::Task { .. } => Icon::Schedule,
        }
    }

    pub fn label(&self) -> Text {
        match self {
            Screen::Home => Text::DashboardHome,
            Screen::Settings { section } => section.label(),
            Screen::Users | Screen::User { .. } | Screen::UserNew => Text::UsersTitle,
            Screen::Libraries | Screen::Library { .. } => Text::LibrariesTitle,
            Screen::Tasks | Screen::Task { .. } => Text::TasksTitle,
            Screen::Logs | Screen::Log { .. } => Text::LogsTitle,
            Screen::Activity => Text::ActivityTitle,
            Screen::Catalog => Text::CatalogTitle,
            Screen::Repositories => Text::RepositoriesTitle,
            Screen::Devices => Text::DevicesTitle,
            Screen::Keys => Text::KeysTitle,
            Screen::LiveTv { tab } => tab.label(),
            Screen::Plugins => Text::PluginsTitle,
            Screen::PluginPage { .. } => Text::PluginsConfigurationPages,
        }
    }

    /// The feeds this screen consumes, which are exactly the ones held open
    /// while it is on top of the history stack.
    pub fn feeds(&self) -> &'static [jellium_protocol::Feed] {
        use jellium_protocol::Feed;
        match self {
            Screen::Home => &[Feed::Sessions, Feed::Tasks, Feed::Refresh],
            Screen::Libraries => &[Feed::Refresh],
            Screen::Tasks | Screen::Task { .. } => &[Feed::Tasks],
            Screen::Activity => &[Feed::Activity],
            Screen::Plugins | Screen::Catalog => &[Feed::Packages],
            Screen::Settings { .. }
            | Screen::Users
            | Screen::User { .. }
            | Screen::UserNew
            | Screen::Library { .. }
            | Screen::Logs
            | Screen::Log { .. }
            | Screen::Repositories
            | Screen::Devices
            | Screen::Keys
            | Screen::LiveTv { .. }
            | Screen::PluginPage { .. } => &[],
        }
    }
}

/// What the shown screen holds.
#[derive(Debug)]
pub enum Body {
    Home(Box<home::State>),
    Settings(Box<settings::State>),
    Users(Box<users::State>),
    User(Box<users::One>),
    Libraries(Box<libraries::State>),
    Library(Box<libraries::One>),
    Tasks(Box<tasks::State>),
    Task(Box<tasks::One>),
    Logs(Box<logs::State>),
    Log(Box<logs::Viewer>),
    Activity(Box<activity::State>),
    Plugins(Box<plugins::State>),
    Catalog(Box<catalog::State>),
    Repositories(Box<repositories::State>),
    Devices(Box<devices::State>),
    Keys(Box<keys::State>),
    LiveTv(Box<livetv::State>),
    Page(Box<page::State>),
}

/// What one dashboard screen answered with, before anything it owns is
/// mounted; a message carries this rather than `State`, because a mounted
/// frame is removed by its own `Drop`.
#[derive(Debug, Clone)]
pub enum Loaded {
    Home(Box<home::State>),
    Settings(Box<settings::State>),
    Users(Box<users::State>),
    User(Box<users::One>),
    Libraries(Box<libraries::State>),
    Library(Box<libraries::One>),
    Tasks(Box<tasks::State>),
    Task(Box<tasks::One>),
    Logs(Box<logs::State>),
    Log(Box<logs::Viewer>),
    Activity(Box<activity::State>),
    Plugins(Box<plugins::State>),
    Catalog(Box<catalog::State>),
    Repositories(Box<repositories::State>),
    Devices(Box<devices::State>),
    Keys(Box<keys::State>),
    LiveTv(Box<livetv::State>),
    Page(page::Opened),
}

/// The dashboard screen: what it shows, what it holds, and what stands over it.
#[derive(Debug)]
pub struct State {
    pub screen: Screen,
    pub body: Body,
    /// The destructive action awaiting its confirmation.
    pub confirming: Option<crate::screen::confirm::Pending>,
    /// The drawer groups standing over what they hold, seeded with the group
    /// the shown screen stands in.
    pub opened: std::collections::BTreeSet<frame::Group>,
}

impl State {
    /// Installs what `loaded` answered with, mounting whatever the screen owns.
    pub fn of(screen: Screen, loaded: Loaded) -> State {
        let body = match loaded {
            Loaded::Home(held) => Body::Home(held),
            Loaded::Settings(held) => Body::Settings(held),
            Loaded::Users(held) => Body::Users(held),
            Loaded::User(held) => Body::User(held),
            Loaded::Libraries(held) => Body::Libraries(held),
            Loaded::Library(held) => Body::Library(held),
            Loaded::Tasks(held) => Body::Tasks(held),
            Loaded::Task(held) => Body::Task(held),
            Loaded::Logs(held) => Body::Logs(held),
            Loaded::Log(held) => Body::Log(held),
            Loaded::Activity(held) => Body::Activity(held),
            Loaded::Plugins(held) => Body::Plugins(held),
            Loaded::Catalog(held) => Body::Catalog(held),
            Loaded::Repositories(held) => Body::Repositories(held),
            Loaded::Devices(held) => Body::Devices(held),
            Loaded::Keys(held) => Body::Keys(held),
            Loaded::LiveTv(held) => Body::LiveTv(held),
            Loaded::Page(opened) => Body::Page(Box::new(page::mounted(opened))),
        };
        State {
            opened: frame::Group::of(&screen).into_iter().collect(),
            screen,
            body,
            confirming: None,
        }
    }
}

/// One administrative write the dashboard issues.
#[derive(Debug, Clone, PartialEq)]
pub enum Written {
    ScanAll,
    ScanLibrary {
        name: String,
    },
    CreateLibrary {
        name: String,
        content_type: String,
    },
    RenameLibrary {
        name: String,
        renamed: String,
    },
    AddPath {
        library: String,
        path: String,
    },
    SetPassword {
        id: Uuid,
    },
    StartTask {
        id: String,
        name: String,
    },
    SetTriggers {
        id: String,
        name: String,
    },
    AddTrigger {
        kind: jellyfin_api::types::TaskTriggerInfoType,
    },
    RemoveTrigger {
        index: usize,
    },
    CancelInstall {
        package: Uuid,
        name: String,
    },
    SetDeviceName {
        id: String,
        name: String,
    },
    CreateKey {
        app: String,
    },
    AddTuner {
        url: String,
        kind: String,
    },
    ResetTuner {
        id: String,
        name: String,
    },
    DiscoverTuners,
    AddProvider,
    FetchLineups,
    MapChannel {
        tuner: String,
        provider: String,
    },
    CreateUser {
        name: String,
        password: String,
    },
    EnablePlugin {
        id: Uuid,
        version: String,
        /// The plugin's name, which is what a refusal's sentence names.
        name: String,
    },
    DisablePlugin {
        id: Uuid,
        version: String,
        name: String,
    },
}

/// Every control the dashboard resolves to.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Open(Screen),
    /// Opens the drawer group, or closes it where it is already open.
    Opened(frame::Group),
    /// Browses the server's own filesystem at `path`, which is how a media
    /// path is chosen rather than typed.
    Browse(String),
    /// Shows only the activity entries naming a user, only those naming none,
    /// or all of them.
    Filtered(Option<bool>),
    /// Chooses what a listing provider is being added as, and what it carries.
    ProviderKind(bool),
    ProviderCountry(String),
    ProviderPostcode(String),
    ProviderLineup(String),
    /// Types into the one field the shown screen offers.
    TypedPassword(String),
    TypedCurrentPassword(String),
    /// Edits one field of the form the shown screen holds.
    Edited(jellium_model::form::Field, String),
    /// Writes the form the shown screen holds, whole.
    Save,
    /// Asks about a destructive action, types its object's name, abandons it,
    /// and carries it out.
    /// Opens the file input's picker for the user screen's image.
    ChooseImage,
    Ask(crate::screen::confirm::Pending),
    Typed(String),
    /// Chooses what a new library is created as.
    ContentType(crate::widget::Choice<String>),
    Close,
    Confirm,
    /// Runs one write that is not destructive.
    Write(Written),
    /// One bridge payload a configuration frame raised.
    Bridged(String),
    /// Opens the menu one account's card offers, and closes it.
    UserMenu(Option<Uuid>),
}

/// Loads the screen `screen` names, against the page `viewport` measures.
pub async fn load(
    api: Rc<Api>,
    screen: Screen,
    viewport: Viewport,
    device: String,
) -> Answer<(Screen, Loaded)> {
    let height = viewport.canvas().height();
    Answer::of(async {
        let loaded = match screen.clone() {
            Screen::Home => Loaded::Home(Box::new(home::load(api).await.bubbled()?)),
            Screen::Settings { section } => {
                Loaded::Settings(Box::new(settings::load(api, section).await.bubbled()?))
            }
            Screen::Users | Screen::UserNew => {
                Loaded::Users(Box::new(users::load(api).await.bubbled()?))
            }
            Screen::User { id, tab } => {
                Loaded::User(Box::new(users::open(api, id, tab).await.bubbled()?))
            }
            Screen::Libraries => Loaded::Libraries(Box::new(libraries::load(api).await.bubbled()?)),
            Screen::Library { name } => {
                Loaded::Library(Box::new(libraries::open(api, name).await.bubbled()?))
            }
            Screen::Tasks => Loaded::Tasks(Box::new(tasks::load(api).await.bubbled()?)),
            Screen::Task { id } => Loaded::Task(Box::new(tasks::open(api, id).await.bubbled()?)),
            Screen::Logs => Loaded::Logs(Box::new(logs::load(api).await.bubbled()?)),
            Screen::Log { name } => {
                Loaded::Log(Box::new(logs::open(api, name, height).await.bubbled()?))
            }
            Screen::Activity => Loaded::Activity(Box::new(
                activity::load(api, None, viewport).await.bubbled()?,
            )),
            Screen::Plugins => Loaded::Plugins(Box::new(plugins::load(api).await.bubbled()?)),
            Screen::Catalog => {
                Loaded::Catalog(Box::new(catalog::load(api, height).await.bubbled()?))
            }
            Screen::Repositories => {
                Loaded::Repositories(Box::new(repositories::load(api).await.bubbled()?))
            }
            Screen::Devices => Loaded::Devices(Box::new(
                devices::load(api, device, viewport).await.bubbled()?,
            )),
            Screen::Keys => Loaded::Keys(Box::new(keys::load(api, viewport).await.bubbled()?)),
            Screen::LiveTv { tab } => {
                Loaded::LiveTv(Box::new(livetv::load(api, tab).await.bubbled()?))
            }
            Screen::PluginPage { plugin, name } => {
                Loaded::Page(page::load(name, plugin).await.bubbled()?)
            }
        };
        Ok((screen, loaded))
    })
    .await
}

/// The screen shown inside the dashboard's own page frame.
pub fn view<'a>(
    state: &'a State,
    session: &'a jellium_protocol::Session,
    images: &'a crate::images::Cache,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let filling: frame::Filling<'a> = match state.confirming.as_ref() {
        Some(pending) => frame::Filling::Stacked(vec![crate::screen::confirm::view(
            pending,
            crate::screen::confirm::Region::Dashboard,
        )]),
        None => match &state.body {
            Body::Home(held) => frame::Filling::Stacked(home::view(held, session.read_only)),
            Body::Settings(held) => settings::view(held, session.read_only, viewport),
            Body::Users(held) => match state.screen {
                Screen::UserNew => frame::Filling::Stacked(users::new(held)),
                _ => frame::Filling::Whole(users::view(held, session.read_only, images, viewport)),
            },
            Body::User(held) => {
                frame::Filling::Stacked(users::one(held, session.read_only, session.user_id))
            }
            Body::Libraries(held) => {
                frame::Filling::Stacked(libraries::view(held, session.read_only))
            }
            Body::Library(held) => frame::Filling::Stacked(libraries::one(held, session.read_only)),
            Body::Tasks(held) => tasks::view(held, session.read_only, now, viewport.band()),
            Body::Task(held) => frame::Filling::Stacked(tasks::one(held, session.read_only)),
            Body::Logs(held) => logs::view(held, session.read_only, viewport),
            Body::Log(held) => logs::viewer(held, viewport.band()),
            Body::Activity(held) => activity::view(held, viewport.band()),
            Body::Catalog(held) => frame::Filling::Stacked(catalog::view(held, session.read_only)),
            Body::Repositories(held) => {
                repositories::view(held, session.read_only, viewport.band())
            }
            Body::Devices(held) => devices::view(held, session.read_only),
            Body::Keys(held) => keys::view(held, session.read_only),
            Body::LiveTv(held) => frame::Filling::Stacked(livetv::view(held, session.read_only)),
            Body::Plugins(held) => frame::Filling::Stacked(plugins::view(held, session.read_only)),
            Body::Page(held) => frame::Filling::Stacked(shown_page(held)),
        },
    };

    let title = match state.screen {
        Screen::Tasks | Screen::Log { .. } => None,
        _ => Some(state.screen.label()),
    };
    frame::frame(&state.screen, &state.opened, title, filling, viewport)
}

/// What stands beside a configuration page: the frame occupies the viewport
/// itself, so only the page's name and its busy state are drawn.
fn shown_page<'a>(held: &'a page::State) -> Vec<Element<'a, Message>> {
    let mut shown = vec![prose(held.name.clone(), typeface::BODY)];
    if held.busy {
        shown.push(prose(strings::lookup(Text::StatusLoading), typeface::BODY));
    }
    shown
}

/// The write that deletes one device, told apart on its way back by the
/// operation it reports under.
fn device_deleted(
    api: std::rc::Rc<crate::api::Api>,
    operation: Operation,
    id: String,
    object: String,
) -> Task<Message> {
    Task::perform(
        async move { api.delete_device(&id).await },
        move |outcome| {
            Message::DashboardWrote(
                Wrote {
                    operation,
                    object: object.clone(),
                },
                outcome,
            )
        },
    )
}

/// Applies a control.
/// A write the mode forecloses is never reachable, because `--read-only` leaves
/// its control out of the view.
pub fn act(signed: &mut Signed, action: Action, viewport: Viewport) -> Task<Message> {
    match action {
        Action::Open(screen) => Task::done(Message::Navigated(crate::route::Route::Dashboard {
            screen,
        })),
        Action::Opened(group) => {
            if let Some(state) = shown_mut(signed)
                && !state.opened.remove(&group)
            {
                state.opened.insert(group);
            }
            Task::none()
        }
        Action::Edited(field, value) => {
            match shown_mut(signed).map(|state| &mut state.body) {
                Some(Body::Settings(held)) => {
                    held.form.edit(field, value);
                    held.saved = false;
                }
                Some(Body::Logs(held)) => {
                    held.form.edit(field, value);
                    held.saved = false;
                }
                Some(Body::LiveTv(held)) => held.dvr.edit(field, value),
                Some(Body::User(held)) => {
                    if users::CONFIGURATION.contains(&field) {
                        held.configuration.edit(field, value);
                    } else {
                        held.policy.edit(field, value);
                    }
                }
                Some(Body::Library(held)) => held.options.edit(field, value),
                _ => {}
            }
            Task::none()
        }
        Action::Browse(path) => {
            let api = signed.api.clone();
            Task::perform(
                async move {
                    if path.is_empty() {
                        api.drives().await
                    } else {
                        api.directory(&path).await
                    }
                    .map(|entries| (path, entries))
                },
                Message::DashboardBrowsed,
            )
        }
        Action::Filtered(with_user) => {
            let api = signed.api.clone();
            Task::perform(
                async move {
                    activity::load(api, with_user, viewport)
                        .await
                        .map(|state| (Screen::Activity, Loaded::Activity(Box::new(state))))
                },
                Message::DashboardLoaded,
            )
        }
        Action::ProviderKind(schedules_direct) => {
            if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.provider.schedules_direct = schedules_direct;
            }
            Task::none()
        }
        Action::ProviderCountry(country) => {
            if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.provider.country = country;
            }
            Task::none()
        }
        Action::ProviderPostcode(postcode) => {
            if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.provider.postcode = postcode;
            }
            Task::none()
        }
        Action::ProviderLineup(lineup) => {
            if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.provider.lineup = lineup;
            }
            Task::none()
        }
        Action::TypedPassword(typed) => {
            match shown_mut(signed).map(|state| &mut state.body) {
                Some(Body::User(held)) => held.replacement = typed,
                Some(Body::Users(held)) => held.password = typed,
                Some(Body::Repositories(held)) => held.url = typed,
                Some(Body::LiveTv(held)) => match held.tab {
                    LiveTvTab::Tuners => held.tuner_type = typed,
                    _ => held.provider.password = typed,
                },
                _ => {}
            }
            Task::none()
        }
        Action::TypedCurrentPassword(typed) => {
            if let Some(Body::User(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.current = typed;
            }
            Task::none()
        }
        Action::Save => {
            let api = signed.api.clone();
            match shown_mut(signed).map(|state| &mut state.body) {
                Some(Body::User(held)) => {
                    let id = held.id;
                    let object = held.name.clone();
                    let policy = held.policy.written();
                    let configuration = held.configuration.written();
                    return Task::perform(
                        crate::error::Answer::of(async move {
                            api.save_policy(id, &policy).await.bubbled()?;
                            api.save_user_configuration(id, &configuration)
                                .await
                                .bubbled()
                        }),
                        move |outcome| {
                            Message::DashboardSaved(
                                Wrote {
                                    operation: Operation::UserSave,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    );
                }
                Some(Body::Logs(held)) => {
                    let body = held.form.written();
                    return Task::perform(
                        async move { api.save_server_configuration(&body).await },
                        move |outcome| {
                            Message::DashboardSaved(
                                Wrote {
                                    operation: Operation::Configuration,
                                    object: strings::lookup(Text::LogsTitle).to_string(),
                                },
                                outcome,
                            )
                        },
                    );
                }
                Some(Body::LiveTv(held)) => {
                    let body = held.dvr.written();
                    return Task::perform(
                        async move { api.save_section("livetv", &body).await },
                        move |outcome| {
                            Message::DashboardSaved(
                                Wrote {
                                    operation: Operation::Dvr,
                                    object: String::new(),
                                },
                                outcome,
                            )
                        },
                    );
                }
                Some(Body::Library(held)) => {
                    let id = held.id.clone();
                    let object = held.name.clone();
                    let options = held.options.written();
                    return Task::perform(
                        async move { api.save_library_options(&id, &options).await },
                        move |outcome| {
                            Message::DashboardSaved(
                                Wrote {
                                    operation: Operation::LibraryOptions,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    );
                }
                _ => {}
            }
            let Some(Body::Settings(held)) = shown_mut(signed).map(|state| &mut state.body) else {
                return Task::none();
            };
            let section = held.section;
            let body = held.form.written();
            let object = strings::lookup(section.label()).to_string();
            Task::perform(
                async move {
                    match section.key() {
                        Some(key) => api.save_section(key, &body).await,
                        None => api.save_server_configuration(&body).await,
                    }
                },
                move |outcome| {
                    Message::DashboardSaved(
                        Wrote {
                            operation: Operation::Configuration,
                            object: object.clone(),
                        },
                        outcome,
                    )
                },
            )
        }
        Action::Bridged(payload) => page::asked(signed, &payload),
        Action::ChooseImage => {
            if let Some(picker) = signed.picker.as_ref() {
                picker.choose();
            }
            Task::none()
        }
        Action::Ask(pending) => {
            if let Some(state) = shown_mut(signed) {
                state.confirming = Some(pending);
                if let Body::Users(held) = &mut state.body {
                    held.menu = None;
                }
            }
            Task::none()
        }
        Action::UserMenu(open) => {
            if let Some(Body::Users(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.menu = open;
            }
            Task::none()
        }
        Action::ContentType(choice) => {
            if let Some(Body::Libraries(held)) = shown_mut(signed).map(|state| &mut state.body) {
                held.content_type = choice;
            }
            Task::none()
        }
        Action::Typed(typed) => {
            if let Some(state) = shown_mut(signed) {
                if let Some(pending) = state.confirming.as_mut() {
                    pending.typed = typed;
                } else {
                    match &mut state.body {
                        Body::Libraries(held) => held.naming = typed,
                        Body::Library(held) => held.renaming = typed,
                        Body::Users(held) => held.naming = typed,
                        Body::Repositories(held) => held.naming = typed,
                        Body::Devices(held) => held.renaming = typed,
                        Body::Keys(held) => held.naming = typed,
                        Body::LiveTv(held) => match held.tab {
                            LiveTvTab::Tuners => held.tuner_url = typed,
                            _ if held.provider.schedules_direct => held.provider.username = typed,
                            _ => held.provider.path = typed,
                        },
                        _ => {}
                    }
                }
            }
            Task::none()
        }
        Action::Close => {
            if let Some(state) = shown_mut(signed) {
                state.confirming = None;
            }
            Task::none()
        }
        Action::Confirm => {
            let Some(state) = shown_mut(signed) else {
                return Task::none();
            };
            let Some(pending) = state
                .confirming
                .take()
                .filter(crate::screen::confirm::Pending::ready)
            else {
                return Task::none();
            };
            let api = signed.api.clone();
            let object = pending.object;
            match pending.action {
                crate::screen::confirm::Destructive::AuthorizeQuickConnect { .. }
                | crate::screen::confirm::Destructive::DeleteItem { .. } => Task::none(),
                crate::screen::confirm::Destructive::Restart => {
                    Task::perform(async move { api.restart().await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::Restart,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::Shutdown => {
                    Task::perform(async move { api.shutdown().await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::Shutdown,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::DeleteUser { id } => {
                    Task::perform(async move { api.delete_user(id).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::UserDelete,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::RemoveUserImage { id } => Task::perform(
                    async move { api.remove_user_image(id).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::UserImage,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                crate::screen::confirm::Destructive::DeleteLibrary { name } => Task::perform(
                    async move { api.delete_library(&name).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::LibraryDelete,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                crate::screen::confirm::Destructive::DeletePath { library, path } => Task::perform(
                    async move { api.remove_path(&library, &path).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::LibraryPath,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                crate::screen::confirm::Destructive::StopTask { id } => {
                    Task::perform(async move { api.stop_task(&id).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::TaskStop,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::InstallPackage {
                    name,
                    version,
                    repository,
                } => Task::perform(
                    async move { api.install_package(&name, &version, &repository).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::PackageInstall,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                crate::screen::confirm::Destructive::AddRepository { name, url } => {
                    let Some(Body::Repositories(held)) =
                        shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let mut repositories = held.repositories.clone();
                    repositories.push(jellyfin_api::types::RepositoryInfo {
                        name: Some(name),
                        url: Some(url),
                        enabled: Some(true),
                    });
                    Task::perform(
                        async move { api.save_repositories(&repositories).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::RepositoryAdd,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                crate::screen::confirm::Destructive::RemoveRepository { url } => {
                    let Some(Body::Repositories(held)) =
                        shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let repositories = held
                        .repositories
                        .iter()
                        .filter(|held| held.url.as_deref() != Some(url.as_str()))
                        .cloned()
                        .collect::<Vec<_>>();
                    Task::perform(
                        async move { api.save_repositories(&repositories).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::RepositoryRemove,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                crate::screen::confirm::Destructive::DeleteDevice { id } => {
                    device_deleted(api, Operation::DeviceDelete, id, object)
                }
                crate::screen::confirm::Destructive::DeleteOwnDevice { id } => {
                    device_deleted(api, Operation::OwnDeviceDelete, id, object)
                }
                crate::screen::confirm::Destructive::RevokeKey { key } => {
                    Task::perform(async move { api.revoke_key(&key).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::KeyRevoke,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::DeleteTuner { id } => {
                    Task::perform(async move { api.delete_tuner(&id).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::TunerDelete,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                crate::screen::confirm::Destructive::DeleteProvider { id } => Task::perform(
                    async move { api.delete_provider(&id).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::ProviderDelete,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                crate::screen::confirm::Destructive::UninstallPlugin { id, version } => {
                    Task::perform(
                        async move { api.uninstall_plugin(id, &version).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::PluginUninstall,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
            }
        }
        Action::Write(written) => {
            let api = signed.api.clone();
            match written {
                Written::ScanAll => Task::perform(async move { api.scan_all().await }, |outcome| {
                    Message::DashboardWrote(
                        Wrote {
                            operation: Operation::Scan,
                            object: String::new(),
                        },
                        outcome,
                    )
                }),
                Written::ScanLibrary { name } => {
                    let object = name.clone();
                    Task::perform(
                        async move { api.scan_library(&name).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::Scan,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::CreateLibrary { name, content_type } => {
                    let object = name.clone();
                    Task::perform(
                        async move {
                            api.create_library(&name, &content_type, &serde_json::json!({}))
                                .await
                        },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::LibraryCreate,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::RenameLibrary { name, renamed } => {
                    let object = name.clone();
                    Task::perform(
                        async move { api.rename_library(&name, &renamed).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::LibraryRename,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::AddPath { library, path } => {
                    let object = path.clone();
                    Task::perform(
                        async move { api.add_path(&library, &path).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::LibraryPath,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::SetPassword { id } => {
                    let Some(Body::User(held)) = shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let object = held.name.clone();
                    let current = held.current.clone();
                    let replacement = std::mem::take(&mut held.replacement);
                    held.current.clear();
                    Task::perform(
                        async move { api.set_password(id, Some(&current), &replacement).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::UserPassword,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::CreateUser { name, password } => {
                    let object = name.clone();
                    Task::perform(
                        async move { api.create_user(&name, &password).await.map(|_| ()) },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::UserCreate,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::StartTask { id, name } => {
                    Task::perform(async move { api.start_task(&id).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::TaskStart,
                                object: name.clone(),
                            },
                            outcome,
                        )
                    })
                }
                Written::SetTriggers { id, name } => {
                    let Some(Body::Task(held)) = shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let triggers = held.triggers.clone();
                    Task::perform(
                        async move { api.set_triggers(&id, &triggers).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::TaskTriggers,
                                    object: name.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::AddTrigger { kind } => {
                    if let Some(Body::Task(held)) = shown_mut(signed).map(|state| &mut state.body) {
                        held.triggers.push(jellyfin_api::types::TaskTriggerInfo {
                            type_: Some(kind),
                            ..Default::default()
                        });
                    }
                    Task::none()
                }
                Written::RemoveTrigger { index } => {
                    if let Some(Body::Task(held)) = shown_mut(signed).map(|state| &mut state.body)
                        && index < held.triggers.len()
                    {
                        held.triggers.remove(index);
                    }
                    Task::none()
                }
                Written::CancelInstall { package, name } => Task::perform(
                    async move { api.cancel_install(package).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::PackageCancel,
                                object: name.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                Written::SetDeviceName { id, name } => {
                    let object = name.clone();
                    Task::perform(
                        async move { api.set_device_name(&id, &name).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::DeviceRename,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::CreateKey { app } => {
                    let object = app.clone();
                    Task::perform(async move { api.create_key(&app).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::KeyCreate,
                                object: object.clone(),
                            },
                            outcome,
                        )
                    })
                }
                Written::AddTuner { url, kind } => {
                    let object = url.clone();
                    Task::perform(
                        async move { api.add_tuner(&url, &kind).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::TunerAdd,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::ResetTuner { id, name } => {
                    Task::perform(async move { api.reset_tuner(&id).await }, move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::TunerReset,
                                object: name.clone(),
                            },
                            outcome,
                        )
                    })
                }
                Written::DiscoverTuners => Task::perform(
                    async move { api.discover_tuners().await },
                    Message::DashboardDiscovered,
                ),
                Written::FetchLineups => {
                    let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let country = held.provider.country.clone();
                    let postcode = held.provider.postcode.clone();
                    Task::perform(
                        async move { api.lineups(&country, &postcode).await },
                        Message::DashboardLineups,
                    )
                }
                Written::AddProvider => {
                    let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let provider = &held.provider;
                    let body = if provider.schedules_direct {
                        ListingProvider::SchedulesDirect {
                            username: provider.username.clone(),
                            password: provider.password.clone(),
                            country: provider.country.clone(),
                            zip_code: provider.postcode.clone(),
                            listings_id: provider.lineup.clone(),
                        }
                    } else {
                        ListingProvider::Xmltv {
                            path: provider.path.clone(),
                        }
                    };
                    let object = if provider.schedules_direct {
                        provider.lineup.clone()
                    } else {
                        provider.path.clone()
                    };
                    Task::perform(
                        async move { api.add_provider(&body).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::ProviderAdd,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::MapChannel { tuner, provider } => {
                    let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body)
                    else {
                        return Task::none();
                    };
                    let named = held
                        .providers
                        .first()
                        .and_then(|held| held.id.clone())
                        .unwrap_or_default();
                    let object = tuner.clone();
                    Task::perform(
                        async move { api.map_channel(&named, &tuner, &provider).await },
                        move |outcome| {
                            Message::DashboardWrote(
                                Wrote {
                                    operation: Operation::ChannelMapping,
                                    object: object.clone(),
                                },
                                outcome,
                            )
                        },
                    )
                }
                Written::EnablePlugin { id, version, name } => Task::perform(
                    async move { api.enable_plugin(id, &version).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::PluginEnable,
                                object: name.clone(),
                            },
                            outcome,
                        )
                    },
                ),
                Written::DisablePlugin { id, version, name } => Task::perform(
                    async move { api.disable_plugin(id, &version).await },
                    move |outcome| {
                        Message::DashboardWrote(
                            Wrote {
                                operation: Operation::PluginDisable,
                                object: name.clone(),
                            },
                            outcome,
                        )
                    },
                ),
            }
        }
    }
}

/// The dashboard shown now, and nothing when another screen is.
fn shown_mut(signed: &mut Signed) -> Option<&mut State> {
    match &mut signed.view {
        crate::app::View::Dashboard(state) => Some(state.as_mut()),
        _ => None,
    }
}

/// The images the shown dashboard screen draws.
pub fn images(state: &State) -> std::collections::HashSet<crate::images::Key> {
    match &state.body {
        Body::Users(held) => users::images(held),
        _ => std::collections::HashSet::new(),
    }
}

/// The grant the shown screen holds open, which is what leaving it releases.
pub fn held_grant(view: &crate::app::View) -> Option<String> {
    match view {
        crate::app::View::Dashboard(state) => match &state.body {
            Body::Page(page) => Some(page.grant().to_owned()),
            _ => None,
        },
        _ => None,
    }
}

/// Applies the events the dashboard consumes: the session listing and the
/// scheduled tasks land in place, without moving the scroll position.
pub fn received(signed: &mut Signed, event: &jellium_protocol::Event) -> Task<Message> {
    let Some(Body::Home(held)) = shown_mut(signed).map(|state| &mut state.body) else {
        return Task::none();
    };
    match event {
        jellium_protocol::Event::Sessions { sessions } => home::sessions(held, sessions.clone()),
        jellium_protocol::Event::Tasks { tasks } => home::tasks(held, tasks.clone()),
        _ => {}
    }
    Task::none()
}

/// Applies one of the five package messages to the catalog, in place.
pub fn packaged(signed: &mut Signed, event: &jellium_protocol::Event) -> Task<Message> {
    if let Some(Body::Catalog(held)) = shown_mut(signed).map(|state| &mut state.body) {
        catalog::packaged(held, event);
    }
    Task::none()
}

/// Applies the tasks one push carried to the task list, in place.
pub fn tasked(signed: &mut Signed, tasks: &[jellium_protocol::TaskState]) -> Task<Message> {
    if let Some(Body::Tasks(held)) = shown_mut(signed).map(|state| &mut state.body) {
        tasks::tasks(held, tasks.to_vec());
    }
    Task::none()
}

/// Puts the activity entries one coalesced push carried at the front, without
/// moving the scroll position.
pub fn logged(signed: &mut Signed, entries: &[jellium_protocol::ActivityEntry]) -> Task<Message> {
    if let Some(Body::Activity(held)) = shown_mut(signed).map(|state| &mut state.body) {
        activity::prepended(held, entries.to_vec());
    }
    Task::none()
}

/// The page the activity window needs and no page already held or in flight.
pub fn fetch_if_stale(signed: &mut Signed) -> Task<Message> {
    let api = signed.api.clone();
    let Some(Body::Activity(held)) = shown_mut(signed).map(|state| &mut state.body) else {
        return Task::none();
    };
    let with_user = held.with_user;
    let Some(page) = activity::wanted(held) else {
        return Task::none();
    };
    let asked = page.clone();
    Task::perform(
        async move {
            api.activity(page.start as i32, page.len() as i32, with_user)
                .await
                .map(|(rows, _)| rows)
        },
        move |rows| Message::ActivityPaged(asked.clone(), rows),
    )
}

/// Takes the rows one activity page answered with.
pub fn paged(
    signed: &mut Signed,
    page: std::ops::Range<usize>,
    rows: Vec<jellium_protocol::ActivityEntry>,
) {
    if let Some(Body::Activity(held)) = shown_mut(signed).map(|state| &mut state.body) {
        activity::filled(held, page, rows);
    }
}

/// Applies a refresh progress in place; an event naming an item no open screen
/// shows changes nothing.
pub fn refreshed(signed: &mut Signed, items: &[jellium_protocol::Refreshed]) -> Task<Message> {
    if let Some(Body::Libraries(held)) = shown_mut(signed).map(|state| &mut state.body) {
        libraries::refreshed(held, items);
    }
    Task::none()
}

/// Takes what the filesystem browser answered with.
pub fn browsed(
    signed: &mut Signed,
    path: String,
    entries: Vec<jellyfin_api::types::FileSystemEntryInfo>,
) {
    if let Some(Body::Library(held)) = shown_mut(signed).map(|state| &mut state.body) {
        held.browsing = Some(path);
        held.entries = entries;
    }
}

/// True while the shown screen holds edits that a save has not taken.
pub fn dirty(view: &crate::app::View) -> bool {
    match view {
        crate::app::View::Dashboard(state) => match &state.body {
            Body::Settings(held) => held.form.dirty(),
            Body::Logs(held) => held.form.dirty(),
            Body::LiveTv(held) => held.dvr.dirty(),
            Body::User(held) => held.policy.dirty() || held.configuration.dirty(),
            Body::Library(held) => held.options.dirty(),
            _ => false,
        },
        _ => false,
    }
}

/// Takes the section a save answered with, clearing the edits.
pub fn saved(signed: &mut Signed) {
    match shown_mut(signed).map(|state| &mut state.body) {
        Some(Body::Settings(held)) => {
            let written = held.form.written();
            held.form.saved(written);
            held.saved = true;
        }
        Some(Body::Logs(held)) => {
            let written = held.form.written();
            held.form.saved(written);
            held.saved = true;
        }
        Some(Body::User(held)) => {
            let policy = held.policy.written();
            let configuration = held.configuration.written();
            held.policy.saved(policy);
            held.configuration.saved(configuration);
        }
        Some(Body::Library(held)) => {
            let options = held.options.written();
            held.options.saved(options);
        }
        Some(Body::LiveTv(held)) => {
            let written = held.dvr.written();
            held.dvr.saved(written);
        }
        _ => {}
    }
}

/// Discards the edits the shown screen holds, which is what leaving anyway
/// does.
pub fn abandoned(signed: &mut Signed) {
    match shown_mut(signed).map(|state| &mut state.body) {
        Some(Body::Settings(held)) => held.form.discard(),
        Some(Body::Logs(held)) => held.form.discard(),
        Some(Body::User(held)) => {
            held.policy.discard();
            held.configuration.discard();
        }
        Some(Body::Library(held)) => held.options.discard(),
        Some(Body::LiveTv(held)) => held.dvr.discard(),
        _ => {}
    }
}

/// Takes the tuners a discovery answered with.
pub fn discovered(signed: &mut Signed, found: Vec<jellyfin_api::types::TunerHostInfo>) {
    if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
        held.discovered = found;
    }
}

/// Takes the lineups the server reported for the provider being added.
pub fn lineups(signed: &mut Signed, found: Vec<jellyfin_api::types::NameIdPair>) {
    if let Some(Body::LiveTv(held)) = shown_mut(signed).map(|state| &mut state.body) {
        held.lineups = found;
    }
}

/// Takes the file the input reported for the dashboard's user screen, under the
/// same two bounds the settings region applies.
pub fn chosen(signed: &mut Signed, chosen: &crate::overlay::Chosen) -> Task<Message> {
    let refused = jellium_model::upload::refused(&chosen.mime, chosen.size);
    let crate::app::View::Dashboard(state) = &mut signed.view else {
        return Task::none();
    };
    let Body::User(user) = &mut state.body else {
        return Task::none();
    };
    if let Some(refused) = &refused {
        crate::failure::raise(crate::error::upload_refused(refused));
        return Task::none();
    }
    let id = user.id;
    let object = user.name.clone();
    let api = signed.api.clone();
    let mime = chosen.mime.clone();
    let bytes = chosen.bytes();
    let wrote = crate::error::Wrote {
        operation: crate::error::Operation::UserImage,
        object,
    };
    Task::perform(
        async move { api.upload_user_image(id, &mime, bytes).await },
        move |result| Message::DashboardWrote(wrote.clone(), result),
    )
}

/// What `/LiveTv/ListingProviders` takes.
#[derive(serde::Serialize)]
#[serde(tag = "Type")]
enum ListingProvider {
    #[serde(rename = "SchedulesDirect", rename_all = "PascalCase")]
    SchedulesDirect {
        username: String,
        password: String,
        country: String,
        zip_code: String,
        listings_id: String,
    },
    #[serde(rename = "xmltv", rename_all = "PascalCase")]
    Xmltv { path: String },
}
