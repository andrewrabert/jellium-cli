use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum StartupCommand {
    /// Get startup configuration
    GetConfig,
    /// Get startup user
    GetUser,
    /// Complete the startup wizard
    Complete,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: StartupCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        StartupCommand::GetConfig => {
            let result = client.get_startup_configuration().await?;
            crate::output::print_json(&result)?;
        }
        StartupCommand::GetUser => {
            let result = client.get_first_user().await?;
            crate::output::print_json(&result)?;
        }
        StartupCommand::Complete => {
            client.complete_wizard().await?;
        }
    }
    Ok(())
}
