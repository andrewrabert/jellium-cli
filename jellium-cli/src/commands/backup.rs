use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum BackupCommand {
    /// List all backups
    List,
    /// Create a new backup
    Create {
        /// Include database contents
        #[arg(long)]
        database: Option<bool>,
        /// Include metadata contents
        #[arg(long)]
        metadata: Option<bool>,
        /// Include subtitle contents
        #[arg(long)]
        subtitles: Option<bool>,
        /// Include trickplay contents
        #[arg(long)]
        trickplay: Option<bool>,
    },
    /// Get backup manifest by path
    Get {
        /// Path to the backup archive
        path: String,
    },
    /// Restore from a backup
    Restore {
        /// Path to the backup archive
        #[arg(long)]
        archive_file_name: Option<String>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: BackupCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        BackupCommand::List => {
            let result = client.list_backups().await?;
            crate::output::print_json(&result)?;
        }
        BackupCommand::Create {
            database,
            metadata,
            subtitles,
            trickplay,
        } => {
            let body = jellyfin_api::types::BackupOptionsDto {
                database,
                metadata,
                subtitles,
                trickplay,
            };
            let result = client.create_backup(&body).await?;
            crate::output::print_json(&result)?;
        }
        BackupCommand::Get { path } => {
            let result = client.get_backup(&path).await?;
            crate::output::print_json(&result)?;
        }
        BackupCommand::Restore { archive_file_name } => {
            let body = jellyfin_api::types::BackupRestoreRequestDto {
                archive_file_name,
            };
            client.start_restore_backup(&body).await?;
        }
    }
    Ok(())
}
