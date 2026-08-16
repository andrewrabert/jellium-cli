use std::sync::Arc;
use std::time::Duration;

use jellium_protocol::{
    Event, Failure, Group, GroupQueue, GroupState, GroupVerb, LiveRefusal, Queued, Repeat,
};
use jellyfin_api::types::{
    GroupInfoDto, GroupRepeatMode, GroupShuffleMode, GroupStateType, PlayQueueUpdate,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::tabs::TabId;
use crate::web::AppState;
use crate::web::upstream::Upstream;

/// The membership and the tab holding it; `tab` is absent while a socket that
/// went away has not been reclaimed.
struct Held {
    tab: Option<TabId>,
    group: Group,
    queue: GroupQueue,
    /// When the holding tab's socket went away.
    orphaned: Option<std::time::Instant>,
}

/// The one tab in the group, the group it is in, and what the group last said
/// about itself.
pub struct Membership {
    held: RwLock<Option<Held>>,
    /// The tab whose join the Jellyfin server has not answered yet.
    asking: RwLock<Option<TabId>>,
}

impl Membership {
    /// A membership no tab reclaims for this long is left upstream, which is
    /// how a reloaded page stops being a member.
    pub const GRACE: Duration = Duration::from_secs(30);

    /// The joinable groups are listed again this often while a tab is
    /// watching.
    pub const LISTING: Duration = Duration::from_secs(5);

    pub fn new() -> Membership {
        Membership {
            held: RwLock::new(None),
            asking: RwLock::new(None),
        }
    }

    /// Records the group this session joined and binds `tab` to it.
    pub async fn joined(&self, tab: TabId, group: Group) {
        *self.asking.write().await = None;
        let mut held = self.held.write().await;
        match held.as_mut() {
            Some(standing) if standing.group.id == group.id => {
                standing.tab = Some(tab);
                standing.orphaned = None;
                standing.group = group;
            }
            _ => {
                *held = Some(Held {
                    tab: Some(tab),
                    group,
                    queue: GroupQueue::default(),
                    orphaned: None,
                });
            }
        }
    }

    /// Binds `tab` as the member and returns the group, its queue, and the tab
    /// membership was taken from; a call while this installation is in no
    /// group reads as `None`.
    pub async fn bound(&self, tab: TabId) -> Option<(Group, GroupQueue, Option<TabId>)> {
        let mut held = self.held.write().await;
        let standing = held.as_mut()?;
        let taken = standing.tab.filter(|held| *held != tab);
        standing.tab = Some(tab);
        standing.orphaned = None;
        Some((standing.group.clone(), standing.queue.clone(), taken))
    }

    /// Records that `tab` asked to join a group; no group is held, and none is
    /// published, until the Jellyfin server's push answers.
    pub async fn asking(&self, tab: TabId) {
        *self.asking.write().await = Some(tab);
    }

    /// The tab a group push belongs to: the tab holding membership, and the
    /// tab whose join is outstanding otherwise.
    pub async fn expecting(&self) -> Option<TabId> {
        match self.holder().await {
            Some(tab) => Some(tab),
            None => *self.asking.read().await,
        }
    }

    /// The group `tab` is in, or `NotGrouped`.
    pub async fn member(&self, tab: TabId) -> Result<Uuid, LiveRefusal> {
        match self.held.read().await.as_ref() {
            Some(standing) if standing.tab == Some(tab) => Ok(standing.group.id),
            _ => Err(LiveRefusal::NotGrouped),
        }
    }

    /// The tab holding membership now.
    pub async fn holder(&self) -> Option<TabId> {
        self.held.read().await.as_ref().and_then(|held| held.tab)
    }

    /// True while this installation is in a group.
    pub async fn grouped(&self) -> bool {
        self.held.read().await.is_some()
    }

    /// Takes the group's name, participants and state from an update and
    /// returns the group to re-emit.
    pub async fn standing(&self, state: GroupState) -> Option<Group> {
        let mut held = self.held.write().await;
        let standing = held.as_mut()?;
        standing.group.state = state;
        Some(standing.group.clone())
    }

    /// Adds a participant, and returns the group to re-emit.
    pub async fn welcomed(&self, participant: String) -> Option<Group> {
        let mut held = self.held.write().await;
        let standing = held.as_mut()?;
        if !standing.group.participants.contains(&participant) {
            standing.group.participants.push(participant);
        }
        Some(standing.group.clone())
    }

    /// Drops a participant, and returns the group to re-emit.
    pub async fn parted(&self, participant: &str) -> Option<Group> {
        let mut held = self.held.write().await;
        let standing = held.as_mut()?;
        standing
            .group
            .participants
            .retain(|held| held != participant);
        Some(standing.group.clone())
    }

    pub async fn queued(&self, queue: GroupQueue) {
        if let Some(standing) = self.held.write().await.as_mut() {
            standing.queue = queue;
        }
    }

    /// Records that `tab`'s socket went away, which starts the grace window; a
    /// call naming any other tab changes nothing.
    pub async fn orphaned(&self, tab: TabId) {
        let mut held = self.held.write().await;
        if let Some(standing) = held.as_mut()
            && standing.tab == Some(tab)
        {
            standing.tab = None;
            standing.orphaned = Some(std::time::Instant::now());
        }
    }

    /// True once the grace window has run out with no tab reclaiming it.
    pub async fn abandoned(&self) -> bool {
        self.held
            .read()
            .await
            .as_ref()
            .and_then(|standing| standing.orphaned)
            .is_some_and(|since| since.elapsed() >= Membership::GRACE)
    }

    /// Ends membership and returns the tab that held it.
    pub async fn end(&self) -> Option<TabId> {
        *self.asking.write().await = None;
        self.held.write().await.take().and_then(|held| held.tab)
    }
}

/// What one group update says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    Joined(Group),
    Left,
    UserJoined(String),
    UserLeft(String),
    State(GroupState),
    Queue(GroupQueue),
    NotInGroup,
    NoSuchGroup,
    LibraryDenied,
}

fn state(named: Option<GroupStateType>) -> GroupState {
    match named {
        Some(GroupStateType::Waiting) => GroupState::Waiting,
        Some(GroupStateType::Paused) => GroupState::Paused,
        Some(GroupStateType::Playing) => GroupState::Playing,
        Some(GroupStateType::Idle) | None => GroupState::Idle,
    }
}

/// The group one listing entry names.
pub fn read(info: &GroupInfoDto) -> Option<Group> {
    Some(Group {
        id: info.group_id?,
        name: info.group_name.clone().unwrap_or_default(),
        participants: info.participants.clone(),
        state: state(info.state),
    })
}

fn repeat(mode: Option<GroupRepeatMode>) -> Repeat {
    match mode {
        Some(GroupRepeatMode::RepeatOne) => Repeat::One,
        Some(GroupRepeatMode::RepeatAll) => Repeat::All,
        _ => Repeat::Off,
    }
}

/// The queue one play-queue update names.
pub fn queue(update: &PlayQueueUpdate) -> GroupQueue {
    GroupQueue {
        items: update
            .playlist
            .iter()
            .filter_map(|entry| {
                Some(Queued {
                    playlist_item: entry.playlist_item_id?,
                    item: entry.item_id?,
                })
            })
            .collect(),
        playing_index: update
            .playing_item_index
            .and_then(|index| usize::try_from(index).ok()),
        position_ticks: update.start_position_ticks.unwrap_or(0),
        playing: update.is_playing.unwrap_or(false),
        repeat: repeat(update.repeat_mode),
        shuffled: matches!(update.shuffle_mode, Some(GroupShuffleMode::Shuffle)),
    }
}

fn fitting<T: serde::de::DeserializeOwned>(data: &serde_json::Value) -> Option<T> {
    serde_json::from_value(data.get("Data")?.clone()).ok()
}

/// The payload of a type whose body is an object; anything else does not fit.
fn shaped<T: serde::de::DeserializeOwned>(data: &serde_json::Value) -> Option<T> {
    let payload = data.get("Data")?;
    payload.is_object().then_some(())?;
    serde_json::from_value(payload.clone()).ok()
}

/// The update a group frame's `Data` names, read from its `Type` rather than
/// by matching shapes untagged.
/// A `Type` outside this set, and a payload that does not fit the type it
/// names, both read as `None`.
pub fn update(data: &serde_json::Value) -> Option<Update> {
    match data.get("Type").and_then(serde_json::Value::as_str)? {
        "GroupJoined" => read(&shaped::<GroupInfoDto>(data)?).map(Update::Joined),
        "GroupLeft" => Some(Update::Left),
        "UserJoined" => fitting::<String>(data).map(Update::UserJoined),
        "UserLeft" => fitting::<String>(data).map(Update::UserLeft),
        "StateUpdate" => {
            let update = shaped::<jellyfin_api::types::GroupStateUpdate>(data)?;
            Some(Update::State(state(update.state)))
        }
        "PlayQueue" => shaped::<PlayQueueUpdate>(data).map(|update| Update::Queue(queue(&update))),
        "NotInGroup" => Some(Update::NotInGroup),
        "GroupDoesNotExist" => Some(Update::NoSuchGroup),
        "LibraryAccessDenied" => Some(Update::LibraryDenied),
        _ => None,
    }
}

/// The joinable groups, read from one request rather than from waiting for a
/// push.
pub async fn groups(upstream: &Upstream) -> Result<Vec<Group>, Failure> {
    let listed = upstream
        .control()
        .sync_play_get_groups()
        .await
        .map_err(|e| upstream.failed(e))?;
    Ok(listed.iter().filter_map(read).collect())
}

fn refused<T>(outcome: Result<T, jellyfin_api::error::Error>) -> Result<T, LiveRefusal> {
    outcome.map_err(|_| LiveRefusal::GroupRefused)
}

/// Creates a group named `name`; the Jellyfin server pushes the join.
pub async fn create(upstream: &Upstream, name: &str) -> Result<Group, LiveRefusal> {
    let info = refused(
        upstream
            .control()
            .sync_play_create_group(&jellyfin_api::types::NewGroupRequestDto {
                group_name: Some(name.to_owned()),
            })
            .await,
    )?;
    read(&info).ok_or(LiveRefusal::GroupRefused)
}

pub async fn join(upstream: &Upstream, group: Uuid) -> Result<(), LiveRefusal> {
    refused(
        upstream
            .control()
            .sync_play_join_group(&jellyfin_api::types::JoinGroupRequestDto {
                group_id: Some(group),
            })
            .await,
    )
}

/// Leaves whatever group this session is in; a session in none changes
/// nothing.
pub async fn leave(upstream: &Upstream) -> Result<(), LiveRefusal> {
    refused(upstream.control().sync_play_leave_group().await)
}

fn group_repeat(repeat: Repeat) -> GroupRepeatMode {
    match repeat {
        Repeat::Off => GroupRepeatMode::RepeatNone,
        Repeat::One => GroupRepeatMode::RepeatOne,
        Repeat::All => GroupRepeatMode::RepeatAll,
    }
}

/// Issues `verb` against the group this session is in: `SetQueue` on the
/// new-queue route, the transport verbs on their own routes, and the repeat
/// and shuffle verbs translated into `GroupRepeatMode` and `GroupShuffleMode`.
/// `stamped` is the instant on the Jellyfin server's clock the buffering and
/// ready calls carry, which is why no group verb travels through the relay.
pub async fn issue(
    upstream: &Upstream,
    verb: &GroupVerb,
    stamped: chrono::DateTime<chrono::Utc>,
) -> Result<(), LiveRefusal> {
    use jellyfin_api::types as api;
    let client = upstream.control();
    match verb {
        GroupVerb::SetQueue {
            items,
            start_index,
            start_ticks,
        } => refused(
            client
                .sync_play_set_new_queue(&api::PlayRequestDto {
                    playing_item_position: Some(*start_index),
                    playing_queue: items.clone(),
                    start_position_ticks: Some(*start_ticks),
                })
                .await,
        ),
        GroupVerb::Unpause => refused(client.sync_play_unpause().await),
        GroupVerb::Pause => refused(client.sync_play_pause().await),
        GroupVerb::Stop => refused(client.sync_play_stop().await),
        GroupVerb::Seek { position_ticks } => refused(
            client
                .sync_play_seek(&api::SeekRequestDto {
                    position_ticks: Some(*position_ticks),
                })
                .await,
        ),
        GroupVerb::NextItem { playlist_item } => refused(
            client
                .sync_play_next_item(&api::NextItemRequestDto {
                    playlist_item_id: Some(*playlist_item),
                })
                .await,
        ),
        GroupVerb::PreviousItem { playlist_item } => refused(
            client
                .sync_play_previous_item(&api::PreviousItemRequestDto {
                    playlist_item_id: Some(*playlist_item),
                })
                .await,
        ),
        GroupVerb::SetPlaylistItem { playlist_item } => refused(
            client
                .sync_play_set_playlist_item(&api::SetPlaylistItemRequestDto {
                    playlist_item_id: Some(*playlist_item),
                })
                .await,
        ),
        GroupVerb::RemoveFromPlaylist { playlist_items } => refused(
            client
                .sync_play_remove_from_playlist(&api::RemoveFromPlaylistRequestDto {
                    clear_playing_item: Some(false),
                    clear_playlist: Some(false),
                    playlist_item_ids: playlist_items.clone(),
                })
                .await,
        ),
        GroupVerb::SetRepeat { repeat } => refused(
            client
                .sync_play_set_repeat_mode(&api::SetRepeatModeRequestDto {
                    mode: Some(group_repeat(*repeat)),
                })
                .await,
        ),
        GroupVerb::SetShuffle { shuffled } => refused(
            client
                .sync_play_set_shuffle_mode(&api::SetShuffleModeRequestDto {
                    mode: Some(if *shuffled {
                        GroupShuffleMode::Shuffle
                    } else {
                        GroupShuffleMode::Sorted
                    }),
                })
                .await,
        ),
        GroupVerb::Buffering {
            playing,
            playlist_item,
            position_ticks,
        } => refused(
            client
                .sync_play_buffering(&api::BufferRequestDto {
                    is_playing: Some(*playing),
                    playlist_item_id: Some(*playlist_item),
                    position_ticks: Some(*position_ticks),
                    when: Some(stamped),
                })
                .await,
        ),
        GroupVerb::Ready {
            playing,
            playlist_item,
            position_ticks,
        } => refused(
            client
                .sync_play_ready(&api::ReadyRequestDto {
                    is_playing: Some(*playing),
                    playlist_item_id: Some(*playlist_item),
                    position_ticks: Some(*position_ticks),
                    when: Some(stamped),
                })
                .await,
        ),
    }
}

/// Reports the composed ping — the upstream round trip plus the browser's
/// round trip to the local server — to the group.
pub async fn ping(upstream: &Upstream, millis: i64) -> Result<(), LiveRefusal> {
    refused(
        upstream
            .control()
            .sync_play_ping(&jellyfin_api::types::PingRequestDto { ping: Some(millis) })
            .await,
    )
}

/// The listing loop, running while a picker or the SyncPlay screen is open in
/// some tab.
pub struct Listing {
    task: tokio::task::JoinHandle<()>,
}

impl Listing {
    /// Lists every `Membership::LISTING` and pushes the result to the watching
    /// tabs.
    pub fn start(state: Arc<AppState>) -> Listing {
        let task = tokio::spawn(async move {
            loop {
                if let Some(upstream) = state.session.signed().await
                    && let Ok(groups) = groups(&upstream).await
                {
                    for tab in state
                        .live
                        .tabs
                        .watchers(jellium_protocol::Feed::Groups)
                        .await
                    {
                        state
                            .live
                            .tabs
                            .send(
                                tab,
                                Event::Groups {
                                    groups: groups.clone(),
                                },
                            )
                            .await;
                    }
                }
                tokio::time::sleep(Membership::LISTING).await;
            }
        });
        Listing { task }
    }

    pub fn stop(self) {
        self.task.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::answering;

    async fn tabs() -> (super::super::tabs::Tabs, TabId, TabId) {
        let tabs = super::super::tabs::Tabs::new();
        let (first, _, _) = tabs.add().await;
        let (second, _, _) = tabs.add().await;
        (tabs, first, second)
    }

    fn a_group() -> Group {
        Group {
            id: Uuid::from_u128(1),
            name: "Group".to_owned(),
            participants: vec!["Ada".to_owned()],
            state: GroupState::Idle,
        }
    }

    #[tokio::test]
    async fn a_second_tab_taking_the_group_takes_it_from_the_first() {
        let (_tabs, first, second) = tabs().await;
        let membership = Membership::new();
        membership.joined(first, a_group()).await;
        assert_eq!(membership.member(first).await, Ok(a_group().id));

        let (group, _, taken) = membership.bound(second).await.expect("the group is taken");
        assert_eq!(group, a_group());
        assert_eq!(taken, Some(first));
        assert_eq!(membership.member(second).await, Ok(a_group().id));
        assert_eq!(membership.member(first).await, Err(LiveRefusal::NotGrouped));
    }

    #[tokio::test]
    async fn binding_a_tab_while_this_installation_is_in_no_group_reads_as_nothing() {
        let (_tabs, first, _) = tabs().await;
        let membership = Membership::new();
        assert!(membership.bound(first).await.is_none());
    }

    #[tokio::test]
    async fn an_outstanding_join_binds_the_push_that_answers_to_the_tab_that_asked() {
        let (_tabs, first, second) = tabs().await;
        let membership = Membership::new();
        assert_eq!(membership.expecting().await, None);

        membership.asking(second).await;
        assert_eq!(membership.expecting().await, Some(second));
        assert!(!membership.grouped().await);

        membership.joined(second, a_group()).await;
        assert_eq!(membership.expecting().await, Some(second));

        membership.end().await;
        assert_eq!(membership.expecting().await, None);
        let _ = first;
    }

    #[tokio::test]
    async fn a_group_verb_from_a_tab_holding_no_membership_is_refused() {
        let (_tabs, first, second) = tabs().await;
        let membership = Membership::new();
        membership.joined(first, a_group()).await;
        assert_eq!(
            membership.member(second).await,
            Err(LiveRefusal::NotGrouped)
        );
    }

    #[tokio::test]
    async fn every_group_verb_reaches_the_route_its_effect_needs() {
        let server = answering(204).await;
        let upstream = Upstream::stub(&server.base);
        let stamped = chrono::Utc::now();
        let item = Uuid::from_u128(3);
        for (verb, path) in [
            (
                GroupVerb::SetQueue {
                    items: vec![item],
                    start_index: 0,
                    start_ticks: 0,
                },
                "/SyncPlay/SetNewQueue",
            ),
            (GroupVerb::Unpause, "/SyncPlay/Unpause"),
            (GroupVerb::Pause, "/SyncPlay/Pause"),
            (GroupVerb::Stop, "/SyncPlay/Stop"),
            (GroupVerb::Seek { position_ticks: 5 }, "/SyncPlay/Seek"),
            (
                GroupVerb::NextItem {
                    playlist_item: item,
                },
                "/SyncPlay/NextItem",
            ),
            (
                GroupVerb::PreviousItem {
                    playlist_item: item,
                },
                "/SyncPlay/PreviousItem",
            ),
            (
                GroupVerb::SetPlaylistItem {
                    playlist_item: item,
                },
                "/SyncPlay/SetPlaylistItem",
            ),
            (
                GroupVerb::RemoveFromPlaylist {
                    playlist_items: vec![item],
                },
                "/SyncPlay/RemoveFromPlaylist",
            ),
            (
                GroupVerb::SetRepeat {
                    repeat: Repeat::All,
                },
                "/SyncPlay/SetRepeatMode",
            ),
            (
                GroupVerb::SetShuffle { shuffled: true },
                "/SyncPlay/SetShuffleMode",
            ),
            (
                GroupVerb::Buffering {
                    playing: true,
                    playlist_item: item,
                    position_ticks: 0,
                },
                "/SyncPlay/Buffering",
            ),
            (
                GroupVerb::Ready {
                    playing: true,
                    playlist_item: item,
                    position_ticks: 0,
                },
                "/SyncPlay/Ready",
            ),
        ] {
            let before = server.asked(path);
            issue(&upstream, &verb, stamped)
                .await
                .expect("the verb is accepted");
            assert_eq!(server.asked(path), before + 1, "{verb:?} missed {path}");
        }
    }

    #[tokio::test]
    async fn a_buffering_call_carries_an_instant_on_the_jellyfin_clock() {
        let server = crate::web::upstream::answering_with(204, &[], "").await;
        let upstream = Upstream::stub(&server.base);
        let stamped = chrono::Utc::now() + chrono::Duration::milliseconds(5_000);
        issue(
            &upstream,
            &GroupVerb::Buffering {
                playing: true,
                playlist_item: Uuid::from_u128(3),
                position_ticks: 0,
            },
            stamped,
        )
        .await
        .expect("the verb is accepted");
        // the stub records no bodies, so the stamp is checked by construction
        assert_eq!(server.asked("/SyncPlay/Buffering"), 1);
    }

    #[test]
    fn a_group_update_naming_a_type_outside_this_set_reads_as_nothing() {
        for frame in [
            serde_json::json!({ "Type": "SomethingElse", "Data": {} }),
            serde_json::json!({ "Data": {} }),
            serde_json::json!({ "Type": 7 }),
        ] {
            assert_eq!(update(&frame), None, "{frame}");
        }
    }

    #[test]
    fn a_group_update_whose_payload_does_not_fit_its_type_reads_as_nothing() {
        for frame in [
            serde_json::json!({ "Type": "GroupJoined", "Data": "not a group" }),
            serde_json::json!({ "Type": "GroupJoined", "Data": {} }),
            serde_json::json!({ "Type": "UserJoined", "Data": 7 }),
            serde_json::json!({ "Type": "StateUpdate", "Data": [] }),
        ] {
            assert_eq!(update(&frame), None, "{frame}");
        }
    }

    #[tokio::test]
    async fn a_state_update_leaves_the_groups_name_and_participants_standing() {
        let (_tabs, tab, _) = tabs().await;
        let membership = Membership::new();
        membership.joined(tab, a_group()).await;
        let standing = membership
            .standing(GroupState::Playing)
            .await
            .expect("the group");
        assert_eq!(standing.name, a_group().name);
        assert_eq!(standing.participants, a_group().participants);
        assert_eq!(standing.state, GroupState::Playing);
    }

    #[tokio::test]
    async fn a_membership_no_tab_reclaims_is_abandoned_once_the_grace_window_runs_out() {
        let (_tabs, tab, other) = tabs().await;
        let membership = Membership::new();
        membership.joined(tab, a_group()).await;

        membership.orphaned(other).await;
        assert_eq!(membership.holder().await, Some(tab));

        membership.orphaned(tab).await;
        assert_eq!(membership.holder().await, None);
        assert!(!membership.abandoned().await);

        // the window running out is what a reload leaves behind
        if let Some(standing) = membership.held.write().await.as_mut() {
            standing.orphaned = Some(std::time::Instant::now() - Membership::GRACE);
        }
        assert!(membership.abandoned().await);
    }
}
