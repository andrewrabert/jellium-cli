use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum StudiosCommand {
    /// List all studios
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
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get a studio by name
    Get {
        /// The studio name
        name: String,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &StudiosCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        StudiosCommand::List {
            limit,
            start_index,
            search_term,
            parent_id,
            include_item_types,
            fields,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_studios(&jellyfin_api::query::GetStudios {
                    fields: fields.as_ref(),
                    include_item_types: include_item_types.as_ref(),
                    limit: *limit,
                    parent_id: parent_id.as_ref(),
                    search_term: search_term.as_deref(),
                    start_index: *start_index,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        StudiosCommand::Get { name, user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_studio(name, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
