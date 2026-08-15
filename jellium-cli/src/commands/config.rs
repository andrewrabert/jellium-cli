use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum ConfigCommand {
    /// Get server configuration
    Get,
    /// Get a named configuration
    GetNamed {
        /// Configuration key
        key: String,
    },
    /// Get default metadata options
    MetadataDefaults,
    /// List fallback font files
    FallbackFonts,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: ConfigCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ConfigCommand::Get => {
            let result = client.get_configuration().await?;
            crate::output::print_json(&result)?;
        }
        ConfigCommand::GetNamed { key } => {
            let result = client
                .get_named_configuration::<serde_json::Value>(&key)
                .await?;
            crate::output::print_json(&result)?;
        }
        ConfigCommand::MetadataDefaults => {
            let result = client.get_default_metadata_options().await?;
            crate::output::print_json(&result)?;
        }
        ConfigCommand::FallbackFonts => {
            let result = client.get_fallback_font_list().await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
