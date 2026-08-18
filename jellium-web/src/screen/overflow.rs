use iced::widget::{button, column, row, text_input};
use iced::{Element, Task};
use jellium_model::item::{self, Mark, Replace, Scope};
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, SeriesTimerInfoDto, TimerInfoDto};
use uuid::Uuid;

use crate::app::{Message, Signed};
use crate::error::Operation;
use crate::icon::Icon;
use crate::screen::confirm::{Destructive, Pending};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;
use crate::widget::sheet::{Entry, Item, sheet};

/// What a card's menu was opened on, which is what `getItem` fetches by the
/// card's own type before `getCommands` reads it.
// reference: shortcut-item
pub enum Subject<'a> {
    Item(&'a BaseItemDto),
    Channel(&'a crate::livetv::Channel),
    Timer(&'a TimerInfoDto),
    SeriesTimer(&'a SeriesTimerInfoDto),
}

/// One command `getCommands` pushes, carrying what running it takes.
// reference: item-context-play
// reference: item-context-delete
// reference: detail-refresh
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Play(crate::player::Intent),
    /// `play` on a channel, which the reference tunes rather than resolves.
    PlayChannel {
        channel: Uuid,
    },
    MarkPlayed {
        item: Uuid,
        played: Mark,
    },
    Favorite {
        item: Uuid,
        favorite: Mark,
    },
    AddTo {
        item: Uuid,
        into: Into,
    },
    RemoveFrom {
        collection: Uuid,
        item: Uuid,
    },
    /// `canceltimer`, which the reference raises its own confirmation for.
    // reference: item-context-cancel-timer
    CancelTimer {
        timer: String,
        name: String,
    },
    /// `cancelseriestimer`, likewise.
    // reference: item-context-cancel-series
    CancelSeriesTimer {
        timer: String,
        name: String,
    },
    /// `delete`, which the reference raises `deleteHelper`'s confirmation for.
    Delete {
        item: Uuid,
        name: String,
    },
    Refresh {
        item: Uuid,
        replace: Replace,
        scope: Scope,
    },
}

impl Command {
    /// What the sheet writes this command as.
    pub fn label(&self) -> Text {
        match self {
            Command::Play(_) | Command::PlayChannel { .. } => Text::MenuPlay,
            Command::MarkPlayed { played, .. } => match played {
                Mark::Set => Text::OverflowMarkPlayed,
                Mark::Cleared => Text::OverflowMarkUnplayed,
            },
            Command::Favorite { favorite, .. } => match favorite {
                Mark::Set => Text::OverflowFavorite,
                Mark::Cleared => Text::OverflowUnfavorite,
            },
            Command::AddTo { into, .. } => match into {
                Into::Collection => Text::OverflowAddToCollection,
                Into::Playlist => Text::OverflowAddToPlaylist,
            },
            Command::RemoveFrom { .. } => Text::OverflowRemoveFromCollection,
            Command::CancelTimer { .. } => Text::MenuCancelRecording,
            Command::CancelSeriesTimer { .. } => Text::MenuCancelSeries,
            Command::Delete { .. } => Text::MenuDeleteMedia,
            Command::Refresh { replace, scope, .. } => match (replace, scope) {
                (Replace::Missing, Scope::Tree) => Text::DetailRefreshMetadata,
                (Replace::All, Scope::Tree) => Text::DetailRefreshReplace,
                (_, Scope::Item) => Text::DetailRefreshScanMode,
            },
        }
    }

    /// `actionSheet`'s own glyph for this command.
    pub fn glyph(&self) -> Icon {
        match self {
            Command::Play(_) | Command::PlayChannel { .. } => Icon::PlayArrow,
            Command::MarkPlayed { .. } => Icon::Check,
            Command::Favorite { favorite, .. } => match favorite {
                Mark::Set => Icon::Favorite,
                Mark::Cleared => Icon::FavoriteBorder,
            },
            Command::AddTo { .. } => Icon::PlaylistAdd,
            Command::RemoveFrom { .. } => Icon::PlaylistRemove,
            Command::CancelTimer { .. } | Command::CancelSeriesTimer { .. } => Icon::Cancel,
            Command::Delete { .. } => Icon::Delete,
            Command::Refresh { .. } => Icon::Refresh,
        }
    }

    /// The confirmation this command is asked about behind, and `None` where
    /// the reference runs it at once.
    // reference: delete-confirm
    pub fn asks(&self) -> Option<Pending> {
        match self {
            Command::Delete { item, name } => Some(Pending::of(
                Destructive::DeleteItem { id: *item },
                name.clone(),
            )),
            Command::CancelTimer { timer, name } => Some(Pending::of(
                Destructive::CancelTimer {
                    timer: timer.clone(),
                },
                name.clone(),
            )),
            Command::CancelSeriesTimer { timer, name } => Some(Pending::of(
                Destructive::CancelSeriesTimer {
                    timer: timer.clone(),
                },
                name.clone(),
            )),
            Command::Play(_)
            | Command::PlayChannel { .. }
            | Command::MarkPlayed { .. }
            | Command::Favorite { .. }
            | Command::AddTo { .. }
            | Command::RemoveFrom { .. }
            | Command::Refresh { .. } => None,
        }
    }
}

/// The commands the menu offers for `subject`, in the order `getCommands`
/// pushes them; a read-only session is offered the ones that write nothing, and
/// `collection` names the collection whose screen the card stands on.
// reference: item-context-play
// reference: item-context-delete
// reference: item-can-play
// reference: item-can-mark-played
// reference: item-can-rate
// reference: item-context-cancel-timer
// reference: item-context-cancel-series
pub fn commands(
    subject: Subject<'_>,
    session: &Session,
    collection: Option<Uuid>,
    now: chrono::DateTime<chrono::Utc>,
) -> Vec<Command> {
    let item = match subject {
        Subject::Item(item) => item,
        Subject::Channel(channel) => {
            return vec![Command::PlayChannel {
                channel: channel.id,
            }];
        }
        Subject::Timer(timer) => {
            let (Some(id), true) = (timer.id.clone(), manageable(session)) else {
                return Vec::new();
            };
            return vec![Command::CancelTimer {
                timer: id,
                name: timer
                    .program_info
                    .as_ref()
                    .and_then(|held| held.name.clone())
                    .or_else(|| timer.name.clone())
                    .unwrap_or_default(),
            }];
        }
        Subject::SeriesTimer(timer) => {
            let (Some(id), true) = (timer.id.clone(), manageable(session)) else {
                return Vec::new();
            };
            return vec![Command::CancelSeriesTimer {
                timer: id,
                name: timer.name.clone().unwrap_or_default(),
            }];
        }
    };
    let mut offered = Vec::new();
    let Some(id) = item.id else {
        return offered;
    };

    if item::playable(item, now) {
        offered.push(Command::Play(crate::player::Intent::Item {
            item: id,
            resume: true,
        }));
    }
    if session.read_only {
        return offered;
    }
    if item::markable(item) {
        offered.push(Command::MarkPlayed {
            item: id,
            played: item::played(item).flipped(),
        });
    }
    if item::ratable(item) {
        offered.push(Command::Favorite {
            item: id,
            favorite: item::favorited(item).flipped(),
        });
    }
    offered.push(Command::AddTo {
        item: id,
        into: Into::Collection,
    });
    offered.push(Command::AddTo {
        item: id,
        into: Into::Playlist,
    });
    if let Some(collection) = collection {
        offered.push(Command::RemoveFrom {
            collection,
            item: id,
        });
    }
    if let Some(timer) =
        crate::screen::livetv::recordings::writing(item).filter(|_| manageable(session))
    {
        offered.push(Command::CancelTimer {
            timer: timer.to_string(),
            name: item.name.clone().unwrap_or_default(),
        });
    }
    if item.can_delete == Some(true) {
        offered.push(Command::Delete {
            item: id,
            name: item.name.clone().unwrap_or_default(),
        });
    }
    if session.administrator {
        for (replace, scope) in [
            (Replace::Missing, Scope::Tree),
            (Replace::All, Scope::Tree),
            (Replace::Missing, Scope::Item),
        ] {
            offered.push(Command::Refresh {
                item: id,
                replace,
                scope,
            });
        }
    }
    offered
}

/// Whether this session may cancel a timer: the Live TV management the
/// reference gates both cancel commands on, and a session that writes at all.
// reference: item-context-cancel-timer
fn manageable(session: &Session) -> bool {
    session.live_tv.allowed() && !session.read_only
}

/// The menu open now; at most one is open.
#[derive(Debug, Clone)]
pub struct Open {
    pub offered: Vec<Command>,
    /// The picker open over the menu, and `None` while the menu itself shows.
    pub filing: Option<Filing>,
}

/// Filing one item into a collection or a playlist, with the option of creating
/// one without leaving the screen.
#[derive(Debug, Clone)]
pub struct Filing {
    /// The item being filed.
    pub item: Uuid,
    pub into: Into,
    /// The collections or playlists the server holds, offered to file into.
    pub offered: Vec<BaseItemDto>,
    /// The name typed to create a new one.
    pub naming: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Into {
    Collection,
    Playlist,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Open {
        offered: Vec<Command>,
    },
    Close,
    /// Runs one command the menu offered, or raises the confirmation it takes.
    Chose(Command),
    /// The object's name typed into that confirmation.
    Named(String),
    /// Runs the command the confirmation stands for.
    Confirm,
    /// Abandons the confirmation, the command unrun.
    Dismiss,
    /// The name typed into the picker's create field.
    Typed(String),
    /// Files the item into the collection or playlist named.
    File {
        target: Uuid,
    },
    /// Creates a collection or playlist holding the item alone; a playlist's
    /// media type follows from that item.
    CreateAndFile,
}

/// The menu drawn over the screen it was opened from, and the picker over the
/// menu while one is open.
// reference: action-sheet-markup
pub fn view<'a>(open: &'a Open, viewport: Viewport) -> Element<'a, Message> {
    let close = Message::OverflowAction(Action::Close);

    let Some(filing) = &open.filing else {
        let written = |command: &Command| {
            Entry::Item(Item {
                glyph: Some(command.glyph()),
                name: strings::lookup(command.label()).into(),
                secondary: None,
                aside: None,
                press: Message::OverflowAction(Action::Chose(command.clone())),
            })
        };
        let split = open
            .offered
            .iter()
            .position(|command| matches!(command, Command::Refresh { .. }))
            .unwrap_or(open.offered.len());
        let (early, late) = open.offered.split_at(split);
        let mut entries: Vec<Entry<'a>> = early.iter().map(written).collect();
        if !early.is_empty() && !late.is_empty() {
            entries.push(Entry::Divider);
        }
        entries.extend(late.iter().map(written));
        return sheet(None, None, entries, Some(close), viewport);
    };

    let offered = filing.offered.iter().filter_map(|held| {
        let target = held.id?;
        Some(Entry::Item(Item {
            glyph: None,
            name: held.name.clone().unwrap_or_default().into(),
            secondary: None,
            aside: None,
            press: Message::OverflowAction(Action::File { target }),
        }))
    });

    column![
        row![
            text_input("", &filing.naming)
                .style(style::input)
                .on_input(|typed| Message::OverflowAction(Action::Typed(typed)))
                .padding(style::drawn(space::CONTROL_GAP.drawn())),
            button(prose(
                strings::lookup(Text::OverflowCreateAndFile),
                typeface::BODY
            ))
            .style(style::submit)
            .on_press(Message::OverflowAction(Action::CreateAndFile)),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .align_y(iced::Alignment::Center),
        sheet(None, None, offered, Some(close), viewport),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .padding(style::padding(space::PAGE_PAD))
    .into()
}

/// Runs one command the menu offered; the menu is already closed, and the one
/// command that draws the picker over it re-opens it carrying `offered`.
fn run(signed: &mut Signed, offered: Vec<Command>, command: Command) -> Task<Message> {
    match command {
        Command::Play(intent) => Task::done(Message::PlayPressed(intent)),
        Command::PlayChannel { channel } => Task::done(Message::LiveTvAction(
            crate::screen::livetv::Action::PlayChannel(channel),
        )),
        Command::MarkPlayed { item, played } => Task::done(Message::PlayedToggled(item, played)),
        Command::Favorite { item, favorite } => {
            Task::done(Message::FavoriteToggled(item, favorite))
        }
        Command::AddTo { item, into } => {
            signed.overflow = Some(Open {
                offered,
                filing: Some(Filing {
                    item,
                    into,
                    offered: Vec::new(),
                    naming: String::new(),
                }),
            });
            let api = signed.api.clone();
            Task::perform(
                async move {
                    let page = match into {
                        Into::Collection => api.collections(0, OFFERED).await,
                        Into::Playlist => api.playlists(0, OFFERED).await,
                    };
                    page.map(|page| page.items)
                },
                Message::FilingLoaded,
            )
        }
        Command::RemoveFrom { collection, item } => Task::done(Message::CollectionAction(
            crate::screen::collections::Action::Remove { collection, item },
        )),
        Command::CancelTimer { timer, .. } => carried(signed, Destructive::CancelTimer { timer }),
        Command::CancelSeriesTimer { timer, .. } => {
            carried(signed, Destructive::CancelSeriesTimer { timer })
        }
        Command::Delete { item, .. } => carried(signed, Destructive::DeleteItem { id: item }),
        Command::Refresh {
            item,
            replace,
            scope,
        } => Task::done(Message::RefreshItem {
            item,
            replace,
            scope,
        }),
    }
}

/// Runs the action a confirmation the menu raised stood for.
fn carried(signed: &mut Signed, action: Destructive) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Destructive::DeleteItem { id } => {
            Task::perform(async move { api.delete_item(id).await }, |wrote| {
                Message::Wrote(Operation::ItemDelete, wrote)
            })
        }
        Destructive::CancelTimer { timer } => Task::done(Message::LiveTvAction(
            crate::screen::livetv::Action::CancelTimer(timer),
        )),
        Destructive::CancelSeriesTimer { timer } => Task::done(Message::LiveTvAction(
            crate::screen::livetv::Action::CancelSeriesTimer(timer),
        )),
        _ => Task::none(),
    }
}

pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Open { offered } => {
            signed.overflow = Some(Open {
                offered,
                filing: None,
            });
            Task::none()
        }
        Action::Close => {
            signed.overflow = None;
            Task::none()
        }
        Action::Chose(command) => {
            let Some(open) = signed.overflow.take() else {
                return Task::none();
            };
            match command.asks() {
                Some(pending) => {
                    signed.confirming = Some(pending);
                    Task::none()
                }
                None => run(signed, open.offered, command),
            }
        }
        Action::Named(typed) => {
            if let Some(pending) = signed.confirming.as_mut() {
                pending.typed = typed;
            }
            Task::none()
        }
        Action::Confirm => {
            let Some(pending) = signed.confirming.take() else {
                return Task::none();
            };
            carried(signed, pending.action)
        }
        Action::Dismiss => {
            signed.confirming = None;
            Task::none()
        }
        Action::Typed(typed) => {
            if let Some(filing) = signed
                .overflow
                .as_mut()
                .and_then(|open| open.filing.as_mut())
            {
                filing.naming = typed;
            }
            Task::none()
        }
        Action::File { target } => {
            let Some(filing) = signed
                .overflow
                .as_ref()
                .and_then(|open| open.filing.as_ref())
            else {
                return Task::none();
            };
            let (item, into) = (filing.item, filing.into);
            signed.overflow = None;
            Task::perform(
                async move {
                    match into {
                        Into::Collection => api.add_to_collection(target, &[item]).await,
                        Into::Playlist => api.add_to_playlist(target, &[item]).await,
                    }
                },
                move |wrote| {
                    Message::Wrote(
                        match into {
                            Into::Collection => Operation::CollectionAdd,
                            Into::Playlist => Operation::PlaylistAdd,
                        },
                        wrote,
                    )
                },
            )
        }
        Action::CreateAndFile => {
            let Some(filing) = signed
                .overflow
                .as_ref()
                .and_then(|open| open.filing.as_ref())
            else {
                return Task::none();
            };
            let name = filing.naming.clone();
            if name.trim().is_empty() {
                return Task::none();
            }
            let (item, into) = (filing.item, filing.into);
            signed.overflow = None;
            Task::perform(
                async move {
                    match into {
                        Into::Collection => api.create_collection(&name, &[item]).await.map(|_| ()),
                        Into::Playlist => api.create_playlist(&name, &[item]).await.map(|_| ()),
                    }
                },
                move |wrote| {
                    Message::Wrote(
                        match into {
                            Into::Collection => Operation::CollectionCreate,
                            Into::Playlist => Operation::PlaylistCreate,
                        },
                        wrote,
                    )
                },
            )
        }
    }
}

/// The most collections or playlists a picker offers.
const OFFERED: i32 = 200;
