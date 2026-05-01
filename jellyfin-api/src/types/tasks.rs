use super::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TaskCompletionStatus {
    Completed,
    Failed,
    Cancelled,
    Aborted,
}

impl std::fmt::Display for TaskCompletionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("Completed"),
            Self::Failed => f.write_str("Failed"),
            Self::Cancelled => f.write_str("Cancelled"),
            Self::Aborted => f.write_str("Aborted"),
        }
    }
}

impl std::str::FromStr for TaskCompletionStatus {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Completed" => Ok(Self::Completed),
            "Failed" => Ok(Self::Failed),
            "Cancelled" => Ok(Self::Cancelled),
            "Aborted" => Ok(Self::Aborted),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TaskCompletionStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TaskCompletionStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TaskCompletionStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class TaskInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskInfo {
    #[doc = "Gets or sets the category."]
    #[serde(
        rename = "Category",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub category: Option<String>,
    #[doc = "Gets or sets the progress."]
    #[serde(
        rename = "CurrentProgressPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_progress_percentage: Option<f64>,
    #[doc = "Gets or sets the description."]
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[doc = "Gets or sets the id."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[doc = "Gets or sets a value indicating whether this instance is hidden."]
    #[serde(
        rename = "IsHidden",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hidden: Option<bool>,
    #[doc = "Gets or sets the key."]
    #[serde(
        rename = "Key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[doc = "Gets or sets the last execution result."]
    #[serde(
        rename = "LastExecutionResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_execution_result: Option<TaskResult>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        rename = "State",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<TaskState>,
    #[doc = "Gets or sets the triggers."]
    #[serde(
        rename = "Triggers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub triggers: Option<Vec<TaskTriggerInfo>>,
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            category: Default::default(),
            current_progress_percentage: Default::default(),
            description: Default::default(),
            id: Default::default(),
            is_hidden: Default::default(),
            key: Default::default(),
            last_execution_result: Default::default(),
            name: Default::default(),
            state: Default::default(),
            triggers: Default::default(),
        }
    }
}

#[doc = "Class TaskExecutionInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskResult {
    #[doc = "Gets or sets the end time UTC."]
    #[serde(
        rename = "EndTimeUtc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_time_utc: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the error message."]
    #[serde(
        rename = "ErrorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    #[doc = "Gets or sets the id."]
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[doc = "Gets or sets the key."]
    #[serde(
        rename = "Key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[doc = "Gets or sets the long error message."]
    #[serde(
        rename = "LongErrorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_error_message: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[doc = "Gets or sets the start time UTC."]
    #[serde(
        rename = "StartTimeUtc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time_utc: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "Status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<TaskCompletionStatus>,
}

impl Default for TaskResult {
    fn default() -> Self {
        Self {
            end_time_utc: Default::default(),
            error_message: Default::default(),
            id: Default::default(),
            key: Default::default(),
            long_error_message: Default::default(),
            name: Default::default(),
            start_time_utc: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TaskState {
    Idle,
    Cancelling,
    Running,
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("Idle"),
            Self::Cancelling => f.write_str("Cancelling"),
            Self::Running => f.write_str("Running"),
        }
    }
}

impl std::str::FromStr for TaskState {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Idle" => Ok(Self::Idle),
            "Cancelling" => Ok(Self::Cancelling),
            "Running" => Ok(Self::Running),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TaskState {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TaskState {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TaskState {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Class TaskTriggerInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TaskTriggerInfo {
    #[doc = "Gets or sets the day of week."]
    #[serde(
        rename = "DayOfWeek",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub day_of_week: Option<DayOfWeek>,
    #[doc = "Gets or sets the interval."]
    #[serde(
        rename = "IntervalTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_ticks: Option<i64>,
    #[doc = "Gets or sets the maximum runtime ticks."]
    #[serde(
        rename = "MaxRuntimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_runtime_ticks: Option<i64>,
    #[doc = "Gets or sets the time of day."]
    #[serde(
        rename = "TimeOfDayTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_of_day_ticks: Option<i64>,
    #[serde(
        rename = "Type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<TaskTriggerInfoType>,
}

impl Default for TaskTriggerInfo {
    fn default() -> Self {
        Self {
            day_of_week: Default::default(),
            interval_ticks: Default::default(),
            max_runtime_ticks: Default::default(),
            time_of_day_ticks: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TaskTriggerInfoType {
    DailyTrigger,
    WeeklyTrigger,
    IntervalTrigger,
    StartupTrigger,
}

impl std::fmt::Display for TaskTriggerInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::DailyTrigger => f.write_str("DailyTrigger"),
            Self::WeeklyTrigger => f.write_str("WeeklyTrigger"),
            Self::IntervalTrigger => f.write_str("IntervalTrigger"),
            Self::StartupTrigger => f.write_str("StartupTrigger"),
        }
    }
}

impl std::str::FromStr for TaskTriggerInfoType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "DailyTrigger" => Ok(Self::DailyTrigger),
            "WeeklyTrigger" => Ok(Self::WeeklyTrigger),
            "IntervalTrigger" => Ok(Self::IntervalTrigger),
            "StartupTrigger" => Ok(Self::StartupTrigger),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for TaskTriggerInfoType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TaskTriggerInfoType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: &String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TaskTriggerInfoType {
    type Error = super::error::ConversionError;
    fn try_from(
        value: String,
    ) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

