//! The sections a search result is drawn in, which is the reference's own
//! order and the item kind each one holds.

use jellyfin_api::types::BaseItemKind;

/// One section of a search result.
// reference: search-section-order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section {
    Movies,
    Shows,
    Episodes,
    People,
    Playlists,
    Artists,
    Albums,
    Songs,
    Videos,
    Programs,
    Channels,
    PhotoAlbums,
    Photos,
    AudioBooks,
    Books,
    Collections,
    Studios,
}

impl Section {
    /// Every section, in the order the reference draws them; `Studios`, which
    /// the reference has no section for, follows the ones it names.
    // reference: search-section-order
    pub const ALL: [Section; 17] = [
        Section::Movies,
        Section::Shows,
        Section::Episodes,
        Section::People,
        Section::Playlists,
        Section::Artists,
        Section::Albums,
        Section::Songs,
        Section::Videos,
        Section::Programs,
        Section::Channels,
        Section::PhotoAlbums,
        Section::Photos,
        Section::AudioBooks,
        Section::Books,
        Section::Collections,
        Section::Studios,
    ];

    /// The item kind whose results fill this section, and `None` for the three
    /// the server answers from a listing of its own.
    // reference: search-section-title
    // reference: search-section-kinds
    pub fn kind(self) -> Option<BaseItemKind> {
        match self {
            Section::Movies => Some(BaseItemKind::Movie),
            Section::Shows => Some(BaseItemKind::Series),
            Section::Episodes => Some(BaseItemKind::Episode),
            Section::Playlists => Some(BaseItemKind::Playlist),
            Section::Artists => Some(BaseItemKind::MusicArtist),
            Section::Albums => Some(BaseItemKind::MusicAlbum),
            Section::Songs => Some(BaseItemKind::Audio),
            Section::Videos => Some(BaseItemKind::Video),
            Section::Channels => Some(BaseItemKind::TvChannel),
            Section::PhotoAlbums => Some(BaseItemKind::PhotoAlbum),
            Section::Photos => Some(BaseItemKind::Photo),
            Section::AudioBooks => Some(BaseItemKind::AudioBook),
            Section::Books => Some(BaseItemKind::Book),
            Section::Collections => Some(BaseItemKind::BoxSet),
            Section::People | Section::Programs | Section::Studios => None,
        }
    }
}
