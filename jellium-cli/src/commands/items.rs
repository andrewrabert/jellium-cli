use clap::Subcommand;
use jellyfin_api::types::{
    BaseItemKind, ImageType, ItemFields, ItemFilter, ItemSortBy, LocationType, MediaSegmentType,
    MediaType, MetadataRefreshMode, SeriesStatus, SortOrder, VideoType,
};
use uuid::Uuid;

/// Every flag `items list` takes.
#[derive(clap::Args)]
pub struct ListArgs {
    /// Return items that are siblings of a supplied item
    #[arg(long)]
    pub adjacent_to: Option<Uuid>,

    /// Filter by album artist IDs
    #[arg(long, value_delimiter = ',')]
    pub album_artist_ids: Option<Vec<Uuid>>,

    /// Filter by album IDs
    #[arg(long, value_delimiter = ',')]
    pub album_ids: Option<Vec<Uuid>>,

    /// Filter by album names
    #[arg(long, value_delimiter = ',')]
    pub albums: Option<Vec<String>>,

    /// Filter by artist IDs
    #[arg(long, value_delimiter = ',')]
    pub artist_ids: Option<Vec<Uuid>>,

    /// Filter by artist names
    #[arg(long, value_delimiter = ',')]
    pub artists: Option<Vec<String>>,

    /// Whether to hide items behind their boxsets
    #[arg(long)]
    pub collapse_box_set_items: Option<bool>,

    /// Filter by contributing artist IDs
    #[arg(long, value_delimiter = ',')]
    pub contributing_artist_ids: Option<Vec<Uuid>>,

    /// Image types to include in the output
    #[arg(long, value_delimiter = ',')]
    pub enable_image_types: Option<Vec<ImageType>>,

    /// Include image information in output
    #[arg(long)]
    pub enable_images: Option<bool>,

    /// Enable the total record count
    #[arg(long)]
    pub enable_total_record_count: Option<bool>,

    /// Include user data
    #[arg(long)]
    pub enable_user_data: Option<bool>,

    /// Exclude artist IDs
    #[arg(long, value_delimiter = ',')]
    pub exclude_artist_ids: Option<Vec<Uuid>>,

    /// Exclude item IDs
    #[arg(long, value_delimiter = ',')]
    pub exclude_item_ids: Option<Vec<Uuid>>,

    /// Exclude item types
    #[arg(long, value_delimiter = ',')]
    pub exclude_item_types: Option<Vec<BaseItemKind>>,

    /// Exclude location types
    #[arg(long, value_delimiter = ',')]
    pub exclude_location_types: Option<Vec<LocationType>>,

    /// Additional fields to return
    #[arg(long, value_delimiter = ',')]
    pub fields: Option<Vec<ItemFields>>,

    /// Additional filters to apply
    #[arg(long, value_delimiter = ',')]
    pub filters: Option<Vec<ItemFilter>>,

    /// Filter by genre IDs
    #[arg(long, value_delimiter = ',')]
    pub genre_ids: Option<Vec<Uuid>>,

    /// Filter by genre names
    #[arg(long, value_delimiter = ',')]
    pub genres: Option<Vec<String>>,

    /// Filter by items that have an IMDb id
    #[arg(long)]
    pub has_imdb_id: Option<bool>,

    /// Filter by items that have official ratings
    #[arg(long)]
    pub has_official_rating: Option<bool>,

    /// Filter by items that have an overview
    #[arg(long)]
    pub has_overview: Option<bool>,

    /// Filter by items that have a parental rating
    #[arg(long)]
    pub has_parental_rating: Option<bool>,

    /// Filter by items with special features
    #[arg(long)]
    pub has_special_feature: Option<bool>,

    /// Filter by items with subtitles
    #[arg(long)]
    pub has_subtitles: Option<bool>,

    /// Filter by items with theme songs
    #[arg(long)]
    pub has_theme_song: Option<bool>,

    /// Filter by items with theme videos
    #[arg(long)]
    pub has_theme_video: Option<bool>,

    /// Filter by items that have a TMDb id
    #[arg(long)]
    pub has_tmdb_id: Option<bool>,

    /// Filter by items with trailers
    #[arg(long)]
    pub has_trailer: Option<bool>,

    /// Filter by items that have a TVDb id
    #[arg(long)]
    pub has_tvdb_id: Option<bool>,

    /// Specific item IDs to retrieve
    #[arg(long, value_delimiter = ',')]
    pub ids: Option<Vec<Uuid>>,

    /// Max number of images to return per image type
    #[arg(long)]
    pub image_type_limit: Option<i32>,

    /// Filter by image types
    #[arg(long, value_delimiter = ',')]
    pub image_types: Option<Vec<ImageType>>,

    /// Include item types
    #[arg(long, value_delimiter = ',')]
    pub include_item_types: Option<Vec<BaseItemKind>>,

    /// Filter by index number
    #[arg(long)]
    pub index_number: Option<i32>,

    /// Filter by items that are 3D
    #[arg(long)]
    pub is3_d: Option<bool>,

    /// Filter by items that are 4K
    #[arg(long)]
    pub is4_k: Option<bool>,

    /// Filter by favorite items
    #[arg(long)]
    pub is_favorite: Option<bool>,

    /// Filter by HD items
    #[arg(long)]
    pub is_hd: Option<bool>,

    /// Filter for kids content
    #[arg(long)]
    pub is_kids: Option<bool>,

    /// Filter by locked items
    #[arg(long)]
    pub is_locked: Option<bool>,

    /// Filter by missing episodes
    #[arg(long)]
    pub is_missing: Option<bool>,

    /// Filter for movies
    #[arg(long)]
    pub is_movie: Option<bool>,

    /// Filter for news
    #[arg(long)]
    pub is_news: Option<bool>,

    /// Filter by placeholder items
    #[arg(long)]
    pub is_place_holder: Option<bool>,

    /// Filter by played items
    #[arg(long)]
    pub is_played: Option<bool>,

    /// Filter for series
    #[arg(long)]
    pub is_series: Option<bool>,

    /// Filter for sports
    #[arg(long)]
    pub is_sports: Option<bool>,

    /// Filter by unaired items
    #[arg(long)]
    pub is_unaired: Option<bool>,

    /// Maximum number of records to return
    #[arg(long)]
    pub limit: Option<i32>,

    /// Filter by location types
    #[arg(long, value_delimiter = ',')]
    pub location_types: Option<Vec<LocationType>>,

    /// Filter by maximum height
    #[arg(long)]
    pub max_height: Option<i32>,

    /// Filter by maximum official rating
    #[arg(long)]
    pub max_official_rating: Option<String>,

    /// Maximum premiere date (ISO format)
    #[arg(long)]
    pub max_premiere_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter by maximum width
    #[arg(long)]
    pub max_width: Option<i32>,

    /// Filter by media types
    #[arg(long, value_delimiter = ',')]
    pub media_types: Option<Vec<MediaType>>,

    /// Filter by minimum community rating
    #[arg(long)]
    pub min_community_rating: Option<f64>,

    /// Filter by minimum critic rating
    #[arg(long)]
    pub min_critic_rating: Option<f64>,

    /// Minimum last saved date (ISO format)
    #[arg(long)]
    pub min_date_last_saved: Option<chrono::DateTime<chrono::Utc>>,

    /// Minimum last saved date for the current user (ISO format)
    #[arg(long)]
    pub min_date_last_saved_for_user: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter by minimum height
    #[arg(long)]
    pub min_height: Option<i32>,

    /// Filter by minimum official rating
    #[arg(long)]
    pub min_official_rating: Option<String>,

    /// Minimum premiere date (ISO format)
    #[arg(long)]
    pub min_premiere_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter by minimum width
    #[arg(long)]
    pub min_width: Option<i32>,

    /// Filter by items whose name is less than a given string
    #[arg(long)]
    pub name_less_than: Option<String>,

    /// Filter by items whose name starts with a given string
    #[arg(long)]
    pub name_starts_with: Option<String>,

    /// Filter by items whose name starts with or is greater than a given string
    #[arg(long)]
    pub name_starts_with_or_greater: Option<String>,

    /// Filter by official ratings
    #[arg(long, value_delimiter = ',')]
    pub official_ratings: Option<Vec<String>>,

    /// Parent folder ID
    #[arg(long)]
    pub parent_id: Option<Uuid>,

    /// Filter by parent index number
    #[arg(long)]
    pub parent_index_number: Option<i32>,

    /// Filter by person name
    #[arg(long)]
    pub person: Option<String>,

    /// Filter by person IDs
    #[arg(long, value_delimiter = ',')]
    pub person_ids: Option<Vec<Uuid>>,

    /// Filter by person types
    #[arg(long, value_delimiter = ',')]
    pub person_types: Option<Vec<String>>,

    /// Search recursively
    #[arg(long)]
    pub recursive: Option<bool>,

    /// Filter based on a search term
    #[arg(long)]
    pub search_term: Option<String>,

    /// Filter by series status
    #[arg(long, value_delimiter = ',')]
    pub series_status: Option<Vec<SeriesStatus>>,

    /// Sort orders
    #[arg(long, value_delimiter = ',')]
    pub sort_by: Option<Vec<ItemSortBy>>,

    /// Sort order direction
    #[arg(long, value_delimiter = ',')]
    pub sort_order: Option<Vec<SortOrder>>,

    /// Record index to start at
    #[arg(long)]
    pub start_index: Option<i32>,

    /// Filter by studio IDs
    #[arg(long, value_delimiter = ',')]
    pub studio_ids: Option<Vec<Uuid>>,

    /// Filter by studio names
    #[arg(long, value_delimiter = ',')]
    pub studios: Option<Vec<String>>,

    /// Filter by tags
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// Filter by video types
    #[arg(long, value_delimiter = ',')]
    pub video_types: Option<Vec<VideoType>>,

    /// Filter by production years
    #[arg(long, value_delimiter = ',')]
    pub years: Option<Vec<i32>>,
}

#[derive(Subcommand)]
pub enum ItemsCommand {
    /// List items based on query parameters
    List(Box<ListArgs>),

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
        ItemsCommand::List(args) => {
            let result = client
                .get_items(&jellyfin_api::query::GetItems {
                    adjacent_to: args.adjacent_to.as_ref(),
                    album_artist_ids: args.album_artist_ids.as_ref(),
                    album_ids: args.album_ids.as_ref(),
                    albums: args.albums.as_ref(),
                    artist_ids: args.artist_ids.as_ref(),
                    artists: args.artists.as_ref(),
                    collapse_box_set_items: args.collapse_box_set_items,
                    contributing_artist_ids: args.contributing_artist_ids.as_ref(),
                    enable_image_types: args.enable_image_types.as_ref(),
                    enable_images: args.enable_images,
                    enable_total_record_count: args.enable_total_record_count,
                    enable_user_data: args.enable_user_data,
                    exclude_artist_ids: args.exclude_artist_ids.as_ref(),
                    exclude_item_ids: args.exclude_item_ids.as_ref(),
                    exclude_item_types: args.exclude_item_types.as_ref(),
                    exclude_location_types: args.exclude_location_types.as_ref(),
                    fields: args.fields.as_ref(),
                    filters: args.filters.as_ref(),
                    genre_ids: args.genre_ids.as_ref(),
                    genres: args.genres.as_ref(),
                    has_imdb_id: args.has_imdb_id,
                    has_official_rating: args.has_official_rating,
                    has_overview: args.has_overview,
                    has_parental_rating: args.has_parental_rating,
                    has_special_feature: args.has_special_feature,
                    has_subtitles: args.has_subtitles,
                    has_theme_song: args.has_theme_song,
                    has_theme_video: args.has_theme_video,
                    has_tmdb_id: args.has_tmdb_id,
                    has_trailer: args.has_trailer,
                    has_tvdb_id: args.has_tvdb_id,
                    ids: args.ids.as_ref(),
                    image_type_limit: args.image_type_limit,
                    image_types: args.image_types.as_ref(),
                    include_item_types: args.include_item_types.as_ref(),
                    index_number: args.index_number,
                    is_3d: args.is3_d,
                    is_4k: args.is4_k,
                    is_favorite: args.is_favorite,
                    is_hd: args.is_hd,
                    is_kids: args.is_kids,
                    is_locked: args.is_locked,
                    is_missing: args.is_missing,
                    is_movie: args.is_movie,
                    is_news: args.is_news,
                    is_place_holder: args.is_place_holder,
                    is_played: args.is_played,
                    is_series: args.is_series,
                    is_sports: args.is_sports,
                    is_unaired: args.is_unaired,
                    limit: args.limit,
                    location_types: args.location_types.as_ref(),
                    max_height: args.max_height,
                    max_official_rating: args.max_official_rating.as_deref(),
                    max_premiere_date: args.max_premiere_date.as_ref(),
                    max_width: args.max_width,
                    media_types: args.media_types.as_ref(),
                    min_community_rating: args.min_community_rating,
                    min_critic_rating: args.min_critic_rating,
                    min_date_last_saved: args.min_date_last_saved.as_ref(),
                    min_date_last_saved_for_user: args.min_date_last_saved_for_user.as_ref(),
                    min_height: args.min_height,
                    min_official_rating: args.min_official_rating.as_deref(),
                    min_premiere_date: args.min_premiere_date.as_ref(),
                    min_width: args.min_width,
                    name_less_than: args.name_less_than.as_deref(),
                    name_starts_with: args.name_starts_with.as_deref(),
                    name_starts_with_or_greater: args.name_starts_with_or_greater.as_deref(),
                    official_ratings: args.official_ratings.as_ref(),
                    parent_id: args.parent_id.as_ref(),
                    parent_index_number: args.parent_index_number,
                    person: args.person.as_deref(),
                    person_ids: args.person_ids.as_ref(),
                    person_types: args.person_types.as_ref(),
                    recursive: args.recursive,
                    search_term: args.search_term.as_deref(),
                    series_status: args.series_status.as_ref(),
                    sort_by: args.sort_by.as_ref(),
                    sort_order: args.sort_order.as_ref(),
                    start_index: args.start_index,
                    studio_ids: args.studio_ids.as_ref(),
                    studios: args.studios.as_ref(),
                    tags: args.tags.as_ref(),
                    user_id: Some(user_id),
                    video_types: args.video_types.as_ref(),
                    years: args.years.as_ref(),
                })
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
                .get_latest_media(&jellyfin_api::query::GetLatestMedia {
                    enable_image_types: enable_image_types.as_ref(),
                    enable_images: *enable_images,
                    enable_user_data: *enable_user_data,
                    fields: fields.as_ref(),
                    group_items: *group_items,
                    image_type_limit: *image_type_limit,
                    include_item_types: include_item_types.as_ref(),
                    is_played: *is_played,
                    limit: *limit,
                    parent_id: parent_id.as_ref(),
                    user_id: Some(user_id),
                })
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
                .get_query_filters(&jellyfin_api::query::GetQueryFilters {
                    include_item_types: include_item_types.as_ref(),
                    is_airing: *is_airing,
                    is_kids: *is_kids,
                    is_movie: *is_movie,
                    is_news: *is_news,
                    is_series: *is_series,
                    is_sports: *is_sports,
                    parent_id: parent_id.as_ref(),
                    recursive: *recursive,
                    user_id: Some(user_id),
                })
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
                .get_years(&jellyfin_api::query::GetYears {
                    enable_image_types: enable_image_types.as_ref(),
                    enable_images: *enable_images,
                    enable_user_data: *enable_user_data,
                    exclude_item_types: exclude_item_types.as_ref(),
                    fields: fields.as_ref(),
                    image_type_limit: *image_type_limit,
                    include_item_types: include_item_types.as_ref(),
                    limit: *limit,
                    media_types: media_types.as_ref(),
                    parent_id: parent_id.as_ref(),
                    recursive: *recursive,
                    sort_by: sort_by.as_ref(),
                    sort_order: sort_order.as_ref(),
                    start_index: *start_index,
                    user_id: Some(user_id),
                })
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
                    &jellyfin_api::query::GetInstantMixFromItem {
                        enable_image_types: enable_image_types.as_ref(),
                        enable_images: *enable_images,
                        enable_user_data: *enable_user_data,
                        fields: fields.as_ref(),
                        image_type_limit: *image_type_limit,
                        limit: *limit,
                        user_id: Some(user_id),
                    },
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
