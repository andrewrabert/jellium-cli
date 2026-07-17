use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum DisplayPrefsCommand {
    /// Get display preferences
    Get {
        /// Display preferences ID
        id: String,
        /// Client name
        client: String,
        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: DisplayPrefsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        DisplayPrefsCommand::Get {
            id,
            client: client_name,
            user_id,
        } => {
            let result = client
                .get_display_preferences(&id, &client_name, user_id.as_ref())
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
