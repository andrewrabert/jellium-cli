//! The sections the reference's Favorites tab draws, with the request and the
//! card shape it gives each.

use jellyfin_api::types::BaseItemKind;

use crate::appearance::card;
use crate::paged::Limit;

/// One section of the reference's Favorites tab, in the order it draws them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Movies,
    Shows,
    Seasons,
    Episodes,
    Videos,
    MusicVideos,
    Collections,
    Playlists,
    People,
    Artists,
    Albums,
    Songs,
    Books,
    Channels,
    PhotoAlbums,
    Photos,
}

/// Which route the reference asks a section's favourites from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Asked {
    /// `/Items` narrowed to one `IncludeItemTypes`.
    Items(BaseItemKind),
    /// `/Artists`, which takes no `IncludeItemTypes`.
    Artists,
    /// `/Persons`, likewise.
    People,
}

impl Section {
    pub const ALL: [Section; 16] = [
        Section::Movies,
        Section::Shows,
        Section::Seasons,
        Section::Episodes,
        Section::Videos,
        Section::MusicVideos,
        Section::Collections,
        Section::Playlists,
        Section::People,
        Section::Artists,
        Section::Albums,
        Section::Songs,
        Section::Books,
        Section::Channels,
        Section::PhotoAlbums,
        Section::Photos,
    ];

    /// Movies, Shows, Seasons, Collections, People and Books take the portrait
    /// rail; Playlists, Artists, Albums and Songs take the square rail; the
    /// rest take the backdrop rail.
    // reference: favorites-shapes
    pub fn card(self) -> card::Card {
        card::Card::Rail(match self {
            Section::Movies
            | Section::Shows
            | Section::Seasons
            | Section::Collections
            | Section::People
            | Section::Books => card::Rail::Portrait,
            Section::Playlists | Section::Artists | Section::Albums | Section::Songs => {
                card::Rail::Square
            }
            Section::Episodes
            | Section::Videos
            | Section::MusicVideos
            | Section::Channels
            | Section::PhotoAlbums
            | Section::Photos => card::Rail::Backdrop,
        })
    }

    /// Movies, Shows and Books write the year under the name; Seasons,
    /// Episodes, Albums and Songs write the parent title instead; the rest
    /// write the name alone.
    // reference: favorites-shapes
    pub fn footer(self) -> card::Footer {
        match self {
            Section::Movies | Section::Shows | Section::Books => card::Footer::of(
                card::Parent::Withheld,
                card::Title::Shown,
                &[card::Line::Year],
            ),
            Section::Seasons | Section::Episodes | Section::Albums | Section::Songs => {
                card::Footer::of(card::Parent::Shown, card::Title::Shown, &[])
            }
            Section::Videos
            | Section::MusicVideos
            | Section::Collections
            | Section::Playlists
            | Section::People
            | Section::Artists
            | Section::Channels
            | Section::PhotoAlbums
            | Section::Photos => card::Footer::of(card::Parent::Withheld, card::Title::Shown, &[]),
        }
    }

    // reference: favorites-query
    pub fn asked(self) -> Asked {
        match self {
            Section::Artists => Asked::Artists,
            Section::People => Asked::People,
            Section::Movies => Asked::Items(BaseItemKind::Movie),
            Section::Shows => Asked::Items(BaseItemKind::Series),
            Section::Seasons => Asked::Items(BaseItemKind::Season),
            Section::Episodes => Asked::Items(BaseItemKind::Episode),
            Section::Videos => Asked::Items(BaseItemKind::Video),
            Section::MusicVideos => Asked::Items(BaseItemKind::MusicVideo),
            Section::Collections => Asked::Items(BaseItemKind::BoxSet),
            Section::Playlists => Asked::Items(BaseItemKind::Playlist),
            Section::Albums => Asked::Items(BaseItemKind::MusicAlbum),
            Section::Songs => Asked::Items(BaseItemKind::Audio),
            Section::Books => Asked::Items(BaseItemKind::Book),
            Section::Channels => Asked::Items(BaseItemKind::TvChannel),
            Section::PhotoAlbums => Asked::Items(BaseItemKind::PhotoAlbum),
            Section::Photos => Asked::Items(BaseItemKind::Photo),
        }
    }
}

/// How many favourites one section asks for.
// reference: favorites-query
pub const ASKED: Limit = Limit::of(20);
