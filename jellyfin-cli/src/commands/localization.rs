use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum LocalizationCommand {
    /// Get known countries
    Countries,
    /// Get known cultures
    Cultures,
    /// Get localization options
    Options,
    /// Get known parental ratings
    ParentalRatings,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: LocalizationCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        LocalizationCommand::Countries => {
            let result = client.get_countries().await?;
            crate::output::print_json(&result)?;
        }
        LocalizationCommand::Cultures => {
            let result = client.get_cultures().await?;
            crate::output::print_json(&result)?;
        }
        LocalizationCommand::Options => {
            let result = client.get_localization_options().await?;
            crate::output::print_json(&result)?;
        }
        LocalizationCommand::ParentalRatings => {
            let result = client.get_parental_ratings().await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
