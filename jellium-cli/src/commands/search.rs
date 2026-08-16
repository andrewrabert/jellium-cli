use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum SearchCommand {
    /// Search for items (search hints)
    Hints {
        /// The search term
        search_term: String,

        #[arg(long)]
        limit: Option<i32>,

        #[arg(long)]
        start_index: Option<i32>,

        #[arg(long)]
        parent_id: Option<Uuid>,

        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<jellyfin_api::types::BaseItemKind>>,

        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<jellyfin_api::types::BaseItemKind>>,

        #[arg(long, value_delimiter = ',')]
        media_types: Option<Vec<jellyfin_api::types::MediaType>>,

        #[arg(long)]
        include_artists: Option<bool>,

        #[arg(long)]
        include_genres: Option<bool>,

        #[arg(long)]
        include_media: Option<bool>,

        #[arg(long)]
        include_people: Option<bool>,

        #[arg(long)]
        include_studios: Option<bool>,

        #[arg(long)]
        is_kids: Option<bool>,

        #[arg(long)]
        is_movie: Option<bool>,

        #[arg(long)]
        is_news: Option<bool>,

        #[arg(long)]
        is_series: Option<bool>,

        #[arg(long)]
        is_sports: Option<bool>,

        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Remote search for books (reads JSON query from stdin)
    RemoteBook,
    /// Remote search for box sets (reads JSON query from stdin)
    RemoteBoxSet,
    /// Remote search for movies (reads JSON query from stdin)
    RemoteMovie,
    /// Remote search for music albums (reads JSON query from stdin)
    RemoteMusicAlbum,
    /// Remote search for music artists (reads JSON query from stdin)
    RemoteMusicArtist,
    /// Remote search for music videos (reads JSON query from stdin)
    RemoteMusicVideo,
    /// Remote search for persons (reads JSON query from stdin)
    RemotePerson,
    /// Remote search for series (reads JSON query from stdin)
    RemoteSeries,
    /// Remote search for trailers (reads JSON query from stdin)
    RemoteTrailer,
    /// Apply remote search result to an item (reads JSON from stdin)
    ApplySearchResult {
        /// The item ID to apply results to
        item_id: Uuid,
        /// Whether to replace all images
        #[arg(long)]
        replace_all_images: Option<bool>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &SearchCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        SearchCommand::Hints {
            search_term,
            limit,
            start_index,
            parent_id,
            include_item_types,
            exclude_item_types,
            media_types,
            include_artists,
            include_genres,
            include_media,
            include_people,
            include_studios,
            is_kids,
            is_movie,
            is_news,
            is_series,
            is_sports,
            user_id: uid_override,
        } => {
            let uid = uid_override.as_ref().unwrap_or(user_id);
            let result = client
                .get_search_hints(
                    search_term,
                    &jellyfin_api::query::GetSearchHints {
                        exclude_item_types: exclude_item_types.as_ref(),
                        include_artists: *include_artists,
                        include_genres: *include_genres,
                        include_item_types: include_item_types.as_ref(),
                        include_media: *include_media,
                        include_people: *include_people,
                        include_studios: *include_studios,
                        is_kids: *is_kids,
                        is_movie: *is_movie,
                        is_news: *is_news,
                        is_series: *is_series,
                        is_sports: *is_sports,
                        limit: *limit,
                        media_types: media_types.as_ref(),
                        parent_id: parent_id.as_ref(),
                        start_index: *start_index,
                        user_id: Some(uid),
                    },
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteBook => {
            let body: jellyfin_api::types::BookInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_book_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteBoxSet => {
            let body: jellyfin_api::types::BoxSetInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_box_set_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteMovie => {
            let body: jellyfin_api::types::MovieInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_movie_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteMusicAlbum => {
            let body: jellyfin_api::types::AlbumInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_music_album_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteMusicArtist => {
            let body: jellyfin_api::types::ArtistInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_music_artist_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteMusicVideo => {
            let body: jellyfin_api::types::MusicVideoInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_music_video_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemotePerson => {
            let body: jellyfin_api::types::PersonLookupInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_person_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteSeries => {
            let body: jellyfin_api::types::SeriesInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_series_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::RemoteTrailer => {
            let body: jellyfin_api::types::TrailerInfoRemoteSearchQuery =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.get_trailer_remote_search_results(&body).await?;
            crate::output::print_json(&result)?;
        }
        SearchCommand::ApplySearchResult {
            item_id,
            replace_all_images,
        } => {
            let body: jellyfin_api::types::RemoteSearchResult =
                serde_json::from_reader(std::io::stdin())?;
            client
                .apply_search_criteria(item_id, *replace_all_images, &body)
                .await?;
        }
    }
    Ok(())
}
