use clap::Subcommand;
use jellyfin_api::types::{
    BaseItemKind, ImageType, ItemFields, ItemFilter, ItemSortBy, LocationType, MediaSegmentType,
    MediaType, MetadataRefreshMode, SeriesStatus, SortOrder, VideoType,
};
use uuid::Uuid;

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum ItemsCommand {
    /// List items based on query parameters
    List {
        /// Return items that are siblings of a supplied item
        #[arg(long)]
        adjacent_to: Option<Uuid>,

        /// Filter by album artist IDs
        #[arg(long, value_delimiter = ',')]
        album_artist_ids: Option<Vec<Uuid>>,

        /// Filter by album IDs
        #[arg(long, value_delimiter = ',')]
        album_ids: Option<Vec<Uuid>>,

        /// Filter by album names
        #[arg(long, value_delimiter = ',')]
        albums: Option<Vec<String>>,

        /// Filter by artist IDs
        #[arg(long, value_delimiter = ',')]
        artist_ids: Option<Vec<Uuid>>,

        /// Filter by artist names
        #[arg(long, value_delimiter = ',')]
        artists: Option<Vec<String>>,

        /// Whether to hide items behind their boxsets
        #[arg(long)]
        collapse_box_set_items: Option<bool>,

        /// Filter by contributing artist IDs
        #[arg(long, value_delimiter = ',')]
        contributing_artist_ids: Option<Vec<Uuid>>,

        /// Image types to include in the output
        #[arg(long, value_delimiter = ',')]
        enable_image_types: Option<Vec<ImageType>>,

        /// Include image information in output
        #[arg(long)]
        enable_images: Option<bool>,

        /// Enable the total record count
        #[arg(long)]
        enable_total_record_count: Option<bool>,

        /// Include user data
        #[arg(long)]
        enable_user_data: Option<bool>,

        /// Exclude artist IDs
        #[arg(long, value_delimiter = ',')]
        exclude_artist_ids: Option<Vec<Uuid>>,

        /// Exclude item IDs
        #[arg(long, value_delimiter = ',')]
        exclude_item_ids: Option<Vec<Uuid>>,

        /// Exclude item types
        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<BaseItemKind>>,

        /// Exclude location types
        #[arg(long, value_delimiter = ',')]
        exclude_location_types: Option<Vec<LocationType>>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Additional filters to apply
        #[arg(long, value_delimiter = ',')]
        filters: Option<Vec<ItemFilter>>,

        /// Filter by genre IDs
        #[arg(long, value_delimiter = ',')]
        genre_ids: Option<Vec<Uuid>>,

        /// Filter by genre names
        #[arg(long, value_delimiter = ',')]
        genres: Option<Vec<String>>,

        /// Filter by items that have an IMDb id
        #[arg(long)]
        has_imdb_id: Option<bool>,

        /// Filter by items that have official ratings
        #[arg(long)]
        has_official_rating: Option<bool>,

        /// Filter by items that have an overview
        #[arg(long)]
        has_overview: Option<bool>,

        /// Filter by items that have a parental rating
        #[arg(long)]
        has_parental_rating: Option<bool>,

        /// Filter by items with special features
        #[arg(long)]
        has_special_feature: Option<bool>,

        /// Filter by items with subtitles
        #[arg(long)]
        has_subtitles: Option<bool>,

        /// Filter by items with theme songs
        #[arg(long)]
        has_theme_song: Option<bool>,

        /// Filter by items with theme videos
        #[arg(long)]
        has_theme_video: Option<bool>,

        /// Filter by items that have a TMDb id
        #[arg(long)]
        has_tmdb_id: Option<bool>,

        /// Filter by items with trailers
        #[arg(long)]
        has_trailer: Option<bool>,

        /// Filter by items that have a TVDb id
        #[arg(long)]
        has_tvdb_id: Option<bool>,

        /// Specific item IDs to retrieve
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<Uuid>>,

        /// Max number of images to return per image type
        #[arg(long)]
        image_type_limit: Option<i32>,

        /// Filter by image types
        #[arg(long, value_delimiter = ',')]
        image_types: Option<Vec<ImageType>>,

        /// Include item types
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Filter by index number
        #[arg(long)]
        index_number: Option<i32>,

        /// Filter by items that are 3D
        #[arg(long)]
        is3_d: Option<bool>,

        /// Filter by items that are 4K
        #[arg(long)]
        is4_k: Option<bool>,

        /// Filter by favorite items
        #[arg(long)]
        is_favorite: Option<bool>,

        /// Filter by HD items
        #[arg(long)]
        is_hd: Option<bool>,

        /// Filter for kids content
        #[arg(long)]
        is_kids: Option<bool>,

        /// Filter by locked items
        #[arg(long)]
        is_locked: Option<bool>,

        /// Filter by missing episodes
        #[arg(long)]
        is_missing: Option<bool>,

        /// Filter for movies
        #[arg(long)]
        is_movie: Option<bool>,

        /// Filter for news
        #[arg(long)]
        is_news: Option<bool>,

        /// Filter by placeholder items
        #[arg(long)]
        is_place_holder: Option<bool>,

        /// Filter by played items
        #[arg(long)]
        is_played: Option<bool>,

        /// Filter for series
        #[arg(long)]
        is_series: Option<bool>,

        /// Filter for sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Filter by unaired items
        #[arg(long)]
        is_unaired: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by location types
        #[arg(long, value_delimiter = ',')]
        location_types: Option<Vec<LocationType>>,

        /// Filter by maximum height
        #[arg(long)]
        max_height: Option<i32>,

        /// Filter by maximum official rating
        #[arg(long)]
        max_official_rating: Option<String>,

        /// Maximum premiere date (ISO format)
        #[arg(long)]
        max_premiere_date: Option<chrono::DateTime<chrono::Utc>>,

        /// Filter by maximum width
        #[arg(long)]
        max_width: Option<i32>,

        /// Filter by media types
        #[arg(long, value_delimiter = ',')]
        media_types: Option<Vec<MediaType>>,

        /// Filter by minimum community rating
        #[arg(long)]
        min_community_rating: Option<f64>,

        /// Filter by minimum critic rating
        #[arg(long)]
        min_critic_rating: Option<f64>,

        /// Minimum last saved date (ISO format)
        #[arg(long)]
        min_date_last_saved: Option<chrono::DateTime<chrono::Utc>>,

        /// Minimum last saved date for the current user (ISO format)
        #[arg(long)]
        min_date_last_saved_for_user: Option<chrono::DateTime<chrono::Utc>>,

        /// Filter by minimum height
        #[arg(long)]
        min_height: Option<i32>,

        /// Filter by minimum official rating
        #[arg(long)]
        min_official_rating: Option<String>,

        /// Minimum premiere date (ISO format)
        #[arg(long)]
        min_premiere_date: Option<chrono::DateTime<chrono::Utc>>,

        /// Filter by minimum width
        #[arg(long)]
        min_width: Option<i32>,

        /// Filter by items whose name is less than a given string
        #[arg(long)]
        name_less_than: Option<String>,

        /// Filter by items whose name starts with a given string
        #[arg(long)]
        name_starts_with: Option<String>,

        /// Filter by items whose name starts with or is greater than a given string
        #[arg(long)]
        name_starts_with_or_greater: Option<String>,

        /// Filter by official ratings
        #[arg(long, value_delimiter = ',')]
        official_ratings: Option<Vec<String>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Filter by parent index number
        #[arg(long)]
        parent_index_number: Option<i32>,

        /// Filter by person name
        #[arg(long)]
        person: Option<String>,

        /// Filter by person IDs
        #[arg(long, value_delimiter = ',')]
        person_ids: Option<Vec<Uuid>>,

        /// Filter by person types
        #[arg(long, value_delimiter = ',')]
        person_types: Option<Vec<String>>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,

        /// Filter based on a search term
        #[arg(long)]
        search_term: Option<String>,

        /// Filter by series status
        #[arg(long, value_delimiter = ',')]
        series_status: Option<Vec<SeriesStatus>>,

        /// Sort orders
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order direction
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by studio IDs
        #[arg(long, value_delimiter = ',')]
        studio_ids: Option<Vec<Uuid>>,

        /// Filter by studio names
        #[arg(long, value_delimiter = ',')]
        studios: Option<Vec<String>>,

        /// Filter by tags
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Filter by video types
        #[arg(long, value_delimiter = ',')]
        video_types: Option<Vec<VideoType>>,

        /// Filter by production years
        #[arg(long, value_delimiter = ',')]
        years: Option<Vec<i32>>,
    },

    /// Get a single item by ID
    Get {
        /// The item ID
        item_id: Uuid,
    },

    /// Delete an item
    Delete {
        /// The item ID
        item_id: Uuid,
    },

    /// Delete multiple items
    DeleteMultiple {
        /// The item IDs
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<Uuid>>,
    },

    /// Get item counts
    Counts {
        /// Get counts of favorite items
        #[arg(long)]
        is_favorite: Option<bool>,
    },

    /// Get similar items
    Similar {
        /// The item ID
        item_id: Uuid,

        /// Exclude artist IDs
        #[arg(long, value_delimiter = ',')]
        exclude_artist_ids: Option<Vec<Uuid>>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
    },

    /// Get latest media
    Latest {
        /// Image types to include in the output
        #[arg(long, value_delimiter = ',')]
        enable_image_types: Option<Vec<ImageType>>,

        /// Include image information in output
        #[arg(long)]
        enable_images: Option<bool>,

        /// Include user data
        #[arg(long)]
        enable_user_data: Option<bool>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Whether to group items into a parent container
        #[arg(long)]
        group_items: Option<bool>,

        /// Max number of images to return per image type
        #[arg(long)]
        image_type_limit: Option<i32>,

        /// Include item types
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Filter by played items
        #[arg(long)]
        is_played: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,
    },

    /// Get all ancestors of an item
    Ancestors {
        /// The item ID
        item_id: Uuid,
    },

    /// Get suggestions
    Suggestions {
        /// Enable the total record count
        #[arg(long)]
        enable_total_record_count: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by media types
        #[arg(long, value_delimiter = ',')]
        media_type: Option<Vec<MediaType>>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by item types
        #[arg(long, value_delimiter = ',')]
        type_: Option<Vec<BaseItemKind>>,
    },

    /// Refresh metadata for an item
    Refresh {
        /// The item ID
        item_id: Uuid,

        /// Image refresh mode
        #[arg(long)]
        image_refresh_mode: Option<MetadataRefreshMode>,

        /// Metadata refresh mode
        #[arg(long)]
        metadata_refresh_mode: Option<MetadataRefreshMode>,

        /// Regenerate trickplay images
        #[arg(long)]
        regenerate_trickplay: Option<bool>,

        /// Replace all images
        #[arg(long)]
        replace_all_images: Option<bool>,

        /// Replace all metadata
        #[arg(long)]
        replace_all_metadata: Option<bool>,
    },

    /// Update an item's content type
    ContentType {
        /// The item ID
        item_id: Uuid,

        /// The content type
        #[arg(long)]
        content_type: Option<String>,
    },

    /// Get metadata editor info for an item
    MetadataEditor {
        /// The item ID
        item_id: Uuid,
    },

    /// Get external ID info for an item
    ExternalIds {
        /// The item ID
        item_id: Uuid,
    },

    /// Get media segments for an item
    Segments {
        /// The item ID
        item_id: Uuid,

        /// Filter by segment types
        #[arg(long, value_delimiter = ',')]
        include_segment_types: Option<Vec<MediaSegmentType>>,
    },

    /// Get intros for an item
    Intros {
        /// The item ID
        item_id: Uuid,
    },

    /// Get local trailers for an item
    LocalTrailers {
        /// The item ID
        item_id: Uuid,
    },

    /// Get special features for an item
    SpecialFeatures {
        /// The item ID
        item_id: Uuid,
    },

    /// Get the root folder
    RootFolder,

    /// Get theme media (songs and videos) for an item
    ThemeMedia {
        /// The item ID
        item_id: Uuid,

        /// Inherit from parent
        #[arg(long)]
        inherit_from_parent: Option<bool>,

        /// Sort orders
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order direction
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,
    },

    /// Get theme songs for an item
    ThemeSongs {
        /// The item ID
        item_id: Uuid,

        /// Inherit from parent
        #[arg(long)]
        inherit_from_parent: Option<bool>,

        /// Sort orders
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order direction
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,
    },

    /// Get theme videos for an item
    ThemeVideos {
        /// The item ID
        item_id: Uuid,

        /// Inherit from parent
        #[arg(long)]
        inherit_from_parent: Option<bool>,

        /// Sort orders
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order direction
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,
    },

    /// Get critic reviews for an item
    CriticReviews {
        /// The item ID
        item_id: String,
    },

    /// Get query filters
    Filters {
        /// Include item types
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Is item airing
        #[arg(long)]
        is_airing: Option<bool>,

        /// Is item kids
        #[arg(long)]
        is_kids: Option<bool>,

        /// Is item movie
        #[arg(long)]
        is_movie: Option<bool>,

        /// Is item news
        #[arg(long)]
        is_news: Option<bool>,

        /// Is item series
        #[arg(long)]
        is_series: Option<bool>,

        /// Is item sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,
    },

    /// Get legacy query filters
    FiltersLegacy {
        /// Include item types
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Filter by media types
        #[arg(long, value_delimiter = ',')]
        media_types: Option<Vec<MediaType>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,
    },

    /// Get years
    Years {
        /// Image types to include in the output
        #[arg(long, value_delimiter = ',')]
        enable_image_types: Option<Vec<ImageType>>,

        /// Include image information in output
        #[arg(long)]
        enable_images: Option<bool>,

        /// Include user data
        #[arg(long)]
        enable_user_data: Option<bool>,

        /// Exclude item types
        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<BaseItemKind>>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Max number of images to return per image type
        #[arg(long)]
        image_type_limit: Option<i32>,

        /// Include item types
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<BaseItemKind>>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by media types
        #[arg(long, value_delimiter = ',')]
        media_types: Option<Vec<MediaType>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Search recursively
        #[arg(long)]
        recursive: Option<bool>,

        /// Sort orders
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order direction
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,
    },

    /// Get a specific year
    Year {
        /// The year
        year: i32,
    },

    /// Get remote images for an item
    RemoteImages {
        /// The item ID
        item_id: Uuid,

        /// Include all languages
        #[arg(long)]
        include_all_languages: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Image provider name
        #[arg(long)]
        provider_name: Option<String>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Image type
        #[arg(long)]
        type_: Option<ImageType>,
    },

    /// Download a remote image for an item
    DownloadRemoteImage {
        /// The item ID
        item_id: Uuid,

        /// The image URL
        #[arg(long)]
        image_url: Option<String>,

        /// The image type
        #[arg(long)]
        type_: ImageType,
    },

    /// Get remote image providers for an item
    RemoteImageProviders {
        /// The item ID
        item_id: Uuid,
    },

    /// Search for remote subtitles
    SearchSubtitles {
        /// The item ID
        item_id: Uuid,

        /// The subtitle language
        language: String,

        /// Only show perfect matches
        #[arg(long)]
        is_perfect_match: Option<bool>,
    },

    /// Download remote subtitles
    DownloadSubtitles {
        /// The item ID
        item_id: Uuid,

        /// The subtitle ID
        subtitle_id: String,
    },

    /// Get playback info for an item
    PlaybackInfo {
        /// The item ID
        item_id: Uuid,
    },

    /// Get instant mix from an item
    InstantMix {
        /// The item ID
        item_id: Uuid,

        /// Image types to include in the output
        #[arg(long, value_delimiter = ',')]
        enable_image_types: Option<Vec<ImageType>>,

        /// Include image information in output
        #[arg(long)]
        enable_images: Option<bool>,

        /// Include user data
        #[arg(long)]
        enable_user_data: Option<bool>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Max number of images to return per image type
        #[arg(long)]
        image_type_limit: Option<i32>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &ItemsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ItemsCommand::List {
            adjacent_to,
            album_artist_ids,
            album_ids,
            albums,
            artist_ids,
            artists,
            collapse_box_set_items,
            contributing_artist_ids,
            enable_image_types,
            enable_images,
            enable_total_record_count,
            enable_user_data,
            exclude_artist_ids,
            exclude_item_ids,
            exclude_item_types,
            exclude_location_types,
            fields,
            filters,
            genre_ids,
            genres,
            has_imdb_id,
            has_official_rating,
            has_overview,
            has_parental_rating,
            has_special_feature,
            has_subtitles,
            has_theme_song,
            has_theme_video,
            has_tmdb_id,
            has_trailer,
            has_tvdb_id,
            ids,
            image_type_limit,
            image_types,
            include_item_types,
            index_number,
            is3_d,
            is4_k,
            is_favorite,
            is_hd,
            is_kids,
            is_locked,
            is_missing,
            is_movie,
            is_news,
            is_place_holder,
            is_played,
            is_series,
            is_sports,
            is_unaired,
            limit,
            location_types,
            max_height,
            max_official_rating,
            max_premiere_date,
            max_width,
            media_types,
            min_community_rating,
            min_critic_rating,
            min_date_last_saved,
            min_date_last_saved_for_user,
            min_height,
            min_official_rating,
            min_premiere_date,
            min_width,
            name_less_than,
            name_starts_with,
            name_starts_with_or_greater,
            official_ratings,
            parent_id,
            parent_index_number,
            person,
            person_ids,
            person_types,
            recursive,
            search_term,
            series_status,
            sort_by,
            sort_order,
            start_index,
            studio_ids,
            studios,
            tags,
            video_types,
            years,
        } => {
            let result = client
                .get_items(
                    adjacent_to.as_ref(),
                    album_artist_ids.as_ref(),
                    album_ids.as_ref(),
                    albums.as_ref(),
                    artist_ids.as_ref(),
                    artists.as_ref(),
                    *collapse_box_set_items,
                    contributing_artist_ids.as_ref(),
                    enable_image_types.as_ref(),
                    *enable_images,
                    *enable_total_record_count,
                    *enable_user_data,
                    exclude_artist_ids.as_ref(),
                    exclude_item_ids.as_ref(),
                    exclude_item_types.as_ref(),
                    exclude_location_types.as_ref(),
                    fields.as_ref(),
                    filters.as_ref(),
                    genre_ids.as_ref(),
                    genres.as_ref(),
                    *has_imdb_id,
                    *has_official_rating,
                    *has_overview,
                    *has_parental_rating,
                    *has_special_feature,
                    *has_subtitles,
                    *has_theme_song,
                    *has_theme_video,
                    *has_tmdb_id,
                    *has_trailer,
                    *has_tvdb_id,
                    ids.as_ref(),
                    *image_type_limit,
                    image_types.as_ref(),
                    include_item_types.as_ref(),
                    *index_number,
                    *is3_d,
                    *is4_k,
                    *is_favorite,
                    *is_hd,
                    *is_kids,
                    *is_locked,
                    *is_missing,
                    *is_movie,
                    *is_news,
                    *is_place_holder,
                    *is_played,
                    *is_series,
                    *is_sports,
                    *is_unaired,
                    *limit,
                    location_types.as_ref(),
                    *max_height,
                    max_official_rating.as_deref(),
                    max_premiere_date.as_ref(),
                    *max_width,
                    media_types.as_ref(),
                    *min_community_rating,
                    *min_critic_rating,
                    min_date_last_saved.as_ref(),
                    min_date_last_saved_for_user.as_ref(),
                    *min_height,
                    min_official_rating.as_deref(),
                    min_premiere_date.as_ref(),
                    *min_width,
                    name_less_than.as_deref(),
                    name_starts_with.as_deref(),
                    name_starts_with_or_greater.as_deref(),
                    official_ratings.as_ref(),
                    parent_id.as_ref(),
                    *parent_index_number,
                    person.as_deref(),
                    person_ids.as_ref(),
                    person_types.as_ref(),
                    *recursive,
                    search_term.as_deref(),
                    series_status.as_ref(),
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    studio_ids.as_ref(),
                    studios.as_ref(),
                    tags.as_ref(),
                    Some(user_id),
                    video_types.as_ref(),
                    years.as_ref(),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ItemsCommand::Get { item_id } => {
            let result = client.get_item(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Delete { item_id } => {
            client.delete_item(item_id).await?;
        }
        ItemsCommand::DeleteMultiple { ids } => {
            client.delete_items(ids.as_ref()).await?;
        }
        ItemsCommand::Counts { is_favorite } => {
            let result = client.get_item_counts(*is_favorite, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Similar {
            item_id,
            exclude_artist_ids,
            fields,
            limit,
        } => {
            let result = client
                .get_similar_items(
                    item_id,
                    exclude_artist_ids.as_ref(),
                    fields.as_ref(),
                    *limit,
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Latest {
            enable_image_types,
            enable_images,
            enable_user_data,
            fields,
            group_items,
            image_type_limit,
            include_item_types,
            is_played,
            limit,
            parent_id,
        } => {
            let result = client
                .get_latest_media(
                    enable_image_types.as_ref(),
                    *enable_images,
                    *enable_user_data,
                    fields.as_ref(),
                    *group_items,
                    *image_type_limit,
                    include_item_types.as_ref(),
                    *is_played,
                    *limit,
                    parent_id.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Ancestors { item_id } => {
            let result = client.get_ancestors(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Suggestions {
            enable_total_record_count,
            limit,
            media_type,
            start_index,
            type_,
        } => {
            let result = client
                .get_suggestions(
                    *enable_total_record_count,
                    *limit,
                    media_type.as_ref(),
                    *start_index,
                    type_.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ItemsCommand::Refresh {
            item_id,
            image_refresh_mode,
            metadata_refresh_mode,
            regenerate_trickplay,
            replace_all_images,
            replace_all_metadata,
        } => {
            client
                .refresh_item(
                    item_id,
                    *image_refresh_mode,
                    *metadata_refresh_mode,
                    *regenerate_trickplay,
                    *replace_all_images,
                    *replace_all_metadata,
                )
                .await?;
        }
        ItemsCommand::ContentType {
            item_id,
            content_type,
        } => {
            client
                .update_item_content_type(item_id, content_type.as_deref())
                .await?;
        }
        ItemsCommand::MetadataEditor { item_id } => {
            let result = client.get_metadata_editor_info(item_id).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::ExternalIds { item_id } => {
            let result = client.get_external_id_infos(item_id).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Segments {
            item_id,
            include_segment_types,
        } => {
            let result = client
                .get_item_segments(item_id, include_segment_types.as_ref())
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Intros { item_id } => {
            let result = client.get_intros(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::LocalTrailers { item_id } => {
            let result = client.get_local_trailers(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::SpecialFeatures { item_id } => {
            let result = client.get_special_features(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::RootFolder => {
            let result = client.get_root_folder(Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::ThemeMedia {
            item_id,
            inherit_from_parent,
            sort_by,
            sort_order,
        } => {
            let result = client
                .get_theme_media(
                    item_id,
                    *inherit_from_parent,
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::ThemeSongs {
            item_id,
            inherit_from_parent,
            sort_by,
            sort_order,
        } => {
            let result = client
                .get_theme_songs(
                    item_id,
                    *inherit_from_parent,
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::ThemeVideos {
            item_id,
            inherit_from_parent,
            sort_by,
            sort_order,
        } => {
            let result = client
                .get_theme_videos(
                    item_id,
                    *inherit_from_parent,
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::CriticReviews { item_id } => {
            let result = client.get_critic_reviews(item_id).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Filters {
            include_item_types,
            is_airing,
            is_kids,
            is_movie,
            is_news,
            is_series,
            is_sports,
            parent_id,
            recursive,
        } => {
            let result = client
                .get_query_filters(
                    include_item_types.as_ref(),
                    *is_airing,
                    *is_kids,
                    *is_movie,
                    *is_news,
                    *is_series,
                    *is_sports,
                    parent_id.as_ref(),
                    *recursive,
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::FiltersLegacy {
            include_item_types,
            media_types,
            parent_id,
        } => {
            let result = client
                .get_query_filters_legacy(
                    include_item_types.as_ref(),
                    media_types.as_ref(),
                    parent_id.as_ref(),
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::Years {
            enable_image_types,
            enable_images,
            enable_user_data,
            exclude_item_types,
            fields,
            image_type_limit,
            include_item_types,
            limit,
            media_types,
            parent_id,
            recursive,
            sort_by,
            sort_order,
            start_index,
        } => {
            let result = client
                .get_years(
                    enable_image_types.as_ref(),
                    *enable_images,
                    *enable_user_data,
                    exclude_item_types.as_ref(),
                    fields.as_ref(),
                    *image_type_limit,
                    include_item_types.as_ref(),
                    *limit,
                    media_types.as_ref(),
                    parent_id.as_ref(),
                    *recursive,
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    Some(user_id),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ItemsCommand::Year { year } => {
            let result = client.get_year(*year, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::RemoteImages {
            item_id,
            include_all_languages,
            limit,
            provider_name,
            start_index,
            type_,
        } => {
            let result = client
                .get_remote_images(
                    item_id,
                    *include_all_languages,
                    *limit,
                    provider_name.as_deref(),
                    *start_index,
                    *type_,
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::DownloadRemoteImage {
            item_id,
            image_url,
            type_,
        } => {
            client
                .download_remote_image(item_id, image_url.as_deref(), *type_)
                .await?;
        }
        ItemsCommand::RemoteImageProviders { item_id } => {
            let result = client.get_remote_image_providers(item_id).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::SearchSubtitles {
            item_id,
            language,
            is_perfect_match,
        } => {
            let result = client
                .search_remote_subtitles(item_id, language, *is_perfect_match)
                .await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::DownloadSubtitles {
            item_id,
            subtitle_id,
        } => {
            client
                .download_remote_subtitles(item_id, subtitle_id)
                .await?;
        }
        ItemsCommand::PlaybackInfo { item_id } => {
            let result = client.get_playback_info(item_id, Some(user_id)).await?;
            crate::output::print_json(&result)?;
        }
        ItemsCommand::InstantMix {
            item_id,
            enable_image_types,
            enable_images,
            enable_user_data,
            fields,
            image_type_limit,
            limit,
        } => {
            let result = client
                .get_instant_mix_from_item(
                    item_id,
                    enable_image_types.as_ref(),
                    *enable_images,
                    *enable_user_data,
                    fields.as_ref(),
                    *image_type_limit,
                    *limit,
                    Some(user_id),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
