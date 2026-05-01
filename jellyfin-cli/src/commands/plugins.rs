use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum PluginsCommand {
    /// List installed plugins
    List,
    /// Uninstall a plugin
    Uninstall {
        /// Plugin ID
        id: Uuid,
    },
    /// Uninstall a specific plugin version
    UninstallVersion {
        /// Plugin ID
        id: Uuid,
        /// Plugin version
        version: String,
    },
    /// Disable a plugin
    Disable {
        /// Plugin ID
        id: Uuid,
        /// Plugin version
        version: String,
    },
    /// Enable a plugin
    Enable {
        /// Plugin ID
        id: Uuid,
        /// Plugin version
        version: String,
    },
    /// Get plugin configuration
    GetConfig {
        /// Plugin ID
        id: Uuid,
    },
    /// Update plugin configuration
    UpdateConfig {
        /// Plugin ID
        id: Uuid,
    },
    /// Get plugin manifest
    Manifest {
        /// Plugin ID
        id: Uuid,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: PluginsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PluginsCommand::List => {
            let result = client.get_plugins().await?;
            crate::output::print_json(&result)?;
        }
        PluginsCommand::Uninstall { id } => {
            client.uninstall_plugin(&id).await?;
        }
        PluginsCommand::UninstallVersion { id, version } => {
            client.uninstall_plugin_by_version(&id, &version).await?;
        }
        PluginsCommand::Disable { id, version } => {
            client.disable_plugin(&id, &version).await?;
        }
        PluginsCommand::Enable { id, version } => {
            client.enable_plugin(&id, &version).await?;
        }
        PluginsCommand::GetConfig { id } => {
            let result = client.get_plugin_configuration(&id).await?;
            crate::output::print_json(&result)?;
        }
        PluginsCommand::UpdateConfig { id } => {
            client.update_plugin_configuration(&id).await?;
        }
        PluginsCommand::Manifest { id } => {
            client.get_plugin_manifest(&id).await?;
        }
    }
    Ok(())
}
