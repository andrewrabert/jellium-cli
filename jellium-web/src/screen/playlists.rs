use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{button, checkbox, column, row, text_input};
use iced::{Element, Task};
use jellium_model::paged::Paged;
use jellium_model::window;
use jellyfin_api::types::{BaseItemDto, CollectionType};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::screen::browse::{self, Browse};
use crate::style::{self, Drawn, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::{line, prose};

/// The playlists destination: every playlist, windowed, with the create control
/// absent under read-only.
#[derive(Debug, Clone)]
pub struct Listed {
    pub browse: Browse,
    pub naming: String,
}

/// One playlist: its entries in playlist order, each addressed by entry id so
/// two copies of one item are told apart.
#[derive(Debug, Clone)]
pub struct State {
    pub playlist: BaseItemDto,
    pub window: window::Window,
    pub entries: Paged<Entry>,
    pub naming: String,
    /// Whether this user may edit this playlist, which is what puts the
    /// reorder, removal and sharing controls on screen.
    pub editable: bool,
    /// Open access, and the users this playlist is shared with.
    pub sharing: Sharing,
}

impl State {
    /// The page the window wants that is neither held nor in flight.
    pub fn wanted(&self) -> Option<std::ops::Range<usize>> {
        self.entries.wanted(self.window.built(self.entries.len()))
    }
}

/// One entry of a playlist: the item filed and the entry id distinguishing this
/// copy of it.
#[derive(Debug, Clone)]
pub struct Entry {
    pub item: BaseItemDto,
    pub entry: String,
}

/// A playlist's sharing model: open to everyone, and the per-user edit list.
#[derive(Debug, Clone, Default)]
pub struct Sharing {
    pub open: bool,
    pub users: Vec<Shared>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shared {
    pub user: Uuid,
    pub name: String,
    pub can_edit: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Typed(String),
    Create,
    Rename {
        id: Uuid,
    },
    Delete {
        id: Uuid,
    },
    /// Removes one copy, leaving any other copy of the same item in place.
    Remove {
        playlist: Uuid,
        entry: String,
    },
    Move {
        playlist: Uuid,
        entry: String,
        to: usize,
    },
    PlayAll {
        id: Uuid,
        shuffle: bool,
    },
    /// Plays one entry and queues the remainder of the playlist after it.
    PlayFrom {
        playlist: Uuid,
        index: usize,
    },
    SetOpen {
        playlist: Uuid,
        open: bool,
    },
    Share {
        playlist: Uuid,
        user: Uuid,
        can_edit: bool,
    },
    Unshare {
        playlist: Uuid,
        user: Uuid,
    },
}

pub async fn listed(
    api: Rc<Api>,
    viewport: Viewport,
    overflow: widget::Overflow,
) -> Answer<Listed> {
    Answer::of(async {
        let heading = strings::lookup(Text::NavPlaylists).to_string();
        let mut browse = Browse::new(
            window::Id::Browse,
            heading,
            Listing::default(),
            Some(CollectionType::Playlists),
            viewport,
            overflow,
        );
        let answered = api
            .playlists(0, Paged::<BaseItemDto>::PAGE as i32)
            .await
            .bubbled()?;
        browse.items = Paged::new(answered.total.max(0) as usize);
        browse.filled(0..answered.items.len(), answered.items);

        Ok(Listed {
            browse,
            naming: String::new(),
        })
    })
    .await
}

pub async fn load(api: Rc<Api>, playlist: Uuid, user: Uuid, viewport: Viewport) -> Answer<State> {
    Answer::of(async {
        let held = api.item(playlist).await.bubbled()?;
        let answered = api
            .playlist_entries(playlist, 0, Paged::<Entry>::PAGE as i32)
            .await
            .bubbled()?;
        let mut entries = Paged::new(answered.total.max(0) as usize);
        entries.filled(0..answered.entries.len(), answered.entries);

        let sharing = api
            .playlist_sharing(playlist)
            .await
            .or_default(Text::FailurePlaylistSharingUnread);
        let editable = sharing.open
            || held.id == Some(playlist)
                && sharing
                    .users
                    .iter()
                    .any(|shared| shared.user == user && shared.can_edit);

        Ok(State {
            playlist: held,
            window: window::Window::new(
                window::Id::Entries,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                viewport.canvas().height(),
            ),
            entries,
            naming: String::new(),
            editable,
            sharing: Sharing {
                open: sharing.open,
                users: sharing.users,
            },
        })
    })
    .await
}

/// One page of a playlist's entries, and the total the server reports.
pub async fn page(
    api: Rc<Api>,
    playlist: Uuid,
    page: std::ops::Range<usize>,
) -> Answer<(Vec<Entry>, usize)> {
    Answer::of(async {
        let answered = api
            .playlist_entries(playlist, page.start as i32, page.len() as i32)
            .await
            .bubbled()?;
        Ok((answered.entries, answered.total.max(0) as usize))
    })
    .await
}

fn naming<'a>(held: &'a str, label: Text, apply: Message) -> Element<'a, Message> {
    row![
        text_input("", held)
            .style(style::input)
            .on_input(|typed| Message::PlaylistAction(Action::Typed(typed)))
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
        button(prose(strings::lookup(label), typeface::BODY))
            .style(style::submit)
            .on_press(apply),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

pub fn view_listed<'a>(
    state: &'a Listed,
    viewport: Viewport,
    images: &'a Cache,
    read_only: bool,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::GUTTER.drawn()));
    if !read_only {
        page = page.push(naming(
            &state.naming,
            Text::PlaylistCreate,
            Message::PlaylistAction(Action::Create),
        ));
    }
    page.push(browse::view(&state.browse, viewport, images))
        .into()
}

/// One entry row: its position, its name, and the reorder and removal controls
/// when this user may edit.
fn entry_row<'a>(
    state: &'a State,
    playlist: Uuid,
    index: usize,
    editable: bool,
) -> Element<'a, Message> {
    let Some(entry) = state.entries.row(index) else {
        return iced::widget::Space::new()
            .height(style::drawn(space::LIST_ROW.drawn()))
            .into();
    };

    let mut held = row![
        prose(format!("{}", index + 1), typeface::BODY),
        button(line(
            entry.item.name.clone().unwrap_or_default(),
            typeface::BODY,
            typeface::Weight::Regular,
        ))
        .style(style::flat)
        .on_press(Message::PlaylistAction(Action::PlayFrom {
            playlist,
            index
        })),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .align_y(iced::Alignment::Center);

    if editable {
        let entry_id = entry.entry.clone();
        if index > 0 {
            held = held.push(
                button(prose(strings::lookup(Text::PlaylistMoveUp), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::PlaylistAction(Action::Move {
                        playlist,
                        entry: entry_id.clone(),
                        to: index - 1,
                    })),
            );
        }
        if index + 1 < state.entries.len() {
            held = held.push(
                button(prose(
                    strings::lookup(Text::PlaylistMoveDown),
                    typeface::BODY,
                ))
                .style(style::raised)
                .on_press(Message::PlaylistAction(Action::Move {
                    playlist,
                    entry: entry_id.clone(),
                    to: index + 1,
                })),
            );
        }
        held = held.push(
            button(prose(strings::lookup(Text::PlaylistRemove), typeface::BODY))
                .style(style::raised)
                .on_press(Message::PlaylistAction(Action::Remove {
                    playlist,
                    entry: entry_id,
                })),
        );
    }

    held.into()
}

pub fn view<'a>(state: &'a State, images: &'a Cache, read_only: bool) -> Element<'a, Message> {
    let Some(id) = state.playlist.id else {
        return column![].into();
    };
    let editable = state.editable && !read_only;

    let mut page = column![
        prose(
            state.playlist.name.clone().unwrap_or_default(),
            typeface::HEADING_1
        ),
        row![
            button(prose(strings::lookup(Text::DetailPlayAll), typeface::BODY))
                .style(style::raised)
                .on_press(Message::PlaylistAction(Action::PlayAll {
                    id,
                    shuffle: false
                })),
            button(prose(strings::lookup(Text::DetailShuffle), typeface::BODY))
                .style(style::raised)
                .on_press(Message::PlaylistAction(Action::PlayAll {
                    id,
                    shuffle: true
                })),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()));

    if editable {
        page = page
            .push(naming(
                &state.naming,
                Text::PlaylistRename,
                Message::PlaylistAction(Action::Rename { id }),
            ))
            .push(
                button(prose(strings::lookup(Text::PlaylistDelete), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::PlaylistAction(Action::Delete { id })),
            )
            .push(
                row![
                    checkbox(state.sharing.open).on_toggle(move |open| Message::PlaylistAction(
                        Action::SetOpen { playlist: id, open }
                    )),
                    prose(strings::lookup(Text::PlaylistOpenAccess), typeface::BODY),
                ]
                .spacing(style::drawn(space::GUTTER.drawn()))
                .align_y(iced::Alignment::Center),
            );

        for shared in &state.sharing.users {
            let user = shared.user;
            page = page.push(
                row![
                    prose(shared.name.clone(), typeface::BODY),
                    checkbox(shared.can_edit).on_toggle(move |can_edit| Message::PlaylistAction(
                        Action::Share {
                            playlist: id,
                            user,
                            can_edit,
                        }
                    )),
                    button(prose(
                        strings::lookup(Text::PlaylistUnshare),
                        typeface::BODY
                    ))
                    .style(style::raised)
                    .on_press(Message::PlaylistAction(Action::Unshare {
                        playlist: id,
                        user
                    })),
                ]
                .spacing(style::drawn(space::GUTTER.drawn()))
                .align_y(iced::Alignment::Center),
            );
        }
    }

    let _ = images;
    let count = state.entries.len();
    page.push(crate::window::list(state.window, count, move |index| {
        entry_row(state, id, index, editable)
    }))
    .into()
}

pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Typed(typed) => {
            match &mut signed.view {
                crate::app::View::Playlists(listed) => listed.naming = typed,
                crate::app::View::Playlist(state) => state.naming = typed,
                _ => {}
            }
            Task::none()
        }
        Action::Create => {
            let name = match &signed.view {
                crate::app::View::Playlists(listed) => listed.naming.clone(),
                _ => return Task::none(),
            };
            if name.trim().is_empty() {
                return Task::none();
            }
            Task::perform(
                async move { api.create_playlist(&name, &[]).await.map(|_| ()) },
                |wrote| Message::Wrote(Operation::PlaylistCreate, wrote),
            )
        }
        Action::Rename { id } => {
            let name = match &signed.view {
                crate::app::View::Playlist(state) => state.naming.clone(),
                _ => return Task::none(),
            };
            if name.trim().is_empty() {
                return Task::none();
            }
            Task::perform(async move { api.rename_item(id, &name).await }, |wrote| {
                Message::Wrote(Operation::PlaylistRename, wrote)
            })
        }
        Action::Delete { id } => Task::perform(async move { api.delete_item(id).await }, |wrote| {
            Message::Wrote(Operation::PlaylistDelete, wrote)
        }),
        Action::Remove { playlist, entry } => Task::perform(
            async move { api.remove_playlist_entries(playlist, &[entry]).await },
            |wrote| Message::Wrote(Operation::PlaylistRemove, wrote),
        ),
        Action::Move {
            playlist,
            entry,
            to,
        } => Task::perform(
            async move { api.move_playlist_entry(playlist, &entry, to).await },
            |wrote| Message::Wrote(Operation::PlaylistMove, wrote),
        ),
        Action::PlayAll { id, shuffle } => {
            Task::done(Message::PlayPressed(crate::player::Intent::All {
                item: id,
                shuffle,
            }))
        }
        Action::PlayFrom { playlist, index } => {
            let Some(entry) = playlist_entry(signed, index) else {
                return Task::none();
            };
            let _ = playlist;
            Task::done(Message::PlayPressed(crate::player::Intent::Item {
                item: entry,
                resume: false,
            }))
        }
        Action::SetOpen { playlist, open } => Task::perform(
            async move { api.set_playlist_open(playlist, open).await },
            |wrote| Message::Wrote(Operation::PlaylistShare, wrote),
        ),
        Action::Share {
            playlist,
            user,
            can_edit,
        } => Task::perform(
            async move { api.share_playlist(playlist, user, can_edit).await },
            |wrote| Message::Wrote(Operation::PlaylistShare, wrote),
        ),
        Action::Unshare { playlist, user } => Task::perform(
            async move { api.unshare_playlist(playlist, user).await },
            |wrote| Message::Wrote(Operation::PlaylistShare, wrote),
        ),
    }
}

/// The item the entry at `index` names, and `None` while its page is not held.
fn playlist_entry(signed: &Signed, index: usize) -> Option<Uuid> {
    match &signed.view {
        crate::app::View::Playlist(state) => state.entries.row(index)?.item.id,
        _ => None,
    }
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .window
        .shown(state.entries.len())
        .filter_map(|index| state.entries.row(index))
        .filter_map(|entry| widget::poster_key(&entry.item))
        .collect()
}

pub fn listed_images(state: &Listed) -> HashSet<images::Key> {
    browse::images(&state.browse)
}
