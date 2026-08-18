use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use jellium_protocol::PlayMode;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use super::{Asked, Kind, Selection, Start};
use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation, Trouble};
use crate::livetv::Channel;
use crate::style::Viewport;
use crate::text::Text;
use iced::Task;

pub use jellium_model::live::Live;

/// The pass that advances the guide's present-instant marker, the elapsed
/// bars, and a paused live playback's timer.
pub const LIVE_TICK: Duration = Duration::from_secs(1);

/// The start `item` plays as a channel: the channel alone, from the live edge
/// and never resumed, carrying the channel list the display moves through and
/// the program airing on it.
/// An item the Jellyfin server did not hand over as a channel is a relay
/// trouble.
pub async fn tuned(api: Rc<Api>, item: BaseItemDto) -> Answer<Start> {
    Answer::of(async {
        let watched = Channel::read(&item).ok_or(Trouble::Relay {
            status: None,
            detail: "the server described no channel".to_string(),
        })?;
        let channels = api.live_tv_channels(watched.kind, None).await.bubbled()?;
        let program = api.airing(watched.id).await.bubbled()?;

        Ok(Start {
            kind: if watched.kind == jellyfin_api::types::ChannelType::Radio {
                Kind::Audio
            } else {
                Kind::Video
            },
            items: vec![item],
            position: 0,
            start_ticks: 0,
            mode: PlayMode::Now,
            selection: Selection::default(),
            live: Some(Live {
                channel: watched,
                channels,
                program,
                paused: Duration::ZERO,
                resumed: false,
                tuning: true,
                asked: None,
            }),
        })
    })
    .await
}

/// The start `channel` plays, fetched first.
pub async fn resolve(api: Rc<Api>, channel: Uuid) -> Answer<Start> {
    Answer::of(async {
        let item = api.item(channel).await.bubbled()?;
        tuned(api, item).await.bubbled()
    })
    .await
}

/// Plays `channel` here, showing the tuning indicator from the moment of
/// selection.
pub fn play(signed: &mut Signed, channel: Uuid) -> Task<Message> {
    let api = signed.api.clone();
    Task::perform(resolve(api, channel), Message::Resolved)
}

/// Moves to the next or previous channel in the channel list.
pub fn step(signed: &mut Signed, forward: bool) -> Task<Message> {
    let Some(playing) = signed.playing.as_ref() else {
        return Task::none();
    };
    let Some(live) = playing.live.as_ref() else {
        return Task::none();
    };
    let moved = if forward {
        live.next()
    } else {
        live.previous()
    };
    let Some(moved) = moved.map(|channel| channel.id) else {
        return Task::none();
    };
    play(signed, moved)
}

/// Unpauses at the live edge and states that it did.
pub fn unpause(signed: &mut Signed) -> Task<Message> {
    let Some(playing) = signed.playing.as_mut() else {
        return Task::none();
    };
    playing.element.ask(&Asked::SeekToLive);
    playing.element.ask(&Asked::Play);
    playing.paused = false;
    playing.trouble = Some(Text::PlayerLiveEdge);
    if let Some(live) = playing.live.as_mut() {
        live.paused = Duration::ZERO;
    }
    Task::none()
}

/// Creates a timer for the program being watched from the Jellyfin server's
/// defaults.
pub fn record(signed: &mut Signed) -> Task<Message> {
    let api = signed.api.clone();
    let Some(program) = signed
        .playing
        .as_ref()
        .and_then(|playing| playing.live.as_ref())
        .and_then(|live| live.program.as_ref())
        .map(|program| program.id.clone())
    else {
        return Task::none();
    };
    Task::perform(async move { api.record(&program).await }, |outcome| {
        Message::Wrote(Operation::Timer, outcome)
    })
}

/// One `LIVE_TICK` pass while a channel plays: it ages the paused
/// timer, stops playback and releases the tuner once it passes `PAUSED`, and
/// refetches the watched channel's current program — and nothing else — once
/// `Live::due` says the boundary owes one.
pub fn ticked(signed: &mut Signed, now: DateTime<Utc>, viewport: Viewport) -> Task<Message> {
    let api = signed.api.clone();
    let Some(playing) = signed.playing.as_mut() else {
        return Task::none();
    };
    let paused = playing.paused;
    let Some(live) = playing.live.as_mut() else {
        return Task::none();
    };

    if paused {
        live.paused += LIVE_TICK;
        if live.paused >= Live::PAUSED {
            crate::failure::raise(crate::error::refused(
                &jellium_protocol::PlaybackRefused::TunerReleased,
            ));
            return super::leave(signed, viewport);
        }
        return Task::none();
    }
    live.paused = Duration::ZERO;

    if live.due(now) {
        let channel = live.channel.id;
        live.asking(now);
        return Task::perform(
            async move { api.airing(channel).await },
            Message::AiringFetched,
        );
    }
    Task::none()
}
