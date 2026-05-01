use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum VideosCommand {
    /// Merge multiple video versions into one
    MergeVersions {
        /// Item IDs to merge (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<Uuid>,
    },
    /// Delete alternate video sources
    DeleteAlternateSources {
        /// The item ID
        item_id: Uuid,
    },
    /// Get additional parts for a video
    AdditionalParts {
        /// The item ID
        item_id: Uuid,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Delete an external subtitle file
    DeleteSubtitle {
        /// The item ID
        item_id: Uuid,
        /// The subtitle stream index
        index: i32,
    },
    /// Stop an active encoding process
    StopEncoding {
        /// The device ID
        #[arg(long)]
        device_id: String,
        /// The play session ID
        #[arg(long)]
        play_session_id: String,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &VideosCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        VideosCommand::MergeVersions { ids } => {
            client.merge_versions(ids).await?;
        }
        VideosCommand::DeleteAlternateSources { item_id } => {
            client.delete_alternate_sources(item_id).await?;
        }
        VideosCommand::AdditionalParts {
            item_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_additional_part(item_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        VideosCommand::DeleteSubtitle { item_id, index } => {
            client.delete_subtitle(item_id, *index).await?;
        }
        VideosCommand::StopEncoding {
            device_id,
            play_session_id,
        } => {
            client
                .stop_encoding_process(device_id, play_session_id)
                .await?;
        }
    }
    Ok(())
}
