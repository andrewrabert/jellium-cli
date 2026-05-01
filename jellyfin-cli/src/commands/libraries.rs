use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum LibrariesCommand {
    /// Get user views (libraries)
    Views {
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
        /// Include external content (channels, live tv)
        #[arg(long)]
        include_external_content: Option<bool>,
        /// Include hidden content
        #[arg(long)]
        include_hidden: Option<bool>,
    },
    /// Get all media folders
    MediaFolders {
        /// Filter by hidden status
        #[arg(long)]
        is_hidden: Option<bool>,
    },
    /// Start a library scan
    Refresh,
    /// Get all virtual folders
    VirtualFolders,
    /// Add a virtual folder
    AddVirtualFolder {
        /// The name of the virtual folder
        #[arg(long)]
        name: Option<String>,
        /// The collection type
        #[arg(long)]
        collection_type: Option<jellyfin_api::types::CollectionTypeOptions>,
        /// Paths for the virtual folder (comma separated)
        #[arg(long, value_delimiter = ',')]
        paths: Option<Vec<String>>,
        /// Whether to refresh the library
        #[arg(long)]
        refresh_library: Option<bool>,
    },
    /// Remove a virtual folder
    RemoveVirtualFolder {
        /// The name of the folder
        #[arg(long)]
        name: Option<String>,
        /// Whether to refresh the library
        #[arg(long)]
        refresh_library: Option<bool>,
    },
    /// Rename a virtual folder
    RenameVirtualFolder {
        /// The current name of the folder
        #[arg(long)]
        name: Option<String>,
        /// The new name for the folder
        #[arg(long)]
        new_name: Option<String>,
        /// Whether to refresh the library
        #[arg(long)]
        refresh_library: Option<bool>,
    },
    /// Add a media path to a library
    AddMediaPath {
        /// The name of the library
        #[arg(long)]
        name: String,
        /// The path to add
        #[arg(long)]
        path: Option<String>,
        /// Whether to refresh the library
        #[arg(long)]
        refresh_library: Option<bool>,
    },
    /// Remove a media path from a library
    RemoveMediaPath {
        /// The name of the library
        #[arg(long)]
        name: Option<String>,
        /// The path to remove
        #[arg(long)]
        path: Option<String>,
        /// Whether to refresh the library
        #[arg(long)]
        refresh_library: Option<bool>,
    },
    /// Update a media path
    UpdateMediaPath {
        /// The library name
        #[arg(long)]
        name: String,
        /// The path
        #[arg(long)]
        path: Option<String>,
    },
    /// Update library options
    UpdateLibraryOptions {
        /// The library item ID
        #[arg(long)]
        id: Option<Uuid>,
    },
    /// Get physical paths
    PhysicalPaths,
    /// Get library options info
    OptionsInfo {
        /// Whether this is a new library
        #[arg(long)]
        is_new_library: Option<bool>,
        /// Library content type
        #[arg(long)]
        library_content_type: Option<jellyfin_api::types::CollectionType>,
    },
    /// Get user view grouping options
    GroupingOptions {
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Report new movies added by an external source
    NotifyAddedMovies {
        /// The IMDb ID
        #[arg(long)]
        imdb_id: Option<String>,
        /// The TMDb ID
        #[arg(long)]
        tmdb_id: Option<String>,
    },
    /// Report updated movies by an external source
    NotifyUpdatedMovies {
        /// The IMDb ID
        #[arg(long)]
        imdb_id: Option<String>,
        /// The TMDb ID
        #[arg(long)]
        tmdb_id: Option<String>,
    },
    /// Report new series added by an external source
    NotifyAddedSeries {
        /// The TVDb ID
        #[arg(long)]
        tvdb_id: Option<String>,
    },
    /// Report updated series by an external source
    NotifyUpdatedSeries {
        /// The TVDb ID
        #[arg(long)]
        tvdb_id: Option<String>,
    },
    /// Report updated media by an external source
    NotifyUpdatedMedia {
        /// Media path
        #[arg(long)]
        path: Option<String>,
        /// Update type (Created, Modified, Deleted)
        #[arg(long)]
        update_type: Option<String>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &LibrariesCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        LibrariesCommand::Views {
            user_id: uid,
            include_external_content,
            include_hidden,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_user_views(
                    *include_external_content,
                    *include_hidden,
                    None,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::MediaFolders { is_hidden } => {
            let result = client.get_media_folders(*is_hidden).await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::Refresh => {
            client.refresh_library().await?;
        }
        LibrariesCommand::VirtualFolders => {
            let result = client.get_virtual_folders().await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::AddVirtualFolder {
            name,
            collection_type,
            paths,
            refresh_library,
        } => {
            let body = jellyfin_api::types::AddVirtualFolderDto::default();
            client
                .add_virtual_folder(
                    *collection_type,
                    name.as_deref(),
                    paths.as_ref(),
                    *refresh_library,
                    &body,
                )
                .await?;
        }
        LibrariesCommand::RemoveVirtualFolder {
            name,
            refresh_library,
        } => {
            client
                .remove_virtual_folder(name.as_deref(), *refresh_library)
                .await?;
        }
        LibrariesCommand::RenameVirtualFolder {
            name,
            new_name,
            refresh_library,
        } => {
            client
                .rename_virtual_folder(name.as_deref(), new_name.as_deref(), *refresh_library)
                .await?;
        }
        LibrariesCommand::AddMediaPath {
            name,
            path,
            refresh_library,
        } => {
            let body = jellyfin_api::types::MediaPathDto {
                name: name.clone(),
                path: path.clone(),
                path_info: None,
            };
            client.add_media_path(*refresh_library, &body).await?;
        }
        LibrariesCommand::RemoveMediaPath {
            name,
            path,
            refresh_library,
        } => {
            client
                .remove_media_path(name.as_deref(), path.as_deref(), *refresh_library)
                .await?;
        }
        LibrariesCommand::UpdateMediaPath { name, path } => {
            let body = jellyfin_api::types::UpdateMediaPathRequestDto {
                name: name.clone(),
                path_info: jellyfin_api::types::MediaPathInfo {
                    path: path.clone(),
                },
            };
            client.update_media_path(&body).await?;
        }
        LibrariesCommand::UpdateLibraryOptions { id } => {
            let body = jellyfin_api::types::UpdateLibraryOptionsDto {
                id: *id,
                library_options: None,
            };
            client.update_library_options(&body).await?;
        }
        LibrariesCommand::PhysicalPaths => {
            let result = client.get_physical_paths().await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::OptionsInfo {
            is_new_library,
            library_content_type,
        } => {
            let result = client
                .get_library_options_info(*is_new_library, *library_content_type)
                .await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::GroupingOptions { user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_grouping_options(Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        LibrariesCommand::NotifyAddedMovies { imdb_id, tmdb_id } => {
            client
                .post_added_movies(imdb_id.as_deref(), tmdb_id.as_deref())
                .await?;
        }
        LibrariesCommand::NotifyUpdatedMovies { imdb_id, tmdb_id } => {
            client
                .post_updated_movies(imdb_id.as_deref(), tmdb_id.as_deref())
                .await?;
        }
        LibrariesCommand::NotifyAddedSeries { tvdb_id } => {
            client.post_added_series(tvdb_id.as_deref()).await?;
        }
        LibrariesCommand::NotifyUpdatedSeries { tvdb_id } => {
            client.post_updated_series(tvdb_id.as_deref()).await?;
        }
        LibrariesCommand::NotifyUpdatedMedia { path, update_type } => {
            let update = jellyfin_api::types::MediaUpdateInfoPathDto {
                path: path.clone(),
                update_type: update_type.clone(),
            };
            let body = jellyfin_api::types::MediaUpdateInfoDto {
                updates: vec![update],
            };
            client.post_updated_media(&body).await?;
        }
    }
    Ok(())
}
