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
                .get_trailers(&jellyfin_api::query::GetTrailers {
                    fields: fields.as_ref(),
                    limit: *limit,
                    parent_id: parent_id.as_ref(),
                    recursive: *recursive,
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
    }
    Ok(())
}
