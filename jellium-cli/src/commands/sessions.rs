use clap::Subcommand;
use jellyfin_api::types::{
    BaseItemKind, GeneralCommandType, MediaType, MessageCommand, PlayCommand, PlaystateCommand,
};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum SessionsCommand {
    /// List active sessions
    List {
        /// Filter by sessions active in the last N seconds
        #[arg(long)]
        active_within_seconds: Option<i32>,

        /// Filter by sessions controllable by this user
        #[arg(long)]
        controllable_by_user_id: Option<Uuid>,

        /// Filter by device ID
        #[arg(long)]
        device_id: Option<String>,
    },

    /// Send a message to a session
    SendMessage {
        /// The session ID
        session_id: String,

        /// Message header
        #[arg(long)]
        header: Option<String>,

        /// Message text
        #[arg(long)]
        text: String,

        /// Timeout in milliseconds
        #[arg(long)]
        timeout_ms: Option<i64>,
    },

    /// Send a general command to a session
    SendGeneralCommand {
        /// The session ID
        session_id: String,

        /// The command to send
        command: GeneralCommandType,
    },

    /// Send a system command to a session
    SendSystemCommand {
        /// The session ID
        session_id: String,

        /// The command to send
        command: GeneralCommandType,
    },

    /// Instruct a session to play items
    Play {
        /// The session ID
        session_id: String,

        /// Item IDs to play (comma separated)
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<Uuid>,

        /// Play command (PlayNow, PlayNext, PlayLast)
        #[arg(long)]
        play_command: PlayCommand,

        /// Audio stream index
        #[arg(long)]
        audio_stream_index: Option<i32>,

        /// Media source ID
        #[arg(long)]
        media_source_id: Option<String>,

        /// Start index in the playlist
        #[arg(long)]
        start_index: Option<i32>,

        /// Starting position in ticks
        #[arg(long)]
        start_position_ticks: Option<i64>,

        /// Subtitle stream index
        #[arg(long)]
        subtitle_stream_index: Option<i32>,
    },

    /// Send a playstate command to a session
    PlayState {
        /// The session ID
        session_id: String,

        /// The playstate command
        command: PlaystateCommand,

        /// Controlling user ID
        #[arg(long)]
        controlling_user_id: Option<String>,

        /// Seek position in ticks
        #[arg(long)]
        seek_position_ticks: Option<i64>,
    },

    /// Add a user to a session
    AddUser {
        /// The session ID
        session_id: String,

        /// The user ID to add
        user_id: Uuid,
    },

    /// Remove a user from a session
    RemoveUser {
        /// The session ID
        session_id: String,

        /// The user ID to remove
        user_id: Uuid,
    },

    /// Instruct a session to browse to an item
    DisplayContent {
        /// The session ID
        session_id: String,

        /// The item ID
        #[arg(long)]
        item_id: String,

        /// The item name
        #[arg(long)]
        item_name: String,

        /// The item type
        #[arg(long)]
        item_type: BaseItemKind,
    },

    /// Update capabilities for a device
    PostCapabilities {
        /// The session ID
        #[arg(long)]
        id: Option<String>,

        /// Playable media types (comma separated)
        #[arg(long, value_delimiter = ',')]
        playable_media_types: Option<Vec<MediaType>>,

        /// Supported remote control commands (comma separated)
        #[arg(long, value_delimiter = ',')]
        supported_commands: Option<Vec<GeneralCommandType>>,

        /// Whether the device supports media control
        #[arg(long)]
        supports_media_control: Option<bool>,

        /// Whether the device supports a persistent identifier
        #[arg(long)]
        supports_persistent_identifier: Option<bool>,
    },

    /// Report that a session has ended
    ReportSessionEnded,

    /// Report that a session is viewing an item
    ReportViewing {
        /// The item ID
        item_id: String,

        /// The session ID
        #[arg(long)]
        session_id: Option<String>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    _user_id: &Uuid,
    command: &SessionsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        SessionsCommand::List {
            active_within_seconds,
            controllable_by_user_id,
            device_id,
        } => {
            let result = client
                .get_sessions(
                    *active_within_seconds,
                    controllable_by_user_id.as_ref(),
                    device_id.as_deref(),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        SessionsCommand::SendMessage {
            session_id,
            header,
            text,
            timeout_ms,
        } => {
            let body = MessageCommand {
                header: header.clone(),
                text: text.clone(),
                timeout_ms: *timeout_ms,
            };
            client.send_message_command(session_id, &body).await?;
        }
        SessionsCommand::SendGeneralCommand {
            session_id,
            command,
        } => {
            client.send_general_command(session_id, *command).await?;
        }
        SessionsCommand::SendSystemCommand {
            session_id,
            command,
        } => {
            client.send_system_command(session_id, *command).await?;
        }
        SessionsCommand::Play {
            session_id,
            item_ids,
            play_command,
            audio_stream_index,
            media_source_id,
            start_index,
            start_position_ticks,
            subtitle_stream_index,
        } => {
            client
                .play(
                    session_id,
                    *audio_stream_index,
                    item_ids,
                    media_source_id.as_deref(),
                    *play_command,
                    *start_index,
                    *start_position_ticks,
                    *subtitle_stream_index,
                )
                .await?;
        }
        SessionsCommand::PlayState {
            session_id,
            command,
            controlling_user_id,
            seek_position_ticks,
        } => {
            client
                .send_playstate_command(
                    session_id,
                    *command,
                    controlling_user_id.as_deref(),
                    *seek_position_ticks,
                )
                .await?;
        }
        SessionsCommand::AddUser {
            session_id,
            user_id,
        } => {
            client.add_user_to_session(session_id, user_id).await?;
        }
        SessionsCommand::RemoveUser {
            session_id,
            user_id,
        } => {
            client.remove_user_from_session(session_id, user_id).await?;
        }
        SessionsCommand::DisplayContent {
            session_id,
            item_id,
            item_name,
            item_type,
        } => {
            client
                .display_content(session_id, item_id, item_name, *item_type)
                .await?;
        }
        SessionsCommand::PostCapabilities {
            id,
            playable_media_types,
            supported_commands,
            supports_media_control,
            supports_persistent_identifier,
        } => {
            client
                .post_capabilities(
                    id.as_deref(),
                    playable_media_types.as_ref(),
                    supported_commands.as_ref(),
                    *supports_media_control,
                    *supports_persistent_identifier,
                )
                .await?;
        }
        SessionsCommand::ReportSessionEnded => {
            client.report_session_ended().await?;
        }
        SessionsCommand::ReportViewing {
            item_id,
            session_id,
        } => {
            client
                .report_viewing(item_id, session_id.as_deref())
                .await?;
        }
    }
    Ok(())
}
