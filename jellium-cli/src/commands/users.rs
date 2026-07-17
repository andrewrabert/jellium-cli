use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum UsersCommand {
    /// List all users
    List {
        /// Filter by disabled status
        #[arg(long)]
        is_disabled: Option<bool>,
        /// Filter by hidden status
        #[arg(long)]
        is_hidden: Option<bool>,
    },
    /// Get a user by ID
    Get {
        /// The user ID
        user_id: Uuid,
    },
    /// Get the current authenticated user
    Me,
    /// Get publicly visible users
    Public,
    /// Delete a user
    Delete {
        /// The user ID to delete
        user_id: Uuid,
    },
    /// Create a new user
    Create {
        /// Username
        #[arg(long)]
        name: String,
        /// Password
        #[arg(long)]
        password: Option<String>,
    },
    /// Update a user
    Update {
        /// The user's name
        #[arg(long)]
        name: Option<String>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Update a user's password
    UpdatePassword {
        /// Current password
        #[arg(long)]
        current_password: Option<String>,
        /// New password
        #[arg(long)]
        new_password: Option<String>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Update a user's policy
    UpdatePolicy {
        /// The user ID
        user_id: Uuid,
        /// Is administrator
        #[arg(long)]
        is_administrator: Option<bool>,
        /// Is disabled
        #[arg(long)]
        is_disabled: Option<bool>,
        /// Is hidden
        #[arg(long)]
        is_hidden: Option<bool>,
        /// Enable media playback
        #[arg(long)]
        enable_media_playback: Option<bool>,
        /// Enable remote access
        #[arg(long)]
        enable_remote_access: Option<bool>,
        /// Authentication provider ID
        #[arg(long)]
        authentication_provider_id: Option<String>,
        /// Password reset provider ID
        #[arg(long)]
        password_reset_provider_id: Option<String>,
    },
    /// Update a user's configuration
    UpdateConfig {
        /// Audio language preference
        #[arg(long)]
        audio_language_preference: Option<String>,
        /// Play default audio track
        #[arg(long)]
        play_default_audio_track: Option<bool>,
        /// Display missing episodes
        #[arg(long)]
        display_missing_episodes: Option<bool>,
        /// Enable next episode auto play
        #[arg(long)]
        enable_next_episode_auto_play: Option<bool>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Initiate forgot password process
    ForgotPassword {
        /// The username
        username: String,
    },
    /// Redeem a forgot password pin
    ForgotPasswordPin {
        /// The pin
        pin: String,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &UsersCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        UsersCommand::List {
            is_disabled,
            is_hidden,
        } => {
            let result = client.get_users(*is_disabled, *is_hidden).await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::Get { user_id: uid } => {
            let result = client.get_user_by_id(uid).await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::Me => {
            let result = client.get_current_user().await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::Public => {
            let result = client.get_public_users().await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::Delete { user_id: uid } => {
            client.delete_user(uid).await?;
        }
        UsersCommand::Create { name, password } => {
            let body = jellyfin_api::types::CreateUserByName {
                name: name.clone(),
                password: password.clone(),
            };
            let result = client.create_user_by_name(&body).await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::Update {
            name,
            user_id: uid,
        } => {
            let effective_user_id = uid.as_ref().unwrap_or(user_id);
            let body = jellyfin_api::types::UserDto {
                name: name.clone(),
                ..Default::default()
            };
            client
                .update_user(Some(effective_user_id), &body)
                .await?;
        }
        UsersCommand::UpdatePassword {
            current_password,
            new_password,
            user_id: uid,
        } => {
            let effective_user_id = uid.as_ref().unwrap_or(user_id);
            let body = jellyfin_api::types::UpdateUserPassword {
                current_pw: current_password.clone(),
                new_pw: new_password.clone(),
                ..Default::default()
            };
            client
                .update_user_password(Some(effective_user_id), &body)
                .await?;
        }
        UsersCommand::UpdatePolicy {
            user_id: uid,
            is_administrator,
            is_disabled,
            is_hidden,
            enable_media_playback,
            enable_remote_access,
            authentication_provider_id,
            password_reset_provider_id,
        } => {
            let body = jellyfin_api::types::UserPolicy {
                is_administrator: *is_administrator,
                is_disabled: *is_disabled,
                is_hidden: *is_hidden,
                enable_media_playback: *enable_media_playback,
                enable_remote_access: *enable_remote_access,
                authentication_provider_id: authentication_provider_id
                    .clone()
                    .unwrap_or_default(),
                password_reset_provider_id: password_reset_provider_id
                    .clone()
                    .unwrap_or_default(),
                enable_collection_management: false,
                enable_lyric_management: false,
                enable_subtitle_management: false,
                access_schedules: None,
                allowed_tags: None,
                block_unrated_items: None,
                blocked_channels: None,
                blocked_media_folders: None,
                blocked_tags: None,
                enable_all_channels: None,
                enable_all_devices: None,
                enable_all_folders: None,
                enable_audio_playback_transcoding: None,
                enable_content_deletion: None,
                enable_content_deletion_from_folders: None,
                enable_content_downloading: None,
                enable_live_tv_access: None,
                enable_live_tv_management: None,
                enable_media_conversion: None,
                enable_playback_remuxing: None,
                enable_public_sharing: None,
                enable_remote_control_of_other_users: None,
                enable_shared_device_control: None,
                enable_sync_transcoding: None,
                enable_user_preference_access: None,
                enable_video_playback_transcoding: None,
                enabled_channels: None,
                enabled_devices: None,
                enabled_folders: None,
                force_remote_source_transcoding: None,
                invalid_login_attempt_count: None,
                login_attempts_before_lockout: None,
                max_active_sessions: None,
                max_parental_rating: None,
                max_parental_sub_rating: None,
                remote_client_bitrate_limit: None,
                sync_play_access: None,
            };
            client.update_user_policy(uid, &body).await?;
        }
        UsersCommand::UpdateConfig {
            audio_language_preference,
            play_default_audio_track,
            display_missing_episodes,
            enable_next_episode_auto_play,
            user_id: uid,
        } => {
            let effective_user_id = uid.as_ref().unwrap_or(user_id);
            let body = jellyfin_api::types::UserConfiguration {
                audio_language_preference: audio_language_preference.clone(),
                play_default_audio_track: *play_default_audio_track,
                display_missing_episodes: *display_missing_episodes,
                enable_next_episode_auto_play: *enable_next_episode_auto_play,
                ..Default::default()
            };
            client
                .update_user_configuration(Some(effective_user_id), &body)
                .await?;
        }
        UsersCommand::ForgotPassword { username } => {
            let body = jellyfin_api::types::ForgotPasswordDto {
                entered_username: username.clone(),
            };
            let result = client.forgot_password(&body).await?;
            crate::output::print_json(&result)?;
        }
        UsersCommand::ForgotPasswordPin { pin } => {
            let body = jellyfin_api::types::ForgotPasswordPinDto { pin: pin.clone() };
            let result = client.forgot_password_pin(&body).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
