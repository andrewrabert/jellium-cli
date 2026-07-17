use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum DevicesCommand {
    /// List all devices
    List {
        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get device info
    Get {
        /// Device ID
        id: String,
    },
    /// Delete a device
    Delete {
        /// Device ID
        id: String,
    },
    /// Get device options
    Options {
        /// Device ID
        id: String,
    },
    /// Update device options
    UpdateOptions {
        /// Device ID
        id: String,
        /// Custom name for the device
        #[arg(long)]
        custom_name: Option<String>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: DevicesCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        DevicesCommand::List { user_id } => {
            let result = client.get_devices(user_id.as_ref()).await?;
            crate::output::print_json(&result)?;
        }
        DevicesCommand::Get { id } => {
            let result = client.get_device_info(&id).await?;
            crate::output::print_json(&result)?;
        }
        DevicesCommand::Delete { id } => {
            client.delete_device(&id).await?;
        }
        DevicesCommand::Options { id } => {
            let result = client.get_device_options(&id).await?;
            crate::output::print_json(&result)?;
        }
        DevicesCommand::UpdateOptions { id, custom_name } => {
            let body = jellyfin_api::types::DeviceOptionsDto {
                custom_name,
                ..Default::default()
            };
            client.update_device_options(&id, &body).await?;
        }
    }
    Ok(())
}
