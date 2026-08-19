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
        UsersCommand::Update { name, user_id: uid } => {
            let effective_user_id = uid.as_ref().unwrap_or(user_id);
            let body = jellyfin_api::types::UserDto {
                name: name.clone(),
                ..Default::default()
            };
            client.update_user(Some(effective_user_id), &body).await?;
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
            let mut body = client
                .get_user_by_id(uid)
                .await?
                .policy
                .ok_or("the server reported no policy for this user")?;
            if let Some(value) = is_administrator {
                body.is_administrator = Some(*value);
            }
            if let Some(value) = is_disabled {
                body.is_disabled = Some(*value);
            }
            if let Some(value) = is_hidden {
                body.is_hidden = Some(*value);
            }
            if let Some(value) = enable_media_playback {
                body.enable_media_playback = Some(*value);
            }
            if let Some(value) = enable_remote_access {
                body.enable_remote_access = Some(*value);
            }
            if let Some(value) = authentication_provider_id {
                body.authentication_provider_id = value.clone();
            }
            if let Some(value) = password_reset_provider_id {
                body.password_reset_provider_id = value.clone();
            }
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
            let mut body = client
                .get_user_by_id(effective_user_id)
                .await?
                .configuration
                .unwrap_or_default();
            if let Some(value) = audio_language_preference {
                body.audio_language_preference = Some(value.clone());
            }
            if let Some(value) = play_default_audio_track {
                body.play_default_audio_track = Some(*value);
            }
            if let Some(value) = display_missing_episodes {
                body.display_missing_episodes = Some(*value);
            }
            if let Some(value) = enable_next_episode_auto_play {
                body.enable_next_episode_auto_play = Some(*value);
            }
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
