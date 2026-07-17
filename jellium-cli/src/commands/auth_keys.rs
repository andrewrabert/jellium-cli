use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum AuthKeysCommand {
    /// List all API keys
    List,
    /// Create a new API key
    Create {
        /// Name of the app using the key
        app: String,
    },
    /// Revoke an API key
    Revoke {
        /// The access token to revoke
        key: String,
    },
    /// List password reset providers
    PasswordResetProviders,
    /// List auth providers
    AuthProviders,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: AuthKeysCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        AuthKeysCommand::List => {
            let result = client.get_keys().await?;
            crate::output::print_json(&result)?;
        }
        AuthKeysCommand::Create { app } => {
            client.create_key(&app).await?;
        }
        AuthKeysCommand::Revoke { key } => {
            client.revoke_key(&key).await?;
        }
        AuthKeysCommand::PasswordResetProviders => {
            let result = client.get_password_reset_providers().await?;
            crate::output::print_json(&result)?;
        }
        AuthKeysCommand::AuthProviders => {
            let result = client.get_auth_providers().await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
