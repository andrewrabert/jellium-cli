use uuid::Uuid;

use jellium_model::facets::Facets;
use jellium_model::sort::Sort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Home,
    /// The library screen: the library, and the tab shown with what that tab
    /// carries.
    Library {
        id: Uuid,
        tab: Box<crate::screen::library::Tab>,
    },
    /// One facet value's items, entered from a hub entry, a cast name or a
    /// search person.
    Filtered(Box<Filtered>),
    Detail {
        id: Uuid,
    },
    /// The metadata manager for one item, at one of its six parts.
    Metadata {
        item: Uuid,
        part: crate::screen::metadata::Part,
    },
    Collections,
    Collection {
        id: Uuid,
        listing: Box<Listing>,
    },
    Playlists,
    Playlist {
        id: Uuid,
    },
    Search {
        term: String,
    },
    Queue,
    Remote,
    SyncPlay,
    LiveTv {
        tab: crate::screen::livetv::Tab,
    },
    Program {
        id: String,
    },
    Dashboard {
        screen: crate::screen::dashboard::Screen,
    },
    Settings {
        screen: crate::screen::settings::Screen,
    },
}

/// What one browse surface is showing: its order and its active filters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Listing {
    pub sort: Sort,
    pub facets: Facets,
}

/// A filtered list: the library it is narrowed to, the item whose image and
/// overview head it, and what it is showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filtered {
    /// `None` narrows across the server, which is what a cast name and a search
    /// person open.
    pub library: Option<Uuid>,
    /// The facet value's own item, drawn as the list's header.
    pub header: Option<Uuid>,
    pub listing: Listing,
}
