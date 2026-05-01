use super::*;

#[doc = "Class BufferRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BufferRequestDto {
    #[doc = "Gets or sets a value indicating whether the client playback is unpaused."]
    #[serde(
        rename = "IsPlaying",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_playing: Option<bool>,
    #[doc = "Gets or sets the playlist item identifier of the playing item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
    #[doc = "Gets or sets when the request has been made by the client."]
    #[serde(
        rename = "When",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub when: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for BufferRequestDto {
    fn default() -> Self {
        Self {
            is_playing: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            when: Default::default(),
        }
    }
}

#[doc = "Class GroupInfoDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GroupInfoDto {
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[doc = "Gets the group name."]
    #[serde(
        rename = "GroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_name: Option<String>,
    #[doc = "Gets the date when this DTO has been created."]
    #[serde(
        rename = "LastUpdatedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets the participants."]
    #[serde(
        rename = "Participants",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub participants: Vec<String>,
    #[serde(
        rename = "State",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<GroupStateType>,
}

impl Default for GroupInfoDto {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            group_name: Default::default(),
            last_updated_at: Default::default(),
            participants: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GroupQueueMode {
    Queue,
    QueueNext,
}

impl std::fmt::Display for GroupQueueMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Queue => f.write_str("Queue"),
            Self::QueueNext => f.write_str("QueueNext"),
        }
    }
}

impl std::str::FromStr for GroupQueueMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Queue" => Ok(Self::Queue),
            "QueueNext" => Ok(Self::QueueNext),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GroupQueueMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GroupQueueMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GroupQueueMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GroupShuffleMode {
    Sorted,
    Shuffle,
}

impl std::fmt::Display for GroupShuffleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Sorted => f.write_str("Sorted"),
            Self::Shuffle => f.write_str("Shuffle"),
        }
    }
}

impl std::str::FromStr for GroupShuffleMode {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Sorted" => Ok(Self::Sorted),
            "Shuffle" => Ok(Self::Shuffle),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GroupShuffleMode {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GroupShuffleMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GroupShuffleMode {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GroupStateType {
    Idle,
    Waiting,
    Paused,
    Playing,
}

impl std::fmt::Display for GroupStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("Idle"),
            Self::Waiting => f.write_str("Waiting"),
            Self::Paused => f.write_str("Paused"),
            Self::Playing => f.write_str("Playing"),
        }
    }
}

impl std::str::FromStr for GroupStateType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Idle" => Ok(Self::Idle),
            "Waiting" => Ok(Self::Waiting),
            "Paused" => Ok(Self::Paused),
            "Playing" => Ok(Self::Playing),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GroupStateType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GroupStateType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GroupStateType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class GroupStateUpdate."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GroupStateUpdate {
    #[serde(
        rename = "Reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reason: Option<PlaybackRequestType>,
    #[serde(
        rename = "State",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<GroupStateType>,
}

impl Default for GroupStateUpdate {
    fn default() -> Self {
        Self {
            reason: Default::default(),
            state: Default::default(),
        }
    }
}

#[doc = "Represents the list of possible group update types"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GroupUpdate {
    GroupDoesNotExistUpdate(SyncPlayGroupDoesNotExistUpdate),
    GroupJoinedUpdate(SyncPlayGroupJoinedUpdate),
    GroupLeftUpdate(SyncPlayGroupLeftUpdate),
    LibraryAccessDeniedUpdate(SyncPlayLibraryAccessDeniedUpdate),
    NotInGroupUpdate(SyncPlayNotInGroupUpdate),
    PlayQueueUpdate(SyncPlayPlayQueueUpdate),
    StateUpdate(SyncPlayStateUpdate),
    UserJoinedUpdate(SyncPlayUserJoinedUpdate),
    UserLeftUpdate(SyncPlayUserLeftUpdate),
}

impl From<SyncPlayGroupDoesNotExistUpdate> for GroupUpdate {
    fn from(value: SyncPlayGroupDoesNotExistUpdate) -> Self {
        Self::GroupDoesNotExistUpdate(value)
    }
}

impl From<SyncPlayGroupJoinedUpdate> for GroupUpdate {
    fn from(value: SyncPlayGroupJoinedUpdate) -> Self {
        Self::GroupJoinedUpdate(value)
    }
}

impl From<SyncPlayGroupLeftUpdate> for GroupUpdate {
    fn from(value: SyncPlayGroupLeftUpdate) -> Self {
        Self::GroupLeftUpdate(value)
    }
}

impl From<SyncPlayLibraryAccessDeniedUpdate> for GroupUpdate {
    fn from(value: SyncPlayLibraryAccessDeniedUpdate) -> Self {
        Self::LibraryAccessDeniedUpdate(value)
    }
}

impl From<SyncPlayNotInGroupUpdate> for GroupUpdate {
    fn from(value: SyncPlayNotInGroupUpdate) -> Self {
        Self::NotInGroupUpdate(value)
    }
}

impl From<SyncPlayPlayQueueUpdate> for GroupUpdate {
    fn from(value: SyncPlayPlayQueueUpdate) -> Self {
        Self::PlayQueueUpdate(value)
    }
}

impl From<SyncPlayStateUpdate> for GroupUpdate {
    fn from(value: SyncPlayStateUpdate) -> Self {
        Self::StateUpdate(value)
    }
}

impl From<SyncPlayUserJoinedUpdate> for GroupUpdate {
    fn from(value: SyncPlayUserJoinedUpdate) -> Self {
        Self::UserJoinedUpdate(value)
    }
}

impl From<SyncPlayUserLeftUpdate> for GroupUpdate {
    fn from(value: SyncPlayUserLeftUpdate) -> Self {
        Self::UserLeftUpdate(value)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GroupUpdateType {
    UserJoined,
    UserLeft,
    GroupJoined,
    GroupLeft,
    StateUpdate,
    PlayQueue,
    NotInGroup,
    GroupDoesNotExist,
    LibraryAccessDenied,
}

impl std::fmt::Display for GroupUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UserJoined => f.write_str("UserJoined"),
            Self::UserLeft => f.write_str("UserLeft"),
            Self::GroupJoined => f.write_str("GroupJoined"),
            Self::GroupLeft => f.write_str("GroupLeft"),
            Self::StateUpdate => f.write_str("StateUpdate"),
            Self::PlayQueue => f.write_str("PlayQueue"),
            Self::NotInGroup => f.write_str("NotInGroup"),
            Self::GroupDoesNotExist => f.write_str("GroupDoesNotExist"),
            Self::LibraryAccessDenied => f.write_str("LibraryAccessDenied"),
        }
    }
}

impl std::str::FromStr for GroupUpdateType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "UserJoined" => Ok(Self::UserJoined),
            "UserLeft" => Ok(Self::UserLeft),
            "GroupJoined" => Ok(Self::GroupJoined),
            "GroupLeft" => Ok(Self::GroupLeft),
            "StateUpdate" => Ok(Self::StateUpdate),
            "PlayQueue" => Ok(Self::PlayQueue),
            "NotInGroup" => Ok(Self::NotInGroup),
            "GroupDoesNotExist" => Ok(Self::GroupDoesNotExist),
            "LibraryAccessDenied" => Ok(Self::LibraryAccessDenied),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for GroupUpdateType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for GroupUpdateType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for GroupUpdateType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class IgnoreWaitRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct IgnoreWaitRequestDto {
    #[doc = "Gets or sets a value indicating whether the client should be ignored."]
    #[serde(
        rename = "IgnoreWait",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_wait: Option<bool>,
}

impl Default for IgnoreWaitRequestDto {
    fn default() -> Self {
        Self {
            ignore_wait: Default::default(),
        }
    }
}

#[doc = "Class JoinGroupRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct JoinGroupRequestDto {
    #[doc = "Gets or sets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
}

impl Default for JoinGroupRequestDto {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
        }
    }
}

#[doc = "Class MovePlaylistItemRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MovePlaylistItemRequestDto {
    #[doc = "Gets or sets the new position."]
    #[serde(
        rename = "NewIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_index: Option<i32>,
    #[doc = "Gets or sets the playlist identifier of the item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl Default for MovePlaylistItemRequestDto {
    fn default() -> Self {
        Self {
            new_index: Default::default(),
            playlist_item_id: Default::default(),
        }
    }
}

#[doc = "Class NewGroupRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NewGroupRequestDto {
    #[doc = "Gets or sets the group name."]
    #[serde(
        rename = "GroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_name: Option<String>,
}

impl Default for NewGroupRequestDto {
    fn default() -> Self {
        Self {
            group_name: Default::default(),
        }
    }
}

#[doc = "Class NextItemRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NextItemRequestDto {
    #[doc = "Gets or sets the playing item identifier."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl Default for NextItemRequestDto {
    fn default() -> Self {
        Self {
            playlist_item_id: Default::default(),
        }
    }
}

#[doc = "Class PingRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PingRequestDto {
    #[doc = "Gets or sets the ping time."]
    #[serde(
        rename = "Ping",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ping: Option<i64>,
}

impl Default for PingRequestDto {
    fn default() -> Self {
        Self {
            ping: Default::default(),
        }
    }
}

#[doc = "Class PreviousItemRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PreviousItemRequestDto {
    #[doc = "Gets or sets the playing item identifier."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl Default for PreviousItemRequestDto {
    fn default() -> Self {
        Self {
            playlist_item_id: Default::default(),
        }
    }
}

#[doc = "Class QueueRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct QueueRequestDto {
    #[doc = "Gets or sets the items to enqueue."]
    #[serde(
        rename = "ItemIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub item_ids: Vec<uuid::Uuid>,
    #[serde(
        rename = "Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mode: Option<GroupQueueMode>,
}

impl Default for QueueRequestDto {
    fn default() -> Self {
        Self {
            item_ids: Default::default(),
            mode: Default::default(),
        }
    }
}

#[doc = "Class ReadyRequest."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReadyRequestDto {
    #[doc = "Gets or sets a value indicating whether the client playback is unpaused."]
    #[serde(
        rename = "IsPlaying",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_playing: Option<bool>,
    #[doc = "Gets or sets the playlist item identifier of the playing item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
    #[doc = "Gets or sets when the request has been made by the client."]
    #[serde(
        rename = "When",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub when: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for ReadyRequestDto {
    fn default() -> Self {
        Self {
            is_playing: Default::default(),
            playlist_item_id: Default::default(),
            position_ticks: Default::default(),
            when: Default::default(),
        }
    }
}

#[doc = "Class RemoveFromPlaylistRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RemoveFromPlaylistRequestDto {
    #[doc = "Gets or sets a value indicating whether the playing item should be removed as well. Used only when clearing the playlist."]
    #[serde(
        rename = "ClearPlayingItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_playing_item: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the entire playlist should be cleared."]
    #[serde(
        rename = "ClearPlaylist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_playlist: Option<bool>,
    #[doc = "Gets or sets the playlist identifiers of the items. Ignored when clearing the playlist."]
    #[serde(
        rename = "PlaylistItemIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playlist_item_ids: Vec<uuid::Uuid>,
}

impl Default for RemoveFromPlaylistRequestDto {
    fn default() -> Self {
        Self {
            clear_playing_item: Default::default(),
            clear_playlist: Default::default(),
            playlist_item_ids: Default::default(),
        }
    }
}

#[doc = "Class SeekRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeekRequestDto {
    #[doc = "Gets or sets the position ticks."]
    #[serde(
        rename = "PositionTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<i64>,
}

impl Default for SeekRequestDto {
    fn default() -> Self {
        Self {
            position_ticks: Default::default(),
        }
    }
}

#[doc = "Class SetPlaylistItemRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SetPlaylistItemRequestDto {
    #[doc = "Gets or sets the playlist identifier of the playing item."]
    #[serde(
        rename = "PlaylistItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub playlist_item_id: Option<uuid::Uuid>,
}

impl Default for SetPlaylistItemRequestDto {
    fn default() -> Self {
        Self {
            playlist_item_id: Default::default(),
        }
    }
}

#[doc = "Class SetShuffleModeRequestDto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SetShuffleModeRequestDto {
    #[serde(
        rename = "Mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mode: Option<GroupShuffleMode>,
}

impl Default for SetShuffleModeRequestDto {
    fn default() -> Self {
        Self {
            mode: Default::default(),
        }
    }
}

#[doc = "`SyncPlayGroupDoesNotExistUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayGroupDoesNotExistUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayGroupDoesNotExistUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "`SyncPlayGroupJoinedUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayGroupJoinedUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<GroupInfoDto>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayGroupJoinedUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "`SyncPlayGroupLeftUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayGroupLeftUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayGroupLeftUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Untyped sync play command."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayGroupUpdateMessage {
    #[doc = "Group update data"]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<GroupUpdate>,
    #[doc = "Gets or sets the message id."]
    #[serde(
        rename = "MessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for SyncPlayGroupUpdateMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "`SyncPlayLibraryAccessDeniedUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayLibraryAccessDeniedUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayLibraryAccessDeniedUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "`SyncPlayNotInGroupUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayNotInGroupUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayNotInGroupUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyncPlayUserAccessType {
    CreateAndJoinGroups,
    JoinGroups,
    None,
}

impl std::fmt::Display for SyncPlayUserAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::CreateAndJoinGroups => f.write_str("CreateAndJoinGroups"),
            Self::JoinGroups => f.write_str("JoinGroups"),
            Self::None => f.write_str("None"),
        }
    }
}

impl std::str::FromStr for SyncPlayUserAccessType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "CreateAndJoinGroups" => Ok(Self::CreateAndJoinGroups),
            "JoinGroups" => Ok(Self::JoinGroups),
            "None" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for SyncPlayUserAccessType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SyncPlayUserAccessType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SyncPlayUserAccessType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`SyncPlayUserJoinedUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayUserJoinedUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayUserJoinedUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "`SyncPlayUserLeftUpdate`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SyncPlayUserLeftUpdate {
    #[doc = "Gets the update data."]
    #[serde(
        rename = "Data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    #[doc = "Gets the group identifier."]
    #[serde(
        rename = "GroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_id: Option<uuid::Uuid>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<GroupUpdateType>,
}

impl Default for SyncPlayUserLeftUpdate {
    fn default() -> Self {
        Self {
            data: Default::default(),
            group_id: Default::default(),
            type_: Default::default(),
        }
    }
}

