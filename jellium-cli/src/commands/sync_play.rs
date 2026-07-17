use clap::Subcommand;
use jellyfin_api::types::{
    BufferRequestDto, GroupQueueMode, GroupRepeatMode, GroupShuffleMode, IgnoreWaitRequestDto,
    JoinGroupRequestDto, MovePlaylistItemRequestDto, NewGroupRequestDto, NextItemRequestDto,
    PingRequestDto, PlayRequestDto, PreviousItemRequestDto, QueueRequestDto, ReadyRequestDto,
    RemoveFromPlaylistRequestDto, SeekRequestDto, SetRepeatModeRequestDto,
    SetShuffleModeRequestDto,
};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum SyncPlayCommand {
    /// Create a new SyncPlay group
    NewGroup {
        /// Group name
        #[arg(long)]
        group_name: Option<String>,
    },

    /// Join an existing SyncPlay group
    JoinGroup {
        /// The group ID to join
        group_id: Uuid,
    },

    /// Leave the current SyncPlay group
    LeaveGroup,

    /// List all SyncPlay groups
    ListGroups,

    /// Request play (unpause) in SyncPlay group
    Play,

    /// Request pause in SyncPlay group
    Pause,

    /// Request stop in SyncPlay group
    Stop,

    /// Request seek in SyncPlay group
    Seek {
        /// Position in ticks
        #[arg(long)]
        position_ticks: Option<i64>,
    },

    /// Request next item in SyncPlay group
    NextItem {
        /// The playlist item ID of the currently playing item
        #[arg(long)]
        playlist_item_id: Option<Uuid>,
    },

    /// Request previous item in SyncPlay group
    PreviousItem {
        /// The playlist item ID of the currently playing item
        #[arg(long)]
        playlist_item_id: Option<Uuid>,
    },

    /// Set a new queue in SyncPlay group
    SetNewQueue {
        /// Item IDs for the new queue (comma separated)
        #[arg(long, value_delimiter = ',')]
        playing_queue: Vec<Uuid>,

        /// Position of the playing item in the queue
        #[arg(long)]
        playing_item_position: Option<i32>,

        /// Start position ticks
        #[arg(long)]
        start_position_ticks: Option<i64>,
    },

    /// Set repeat mode in SyncPlay group
    SetRepeatMode {
        /// Repeat mode (RepeatOne, RepeatAll, RepeatNone)
        #[arg(long)]
        mode: Option<GroupRepeatMode>,
    },

    /// Set shuffle mode in SyncPlay group
    SetShuffleMode {
        /// Shuffle mode (Sorted, Shuffle)
        #[arg(long)]
        mode: Option<GroupShuffleMode>,
    },

    /// Ping the SyncPlay session
    Ping {
        /// Ping time in milliseconds
        #[arg(long)]
        ping: Option<i64>,
    },

    /// Notify SyncPlay group that member is ready for playback
    Ready {
        /// Whether the client playback is unpaused
        #[arg(long)]
        is_playing: Option<bool>,

        /// The playlist item ID of the playing item
        #[arg(long)]
        playlist_item_id: Option<Uuid>,

        /// Position ticks
        #[arg(long)]
        position_ticks: Option<i64>,

        /// When the request has been made (ISO 8601)
        #[arg(long)]
        when: Option<String>,
    },

    /// Notify SyncPlay group that member is buffering
    Buffering {
        /// Whether the client playback is unpaused
        #[arg(long)]
        is_playing: Option<bool>,

        /// The playlist item ID of the playing item
        #[arg(long)]
        playlist_item_id: Option<Uuid>,

        /// Position ticks
        #[arg(long)]
        position_ticks: Option<i64>,

        /// When the request has been made (ISO 8601)
        #[arg(long)]
        when: Option<String>,
    },

    /// Set whether to ignore this member during group-wait
    SetIgnoreWait {
        /// Whether the client should be ignored
        #[arg(long)]
        ignore_wait: Option<bool>,
    },

    /// Move a playlist item to a new position
    MovePlaylistItem {
        /// The playlist item ID to move
        #[arg(long)]
        playlist_item_id: Option<Uuid>,

        /// The new index position
        #[arg(long)]
        new_index: Option<i32>,
    },

    /// Queue items to the playlist
    Queue {
        /// Item IDs to queue (comma separated)
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<Uuid>,

        /// Queue mode (Queue, QueueNext)
        #[arg(long)]
        mode: Option<GroupQueueMode>,
    },

    /// Remove items from the playlist
    RemoveFromPlaylist {
        /// Playlist item IDs to remove (comma separated)
        #[arg(long, value_delimiter = ',')]
        playlist_item_ids: Vec<Uuid>,

        /// Whether to clear the entire playlist
        #[arg(long)]
        clear_playlist: Option<bool>,

        /// Whether to also remove the playing item when clearing
        #[arg(long)]
        clear_playing_item: Option<bool>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    _user_id: &Uuid,
    command: &SyncPlayCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        SyncPlayCommand::NewGroup { group_name } => {
            let body = NewGroupRequestDto {
                group_name: group_name.clone(),
            };
            let result = client.sync_play_create_group(&body).await?;
            crate::output::print_json(&result)?;
        }
        SyncPlayCommand::JoinGroup { group_id } => {
            let body = JoinGroupRequestDto {
                group_id: Some(*group_id),
            };
            client.sync_play_join_group(&body).await?;
        }
        SyncPlayCommand::LeaveGroup => {
            client.sync_play_leave_group().await?;
        }
        SyncPlayCommand::ListGroups => {
            let result = client.sync_play_get_groups().await?;
            crate::output::print_json(&result)?;
        }
        SyncPlayCommand::Play => {
            client.sync_play_unpause().await?;
        }
        SyncPlayCommand::Pause => {
            client.sync_play_pause().await?;
        }
        SyncPlayCommand::Stop => {
            client.sync_play_stop().await?;
        }
        SyncPlayCommand::Seek { position_ticks } => {
            let body = SeekRequestDto {
                position_ticks: *position_ticks,
            };
            client.sync_play_seek(&body).await?;
        }
        SyncPlayCommand::NextItem { playlist_item_id } => {
            let body = NextItemRequestDto {
                playlist_item_id: *playlist_item_id,
            };
            client.sync_play_next_item(&body).await?;
        }
        SyncPlayCommand::PreviousItem { playlist_item_id } => {
            let body = PreviousItemRequestDto {
                playlist_item_id: *playlist_item_id,
            };
            client.sync_play_previous_item(&body).await?;
        }
        SyncPlayCommand::SetNewQueue {
            playing_queue,
            playing_item_position,
            start_position_ticks,
        } => {
            let body = PlayRequestDto {
                playing_queue: playing_queue.clone(),
                playing_item_position: *playing_item_position,
                start_position_ticks: *start_position_ticks,
            };
            client.sync_play_set_new_queue(&body).await?;
        }
        SyncPlayCommand::SetRepeatMode { mode } => {
            let body = SetRepeatModeRequestDto {
                mode: *mode,
            };
            client.sync_play_set_repeat_mode(&body).await?;
        }
        SyncPlayCommand::SetShuffleMode { mode } => {
            let body = SetShuffleModeRequestDto {
                mode: *mode,
            };
            client.sync_play_set_shuffle_mode(&body).await?;
        }
        SyncPlayCommand::Ping { ping } => {
            let body = PingRequestDto {
                ping: *ping,
            };
            client.sync_play_ping(&body).await?;
        }
        SyncPlayCommand::Ready {
            is_playing,
            playlist_item_id,
            position_ticks,
            when,
        } => {
            let when_parsed = when
                .as_ref()
                .map(|w| w.parse::<chrono::DateTime<chrono::Utc>>())
                .transpose()?;
            let body = ReadyRequestDto {
                is_playing: *is_playing,
                playlist_item_id: *playlist_item_id,
                position_ticks: *position_ticks,
                when: when_parsed,
            };
            client.sync_play_ready(&body).await?;
        }
        SyncPlayCommand::Buffering {
            is_playing,
            playlist_item_id,
            position_ticks,
            when,
        } => {
            let when_parsed = when
                .as_ref()
                .map(|w| w.parse::<chrono::DateTime<chrono::Utc>>())
                .transpose()?;
            let body = BufferRequestDto {
                is_playing: *is_playing,
                playlist_item_id: *playlist_item_id,
                position_ticks: *position_ticks,
                when: when_parsed,
            };
            client.sync_play_buffering(&body).await?;
        }
        SyncPlayCommand::SetIgnoreWait { ignore_wait } => {
            let body = IgnoreWaitRequestDto {
                ignore_wait: *ignore_wait,
            };
            client.sync_play_set_ignore_wait(&body).await?;
        }
        SyncPlayCommand::MovePlaylistItem {
            playlist_item_id,
            new_index,
        } => {
            let body = MovePlaylistItemRequestDto {
                playlist_item_id: *playlist_item_id,
                new_index: *new_index,
            };
            client.sync_play_move_playlist_item(&body).await?;
        }
        SyncPlayCommand::Queue { item_ids, mode } => {
            let body = QueueRequestDto {
                item_ids: item_ids.clone(),
                mode: *mode,
            };
            client.sync_play_queue(&body).await?;
        }
        SyncPlayCommand::RemoveFromPlaylist {
            playlist_item_ids,
            clear_playlist,
            clear_playing_item,
        } => {
            let body = RemoveFromPlaylistRequestDto {
                playlist_item_ids: playlist_item_ids.clone(),
                clear_playlist: *clear_playlist,
                clear_playing_item: *clear_playing_item,
            };
            client.sync_play_remove_from_playlist(&body).await?;
        }
    }
    Ok(())
}
