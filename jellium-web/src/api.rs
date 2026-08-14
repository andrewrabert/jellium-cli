use jellyfin_api::types::{BaseItemDto, BaseItemKind, ItemFields, UserItemDataDto};
use uuid::Uuid;

use crate::error::Trouble;
use crate::images;
use crate::screen::library::Sort;

#[derive(Debug, Clone, Default)]
pub struct Page {
    pub items: Vec<BaseItemDto>,
    pub total: i32,
}

#[derive(Default)]
struct Query {
    parent_id: Option<Uuid>,
    album_artist_ids: Option<Vec<Uuid>>,
    include_item_types: Option<Vec<BaseItemKind>>,
    search_term: Option<String>,
    sort: Sort,
    start: i32,
    limit: Option<i32>,
}

pub struct Api {
    client: jellyfin_api::Client,
    http: reqwest::Client,
    base: String,
    user_id: Uuid,
}

fn fields() -> Vec<ItemFields> {
    vec![
        ItemFields::Overview,
        ItemFields::ParentId,
        ItemFields::PrimaryImageAspectRatio,
    ]
}

impl Api {
    pub fn new(user_id: Uuid) -> Api {
        let origin = web_sys::window()
            .expect("a browser window")
            .location()
            .origin()
            .expect("the page has an origin");
        let base = format!("{origin}{}", jellium_protocol::RELAY_PREFIX);
        let http = reqwest::Client::new();
        Api {
            client: jellyfin_api::Client::new(&base, http.clone()),
            http,
            base,
            user_id,
        }
    }

    async fn query(&self, query: Query) -> Result<Page, Trouble> {
        let fields = fields();
        let (by, order) = query.sort.query();
        let sort_by = vec![by];
        let sort_order = vec![order];

        let result = self
            .client
            .get_items(
                None,                              // adjacent_to
                query.album_artist_ids.as_ref(),   // album_artist_ids
                None,                              // album_ids
                None,                              // albums
                None,                              // artist_ids
                None,                              // artists
                None,                              // collapse_box_set_items
                None,                              // contributing_artist_ids
                None,                              // enable_image_types
                Some(true),                        // enable_images
                Some(true),                        // enable_total_record_count
                Some(true),                        // enable_user_data
                None,                              // exclude_artist_ids
                None,                              // exclude_item_ids
                None,                              // exclude_item_types
                None,                              // exclude_location_types
                Some(&fields),                     // fields
                None,                              // filters
                None,                              // genre_ids
                None,                              // genres
                None,                              // has_imdb_id
                None,                              // has_official_rating
                None,                              // has_overview
                None,                              // has_parental_rating
                None,                              // has_special_feature
                None,                              // has_subtitles
                None,                              // has_theme_song
                None,                              // has_theme_video
                None,                              // has_tmdb_id
                None,                              // has_trailer
                None,                              // has_tvdb_id
                None,                              // ids
                None,                              // image_type_limit
                None,                              // image_types
                query.include_item_types.as_ref(), // include_item_types
                None,                              // index_number
                None,                              // is3_d
                None,                              // is4_k
                None,                              // is_favorite
                None,                              // is_hd
                None,                              // is_kids
                None,                              // is_locked
                None,                              // is_missing
                None,                              // is_movie
                None,                              // is_news
                None,                              // is_place_holder
                None,                              // is_played
                None,                              // is_series
                None,                              // is_sports
                None,                              // is_unaired
                query.limit,                       // limit
                None,                              // location_types
                None,                              // max_height
                None,                              // max_official_rating
                None,                              // max_premiere_date
                None,                              // max_width
                None,                              // media_types
                None,                              // min_community_rating
                None,                              // min_critic_rating
                None,                              // min_date_last_saved
                None,                              // min_date_last_saved_for_user
                None,                              // min_height
                None,                              // min_official_rating
                None,                              // min_premiere_date
                None,                              // min_width
                None,                              // name_less_than
                None,                              // name_starts_with
                None,                              // name_starts_with_or_greater
                None,                              // official_ratings
                query.parent_id.as_ref(),          // parent_id
                None,                              // parent_index_number
                None,                              // person
                None,                              // person_ids
                None,                              // person_types
                Some(true),                        // recursive
                query.search_term.as_deref(),      // search_term
                None,                              // series_status
                Some(&sort_by),                    // sort_by
                Some(&sort_order),                 // sort_order
                Some(query.start),                 // start_index
                None,                              // studio_ids
                None,                              // studios
                None,                              // tags
                Some(&self.user_id),               // user_id
                None,                              // video_types
                None,                              // years
            )
            .await?;

        Ok(Page {
            total: result
                .total_record_count
                .unwrap_or(result.items.len() as i32),
            items: result.items,
        })
    }

    pub async fn libraries(&self) -> Result<Vec<BaseItemDto>, Trouble> {
        Ok(self
            .client
            .get_user_views(None, None, None, Some(&self.user_id))
            .await?
            .items)
    }

    pub async fn continue_watching(&self) -> Result<Vec<BaseItemDto>, Trouble> {
        let fields = fields();
        Ok(self
            .client
            .get_resume_items(
                None,
                Some(true),
                None,
                Some(true),
                None,
                None,
                Some(&fields),
                None,
                None,
                Some(RAIL_LIMIT),
                None,
                None,
                None,
                None,
                Some(&self.user_id),
            )
            .await?
            .items)
    }

    pub async fn next_up(&self) -> Result<Vec<BaseItemDto>, Trouble> {
        let fields = fields();
        Ok(self
            .client
            .get_next_up(
                None,
                None,
                Some(true),
                None,
                None,
                None,
                Some(true),
                Some(&fields),
                None,
                Some(RAIL_LIMIT),
                None,
                None,
                None,
                None,
                Some(&self.user_id),
            )
            .await?
            .items)
    }

    pub async fn page(
        &self,
        library: Uuid,
        sort: Sort,
        start: i32,
        limit: i32,
    ) -> Result<Page, Trouble> {
        self.query(Query {
            parent_id: Some(library),
            sort,
            start,
            limit: Some(limit),
            ..Query::default()
        })
        .await
    }

    pub async fn item(&self, item: Uuid) -> Result<BaseItemDto, Trouble> {
        Ok(self.client.get_item(&item, Some(&self.user_id)).await?)
    }

    pub async fn children(&self, item: &BaseItemDto) -> Result<Vec<BaseItemDto>, Trouble> {
        let Some(id) = item.id else {
            return Ok(Vec::new());
        };
        let fields = fields();

        match item.type_ {
            Some(BaseItemKind::Series) => Ok(self
                .client
                .get_seasons(
                    &id,
                    None,
                    None,
                    Some(true),
                    Some(true),
                    Some(&fields),
                    None,
                    None,
                    None,
                    Some(&self.user_id),
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
                        None,
                        None,
                        Some(true),
                        Some(true),
                        Some(&fields),
                        None,
                        None,
                        None,
                        None,
                        Some(&id),
                        None,
                        None,
                        None,
                        Some(&self.user_id),
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
            _ => Ok(Vec::new()),
        }
    }

    pub async fn search(&self, term: &str, start: i32, limit: i32) -> Result<Page, Trouble> {
        self.query(Query {
            search_term: Some(term.to_string()),
            include_item_types: Some(vec![
                BaseItemKind::Movie,
                BaseItemKind::Series,
                BaseItemKind::Episode,
                BaseItemKind::MusicAlbum,
                BaseItemKind::MusicArtist,
                BaseItemKind::Audio,
            ]),
            start,
            limit: Some(limit),
            ..Query::default()
        })
        .await
    }

    pub async fn set_played(&self, item: Uuid, played: bool) -> Result<UserItemDataDto, Trouble> {
        Ok(if played {
            self.client
                .mark_played_item(&item, None, Some(&self.user_id))
                .await?
        } else {
            self.client
                .mark_unplayed_item(&item, Some(&self.user_id))
                .await?
        })
    }

    pub async fn set_favorite(
        &self,
        item: Uuid,
        favorite: bool,
    ) -> Result<UserItemDataDto, Trouble> {
        Ok(if favorite {
            self.client
                .mark_favorite_item(&item, Some(&self.user_id))
                .await?
        } else {
            self.client
                .unmark_favorite_item(&item, Some(&self.user_id))
                .await?
        })
    }

    pub fn image_url(&self, key: images::Key) -> String {
        format!(
            "{}/Items/{}/Images/{}?fillWidth={}",
            self.base,
            key.item,
            key.kind.as_str(),
            key.width,
        )
    }

    /// A non-2xx answer is classified, so a revoked token reads as
    /// `Failure::TokenRejected` rather than as a transport error.
    pub async fn image(&self, url: String) -> Result<Vec<u8>, Trouble> {
        let response = self.http.get(url).send().await?;
        if !response.status().is_success() {
            return Err(crate::error::classify(response).await);
        }
        Ok(response.bytes().await?.to_vec())
    }
}

const RAIL_LIMIT: i32 = 24;
