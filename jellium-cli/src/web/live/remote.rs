use std::collections::HashSet;

use jellium_protocol::{Drive, Failure, LiveRefusal, NowPlaying, PlayMode, Repeat, Target};
use jellyfin_api::types::{
    PlayCommand, PlaybackOrder, PlaystateCommand, RepeatMode, SessionInfoDto,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::tabs::TabId;
use super::verbs;
use crate::web::identity::Device;
use crate::web::upstream::Upstream;

/// The tab holding remote mode, and what it drives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    pub tab: TabId,
    pub target: String,
}

/// The one tab driving one target, and every target the local server has seen.
pub struct Remote {
    bound: RwLock<Option<Bound>>,
    /// The session ids of the newest listing for the current user.
    seen: RwLock<HashSet<String>>,
}

impl Remote {
    pub fn new() -> Remote {
        Remote {
            bound: RwLock::new(None),
            seen: RwLock::new(HashSet::new()),
        }
    }

    /// Replaces the addressable set with exactly the sessions `targets` names,
    /// so it holds one listing however long the run.
    pub async fn listed(&self, targets: &[Target]) {
        *self.seen.write().await = targets
            .iter()
            .map(|target| target.session.clone())
            .collect();
    }

    /// Binds `tab` to `target` and returns the tab it took the mode from.
    /// A target absent from the seen set is refused as `UnknownTarget`.
    pub async fn take(&self, tab: TabId, target: &str) -> Result<Option<TabId>, LiveRefusal> {
        if !self.seen.read().await.contains(target) {
            return Err(LiveRefusal::UnknownTarget);
        }
        let mut bound = self.bound.write().await;
        let taken = bound.take().filter(|held| held.tab != tab);
        *bound = Some(Bound {
            tab,
            target: target.to_owned(),
        });
        Ok(taken.map(|held| held.tab))
    }

    /// The target `tab` drives, or `NotDriving`.
    pub async fn driving(&self, tab: TabId) -> Result<String, LiveRefusal> {
        match self.bound.read().await.as_ref() {
            Some(held) if held.tab == tab => Ok(held.target.clone()),
            _ => Err(LiveRefusal::NotDriving),
        }
    }

    /// Ends the mode `tab` holds; a tab holding none changes nothing.
    pub async fn leave(&self, tab: TabId) {
        let mut bound = self.bound.write().await;
        if bound.as_ref().is_some_and(|held| held.tab == tab) {
            *bound = None;
        }
    }

    /// Ends the mode and returns the tab that held it.
    pub async fn end(&self) -> Option<Bound> {
        self.bound.write().await.take()
    }

    /// Ends the mode when its target is absent from `targets`, and returns the
    /// tab to tell.
    pub async fn checked(&self, targets: &[Target]) -> Option<TabId> {
        let mut bound = self.bound.write().await;
        let held = bound.as_ref()?;
        if targets.iter().any(|target| target.session == held.target) {
            return None;
        }
        bound.take().map(|held| held.tab)
    }

    /// Forgets every seen session and ends the mode, which is what a session
    /// change does.
    pub async fn forget(&self) -> Option<Bound> {
        self.seen.write().await.clear();
        self.bound.write().await.take()
    }
}

fn repeat(mode: Option<RepeatMode>) -> Repeat {
    match mode {
        Some(RepeatMode::RepeatOne) => Repeat::One,
        Some(RepeatMode::RepeatAll) => Repeat::All,
        _ => Repeat::Off,
    }
}

fn now_playing(session: &SessionInfoDto) -> Option<NowPlaying> {
    let item = session.now_playing_item.as_ref()?;
    let state = session.play_state.as_ref();
    Some(NowPlaying {
        item: item.id?,
        title: item.name.clone().unwrap_or_default(),
        subtitle: item.series_name.clone().unwrap_or_default(),
        position_ticks: state.and_then(|state| state.position_ticks).unwrap_or(0),
        run_time_ticks: item.run_time_ticks.unwrap_or(0),
        paused: state.and_then(|state| state.is_paused).unwrap_or(false),
        muted: state.and_then(|state| state.is_muted).unwrap_or(false),
        volume: state.and_then(|state| state.volume_level).unwrap_or(100),
        repeat: repeat(state.and_then(|state| state.repeat_mode)),
        shuffled: matches!(
            state.and_then(|state| state.playback_order),
            Some(PlaybackOrder::Shuffle)
        ),
    })
}

/// True for a session the current user may drive from here.
fn controllable(session: &SessionInfoDto, user: Uuid, device: &Device) -> bool {
    if !session.supports_media_control.unwrap_or(false) {
        return false;
    }
    if session.device_id.as_deref() == Some(device.id().to_string().as_str()) {
        return false;
    }
    let mine = session.user_id == Some(user);
    let shared = session
        .additional_users
        .as_ref()
        .is_some_and(|users| users.iter().any(|other| other.user_id == Some(user)));
    mine || shared
}

/// The sessions a listing names, read as targets: those the current user may
/// control, this client's own device excluded, each carrying what its player
/// state reports and nothing it does not.
pub fn read(sessions: &[SessionInfoDto], user: Uuid, device: &Device) -> Vec<Target> {
    sessions
        .iter()
        .filter(|session| controllable(session, user, device))
        .filter_map(|session| {
            Some(Target {
                session: session.id.clone()?,
                device_name: session.device_name.clone().unwrap_or_default(),
                client_name: session.client.clone().unwrap_or_default(),
                now_playing: now_playing(session),
            })
        })
        .collect()
}

/// The controllable sessions for the current user, this client's own device
/// excluded, read from one request rather than from waiting for a push.
pub async fn targets(upstream: &Upstream, device: &Device) -> Result<Vec<Target>, Failure> {
    let user = upstream.user_id();
    let sessions = upstream
        .control()
        .get_sessions(None, Some(&user), None)
        .await
        .map_err(|e| upstream.failed(e))?;
    Ok(read(&sessions, user, device))
}

fn play_command(mode: PlayMode) -> PlayCommand {
    match mode {
        PlayMode::Now => PlayCommand::PlayNow,
        PlayMode::Next => PlayCommand::PlayNext,
        PlayMode::Last => PlayCommand::PlayLast,
        PlayMode::InstantMix => PlayCommand::PlayInstantMix,
        PlayMode::Shuffle => PlayCommand::PlayShuffle,
    }
}

/// The playstate command a transport drive is issued as.
fn playstate(drive: &Drive) -> Option<(PlaystateCommand, Option<i64>)> {
    match drive {
        Drive::PlayPause => Some((PlaystateCommand::PlayPause, None)),
        Drive::Stop => Some((PlaystateCommand::Stop, None)),
        Drive::Seek { position_ticks } => Some((PlaystateCommand::Seek, Some(*position_ticks))),
        Drive::SkipBack => Some((PlaystateCommand::Rewind, None)),
        Drive::SkipForward => Some((PlaystateCommand::FastForward, None)),
        Drive::NextTrack => Some((PlaystateCommand::NextTrack, None)),
        Drive::PreviousTrack => Some((PlaystateCommand::PreviousTrack, None)),
        _ => None,
    }
}

/// Issues `drive` against `target` on the Jellyfin server: the playstate
/// route for the transport verbs, the playing route for `Play`, and a general
/// command for the rest.
pub async fn drive(upstream: &Upstream, target: &str, drive: &Drive) -> Result<(), LiveRefusal> {
    let client = upstream.control();
    let outcome = match drive {
        Drive::Play {
            items,
            start_index,
            start_ticks,
            mode,
        } => {
            client
                .play(
                    target,
                    items,
                    play_command(*mode),
                    &jellyfin_api::query::Play {
                        start_index: Some(*start_index),
                        start_position_ticks: Some(*start_ticks),
                        ..Default::default()
                    },
                )
                .await
        }
        other => match playstate(other) {
            Some((command, seek)) => {
                client
                    .send_playstate_command(target, command, None, seek)
                    .await
            }
            None => {
                let commanded = verbs::commanded(other).ok_or(LiveRefusal::TargetRefused)?;
                client.send_full_general_command(target, &commanded).await
            }
        },
    };
    outcome.map_err(|_| LiveRefusal::TargetRefused)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device() -> Device {
        Device::new(Uuid::nil())
    }

    fn user() -> Uuid {
        Uuid::from_u128(1)
    }

    fn listed(id: &str, device_id: &str, controllable: bool, owner: Uuid) -> SessionInfoDto {
        SessionInfoDto {
            id: Some(id.to_owned()),
            device_id: Some(device_id.to_owned()),
            device_name: Some(format!("{id} device")),
            client: Some(format!("{id} client")),
            supports_media_control: Some(controllable),
            user_id: Some(owner),
            ..Default::default()
        }
    }

    fn a_target(session: &str) -> Target {
        Target {
            session: session.to_owned(),
            device_name: String::new(),
            client_name: String::new(),
            now_playing: None,
        }
    }

    async fn seeing(sessions: &[&str]) -> Remote {
        let remote = Remote::new();
        let targets = sessions.iter().map(|id| a_target(id)).collect::<Vec<_>>();
        remote.listed(&targets).await;
        remote
    }

    async fn tabs() -> (TabId, TabId) {
        let tabs = super::super::tabs::Tabs::new();
        let (first, _, _) = tabs.add().await;
        let (second, _, _) = tabs.add().await;
        (first, second)
    }

    #[tokio::test]
    async fn a_target_the_local_server_has_not_seen_is_refused() {
        let remote = seeing(&["known"]).await;
        let (tab, _) = tabs().await;
        assert_eq!(
            remote.take(tab, "unknown").await,
            Err(LiveRefusal::UnknownTarget)
        );
        assert!(remote.take(tab, "known").await.is_ok());
    }

    #[tokio::test]
    async fn a_second_tab_taking_remote_mode_takes_it_from_the_first() {
        let remote = seeing(&["target"]).await;
        let (first, second) = tabs().await;
        assert_eq!(remote.take(first, "target").await, Ok(None));
        assert_eq!(remote.take(second, "target").await, Ok(Some(first)));
        assert_eq!(remote.driving(second).await, Ok("target".to_owned()));
        assert_eq!(remote.driving(first).await, Err(LiveRefusal::NotDriving));
    }

    #[tokio::test]
    async fn a_target_absent_from_the_listing_ends_the_mode() {
        let remote = seeing(&["target"]).await;
        let (tab, _) = tabs().await;
        remote.take(tab, "target").await.expect("the mode is taken");
        assert_eq!(remote.checked(&[a_target("target")]).await, None);
        assert_eq!(remote.checked(&[a_target("other")]).await, Some(tab));
        assert_eq!(remote.driving(tab).await, Err(LiveRefusal::NotDriving));
    }

    #[tokio::test]
    async fn a_drive_from_a_tab_holding_no_mode_is_refused() {
        let remote = seeing(&["target"]).await;
        let (first, second) = tabs().await;
        remote
            .take(first, "target")
            .await
            .expect("the mode is taken");
        assert_eq!(remote.driving(second).await, Err(LiveRefusal::NotDriving));
        remote.leave(second).await;
        assert_eq!(remote.driving(first).await, Ok("target".to_owned()));
        remote.leave(first).await;
        assert_eq!(remote.driving(first).await, Err(LiveRefusal::NotDriving));
    }

    #[tokio::test]
    async fn only_the_newest_listing_is_addressable() {
        let remote = seeing(&["first", "second"]).await;
        let (tab, _) = tabs().await;
        remote.listed(&[a_target("second")]).await;
        assert_eq!(
            remote.take(tab, "first").await,
            Err(LiveRefusal::UnknownTarget)
        );
        assert!(remote.take(tab, "second").await.is_ok());
    }

    #[test]
    fn this_clients_own_session_is_not_a_target() {
        let sessions = vec![
            listed("mine", &Uuid::nil().to_string(), true, user()),
            listed("theirs", "another-device", true, user()),
        ];
        let targets = read(&sessions, user(), &device());
        assert_eq!(
            targets
                .iter()
                .map(|t| t.session.as_str())
                .collect::<Vec<_>>(),
            vec!["theirs"]
        );
    }

    #[test]
    fn a_session_the_user_cannot_control_is_not_a_target() {
        let sessions = vec![
            listed("no-control", "device-a", false, user()),
            listed("another-user", "device-b", true, Uuid::from_u128(2)),
            listed("drivable", "device-c", true, user()),
        ];
        let targets = read(&sessions, user(), &device());
        assert_eq!(
            targets
                .iter()
                .map(|t| t.session.as_str())
                .collect::<Vec<_>>(),
            vec!["drivable"]
        );
    }

    #[tokio::test]
    async fn a_forgotten_session_makes_every_target_unknown_again() {
        let remote = seeing(&["target"]).await;
        let (tab, _) = tabs().await;
        remote.take(tab, "target").await.expect("the mode is taken");
        assert_eq!(
            remote.forget().await,
            Some(Bound {
                tab,
                target: "target".to_owned()
            })
        );
        assert_eq!(
            remote.take(tab, "target").await,
            Err(LiveRefusal::UnknownTarget)
        );
    }

    #[tokio::test]
    async fn taking_remote_mode_while_in_a_group_is_the_group_being_left() {
        use crate::web::AppState;
        use jellium_protocol::{Event, GroupEnded, GroupState};
        use std::sync::Arc;

        let path = std::env::temp_dir()
            .join("jellium-cli-remote-tests")
            .join("grouped");
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        let server = crate::web::upstream::answering(204).await;
        let state = Arc::new(AppState::stub(path.join("session.env")));
        state
            .session
            .install(crate::web::upstream::Upstream::stub(&server.base))
            .await;

        let (tab, mut arriving, _) = state.live.tabs_for_test().add().await;
        state
            .live
            .entered(
                &state,
                tab,
                jellium_protocol::Group {
                    id: Uuid::from_u128(1),
                    name: "Group".to_owned(),
                    participants: Vec::new(),
                    state: GroupState::Idle,
                },
            )
            .await;
        assert!(matches!(arriving.try_recv(), Ok(Event::Joined { .. })));

        state.live.pushed(vec![a_target("target")]).await;
        super::super::reported(
            &state,
            tab,
            jellium_protocol::Report::TakeRemote {
                target: "target".to_owned(),
            },
        )
        .await;

        let mut ended = false;
        while let Ok(event) = arriving.try_recv() {
            if event
                == (Event::GroupEnded {
                    cause: GroupEnded::Remote,
                })
            {
                ended = true;
            }
        }
        assert!(ended, "taking remote mode did not end the group");
        assert_eq!(server.asked("/SyncPlay/Leave"), 1);
    }

    #[test]
    fn a_target_carries_what_its_player_state_reports() {
        let mut session = listed("playing", "device-a", true, user());
        session.now_playing_item = Some(jellyfin_api::types::BaseItemDto {
            id: Some(Uuid::from_u128(7)),
            name: Some("Episode".to_owned()),
            series_name: Some("Series".to_owned()),
            run_time_ticks: Some(1_000),
            ..Default::default()
        });
        session.play_state = Some(jellyfin_api::types::PlayerStateInfo {
            position_ticks: Some(250),
            is_paused: Some(true),
            is_muted: Some(true),
            volume_level: Some(30),
            repeat_mode: Some(RepeatMode::RepeatAll),
            playback_order: Some(PlaybackOrder::Shuffle),
            ..Default::default()
        });
        let targets = read(&[session], user(), &device());
        assert_eq!(
            targets[0].now_playing,
            Some(NowPlaying {
                item: Uuid::from_u128(7),
                title: "Episode".to_owned(),
                subtitle: "Series".to_owned(),
                position_ticks: 250,
                run_time_ticks: 1_000,
                paused: true,
                muted: true,
                volume: 30,
                repeat: Repeat::All,
                shuffled: true,
            })
        );
    }
}
