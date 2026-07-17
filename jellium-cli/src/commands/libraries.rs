use clap::Subcommand;
use jellyfin_api::types::TaskState;
use std::time::Duration;
use uuid::Uuid;

const MAX_INITIAL_IDLE_POLLS: u32 = 5;

pub enum WaitTarget {
    Task(String),
    Libraries(Vec<String>),
}

fn is_task_active(state: &Option<TaskState>) -> bool {
    matches!(state, Some(TaskState::Running) | Some(TaskState::Cancelling))
}

async fn wait_for_refresh(
    client: &jellyfin_api::Client,
    target: WaitTarget,
    poll_secs: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let poll_interval = Duration::from_secs(poll_secs);
    let mut saw_active = false;
    let mut idle_polls: u32 = 0;
    loop {
        let any_active = match &target {
            WaitTarget::Task(task_id) => {
                is_task_active(&client.get_task(task_id).await?.state)
            }
            WaitTarget::Libraries(ids) => {
                let folders = client.get_virtual_folders().await?;
                ids.iter().any(|id| {
                    folders
                        .iter()
                        .find(|f| f.item_id.as_deref() == Some(id.as_str()))
                        .and_then(|f| f.refresh_status.as_deref())
                        .is_some_and(|s| !s.eq_ignore_ascii_case("Idle"))
                })
            }
        };

        if any_active {
            saw_active = true;
            idle_polls = 0;
        } else if saw_active || idle_polls >= MAX_INITIAL_IDLE_POLLS {
            break;
        } else {
            idle_polls += 1;
        }
        tokio::time::sleep(poll_interval).await;
    }
    Ok(())
}

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
    ///
    /// Requires either --name (repeatable) to refresh specific libraries, or
    /// --all to refresh every library. By default, returns immediately;
    /// pass --wait/-w to block until the scan finishes.
    Refresh {
        /// Name of a library to refresh (repeatable; ignored when --all is set)
        #[arg(long)]
        name: Vec<String>,
        /// Refresh all libraries
        #[arg(long)]
        all: bool,
        /// Wait for the refresh to finish
        #[arg(short = 'w', long)]
        wait: bool,
        /// Poll interval in seconds while waiting
        #[arg(long, default_value_t = 5)]
        poll: u64,
    },
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
        LibrariesCommand::Refresh {
            name,
            all,
            wait,
            poll,
        } => {
            let target = if *all {
                let tasks = client.get_tasks(None, None).await?;
                let task = tasks
                    .into_iter()
                    .find(|t| t.key.as_deref() == Some("RefreshLibrary"))
                    .ok_or("RefreshLibrary scheduled task not found")?;
                let id = task.id.ok_or("RefreshLibrary task has no id")?;
                if !is_task_active(&task.state) {
                    client.start_task(&id).await?;
                }
                WaitTarget::Task(id)
            } else if name.is_empty() {
                return Err("must specify --name (repeatable) or --all".into());
            } else {
                let folders = client.get_virtual_folders().await?;
                let mut ids = Vec::with_capacity(name.len());
                for n in name {
                    let folder = folders
                        .iter()
                        .find(|f| f.name.as_deref() == Some(n.as_str()))
                        .ok_or_else(|| format!("library not found: {n}"))?;
                    let item_id_str = folder
                        .item_id
                        .as_deref()
                        .ok_or_else(|| format!("library has no item id: {n}"))?;
                    let item_id = Uuid::parse_str(item_id_str)?;
                    client
                        .refresh_item(&item_id, None, None, None, None, None)
                        .await?;
                    ids.push(item_id_str.to_string());
                }
                WaitTarget::Libraries(ids)
            };
            if *wait {
                wait_for_refresh(client, target, *poll).await?;
            }
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
