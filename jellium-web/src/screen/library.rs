use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::facets::{Facet, Facets};
use jellium_model::paged::Paged;
use jellium_model::sort::Sort;
use jellium_model::window;
use jellyfin_api::types::{BaseItemDto, BaseItemKind, CollectionType};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::{Listing, Route};
use crate::screen::browse::{self, Browse};
use crate::style::{self, Viewport, space};
use crate::text::Text;
use crate::widget;

/// A library tab named without what its list carries, which is what the strip
/// draws and what a collection type offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Items,
    Suggestions,
    Favorites,
    Upcoming,
    Episodes,
    Songs,
    AlbumArtists,
    Artists,
    Genres,
    Studios,
    Networks,
}

impl Kind {
    /// The facet this tab enumerates, and `None` for a tab drawn as something
    /// other than a hub.
    pub fn facet(self, music: bool) -> Option<Facet> {
        match self {
            Kind::Genres if music => Some(Facet::MusicGenre),
            Kind::Genres => Some(Facet::Genre),
            Kind::Studios => Some(Facet::Studio),
            Kind::Networks => Some(Facet::Network),
            Kind::Artists => Some(Facet::Artist),
            Kind::AlbumArtists => Some(Facet::AlbumArtist),
            _ => None,
        }
    }

    /// The tabs a library of `collection_type` carries, the item grid first; a
    /// book, music-video, home-video, photo or mixed library carries the item
    /// grid alone, and no library carries a Collections or a Playlists tab.
    pub fn of(collection_type: Option<CollectionType>) -> Vec<Kind> {
        match collection_type {
            Some(CollectionType::Movies) => vec![
                Kind::Items,
                Kind::Suggestions,
                Kind::Favorites,
                Kind::Genres,
                Kind::Studios,
            ],
            Some(CollectionType::Tvshows) => vec![
                Kind::Items,
                Kind::Suggestions,
                Kind::Favorites,
                Kind::Upcoming,
                Kind::Episodes,
                Kind::Genres,
                Kind::Networks,
            ],
            Some(CollectionType::Music) => vec![
                Kind::Items,
                Kind::Suggestions,
                Kind::Favorites,
                Kind::AlbumArtists,
                Kind::Artists,
                Kind::Songs,
                Kind::Genres,
            ],
            _ => vec![Kind::Items],
        }
    }

    pub fn label(self) -> Text {
        match self {
            Kind::Items => Text::LibraryTabItems,
            Kind::Suggestions => Text::LibraryTabSuggestions,
            Kind::Favorites => Text::LibraryTabFavorites,
            Kind::Upcoming => Text::LibraryTabUpcoming,
            Kind::Episodes => Text::LibraryTabEpisodes,
            Kind::Songs => Text::LibraryTabSongs,
            Kind::AlbumArtists => Text::LibraryTabAlbumArtists,
            Kind::Artists => Text::LibraryTabArtists,
            Kind::Genres => Text::LibraryTabGenres,
            Kind::Studios => Text::LibraryTabStudios,
            Kind::Networks => Text::LibraryTabNetworks,
        }
    }

    /// The tab this kind opens at, with its preset facets and `sort`.
    pub fn tab(self, sort: Sort) -> Tab {
        let listing = |facets: Facets| Box::new(Listing { sort, facets });
        match self {
            Kind::Items => Tab::Items(listing(Facets::default())),
            Kind::Suggestions => Tab::Suggestions,
            Kind::Favorites => Tab::Favorites(listing(Facets::favorites())),
            Kind::Upcoming => Tab::Upcoming,
            Kind::Episodes => Tab::Episodes(listing(Facets::of_kind(BaseItemKind::Episode))),
            Kind::Songs => Tab::Songs(listing(Facets::of_kind(BaseItemKind::Audio))),
            Kind::AlbumArtists => Tab::AlbumArtists,
            Kind::Artists => Tab::Artists,
            Kind::Genres => Tab::Genres,
            Kind::Studios => Tab::Studios,
            Kind::Networks => Tab::Networks,
        }
    }
}

/// The tab shown and what it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Items(Box<Listing>),
    Suggestions,
    Favorites(Box<Listing>),
    Upcoming,
    Episodes(Box<Listing>),
    Songs(Box<Listing>),
    AlbumArtists,
    Artists,
    Genres,
    Studios,
    Networks,
}

impl Tab {
    pub fn kind(&self) -> Kind {
        match self {
            Tab::Items(_) => Kind::Items,
            Tab::Suggestions => Kind::Suggestions,
            Tab::Favorites(_) => Kind::Favorites,
            Tab::Upcoming => Kind::Upcoming,
            Tab::Episodes(_) => Kind::Episodes,
            Tab::Songs(_) => Kind::Songs,
            Tab::AlbumArtists => Kind::AlbumArtists,
            Tab::Artists => Kind::Artists,
            Tab::Genres => Kind::Genres,
            Tab::Studios => Kind::Studios,
            Tab::Networks => Kind::Networks,
        }
    }

    /// What this tab's grid is showing, and `None` for a tab drawn as
    /// something other than a browse surface.
    pub fn listing(&self) -> Option<&Listing> {
        match self {
            Tab::Items(listing)
            | Tab::Favorites(listing)
            | Tab::Episodes(listing)
            | Tab::Songs(listing) => Some(listing),
            _ => None,
        }
    }
}

/// What the shown tab holds.
#[derive(Debug, Clone)]
pub enum Body {
    Browse(Box<Browse>),
    Suggestions(Box<crate::screen::suggestions::State>),
    Rows(Box<Browse>),
    Hub(Box<crate::screen::hub::State>),
}

#[derive(Debug, Clone)]
pub struct State {
    pub library: BaseItemDto,
    pub tabs: Vec<Kind>,
    pub tab: Tab,
    pub body: Body,
}

pub async fn load(
    api: Rc<Api>,
    library: Uuid,
    tab: Tab,
    viewport: Viewport,
    overflow: widget::Overflow,
) -> Answer<State> {
    Answer::of(async {
        let held = api.item(library).await.bubbled()?;
        let tabs = Kind::of(held.collection_type);
        let heading = held.name.clone().unwrap_or_default();
        let music = held.collection_type == Some(CollectionType::Music);
        let kind = tab.kind();

        let body = if let Some(facet) = kind.facet(music) {
            Body::Hub(Box::new(
                crate::screen::hub::load(api.clone(), facet, library, Sort::default(), viewport)
                    .await
                    .bubbled()?,
            ))
        } else if kind == Kind::Suggestions {
            Body::Suggestions(Box::new(
                crate::screen::suggestions::load(api.clone(), library, held.collection_type)
                    .await
                    .bubbled()?,
            ))
        } else if kind == Kind::Upcoming {
            let mut rows = Browse::new(
                window::Id::Browse,
                heading,
                Listing::default(),
                held.collection_type,
                viewport,
                overflow,
            );
            let items = api.upcoming(library, UPCOMING).await.bubbled()?;
            rows.items = Paged::new(items.len());
            rows.filled(0..items.len(), items);
            Body::Rows(Box::new(rows))
        } else {
            let listing = tab.listing().cloned().unwrap_or_default();
            let mut browse = Browse::new(
                window::Id::Browse,
                heading,
                listing.clone(),
                held.collection_type,
                viewport,
                overflow,
            );
            let answered = api
                .browse(
                    Some(library),
                    None,
                    &listing,
                    0,
                    Paged::<BaseItemDto>::PAGE as i32,
                )
                .await
                .bubbled()?;
            browse.items = Paged::new(answered.total.max(0) as usize);
            browse.filled(0..answered.items.len(), answered.items);
            browse.choices = choices(&api, Some(library), &held).await;
            Body::Browse(Box::new(browse))
        };

        Ok(State {
            library: held,
            tabs,
            tab,
            body,
        })
    })
    .await
}

/// The most episodes an Upcoming tab lists.
pub const UPCOMING: i32 = 64;

/// The filter choices the server offers for `parent`; a listing the server
/// refuses leaves the surface offering what it did answer.
pub async fn choices(api: &Api, parent: Option<Uuid>, library: &BaseItemDto) -> browse::Choices {
    let offered = api
        .filters(parent)
        .await
        .or_default(Text::FailureFiltersUnread);
    let studios = api
        .studios(parent)
        .await
        .or_default(Text::FailureStudiosUnread);
    let genres = api
        .genres(parent)
        .await
        .or_default(Text::FailureGenresUnread);
    browse::Choices {
        genres: genres
            .into_iter()
            .filter_map(|genre| {
                Some(browse::Named {
                    id: genre.id?,
                    name: genre.name?,
                })
            })
            .collect(),
        official_ratings: offered.official_ratings.unwrap_or_default(),
        years: offered.years.unwrap_or_default(),
        tags: offered.tags.unwrap_or_default(),
        studios: studios
            .into_iter()
            .filter_map(|studio| {
                Some(browse::Named {
                    id: studio.id?,
                    name: studio.name?,
                })
            })
            .collect(),
        series: library.collection_type == Some(CollectionType::Tvshows),
    }
}

pub fn view<'a>(
    state: &'a State,
    viewport: Viewport,
    images: &'a Cache,
    read_only: bool,
) -> Element<'a, Message> {
    let Some(id) = state.library.id else {
        return column![].into();
    };
    let shown = state.tab.kind();
    let strip = widget::tabs(
        viewport,
        state.tabs.iter().map(|kind| widget::Entry {
            label: kind.label(),
            showing: match *kind == shown {
                true => widget::Showing::Shown,
                false => widget::Showing::Offered(Message::Navigated(Route::Library {
                    id,
                    tab: Box::new(kind.tab(Sort::default())),
                })),
            },
        }),
    );

    let body = match &state.body {
        Body::Browse(browse) | Body::Rows(browse) => browse::view(browse, viewport, images),
        Body::Suggestions(held) => crate::screen::suggestions::view(
            held,
            viewport,
            images,
            match read_only {
                true => widget::Overflow::Withheld,
                false => widget::Overflow::Offered,
            },
        ),
        Body::Hub(held) => crate::screen::hub::view(held, viewport, images),
    };

    column![strip, body]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .padding(style::padding(space::PAGE_PAD))
        .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    match &state.body {
        Body::Browse(browse) | Body::Rows(browse) => browse::images(browse),
        Body::Suggestions(held) => crate::screen::suggestions::images(held),
        Body::Hub(held) => crate::screen::hub::images(held),
    }
}
