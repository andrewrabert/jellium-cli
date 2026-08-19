use super::*;

#[doc = "Force keep alive websocket messages."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ForceKeepAliveMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<i32>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[doc = "Keep alive websocket messages."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct InboundKeepAliveMessage {
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[doc = "Represents the list of possible inbound websocket types"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InboundWebSocketMessage {
    ActivityLogEntryStartMessage(ActivityLogEntryStartMessage),
    ActivityLogEntryStopMessage(ActivityLogEntryStopMessage),
    InboundKeepAliveMessage(InboundKeepAliveMessage),
    ScheduledTasksInfoStartMessage(ScheduledTasksInfoStartMessage),
    ScheduledTasksInfoStopMessage(ScheduledTasksInfoStopMessage),
    SessionsStartMessage(SessionsStartMessage),
    SessionsStopMessage(SessionsStopMessage),
}

impl From<ActivityLogEntryStartMessage> for InboundWebSocketMessage {
    fn from(value: ActivityLogEntryStartMessage) -> Self {
        Self::ActivityLogEntryStartMessage(value)
    }
}

impl From<ActivityLogEntryStopMessage> for InboundWebSocketMessage {
    fn from(value: ActivityLogEntryStopMessage) -> Self {
        Self::ActivityLogEntryStopMessage(value)
    }
}

impl From<InboundKeepAliveMessage> for InboundWebSocketMessage {
    fn from(value: InboundKeepAliveMessage) -> Self {
        Self::InboundKeepAliveMessage(value)
    }
}

impl From<ScheduledTasksInfoStartMessage> for InboundWebSocketMessage {
    fn from(value: ScheduledTasksInfoStartMessage) -> Self {
        Self::ScheduledTasksInfoStartMessage(value)
    }
}

impl From<ScheduledTasksInfoStopMessage> for InboundWebSocketMessage {
    fn from(value: ScheduledTasksInfoStopMessage) -> Self {
        Self::ScheduledTasksInfoStopMessage(value)
    }
}

impl From<SessionsStartMessage> for InboundWebSocketMessage {
    fn from(value: SessionsStartMessage) -> Self {
        Self::SessionsStartMessage(value)
    }
}

impl From<SessionsStopMessage> for InboundWebSocketMessage {
    fn from(value: SessionsStopMessage) -> Self {
        Self::SessionsStopMessage(value)
    }
}

#[doc = "Keep alive websocket messages."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct OutboundKeepAliveMessage {
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[doc = "Represents the list of possible outbound websocket types"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OutboundWebSocketMessage {
    ActivityLogEntryMessage(ActivityLogEntryMessage),
    ForceKeepAliveMessage(ForceKeepAliveMessage),
    GeneralCommandMessage(GeneralCommandMessage),
    LibraryChangedMessage(LibraryChangedMessage),
    OutboundKeepAliveMessage(OutboundKeepAliveMessage),
    PlayMessage(PlayMessage),
    PlaystateMessage(PlaystateMessage),
    PluginInstallationCancelledMessage(PluginInstallationCancelledMessage),
    PluginInstallationCompletedMessage(PluginInstallationCompletedMessage),
    PluginInstallationFailedMessage(PluginInstallationFailedMessage),
    PluginInstallingMessage(PluginInstallingMessage),
    PluginUninstalledMessage(PluginUninstalledMessage),
    RefreshProgressMessage(RefreshProgressMessage),
    RestartRequiredMessage(RestartRequiredMessage),
    ScheduledTaskEndedMessage(ScheduledTaskEndedMessage),
    ScheduledTasksInfoMessage(ScheduledTasksInfoMessage),
    SeriesTimerCancelledMessage(SeriesTimerCancelledMessage),
    SeriesTimerCreatedMessage(SeriesTimerCreatedMessage),
    ServerRestartingMessage(ServerRestartingMessage),
    ServerShuttingDownMessage(ServerShuttingDownMessage),
    SessionsMessage(SessionsMessage),
    SyncPlayCommandMessage(SyncPlayCommandMessage),
    TimerCancelledMessage(TimerCancelledMessage),
    TimerCreatedMessage(TimerCreatedMessage),
    UserDataChangedMessage(UserDataChangedMessage),
    UserDeletedMessage(UserDeletedMessage),
    UserUpdatedMessage(Box<UserUpdatedMessage>),
    SyncPlayGroupUpdateMessage(SyncPlayGroupUpdateMessage),
}

impl From<ActivityLogEntryMessage> for OutboundWebSocketMessage {
    fn from(value: ActivityLogEntryMessage) -> Self {
        Self::ActivityLogEntryMessage(value)
    }
}

impl From<ForceKeepAliveMessage> for OutboundWebSocketMessage {
    fn from(value: ForceKeepAliveMessage) -> Self {
        Self::ForceKeepAliveMessage(value)
    }
}

impl From<GeneralCommandMessage> for OutboundWebSocketMessage {
    fn from(value: GeneralCommandMessage) -> Self {
        Self::GeneralCommandMessage(value)
    }
}

impl From<LibraryChangedMessage> for OutboundWebSocketMessage {
    fn from(value: LibraryChangedMessage) -> Self {
        Self::LibraryChangedMessage(value)
    }
}

impl From<OutboundKeepAliveMessage> for OutboundWebSocketMessage {
    fn from(value: OutboundKeepAliveMessage) -> Self {
        Self::OutboundKeepAliveMessage(value)
    }
}

impl From<PlayMessage> for OutboundWebSocketMessage {
    fn from(value: PlayMessage) -> Self {
        Self::PlayMessage(value)
    }
}

impl From<PlaystateMessage> for OutboundWebSocketMessage {
    fn from(value: PlaystateMessage) -> Self {
        Self::PlaystateMessage(value)
    }
}

impl From<PluginInstallationCancelledMessage> for OutboundWebSocketMessage {
    fn from(value: PluginInstallationCancelledMessage) -> Self {
        Self::PluginInstallationCancelledMessage(value)
    }
}

impl From<PluginInstallationCompletedMessage> for OutboundWebSocketMessage {
    fn from(value: PluginInstallationCompletedMessage) -> Self {
        Self::PluginInstallationCompletedMessage(value)
    }
}

impl From<PluginInstallationFailedMessage> for OutboundWebSocketMessage {
    fn from(value: PluginInstallationFailedMessage) -> Self {
        Self::PluginInstallationFailedMessage(value)
    }
}

impl From<PluginInstallingMessage> for OutboundWebSocketMessage {
    fn from(value: PluginInstallingMessage) -> Self {
        Self::PluginInstallingMessage(value)
    }
}

impl From<PluginUninstalledMessage> for OutboundWebSocketMessage {
    fn from(value: PluginUninstalledMessage) -> Self {
        Self::PluginUninstalledMessage(value)
    }
}

impl From<RefreshProgressMessage> for OutboundWebSocketMessage {
    fn from(value: RefreshProgressMessage) -> Self {
        Self::RefreshProgressMessage(value)
    }
}

impl From<RestartRequiredMessage> for OutboundWebSocketMessage {
    fn from(value: RestartRequiredMessage) -> Self {
        Self::RestartRequiredMessage(value)
    }
}

impl From<ScheduledTaskEndedMessage> for OutboundWebSocketMessage {
    fn from(value: ScheduledTaskEndedMessage) -> Self {
        Self::ScheduledTaskEndedMessage(value)
    }
}

impl From<ScheduledTasksInfoMessage> for OutboundWebSocketMessage {
    fn from(value: ScheduledTasksInfoMessage) -> Self {
        Self::ScheduledTasksInfoMessage(value)
    }
}

impl From<SeriesTimerCancelledMessage> for OutboundWebSocketMessage {
    fn from(value: SeriesTimerCancelledMessage) -> Self {
        Self::SeriesTimerCancelledMessage(value)
    }
}

impl From<SeriesTimerCreatedMessage> for OutboundWebSocketMessage {
    fn from(value: SeriesTimerCreatedMessage) -> Self {
        Self::SeriesTimerCreatedMessage(value)
    }
}

impl From<ServerRestartingMessage> for OutboundWebSocketMessage {
    fn from(value: ServerRestartingMessage) -> Self {
        Self::ServerRestartingMessage(value)
    }
}

impl From<ServerShuttingDownMessage> for OutboundWebSocketMessage {
    fn from(value: ServerShuttingDownMessage) -> Self {
        Self::ServerShuttingDownMessage(value)
    }
}

impl From<SessionsMessage> for OutboundWebSocketMessage {
    fn from(value: SessionsMessage) -> Self {
        Self::SessionsMessage(value)
    }
}

impl From<SyncPlayCommandMessage> for OutboundWebSocketMessage {
    fn from(value: SyncPlayCommandMessage) -> Self {
        Self::SyncPlayCommandMessage(value)
    }
}

impl From<TimerCancelledMessage> for OutboundWebSocketMessage {
    fn from(value: TimerCancelledMessage) -> Self {
        Self::TimerCancelledMessage(value)
    }
}

impl From<TimerCreatedMessage> for OutboundWebSocketMessage {
    fn from(value: TimerCreatedMessage) -> Self {
        Self::TimerCreatedMessage(value)
    }
}

impl From<UserDataChangedMessage> for OutboundWebSocketMessage {
    fn from(value: UserDataChangedMessage) -> Self {
        Self::UserDataChangedMessage(value)
    }
}

impl From<UserDeletedMessage> for OutboundWebSocketMessage {
    fn from(value: UserDeletedMessage) -> Self {
        Self::UserDeletedMessage(value)
    }
}

impl From<UserUpdatedMessage> for OutboundWebSocketMessage {
    fn from(value: UserUpdatedMessage) -> Self {
        Self::UserUpdatedMessage(Box::new(value))
    }
}

impl From<SyncPlayGroupUpdateMessage> for OutboundWebSocketMessage {
    fn from(value: SyncPlayGroupUpdateMessage) -> Self {
        Self::SyncPlayGroupUpdateMessage(value)
    }
}

#[doc = "Represents the possible websocket types"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum WebSocketMessage {
    InboundWebSocketMessage(InboundWebSocketMessage),
    OutboundWebSocketMessage(Box<OutboundWebSocketMessage>),
}

impl From<InboundWebSocketMessage> for WebSocketMessage {
    fn from(value: InboundWebSocketMessage) -> Self {
        Self::InboundWebSocketMessage(value)
    }
}

impl From<OutboundWebSocketMessage> for WebSocketMessage {
    fn from(value: OutboundWebSocketMessage) -> Self {
        Self::OutboundWebSocketMessage(Box::new(value))
    }
}
