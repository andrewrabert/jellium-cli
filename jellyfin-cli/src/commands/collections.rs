use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum CollectionsCommand {
    /// Create a new collection
    Create {
        /// Collection name
        #[arg(long)]
        name: Option<String>,
        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,
        /// Item IDs to add (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<String>>,
        /// Whether to lock the collection
        #[arg(long)]
        is_locked: Option<bool>,
    },
    /// Add items to a collection
    AddItems {
        /// Collection ID
        collection_id: Uuid,
        /// Item IDs to add (comma-separated)
        #[arg(value_delimiter = ',')]
        ids: Vec<Uuid>,
    },
    /// Remove items from a collection
    RemoveItems {
        /// Collection ID
        collection_id: Uuid,
        /// Item IDs to remove (comma-separated)
        #[arg(value_delimiter = ',')]
        ids: Vec<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: CollectionsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        CollectionsCommand::Create {
            name,
            parent_id,
            ids,
            is_locked,
        } => {
            let result = client
                .create_collection(
                    ids.as_ref(),
                    is_locked,
                    name.as_deref(),
                    parent_id.as_ref(),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        CollectionsCommand::AddItems {
            collection_id,
            ids,
        } => {
            client.add_to_collection(&collection_id, &ids).await?;
        }
        CollectionsCommand::RemoveItems {
            collection_id,
            ids,
        } => {
            client
                .remove_from_collection(&collection_id, &ids)
                .await?;
        }
    }
    Ok(())
}
