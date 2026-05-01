use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Reports playback has started within a session\n\nSends a `POST` request to `/Sessions/Playing`\n\nArguments:\n- `body`: The playback start info.\n"]
    pub async fn report_playback_start(
        &self,
        body: &types::PlaybackStartInfo,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Playing".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Pings a playback session\n\nSends a `POST` request to `/Sessions/Playing/Ping`\n\nArguments:\n- `play_session_id`: Playback session id.\n"]
    pub async fn ping_playback_session(
        &self,
        play_session_id: &str,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Playing/Ping".into())
            .query("playSessionId", play_session_id)
            .send_no_content()
            .await
    }

    #[doc = "Reports playback progress within a session\n\nSends a `POST` request to `/Sessions/Playing/Progress`\n\nArguments:\n- `body`: The playback progress info.\n"]
    pub async fn report_playback_progress(
        &self,
        body: &types::PlaybackProgressInfo,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Playing/Progress".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Reports playback has stopped within a session\n\nSends a `POST` request to `/Sessions/Playing/Stopped`\n\nArguments:\n- `body`: The playback stop info.\n"]
    pub async fn report_playback_stopped(
        &self,
        body: &types::PlaybackStopInfo,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Playing/Stopped".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a list of sessions\n\nSends a `GET` request to `/Sessions`\n\nArguments:\n- `active_within_seconds`: Optional. Filter by sessions that were active in the last n seconds.\n- `controllable_by_user_id`: Filter by sessions that a given user is allowed to remote control.\n- `device_id`: Filter by device Id.\n"]
    pub async fn get_sessions(
        &self,
        active_within_seconds: Option<i32>,
        controllable_by_user_id: Option<&uuid::Uuid>,
        device_id: Option<&str>,
    ) -> Result<Vec<types::SessionInfoDto>, Error> {
        self.request(reqwest::Method::GET, "/Sessions".into())
            .query_opt("activeWithinSeconds", active_within_seconds)
            .query_opt("controllableByUserId", controllable_by_user_id)
            .query_opt("deviceId", device_id)
            .send()
            .await
    }

    #[doc = "Issues a full general command to a client\n\nSends a `POST` request to `/Sessions/{sessionId}/Command`\n\nArguments:\n- `session_id`: The session id.\n- `body`: The MediaBrowser.Model.Session.GeneralCommand.\n"]
    pub async fn send_full_general_command(
        &self,
        session_id: &str,
        body: &types::GeneralCommand,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Command", encode_path(session_id)))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Issues a general command to a client\n\nSends a `POST` request to `/Sessions/{sessionId}/Command/{command}`\n\nArguments:\n- `session_id`: The session id.\n- `command`: The command to send.\n"]
    pub async fn send_general_command(
        &self,
        session_id: &str,
        command: types::GeneralCommandType,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Command/{}", encode_path(session_id), encode_path(&command.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Issues a command to a client to display a message to the user\n\nSends a `POST` request to `/Sessions/{sessionId}/Message`\n\nArguments:\n- `session_id`: The session id.\n- `body`: The MediaBrowser.Model.Session.MessageCommand object containing Header, Message Text, and TimeoutMs.\n"]
    pub async fn send_message_command(
        &self,
        session_id: &str,
        body: &types::MessageCommand,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Message", encode_path(session_id)))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Instructs a session to play an item\n\nSends a `POST` request to `/Sessions/{sessionId}/Playing`\n\nArguments:\n- `session_id`: The session id.\n- `audio_stream_index`: Optional. The index of the audio stream to play.\n- `item_ids`: The ids of the items to play, comma delimited.\n- `media_source_id`: Optional. The media source id.\n- `play_command`: The type of play command to issue (PlayNow, PlayNext, PlayLast). Clients who have not yet implemented play next and play last may play now.\n- `start_index`: Optional. The start index.\n- `start_position_ticks`: The starting position of the first item.\n- `subtitle_stream_index`: Optional. The index of the subtitle stream to play.\n"]
    pub async fn play(
        &self,
        session_id: &str,
        audio_stream_index: Option<i32>,
        item_ids: &[uuid::Uuid],
        media_source_id: Option<&str>,
        play_command: types::PlayCommand,
        start_index: Option<i32>,
        start_position_ticks: Option<i64>,
        subtitle_stream_index: Option<i32>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Playing", encode_path(session_id)))
            .query_opt("audioStreamIndex", audio_stream_index)
            .query_list("itemIds", item_ids)
            .query_opt("mediaSourceId", media_source_id)
            .query("playCommand", play_command)
            .query_opt("startIndex", start_index)
            .query_opt("startPositionTicks", start_position_ticks)
            .query_opt("subtitleStreamIndex", subtitle_stream_index)
            .send_no_content()
            .await
    }

    #[doc = "Issues a playstate command to a client\n\nSends a `POST` request to `/Sessions/{sessionId}/Playing/{command}`\n\nArguments:\n- `session_id`: The session id.\n- `command`: The MediaBrowser.Model.Session.PlaystateCommand.\n- `controlling_user_id`: The optional controlling user id.\n- `seek_position_ticks`: The optional position ticks.\n"]
    pub async fn send_playstate_command(
        &self,
        session_id: &str,
        command: types::PlaystateCommand,
        controlling_user_id: Option<&str>,
        seek_position_ticks: Option<i64>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Playing/{}", encode_path(session_id), encode_path(&command.to_string())))
            .query_opt("controllingUserId", controlling_user_id)
            .query_opt("seekPositionTicks", seek_position_ticks)
            .send_no_content()
            .await
    }

    #[doc = "Issues a system command to a client\n\nSends a `POST` request to `/Sessions/{sessionId}/System/{command}`\n\nArguments:\n- `session_id`: The session id.\n- `command`: The command to send.\n"]
    pub async fn send_system_command(
        &self,
        session_id: &str,
        command: types::GeneralCommandType,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/System/{}", encode_path(session_id), encode_path(&command.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Adds an additional user to a session\n\nSends a `POST` request to `/Sessions/{sessionId}/User/{userId}`\n\nArguments:\n- `session_id`: The session id.\n- `user_id`: The user id.\n"]
    pub async fn add_user_to_session(
        &self,
        session_id: &str,
        user_id: &uuid::Uuid,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/User/{}", encode_path(session_id), encode_path(&user_id.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Removes an additional user from a session\n\nSends a `DELETE` request to `/Sessions/{sessionId}/User/{userId}`\n\nArguments:\n- `session_id`: The session id.\n- `user_id`: The user id.\n"]
    pub async fn remove_user_from_session(
        &self,
        session_id: &str,
        user_id: &uuid::Uuid,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/Sessions/{}/User/{}", encode_path(session_id), encode_path(&user_id.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Instructs a session to browse to an item or view\n\nSends a `POST` request to `/Sessions/{sessionId}/Viewing`\n\nArguments:\n- `session_id`: The session Id.\n- `item_id`: The Id of the item.\n- `item_name`: The name of the item.\n- `item_type`: The type of item to browse to.\n"]
    pub async fn display_content(
        &self,
        session_id: &str,
        item_id: &str,
        item_name: &str,
        item_type: types::BaseItemKind,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Sessions/{}/Viewing", encode_path(session_id)))
            .query("itemId", item_id)
            .query("itemName", item_name)
            .query("itemType", item_type)
            .send_no_content()
            .await
    }

    #[doc = "Updates capabilities for a device\n\nSends a `POST` request to `/Sessions/Capabilities`\n\nArguments:\n- `id`: The session id.\n- `playable_media_types`: A list of playable media types, comma delimited. Audio, Video, Book, Photo.\n- `supported_commands`: A list of supported remote control commands, comma delimited.\n- `supports_media_control`: Determines whether media can be played remotely..\n- `supports_persistent_identifier`: Determines whether the device supports a unique identifier.\n"]
    pub async fn post_capabilities(
        &self,
        id: Option<&str>,
        playable_media_types: Option<&Vec<types::MediaType>>,
        supported_commands: Option<&Vec<types::GeneralCommandType>>,
        supports_media_control: Option<bool>,
        supports_persistent_identifier: Option<bool>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Capabilities".into())
            .query_opt("id", id)
            .query_list_opt("playableMediaTypes", playable_media_types)
            .query_list_opt("supportedCommands", supported_commands)
            .query_opt("supportsMediaControl", supports_media_control)
            .query_opt("supportsPersistentIdentifier", supports_persistent_identifier)
            .send_no_content()
            .await
    }

    #[doc = "Updates capabilities for a device\n\nSends a `POST` request to `/Sessions/Capabilities/Full`\n\nArguments:\n- `id`: The session id.\n- `body`: The MediaBrowser.Model.Session.ClientCapabilities.\n"]
    pub async fn post_full_capabilities(
        &self,
        id: Option<&str>,
        body: &types::ClientCapabilitiesDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Capabilities/Full".into())
            .query_opt("id", id)
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Reports that a session has ended\n\nSends a `POST` request to `/Sessions/Logout`\n\n"]
    pub async fn report_session_ended(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Logout".into())
            .send_no_content()
            .await
    }

    #[doc = "Reports that a session is viewing an item\n\nSends a `POST` request to `/Sessions/Viewing`\n\nArguments:\n- `item_id`: The item id.\n- `session_id`: The session id.\n"]
    pub async fn report_viewing(
        &self,
        item_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Sessions/Viewing".into())
            .query("itemId", item_id)
            .query_opt("sessionId", session_id)
            .send_no_content()
            .await
    }
}
