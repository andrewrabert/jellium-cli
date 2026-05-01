use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum EnvironmentCommand {
    /// Get default directory browser info
    DefaultDirectoryBrowser,
    /// Get contents of a directory
    DirectoryContents {
        /// The path to list
        path: String,
        /// Include directories in results
        #[arg(long)]
        include_directories: Option<bool>,
        /// Include files in results
        #[arg(long)]
        include_files: Option<bool>,
    },
    /// Get available drives
    Drives,
    /// Get network shares
    NetworkShares,
    /// Get parent path of a given path
    ParentPath {
        /// The path
        path: String,
    },
    /// Validate a path
    ValidatePath {
        /// The path to validate
        #[arg(long)]
        path: Option<String>,
        /// Whether the path is a file
        #[arg(long)]
        is_file: Option<bool>,
        /// Whether to validate if path is writable
        #[arg(long)]
        validate_writable: Option<bool>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: EnvironmentCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        EnvironmentCommand::DefaultDirectoryBrowser => {
            let result = client.get_default_directory_browser().await?;
            crate::output::print_json(&result)?;
        }
        EnvironmentCommand::DirectoryContents {
            path,
            include_directories,
            include_files,
        } => {
            let result = client
                .get_directory_contents(include_directories, include_files, &path)
                .await?;
            crate::output::print_json(&result)?;
        }
        EnvironmentCommand::Drives => {
            let result = client.get_drives().await?;
            crate::output::print_json(&result)?;
        }
        EnvironmentCommand::NetworkShares => {
            let result = client.get_network_shares().await?;
            crate::output::print_json(&result)?;
        }
        EnvironmentCommand::ParentPath { path } => {
            let result = client.get_parent_path(&path).await?;
            crate::output::print_json(&result)?;
        }
        EnvironmentCommand::ValidatePath {
            path,
            is_file,
            validate_writable,
        } => {
            let body = jellyfin_api::types::ValidatePathDto {
                path,
                is_file,
                validate_writable,
            };
            client.validate_path(&body).await?;
        }
    }
    Ok(())
}
