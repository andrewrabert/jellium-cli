use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum BrandingCommand {
    /// Get branding configuration
    Options,
    /// Get branding CSS
    Css,
    /// Delete custom splashscreen
    DeleteSplashscreen,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: BrandingCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        BrandingCommand::Options => {
            let result = client.get_branding_options().await?;
            crate::output::print_json(&result)?;
        }
        BrandingCommand::Css => {
            let result = client.get_branding_css().await?;
            crate::output::print_json(&result)?;
        }
        BrandingCommand::DeleteSplashscreen => {
            client.delete_custom_splashscreen().await?;
        }
    }
    Ok(())
}
