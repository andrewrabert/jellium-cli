use jellyfin_api::types::BaseItemKind;
use uuid::Uuid;

/// What a browse surface is narrowed by; `Facets::default()` narrows by
/// nothing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Facets {
    pub played: Option<bool>,
    pub resumable: bool,
    pub favorite: bool,
    pub genres: Vec<Uuid>,
    pub studios: Vec<Uuid>,
    pub persons: Vec<Uuid>,
    pub artists: Vec<Uuid>,
    pub album_artists: Vec<Uuid>,
    pub official_ratings: Vec<String>,
    pub years: Vec<i32>,
    pub tags: Vec<String>,
    pub has_subtitles: bool,
    pub hd: bool,
    pub uhd: bool,
    pub video_kinds: Vec<VideoKind>,
    pub series_states: Vec<SeriesState>,
    pub kinds: Vec<BaseItemKind>,
}

impl Facets {
    /// How many narrowings are active, which is the count the surface states.
    pub fn count(&self) -> usize {
        usize::from(self.played.is_some())
            + usize::from(self.resumable)
            + usize::from(self.favorite)
            + self.genres.len()
            + self.studios.len()
            + self.persons.len()
            + self.artists.len()
            + self.album_artists.len()
            + self.official_ratings.len()
            + self.years.len()
            + self.tags.len()
            + usize::from(self.has_subtitles)
            + usize::from(self.hd)
            + usize::from(self.uhd)
            + self.video_kinds.len()
            + self.series_states.len()
            + self.kinds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// The preset the Favorites tab opens with.
    pub fn favorites() -> Facets {
        Facets {
            favorite: true,
            ..Facets::default()
        }
    }

    /// The preset a tab listing one item kind opens with, which is what the
    /// Episodes and Songs tabs carry.
    pub fn of_kind(kind: BaseItemKind) -> Facets {
        Facets {
            kinds: vec![kind],
            ..Facets::default()
        }
    }

    /// Narrowed to one facet value by id, which is what a hub entry, a cast
    /// name and a search person open.
    pub fn of(facet: Facet, id: Uuid) -> Facets {
        let mut facets = Facets::default();
        match facet {
            Facet::Genre | Facet::MusicGenre => facets.genres.push(id),
            Facet::Studio | Facet::Network => facets.studios.push(id),
            Facet::Person => facets.persons.push(id),
            Facet::Artist => facets.artists.push(id),
            Facet::AlbumArtist => facets.album_artists.push(id),
        }
        facets
    }
}

/// One facet a hub enumerates and a filtered list narrows by, addressed by id
/// and never by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facet {
    Genre,
    MusicGenre,
    Studio,
    Network,
    Person,
    Artist,
    AlbumArtist,
}

impl Facet {
    /// The upstream listing this hub reads.
    pub fn listing(self) -> &'static str {
        match self {
            Facet::Genre => "Genres",
            Facet::MusicGenre => "MusicGenres",
            Facet::Studio | Facet::Network => "Studios",
            Facet::Person => "Persons",
            Facet::Artist => "Artists",
            Facet::AlbumArtist => "Artists/AlbumArtists",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoKind {
    BluRay,
    Dvd,
    Iso,
    VideoFile,
}

impl VideoKind {
    pub const ALL: [VideoKind; 4] = [
        VideoKind::BluRay,
        VideoKind::Dvd,
        VideoKind::Iso,
        VideoKind::VideoFile,
    ];

    pub fn query(self) -> &'static str {
        match self {
            VideoKind::BluRay => "BluRay",
            VideoKind::Dvd => "Dvd",
            VideoKind::Iso => "Iso",
            VideoKind::VideoFile => "VideoFile",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesState {
    Continuing,
    Ended,
    Unreleased,
}

impl SeriesState {
    pub const ALL: [SeriesState; 3] = [
        SeriesState::Continuing,
        SeriesState::Ended,
        SeriesState::Unreleased,
    ];

    pub fn query(self) -> &'static str {
        match self {
            SeriesState::Continuing => "Continuing",
            SeriesState::Ended => "Ended",
            SeriesState::Unreleased => "Unreleased",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_narrows_by_nothing() {
        let facets = Facets::default();
        assert!(facets.is_empty());
        assert_eq!(facets.count(), 0);
    }

    #[test]
    fn every_active_narrowing_is_counted_once() {
        let facets = Facets {
            played: Some(false),
            resumable: true,
            favorite: true,
            genres: vec![Uuid::from_u128(1), Uuid::from_u128(2)],
            years: vec![1999],
            hd: true,
            video_kinds: vec![VideoKind::BluRay],
            ..Facets::default()
        };
        assert_eq!(facets.count(), 8);
        assert!(!facets.is_empty());
    }

    #[test]
    fn each_facet_narrows_the_dimension_it_addresses() {
        let id = Uuid::from_u128(7);
        assert_eq!(Facets::of(Facet::Genre, id).genres, vec![id]);
        assert_eq!(Facets::of(Facet::MusicGenre, id).genres, vec![id]);
        assert_eq!(Facets::of(Facet::Studio, id).studios, vec![id]);
        assert_eq!(Facets::of(Facet::Network, id).studios, vec![id]);
        assert_eq!(Facets::of(Facet::Person, id).persons, vec![id]);
        assert_eq!(Facets::of(Facet::Artist, id).artists, vec![id]);
        assert_eq!(Facets::of(Facet::AlbumArtist, id).album_artists, vec![id]);
        for facet in [
            Facet::Genre,
            Facet::MusicGenre,
            Facet::Studio,
            Facet::Network,
            Facet::Person,
            Facet::Artist,
            Facet::AlbumArtist,
        ] {
            assert_eq!(Facets::of(facet, id).count(), 1);
        }
    }

    #[test]
    fn the_presets_narrow_by_what_their_tabs_show() {
        assert_eq!(Facets::favorites().count(), 1);
        assert!(Facets::favorites().favorite);
        let songs = Facets::of_kind(BaseItemKind::Audio);
        assert_eq!(songs.kinds, vec![BaseItemKind::Audio]);
        assert_eq!(songs.count(), 1);
    }

    #[test]
    fn a_network_reads_the_studio_listing() {
        assert_eq!(Facet::Network.listing(), "Studios");
        assert_eq!(Facet::AlbumArtist.listing(), "Artists/AlbumArtists");
    }
}
