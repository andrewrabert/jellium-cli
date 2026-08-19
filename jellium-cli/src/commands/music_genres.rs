use clap::Subcommand;
use jellyfin_api::types::{ItemFields, ItemSortBy, SortOrder};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum MusicGenresCommand {
    /// List all music genres
    List {
        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Search term
        #[arg(long)]
        search_term: Option<String>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Sort by fields
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,
    },

    /// Get a music genre by name
    Get {
        /// Genre name
        name: String,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Create an instant mix from a music genre
    InstantMix {
        /// The music genre ID
        id: Uuid,

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
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &MusicGenresCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        MusicGenresCommand::List {
            user_id: uid,
            limit,
            start_index,
            fields,
            search_term,
            parent_id,
            sort_by,
            sort_order,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_music_genres(&jellyfin_api::query::GetMusicGenres {
                    fields: fields.as_ref(),
                    limit: *limit,
                    parent_id: parent_id.as_ref(),
                    search_term: search_term.as_deref(),
                    sort_by: sort_by.as_ref(),
                    sort_order: sort_order.as_ref(),
                    start_index: *start_index,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        MusicGenresCommand::Get { name, user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_music_genre(name, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        MusicGenresCommand::InstantMix {
            id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_instant_mix_from_music_genre_by_id(
                    id,
                    &jellyfin_api::query::GetInstantMixFromMusicGenreById {
                        fields: fields.as_ref(),
                        limit: *limit,
                        user_id: Some(effective_uid),
                        ..Default::default()
                    },
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
