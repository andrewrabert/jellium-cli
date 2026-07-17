use clap::Subcommand;
use jellyfin_api::types::{BaseItemKind, ItemFields, ItemFilter, ItemSortBy, SortOrder};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum ArtistsCommand {
    /// List all artists
    List {
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Search term
        #[arg(long)]
        search_term: Option<String>,

        /// Sort by fields (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Include item types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Exclude item types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<BaseItemKind>>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,

        /// Filter by genres (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        genres: Option<Vec<String>>,

        /// Filter by genre IDs (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        genre_ids: Option<Vec<Uuid>>,

        /// Additional filters (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        filters: Option<Vec<ItemFilter>>,

        /// Filter by items whose name starts with
        #[arg(long)]
        name_starts_with: Option<String>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get an artist by name
    Get {
        /// Artist name
        name: String,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// List all album artists
    AlbumArtists {
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Search term
        #[arg(long)]
        search_term: Option<String>,

        /// Sort by fields (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Include item types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Exclude item types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<BaseItemKind>>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,

        /// Filter by genres (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        genres: Option<Vec<String>>,

        /// Filter by genre IDs (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        genre_ids: Option<Vec<Uuid>>,

        /// Additional filters (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        filters: Option<Vec<ItemFilter>>,

        /// Filter by items whose name starts with
        #[arg(long)]
        name_starts_with: Option<String>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Create an instant mix from an artist
    InstantMix {
        /// The artist item ID
        item_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,
    },

    /// Get similar albums
    SimilarAlbums {
        /// The album item ID
        item_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,
    },

    /// Get similar artists
    SimilarArtists {
        /// The artist item ID
        item_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &ArtistsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ArtistsCommand::List {
            limit,
            start_index,
            search_term,
            sort_by,
            sort_order,
            parent_id,
            fields,
            include_item_types,
            exclude_item_types,
            recursive: _,
            genres,
            genre_ids,
            filters,
            name_starts_with,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_artists(
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_total_record_count
                    None, // enable_user_data
                    exclude_item_types.as_ref(),
                    fields.as_ref(),
                    filters.as_ref(),
                    genre_ids.as_ref(),
                    genres.as_ref(),
                    None, // image_type_limit
                    include_item_types.as_ref(),
                    None, // is_favorite
                    *limit,
                    None, // media_types
                    None, // min_community_rating
                    None, // name_less_than
                    name_starts_with.as_deref(),
                    None, // name_starts_with_or_greater
                    None, // official_ratings
                    parent_id.as_ref(),
                    None, // person
                    None, // person_ids
                    None, // person_types
                    search_term.as_deref(),
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    None, // studio_ids
                    None, // studios
                    None, // tags
                    Some(effective_uid),
                    None, // years
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ArtistsCommand::Get { name, user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_artist_by_name(name, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        ArtistsCommand::AlbumArtists {
            limit,
            start_index,
            search_term,
            sort_by,
            sort_order,
            parent_id,
            fields,
            include_item_types,
            exclude_item_types,
            recursive: _,
            genres,
            genre_ids,
            filters,
            name_starts_with,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_album_artists(
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_total_record_count
                    None, // enable_user_data
                    exclude_item_types.as_ref(),
                    fields.as_ref(),
                    filters.as_ref(),
                    genre_ids.as_ref(),
                    genres.as_ref(),
                    None, // image_type_limit
                    include_item_types.as_ref(),
                    None, // is_favorite
                    *limit,
                    None, // media_types
                    None, // min_community_rating
                    None, // name_less_than
                    name_starts_with.as_deref(),
                    None, // name_starts_with_or_greater
                    None, // official_ratings
                    parent_id.as_ref(),
                    None, // person
                    None, // person_ids
                    None, // person_types
                    search_term.as_deref(),
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    None, // studio_ids
                    None, // studios
                    None, // tags
                    Some(effective_uid),
                    None, // years
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ArtistsCommand::InstantMix {
            item_id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_instant_mix_from_artists(
                    item_id,
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_user_data
                    fields.as_ref(),
                    None, // image_type_limit
                    *limit,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ArtistsCommand::SimilarAlbums {
            item_id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_similar_albums(
                    item_id,
                    None, // exclude_artist_ids
                    fields.as_ref(),
                    *limit,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ArtistsCommand::SimilarArtists {
            item_id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_similar_artists(
                    item_id,
                    None, // exclude_artist_ids
                    fields.as_ref(),
                    *limit,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
