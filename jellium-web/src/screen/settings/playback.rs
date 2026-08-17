//! The playback screen: language preferences, autoplay, the bitrate ceiling,
//! the two skip lengths, and the SyncPlay tuning.

use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::prefs::{Held, SKIPS, SYNC_ATTEMPTS, SYNC_OFFSETS};
use jellium_protocol::{Quality, SyncAccess, sync::SyncMethod};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space};
use crate::text::{self as strings, Text};

use super::{Setting, choice, choices, flag, listed, save};

/// The languages the server reports, which is what the two pickers offer.
#[derive(Debug, Clone)]
pub struct State {
    pub cultures: Vec<jellyfin_api::types::CultureDto>,
}

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            cultures: api.cultures().await.bubbled()?,
        })
    })
    .await
}

const METHODS: [SyncMethod; 3] = [SyncMethod::Auto, SyncMethod::Rate, SyncMethod::Seek];

fn method_label(method: SyncMethod) -> String {
    strings::lookup(match method {
        SyncMethod::Auto => Text::PlaybackSyncMethodAuto,
        SyncMethod::Rate => Text::PlaybackSyncMethodRate,
        SyncMethod::Seek => Text::PlaybackSyncMethodSeek,
    })
    .to_owned()
}

fn quality_label(quality: Quality) -> String {
    match quality {
        Quality::Auto => strings::lookup(Text::PlayerQualityAuto).to_owned(),
        Quality::Limit { bits_per_second } => strings::format(
            Text::PlayerQualityLimit,
            &[&(bits_per_second.bits_per_second() / 1_000_000).to_string()],
        ),
    }
}

/// The two language pickers, play-default-audio-track, subtitle mode, next-
/// episode autoplay, the bitrate ceiling from `Quality::LADDER`, the two skip
/// lengths from `prefs::SKIPS`, and the three SyncPlay controls, which are
/// absent when `sync_play` is `SyncAccess::None`; the save is absent under
/// read-only.
pub fn view<'a>(
    state: &'a State,
    held: Held,
    configuration: &'a jellium_model::form::Form,
    sync_play: SyncAccess,
    read_only: bool,
) -> Element<'a, Message> {
    let mut shown = column![
        listed(
            Text::PlaybackAudioLanguage,
            jellium_model::user::AUDIO_LANGUAGE,
            configuration,
            &state.cultures,
        ),
        listed(
            Text::PlaybackSubtitleLanguage,
            jellium_model::user::SUBTITLE_LANGUAGE,
            configuration,
            &state.cultures,
        ),
        flag(
            Text::PlaybackDefaultAudioTrack,
            jellium_model::user::PLAY_DEFAULT_AUDIO_TRACK,
            configuration,
        ),
        choice(
            Text::PlaybackSubtitleMode,
            jellium_model::user::SUBTITLE_MODE,
            configuration,
        ),
        flag(
            Text::PlaybackNextEpisode,
            jellium_model::user::NEXT_EPISODE_AUTOPLAY,
            configuration,
        ),
        choices(
            Text::PlaybackQuality,
            &Quality::LADDER,
            held.quality,
            quality_label,
            Setting::Quality,
        ),
        choices(
            Text::PlaybackSkipBack,
            &SKIPS,
            held.skip_back_seconds,
            |seconds| strings::format(Text::PlaybackSeconds, &[&seconds.to_string()]),
            Setting::SkipBack,
        ),
        choices(
            Text::PlaybackSkipForward,
            &SKIPS,
            held.skip_forward_seconds,
            |seconds| strings::format(Text::PlaybackSeconds, &[&seconds.to_string()]),
            Setting::SkipForward,
        ),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if sync_play != SyncAccess::None {
        shown = shown
            .push(iced::widget::text(strings::lookup(Text::PlaybackSyncPlay)))
            .push(choices(
                Text::PlaybackSyncOffset,
                &SYNC_OFFSETS,
                held.sync.extra_offset_ms,
                |ms| strings::format(Text::PlaybackSyncMilliseconds, &[&ms.to_string()]),
                Setting::SyncExtraOffset,
            ))
            .push(choices(
                Text::PlaybackSyncMethod,
                &METHODS,
                held.sync.method,
                method_label,
                Setting::SyncMethod,
            ))
            .push(choices(
                Text::PlaybackSyncRateAttempts,
                &SYNC_ATTEMPTS,
                held.sync.rate_attempts,
                |attempts| attempts.to_string(),
                Setting::SyncRateAttempts,
            ))
            .push(choices(
                Text::PlaybackSyncSeekAttempts,
                &SYNC_ATTEMPTS,
                held.sync.seek_attempts,
                |attempts| attempts.to_string(),
                Setting::SyncSeekAttempts,
            ));
    }

    if !read_only {
        shown = shown.push(save());
    }

    shown.into()
}
