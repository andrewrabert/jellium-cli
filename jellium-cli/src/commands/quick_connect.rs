use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum QuickConnectCommand {
    /// Authorize a pending quick connect request
    Authorize {
        /// Quick connect code to authorize
        code: String,
        /// The user to authorize
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get quick connect authentication state
    State {
        /// Secret previously returned from the Initiate endpoint
        secret: String,
    },
    /// Check if quick connect is enabled
    Enabled,
    /// Initiate a new quick connect request
    Initiate,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: QuickConnectCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        QuickConnectCommand::Authorize { code, user_id } => {
            let result = client
                .authorize_quick_connect(&code, user_id.as_ref())
                .await?;
            crate::output::print_json(&result)?;
        }
        QuickConnectCommand::State { secret } => {
            let result = client.get_quick_connect_state(&secret).await?;
            crate::output::print_json(&result)?;
        }
        QuickConnectCommand::Enabled => {
            let result = client.get_quick_connect_enabled().await?;
            crate::output::print_json(&result)?;
        }
        QuickConnectCommand::Initiate => {
            let result = client.initiate_quick_connect().await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
