//! The page every dashboard screen stands in, and the navigation drawer that
//! stands beside it.

use std::borrow::Cow;
use std::collections::BTreeSet;

use iced::widget::{Space, column, container, row};
use iced::{Element, Fill};

use super::{Action, LiveTvTab, Screen, Section};
use crate::app::Message;
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Held, Link, Rung, Showing};

/// A group of drawer entries the reference lets the reader open and close.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Libraries,
    Playback,
}

impl Group {
    /// The group `screen` stands in, which is the group the drawer opens with
    /// while that screen is shown.
    // reference: drawer-server
    pub fn of(screen: &Screen) -> Option<Group> {
        Group::ALL
            .into_iter()
            .find(|group| group.held().contains(screen))
    }

    const ALL: [Group; 2] = [Group::Libraries, Group::Playback];

    pub fn glyph(self) -> crate::icon::Icon {
        match self {
            Group::Libraries => crate::icon::Icon::LibraryAdd,
            Group::Playback => crate::icon::Icon::PlayCircle,
        }
    }

    pub fn label(self) -> Text {
        match self {
            Group::Libraries => Text::LibrariesTitle,
            Group::Playback => Text::SettingsPlayback,
        }
    }

    /// The screens the group holds, in the order the reference stands them.
    // reference: drawer-server
    pub fn held(self) -> Vec<Screen> {
        match self {
            Group::Libraries => vec![Screen::Libraries],
            Group::Playback => vec![
                Screen::Settings {
                    section: Section::Transcoding,
                },
                Screen::Settings {
                    section: Section::Resume,
                },
                Screen::Settings {
                    section: Section::Streaming,
                },
                Screen::Settings {
                    section: Section::Trickplay,
                },
            ],
        }
    }
}

/// One destination of the drawer, drawn as shown where `shown` stands under it.
fn link(shown: &Screen, reached: Screen) -> Link {
    let showing = match shown.under() == reached {
        true => Showing::Shown,
        false => Showing::Offered(Message::DashboardAction(Action::Open(reached.clone()))),
    };
    Link {
        glyph: reached.glyph(),
        label: reached.label(),
        showing,
    }
}

/// The drawer's rows, in the order the reference's own five sections stand
/// them, with the row the shown screen stands under drawn as shown.
// reference: drawer-sections
// reference: drawer-server
// reference: drawer-devices
// reference: drawer-livetv
// reference: drawer-plugins
// reference: drawer-advanced
pub fn drawer(shown: &Screen, opened: &BTreeSet<Group>) -> Vec<Rung> {
    let mut rungs = vec![
        Rung::Reached(link(shown, Screen::Home)),
        Rung::Reached(link(
            shown,
            Screen::Settings {
                section: Section::General,
            },
        )),
        Rung::Reached(link(
            shown,
            Screen::Settings {
                section: Section::Branding,
            },
        )),
        Rung::Reached(link(shown, Screen::Users)),
    ];
    for group in Group::ALL {
        rungs.push(Rung::Group {
            glyph: group.glyph(),
            label: group.label(),
            press: Message::DashboardAction(Action::Opened(group)),
            showing: match opened.contains(&group) {
                true => Held::Shown,
                false => Held::Hidden,
            },
            held: group
                .held()
                .into_iter()
                .map(|screen| link(shown, screen))
                .collect(),
        });
    }
    rungs.extend([
        Rung::Reached(link(shown, Screen::Devices)),
        Rung::Reached(link(shown, Screen::Activity)),
        Rung::Reached(link(
            shown,
            Screen::LiveTv {
                tab: LiveTvTab::Tuners,
            },
        )),
        Rung::Reached(link(
            shown,
            Screen::LiveTv {
                tab: LiveTvTab::Dvr,
            },
        )),
        Rung::Reached(link(shown, Screen::Plugins)),
        Rung::Reached(link(
            shown,
            Screen::Settings {
                section: Section::Networking,
            },
        )),
        Rung::Reached(link(shown, Screen::Keys)),
        Rung::Reached(link(shown, Screen::Logs)),
        Rung::Reached(link(shown, Screen::Tasks)),
    ]);
    rungs
}

/// What a dashboard screen fills its page with.
pub enum Filling<'a> {
    Stacked(Vec<Element<'a, Message>>),
    Tabled {
        subtitle: Option<Text>,
        table: widget::table::Table<'a>,
    },
}

/// The page every dashboard screen stands in: the drawer beside the content on
/// a page wide enough for it and absent below that, `.content-primary`'s own
/// top and side padding, the screen's title over what it fills the page with,
/// and the rhythm a stacked screen's content stacks at. A tabled screen stands
/// its one table under that title and the line under it.
// reference: dashboard-frame
// reference: dashboard-content
// reference: dashboard-content-side
// reference: table-page
pub fn frame<'a>(
    shown: &Screen,
    opened: &BTreeSet<Group>,
    title: impl Into<Cow<'a, str>>,
    filling: Filling<'a>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let band = viewport.band();
    let filled: Element<'a, Message> = match filling {
        Filling::Stacked(content) => {
            let mut stacked = vec![widget::heading(title)];
            stacked.extend(content);
            widget::scrolled(
                column(stacked).spacing(style::drawn(space::DASHBOARD_GAP.drawn(band))),
            )
            .into()
        }
        Filling::Tabled { subtitle, table } => {
            let mut titled = vec![widget::heading(title)];
            if let Some(subtitle) = subtitle {
                titled.push(widget::prose(strings::lookup(subtitle), typeface::BODY));
            }
            column![
                column(titled).spacing(style::drawn(space::TABLE_TITLE_GAP.drawn(band))),
                Space::new().height(style::drawn(space::TABLE_TITLE_BOTTOM.drawn(band))),
                widget::table::drawn(table, band),
            ]
            .height(Fill)
            .into()
        }
    };
    let primary = container(filled).width(Fill).height(Fill).padding(
        iced::Padding::ZERO
            .top(style::drawn(space::DASHBOARD_TOP.drawn()))
            .right(style::drawn(space::DASHBOARD_SIDE.drawn()))
            .left(style::drawn(space::DASHBOARD_SIDE.drawn())),
    );

    if !viewport.matches(space::DRAWER_BESIDE_AT) {
        return primary.into();
    }
    row![widget::drawer(drawer(shown, opened), band), primary]
        .width(Fill)
        .height(Fill)
        .into()
}
