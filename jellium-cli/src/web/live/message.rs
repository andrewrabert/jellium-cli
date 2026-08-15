use std::time::Duration;

use jellium_protocol::{Control, Event, Marked};
use jellyfin_api::types::SessionInfoDto;
use uuid::Uuid;

use super::{group, verbs};

/// The ids one `LibraryChanged` message named; a message naming none dispatches
/// as `Ignored`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Changed {
    pub added: Vec<Uuid>,
    pub removed: Vec<Uuid>,
    pub updated: Vec<Uuid>,
}

impl Changed {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.updated.is_empty()
    }

    /// Takes every id `other` names, keeping each list free of duplicates.
    pub fn absorb(&mut self, other: Changed) {
        for (held, more) in [
            (&mut self.added, other.added),
            (&mut self.removed, other.removed),
            (&mut self.updated, other.updated),
        ] {
            for id in more {
                if !held.contains(&id) {
                    held.push(id);
                }
            }
        }
    }
}

/// What one upstream frame produces locally.
#[derive(Debug, Clone)]
pub enum Dispatched {
    /// Reaches every tab.
    Broadcast(Event),
    /// The user-data changes one message carried, coalesced before they are
    /// sent.
    Marked(Vec<Marked>),
    /// Reaches the one destination tab.
    Command(Control),
    /// The session listing, which feeds the targets and remote mode.
    Sessions(Vec<SessionInfoDto>),
    /// The interval the Jellyfin server asked to be kept alive at.
    KeepAlive(Duration),
    /// A group command, whose instant the clock converts before a tab sees it.
    Scheduled(jellyfin_api::types::SendCommand),
    /// A group update, read by the type it names.
    Group(group::Update),
    /// One of the four timer events, coalesced before it is sent.
    Timer(jellium_protocol::TimerChanged),
    /// Every scheduled task one `ScheduledTasksInfo` carried.
    Tasks(Vec<jellium_protocol::TaskState>),
    /// The ids one `LibraryChanged` carried, coalesced before they are sent.
    Library(Changed),
    /// One activity entry, coalesced before it is sent.
    Activity(jellium_protocol::ActivityEntry),
    /// One refresh progress, coalesced before it is sent.
    Refresh(jellium_protocol::Refreshed),
    /// One of the five package messages, already shaped as the event it
    /// becomes.
    Package(Event),
    /// The signed-in user's policy changed.
    UserUpdated {
        administrator: bool,
        preference_access: bool,
    },
    /// Nothing this milestone handles.
    Ignored,
}

/// Session listings are compared by what they serialize to, because the
/// generated `SessionInfoDto` carries no equality.
/// Every variant is named on the left-hand side and no arm is a catch-all, so
/// a variant added without an arm of its own fails to compile.
#[cfg(test)]
impl PartialEq for Dispatched {
    fn eq(&self, other: &Dispatched) -> bool {
        match (self, other) {
            (Dispatched::Broadcast(ours), Dispatched::Broadcast(theirs)) => ours == theirs,
            (Dispatched::Marked(ours), Dispatched::Marked(theirs)) => ours == theirs,
            (Dispatched::Library(ours), Dispatched::Library(theirs)) => ours == theirs,
            (Dispatched::Command(ours), Dispatched::Command(theirs)) => ours == theirs,
            (Dispatched::Sessions(ours), Dispatched::Sessions(theirs)) => {
                serde_json::to_value(ours).ok() == serde_json::to_value(theirs).ok()
            }
            (Dispatched::KeepAlive(ours), Dispatched::KeepAlive(theirs)) => ours == theirs,
            (Dispatched::Scheduled(ours), Dispatched::Scheduled(theirs)) => {
                serde_json::to_value(ours).ok() == serde_json::to_value(theirs).ok()
            }
            (Dispatched::Group(ours), Dispatched::Group(theirs)) => ours == theirs,
            (Dispatched::Timer(ours), Dispatched::Timer(theirs)) => ours == theirs,
            (Dispatched::Tasks(ours), Dispatched::Tasks(theirs)) => ours == theirs,
            (Dispatched::Activity(ours), Dispatched::Activity(theirs)) => ours == theirs,
            (Dispatched::Refresh(ours), Dispatched::Refresh(theirs)) => ours == theirs,
            (Dispatched::Package(ours), Dispatched::Package(theirs)) => ours == theirs,
            (
                Dispatched::UserUpdated {
                    administrator: ours,
                    preference_access: our_preferences,
                },
                Dispatched::UserUpdated {
                    administrator: theirs,
                    preference_access: their_preferences,
                },
            ) => ours == theirs && our_preferences == their_preferences,
            (Dispatched::Ignored, Dispatched::Ignored) => true,
            (
                Dispatched::Broadcast(_)
                | Dispatched::Marked(_)
                | Dispatched::Command(_)
                | Dispatched::Sessions(_)
                | Dispatched::KeepAlive(_)
                | Dispatched::Scheduled(_)
                | Dispatched::Group(_)
                | Dispatched::Timer(_)
                | Dispatched::Tasks(_)
                | Dispatched::Library(_)
                | Dispatched::Activity(_)
                | Dispatched::Refresh(_)
                | Dispatched::Package(_)
                | Dispatched::UserUpdated { .. }
                | Dispatched::Ignored,
                _,
            ) => false,
        }
    }
}

fn typed<T: serde::de::DeserializeOwned>(frame: &str) -> Option<T> {
    serde_json::from_str(frame).ok()
}

/// One scheduled task as the dashboard takes it; a task the server named no id
/// for is dropped.
pub fn task_state(info: jellyfin_api::types::TaskInfo) -> Option<jellium_protocol::TaskState> {
    use jellyfin_api::types::TaskState as Upstream;
    Some(jellium_protocol::TaskState {
        id: info.id?,
        name: info.name.unwrap_or_default(),
        category: info.category.unwrap_or_default(),
        description: info.description.unwrap_or_default(),
        state: match info.state {
            Some(Upstream::Cancelling) => jellium_protocol::TaskRunState::Cancelling,
            Some(Upstream::Running) => jellium_protocol::TaskRunState::Running,
            Some(Upstream::Idle) | None => jellium_protocol::TaskRunState::Idle,
        },
        progress: info.current_progress_percentage,
    })
}

/// One activity entry as the activity screen takes it; an entry the server
/// named no id for is dropped.
pub fn activity_entry(
    entry: jellyfin_api::types::ActivityLogEntry,
) -> Option<jellium_protocol::ActivityEntry> {
    Some(jellium_protocol::ActivityEntry {
        id: entry.id?,
        name: entry.name.unwrap_or_default(),
        overview: entry.short_overview.or(entry.overview).unwrap_or_default(),
        kind: entry.type_.unwrap_or_default(),
        severity: entry
            .severity
            .map(|severity| severity.to_string())
            .unwrap_or_default(),
        user: entry.user_id.filter(|user| !user.is_nil()),
        user_name: String::new(),
        at: entry.date.map(|at| at.timestamp_millis()).unwrap_or(0),
    })
}

/// The refresh progress one `RefreshProgress` body carries; a body naming no
/// item, or no progress that reads as a number, carries none.
fn refreshed(
    data: std::collections::HashMap<String, Option<String>>,
) -> Option<jellium_protocol::Refreshed> {
    let item = data
        .get("ItemId")
        .and_then(|held| held.as_deref())
        .and_then(|held| held.parse::<uuid::Uuid>().ok())?;
    let progress = data
        .get("Progress")
        .and_then(|held| held.as_deref())
        .and_then(|held| held.parse::<f64>().ok())?;
    Some(jellium_protocol::Refreshed { item, progress })
}

/// The event one of the five package messages becomes.
fn packaged(named: &str, info: jellyfin_api::types::InstallationInfo) -> Event {
    let package = jellium_protocol::Packaged {
        name: info.name.unwrap_or_default(),
        version: info.version.unwrap_or_default(),
        plugin: info.guid,
    };
    match named {
        "PluginInstalling" => Event::PackageInstalling { package },
        "PluginInstallationCompleted" => Event::PackageInstalled { package },
        "PluginInstallationFailed" => Event::PackageFailed { package },
        "PluginInstallationCancelled" => Event::PackageCancelled { package },
        _ => Event::PackageUninstalled { package },
    }
}

/// Reads `frame`'s `MessageType` and deserializes the message it names.
/// A frame that is not json, names no type, names one outside this milestone,
/// or carries a body its type does not fit reads as `Ignored`.
/// A `UserDataChanged` for another user, and a `UserDeleted` naming another
/// user, read as `Ignored`; `user` is the signed-in user.
/// `TimerCreated`, `TimerCancelled`, `SeriesTimerCreated` and
/// `SeriesTimerCancelled` dispatch as `Timer`; a body naming no timer id, and
/// any of the four while `live_tv` is false, read as `Ignored`.
pub fn dispatch(frame: &str, user: Uuid, live_tv: bool) -> Dispatched {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(frame) else {
        return Dispatched::Ignored;
    };
    let Some(named) = value.get("MessageType").and_then(serde_json::Value::as_str) else {
        return Dispatched::Ignored;
    };
    match named {
        "LibraryChanged" => {
            let held = value.get("Data");
            let ids = |key: &str| -> Vec<Uuid> {
                held.and_then(|held| held.get(key))
                    .and_then(serde_json::Value::as_array)
                    .map(|held| {
                        held.iter()
                            .filter_map(serde_json::Value::as_str)
                            .filter_map(|id| id.parse().ok())
                            .collect()
                    })
                    .unwrap_or_default()
            };
            let changed = Changed {
                added: ids("ItemsAdded"),
                removed: ids("ItemsRemoved"),
                updated: ids("ItemsUpdated"),
            };
            if changed.is_empty() {
                return Dispatched::Ignored;
            }
            Dispatched::Library(changed)
        }
        "UserDataChanged" => {
            let Some(message) = typed::<jellyfin_api::types::UserDataChangedMessage>(frame) else {
                return Dispatched::Ignored;
            };
            let Some(data) = message.data else {
                return Dispatched::Ignored;
            };
            if data.user_id != Some(user) {
                return Dispatched::Ignored;
            }
            let items = data
                .user_data_list
                .into_iter()
                .filter_map(|entry| {
                    Some(Marked {
                        item: entry.item_id?,
                        played: entry.played.unwrap_or(false),
                        favorite: entry.is_favorite.unwrap_or(false),
                        play_count: entry.play_count.unwrap_or(0),
                        position_ticks: entry.playback_position_ticks.unwrap_or(0),
                    })
                })
                .collect::<Vec<_>>();
            if items.is_empty() {
                Dispatched::Ignored
            } else {
                Dispatched::Marked(items)
            }
        }
        "UserDeleted" => match typed::<jellyfin_api::types::UserDeletedMessage>(frame) {
            Some(message) if message.data == Some(user) => {
                Dispatched::Broadcast(Event::UserDeleted)
            }
            _ => Dispatched::Ignored,
        },
        "ServerRestarting" => Dispatched::Broadcast(Event::ServerStopping { restarting: true }),
        "ServerShuttingDown" => Dispatched::Broadcast(Event::ServerStopping { restarting: false }),
        "ForceKeepAlive" => match typed::<jellyfin_api::types::ForceKeepAliveMessage>(frame) {
            Some(message) => match message.data {
                Some(seconds) if seconds > 0 => {
                    Dispatched::KeepAlive(Duration::from_secs(seconds as u64))
                }
                _ => Dispatched::Ignored,
            },
            None => Dispatched::Ignored,
        },
        "Sessions" => match typed::<jellyfin_api::types::SessionsMessage>(frame) {
            Some(message) => Dispatched::Sessions(message.data.unwrap_or_default()),
            None => Dispatched::Ignored,
        },
        "TimerCreated" | "TimerCancelled" | "SeriesTimerCreated" | "SeriesTimerCancelled" => {
            if !live_tv {
                return Dispatched::Ignored;
            }
            let change = match named {
                "TimerCreated" => jellium_protocol::TimerChange::Created,
                "TimerCancelled" => jellium_protocol::TimerChange::Cancelled,
                "SeriesTimerCreated" => jellium_protocol::TimerChange::SeriesCreated,
                _ => jellium_protocol::TimerChange::SeriesCancelled,
            };
            let Some(data) = value
                .get("Data")
                .and_then(|data| {
                    serde_json::from_value::<jellyfin_api::types::TimerEventInfo>(data.clone()).ok()
                })
                .filter(|data| data.id.is_some())
            else {
                return Dispatched::Ignored;
            };
            Dispatched::Timer(jellium_protocol::TimerChanged {
                change,
                timer: data.id.unwrap_or_default(),
                program: data.program_id,
            })
        }
        "GeneralCommand" => typed::<jellyfin_api::types::GeneralCommandMessage>(frame)
            .and_then(|message| verbs::general(&message.data?, live_tv))
            .map_or(Dispatched::Ignored, Dispatched::Command),
        "Play" => typed::<jellyfin_api::types::PlayMessage>(frame)
            .and_then(|message| verbs::play(&message.data?))
            .map_or(Dispatched::Ignored, Dispatched::Command),
        "SyncPlayCommand" => match value.get("Data") {
            Some(data) => serde_json::from_value(data.clone())
                .map_or(Dispatched::Ignored, Dispatched::Scheduled),
            None => Dispatched::Ignored,
        },
        "SyncPlayGroupUpdate" => match value.get("Data") {
            Some(data) => group::update(data).map_or(Dispatched::Ignored, Dispatched::Group),
            None => Dispatched::Ignored,
        },
        "ScheduledTasksInfo" => {
            match typed::<jellyfin_api::types::ScheduledTasksInfoMessage>(frame) {
                Some(message) => Dispatched::Tasks(
                    message
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(task_state)
                        .collect(),
                ),
                None => Dispatched::Ignored,
            }
        }
        "ActivityLogEntry" => match value.get("Data") {
            Some(data) => {
                serde_json::from_value::<Vec<jellyfin_api::types::ActivityLogEntry>>(data.clone())
                    .ok()
                    .and_then(|entries| entries.into_iter().next())
                    .and_then(activity_entry)
                    .map_or(Dispatched::Ignored, Dispatched::Activity)
            }
            None => Dispatched::Ignored,
        },
        "RefreshProgress" => match typed::<jellyfin_api::types::RefreshProgressMessage>(frame) {
            Some(message) => refreshed(message.data.unwrap_or_default())
                .map_or(Dispatched::Ignored, Dispatched::Refresh),
            None => Dispatched::Ignored,
        },
        "PluginInstalling"
        | "PluginInstallationCompleted"
        | "PluginInstallationFailed"
        | "PluginInstallationCancelled"
        | "PluginUninstalled" => match value.get("Data") {
            Some(data) => {
                match serde_json::from_value::<jellyfin_api::types::InstallationInfo>(data.clone())
                {
                    Ok(info) => Dispatched::Package(packaged(named, info)),
                    Err(_) => Dispatched::Ignored,
                }
            }
            None => Dispatched::Ignored,
        },
        "RestartRequired" => Dispatched::Broadcast(Event::RestartRequired),
        "UserUpdated" => match typed::<jellyfin_api::types::UserUpdatedMessage>(frame) {
            Some(message) => match message.data {
                Some(dto) if dto.id == Some(user) => Dispatched::UserUpdated {
                    administrator: dto
                        .policy
                        .as_ref()
                        .and_then(|policy| policy.is_administrator)
                        .unwrap_or(false),
                    preference_access: dto
                        .policy
                        .as_ref()
                        .and_then(|policy| policy.enable_user_preference_access)
                        .unwrap_or(false),
                },
                _ => Dispatched::Ignored,
            },
            None => Dispatched::Ignored,
        },
        "Playstate" => typed::<jellyfin_api::types::PlaystateMessage>(frame)
            .and_then(|message| verbs::playstate(&message.data?))
            .map_or(Dispatched::Ignored, Dispatched::Command),
        _ => Dispatched::Ignored,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> Uuid {
        Uuid::from_u128(1)
    }

    #[test]
    fn a_library_change_dispatches_as_the_ids_it_named() {
        let frame = format!(
            r#"{{"MessageType":"LibraryChanged","Data":{{"ItemsAdded":["{}"],"ItemsUpdated":["{}"]}}}}"#,
            Uuid::from_u128(7),
            Uuid::from_u128(8),
        );
        assert_eq!(
            dispatch(&frame, user(), true),
            Dispatched::Library(Changed {
                added: vec![Uuid::from_u128(7)],
                removed: Vec::new(),
                updated: vec![Uuid::from_u128(8)],
            })
        );
    }

    #[test]
    fn a_library_change_naming_no_id_is_ignored() {
        for frame in [
            r#"{"MessageType":"LibraryChanged","Data":{}}"#,
            r#"{"MessageType":"LibraryChanged","Data":{"ItemsAdded":[]}}"#,
        ] {
            assert_eq!(
                dispatch(frame, user(), true),
                Dispatched::Ignored,
                "{frame}"
            );
        }
    }

    #[test]
    fn one_window_absorbs_every_id_without_repeating_one() {
        let mut held = Changed {
            added: vec![Uuid::from_u128(1)],
            ..Changed::default()
        };
        held.absorb(Changed {
            added: vec![Uuid::from_u128(1), Uuid::from_u128(2)],
            updated: vec![Uuid::from_u128(3)],
            ..Changed::default()
        });
        assert_eq!(held.added, vec![Uuid::from_u128(1), Uuid::from_u128(2)]);
        assert_eq!(held.updated, vec![Uuid::from_u128(3)]);
        assert!(held.removed.is_empty());
    }

    #[test]
    fn a_message_type_outside_this_milestone_is_ignored() {
        let frame = r#"{"MessageType":"SomethingElse","Data":{}}"#;
        assert_eq!(dispatch(frame, user(), true), Dispatched::Ignored);
    }

    #[test]
    fn a_user_data_change_for_another_user_is_ignored() {
        let frame = format!(
            r#"{{"MessageType":"UserDataChanged","Data":{{"UserId":"{}","UserDataList":[{{"ItemId":"{}","Played":true}}]}}}}"#,
            Uuid::from_u128(2),
            Uuid::from_u128(9)
        );
        assert_eq!(dispatch(&frame, user(), true), Dispatched::Ignored);
    }

    #[test]
    fn a_user_data_change_for_the_signed_in_user_is_marked() {
        let frame = format!(
            r#"{{"MessageType":"UserDataChanged","Data":{{"UserId":"{}","UserDataList":[{{"ItemId":"{}","Played":true,"IsFavorite":true,"PlayCount":3,"PlaybackPositionTicks":42}}]}}}}"#,
            user(),
            Uuid::from_u128(9)
        );
        assert_eq!(
            dispatch(&frame, user(), true),
            Dispatched::Marked(vec![Marked {
                item: Uuid::from_u128(9),
                played: true,
                favorite: true,
                play_count: 3,
                position_ticks: 42,
            }])
        );
    }

    #[test]
    fn a_user_deleted_naming_the_signed_in_user_is_broadcast() {
        let frame = format!(r#"{{"MessageType":"UserDeleted","Data":"{}"}}"#, user());
        assert_eq!(
            dispatch(&frame, user(), true),
            Dispatched::Broadcast(Event::UserDeleted)
        );
        let other = format!(
            r#"{{"MessageType":"UserDeleted","Data":"{}"}}"#,
            Uuid::from_u128(2)
        );
        assert_eq!(dispatch(&other, user(), true), Dispatched::Ignored);
    }

    #[test]
    fn a_server_restarting_message_is_told_apart_from_a_shutdown() {
        assert_eq!(
            dispatch(r#"{"MessageType":"ServerRestarting"}"#, user(), true),
            Dispatched::Broadcast(Event::ServerStopping { restarting: true })
        );
        assert_eq!(
            dispatch(r#"{"MessageType":"ServerShuttingDown"}"#, user(), true),
            Dispatched::Broadcast(Event::ServerStopping { restarting: false })
        );
    }

    #[test]
    fn a_frame_that_is_not_json_is_ignored() {
        for frame in ["", "not json", "[]", r#"{"Data":{}}"#, "null"] {
            assert_eq!(
                dispatch(frame, user(), true),
                Dispatched::Ignored,
                "{frame}"
            );
        }
    }

    #[test]
    fn a_body_its_type_does_not_fit_is_ignored() {
        assert_eq!(
            dispatch(
                r#"{"MessageType":"Sessions","Data":"not a list"}"#,
                user(),
                true
            ),
            Dispatched::Ignored
        );
        assert_eq!(
            dispatch(
                r#"{"MessageType":"UserDataChanged","Data":7}"#,
                user(),
                true
            ),
            Dispatched::Ignored
        );
    }

    #[test]
    fn a_keep_alive_names_the_interval_the_server_asked_for() {
        assert_eq!(
            dispatch(
                r#"{"MessageType":"ForceKeepAlive","Data":60}"#,
                user(),
                true
            ),
            Dispatched::KeepAlive(Duration::from_secs(60))
        );
    }

    #[test]
    fn a_playstate_message_reaches_the_destination_tab_as_a_control() {
        assert_eq!(
            dispatch(
                r#"{"MessageType":"Playstate","Data":{"Command":"PlayPause"}}"#,
                user(),
                true
            ),
            Dispatched::Command(Control::PlayPause)
        );
    }

    #[test]
    fn a_sync_play_command_dispatches_as_a_scheduled_command() {
        let frame = r#"{"MessageType":"SyncPlayCommand","Data":{"Command":"Unpause","PositionTicks":42,"When":"2026-01-01T00:00:00Z"}}"#;
        let Dispatched::Scheduled(command) = dispatch(frame, user(), true) else {
            panic!("{frame} did not dispatch as a scheduled command");
        };
        assert_eq!(
            command.command,
            Some(jellyfin_api::types::SendCommandType::Unpause)
        );
        assert_eq!(command.position_ticks, Some(42));
        assert!(command.when.is_some());
    }

    #[test]
    fn a_sync_play_group_update_dispatches_as_the_update_its_type_names() {
        let group = Uuid::from_u128(5);
        let frame = format!(
            r#"{{"MessageType":"SyncPlayGroupUpdate","Data":{{"GroupId":"{group}","Type":"GroupJoined","Data":{{"GroupId":"{group}","GroupName":"Group","Participants":["Ada"],"State":"Idle"}}}}}}"#
        );
        assert_eq!(
            dispatch(&frame, user(), true),
            Dispatched::Group(group::Update::Joined(jellium_protocol::Group {
                id: group,
                name: "Group".to_owned(),
                participants: vec!["Ada".to_owned()],
                state: jellium_protocol::GroupState::Idle,
            }))
        );
    }

    #[test]
    fn an_unrecognised_group_update_dispatches_as_ignored() {
        for frame in [
            r#"{"MessageType":"SyncPlayGroupUpdate","Data":{"Type":"SomethingElse"}}"#,
            r#"{"MessageType":"SyncPlayGroupUpdate","Data":{}}"#,
            r#"{"MessageType":"SyncPlayGroupUpdate"}"#,
            r#"{"MessageType":"SyncPlayGroupUpdate","Data":{"Type":"GroupJoined","Data":"nope"}}"#,
        ] {
            assert_eq!(
                dispatch(frame, user(), true),
                Dispatched::Ignored,
                "{frame}"
            );
        }
    }

    #[test]
    fn no_dispatched_group_event_carries_an_upstream_instant() {
        let group = Uuid::from_u128(5);
        let frame = format!(
            r#"{{"MessageType":"SyncPlayGroupUpdate","Data":{{"GroupId":"{group}","Type":"GroupJoined","Data":{{"GroupId":"{group}","GroupName":"Group","LastUpdatedAt":"2026-01-01T00:00:00Z","State":"Idle"}}}}}}"#
        );
        let Dispatched::Group(group::Update::Joined(joined)) = dispatch(&frame, user(), true)
        else {
            panic!("{frame} did not dispatch as a join");
        };
        let event = serde_json::to_string(&jellium_protocol::Event::Joined {
            group: joined,
            member: true,
        })
        .expect("the event serializes");
        assert!(!event.contains("2026-01-01"), "{event} carries an instant");
    }

    fn timer_frame(named: &str, id: &str, program: Uuid) -> String {
        format!(r#"{{"MessageType":"{named}","Data":{{"Id":"{id}","ProgramId":"{program}"}}}}"#)
    }

    #[test]
    fn each_of_the_four_timer_messages_dispatches_as_its_change() {
        use jellium_protocol::TimerChange;
        let program = Uuid::from_u128(7);
        for (named, change) in [
            ("TimerCreated", TimerChange::Created),
            ("TimerCancelled", TimerChange::Cancelled),
            ("SeriesTimerCreated", TimerChange::SeriesCreated),
            ("SeriesTimerCancelled", TimerChange::SeriesCancelled),
        ] {
            assert_eq!(
                dispatch(&timer_frame(named, "timer-1", program), user(), true),
                Dispatched::Timer(jellium_protocol::TimerChanged {
                    change,
                    timer: "timer-1".to_owned(),
                    program: Some(program),
                }),
                "{named}"
            );
        }
    }

    #[test]
    fn a_timer_message_naming_no_timer_id_is_ignored() {
        for frame in [
            r#"{"MessageType":"TimerCreated","Data":{}}"#,
            r#"{"MessageType":"TimerCancelled","Data":{"ProgramId":"00000000-0000-0000-0000-000000000007"}}"#,
            r#"{"MessageType":"SeriesTimerCreated"}"#,
        ] {
            assert_eq!(
                dispatch(frame, user(), true),
                Dispatched::Ignored,
                "{frame}"
            );
        }
    }

    #[test]
    fn a_timer_message_without_live_tv_is_ignored() {
        let program = Uuid::from_u128(7);
        for named in [
            "TimerCreated",
            "TimerCancelled",
            "SeriesTimerCreated",
            "SeriesTimerCancelled",
        ] {
            assert_eq!(
                dispatch(&timer_frame(named, "timer-1", program), user(), false),
                Dispatched::Ignored,
                "{named}"
            );
        }
    }

    #[test]
    fn no_dispatched_timer_event_carries_an_upstream_url_or_a_token() {
        let Dispatched::Timer(changed) = dispatch(
            &timer_frame("TimerCreated", "timer-1", Uuid::from_u128(7)),
            user(),
            true,
        ) else {
            panic!("a timer message dispatches as a timer change");
        };
        let event = serde_json::to_string(&Event::Timers {
            changes: vec![changed],
        })
        .expect("the event serializes");
        for secret in ["token", "api_key", "http://", "https://", "Authorization"] {
            assert!(!event.contains(secret), "{event} carries {secret}");
        }
    }

    #[test]
    fn no_dispatched_event_carries_an_upstream_url_or_a_token() {
        let frames = [
            format!(
                r#"{{"MessageType":"UserDataChanged","Data":{{"UserId":"{}","UserDataList":[{{"ItemId":"{}","Played":true}}]}}}}"#,
                user(),
                Uuid::from_u128(9)
            ),
            r#"{"MessageType":"ServerRestarting"}"#.to_owned(),
            format!(r#"{{"MessageType":"UserDeleted","Data":"{}"}}"#, user()),
            r#"{"MessageType":"Playstate","Data":{"Command":"Seek","SeekPositionTicks":5}}"#
                .to_owned(),
            r#"{"MessageType":"GeneralCommand","Data":{"Name":"DisplayMessage","Arguments":{"Header":"H","Text":"T"}}}"#
                .to_owned(),
        ];
        for frame in frames {
            let event = match dispatch(&frame, user(), true) {
                Dispatched::Broadcast(event) => serde_json::to_string(&event),
                Dispatched::Marked(items) => serde_json::to_string(&Event::Marked { items }),
                Dispatched::Command(control) => serde_json::to_string(&Event::Control(control)),
                other => panic!("{frame} dispatched as {other:?}"),
            }
            .expect("the event serializes");
            for secret in ["token", "api_key", "http://", "https://", "Authorization"] {
                assert!(!event.contains(secret), "{event} carries {secret}");
            }
        }
    }
}
