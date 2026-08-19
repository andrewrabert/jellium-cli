use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum UserDataCommand {
    /// Get user data for an item
    Get {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Update user data for an item
    Update {
        /// The item ID
        item_id: Uuid,
        /// Whether the item is played
        #[arg(long)]
        played: Option<bool>,
        /// Whether the item is a favorite
        #[arg(long)]
        is_favorite: Option<bool>,
        /// Whether the user likes the item
        #[arg(long)]
        likes: Option<bool>,
        /// Play count
        #[arg(long)]
        play_count: Option<i32>,
        /// Playback position ticks
        #[arg(long)]
        playback_position_ticks: Option<i64>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Mark an item as played
    MarkPlayed {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Mark an item as unplayed
    MarkUnplayed {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Mark an item as a favorite
    Favorite {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Unmark an item as a favorite
    Unfavorite {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Rate an item
    Rate {
        /// The item ID
        item_id: Uuid,
        /// Whether the user likes the item
        #[arg(long)]
        likes: Option<bool>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Remove rating from an item
    Unrate {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get resume items
    Resume {
        /// Maximum number of items to return
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
        /// Include item types (comma separated)
        #[arg(long, value_delimiter = ',')]
        include_item_types: Option<Vec<jellyfin_api::types::BaseItemKind>>,
        /// Exclude item types (comma separated)
        #[arg(long, value_delimiter = ',')]
        exclude_item_types: Option<Vec<jellyfin_api::types::BaseItemKind>>,
        /// Media types (comma separated)
        #[arg(long, value_delimiter = ',')]
        media_types: Option<Vec<jellyfin_api::types::MediaType>>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get user views (libraries)
    Views {
        /// Include external content (channels, live tv)
        #[arg(long)]
        include_external_content: Option<bool>,
        /// Include hidden content
        #[arg(long)]
        include_hidden: Option<bool>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get user view grouping options
    GroupingOptions {
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &UserDataCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        UserDataCommand::Get {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_item_user_data(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Update {
            item_id,
            played,
            is_favorite,
            likes,
            play_count,
            playback_position_ticks,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let body = jellyfin_api::types::UpdateUserItemDataDto {
                played: *played,
                is_favorite: *is_favorite,
                likes: *likes,
                play_count: *play_count,
                playback_position_ticks: *playback_position_ticks,
                ..Default::default()
            };
            let result = client
                .update_item_user_data(item_id, Some(effective_uid), &body)
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::MarkPlayed {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .mark_played_item(item_id, None, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::MarkUnplayed {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .mark_unplayed_item(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Favorite {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .mark_favorite_item(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Unfavorite {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .unmark_favorite_item(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Rate {
            item_id,
            likes,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .update_user_item_rating(item_id, *likes, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Unrate {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .delete_user_item_rating(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::Resume {
            limit,
            start_index,
            search_term,
            parent_id,
            include_item_types,
            exclude_item_types,
            media_types,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_resume_items(&jellyfin_api::query::GetResumeItems {
                    exclude_item_types: exclude_item_types.as_ref(),
                    include_item_types: include_item_types.as_ref(),
                    limit: *limit,
                    media_types: media_types.as_ref(),
                    parent_id: parent_id.as_ref(),
                    search_term: search_term.as_deref(),
                    start_index: *start_index,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        UserDataCommand::Views {
            include_external_content,
            include_hidden,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_user_views(
                    *include_external_content,
                    *include_hidden,
                    None,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        UserDataCommand::GroupingOptions { user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_grouping_options(Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
