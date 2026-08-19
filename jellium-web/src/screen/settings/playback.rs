//! The playback screen: language preferences, autoplay, the bitrate ceiling,
//! the two skip lengths, and the SyncPlay tuning.

use std::rc::Rc;

use iced::Element;
use jellium_model::prefs::{Held, SKIPS, SYNC_ATTEMPTS, SYNC_OFFSETS};
use jellium_protocol::{Quality, SyncAccess, sync::SyncMethod};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::style::typeface;
use crate::text::{self as strings, Template, Text};
use crate::widget;

use super::{Action, Setting};

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
            Template::PlayerQualityLimit,
            &[&(bits_per_second.bits_per_second() / 1_000_000).to_string()],
        ),
    }
}

/// What this region calls the mode `option` names.
fn mode_label(option: &str) -> Text {
    match option {
        "Always" => Text::PlaybackSubtitleModeAlways,
        "OnlyForced" => Text::PlaybackSubtitleModeOnlyForced,
        "None" => Text::PlaybackSubtitleModeNone,
        "Smart" => Text::PlaybackSubtitleModeSmart,
        _ => Text::PlaybackSubtitleModeDefault,
    }
}

/// The sentence this region writes under the mode `option` names.
fn mode_help(option: &str) -> Text {
    match option {
        "Always" => Text::PlaybackSubtitleModeAlwaysHelp,
        "OnlyForced" => Text::PlaybackSubtitleModeOnlyForcedHelp,
        "None" => Text::PlaybackSubtitleModeNoneHelp,
        "Smart" => Text::PlaybackSubtitleModeSmartHelp,
        _ => Text::PlaybackSubtitleModeDefaultHelp,
    }
}

/// Every language the server names as an option, under the option that leaves
/// the preference unset.
fn languages(
    field: jellium_model::form::Field,
    cultures: &[jellyfin_api::types::CultureDto],
) -> Vec<widget::Choice<Action>> {
    let mut offered = vec![widget::Choice {
        label: strings::lookup(Text::PlaybackLanguageAny).to_owned(),
        value: Action::Edited(field, String::new()),
    }];
    offered.extend(cultures.iter().filter_map(|culture| {
        let code = culture.three_letter_iso_language_name.clone()?;
        Some(widget::Choice {
            label: culture.name.clone().unwrap_or_else(|| code.clone()),
            value: Action::Edited(field, code),
        })
    }));
    offered
}

/// The two language dropdowns, play-default-audio-track, the subtitle mode
/// over the help the held mode carries, next-episode autoplay, the bitrate
/// ceiling and the two skip lengths in the screen's own section; and the
/// SyncPlay section, which is absent when `sync_play` is `SyncAccess::None`.
// reference: settings-playback-form
// reference: settings-subtitles-mode
// reference: settings-playback-remembered
// reference: settings-playback-skip
pub fn sections<'a>(
    state: &'a State,
    held: Held,
    configuration: &'a jellium_model::form::Form,
    sync_play: SyncAccess,
) -> Vec<Element<'a, Message>> {
    let options = match jellium_model::user::SUBTITLE_MODE {
        jellium_model::form::Field::Choice { options, .. } => options,
        _ => &[],
    };

    let mut sections = vec![widget::fields(
        typeface::Rank::Second,
        Text::SettingsPlayback,
        [
            widget::select(
                strings::lookup(Text::PlaybackAudioLanguage),
                None,
                languages(jellium_model::user::AUDIO_LANGUAGE, &state.cultures),
                &Action::Edited(
                    jellium_model::user::AUDIO_LANGUAGE,
                    configuration.value(jellium_model::user::AUDIO_LANGUAGE),
                ),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::PlaybackSubtitleLanguage),
                None,
                languages(jellium_model::user::SUBTITLE_LANGUAGE, &state.cultures),
                &Action::Edited(
                    jellium_model::user::SUBTITLE_LANGUAGE,
                    configuration.value(jellium_model::user::SUBTITLE_LANGUAGE),
                ),
                Message::SettingsAction,
            ),
            widget::flag(
                strings::lookup(Text::PlaybackDefaultAudioTrack),
                None,
                configuration.flagged(jellium_model::user::PLAY_DEFAULT_AUDIO_TRACK),
                |on| {
                    Message::SettingsAction(Action::Flagged(
                        jellium_model::user::PLAY_DEFAULT_AUDIO_TRACK,
                        on,
                    ))
                },
            ),
            widget::select(
                strings::lookup(Text::PlaybackSubtitleMode),
                Some(mode_help(
                    &configuration.value(jellium_model::user::SUBTITLE_MODE),
                )),
                super::choices(
                    options.iter().copied(),
                    |option| strings::lookup(mode_label(option)).to_owned(),
                    |option| Action::Edited(jellium_model::user::SUBTITLE_MODE, option.to_owned()),
                ),
                &Action::Edited(
                    jellium_model::user::SUBTITLE_MODE,
                    configuration.value(jellium_model::user::SUBTITLE_MODE),
                ),
                Message::SettingsAction,
            ),
            widget::flag(
                strings::lookup(Text::PlaybackNextEpisode),
                None,
                configuration.flagged(jellium_model::user::NEXT_EPISODE_AUTOPLAY),
                |on| {
                    Message::SettingsAction(Action::Flagged(
                        jellium_model::user::NEXT_EPISODE_AUTOPLAY,
                        on,
                    ))
                },
            ),
            widget::select(
                strings::lookup(Text::PlaybackQuality),
                None,
                super::choices(Quality::LADDER, quality_label, |quality| {
                    Action::Set(Setting::Quality(quality))
                }),
                &Action::Set(Setting::Quality(held.quality)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::PlaybackSkipBack),
                None,
                super::choices(
                    SKIPS,
                    |seconds| strings::format(Template::PlaybackSeconds, &[&seconds.to_string()]),
                    |seconds| Action::Set(Setting::SkipBack(seconds)),
                ),
                &Action::Set(Setting::SkipBack(held.skip_back_seconds)),
                Message::SettingsAction,
            ),
            widget::select(
                strings::lookup(Text::PlaybackSkipForward),
                None,
                super::choices(
                    SKIPS,
                    |seconds| strings::format(Template::PlaybackSeconds, &[&seconds.to_string()]),
                    |seconds| Action::Set(Setting::SkipForward(seconds)),
                ),
                &Action::Set(Setting::SkipForward(held.skip_forward_seconds)),
                Message::SettingsAction,
            ),
        ],
    )];

    if sync_play != SyncAccess::None {
        let offset = widget::select(
            strings::lookup(Text::PlaybackSyncOffset),
            None,
            super::choices(
                SYNC_OFFSETS,
                |ms| strings::format(Template::PlaybackSyncMilliseconds, &[&ms.to_string()]),
                |ms| Action::Set(Setting::SyncExtraOffset(ms)),
            ),
            &Action::Set(Setting::SyncExtraOffset(held.sync.extra_offset_ms)),
            Message::SettingsAction,
        );
        let method = widget::select(
            strings::lookup(Text::PlaybackSyncMethod),
            None,
            super::choices(METHODS, method_label, |method| {
                Action::Set(Setting::SyncMethod(method))
            }),
            &Action::Set(Setting::SyncMethod(held.sync.method)),
            Message::SettingsAction,
        );
        let rate_attempts = widget::select(
            strings::lookup(Text::PlaybackSyncRateAttempts),
            None,
            super::choices(
                SYNC_ATTEMPTS,
                |attempts| attempts.to_string(),
                |attempts| Action::Set(Setting::SyncRateAttempts(attempts)),
            ),
            &Action::Set(Setting::SyncRateAttempts(held.sync.rate_attempts)),
            Message::SettingsAction,
        );
        let seek_attempts = widget::select(
            strings::lookup(Text::PlaybackSyncSeekAttempts),
            None,
            super::choices(
                SYNC_ATTEMPTS,
                |attempts| attempts.to_string(),
                |attempts| Action::Set(Setting::SyncSeekAttempts(attempts)),
            ),
            &Action::Set(Setting::SyncSeekAttempts(held.sync.seek_attempts)),
            Message::SettingsAction,
        );
        sections.push(widget::fields(
            typeface::Rank::Second,
            Text::PlaybackSyncPlay,
            [offset, method, rate_attempts, seek_attempts],
        ));
    }

    sections
}
