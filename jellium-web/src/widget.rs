mod line;

use std::borrow::Cow;
use std::collections::HashSet;

pub use line::Line;

use iced::widget::{
    Space, button, column, container, grid as grid_widget, image, row, scrollable, text,
};
use iced::{Element, Fill, Length};
use jellium_protocol::{Notice, Session, SyncAccess};
use jellyfin_api::types::BaseItemDto;

use crate::app::Message;
use crate::images::{self, Cache, Kind};
use crate::live;
use crate::livetv::Channel;
use crate::player::group::Joined;
use crate::route::Route;
use crate::style::{self, Drawn, Share, space, typeface};
use crate::text::{self as strings, Text};
use crate::theme;

/// Wrapping text, which is what a server's own disclaimer is. Every string the
/// client draws passes here, so the coverage it needs is observed once. A
/// sentence out of the string table arrives borrowed and is drawn as it lies.
pub fn prose<'a>(content: impl Into<Cow<'a, str>>, size: style::Length) -> Element<'a, Message> {
    let content = content.into();
    crate::fonts::observed(&content, typeface::Weight::Regular);
    text(content).size(style::drawn(size.drawn())).into()
}

/// One line of text, cut with an ellipsis at the width it is given, which is
/// what the reference does to a list row's title and its secondary line.
pub fn line<'a>(
    content: impl Into<Cow<'a, str>>,
    size: style::Length,
    weight: typeface::Weight,
) -> Element<'a, Message> {
    let content = content.into();
    crate::fonts::observed(&content, weight);
    Line::new(content.into_owned(), size, weight).into()
}

/// One option a picker offers: what it shows, and the value a write sends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub label: String,
    pub value: String,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

pub fn poster_key(item: &BaseItemDto) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: Kind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    })
}

pub fn card_images(items: &[BaseItemDto]) -> HashSet<images::Key> {
    items.iter().filter_map(poster_key).collect()
}

/// The route a card opens: a collection and a playlist each have their own
/// screen, and every other item opens its detail.
fn opens(item: &BaseItemDto, id: uuid::Uuid) -> Route {
    match item.type_ {
        Some(jellyfin_api::types::BaseItemKind::BoxSet) => Route::Collection {
            id,
            listing: Box::default(),
        },
        Some(jellyfin_api::types::BaseItemKind::Playlist) => Route::Playlist { id },
        _ => Route::Detail { id },
    }
}

fn subtitle(item: &BaseItemDto) -> String {
    match (&item.series_name, item.production_year) {
        (Some(series), _) => series.clone(),
        (None, Some(year)) => year.to_string(),
        (None, None) => String::new(),
    }
}

/// A card, with an overflow menu when `overflow`; the menu is absent entirely
/// under read-only.
pub fn card<'a>(
    item: &'a BaseItemDto,
    image: Option<image::Handle>,
    overflow: bool,
) -> Element<'a, Message> {
    let poster: Element<'a, Message> = match image {
        Some(handle) => iced::widget::image(handle).width(theme::CARD_WIDTH).into(),
        None => container(prose("", typeface::BODY))
            .width(theme::CARD_WIDTH)
            .height(theme::CARD_WIDTH * 1.5)
            .into(),
    };

    let body = column![
        poster,
        prose(item.name.clone().unwrap_or_default(), typeface::BODY),
        prose(subtitle(item), typeface::SECONDARY),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .width(theme::CARD_WIDTH);

    let pressed = item.id.map(|id| Message::Navigated(opens(item, id)));
    let mut control = button(body).style(style::flat);
    if let Some(message) = pressed {
        control = control.on_press(message);
    }

    let Some(id) = item.id.filter(|_| overflow) else {
        return control.into();
    };
    let held = item.user_data.as_ref();
    column![
        control,
        button(prose(strings::lookup(Text::OverflowOpen), typeface::BODY))
            .style(style::flat)
            .on_press(Message::OverflowAction(
                crate::screen::overflow::Action::Open {
                    item: id,
                    played: held.and_then(|held| held.played).unwrap_or(false),
                    favorite: held.and_then(|held| held.is_favorite).unwrap_or(false),
                }
            )),
    ]
    .into()
}

fn cards<'a>(
    items: &'a [BaseItemDto],
    images: &'a Cache,
    overflow: bool,
) -> Vec<Element<'a, Message>> {
    items
        .iter()
        .map(|item| {
            card(
                item,
                poster_key(item).and_then(|key| images.handle(key)),
                overflow,
            )
        })
        .collect()
}

pub fn rail<'a>(
    title: Text,
    items: &'a [BaseItemDto],
    images: &'a Cache,
    overflow: bool,
) -> Element<'a, Message> {
    strip(
        prose(strings::lookup(title), typeface::HEADING_2),
        items,
        images,
        overflow,
    )
}

/// A rail under a heading the server named rather than one this client holds.
pub fn named_rail<'a>(
    title: &'a str,
    items: &'a [BaseItemDto],
    images: &'a Cache,
    overflow: bool,
) -> Element<'a, Message> {
    strip(prose(title, typeface::HEADING_2), items, images, overflow)
}

fn strip<'a>(
    title: Element<'a, Message>,
    items: &'a [BaseItemDto],
    images: &'a Cache,
    overflow: bool,
) -> Element<'a, Message> {
    column![
        title,
        scrollable(
            row(cards(items, images, overflow)).spacing(style::drawn(space::GUTTER.drawn()))
        )
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default(),
        ))
        .height(theme::RAIL_HEIGHT),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .into()
}

pub fn grid<'a>(
    items: &'a [BaseItemDto],
    images: &'a Cache,
    overflow: bool,
) -> Element<'a, Message> {
    scrollable(
        grid_widget(cards(items, images, overflow))
            .fluid(theme::CARD_WIDTH)
            .spacing(style::drawn(space::GUTTER.drawn())),
    )
    .height(Fill)
    .into()
}

/// The libraries, with a Live TV entry ahead of them when `live_tv` and the
/// Collections and Playlists destinations after them.
pub fn library_row<'a>(
    libraries: impl IntoIterator<Item = &'a BaseItemDto>,
    live_tv: bool,
    collections: bool,
) -> Element<'a, Message> {
    let live_entry: Vec<Element<'a, Message>> = if live_tv {
        vec![
            button(prose(strings::lookup(Text::HomeLiveTv), typeface::BODY))
                .on_press(Message::Navigated(Route::LiveTv {
                    tab: crate::screen::livetv::Tab::Guide,
                }))
                .into(),
        ]
    } else {
        Vec::new()
    };
    let entries = libraries.into_iter().filter_map(|library| {
        let id = library.id?;
        Some(
            button(prose(
                library.name.clone().unwrap_or_default(),
                typeface::BODY,
            ))
            .on_press(Message::Navigated(Route::Library {
                id,
                tab: Box::new(crate::screen::library::Tab::Items(Box::default())),
            }))
            .into(),
        )
    });

    let mut destinations: Vec<Element<'a, Message>> = Vec::new();
    if collections {
        destinations.push(
            button(prose(strings::lookup(Text::NavCollections), typeface::BODY))
                .on_press(Message::Navigated(Route::Collections))
                .into(),
        );
    }
    destinations.push(
        button(prose(strings::lookup(Text::NavPlaylists), typeface::BODY))
            .on_press(Message::Navigated(Route::Playlists))
            .into(),
    );

    scrollable(
        row(live_entry.into_iter().chain(entries).chain(destinations))
            .spacing(style::drawn(space::GUTTER.drawn())),
    )
    .direction(scrollable::Direction::Horizontal(
        scrollable::Scrollbar::default(),
    ))
    .into()
}

/// The portions a bar is divided into, which is the resolution a share the
/// reference writes is exact to.
const PORTIONS: u16 = 10_000;

/// A bar filled to `elapsed`, which is how far through a program `now` is.
pub fn elapsed_bar<'a>(elapsed: Share) -> Element<'a, Message> {
    let filled = style::drawn(elapsed.of(Drawn::of(f32::from(PORTIONS)))) as u16;
    container(
        row![
            container(Space::new())
                .width(Length::FillPortion(filled))
                .height(style::drawn(space::PROGRESS.drawn()))
                .style(|theme: &iced::Theme| container::Style::default()
                    .background(theme.palette().primary)),
            container(Space::new())
                .width(Length::FillPortion(PORTIONS - filled))
                .height(style::drawn(space::PROGRESS.drawn())),
        ]
        .height(style::drawn(space::PROGRESS.drawn())),
    )
    .width(Length::Fill)
    .height(style::drawn(space::PROGRESS.drawn()))
    .into()
}

/// One on-now card: the channel's logo, its number, and its current program's
/// title with an elapsed bar.
pub fn channel_card<'a>(
    channel: &'a Channel,
    now: chrono::DateTime<chrono::Utc>,
    image: Option<image::Handle>,
) -> Element<'a, Message> {
    let logo: Element<'a, Message> = match image {
        Some(handle) => iced::widget::image(handle).width(theme::CARD_WIDTH).into(),
        None => container(prose("", typeface::BODY))
            .width(theme::CARD_WIDTH)
            .height(theme::CARD_WIDTH * 0.6)
            .into(),
    };

    let mut body = column![
        logo,
        prose(
            format!("{} {}", channel.number, channel.name),
            typeface::BODY
        ),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .width(theme::CARD_WIDTH);

    if let Some(program) = &channel.current {
        body = body.push(prose(program.title.clone(), typeface::SECONDARY));
        body = body.push(elapsed_bar(program.elapsed(now)));
    }

    button(body)
        .style(style::flat)
        .on_press(Message::LiveTvAction(
            crate::screen::livetv::Action::PlayChannel(channel.id),
        ))
        .into()
}

/// The on-now row, capped at `home::ON_NOW`.
pub fn on_now_row<'a>(
    channels: &'a [Channel],
    now: chrono::DateTime<chrono::Utc>,
    images: &'a Cache,
) -> Element<'a, Message> {
    let cards = channels
        .iter()
        .take(crate::screen::home::ON_NOW as usize)
        .map(|channel| {
            let handle = images
                .handle(images::Key {
                    item: channel.id,
                    kind: Kind::Primary,
                    index: None,
                    width: theme::IMAGE_WIDTH,
                })
                .clone();
            channel_card(channel, now, handle)
        });

    column![
        prose(strings::lookup(Text::HomeOnNow), typeface::HEADING_2),
        scrollable(row(cards).spacing(style::drawn(space::GUTTER.drawn()))).direction(
            scrollable::Direction::Horizontal(scrollable::Scrollbar::default(),)
        ),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .into()
}

/// One failure report as it is shown: its sentence, the Jellyfin server's own
/// message beneath it as quoted server output under the `serverSaid` label,
/// and the control that dismisses it.
/// Private, so the only failure text any screen can render is the log's.
fn failure<'a>(failure: &'a crate::failure::Failure) -> Element<'a, Message> {
    let mut shown = column![prose(failure.sentence.clone(), typeface::BODY)]
        .spacing(style::drawn(space::GUTTER.drawn()));
    if let Some(server) = &failure.server {
        shown = shown
            .push(prose(
                strings::lookup(Text::ServerSaid),
                typeface::SECONDARY,
            ))
            .push(prose(format!("> {server}"), typeface::SECONDARY));
    }
    shown = shown.push(
        button(prose(strings::lookup(Text::FailureDismiss), typeface::BODY))
            .on_press(Message::FailureDismissed),
    );
    container(shown)
        .padding(style::drawn(space::GUTTER.drawn()))
        .width(Length::Fill)
        .into()
}

/// The terminal stage's screen: the failure that ended the session, over the
/// control that returns to a fresh login screen.
pub fn lost<'a>(failure: &'a crate::failure::Failure) -> Element<'a, Message> {
    let mut shown = column![prose(failure.sentence.clone(), typeface::BODY)]
        .spacing(style::drawn(space::GUTTER.drawn()));
    if let Some(server) = &failure.server {
        shown = shown
            .push(prose(
                strings::lookup(Text::ServerSaid),
                typeface::SECONDARY,
            ))
            .push(prose(format!("> {server}"), typeface::SECONDARY));
    }
    shown = shown.push(
        button(prose(
            strings::lookup(Text::FailureSignInAgain),
            typeface::BODY,
        ))
        .on_press(Message::SignInAgain),
    );
    container(shown)
        .padding(style::drawn(space::GUTTER.drawn()))
        .width(Length::Fill)
        .into()
}

/// Every screen under the failure surfaces: the report shown above it until it
/// is dismissed, the control opening the session's failure list, and the list
/// itself while it is open.
pub fn shell<'a>(
    failures: &'a crate::failure::Log,
    listing: bool,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::GUTTER.drawn()));
    if let Some(showing) = failures.showing() {
        page = page.push(self::failure(showing));
    }
    page = page.push(
        button(prose(strings::lookup(Text::FailuresOpen), typeface::BODY)).on_press(if listing {
            Message::FailuresClosed
        } else {
            Message::FailuresOpened
        }),
    );
    if listing {
        let mut listed = column![prose(strings::lookup(Text::FailuresTitle), typeface::BODY)]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .padding(style::drawn(space::GUTTER.drawn()));
        if failures.raised().is_empty() {
            listed = listed.push(prose(strings::lookup(Text::FailuresEmpty), typeface::BODY));
        }
        for raised in failures.raised() {
            let mut one = column![prose(raised.sentence.clone(), typeface::BODY)];
            if let Some(server) = &raised.server {
                one = one.push(prose(format!("> {server}"), typeface::SECONDARY));
            }
            listed = listed.push(one.spacing(style::drawn(space::BLOCK_GAP.drawn())));
        }
        page = page.push(scrollable(listed));
    }
    page.push(body).into()
}

/// The nav row — with controls opening `/settings`, `/remote` and, for a user
/// whose access allows it, `/syncplay` — the off-snapshot warning, the group
/// indicator and the waiting indicator while membership lasts, the live-updates
/// indicator while `live` is down, above `body`.
/// `browse` is false while a settings route is on top, which is what the
/// settings column replaces; Back and Logout stand either way.
pub fn chrome<'a>(
    session: &'a Session,
    back: bool,
    browse: bool,
    live: live::Link,
    group: Option<&'a Joined>,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut nav = row![].spacing(style::drawn(space::GUTTER.drawn()));

    if back {
        nav = nav.push(
            button(prose(strings::lookup(Text::NavBack), typeface::BODY))
                .on_press(Message::WentBack),
        );
    }
    if browse {
        nav = nav
            .push(
                button(prose(strings::lookup(Text::NavHome), typeface::BODY))
                    .on_press(Message::Navigated(Route::Home)),
            )
            .push(
                button(prose(strings::lookup(Text::NavSearch), typeface::BODY)).on_press(
                    Message::Navigated(Route::Search {
                        term: String::new(),
                        listing: Box::default(),
                    }),
                ),
            )
            .push(
                button(prose(strings::lookup(Text::NavSettings), typeface::BODY)).on_press(
                    Message::Navigated(Route::Settings {
                        screen: crate::screen::settings::Screen::Profile,
                    }),
                ),
            )
            .push(
                button(prose(strings::lookup(Text::NavRemote), typeface::BODY))
                    .on_press(Message::Navigated(Route::Remote)),
            );

        if session.sync_play != SyncAccess::None {
            nav = nav.push(
                button(prose(strings::lookup(Text::NavSyncPlay), typeface::BODY))
                    .on_press(Message::Navigated(Route::SyncPlay)),
            );
        }
        if session.administrator {
            nav = nav.push(
                button(prose(strings::lookup(Text::NavDashboard), typeface::BODY)).on_press(
                    Message::Navigated(Route::Dashboard {
                        screen: crate::screen::dashboard::Screen::Plugins,
                    }),
                ),
            );
        }
    }
    nav = nav.push(
        button(prose(strings::lookup(Text::NavSwitch), typeface::BODY))
            .on_press(Message::SwitchPressed),
    );
    if !session.read_only {
        nav = nav.push(
            button(prose(strings::lookup(Text::NavLogout), typeface::BODY))
                .on_press(Message::LogoutPressed),
        );
    }

    let mut page = column![nav]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    if session.read_only {
        page = page.push(self::banner(
            strings::lookup(Text::DashboardReadOnly).to_string(),
        ));
    }

    if session.off_snapshot() {
        page = page.push(self::banner(strings::format(
            Text::WarningOffSnapshot,
            &[&session.server_version, &session.snapshot_version],
        )));
    }

    if let Some(joined) = group {
        page = page.push(self::banner(
            strings::lookup(Text::SyncPlayActive).to_string(),
        ));
        if joined.waiting() {
            page = page.push(self::banner(
                strings::lookup(Text::SyncPlayWaiting).to_string(),
            ));
        }
    }

    if live.down() {
        page = page.push(self::banner(
            strings::lookup(Text::LiveUnavailable).to_string(),
        ));
    }

    page.push(body).into()
}

/// One sentence shown above a screen.
pub fn banner<'a>(message: String) -> Element<'a, Message> {
    container(prose(message, typeface::BODY))
        .padding(style::drawn(space::GUTTER.drawn()))
        .width(Length::Fill)
        .into()
}

/// The transient notice a `DisplayMessage` renders: its header, then its text.
pub fn message<'a>(notice: &'a Notice) -> Element<'a, Message> {
    column![
        prose(notice.header.clone(), typeface::BODY),
        prose(notice.text.clone(), typeface::BODY)
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()))
    .into()
}

/// The warning raised before leaving a screen holding unsaved edits: what is
/// lost, and the two controls.
pub fn leaving<'a>() -> Element<'a, Message> {
    column![
        prose(strings::lookup(Text::DashboardUnsaved), typeface::BODY),
        row![
            button(prose(
                strings::lookup(Text::DashboardLeaveAnyway),
                typeface::BODY
            ))
            .on_press(Message::LeaveAnyway),
            button(prose(
                strings::lookup(Text::DashboardStayHere),
                typeface::BODY
            ))
            .on_press(Message::StayHere),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()))
    .into()
}
