use jellium_model::facets::{Facet, Facets, SeriesState, VideoKind};
use jellium_model::item::Mark;
use jellium_model::paged::Limit;
use jellium_model::sort::Sort;
use jellium_model::{prefs, quickconnect};
use jellyfin_api::types::{
    BaseItemDto, BaseItemKind, DisplayPreferencesDto, ItemFields, SeriesTimerInfoDto, TimerInfoDto,
    UserItemDataDto,
};
use uuid::Uuid;

use crate::error::{Answer, Bubble, Trouble};
use crate::images;
use crate::livetv::{Channel, Program};
use crate::route::Listing;
use crate::style::card;
use crate::text::Text;

#[derive(Debug, Clone, Default)]
pub struct Page {
    pub items: Vec<BaseItemDto>,
    pub total: i32,
}

#[derive(Default)]
struct Query {
    parent_id: Option<Uuid>,
    album_artist_ids: Option<Vec<Uuid>>,
    ids: Option<Vec<Uuid>>,
    include_item_types: Option<Vec<BaseItemKind>>,
    search_term: Option<String>,
    sort: Sort,
    start: i32,
    limit: Option<i32>,
    facets: Facets,
    /// The initial the letter jump asked for, which bounds the answer to the
    /// items sorting at or after it.
    name_starts_with_or_greater: Option<String>,
    /// True when only the total is wanted, which is what the letter jump reads.
    count_only: bool,
}

/// The video type `kind` names, matched by the spelling `/Items` takes.
fn video_type(kind: VideoKind) -> Option<jellyfin_api::types::VideoType> {
    use jellyfin_api::types::VideoType;
    [
        VideoType::VideoFile,
        VideoType::Iso,
        VideoType::Dvd,
        VideoType::BluRay,
    ]
    .into_iter()
    .find(|held| held.to_string() == kind.query())
}

/// The series status `state` names, matched by the spelling `/Items` takes.
fn series_status(state: SeriesState) -> Option<jellyfin_api::types::SeriesStatus> {
    use jellyfin_api::types::SeriesStatus;
    [
        SeriesStatus::Continuing,
        SeriesStatus::Ended,
        SeriesStatus::Unreleased,
    ]
    .into_iter()
    .find(|held| held.to_string() == state.query())
}

/// The `filters` list the resumable and favorite narrowings render as.
fn item_filters(facets: &Facets) -> Vec<jellyfin_api::types::ItemFilter> {
    use jellyfin_api::types::ItemFilter;
    let mut filters = Vec::new();
    if facets.resumable {
        filters.push(ItemFilter::IsResumable);
    }
    if facets.favorite {
        filters.push(ItemFilter::IsFavorite);
    }
    filters
}

/// A list a query carries only when it narrows by something.
fn narrowing<T>(values: &[T]) -> Option<Vec<T>>
where
    T: Clone,
{
    (!values.is_empty()).then(|| values.to_vec())
}

pub struct Api {
    client: jellyfin_api::Client,
    http: reqwest::Client,
    base: String,
    user_id: Uuid,
}

/// One value as a query string carries it, so no byte the caller chose reaches
/// the url as a delimiter.
fn urlencode(value: &str) -> String {
    percent_encoding::utf8_percent_encode(value, percent_encoding::NON_ALPHANUMERIC).to_string()
}

/// One activity entry as the activity screen takes it; an entry the server
/// named no id for is dropped.
fn read_entry(
    entry: jellyfin_api::types::ActivityLogEntry,
) -> Option<jellium_protocol::ActivityEntry> {
    Some(jellium_protocol::ActivityEntry {
        id: entry.id?,
        name: entry.name.unwrap_or_default(),
        overview: entry.short_overview.or(entry.overview).unwrap_or_default(),
        kind: entry.type_.unwrap_or_default(),
        severity: entry
            .severity
            .map(|severity| severity.to_string())
            .unwrap_or_default(),
        user: entry.user_id.filter(|user| !user.is_nil()),
        item: entry.item_id.as_deref().and_then(|id| {
            let Ok(item) = crate::failure::unraised::read(id) else {
                return None;
            };
            Some(item)
        }),
        at: entry.date.map(|at| at.timestamp_millis()).unwrap_or(0),
    })
}

/// The most values one facet listing offers the filter surface.
const FACET_LIMIT: i32 = 500;

/// The most remote images one provider search offers.
const REMOTE_IMAGES: i32 = 60;

fn fields() -> Vec<ItemFields> {
    vec![
        ItemFields::Chapters,
        ItemFields::MediaSources,
        ItemFields::MediaStreams,
        ItemFields::Overview,
        ItemFields::ParentId,
        ItemFields::PrimaryImageAspectRatio,
    ]
}

impl Api {
    pub fn new(user_id: Uuid) -> Api {
        let base = format!(
            "{}{}",
            crate::page::origin(),
            jellium_protocol::RELAY_PREFIX
        );
        let http = reqwest::Client::new();
        Api {
            client: jellyfin_api::Client::new(&base, http.clone()),
            http,
            base,
            user_id,
        }
    }

    /// The relay client the setup stage uses; it names no user, because a
    /// server in startup mode has none.
    pub fn anonymous() -> Api {
        Api::new(Uuid::nil())
    }

    /// The UI cultures the Jellyfin server offers.
    pub async fn localization_options(
        &self,
    ) -> Answer<Vec<jellyfin_api::types::LocalizationOption>> {
        Answer::of(async { Ok(self.client.get_localization_options().await?) }).await
    }

    /// The metadata languages the Jellyfin server offers.
    pub async fn cultures(&self) -> Answer<Vec<jellyfin_api::types::CultureDto>> {
        Answer::of(async { Ok(self.client.get_cultures().await?) }).await
    }

    /// The metadata countries the Jellyfin server offers.
    pub async fn countries(&self) -> Answer<Vec<jellyfin_api::types::CountryInfo>> {
        Answer::of(async { Ok(self.client.get_countries().await?) }).await
    }

    /// Where the server's filesystem browser opens.
    pub async fn default_directory(&self) -> Answer<String> {
        Answer::of(async {
            Ok(self
                .client
                .get_default_directory_browser()
                .await?
                .path
                .unwrap_or_default())
        })
        .await
    }

    /// The parent of `path`, which is how the wizard's browser goes up.
    pub async fn parent_path(&self, path: &str) -> Answer<String> {
        Answer::of(async { Ok(self.client.get_parent_path(path).await?) }).await
    }

    async fn query(&self, query: Query) -> Result<Page, Trouble> {
        let fields = fields();
        let (by, order) = query.sort.query();
        let sort_by = vec![by];
        let sort_order = vec![order];
        let facets = &query.facets;
        let filters = item_filters(facets);
        let genre_ids = narrowing(&facets.genres);
        let studio_ids = narrowing(&facets.studios);
        let person_ids = narrowing(&facets.persons);
        let artist_ids = narrowing(&facets.artists);
        let facet_album_artist_ids = narrowing(&facets.album_artists);
        let official_ratings = narrowing(&facets.official_ratings);
        let years = narrowing(&facets.years);
        let tags = narrowing(&facets.tags);
        let video_types = narrowing(&facets.video_kinds)
            .map(|kinds| kinds.into_iter().filter_map(video_type).collect::<Vec<_>>());
        let series_status = narrowing(&facets.series_states).map(|states| {
            states
                .into_iter()
                .filter_map(series_status)
                .collect::<Vec<_>>()
        });
        let include_item_types = match (query.include_item_types.clone(), narrowing(&facets.kinds))
        {
            (Some(mut held), Some(kinds)) => {
                held.extend(kinds);
                Some(held)
            }
            (held, kinds) => held.or(kinds),
        };
        let album_artist_ids = match (query.album_artist_ids.clone(), facet_album_artist_ids) {
            (Some(mut held), Some(more)) => {
                held.extend(more);
                Some(held)
            }
            (held, more) => held.or(more),
        };

        let result = self
            .client
            .get_items(&jellyfin_api::query::GetItems {
                album_artist_ids: album_artist_ids.as_ref(),
                artist_ids: artist_ids.as_ref(),
                enable_images: Some(true),
                enable_total_record_count: Some(true),
                enable_user_data: Some(true),
                fields: Some(&fields),
                filters: Some(&filters),
                genre_ids: genre_ids.as_ref(),
                has_subtitles: facets.has_subtitles.then_some(true),
                ids: query.ids.as_ref(),
                include_item_types: include_item_types.as_ref(),
                is_4k: facets.uhd.then_some(true),
                is_hd: facets.hd.then_some(true),
                is_played: facets.played,
                limit: if query.count_only {
                    Some(0)
                } else {
                    query.limit
                },
                name_starts_with_or_greater: query.name_starts_with_or_greater.as_deref(),
                official_ratings: official_ratings.as_ref(),
                parent_id: query.parent_id.as_ref(),
                person_ids: person_ids.as_ref(),
                recursive: Some(true),
                search_term: query.search_term.as_deref(),
                series_status: series_status.as_ref(),
                sort_by: Some(&sort_by),
                sort_order: Some(&sort_order),
                start_index: Some(query.start),
                studio_ids: studio_ids.as_ref(),
                tags: tags.as_ref(),
                user_id: Some(&self.user_id),
                video_types: video_types.as_ref(),
                years: years.as_ref(),
                ..Default::default()
            })
            .await?;

        Ok(Page {
            total: result
                .total_record_count
                .unwrap_or(result.items.len() as i32),
            items: result.items,
        })
    }

    pub async fn libraries(&self) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            Ok(self
                .client
                .get_user_views(None, None, None, Some(&self.user_id))
                .await?
                .items)
        })
        .await
    }

    pub async fn continue_watching(&self) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_resume_items(&jellyfin_api::query::GetResumeItems {
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    limit: Some(RAIL_LIMIT),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    pub async fn next_up(&self) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_next_up(&jellyfin_api::query::GetNextUp {
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    limit: Some(RAIL_LIMIT),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    /// One page of a browse surface: `listing`'s sort and facets narrowed to
    /// `parent`, and `term` when one is being searched for.
    pub async fn browse(
        &self,
        parent: Option<Uuid>,
        term: Option<&str>,
        listing: &Listing,
        start: i32,
        limit: i32,
    ) -> Answer<Page> {
        Answer::of(async {
            Ok(self
                .query(Query {
                    parent_id: parent,
                    search_term: term.map(str::to_owned),
                    sort: listing.sort,
                    facets: listing.facets.clone(),
                    start,
                    limit: Some(limit),
                    ..Query::default()
                })
                .await?)
        })
        .await
    }

    /// The index of the first item at or after `letter`, which is where the
    /// letter jump scrolls to.
    pub async fn letter_index(
        &self,
        parent: Option<Uuid>,
        listing: &Listing,
        letter: char,
    ) -> Answer<usize> {
        Answer::of(async {
            let Some(bound) = jellium_model::window::letter_bound(letter) else {
                return Ok(0);
            };
            let whole = self
                .query(Query {
                    parent_id: parent,
                    sort: listing.sort,
                    facets: listing.facets.clone(),
                    count_only: true,
                    ..Query::default()
                })
                .await?
                .total;
            let at_or_after = self
                .query(Query {
                    parent_id: parent,
                    sort: listing.sort,
                    facets: listing.facets.clone(),
                    name_starts_with_or_greater: Some(bound),
                    count_only: true,
                    ..Query::default()
                })
                .await?
                .total;
            Ok(whole.saturating_sub(at_or_after).max(0) as usize)
        })
        .await
    }

    /// The genre, official-rating, year and tag choices the server offers,
    /// scoped to `parent`.
    pub async fn filters(
        &self,
        parent: Option<Uuid>,
    ) -> Answer<jellyfin_api::types::QueryFiltersLegacy> {
        Answer::of(async {
            Ok(self
                .client
                .get_query_filters_legacy(None, None, parent.as_ref(), Some(&self.user_id))
                .await?)
        })
        .await
    }

    /// One page of a hub's values across `library`.
    pub async fn hub(
        &self,
        facet: Facet,
        library: Uuid,
        sort: Sort,
        start: i32,
        limit: i32,
    ) -> Answer<Page> {
        Answer::of(async {
            let fields = fields();
            let (by, order) = sort.query();
            let sort_by = vec![by];
            let sort_order = vec![order];
            let parent = Some(&library);

            let result = match facet {
                Facet::Genre => {
                    self.client
                        .get_genres(&jellyfin_api::query::GetGenres {
                            enable_images: Some(true),
                            enable_total_record_count: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            parent_id: parent,
                            sort_by: Some(&sort_by),
                            sort_order: Some(&sort_order),
                            start_index: Some(start),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Facet::MusicGenre => {
                    self.client
                        .get_music_genres(&jellyfin_api::query::GetMusicGenres {
                            enable_images: Some(true),
                            enable_total_record_count: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            parent_id: parent,
                            sort_by: Some(&sort_by),
                            sort_order: Some(&sort_order),
                            start_index: Some(start),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Facet::Studio | Facet::Network => {
                    self.client
                        .get_studios(&jellyfin_api::query::GetStudios {
                            enable_images: Some(true),
                            enable_total_record_count: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            parent_id: parent,
                            start_index: Some(start),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Facet::Person => {
                    self.client
                        .get_persons(&jellyfin_api::query::GetPersons {
                            enable_images: Some(true),
                            enable_user_data: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Facet::Artist => {
                    self.client
                        .get_artists(&jellyfin_api::query::GetArtists {
                            enable_images: Some(true),
                            enable_total_record_count: Some(true),
                            enable_user_data: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            parent_id: parent,
                            sort_by: Some(&sort_by),
                            sort_order: Some(&sort_order),
                            start_index: Some(start),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Facet::AlbumArtist => {
                    self.client
                        .get_album_artists(&jellyfin_api::query::GetAlbumArtists {
                            enable_images: Some(true),
                            enable_total_record_count: Some(true),
                            enable_user_data: Some(true),
                            fields: Some(&fields),
                            limit: Some(limit),
                            parent_id: parent,
                            sort_by: Some(&sort_by),
                            sort_order: Some(&sort_order),
                            start_index: Some(start),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
            };

            Ok(Page {
                total: result
                    .total_record_count
                    .unwrap_or(result.items.len() as i32),
                items: result.items,
            })
        })
        .await
    }

    // the reference names no GroupItems, so the server's own grouping stands
    // reference: home-latest-query
    pub async fn latest(&self, library: Uuid, limit: Limit) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_latest_media(&jellyfin_api::query::GetLatestMedia {
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    limit: Some(limit.count()),
                    parent_id: Some(&library),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?)
        })
        .await
    }

    /// The server's suggestions for this user. `/Items/Suggestions` takes no
    /// parent, so the rail is user-scoped rather than library-scoped.
    pub async fn suggestions(&self, limit: i32) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            Ok(self
                .client
                .get_suggestions(None, Some(limit), None, None, None, Some(&self.user_id))
                .await?
                .items)
        })
        .await
    }

    pub async fn recommendations(
        &self,
        library: Uuid,
    ) -> Answer<Vec<jellyfin_api::types::RecommendationDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_movie_recommendations(
                    None,
                    Some(&fields),
                    Some(RAIL_LIMIT),
                    Some(&library),
                    Some(&self.user_id),
                )
                .await?)
        })
        .await
    }

    pub async fn upcoming(&self, library: Uuid, limit: i32) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_upcoming_episodes(&jellyfin_api::query::GetUpcomingEpisodes {
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    limit: Some(limit),
                    parent_id: Some(&library),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    pub async fn similar(&self, item: Uuid, limit: i32) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_similar_items(&item, None, Some(&fields), Some(limit), Some(&self.user_id))
                .await?
                .items)
        })
        .await
    }

    /// The genres the server offers, scoped to `parent`, addressed by id.
    pub async fn genres(&self, parent: Option<Uuid>) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            Ok(self
                .client
                .get_genres(&jellyfin_api::query::GetGenres {
                    enable_images: Some(true),
                    limit: Some(FACET_LIMIT),
                    parent_id: parent.as_ref(),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    /// The studios the server offers, scoped to `parent`.
    pub async fn studios(&self, parent: Option<Uuid>) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            Ok(self
                .client
                .get_studios(&jellyfin_api::query::GetStudios {
                    enable_images: Some(true),
                    limit: Some(FACET_LIMIT),
                    parent_id: parent.as_ref(),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    pub async fn item(&self, item: Uuid) -> Answer<BaseItemDto> {
        Answer::of(async { Ok(self.client.get_item(&item, Some(&self.user_id)).await?) }).await
    }

    /// The items `ids` names, in one request; an id the Jellyfin server will
    /// not hand over is absent from the answer.
    pub async fn items(&self, ids: &[Uuid]) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            if ids.is_empty() {
                return Ok(Vec::new());
            }
            Ok(self
                .query(Query {
                    ids: Some(ids.to_vec()),
                    limit: Some(ids.len() as i32),
                    ..Query::default()
                })
                .await?
                .items)
        })
        .await
    }

    pub async fn children(&self, item: &BaseItemDto) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let Some(id) = item.id else {
                return Ok(Vec::new());
            };
            let fields = fields();

            match item.type_ {
                Some(BaseItemKind::Series) => Ok(self
                    .client
                    .get_seasons(
                        &id,
                        &jellyfin_api::query::GetSeasons {
                            enable_images: Some(true),
                            enable_user_data: Some(true),
                            fields: Some(&fields),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        },
                    )
                    .await?
                    .items),
                Some(BaseItemKind::Season) => {
                    let Some(series) = item.series_id else {
                        return Ok(Vec::new());
                    };
                    Ok(self
                        .client
                        .get_episodes(
                            &series,
                            &jellyfin_api::query::GetEpisodes {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                season_id: Some(&id),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                        .items)
                }
                Some(BaseItemKind::MusicAlbum) => Ok(self
                    .query(Query {
                        parent_id: Some(id),
                        include_item_types: Some(vec![BaseItemKind::Audio]),
                        sort: Sort::Name,
                        ..Query::default()
                    })
                    .await?
                    .items),
                Some(BaseItemKind::MusicArtist) => Ok(self
                    .query(Query {
                        album_artist_ids: Some(vec![id]),
                        include_item_types: Some(vec![BaseItemKind::MusicAlbum]),
                        sort: Sort::ReleaseDate,
                        ..Query::default()
                    })
                    .await?
                    .items),
                Some(BaseItemKind::BoxSet | BaseItemKind::Playlist) => Ok(self
                    .query(Query {
                        parent_id: Some(id),
                        ..Query::default()
                    })
                    .await?
                    .items),
                _ => Ok(Vec::new()),
            }
        })
        .await
    }

    pub async fn set_played(&self, item: Uuid, played: Mark) -> Answer<UserItemDataDto> {
        Answer::of(async {
            Ok(if played.set() {
                self.client
                    .mark_played_item(&item, None, Some(&self.user_id))
                    .await?
            } else {
                self.client
                    .mark_unplayed_item(&item, Some(&self.user_id))
                    .await?
            })
        })
        .await
    }

    pub async fn set_favorite(&self, item: Uuid, favorite: Mark) -> Answer<UserItemDataDto> {
        Answer::of(async {
            Ok(if favorite.set() {
                self.client
                    .mark_favorite_item(&item, Some(&self.user_id))
                    .await?
            } else {
                self.client
                    .unmark_favorite_item(&item, Some(&self.user_id))
                    .await?
            })
        })
        .await
    }

    /// The items a queue is built from: the rest of an episode's season, a
    /// series' episodes, a season's episodes, an album's tracks, or an
    /// artist's tracks.
    pub async fn queue(&self, item: &BaseItemDto) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let Some(id) = item.id else {
                return Ok(Vec::new());
            };
            let fields = fields();
            match item.type_ {
                Some(BaseItemKind::Episode) => {
                    let Some(series) = item.series_id else {
                        return Ok(vec![item.clone()]);
                    };
                    Ok(self
                        .client
                        .get_episodes(
                            &series,
                            &jellyfin_api::query::GetEpisodes {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                season_id: item.season_id.as_ref(),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                        .items)
                }
                Some(BaseItemKind::Series) => Ok(self
                    .client
                    .get_episodes(
                        &id,
                        &jellyfin_api::query::GetEpisodes {
                            enable_images: Some(true),
                            enable_user_data: Some(true),
                            fields: Some(&fields),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        },
                    )
                    .await?
                    .items),
                Some(BaseItemKind::Season) => {
                    let Some(series) = item.series_id else {
                        return Ok(Vec::new());
                    };
                    Ok(self
                        .client
                        .get_episodes(
                            &series,
                            &jellyfin_api::query::GetEpisodes {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                season_id: Some(&id),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                        .items)
                }
                Some(BaseItemKind::MusicAlbum) => Ok(self
                    .query(Query {
                        parent_id: Some(id),
                        include_item_types: Some(vec![BaseItemKind::Audio]),
                        sort: Sort::Name,
                        ..Query::default()
                    })
                    .await?
                    .items),
                Some(BaseItemKind::MusicArtist) => Ok(self
                    .query(Query {
                        album_artist_ids: Some(vec![id]),
                        include_item_types: Some(vec![BaseItemKind::Audio]),
                        sort: Sort::Name,
                        ..Query::default()
                    })
                    .await?
                    .items),
                _ => Ok(vec![item.clone()]),
            }
        })
        .await
    }

    /// The Jellyfin server's instant mix for a song, an album or an artist.
    pub async fn instant_mix(&self, item: &BaseItemDto) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let Some(id) = item.id else {
                return Ok(Vec::new());
            };
            let fields = fields();
            let result = match item.type_ {
                Some(BaseItemKind::MusicAlbum) => {
                    self.client
                        .get_instant_mix_from_album(
                            &id,
                            &jellyfin_api::query::GetInstantMixFromAlbum {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                limit: Some(MIX_LIMIT),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                }
                Some(BaseItemKind::MusicArtist) => {
                    self.client
                        .get_instant_mix_from_artist(
                            &id,
                            &jellyfin_api::query::GetInstantMixFromArtist {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                limit: Some(MIX_LIMIT),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                }
                _ => {
                    self.client
                        .get_instant_mix_from_item(
                            &id,
                            &jellyfin_api::query::GetInstantMixFromItem {
                                enable_images: Some(true),
                                enable_user_data: Some(true),
                                fields: Some(&fields),
                                limit: Some(MIX_LIMIT),
                                user_id: Some(&self.user_id),
                                ..Default::default()
                            },
                        )
                        .await?
                }
            };
            Ok(result.items)
        })
        .await
    }

    pub fn image_url(&self, key: images::Key, fill: card::Fill) -> String {
        format!(
            "{}/Items/{}/Images/{}?fillWidth={}",
            self.base,
            key.item,
            key.kind.as_str(),
            fill.count(),
        )
    }

    /// A non-2xx answer is classified, so a revoked token reads as
    /// `Failure::TokenRejected` rather than as a transport error.
    pub async fn image(&self, url: String) -> Answer<Vec<u8>> {
        Answer::of(async {
            let response = self.http.get(url).send().await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(response.bytes().await?.to_vec())
        })
        .await
    }

    /// The media folders a user's library access is chosen from.
    // reference: user-access-load
    pub async fn media_folders(&self) -> Answer<Vec<jellyfin_api::types::BaseItemDto>> {
        Answer::of(async { Ok(self.client.get_media_folders(Some(false)).await?.items) }).await
    }

    /// The channels a user's channel access is chosen from.
    // reference: user-access-load
    pub async fn channels(&self) -> Answer<Vec<jellyfin_api::types::BaseItemDto>> {
        Answer::of(async {
            Ok(self
                .client
                .get_channels(None, None, None, None, None, None)
                .await?
                .items)
        })
        .await
    }

    /// The metadata readers and subtitle fetchers a library of `content`
    /// offers.
    // reference: library-options-available
    pub async fn library_option_info(
        &self,
        content: Option<jellyfin_api::types::CollectionType>,
    ) -> Answer<jellyfin_api::types::LibraryOptionsResultDto> {
        Answer::of(async {
            Ok(self
                .client
                .get_library_options_info(Some(false), content)
                .await?)
        })
        .await
    }

    /// The channels of `kind`, favourites first and then in channel-number
    /// order, each carrying its current program, in one request.
    /// One section of the reference's Favorites tab, in one request: that
    /// section's own route and item kinds, favourites only, recursive, box sets
    /// uncollapsed, virtual locations excluded, ascending by series name then
    /// name.
    // reference: favorites-query
    pub async fn favorites(
        &self,
        section: jellium_model::favorites::Section,
        limit: jellium_model::paged::Limit,
    ) -> Answer<Vec<BaseItemDto>> {
        use jellium_model::favorites::Asked;
        Answer::of(async {
            let fields = vec![jellyfin_api::types::ItemFields::PrimaryImageAspectRatio];
            let favourite = vec![jellyfin_api::types::ItemFilter::IsFavorite];
            let sorted = vec![
                jellyfin_api::types::ItemSortBy::SeriesSortName,
                jellyfin_api::types::ItemSortBy::SortName,
            ];
            let ascending = vec![jellyfin_api::types::SortOrder::Ascending];
            let kinds = match section.asked() {
                Asked::Items(kind) => vec![kind],
                Asked::Artists | Asked::People => Vec::new(),
            };
            let result = match section.asked() {
                Asked::Artists => {
                    self.client
                        .get_artists(&jellyfin_api::query::GetArtists {
                            enable_total_record_count: Some(false),
                            fields: Some(&fields),
                            filters: Some(&favourite),
                            limit: Some(limit.count()),
                            sort_by: Some(&sorted),
                            sort_order: Some(&ascending),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Asked::People => {
                    self.client
                        .get_persons(&jellyfin_api::query::GetPersons {
                            fields: Some(&fields),
                            filters: Some(&favourite),
                            limit: Some(limit.count()),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
                Asked::Items(_) => {
                    self.client
                        .get_items(&jellyfin_api::query::GetItems {
                            collapse_box_set_items: Some(false),
                            enable_total_record_count: Some(false),
                            exclude_location_types: Some(&vec![
                                jellyfin_api::types::LocationType::Virtual,
                            ]),
                            fields: Some(&fields),
                            filters: Some(&favourite),
                            include_item_types: Some(&kinds),
                            limit: Some(limit.count()),
                            recursive: Some(true),
                            sort_by: Some(&sorted),
                            sort_order: Some(&ascending),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                }
            };
            Ok(result.items)
        })
        .await
    }

    /// One section of the reference's Programs tab, in one request:
    /// `/LiveTv/Programs/Recommended` with `IsAiring` for On Now, and
    /// `/LiveTv/Programs` with `HasAired` false and the section's own flags for
    /// the other five.
    // reference: programs-query
    pub async fn section_programs(
        &self,
        section: jellium_model::livetv::Section,
        limit: jellium_model::paged::Limit,
    ) -> Answer<Vec<BaseItemDto>> {
        use jellium_model::livetv::{Airing, Upcoming};
        Answer::of(async {
            let fields = vec![
                jellyfin_api::types::ItemFields::ChannelInfo,
                jellyfin_api::types::ItemFields::PrimaryImageAspectRatio,
            ];
            let kinds = vec![
                jellyfin_api::types::ImageType::Primary,
                jellyfin_api::types::ImageType::Thumb,
            ];
            let narrowed = match section.airing() {
                Airing::Now => {
                    return Ok(self
                        .client
                        .get_recommended_programs(&jellyfin_api::query::GetRecommendedPrograms {
                            enable_image_types: Some(&vec![
                                jellyfin_api::types::ImageType::Primary,
                                jellyfin_api::types::ImageType::Thumb,
                                jellyfin_api::types::ImageType::Backdrop,
                            ]),
                            enable_total_record_count: Some(false),
                            fields: Some(&fields),
                            image_type_limit: Some(1),
                            is_airing: Some(true),
                            limit: Some(limit.count()),
                            user_id: Some(&self.user_id),
                            ..Default::default()
                        })
                        .await?
                        .items);
                }
                Airing::Upcoming(upcoming) => upcoming,
            };
            let asked = jellyfin_api::query::GetLiveTvPrograms {
                enable_image_types: Some(&kinds),
                enable_total_record_count: Some(false),
                fields: Some(&fields),
                has_aired: Some(false),
                limit: Some(limit.count()),
                user_id: Some(&self.user_id),
                is_series: matches!(narrowed, Upcoming::Shows).then_some(true),
                is_movie: match narrowed {
                    Upcoming::Shows => Some(false),
                    Upcoming::Movies => Some(true),
                    Upcoming::Sports | Upcoming::Kids | Upcoming::News => None,
                },
                is_sports: match narrowed {
                    Upcoming::Shows => Some(false),
                    Upcoming::Sports => Some(true),
                    Upcoming::Movies | Upcoming::Kids | Upcoming::News => None,
                },
                is_kids: match narrowed {
                    Upcoming::Shows => Some(false),
                    Upcoming::Kids => Some(true),
                    Upcoming::Movies | Upcoming::Sports | Upcoming::News => None,
                },
                is_news: match narrowed {
                    Upcoming::Shows => Some(false),
                    Upcoming::News => Some(true),
                    Upcoming::Movies | Upcoming::Sports | Upcoming::Kids => None,
                },
                ..Default::default()
            };
            Ok(self.client.get_live_tv_programs(&asked).await?.items)
        })
        .await
    }

    pub async fn live_tv_channels(
        &self,
        kind: jellyfin_api::types::ChannelType,
        limit: Option<i32>,
    ) -> Answer<Vec<Channel>> {
        Answer::of(async {
            let result = self
                .client
                .get_live_tv_channels(&jellyfin_api::query::GetLiveTvChannels {
                    add_current_program: Some(true),
                    enable_favorite_sorting: Some(true),
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    limit,
                    type_: Some(kind),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?;
            Ok(result.items.iter().filter_map(Channel::read).collect())
        })
        .await
    }

    /// The range the Jellyfin server's guide covers.
    pub async fn guide_range(&self) -> Answer<std::ops::Range<chrono::DateTime<chrono::Utc>>> {
        Answer::of(async {
            let info = self.client.get_guide_info().await?;
            let opens = info.start_date.unwrap_or_else(chrono::Utc::now);
            let closes = info
                .end_date
                .unwrap_or_else(|| opens + chrono::TimeDelta::days(1));
            Ok(opens..closes)
        })
        .await
    }

    /// The programs on `channels` overlapping `span`, posted as one
    /// `GetProgramsDto` rather than asked for in a query string.
    pub async fn programs(
        &self,
        channels: &[Uuid],
        span: std::ops::Range<chrono::DateTime<chrono::Utc>>,
    ) -> Answer<Vec<Program>> {
        Answer::of(async {
            let result = self
                .client
                .get_programs(&jellyfin_api::types::GetProgramsDto {
                    channel_ids: Some(channels.to_vec()),
                    max_start_date: Some(span.end),
                    min_end_date: Some(span.start),
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    enable_total_record_count: false,
                    user_id: Some(self.user_id),
                    ..jellyfin_api::types::GetProgramsDto::default()
                })
                .await?;
            Ok(result.items.iter().filter_map(Program::read).collect())
        })
        .await
    }

    /// The program airing now on `channel`, in one request.
    pub async fn airing(&self, channel: Uuid) -> Answer<Option<Program>> {
        Answer::of(async {
            let now = chrono::Utc::now();
            let result = self
                .client
                .get_programs(&jellyfin_api::types::GetProgramsDto {
                    channel_ids: Some(vec![channel]),
                    max_start_date: Some(now),
                    min_end_date: Some(now),
                    limit: Some(1),
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    enable_total_record_count: false,
                    user_id: Some(self.user_id),
                    ..jellyfin_api::types::GetProgramsDto::default()
                })
                .await?;
            Ok(result.items.iter().find_map(Program::read))
        })
        .await
    }

    /// The recordings, in-progress first and then newest first.
    pub async fn recordings(&self) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            let result = self
                .client
                .get_recordings(&jellyfin_api::query::GetRecordings {
                    enable_images: Some(true),
                    enable_total_record_count: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?;
            let mut items = result.items;
            items.sort_by(|one, two| {
                let progressing =
                    |item: &BaseItemDto| crate::screen::livetv::recordings::in_progress(item);
                progressing(two)
                    .cmp(&progressing(one))
                    .then_with(|| two.date_created.cmp(&one.date_created))
            });
            Ok(items)
        })
        .await
    }

    /// The recordings the Jellyfin server is writing now.
    // reference: livetv-schedule-active
    pub async fn active_recordings(&self) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            let result = self
                .client
                .get_recordings(&jellyfin_api::query::GetRecordings {
                    enable_images: Some(true),
                    enable_total_record_count: Some(false),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    is_in_progress: Some(true),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?;
            Ok(result.items)
        })
        .await
    }

    /// The scheduled timers, ordered by start time.
    pub async fn timers(&self) -> Answer<Vec<TimerInfoDto>> {
        Answer::of(async {
            let result = self.client.get_timers(None, None, None, None).await?;
            let mut items = result.items;
            items.sort_by_key(|timer| timer.start_date);
            Ok(items)
        })
        .await
    }

    /// The series timers, by name.
    pub async fn series_timers(&self) -> Answer<Vec<SeriesTimerInfoDto>> {
        Answer::of(async {
            let result = self
                .client
                .get_series_timers(Some("SortName"), None)
                .await?;
            Ok(result.items)
        })
        .await
    }

    /// The Jellyfin server's defaults for recording `program`.
    pub async fn timer_defaults(&self, program: &str) -> Answer<SeriesTimerInfoDto> {
        Answer::of(async { Ok(self.client.get_default_timer(Some(program)).await?) }).await
    }

    /// Creates a timer for `program` from the Jellyfin server's defaults, in
    /// two requests and with no prompt.
    pub async fn record(&self, program: &str) -> Answer<()> {
        Answer::of(async {
            let defaults = self.timer_defaults(program).await.bubbled()?;
            self.client.create_timer(&timer_of(&defaults)).await?;
            Ok(())
        })
        .await
    }

    pub async fn record_series(&self, options: &SeriesTimerInfoDto) -> Answer<()> {
        Answer::of(async {
            self.client.create_series_timer(options).await?;
            Ok(())
        })
        .await
    }

    pub async fn update_series_timer(&self, options: &SeriesTimerInfoDto) -> Answer<()> {
        Answer::of(async {
            let Some(id) = options.id.as_deref() else {
                return Ok(());
            };
            self.client.update_series_timer(id, options).await?;
            Ok(())
        })
        .await
    }

    /// Writes back one timer as the recording editor holds it.
    pub async fn update_timer(&self, timer: &TimerInfoDto) -> Answer<()> {
        Answer::of(async {
            let Some(id) = timer.id.as_deref() else {
                return Ok(());
            };
            self.client.update_timer(id, timer).await?;
            Ok(())
        })
        .await
    }

    pub async fn cancel_timer(&self, timer: &str) -> Answer<()> {
        Answer::of(async {
            self.client.cancel_timer(timer).await?;
            Ok(())
        })
        .await
    }

    pub async fn cancel_series_timer(&self, timer: &str) -> Answer<()> {
        Answer::of(async {
            self.client.cancel_series_timer(timer).await?;
            Ok(())
        })
        .await
    }

    pub async fn restart(&self) -> Answer<()> {
        Answer::of(async {
            self.client.restart_application().await?;
            Ok(())
        })
        .await
    }

    pub async fn shutdown(&self) -> Answer<()> {
        Answer::of(async {
            self.client.shutdown_application().await?;
            Ok(())
        })
        .await
    }

    pub async fn scan_all(&self) -> Answer<()> {
        Answer::of(async {
            self.client.refresh_library().await?;
            Ok(())
        })
        .await
    }

    /// The section `key` names, read whole so a save preserves what no control
    /// covers.
    pub async fn section(&self, key: &str) -> Answer<serde_json::Value> {
        Answer::of(async {
            Ok(self
                .client
                .get_named_configuration::<serde_json::Value>(key)
                .await?)
        })
        .await
    }

    pub async fn save_section(&self, key: &str, body: &serde_json::Value) -> Answer<()> {
        Answer::of(async {
            self.client.update_named_configuration(key, body).await?;
            Ok(())
        })
        .await
    }

    /// The json `path` answers, read whole so a save preserves every key no
    /// control covers.
    async fn read_whole(&self, path: &str) -> Result<serde_json::Value, Trouble> {
        let response = self.http.get(format!("{}{path}", self.base)).send().await?;
        if !response.status().is_success() {
            return Err(crate::error::classify(response).await);
        }
        Ok(response.json::<serde_json::Value>().await?)
    }

    /// Writes `body` to `path` whole.
    async fn write_whole<T: serde::Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<(), Trouble> {
        let response = self
            .http
            .post(format!("{}{path}", self.base))
            .json(body)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(crate::error::classify(response).await);
        }
        Ok(())
    }

    /// Deletes `path`.
    async fn delete_whole(&self, path: &str) -> Result<(), Trouble> {
        let response = self
            .http
            .delete(format!("{}{path}", self.base))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(crate::error::classify(response).await);
        }
        Ok(())
    }

    /// The item read whole, so a save preserves every field no control covers.
    pub async fn item_whole(&self, item: Uuid) -> Answer<serde_json::Value> {
        Answer::of(async { Ok(self.read_whole(&format!("/Items/{item}")).await?) }).await
    }

    /// Re-reads the item and writes `edits` over what it answered, so a field
    /// another client changed since the form opened is not overwritten by a
    /// stale copy.
    pub async fn save_item(
        &self,
        item: Uuid,
        edits: &serde_json::Map<String, serde_json::Value>,
    ) -> Answer<()> {
        Answer::of(async {
            let mut whole = self.item_whole(item).await.bubbled()?;
            let Some(held) = whole.as_object_mut() else {
                return Err(Bubble::from(Trouble::Relay {
                    status: None,
                    detail: "the item was not a json object".to_owned(),
                }));
            };
            for (key, value) in edits {
                held.insert(key.clone(), value.clone());
            }
            Ok(self.write_whole(&format!("/Items/{item}"), &whole).await?)
        })
        .await
    }

    /// Renames one item by reading it whole and writing back its new name.
    pub async fn rename_item(&self, item: Uuid, name: &str) -> Answer<()> {
        Answer::of(async {
            let mut edits = serde_json::Map::new();
            edits.insert(
                "Name".to_owned(),
                serde_json::Value::String(name.to_owned()),
            );
            self.save_item(item, &edits).await.bubbled()
        })
        .await
    }

    pub async fn delete_item(&self, item: Uuid) -> Answer<()> {
        Answer::of(async { Ok(self.client.delete_item(&item).await?) }).await
    }

    pub async fn collections(&self, start: i32, limit: i32) -> Answer<Page> {
        Answer::of(async {
            Ok(self
                .query(Query {
                    include_item_types: Some(vec![BaseItemKind::BoxSet]),
                    start,
                    limit: Some(limit),
                    ..Query::default()
                })
                .await?)
        })
        .await
    }

    pub async fn create_collection(&self, name: &str, items: &[Uuid]) -> Answer<Uuid> {
        Answer::of(async {
            let ids: Vec<String> = items.iter().map(Uuid::to_string).collect();
            let created = self
                .client
                .create_collection(Some(&ids), None, Some(name), None)
                .await?;
            created.id.ok_or_else(|| {
                Bubble::from(Trouble::Relay {
                    status: None,
                    detail: "the server named no id for the collection it created".to_owned(),
                })
            })
        })
        .await
    }

    pub async fn add_to_collection(&self, collection: Uuid, items: &[Uuid]) -> Answer<()> {
        Answer::of(async { Ok(self.client.add_to_collection(&collection, items).await?) }).await
    }

    pub async fn remove_from_collection(&self, collection: Uuid, items: &[Uuid]) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .client
                .remove_from_collection(&collection, items)
                .await?)
        })
        .await
    }

    pub async fn add_to_playlist(&self, playlist: Uuid, items: &[Uuid]) -> Answer<()> {
        Answer::of(async {
            let ids = items.to_vec();
            Ok(self
                .client
                .add_item_to_playlist(&playlist, Some(&ids), Some(&self.user_id))
                .await?)
        })
        .await
    }

    pub async fn playlists(&self, start: i32, limit: i32) -> Answer<Page> {
        Answer::of(async {
            Ok(self
                .query(Query {
                    include_item_types: Some(vec![BaseItemKind::Playlist]),
                    start,
                    limit: Some(limit),
                    ..Query::default()
                })
                .await?)
        })
        .await
    }

    /// Creates a playlist holding `items`; its media type follows from the
    /// first item, and no argument names one.
    pub async fn create_playlist(&self, name: &str, items: &[Uuid]) -> Answer<Uuid> {
        Answer::of(async {
            let body = jellyfin_api::types::CreatePlaylistDto {
                ids: items.to_vec(),
                name: Some(name.to_owned()),
                user_id: Some(self.user_id),
                ..jellyfin_api::types::CreatePlaylistDto::default()
            };
            let created = self
                .client
                .create_playlist(None, None, Some(name), Some(&self.user_id), &body)
                .await?;
            let named = created.id.ok_or_else(|| {
                Bubble::from(Trouble::Relay {
                    status: None,
                    detail: "the server named no id for the playlist it created".to_owned(),
                })
            })?;
            named.parse().map_err(|error: uuid::Error| {
                Bubble::from(Trouble::Relay {
                    status: None,
                    detail: error.to_string(),
                })
            })
        })
        .await
    }

    /// One page of a playlist's entries, each carrying the entry id that tells
    /// two copies of one item apart.
    pub async fn playlist_entries(
        &self,
        playlist: Uuid,
        start: i32,
        limit: i32,
    ) -> Answer<Entries> {
        Answer::of(async {
            let fields = fields();
            let result = self
                .client
                .get_playlist_items(
                    &playlist,
                    &jellyfin_api::query::GetPlaylistItems {
                        enable_images: Some(true),
                        enable_user_data: Some(true),
                        fields: Some(&fields),
                        limit: Some(limit),
                        start_index: Some(start),
                        user_id: Some(&self.user_id),
                        ..Default::default()
                    },
                )
                .await?;
            let total = result
                .total_record_count
                .unwrap_or(result.items.len() as i32);
            Ok(Entries {
                entries: result
                    .items
                    .into_iter()
                    .map(|item| crate::screen::playlists::Entry {
                        entry: item.playlist_item_id.clone().unwrap_or_default(),
                        item,
                    })
                    .collect(),
                total,
            })
        })
        .await
    }

    pub async fn remove_playlist_entries(&self, playlist: Uuid, entries: &[String]) -> Answer<()> {
        Answer::of(async {
            let held = entries.to_vec();
            Ok(self
                .client
                .remove_item_from_playlist(&playlist.to_string(), Some(&held))
                .await?)
        })
        .await
    }

    pub async fn move_playlist_entry(&self, playlist: Uuid, entry: &str, to: usize) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .client
                .move_item(&playlist.to_string(), entry, to as i32)
                .await?)
        })
        .await
    }

    pub async fn playlist_sharing(
        &self,
        playlist: Uuid,
    ) -> Answer<crate::screen::playlists::Sharing> {
        Answer::of(async {
            let held = self.client.get_playlist(&playlist).await?;
            let shares = self.client.get_playlist_users(&playlist).await?;
            let users = self.users().await.or_default(Text::FailureUsersUnread);
            Ok(crate::screen::playlists::Sharing {
                open: held.open_access.unwrap_or(false),
                users: shares
                    .into_iter()
                    .filter_map(|share| {
                        let user = share.user_id?;
                        Some(crate::screen::playlists::Shared {
                            user,
                            name: users
                                .iter()
                                .find(|held| held.id == Some(user))
                                .and_then(|held| held.name.clone())
                                .unwrap_or_default(),
                            can_edit: share.can_edit.unwrap_or(false),
                        })
                    })
                    .collect(),
            })
        })
        .await
    }

    pub async fn set_playlist_open(&self, playlist: Uuid, open: bool) -> Answer<()> {
        Answer::of(async {
            let body = jellyfin_api::types::UpdatePlaylistDto {
                is_public: Some(open),
                ..jellyfin_api::types::UpdatePlaylistDto::default()
            };
            Ok(self.client.update_playlist(&playlist, &body).await?)
        })
        .await
    }

    pub async fn share_playlist(&self, playlist: Uuid, user: Uuid, can_edit: bool) -> Answer<()> {
        Answer::of(async {
            let body = jellyfin_api::types::UpdatePlaylistUserDto {
                can_edit: Some(can_edit),
            };
            Ok(self
                .client
                .update_playlist_user(&playlist, &user, &body)
                .await?)
        })
        .await
    }

    pub async fn unshare_playlist(&self, playlist: Uuid, user: Uuid) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .client
                .remove_user_from_playlist(&playlist, &user)
                .await?)
        })
        .await
    }

    pub async fn set_content_type(&self, item: Uuid, content_type: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .client
                .update_item_content_type(&item, Some(content_type))
                .await?)
        })
        .await
    }

    pub async fn metadata_editor(
        &self,
        item: Uuid,
    ) -> Answer<jellyfin_api::types::MetadataEditorInfo> {
        Answer::of(async { Ok(self.client.get_metadata_editor_info(&item).await?) }).await
    }

    /// The candidates a remote search answered with; the relay has replaced
    /// every provider image url with a handle it minted.
    pub async fn identify(
        &self,
        search: crate::screen::metadata::identify::Search,
        query: &impl serde::Serialize,
    ) -> Answer<Vec<jellyfin_api::types::RemoteSearchResult>> {
        Answer::of(async {
            let path = format!("/Items/RemoteSearch/{}", search.segment());
            let response = self
                .http
                .post(format!("{}{path}", self.base))
                .json(query)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(response.json().await?)
        })
        .await
    }

    /// Applies one candidate; its `ImageUrl` is a minted handle, which the relay
    /// resolves on the way upstream.
    pub async fn apply_identity(
        &self,
        item: Uuid,
        candidate: &jellyfin_api::types::RemoteSearchResult,
        replace_images: bool,
    ) -> Answer<()> {
        Answer::of(async {
            let path =
                format!("/Items/RemoteSearch/Apply/{item}?replaceAllImages={replace_images}");
            let response = self
                .http
                .post(format!("{}{path}", self.base))
                .json(candidate)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    pub async fn item_images(&self, item: Uuid) -> Answer<Vec<jellyfin_api::types::ImageInfo>> {
        Answer::of(async {
            let response = self
                .http
                .get(format!("{}/Items/{item}/Images", self.base))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(response.json().await?)
        })
        .await
    }

    /// Posts the chosen image's bytes with `mime` as their content type; the
    /// relay is what base64-encodes them for Jellyfin.
    pub async fn upload_item_image(
        &self,
        item: Uuid,
        kind: crate::screen::metadata::artwork::Kind,
        mime: &str,
        bytes: Vec<u8>,
    ) -> Answer<()> {
        Answer::of(async {
            let response = self
                .http
                .post(format!(
                    "{}/Items/{item}/Images/{}",
                    self.base,
                    kind.as_str()
                ))
                .header(reqwest::header::CONTENT_TYPE, mime)
                .body(bytes)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    pub async fn remove_item_image(
        &self,
        item: Uuid,
        kind: crate::screen::metadata::artwork::Kind,
        index: Option<i32>,
    ) -> Answer<()> {
        Answer::of(async {
            let at = match index {
                Some(index) => format!("/{index}"),
                None => String::new(),
            };
            Ok(self
                .delete_whole(&format!("/Items/{item}/Images/{}{at}", kind.as_str()))
                .await?)
        })
        .await
    }

    pub async fn move_item_image(
        &self,
        item: Uuid,
        kind: crate::screen::metadata::artwork::Kind,
        index: i32,
        to: i32,
    ) -> Answer<()> {
        Answer::of(async {
            let response = self
                .http
                .post(format!(
                    "{}/Items/{item}/Images/{}/{index}/Index?newIndex={to}",
                    self.base,
                    kind.as_str()
                ))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    pub async fn remote_image_providers(&self, item: Uuid) -> Answer<Vec<String>> {
        Answer::of(async {
            Ok(self
                .client
                .get_remote_image_providers(&item)
                .await?
                .into_iter()
                .filter_map(|provider| provider.name)
                .collect())
        })
        .await
    }

    /// The remote images the server offers; every url is a minted handle.
    pub async fn remote_images(
        &self,
        item: Uuid,
        kind: crate::screen::metadata::artwork::Kind,
        provider: Option<&str>,
    ) -> Answer<Vec<jellyfin_api::types::RemoteImageInfo>> {
        Answer::of(async {
            let mut path = format!(
                "/Items/{item}/RemoteImages?type={}&limit={REMOTE_IMAGES}",
                kind.as_str()
            );
            if let Some(provider) = provider {
                path.push_str(&format!("&providerName={}", urlencode(provider)));
            }
            let response = self.http.get(format!("{}{path}", self.base)).send().await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            let answered: jellyfin_api::types::RemoteImageResult = response.json().await?;
            Ok(answered.images.unwrap_or_default())
        })
        .await
    }

    /// The Jellyfin server downloads the image the handle names; no bytes cross
    /// the browser.
    pub async fn download_remote_image(
        &self,
        item: Uuid,
        kind: crate::screen::metadata::artwork::Kind,
        handle: &str,
    ) -> Answer<()> {
        Answer::of(async {
            let path = format!(
                "/Items/{item}/RemoteImages/Download?type={}&imageUrl={}",
                kind.as_str(),
                urlencode(handle)
            );
            let response = self
                .http
                .post(format!("{}{path}", self.base))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    /// One foreign image, fetched from the local server by the handle it minted.
    pub async fn foreign_image(&self, handle: &str) -> Answer<Vec<u8>> {
        Answer::of(async {
            let response = self
                .http
                .get(crate::images::foreign_url(handle))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(response.bytes().await?.to_vec())
        })
        .await
    }

    /// The people a search names; each opens that person's filtered list.
    pub async fn people(&self, term: &str, limit: i32) -> Answer<Vec<BaseItemDto>> {
        Answer::of(async {
            let fields = fields();
            Ok(self
                .client
                .get_persons(&jellyfin_api::query::GetPersons {
                    enable_images: Some(true),
                    enable_user_data: Some(true),
                    fields: Some(&fields),
                    limit: Some(limit),
                    search_term: Some(term),
                    user_id: Some(&self.user_id),
                    ..Default::default()
                })
                .await?
                .items)
        })
        .await
    }

    /// The trickplay each of the item's media sources offers.
    pub async fn trickplay(&self, item: Uuid) -> Answer<crate::player::trickplay::Trickplay> {
        Answer::of(async {
            Ok(crate::player::trickplay::Trickplay::of(
                &self.item(item).await.bubbled()?,
            ))
        })
        .await
    }

    /// One trickplay tile sheet.
    pub async fn trickplay_tile(
        &self,
        item: Uuid,
        media_source: &str,
        width: u32,
        index: u32,
    ) -> Answer<Vec<u8>> {
        Answer::of(async {
            let response = self
                .http
                .get(format!(
                    "{}/Videos/{item}/Trickplay/{width}/{index}.jpg?mediaSourceId={}",
                    self.base,
                    urlencode(media_source)
                ))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(response.bytes().await?.to_vec())
        })
        .await
    }

    pub async fn server_configuration(&self) -> Answer<serde_json::Value> {
        Answer::of(async { Ok(self.read_whole("/System/Configuration").await?) }).await
    }

    pub async fn save_server_configuration(&self, body: &serde_json::Value) -> Answer<()> {
        Answer::of(async { Ok(self.write_whole("/System/Configuration", body).await?) }).await
    }

    pub async fn scan_library(&self, name: &str) -> Answer<()> {
        Answer::of(async {
            let encoded = urlencode(name);
            self.write_whole(
                &format!("/Library/VirtualFolders/Name?name={encoded}&newName={encoded}"),
                &serde_json::Value::Null,
            )
            .await?;
            self.scan_all().await.bubbled()
        })
        .await
    }

    // the query string is where a re-read's two choices become the words the
    // server reads
    pub async fn refresh_item(
        &self,
        item: Uuid,
        replace: jellium_model::item::Replace,
        scope: jellium_model::item::Scope,
    ) -> Answer<()> {
        Answer::of(async {
            let (replace, recursive) = (replace.all(), scope.recursive());
            Ok(self
                .write_whole(
                    &format!(
                        "/Items/{item}/Refresh?replaceAllMetadata={replace}&recursive={recursive}&metadataRefreshMode=FullRefresh&imageRefreshMode=FullRefresh"
                    ),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn user(&self, id: Uuid) -> Answer<jellyfin_api::types::UserDto> {
        Answer::of(async { Ok(self.client.get_user_by_id(&id).await?) }).await
    }

    pub async fn create_user(&self, name: &str, password: &str) -> Answer<Uuid> {
        Answer::of(async {
            let made = self
                .client
                .create_user_by_name(&jellyfin_api::types::CreateUserByName {
                    name: name.to_owned(),
                    password: (!password.is_empty()).then(|| password.to_owned()),
                })
                .await?;
            made.id.ok_or_else(|| {
                Bubble::from(Trouble::Relay {
                    status: None,
                    detail: "the server named no user".to_owned(),
                })
            })
        })
        .await
    }

    pub async fn delete_user(&self, id: Uuid) -> Answer<()> {
        Answer::of(async {
            self.client.delete_user(&id).await?;
            Ok(())
        })
        .await
    }

    /// The user's policy, read whole so a save preserves what no control
    /// covers.
    pub async fn policy(&self, id: Uuid) -> Answer<serde_json::Value> {
        Answer::of(async {
            Ok(self
                .read_whole(&format!("/Users/{id}"))
                .await?
                .get("Policy")
                .cloned()
                .unwrap_or(serde_json::Value::Null))
        })
        .await
    }

    pub async fn save_policy(&self, id: Uuid, body: &serde_json::Value) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(&format!("/Users/{id}/Policy"), body)
                .await?)
        })
        .await
    }

    /// The user's own configuration, read whole.
    pub async fn user_configuration(&self, id: Uuid) -> Answer<serde_json::Value> {
        Answer::of(async {
            Ok(self
                .read_whole(&format!("/Users/{id}"))
                .await?
                .get("Configuration")
                .cloned()
                .unwrap_or(serde_json::Value::Null))
        })
        .await
    }

    pub async fn save_user_configuration(&self, id: Uuid, body: &serde_json::Value) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(&format!("/Users/{id}/Configuration"), body)
                .await?)
        })
        .await
    }

    pub async fn set_password(&self, id: Uuid, current: Option<&str>, new: &str) -> Answer<()> {
        Answer::of(async {
            let body = PasswordChange {
                current_pw: current.unwrap_or_default(),
                new_pw: new,
                reset_password: false,
            };
            Ok(self
                .write_whole(&format!("/Users/{id}/Password"), &body)
                .await?)
        })
        .await
    }

    pub async fn remove_user_image(&self, id: Uuid) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/Users/{id}/Images/Primary"))
                .await?)
        })
        .await
    }

    /// Posts the chosen image's bytes with `mime` as their content type; the
    /// relay is what base64-encodes them for Jellyfin.
    pub async fn upload_user_image(&self, id: Uuid, mime: &str, bytes: Vec<u8>) -> Answer<()> {
        Answer::of(async {
            let response = self
                .http
                .post(format!("{}/Users/{id}/Images/Primary", self.base))
                .header(reqwest::header::CONTENT_TYPE, mime)
                .body(bytes)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    /// The signed-in user, read whole so a name change preserves every field no
    /// control covers.
    pub async fn user_whole(&self, id: Uuid) -> Answer<serde_json::Value> {
        Answer::of(async { Ok(self.read_whole(&format!("/Users/{id}")).await?) }).await
    }

    pub async fn save_user(&self, id: Uuid, body: &serde_json::Value) -> Answer<()> {
        Answer::of(async { Ok(self.write_whole(&format!("/Users/{id}"), body).await?) }).await
    }

    /// Authorizes `code` for the signed-in user; `authorized_here` is what tells
    /// an expired code from an unknown one.
    pub async fn authorize_quick_connect(
        &self,
        code: &str,
        authorized_here: bool,
    ) -> Answer<quickconnect::Outcome> {
        Answer::of(async {
            let response = self
                .http
                .post(format!(
                    "{}/QuickConnect/Authorize?code={}&userId={}",
                    self.base,
                    urlencode(code),
                    self.user_id
                ))
                .send()
                .await?;
            let status = response.status().as_u16();
            let message = match response.text().await {
                Ok(message) => message,
                Err(error) => return Err(Bubble::from(error)),
            };
            Ok(quickconnect::outcome(status, &message, authorized_here))
        })
        .await
    }

    /// The preference bag under `client`, and `Bag::missing()` when the server
    /// holds no record.
    pub async fn preferences(&self, client: &str) -> Answer<prefs::Bag> {
        Answer::of(async {
            let encoded = urlencode(client);
            let response = self
                .http
                .get(format!(
                    "{}/DisplayPreferences/{}?client={encoded}&userId={}",
                    self.base,
                    prefs::RECORD,
                    self.user_id
                ))
                .send()
                .await?;
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Ok(prefs::Bag::missing());
            }
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            match response.json::<DisplayPreferencesDto>().await {
                Ok(read) => Ok(prefs::Bag::of(read)),
                Err(error) => Err(Bubble::from(error)),
            }
        })
        .await
    }

    pub async fn save_preferences(
        &self,
        client: &str,
        record: &DisplayPreferencesDto,
    ) -> Answer<()> {
        Answer::of(async {
            let encoded = urlencode(client);
            let response = self
                .http
                .post(format!(
                    "{}/DisplayPreferences/{}?client={encoded}&userId={}",
                    self.base,
                    prefs::RECORD,
                    self.user_id
                ))
                .json(record)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            Ok(())
        })
        .await
    }

    pub async fn create_library(
        &self,
        name: &str,
        content_type: &str,
        options: &serde_json::Value,
    ) -> Answer<()> {
        Answer::of(async {
            let body = LibraryOptionsBody {
                library_options: options,
            };
            Ok(self
                .write_whole(
                    &format!(
                        "/Library/VirtualFolders?name={}&collectionType={}",
                        urlencode(name),
                        urlencode(content_type)
                    ),
                    &body,
                )
                .await?)
        })
        .await
    }

    pub async fn rename_library(&self, name: &str, renamed: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!(
                        "/Library/VirtualFolders/Name?name={}&newName={}",
                        urlencode(name),
                        urlencode(renamed)
                    ),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn delete_library(&self, name: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/Library/VirtualFolders?name={}", urlencode(name)))
                .await?)
        })
        .await
    }

    pub async fn add_path(&self, library: &str, path: &str) -> Answer<()> {
        Answer::of(async {
            let body = MediaPath {
                name: library,
                path,
            };
            Ok(self
                .write_whole("/Library/VirtualFolders/Paths", &body)
                .await?)
        })
        .await
    }

    pub async fn remove_path(&self, library: &str, path: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!(
                    "/Library/VirtualFolders/Paths?name={}&path={}",
                    urlencode(library),
                    urlencode(path)
                ))
                .await?)
        })
        .await
    }

    /// The library's options, read whole so a save preserves what no control
    /// covers.
    pub async fn library_options(&self, name: &str) -> Answer<serde_json::Value> {
        Answer::of(async {
            Ok(self
                .read_whole("/Library/VirtualFolders")
                .await?
                .as_array()
                .into_iter()
                .flatten()
                .find(|folder| folder.get("Name").and_then(serde_json::Value::as_str) == Some(name))
                .and_then(|folder| folder.get("LibraryOptions").cloned())
                .unwrap_or(serde_json::Value::Null))
        })
        .await
    }

    pub async fn save_library_options(&self, id: &str, options: &serde_json::Value) -> Answer<()> {
        Answer::of(async {
            let body = LibraryOptionsUpdate {
                id,
                library_options: options,
            };
            Ok(self
                .write_whole("/Library/VirtualFolders/LibraryOptions", &body)
                .await?)
        })
        .await
    }

    pub async fn drives(&self) -> Answer<Vec<jellyfin_api::types::FileSystemEntryInfo>> {
        Answer::of(async { Ok(self.client.get_drives().await?) }).await
    }

    pub async fn directory(
        &self,
        path: &str,
    ) -> Answer<Vec<jellyfin_api::types::FileSystemEntryInfo>> {
        Answer::of(async {
            Ok(self
                .client
                .get_directory_contents(Some(true), Some(false), path)
                .await?)
        })
        .await
    }

    pub async fn system_info(&self) -> Answer<jellyfin_api::types::SystemInfo> {
        Answer::of(async { Ok(self.client.get_system_info().await?) }).await
    }

    pub async fn users(&self) -> Answer<Vec<jellyfin_api::types::UserDto>> {
        Answer::of(async { Ok(self.client.get_users(None, None).await?) }).await
    }

    pub async fn virtual_folders(&self) -> Answer<Vec<jellyfin_api::types::VirtualFolderInfo>> {
        Answer::of(async { Ok(self.client.get_virtual_folders().await?) }).await
    }

    pub async fn tasks(&self) -> Answer<Vec<jellyfin_api::types::TaskInfo>> {
        Answer::of(async { Ok(self.client.get_tasks(None, None).await?) }).await
    }

    pub async fn task(&self, id: &str) -> Answer<jellyfin_api::types::TaskInfo> {
        Answer::of(async { Ok(self.client.get_task(id).await?) }).await
    }

    pub async fn start_task(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!("/ScheduledTasks/Running/{}", urlencode(id)),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn stop_task(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/ScheduledTasks/Running/{}", urlencode(id)))
                .await?)
        })
        .await
    }

    pub async fn set_triggers(
        &self,
        id: &str,
        triggers: &[jellyfin_api::types::TaskTriggerInfo],
    ) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!("/ScheduledTasks/{}/Triggers", urlencode(id)),
                    triggers,
                )
                .await?)
        })
        .await
    }

    pub async fn log_files(&self) -> Answer<Vec<jellyfin_api::types::LogFile>> {
        Answer::of(async { Ok(self.client.get_server_logs().await?) }).await
    }

    /// The tail of `name` and the file's full length, read from the relay's
    /// `Content-Range`.
    /// A `404` reads as `Trouble::LogMissing`.
    pub async fn log_tail(&self, name: &str) -> Answer<jellium_model::log::Tail> {
        Answer::of(async {
            let response = self
                .http
                .get(format!(
                    "{}/System/Logs/Log?name={}",
                    self.base,
                    urlencode(name)
                ))
                .send()
                .await?;
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(Bubble::from(Trouble::LogMissing {
                    name: name.to_owned(),
                }));
            }
            if !response.status().is_success() {
                return Err(crate::error::classify(response).await.into());
            }
            let size = match response.headers().get(reqwest::header::CONTENT_RANGE) {
                Some(value) => match value.to_str() {
                    Ok(held) => held
                        .rsplit('/')
                        .next()
                        .and_then(|full| crate::failure::read::<u64>(Text::FailureRange, full)),
                    Err(_) => None,
                },
                None => None,
            };
            let text = response.text().await?;
            let size = size.unwrap_or(text.len() as u64);
            Ok(jellium_model::log::Tail::of(
                text,
                jellium_model::log::Bytes::of(size),
            ))
        })
        .await
    }

    /// The activity entries `start`..`start + limit`, and how many the server
    /// holds.
    pub async fn activity(
        &self,
        start: i32,
        limit: i32,
        with_user: Option<bool>,
    ) -> Answer<(Vec<jellium_protocol::ActivityEntry>, usize)> {
        Answer::of(async {
            let answered = self
                .client
                .get_log_entries(with_user, Some(limit), None, Some(start))
                .await?;
            let total = answered.total_record_count.unwrap_or(0).max(0) as usize;
            let entries = answered.items.into_iter().filter_map(read_entry).collect();
            Ok((entries, total))
        })
        .await
    }

    pub async fn plugins(&self) -> Answer<Vec<jellyfin_api::types::PluginInfo>> {
        Answer::of(async { Ok(self.client.get_plugins().await?) }).await
    }

    pub async fn enable_plugin(&self, id: Uuid, version: &str) -> Answer<()> {
        Answer::of(async {
            self.client.enable_plugin(&id, version).await?;
            Ok(())
        })
        .await
    }

    pub async fn disable_plugin(&self, id: Uuid, version: &str) -> Answer<()> {
        Answer::of(async {
            self.client.disable_plugin(&id, version).await?;
            Ok(())
        })
        .await
    }

    pub async fn uninstall_plugin(&self, id: Uuid, version: &str) -> Answer<()> {
        Answer::of(async {
            self.client
                .uninstall_plugin_by_version(&id, version)
                .await?;
            Ok(())
        })
        .await
    }

    /// The plugin's configuration, read whole so a save preserves the plugin's
    /// own fields.
    pub async fn plugin_configuration(&self, id: Uuid) -> Answer<serde_json::Value> {
        Answer::of(async { Ok(self.client.get_plugin_configuration(&id).await?) }).await
    }

    pub async fn save_plugin_configuration(
        &self,
        id: Uuid,
        body: &serde_json::Value,
    ) -> Answer<()> {
        Answer::of(async {
            self.client.update_plugin_configuration(&id, body).await?;
            Ok(())
        })
        .await
    }

    pub async fn packages(&self) -> Answer<Vec<jellyfin_api::types::PackageInfo>> {
        Answer::of(async { Ok(self.client.get_packages().await?) }).await
    }

    pub async fn install_package(&self, name: &str, version: &str, repository: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!(
                        "/Packages/Installed/{}?version={}&repositoryUrl={}",
                        urlencode(name),
                        urlencode(version),
                        urlencode(repository)
                    ),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn cancel_install(&self, package: Uuid) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/Packages/Installing/{package}"))
                .await?)
        })
        .await
    }

    pub async fn repositories(&self) -> Answer<Vec<jellyfin_api::types::RepositoryInfo>> {
        Answer::of(async { Ok(self.client.get_repositories().await?) }).await
    }

    pub async fn save_repositories(
        &self,
        repositories: &[jellyfin_api::types::RepositoryInfo],
    ) -> Answer<()> {
        Answer::of(async { Ok(self.write_whole("/Repositories", repositories).await?) }).await
    }

    pub async fn devices(&self) -> Answer<Vec<jellyfin_api::types::DeviceInfoDto>> {
        Answer::of(async { Ok(self.client.get_devices(None).await?.items) }).await
    }

    pub async fn delete_device(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/Devices?id={}", urlencode(id)))
                .await?)
        })
        .await
    }

    pub async fn set_device_name(&self, id: &str, name: &str) -> Answer<()> {
        Answer::of(async {
            let body = DeviceName { custom_name: name };
            Ok(self
                .write_whole(&format!("/Devices/Options?id={}", urlencode(id)), &body)
                .await?)
        })
        .await
    }

    pub async fn keys(&self) -> Answer<Vec<jellyfin_api::types::AuthenticationInfo>> {
        Answer::of(async { Ok(self.client.get_keys().await?.items) }).await
    }

    pub async fn create_key(&self, app: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!("/Auth/Keys?app={}", urlencode(app)),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn revoke_key(&self, key: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/Auth/Keys/{}", urlencode(key)))
                .await?)
        })
        .await
    }

    pub async fn tuner_types(&self) -> Answer<Vec<jellyfin_api::types::NameIdPair>> {
        Answer::of(async { Ok(self.client.get_tuner_host_types().await?) }).await
    }

    /// The tuner hosts the server holds, read from the Live TV section it
    /// keeps them in.
    pub async fn tuners(&self) -> Answer<Vec<jellyfin_api::types::TunerHostInfo>> {
        Answer::of(async {
            Ok(self
                .section("livetv")
                .await
                .bubbled()?
                .get("TunerHosts")
                .cloned()
                .and_then(|held| crate::failure::parsed(Text::FailureLiveTvSection, held))
                .unwrap_or_default())
        })
        .await
    }

    pub async fn providers(&self) -> Answer<Vec<jellyfin_api::types::ListingsProviderInfo>> {
        Answer::of(async {
            Ok(self
                .section("livetv")
                .await
                .bubbled()?
                .get("ListingProviders")
                .cloned()
                .and_then(|held| crate::failure::parsed(Text::FailureLiveTvSection, held))
                .unwrap_or_default())
        })
        .await
    }

    pub async fn discover_tuners(&self) -> Answer<Vec<jellyfin_api::types::TunerHostInfo>> {
        Answer::of(async {
            let held = self.read_whole("/LiveTv/Tuners/Discover").await?;
            Ok(crate::failure::parsed(Text::FailureLiveTvSection, held).unwrap_or_default())
        })
        .await
    }

    pub async fn add_tuner(&self, url: &str, kind: &str) -> Answer<()> {
        Answer::of(async {
            let body = TunerHost { url, r#type: kind };
            Ok(self.write_whole("/LiveTv/TunerHosts", &body).await?)
        })
        .await
    }

    pub async fn delete_tuner(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/LiveTv/TunerHosts?id={}", urlencode(id)))
                .await?)
        })
        .await
    }

    pub async fn reset_tuner(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole(
                    &format!("/LiveTv/Tuners/{}/Reset", urlencode(id)),
                    &serde_json::Value::Null,
                )
                .await?)
        })
        .await
    }

    pub async fn add_provider<T: serde::Serialize>(&self, provider: &T) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .write_whole("/LiveTv/ListingProviders", provider)
                .await?)
        })
        .await
    }

    pub async fn delete_provider(&self, id: &str) -> Answer<()> {
        Answer::of(async {
            Ok(self
                .delete_whole(&format!("/LiveTv/ListingProviders?id={}", urlencode(id)))
                .await?)
        })
        .await
    }

    pub async fn lineups(
        &self,
        country: &str,
        location: &str,
    ) -> Answer<Vec<jellyfin_api::types::NameIdPair>> {
        Answer::of(async {
            Ok(self
                .client
                .get_lineups(Some(country), None, Some(location), Some("SchedulesDirect"))
                .await?)
        })
        .await
    }

    pub async fn schedules_direct_countries(&self) -> Answer<serde_json::Value> {
        Answer::of(async {
            Ok(self
                .read_whole("/LiveTv/ListingProviders/SchedulesDirect/Countries")
                .await?)
        })
        .await
    }

    pub async fn mapping_options(
        &self,
        provider: &str,
    ) -> Answer<jellyfin_api::types::ChannelMappingOptionsDto> {
        Answer::of(async {
            Ok(self
                .client
                .get_channel_mapping_options(Some(provider))
                .await?)
        })
        .await
    }

    pub async fn map_channel(&self, provider: &str, tuner: &str, channel: &str) -> Answer<()> {
        Answer::of(async {
            let body = ChannelMapping {
                provider_id: provider,
                tuner_channel_id: tuner,
                provider_channel_id: channel,
            };
            Ok(self.write_whole("/LiveTv/ChannelMappings", &body).await?)
        })
        .await
    }

    pub async fn configuration_pages(
        &self,
    ) -> Answer<Vec<jellyfin_api::types::ConfigurationPageInfo>> {
        Answer::of(async { Ok(self.client.get_configuration_pages(None).await?) }).await
    }
}

/// The timer a program's server defaults describe, carried across every field
/// the two dtos share.
fn timer_of(defaults: &SeriesTimerInfoDto) -> TimerInfoDto {
    TimerInfoDto {
        channel_id: defaults.channel_id,
        channel_name: defaults.channel_name.clone(),
        channel_primary_image_tag: defaults.channel_primary_image_tag.clone(),
        end_date: defaults.end_date,
        external_channel_id: defaults.external_channel_id.clone(),
        external_id: defaults.external_id.clone(),
        external_program_id: defaults.external_program_id.clone(),
        external_series_timer_id: None,
        id: None,
        is_post_padding_required: defaults.is_post_padding_required,
        is_pre_padding_required: defaults.is_pre_padding_required,
        keep_until: defaults.keep_until,
        name: defaults.name.clone(),
        overview: defaults.overview.clone(),
        parent_backdrop_image_tags: defaults.parent_backdrop_image_tags.clone(),
        parent_backdrop_item_id: defaults.parent_backdrop_item_id.clone(),
        post_padding_seconds: defaults.post_padding_seconds,
        pre_padding_seconds: defaults.pre_padding_seconds,
        priority: defaults.priority,
        program_id: defaults.program_id.clone(),
        program_info: None,
        run_time_ticks: None,
        series_timer_id: None,
        server_id: defaults.server_id.clone(),
        service_name: defaults.service_name.clone(),
        start_date: defaults.start_date,
        status: None,
        type_: defaults.type_.clone(),
    }
}

const RAIL_LIMIT: i32 = 24;

/// The most items an instant mix queues.
const MIX_LIMIT: i32 = 200;

/// One page of a playlist's entries, and how many it holds.
pub struct Entries {
    pub entries: Vec<crate::screen::playlists::Entry>,
    pub total: i32,
}

/// What `/Users/{id}/Password` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct PasswordChange<'a> {
    current_pw: &'a str,
    new_pw: &'a str,
    reset_password: bool,
}

/// What `/Library/VirtualFolders` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct LibraryOptionsBody<'a> {
    library_options: &'a serde_json::Value,
}

/// What `/Library/VirtualFolders/LibraryOptions` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct LibraryOptionsUpdate<'a> {
    id: &'a str,
    library_options: &'a serde_json::Value,
}

/// What `/Library/VirtualFolders/Paths` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct MediaPath<'a> {
    name: &'a str,
    path: &'a str,
}

/// What `/Devices/Options` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct DeviceName<'a> {
    custom_name: &'a str,
}

/// What `/LiveTv/TunerHosts` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct TunerHost<'a> {
    url: &'a str,
    r#type: &'a str,
}

/// What `/LiveTv/ChannelMappings` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct ChannelMapping<'a> {
    provider_id: &'a str,
    tuner_channel_id: &'a str,
    provider_channel_id: &'a str,
}
