use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum GenresCommand {
    /// List all genres
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
        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,
        /// Include item types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<jellyfin_api::types::BaseItemKind>>,
        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<jellyfin_api::types::ItemFields>>,
        /// Sort by (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<jellyfin_api::types::ItemSortBy>>,
        /// Sort order (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<jellyfin_api::types::SortOrder>>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get a genre by name
    Get {
        /// The genre name
        name: String,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &GenresCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        GenresCommand::List {
            limit,
            start_index,
            search_term,
            parent_id,
            include_item_types,
            fields,
            sort_by,
            sort_order,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_genres(
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_total_record_count
                    None, // exclude_item_types
                    fields.as_ref(),
                    None, // image_type_limit
                    include_item_types.as_ref(),
                    None, // is_favorite
                    *limit,
                    None, // name_less_than
                    None, // name_starts_with
                    None, // name_starts_with_or_greater
                    parent_id.as_ref(),
                    search_term.as_deref(),
                    sort_by.as_ref(),
                    sort_order.as_ref(),
                    *start_index,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        GenresCommand::Get {
            name,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_genre(name, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
