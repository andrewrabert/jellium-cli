use clap::Subcommand;
use uuid::Uuid;

#[derive(Clone, Subcommand)]
pub enum PackagesCommand {
    /// List available packages
    List,
    /// Get package info by name
    Info {
        /// Package name
        name: String,
        /// Assembly GUID
        #[arg(long)]
        assembly_guid: Option<Uuid>,
    },
    /// Install a package
    Install {
        /// Package name
        name: String,
        /// Assembly GUID
        #[arg(long)]
        assembly_guid: Option<Uuid>,
        /// Repository URL
        #[arg(long)]
        repository_url: Option<String>,
        /// Package version
        #[arg(long)]
        version: Option<String>,
    },
    /// Cancel a package installation
    Cancel {
        /// Installation ID
        id: Uuid,
    },
    /// List package repositories
    ReposList,
    /// Set package repositories
    ReposSet {
        /// Repository name
        #[arg(long)]
        name: Option<String>,
        /// Repository URL
        #[arg(long)]
        url: Option<String>,
        /// Whether the repository is enabled
        #[arg(long)]
        enabled: Option<bool>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: PackagesCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PackagesCommand::List => {
            let result = client.get_packages().await?;
            crate::output::print_json(&result)?;
        }
        PackagesCommand::Info {
            name,
            assembly_guid,
        } => {
            let result = client
                .get_package_info(&name, assembly_guid.as_ref())
                .await?;
            crate::output::print_json(&result)?;
        }
        PackagesCommand::Install {
            name,
            assembly_guid,
            repository_url,
            version,
        } => {
            client
                .install_package(
                    &name,
                    assembly_guid.as_ref(),
                    repository_url.as_deref(),
                    version.as_deref(),
                )
                .await?;
        }
        PackagesCommand::Cancel { id } => {
            client.cancel_package_installation(&id).await?;
        }
        PackagesCommand::ReposList => {
            let result = client.get_repositories().await?;
            crate::output::print_json(&result)?;
        }
        PackagesCommand::ReposSet { name, url, enabled } => {
            let repo = jellyfin_api::types::RepositoryInfo {
                name: name.clone(),
                url: url.clone(),
                enabled,
            };
            let body = vec![repo];
            client.set_repositories(&body).await?;
        }
    }
    Ok(())
}
