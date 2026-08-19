//! The `BaseItemDto` fields the metadata manager edits, written through one
//! `form::Form` so a save preserves every field no control covers.

use jellyfin_api::types::{BaseItemDto, BaseItemKind, LocationType, MediaType, MetadataField};

use crate::appearance::Share;
use crate::form::{Field, Form};

pub const NAME: Field = Field::Text { key: "Name" };
pub const ORIGINAL_TITLE: Field = Field::Text {
    key: "OriginalTitle",
};
pub const FORCED_SORT_NAME: Field = Field::Text {
    key: "ForcedSortName",
};
pub const COMMUNITY_RATING: Field = Field::Number {
    key: "CommunityRating",
};
pub const CRITIC_RATING: Field = Field::Number {
    key: "CriticRating",
};
pub const INDEX_NUMBER: Field = Field::Number { key: "IndexNumber" };
pub const PARENT_INDEX_NUMBER: Field = Field::Number {
    key: "ParentIndexNumber",
};
pub const AIRS_BEFORE_SEASON: Field = Field::Number {
    key: "AirsBeforeSeasonNumber",
};
pub const AIRS_AFTER_SEASON: Field = Field::Number {
    key: "AirsAfterSeasonNumber",
};
pub const AIRS_BEFORE_EPISODE: Field = Field::Number {
    key: "AirsBeforeEpisodeNumber",
};
pub const DISPLAY_ORDER: Field = Field::Text {
    key: "DisplayOrder",
};
pub const ALBUM: Field = Field::Text { key: "Album" };
pub const ALBUM_ARTISTS: Field = Field::Named {
    key: "AlbumArtists",
};
pub const ARTISTS: Field = Field::Lines { key: "Artists" };
pub const PRODUCTION_YEAR: Field = Field::Number {
    key: "ProductionYear",
};
pub const PREMIERE_DATE: Field = Field::Text {
    key: "PremiereDate",
};
pub const END_DATE: Field = Field::Text { key: "EndDate" };
pub const OFFICIAL_RATING: Field = Field::Text {
    key: "OfficialRating",
};
pub const CUSTOM_RATING: Field = Field::Text {
    key: "CustomRating",
};
pub const OVERVIEW: Field = Field::Text { key: "Overview" };
pub const TAGLINES: Field = Field::Lines { key: "Taglines" };
pub const GENRES: Field = Field::Lines { key: "Genres" };
pub const TAGS: Field = Field::Lines { key: "Tags" };
pub const STUDIOS: Field = Field::Named { key: "Studios" };
pub const PRODUCTION_LOCATIONS: Field = Field::Lines {
    key: "ProductionLocations",
};
pub const DATE_CREATED: Field = Field::Text { key: "DateCreated" };
pub const METADATA_LANGUAGE: Field = Field::Listed {
    key: "PreferredMetadataLanguage",
};
pub const METADATA_COUNTRY: Field = Field::Listed {
    key: "PreferredMetadataCountryCode",
};
pub const STATUS: Field = Field::Text { key: "Status" };
pub const AIR_DAYS: Field = Field::Lines { key: "AirDays" };
pub const AIR_TIME: Field = Field::Text { key: "AirTime" };
pub const VIDEO_3D_FORMAT: Field = Field::Text {
    key: "Video3DFormat",
};
pub const LOCK_DATA: Field = Field::Flag { key: "LockData" };
pub const LOCKED_FIELDS: Field = Field::Lines {
    key: "LockedFields",
};

/// The fields every item's editor shows.
pub const COMMON: &[Field] = &[
    NAME,
    ORIGINAL_TITLE,
    FORCED_SORT_NAME,
    COMMUNITY_RATING,
    CRITIC_RATING,
    OVERVIEW,
    TAGLINES,
    PRODUCTION_YEAR,
    PREMIERE_DATE,
    DATE_CREATED,
    OFFICIAL_RATING,
    CUSTOM_RATING,
    GENRES,
    TAGS,
    STUDIOS,
    PRODUCTION_LOCATIONS,
    METADATA_LANGUAGE,
    METADATA_COUNTRY,
    LOCK_DATA,
];

/// The fields shown only on an episode.
pub const EPISODE: &[Field] = &[
    INDEX_NUMBER,
    PARENT_INDEX_NUMBER,
    AIRS_BEFORE_SEASON,
    AIRS_AFTER_SEASON,
    AIRS_BEFORE_EPISODE,
];

/// The fields shown only on a series.
pub const SERIES: &[Field] = &[STATUS, END_DATE, AIR_DAYS, AIR_TIME, DISPLAY_ORDER];

/// The fields shown only on an audio item or an album.
pub const MUSIC: &[Field] = &[ALBUM, ALBUM_ARTISTS, ARTISTS, INDEX_NUMBER];

/// The fields shown only on a video item.
pub const VIDEO: &[Field] = &[VIDEO_3D_FORMAT];

/// The nine fields Jellyfin models a lock for; every other control carries no
/// lock.
pub const LOCKS: [MetadataField; 9] = [
    MetadataField::Cast,
    MetadataField::Genres,
    MetadataField::ProductionLocations,
    MetadataField::Studios,
    MetadataField::Tags,
    MetadataField::Name,
    MetadataField::Overview,
    MetadataField::Runtime,
    MetadataField::OfficialRating,
];

/// The lock `field` is covered by, and `None` for a field Jellyfin cannot lock.
pub fn lock_of(field: Field) -> Option<MetadataField> {
    match field.key() {
        "Name" => Some(MetadataField::Name),
        "Overview" => Some(MetadataField::Overview),
        "Genres" => Some(MetadataField::Genres),
        "Studios" => Some(MetadataField::Studios),
        "Tags" => Some(MetadataField::Tags),
        "ProductionLocations" => Some(MetadataField::ProductionLocations),
        "OfficialRating" => Some(MetadataField::OfficialRating),
        _ => None,
    }
}

/// The fields `kind`'s editor shows, in the order it shows them.
pub fn fields_of(kind: Option<BaseItemKind>) -> Vec<Field> {
    let mut held = COMMON.to_vec();
    let extra: &[Field] = match kind {
        Some(BaseItemKind::Episode) => EPISODE,
        Some(BaseItemKind::Series) => SERIES,
        Some(BaseItemKind::Audio | BaseItemKind::MusicAlbum) => MUSIC,
        Some(BaseItemKind::Movie | BaseItemKind::Video | BaseItemKind::MusicVideo) => VIDEO,
        _ => &[],
    };
    held.extend_from_slice(extra);
    held
}

/// One person on an item, as the cast and crew control edits them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Person {
    pub name: String,
    pub kind: String,
    pub role: String,
}

const PEOPLE: &str = "People";
const PROVIDER_IDS: &str = "ProviderIds";
const LOCKED: &str = "LockedFields";

/// The text `value` holds under `key`, and an empty string for anything else.
fn text_at(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

/// The people `item` holds, read from its `People` array.
pub fn people(item: &Form) -> Vec<Person> {
    item.written()
        .get(PEOPLE)
        .and_then(serde_json::Value::as_array)
        .map(|held| {
            held.iter()
                .map(|person| Person {
                    name: text_at(person, "Name"),
                    kind: text_at(person, "Type"),
                    role: text_at(person, "Role"),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// `item` with `people` written into its `People` array.
pub fn set_people(item: &mut Form, people: &[Person]) {
    let held: Vec<serde_json::Value> = people
        .iter()
        .filter(|person| !person.name.trim().is_empty())
        .map(|person| {
            serde_json::json!({
                "Name": person.name,
                "Type": person.kind,
                "Role": person.role,
            })
        })
        .collect();
    item.set(PEOPLE, serde_json::Value::Array(held));
}

/// The provider ids `item` holds, read from its `ProviderIds` object.
pub fn providers(item: &Form) -> Vec<(String, String)> {
    item.written()
        .get(PROVIDER_IDS)
        .and_then(serde_json::Value::as_object)
        .map(|held| {
            let mut held: Vec<(String, String)> = held
                .iter()
                .map(|(provider, id)| {
                    (provider.clone(), id.as_str().unwrap_or_default().to_owned())
                })
                .collect();
            held.sort_by(|one, two| one.0.cmp(&two.0));
            held
        })
        .unwrap_or_default()
}

/// `item` with `providers` written into its `ProviderIds` object.
pub fn set_providers(item: &mut Form, providers: &[(String, String)]) {
    let held: serde_json::Map<String, serde_json::Value> = providers
        .iter()
        .map(|(provider, id)| (provider.clone(), serde_json::Value::String(id.clone())))
        .collect();
    item.set(PROVIDER_IDS, serde_json::Value::Object(held));
}

/// The spelling `LockedFields` holds `lock` under.
fn lock_key(lock: MetadataField) -> String {
    lock.to_string()
}

/// True when `item` names `lock`, read from the `LockedFields` array.
pub fn locked(item: &Form, lock: MetadataField) -> bool {
    let wanted = lock_key(lock);
    item.written()
        .get(LOCKED)
        .and_then(serde_json::Value::as_array)
        .is_some_and(|held| {
            held.iter()
                .filter_map(serde_json::Value::as_str)
                .any(|held| held == wanted)
        })
}

/// `item` with `lock` added to or removed from its `LockedFields` array.
pub fn set_locked(item: &mut Form, lock: MetadataField, locked: bool) {
    let wanted = lock_key(lock);
    let mut held: Vec<String> = item
        .written()
        .get(LOCKED)
        .and_then(serde_json::Value::as_array)
        .map(|held| {
            held.iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    held.retain(|other| *other != wanted);
    if locked {
        held.push(wanted);
    }
    item.set(
        LOCKED,
        serde_json::Value::Array(held.into_iter().map(serde_json::Value::String).collect()),
    );
}

/// Whether a user mark stands on an item or is off it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Set,
    Cleared,
}

impl Mark {
    /// What the server's own field carries, which is the one site a mark
    /// becomes a scalar.
    pub fn set(self) -> bool {
        match self {
            Mark::Set => true,
            Mark::Cleared => false,
        }
    }

    /// The mark the opposite of this one, which is what a control that toggles
    /// asks for.
    pub fn flipped(self) -> Mark {
        match self {
            Mark::Set => Mark::Cleared,
            Mark::Cleared => Mark::Set,
        }
    }
}

/// What a metadata re-read overwrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Replace {
    All,
    Missing,
}

impl Replace {
    /// What `replaceAllMetadata` carries, which is the one site this becomes a
    /// scalar.
    pub fn all(self) -> bool {
        match self {
            Replace::All => true,
            Replace::Missing => false,
        }
    }
}

/// How far a metadata re-read reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Item,
    Tree,
}

impl Scope {
    /// What `recursive` carries, which is the one site this becomes a scalar.
    pub fn recursive(self) -> bool {
        match self {
            Scope::Tree => true,
            Scope::Item => false,
        }
    }
}

/// The mark the server's own field carries, absent where it holds nothing.
fn mark(held: Option<bool>) -> Mark {
    match held {
        Some(true) => Mark::Set,
        Some(false) | None => Mark::Cleared,
    }
}

/// Whether the user has played this item.
pub fn played(item: &BaseItemDto) -> Mark {
    mark(item.user_data.as_ref().and_then(|data| data.played))
}

/// Whether the user has favorited it.
pub fn favorited(item: &BaseItemDto) -> Mark {
    mark(item.user_data.as_ref().and_then(|data| data.is_favorite))
}

/// How far through an item a viewer is, as `getProgressBarHtml` reads it: the
/// share the item's own user data reports for a video that is no channel and
/// for an audio book, the share `now` stands at through a programme's own
/// airing, and none where the item marks no progress.
// a channel marks none, and neither does a recording's own user data
// none of it and the whole of it each draw no bar, which is what the
// reference's `pct && pct < 100` and its `pct > 0 && pct < 100` answer
// the reference's audio podcast names a type Jellyfin's own item kinds do not
// hold, and its timer arrives as a type of its own rather than as an item
// reference: indicator-progress
// reference: indicator-progress-enabled
pub fn elapsed(item: &BaseItemDto, now: chrono::DateTime<chrono::Utc>) -> Option<Share> {
    let marks = (item.media_type == Some(MediaType::Video)
        && item.type_ != Some(BaseItemKind::TvChannel))
        || item.type_ == Some(BaseItemKind::AudioBook);
    if marks
        && item.type_ != Some(BaseItemKind::Recording)
        && let Some(played) = item
            .user_data
            .as_ref()
            .and_then(|data| data.played_percentage)
        && played > 0.0
        && played < 100.0
    {
        return Some(Share::per_hundred(played));
    }
    if matches!(
        item.type_,
        Some(BaseItemKind::Program | BaseItemKind::Recording)
    ) && let (Some(start), Some(end)) = (item.start_date, item.end_date)
    {
        let run = (now - start).num_seconds();
        let whole = (end - start).num_seconds();
        if run > 0 && run < whole {
            return Some(Share::part(run, whole));
        }
    }
    None
}

/// Whether the reference's own player would take this item.
// a book, a photo album, a music genre, a season, a series, a box set, an
// album, an artist and a playlist play whatever they hold
// an item the server holds no file for plays only as a programme
// a programme plays only while `now` falls inside its own airing
// everything else plays where its media type is video or audio
// reference: item-can-play
pub fn playable(item: &BaseItemDto, now: chrono::DateTime<chrono::Utc>) -> bool {
    if matches!(
        item.type_,
        Some(
            BaseItemKind::Book
                | BaseItemKind::PhotoAlbum
                | BaseItemKind::MusicGenre
                | BaseItemKind::Season
                | BaseItemKind::Series
                | BaseItemKind::BoxSet
                | BaseItemKind::MusicAlbum
                | BaseItemKind::MusicArtist
                | BaseItemKind::Playlist
        )
    ) {
        return true;
    }
    let programme = item.type_ == Some(BaseItemKind::Program);
    if item.location_type == Some(LocationType::Virtual) && !programme {
        return false;
    }
    if programme {
        let (Some(start), Some(end)) = (item.start_date, item.end_date) else {
            return false;
        };
        if now > end || now < start {
            return false;
        }
    }
    matches!(item.media_type, Some(MediaType::Video | MediaType::Audio))
}

/// Whether the mobile overlay's own play control stands for this item where
/// the section left its option unset: a video that is no placeholder, is not
/// virtual unless it is a programme, and is not a person.
// reference: card-overlay-buttons
pub fn overlay_playable(item: &BaseItemDto) -> bool {
    item.media_type == Some(MediaType::Video)
        && item.is_place_holder != Some(true)
        && (item.location_type != Some(LocationType::Virtual)
            || item.type_ == Some(BaseItemKind::Program))
        && item.type_ != Some(BaseItemKind::Person)
}

/// Whether the played mark stands on it.
// a programme carries none
// video that is not a channel carries one
// an audio book carries one, the reference's audio podcast naming a type
// Jellyfin's own item kinds do not hold
// a series, a season and a box set carry one, and so does anything the server
// reports as a book, the reference's recording media type being one Jellyfin's
// own media types do not hold either
// reference: item-can-mark-played
pub fn markable(item: &BaseItemDto) -> bool {
    if item.type_ == Some(BaseItemKind::Program) {
        return false;
    }
    match item.media_type {
        Some(MediaType::Video) if item.type_ != Some(BaseItemKind::TvChannel) => return true,
        Some(MediaType::Audio) if item.type_ == Some(BaseItemKind::AudioBook) => return true,
        _ => {}
    }
    matches!(
        item.type_,
        Some(BaseItemKind::Series | BaseItemKind::Season | BaseItemKind::BoxSet)
    ) || matches!(item.media_type, Some(MediaType::Book))
}

/// The count `.countIndicator` writes over a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unplayed(i32);

impl Unplayed {
    /// What the badge writes, which the reference caps at `99+`.
    // reference: indicator-count
    pub fn written(self) -> String {
        match self.0 {
            100.. => "99+".to_owned(),
            count => count.to_string(),
        }
    }
}

/// The badge `getPlayedIndicatorHtml` writes over a card's image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Watched {
    /// `.countIndicator`: how many of the item's children are unplayed.
    Unplayed(Unplayed),
    /// `.playedIndicator`: the check an item a viewer has finished carries.
    Played,
}

/// The badge an item's own user data draws over its card: the count of what is
/// unplayed under it, the check where the whole of it is played, and none where
/// the item carries no played mark at all or reports no user data.
// reference: indicator-played
// reference: item-can-mark-played
pub fn watched(item: &BaseItemDto) -> Option<Watched> {
    if !markable(item) {
        return None;
    }
    let data = item.user_data.as_ref()?;
    if let Some(count) = data.unplayed_item_count.filter(|count| *count != 0) {
        return Some(Watched::Unplayed(Unplayed(count)));
    }
    let whole =
        data.played_percentage.is_some_and(|played| played >= 100.0) || data.played == Some(true);
    whole.then_some(Watched::Played)
}

/// Whether the rating control stands on it.
// a programme, a library, a user view and a channel carry none, and so does an
// item the server reports no user data for; the reference's timer and series
// timer name types Jellyfin's own item kinds do not hold, a timer arriving as a
// type of its own rather than as an item
// reference: item-can-rate
pub fn ratable(item: &BaseItemDto) -> bool {
    !matches!(
        item.type_,
        Some(
            BaseItemKind::Program
                | BaseItemKind::CollectionFolder
                | BaseItemKind::UserView
                | BaseItemKind::Channel
        )
    ) && item.user_data.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(value: serde_json::Value) -> Form {
        Form::of(value)
    }

    #[test]
    fn every_kind_shows_the_common_fields_and_only_its_own_extras() {
        let episode = fields_of(Some(BaseItemKind::Episode));
        assert!(COMMON.iter().all(|field| episode.contains(field)));
        assert!(episode.contains(&AIRS_BEFORE_SEASON));
        assert!(!episode.contains(&STATUS));

        let series = fields_of(Some(BaseItemKind::Series));
        assert!(series.contains(&STATUS));
        assert!(!series.contains(&ALBUM));

        let audio = fields_of(Some(BaseItemKind::Audio));
        assert!(audio.contains(&ALBUM_ARTISTS));
        assert!(!audio.contains(&VIDEO_3D_FORMAT));

        let movie = fields_of(Some(BaseItemKind::Movie));
        assert!(movie.contains(&VIDEO_3D_FORMAT));
        assert!(!movie.contains(&STATUS));

        assert_eq!(fields_of(None), COMMON.to_vec());
    }

    #[test]
    fn a_lock_covers_only_the_fields_jellyfin_models_one_for() {
        assert_eq!(lock_of(NAME), Some(MetadataField::Name));
        assert_eq!(lock_of(GENRES), Some(MetadataField::Genres));
        assert_eq!(lock_of(FORCED_SORT_NAME), None);
        assert_eq!(lock_of(COMMUNITY_RATING), None);
        assert_eq!(LOCKS.len(), 9);
    }

    #[test]
    fn people_round_trip_through_the_form() {
        let mut held = item(serde_json::json!({
            "People": [{"Name": "one", "Type": "Actor", "Role": "Someone"}],
            "Untouched": true,
        }));
        assert_eq!(
            people(&held),
            vec![Person {
                name: "one".to_owned(),
                kind: "Actor".to_owned(),
                role: "Someone".to_owned(),
            }]
        );

        set_people(
            &mut held,
            &[
                Person {
                    name: "two".to_owned(),
                    kind: "Director".to_owned(),
                    role: String::new(),
                },
                Person::default(),
            ],
        );
        assert_eq!(people(&held).len(), 1);
        assert_eq!(people(&held)[0].name, "two");
        assert_eq!(held.written()["Untouched"], serde_json::json!(true));
    }

    #[test]
    fn providers_round_trip_through_the_form() {
        let mut held = item(serde_json::json!({"ProviderIds": {"Tmdb": "42"}}));
        assert_eq!(providers(&held), vec![("Tmdb".to_owned(), "42".to_owned())]);
        set_providers(
            &mut held,
            &[
                ("Imdb".to_owned(), "tt1".to_owned()),
                ("Tmdb".to_owned(), "43".to_owned()),
            ],
        );
        assert_eq!(
            providers(&held),
            vec![
                ("Imdb".to_owned(), "tt1".to_owned()),
                ("Tmdb".to_owned(), "43".to_owned()),
            ]
        );
    }

    #[test]
    fn a_lock_is_set_and_cleared_leaving_every_other_lock_standing() {
        let mut held = item(serde_json::json!({"LockedFields": ["Genres"]}));
        assert!(locked(&held, MetadataField::Genres));
        assert!(!locked(&held, MetadataField::Name));

        set_locked(&mut held, MetadataField::Name, true);
        assert!(locked(&held, MetadataField::Name));
        assert!(locked(&held, MetadataField::Genres));

        set_locked(&mut held, MetadataField::Genres, false);
        assert!(!locked(&held, MetadataField::Genres));
        assert!(locked(&held, MetadataField::Name));
    }

    #[test]
    fn setting_a_lock_twice_leaves_one_entry() {
        let mut held = item(serde_json::json!({}));
        set_locked(&mut held, MetadataField::Name, true);
        set_locked(&mut held, MetadataField::Name, true);
        assert_eq!(held.written()["LockedFields"], serde_json::json!(["Name"]));
    }
}
