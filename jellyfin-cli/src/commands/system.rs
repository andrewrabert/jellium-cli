use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum SystemCommand {
    /// Get server system info
    Info,
    /// Get public system info
    PublicInfo,
    /// Ping the server
    Ping,
    /// Restart the server
    Restart,
    /// Shutdown the server
    Shutdown,
    /// Get server storage info
    Storage,
    /// List server log files
    LogsList,
    /// Get a specific log file
    LogGet {
        /// Log file name
        name: String,
    },
    /// Get activity log entries
    Activity {
        #[arg(long)]
        has_user_id: Option<bool>,
        #[arg(long)]
        limit: Option<i32>,
        #[arg(long)]
        start_index: Option<i32>,
    },
    /// Get current UTC time
    Time,
    /// Get endpoint info
    EndpointInfo,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: SystemCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        SystemCommand::Info => {
            let result = client.get_system_info().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::PublicInfo => {
            let result = client.get_public_system_info().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::Ping => {
            let result = client.get_ping_system().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::Restart => {
            client.restart_application().await?;
        }
        SystemCommand::Shutdown => {
            client.shutdown_application().await?;
        }
        SystemCommand::Storage => {
            let result = client.get_system_storage().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::LogsList => {
            let result = client.get_server_logs().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::LogGet { name } => {
            let response = client.get_log_file(&name).await?;
            let text = response.text().await?;
            print!("{}", text);
        }
        SystemCommand::Activity {
            has_user_id,
            limit,
            start_index,
        } => {
            let result = client
                .get_log_entries(has_user_id, limit, None, start_index)
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        SystemCommand::Time => {
            let result = client.get_utc_time().await?;
            crate::output::print_json(&result)?;
        }
        SystemCommand::EndpointInfo => {
            let result = client.get_endpoint_info().await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
