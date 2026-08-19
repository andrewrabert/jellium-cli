use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum ChannelsCommand {
    /// List available channels
    List {
        /// Filter by favorite channels
        #[arg(long)]
        is_favorite: Option<bool>,
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,
        /// Filter by channels that support getting latest items
        #[arg(long)]
        supports_latest_items: Option<bool>,
        /// Filter by channels that support media deletion
        #[arg(long)]
        supports_media_deletion: Option<bool>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get channel features
    Features {
        /// The channel ID
        channel_id: Uuid,
    },
    /// Get all channel features
    AllFeatures,
    /// Get channel items
    Items {
        /// The channel ID
        channel_id: Uuid,
        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<jellyfin_api::types::ItemFields>>,
        /// Filters to apply (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        filters: Option<Vec<jellyfin_api::types::ItemFilter>>,
        /// Folder ID
        #[arg(long)]
        folder_id: Option<Uuid>,
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
        /// Sort by (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<jellyfin_api::types::ItemSortBy>>,
        /// Sort order (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<jellyfin_api::types::SortOrder>>,
        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get latest channel items
    Latest {
        /// Channel IDs (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        channel_ids: Option<Vec<Uuid>>,
        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<jellyfin_api::types::ItemFields>>,
        /// Filters to apply (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        filters: Option<Vec<jellyfin_api::types::ItemFilter>>,
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &ChannelsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ChannelsCommand::List {
            is_favorite,
            limit,
            start_index,
            supports_latest_items,
            supports_media_deletion,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_channels(
                    *is_favorite,
                    *limit,
                    *start_index,
                    *supports_latest_items,
                    *supports_media_deletion,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ChannelsCommand::Features { channel_id } => {
            let result = client.get_channel_features(channel_id).await?;
            crate::output::print_json(&result)?;
        }
        ChannelsCommand::AllFeatures => {
            let result = client.get_all_channel_features().await?;
            crate::output::print_json(&result)?;
        }
        ChannelsCommand::Items {
            channel_id,
            fields,
            filters,
            folder_id,
            limit,
            sort_by,
            sort_order,
            start_index,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_channel_items(
                    channel_id,
                    &jellyfin_api::query::GetChannelItems {
                        fields: fields.as_ref(),
                        filters: filters.as_ref(),
                        folder_id: folder_id.as_ref(),
                        limit: *limit,
                        sort_by: sort_by.as_ref(),
                        sort_order: sort_order.as_ref(),
                        start_index: *start_index,
                        user_id: Some(effective_uid),
                    },
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ChannelsCommand::Latest {
            channel_ids,
            fields,
            filters,
            limit,
            start_index,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_latest_channel_items(
                    channel_ids.as_ref(),
                    fields.as_ref(),
                    filters.as_ref(),
                    *limit,
                    *start_index,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
    }
    Ok(())
}
