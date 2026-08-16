use clap::Subcommand;
use jellyfin_api::types::{CreatePlaylistDto, ItemFields, MediaType, UpdatePlaylistDto};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum PlaylistsCommand {
    /// Create a new playlist
    Create {
        /// Playlist name
        #[arg(long)]
        name: Option<String>,

        /// Item IDs to add (comma separated)
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<Uuid>>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Media type
        #[arg(long)]
        media_type: Option<MediaType>,
    },

    /// Get a playlist
    Get {
        /// The playlist ID
        playlist_id: Uuid,
    },

    /// Update a playlist
    Update {
        /// The playlist ID
        playlist_id: Uuid,

        /// New playlist name
        #[arg(long)]
        name: Option<String>,

        /// Item IDs for the playlist (comma separated)
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<Uuid>>,

        /// Whether the playlist is public
        #[arg(long)]
        is_public: Option<bool>,
    },

    /// Get playlist items
    Items {
        /// The playlist ID
        playlist_id: Uuid,

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
    },

    /// Add items to a playlist
    AddItems {
        /// The playlist ID
        playlist_id: Uuid,

        /// Item IDs to add (comma separated)
        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<Uuid>>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Remove items from a playlist
    RemoveItems {
        /// The playlist ID
        playlist_id: Uuid,

        /// Entry IDs to remove (comma separated)
        #[arg(long, value_delimiter = ',')]
        entry_ids: Option<Vec<String>>,
    },

    /// Move an item within a playlist
    MoveItem {
        /// The playlist ID
        playlist_id: Uuid,

        /// The item ID to move
        item_id: String,

        /// The new index position
        new_index: i32,
    },

    /// Get playlist users
    Users {
        /// The playlist ID
        playlist_id: Uuid,
    },

    /// Get a specific playlist user
    GetUser {
        /// The playlist ID
        playlist_id: Uuid,

        /// The user ID
        user_id: Uuid,
    },

    /// Remove a user from a playlist
    RemoveUser {
        /// The playlist ID
        playlist_id: Uuid,

        /// The user ID
        user_id: Uuid,
    },

    /// Create an instant mix from a playlist
    InstantMix {
        /// The playlist ID
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
    command: &PlaylistsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PlaylistsCommand::Create {
            name,
            ids,
            user_id: uid,
            media_type,
        } => {
            let effective_uid = uid.unwrap_or(*user_id);
            let body = CreatePlaylistDto {
                name: name.clone(),
                ids: ids.clone().unwrap_or_default(),
                user_id: Some(effective_uid),
                media_type: *media_type,
                ..Default::default()
            };
            let result = client
                .create_playlist(
                    ids.as_ref(),
                    *media_type,
                    name.as_deref(),
                    Some(&effective_uid),
                    &body,
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        PlaylistsCommand::Get { playlist_id } => {
            let result = client.get_playlist(playlist_id).await?;
            crate::output::print_json(&result)?;
        }
        PlaylistsCommand::Update {
            playlist_id,
            name,
            ids,
            is_public,
        } => {
            let body = UpdatePlaylistDto {
                name: name.clone(),
                ids: ids.clone(),
                is_public: *is_public,
                ..Default::default()
            };
            client.update_playlist(playlist_id, &body).await?;
        }
        PlaylistsCommand::Items {
            playlist_id,
            user_id: uid,
            limit,
            start_index,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_playlist_items(
                    playlist_id,
                    &jellyfin_api::query::GetPlaylistItems {
                        fields: fields.as_ref(),
                        limit: *limit,
                        start_index: *start_index,
                        user_id: Some(effective_uid),
                        ..Default::default()
                    },
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        PlaylistsCommand::AddItems {
            playlist_id,
            ids,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            client
                .add_item_to_playlist(playlist_id, ids.as_ref(), Some(effective_uid))
                .await?;
        }
        PlaylistsCommand::RemoveItems {
            playlist_id,
            entry_ids,
        } => {
            client
                .remove_item_from_playlist(&playlist_id.to_string(), entry_ids.as_ref())
                .await?;
        }
        PlaylistsCommand::MoveItem {
            playlist_id,
            item_id,
            new_index,
        } => {
            client
                .move_item(&playlist_id.to_string(), item_id, *new_index)
                .await?;
        }
        PlaylistsCommand::Users { playlist_id } => {
            let result = client.get_playlist_users(playlist_id).await?;
            crate::output::print_json(&result)?;
        }
        PlaylistsCommand::GetUser {
            playlist_id,
            user_id: uid,
        } => {
            let result = client.get_playlist_user(playlist_id, uid).await?;
            crate::output::print_json(&result)?;
        }
        PlaylistsCommand::RemoveUser {
            playlist_id,
            user_id: uid,
        } => {
            client.remove_user_from_playlist(playlist_id, uid).await?;
        }
        PlaylistsCommand::InstantMix {
            id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_instant_mix_from_playlist(
                    id,
                    &jellyfin_api::query::GetInstantMixFromPlaylist {
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
