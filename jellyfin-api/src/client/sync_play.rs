use crate::Client;
use crate::error::Error;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets a SyncPlay group by id\n\nSends a `GET` request to `/SyncPlay/{id}`\n\nArguments:\n- `id`: The id of the group.\n"]
    pub async fn sync_play_get_group(&self, id: &uuid::Uuid) -> Result<types::GroupInfoDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/SyncPlay/{}", encode_path(&id.to_string())),
        )
        .send()
        .await
    }

    #[doc = "Notify SyncPlay group that member is buffering\n\nSends a `POST` request to `/SyncPlay/Buffering`\n\nArguments:\n- `body`: The player status.\n"]
    pub async fn sync_play_buffering(&self, body: &types::BufferRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Buffering".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Join an existing SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Join`\n\nArguments:\n- `body`: The group to join.\n"]
    pub async fn sync_play_join_group(
        &self,
        body: &types::JoinGroupRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Join".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Leave the joined SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Leave`\n\n"]
    pub async fn sync_play_leave_group(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Leave".into())
            .send_no_content()
            .await
    }

    #[doc = "Gets all SyncPlay groups\n\nSends a `GET` request to `/SyncPlay/List`\n\n"]
    pub async fn sync_play_get_groups(&self) -> Result<Vec<types::GroupInfoDto>, Error> {
        self.request(reqwest::Method::GET, "/SyncPlay/List".into())
            .send()
            .await
    }

    #[doc = "Request to move an item in the playlist in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/MovePlaylistItem`\n\nArguments:\n- `body`: The new position for the item.\n"]
    pub async fn sync_play_move_playlist_item(
        &self,
        body: &types::MovePlaylistItemRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/MovePlaylistItem".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Create a new SyncPlay group\n\nSends a `POST` request to `/SyncPlay/New`\n\nArguments:\n- `body`: The settings of the new group.\n"]
    pub async fn sync_play_create_group(
        &self,
        body: &types::NewGroupRequestDto,
    ) -> Result<types::GroupInfoDto, Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/New".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Request next item in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/NextItem`\n\nArguments:\n- `body`: The current item information.\n"]
    pub async fn sync_play_next_item(&self, body: &types::NextItemRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/NextItem".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request pause in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Pause`\n\n"]
    pub async fn sync_play_pause(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Pause".into())
            .send_no_content()
            .await
    }

    #[doc = "Update session ping\n\nSends a `POST` request to `/SyncPlay/Ping`\n\nArguments:\n- `body`: The new ping.\n"]
    pub async fn sync_play_ping(&self, body: &types::PingRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Ping".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request previous item in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/PreviousItem`\n\nArguments:\n- `body`: The current item information.\n"]
    pub async fn sync_play_previous_item(
        &self,
        body: &types::PreviousItemRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/PreviousItem".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to queue items to the playlist of a SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Queue`\n\nArguments:\n- `body`: The items to add.\n"]
    pub async fn sync_play_queue(&self, body: &types::QueueRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Queue".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Notify SyncPlay group that member is ready for playback\n\nSends a `POST` request to `/SyncPlay/Ready`\n\nArguments:\n- `body`: The player status.\n"]
    pub async fn sync_play_ready(&self, body: &types::ReadyRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Ready".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to remove items from the playlist in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/RemoveFromPlaylist`\n\nArguments:\n- `body`: The items to remove.\n"]
    pub async fn sync_play_remove_from_playlist(
        &self,
        body: &types::RemoveFromPlaylistRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/RemoveFromPlaylist".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request seek in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Seek`\n\nArguments:\n- `body`: The new playback position.\n"]
    pub async fn sync_play_seek(&self, body: &types::SeekRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Seek".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request SyncPlay group to ignore member during group-wait\n\nSends a `POST` request to `/SyncPlay/SetIgnoreWait`\n\nArguments:\n- `body`: The settings to set.\n"]
    pub async fn sync_play_set_ignore_wait(
        &self,
        body: &types::IgnoreWaitRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/SetIgnoreWait".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to set new playlist in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/SetNewQueue`\n\nArguments:\n- `body`: The new playlist to play in the group.\n"]
    pub async fn sync_play_set_new_queue(&self, body: &types::PlayRequestDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/SetNewQueue".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to change playlist item in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/SetPlaylistItem`\n\nArguments:\n- `body`: The new item to play.\n"]
    pub async fn sync_play_set_playlist_item(
        &self,
        body: &types::SetPlaylistItemRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/SetPlaylistItem".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to set repeat mode in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/SetRepeatMode`\n\nArguments:\n- `body`: The new repeat mode.\n"]
    pub async fn sync_play_set_repeat_mode(
        &self,
        body: &types::SetRepeatModeRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/SetRepeatMode".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request to set shuffle mode in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/SetShuffleMode`\n\nArguments:\n- `body`: The new shuffle mode.\n"]
    pub async fn sync_play_set_shuffle_mode(
        &self,
        body: &types::SetShuffleModeRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/SetShuffleMode".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Request stop in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Stop`\n\n"]
    pub async fn sync_play_stop(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Stop".into())
            .send_no_content()
            .await
    }

    #[doc = "Request unpause in SyncPlay group\n\nSends a `POST` request to `/SyncPlay/Unpause`\n\n"]
    pub async fn sync_play_unpause(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/SyncPlay/Unpause".into())
            .send_no_content()
            .await
    }
}
