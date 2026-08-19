use super::*;

#[doc = "An activity log entry."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ActivityLogEntry {
    #[doc = "Gets or sets the date."]
    #[serde(rename = "Date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the identifier."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Gets or sets the item identifier."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the overview."]
    #[serde(rename = "Overview", default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "Severity", default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<LogLevel>,
    #[doc = "Gets or sets the short overview."]
    #[serde(
        rename = "ShortOverview",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_overview: Option<String>,
    #[doc = "Gets or sets the type."]
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the user identifier."]
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the user primary image tag."]
    #[serde(
        rename = "UserPrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_primary_image_tag: Option<String>,
}

#[doc = "Activity log created message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ActivityLogEntryMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ActivityLogEntry>>,
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

#[doc = "Query result container."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ActivityLogEntryQueryResult {
    #[doc = "Gets or sets the items."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ActivityLogEntry>,
    #[doc = "Gets or sets the index of the first record in Items."]
    #[serde(
        rename = "StartIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i32>,
    #[doc = "Gets or sets the total number of records available."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

#[doc = "Activity log entry start message.\r\nData is the timing data encoded as \"$initialDelay,$interval\" in ms."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ActivityLogEntryStartMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

#[doc = "Activity log entry stop message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ActivityLogEntryStopMessage {
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}
