use iced::widget::{button, column, row, text_input};
use iced::{Element, Task};
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::app::{Message, Signed};
use crate::error::Operation;
use crate::images::Cache;
use crate::route::Route;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;

/// The overflow menu open now; at most one is open, and none is reachable under
/// read-only.
#[derive(Debug, Clone)]
pub struct Open {
    pub item: Uuid,
    pub played: bool,
    pub favorite: bool,
    /// The picker open over the menu, and `None` while the menu itself shows.
    pub filing: Option<Filing>,
}

/// Filing one item into a collection or a playlist, with the option of creating
/// one without leaving the screen.
#[derive(Debug, Clone)]
pub struct Filing {
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
    /// The card names what it already holds, so the menu draws its two toggles
    /// without a second read.
    Open {
        item: Uuid,
        played: bool,
        favorite: bool,
    },
    Close,
    MarkPlayed {
        item: Uuid,
        played: bool,
    },
    Favorite {
        item: Uuid,
        favorite: bool,
    },
    /// Opens the picker, which loads what the server holds.
    AddTo {
        item: Uuid,
        into: Into,
    },
    Typed(String),
    /// Files the item into the collection or playlist named.
    File {
        target: Uuid,
    },
    /// Creates a collection or playlist holding the item alone; a playlist's
    /// media type follows from that item.
    CreateAndFile,
    /// Removes the item from the collection whose screen it was opened from.
    RemoveFrom {
        collection: Uuid,
        item: Uuid,
    },
}

/// The menu drawn over the card or detail screen it was opened from, and the
/// picker over the menu while one is open.
/// `collection` names the collection whose screen the menu was opened from, and
/// is what puts the removal control on it.
pub fn view<'a>(
    open: &'a Open,
    images: &'a Cache,
    collection: Option<Uuid>,
) -> Element<'a, Message> {
    let _ = images;
    let item = open.item;

    let Some(filing) = &open.filing else {
        let mut menu = column![
            button(prose(
                strings::lookup(if open.played {
                    Text::OverflowMarkUnplayed
                } else {
                    Text::OverflowMarkPlayed
                })
                .to_owned(),
                typeface::BODY
            ))
            .on_press(Message::OverflowAction(Action::MarkPlayed {
                item,
                played: !open.played,
            })),
            button(prose(
                strings::lookup(if open.favorite {
                    Text::OverflowUnfavorite
                } else {
                    Text::OverflowFavorite
                })
                .to_owned(),
                typeface::BODY
            ))
            .on_press(Message::OverflowAction(Action::Favorite {
                item,
                favorite: !open.favorite,
            })),
            button(prose(
                strings::lookup(Text::OverflowAddToCollection).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::OverflowAction(Action::AddTo {
                item,
                into: Into::Collection,
            })),
            button(prose(
                strings::lookup(Text::OverflowAddToPlaylist).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::OverflowAction(Action::AddTo {
                item,
                into: Into::Playlist,
            })),
        ]
        .spacing(8);

        if let Some(collection) = collection {
            menu = menu.push(
                button(prose(
                    strings::lookup(Text::OverflowRemoveFromCollection).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::OverflowAction(Action::RemoveFrom {
                    collection,
                    item,
                })),
            );
        }

        return menu
            .push(
                button(prose(
                    strings::lookup(Text::OverflowClose).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::OverflowAction(Action::Close)),
            )
            .padding(theme::CARD_SPACING)
            .into();
    };

    let offered = filing.offered.iter().filter_map(|held| {
        let target = held.id?;
        Some(
            button(prose(held.name.clone().unwrap_or_default(), typeface::BODY))
                .style(button::text)
                .on_press(Message::OverflowAction(Action::File { target }))
                .into(),
        )
    });

    column![
        row![
            text_input("", &filing.naming)
                .on_input(|typed| Message::OverflowAction(Action::Typed(typed)))
                .padding(8),
            button(prose(
                strings::lookup(Text::OverflowCreateAndFile).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::OverflowAction(Action::CreateAndFile)),
        ]
        .spacing(theme::CARD_SPACING)
        .align_y(iced::Alignment::Center),
        column(offered).spacing(4),
        button(prose(
            strings::lookup(Text::OverflowClose).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::OverflowAction(Action::Close)),
    ]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING)
    .into()
}

pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Open {
            item,
            played,
            favorite,
        } => {
            if signed.session.read_only {
                return Task::none();
            }
            signed.overflow = Some(Open {
                item,
                played,
                favorite,
                filing: None,
            });
            Task::none()
        }
        Action::Close => {
            signed.overflow = None;
            Task::none()
        }
        Action::MarkPlayed { item, played } => {
            if let Some(open) = signed.overflow.as_mut() {
                open.played = played;
            }
            Task::done(Message::PlayedToggled(item, played))
        }
        Action::Favorite { item, favorite } => {
            if let Some(open) = signed.overflow.as_mut() {
                open.favorite = favorite;
            }
            Task::done(Message::FavoriteToggled(item, favorite))
        }
        Action::AddTo { item: _, into } => {
            let Some(open) = signed.overflow.as_mut() else {
                return Task::none();
            };
            open.filing = Some(Filing {
                into,
                offered: Vec::new(),
                naming: String::new(),
            });
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
            let Some(open) = signed.overflow.as_ref() else {
                return Task::none();
            };
            let Some(filing) = open.filing.as_ref() else {
                return Task::none();
            };
            let (item, into) = (open.item, filing.into);
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
            let Some(open) = signed.overflow.as_ref() else {
                return Task::none();
            };
            let Some(filing) = open.filing.as_ref() else {
                return Task::none();
            };
            let name = filing.naming.clone();
            if name.trim().is_empty() {
                return Task::none();
            }
            let (item, into) = (open.item, filing.into);
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
        Action::RemoveFrom { collection, item } => {
            signed.overflow = None;
            Task::done(Message::CollectionAction(
                crate::screen::collections::Action::Remove { collection, item },
            ))
        }
    }
}

/// The most collections or playlists a picker offers.
const OFFERED: i32 = 200;

/// The collection whose screen `route` names, and `None` for every other route;
/// it is what puts the removal control on the menu.
pub fn enclosing(route: Option<&Route>) -> Option<Uuid> {
    match route {
        Some(Route::Collection { id, .. }) => Some(*id),
        _ => None,
    }
}
