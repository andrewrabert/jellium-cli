use clap::Subcommand;
use jellyfin_api::types::{ItemFields, ItemSortBy, SortOrder};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum MoviesCommand {
    /// Get similar movies
    Similar {
        /// The item ID
        item_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,
    },

    /// Get movie recommendations
    Recommendations {
        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Max number of categories
        #[arg(long)]
        category_limit: Option<i32>,

        /// Max items per category
        #[arg(long)]
        item_limit: Option<i32>,
    },

    /// Find trailers
    Trailers {
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,

        /// Search term
        #[arg(long)]
        search_term: Option<String>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Sort by fields
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &MoviesCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        MoviesCommand::Similar {
            item_id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_similar_movies(
                    item_id,
                    None, // exclude_artist_ids
                    fields.as_ref(),
                    *limit,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        MoviesCommand::Recommendations {
            user_id: uid,
            fields,
            parent_id,
            category_limit,
            item_limit,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_movie_recommendations(
                    *category_limit,
                    fields.as_ref(),
                    *item_limit,
                    parent_id.as_ref(),
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        MoviesCommand::Trailers {
            limit,
            start_index,
            parent_id,
            recursive,
            search_term,
            fields,
            sort_by,
            sort_order,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_trailers(
                    None, // adjacent_to
                    None, // album_artist_ids
                    None, // album_ids
                    None, // albums
                    None, // artist_ids
                    None, // artists
                    None, // collapse_box_set_items
                    None, // contributing_artist_ids
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_total_record_count
                    None, // enable_user_data
                    None, // exclude_artist_ids
                    None, // exclude_item_ids
                    None, // exclude_item_types
                    None, // exclude_location_types
                    fields.as_ref(),
                    None, // filters
                    None, // genre_ids
                    None, // genres
                    None, // has_imdb_id
                    None, // has_official_rating
                    None, // has_overview
                    None, // has_parental_rating
                    None, // has_special_feature
                    None, // has_subtitles
                    None, // has_theme_song
                    None, // has_theme_video
                    None, // has_tmdb_id
                    None, // has_trailer
                    None, // has_tvdb_id
                    None, // ids
                    None, // image_type_limit
                    None, // image_types
                    None, // is3_d
                    None, // is4_k
                    None, // is_favorite
                    None, // is_hd
                    None, // is_kids
                    None, // is_locked
                    None, // is_missing
                    None, // is_movie
                    None, // is_news
                    None, // is_place_holder
                    None, // is_played
                    None, // is_series
                    None, // is_sports
                    None, // is_unaired
                    *limit,
                    None, // location_types
                    None, // max_height
                    None, // max_official_rating
                    None, // max_premiere_date
                    None, // max_width
                    None, // media_types
                    None, // min_community_rating
                    None, // min_critic_rating
                    None, // min_date_last_saved
                    None, // min_date_last_saved_for_user
                    None, // min_height
                    None, // min_official_rating
                    None, // min_premiere_date
                    None, // min_width
                    None, // name_less_than
                    None, // name_starts_with
                    None, // name_starts_with_or_greater
                    None, // official_ratings
                    parent_id.as_ref(),
                    None, // parent_index_number
                    None, // person
                    None, // person_ids
                    None, // person_types
                    *recursive,
                    search_term.as_deref(),
                    None, // series_status
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    None, // studio_ids
                    None, // studios
                    None, // tags
                    Some(effective_uid),
                    None, // video_types
                    None, // years
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
    }
    Ok(())
}
