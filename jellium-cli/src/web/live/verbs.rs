use jellium_protocol::{Control, Drive, Notice, PlayMode, Repeat};
use jellyfin_api::types::{GeneralCommand, GeneralCommandType, PlayCommand};
use jellyfin_api::types::{PlaystateCommand, PlaystateRequest};

/// Every command this client honours, which is exactly what it declares.
/// `Play` and `PlayState` stand for the whole `PlayCommand` and
/// `PlaystateCommand` families.
pub const HONOURED: [GeneralCommandType; 22] = [
    GeneralCommandType::Play,
    GeneralCommandType::PlayState,
    GeneralCommandType::PlayNext,
    GeneralCommandType::PlayMediaSource,
    GeneralCommandType::VolumeUp,
    GeneralCommandType::VolumeDown,
    GeneralCommandType::SetVolume,
    GeneralCommandType::Mute,
    GeneralCommandType::Unmute,
    GeneralCommandType::ToggleMute,
    GeneralCommandType::SetAudioStreamIndex,
    GeneralCommandType::SetSubtitleStreamIndex,
    GeneralCommandType::SetMaxStreamingBitrate,
    GeneralCommandType::SetRepeatMode,
    GeneralCommandType::SetShuffleQueue,
    GeneralCommandType::SetPlaybackOrder,
    GeneralCommandType::ToggleFullscreen,
    GeneralCommandType::ToggleOsdMenu,
    GeneralCommandType::GoHome,
    GeneralCommandType::GoToSearch,
    GeneralCommandType::DisplayContent,
    GeneralCommandType::DisplayMessage,
];

/// The three verbs declared and honoured only while Live TV is available to
/// the user.
pub const LIVE_TV: [GeneralCommandType; 3] = [
    GeneralCommandType::ChannelUp,
    GeneralCommandType::ChannelDown,
    GeneralCommandType::Guide,
];

/// `HONOURED`, and `LIVE_TV` as well when `live_tv`.
pub fn honoured(live_tv: bool) -> Vec<GeneralCommandType> {
    let mut commands = HONOURED.to_vec();
    if live_tv {
        commands.extend(LIVE_TV);
    }
    commands
}

fn argument<'a>(command: &'a GeneralCommand, name: &str) -> Option<&'a str> {
    command
        .arguments
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .and_then(|(_, value)| value.as_deref())
}

fn repeat(named: &str) -> Option<Repeat> {
    match named {
        "RepeatNone" => Some(Repeat::Off),
        "RepeatOne" => Some(Repeat::One),
        "RepeatAll" => Some(Repeat::All),
        _ => None,
    }
}

fn repeat_name(repeat: Repeat) -> &'static str {
    match repeat {
        Repeat::Off => "RepeatNone",
        Repeat::One => "RepeatOne",
        Repeat::All => "RepeatAll",
    }
}

fn shuffled(named: &str) -> Option<bool> {
    match named {
        "Sorted" | "Default" => Some(false),
        "Shuffle" => Some(true),
        _ => None,
    }
}

/// The control a general command names.
/// A verb outside `honoured(live_tv)`, and one whose arguments do not carry
/// what its effect needs, read as `None`.
/// `SetShuffleQueue` and `SetPlaybackOrder` both read as `SetShuffle`.
pub fn general(command: &GeneralCommand, live_tv: bool) -> Option<Control> {
    let name = command.name?;
    if !honoured(live_tv).contains(&name) {
        return None;
    }
    match name {
        GeneralCommandType::ChannelUp => Some(Control::ChannelUp),
        GeneralCommandType::ChannelDown => Some(Control::ChannelDown),
        GeneralCommandType::Guide => Some(Control::Guide),
        GeneralCommandType::VolumeUp => Some(Control::VolumeUp),
        GeneralCommandType::VolumeDown => Some(Control::VolumeDown),
        GeneralCommandType::Mute => Some(Control::Mute),
        GeneralCommandType::Unmute => Some(Control::Unmute),
        GeneralCommandType::ToggleMute => Some(Control::ToggleMute),
        GeneralCommandType::ToggleFullscreen => Some(Control::ToggleFullscreen),
        GeneralCommandType::ToggleOsdMenu => Some(Control::ToggleDisplay),
        GeneralCommandType::GoHome => Some(Control::GoHome),
        GeneralCommandType::GoToSearch => Some(Control::GoToSearch),
        GeneralCommandType::SetVolume => argument(command, "Volume")
            .and_then(|level| level.parse().ok())
            .map(|level| Control::SetVolume { level }),
        GeneralCommandType::SetAudioStreamIndex => argument(command, "Index")
            .and_then(|index| index.parse().ok())
            .map(|index| Control::SetAudioStream { index }),
        GeneralCommandType::SetSubtitleStreamIndex => {
            let index = argument(command, "Index")?.parse().ok()?;
            Some(Control::SetSubtitleStream {
                index: (index >= 0).then_some(index),
            })
        }
        GeneralCommandType::PlayMediaSource => argument(command, "MediaSourceId")
            .map(|id| Control::SetMediaSource { id: id.to_owned() }),
        GeneralCommandType::SetMaxStreamingBitrate => {
            let named = argument(command, "Bitrate")?;
            let bits_per_second = match named.parse::<i32>() {
                Ok(bits) if bits > 0 => Some(bits),
                Ok(_) => None,
                Err(_) => return None,
            };
            Some(Control::SetMaxBitrate { bits_per_second })
        }
        GeneralCommandType::SetRepeatMode => argument(command, "RepeatMode")
            .and_then(repeat)
            .map(|repeat| Control::SetRepeat { repeat }),
        GeneralCommandType::SetShuffleQueue | GeneralCommandType::SetPlaybackOrder => {
            let named =
                argument(command, "ShuffleMode").or_else(|| argument(command, "PlaybackOrder"))?;
            shuffled(named).map(|shuffled| Control::SetShuffle { shuffled })
        }
        GeneralCommandType::DisplayContent => argument(command, "ItemId")
            .and_then(|item| item.parse().ok())
            .map(|item| Control::Show { item }),
        GeneralCommandType::DisplayMessage => Some(Control::Notify(Notice {
            header: argument(command, "Header").unwrap_or_default().to_owned(),
            text: argument(command, "Text")?.to_owned(),
        })),
        _ => None,
    }
}

/// The control a `Play` message names; the controlling user is not read,
/// because the Jellyfin server owns who may control whom.
pub fn play(request: &jellyfin_api::types::PlayRequest) -> Option<Control> {
    let items = request.item_ids.clone()?;
    let mode = match request.play_command? {
        PlayCommand::PlayNow => PlayMode::Now,
        PlayCommand::PlayNext => PlayMode::Next,
        PlayCommand::PlayLast => PlayMode::Last,
        PlayCommand::PlayInstantMix => PlayMode::InstantMix,
        PlayCommand::PlayShuffle => PlayMode::Shuffle,
    };
    Some(Control::Play {
        items,
        mode,
        start_index: request.start_index.unwrap_or(0),
        start_ticks: request.start_position_ticks.unwrap_or(0),
        media_source: request.media_source_id.clone(),
        audio_stream: request.audio_stream_index,
        subtitle_stream: request.subtitle_stream_index,
    })
}

/// The control a `Playstate` message names.
pub fn playstate(request: &PlaystateRequest) -> Option<Control> {
    match request.command? {
        PlaystateCommand::Stop => Some(Control::Stop),
        PlaystateCommand::Pause => Some(Control::Pause),
        PlaystateCommand::Unpause => Some(Control::Unpause),
        PlaystateCommand::PlayPause => Some(Control::PlayPause),
        PlaystateCommand::NextTrack => Some(Control::NextTrack),
        PlaystateCommand::PreviousTrack => Some(Control::PreviousTrack),
        PlaystateCommand::Rewind => Some(Control::Rewind),
        PlaystateCommand::FastForward => Some(Control::FastForward),
        PlaystateCommand::Seek => Some(Control::Seek {
            position_ticks: request.seek_position_ticks.unwrap_or(0),
        }),
    }
}

fn commanding(
    name: GeneralCommandType,
    arguments: impl IntoIterator<Item = (&'static str, String)>,
) -> GeneralCommand {
    GeneralCommand {
        arguments: arguments
            .into_iter()
            .map(|(key, value)| (key.to_owned(), Some(value)))
            .collect(),
        controlling_user_id: None,
        name: Some(name),
    }
}

/// The `GeneralCommand` a drive is issued as, for the verbs Jellyfin has no
/// dedicated route for.
pub fn commanded(drive: &Drive) -> Option<GeneralCommand> {
    match drive {
        Drive::SetVolume { level } => Some(commanding(
            GeneralCommandType::SetVolume,
            [("Volume", level.to_string())],
        )),
        Drive::ToggleMute => Some(commanding(GeneralCommandType::ToggleMute, [])),
        Drive::SetRepeat { repeat } => Some(commanding(
            GeneralCommandType::SetRepeatMode,
            [("RepeatMode", repeat_name(*repeat).to_owned())],
        )),
        Drive::SetShuffle { shuffled } => Some(commanding(
            GeneralCommandType::SetShuffleQueue,
            [(
                "ShuffleMode",
                if *shuffled { "Shuffle" } else { "Sorted" }.to_owned(),
            )],
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn commanded_as(name: GeneralCommandType, arguments: &[(&str, &str)]) -> GeneralCommand {
        GeneralCommand {
            arguments: arguments
                .iter()
                .map(|(key, value)| ((*key).to_owned(), Some((*value).to_owned())))
                .collect(),
            controlling_user_id: None,
            name: Some(name),
        }
    }

    fn reads(name: GeneralCommandType) -> Option<Control> {
        let arguments: &[(&str, &str)] = match name {
            GeneralCommandType::SetVolume => &[("Volume", "40")],
            GeneralCommandType::SetAudioStreamIndex
            | GeneralCommandType::SetSubtitleStreamIndex => &[("Index", "1")],
            GeneralCommandType::PlayMediaSource => &[("MediaSourceId", "source")],
            GeneralCommandType::SetMaxStreamingBitrate => &[("Bitrate", "4000000")],
            GeneralCommandType::SetRepeatMode => &[("RepeatMode", "RepeatAll")],
            GeneralCommandType::SetShuffleQueue | GeneralCommandType::SetPlaybackOrder => {
                &[("ShuffleMode", "Shuffle")]
            }
            GeneralCommandType::DisplayContent => {
                &[("ItemId", "0f0f0f0f-0f0f-0f0f-0f0f-0f0f0f0f0f0f")]
            }
            GeneralCommandType::DisplayMessage => &[("Header", "Hello"), ("Text", "There")],
            _ => &[],
        };
        general(&commanded_as(name, arguments), true)
    }

    /// Declared for the message families `play` and `playstate` read; these
    /// never arrive as a general command themselves.
    const AS_FAMILIES: [GeneralCommandType; 3] = [
        GeneralCommandType::Play,
        GeneralCommandType::PlayNext,
        GeneralCommandType::PlayState,
    ];

    /// Every verb the Jellyfin server can send, honoured or not.
    const EVERY: [GeneralCommandType; 43] = [
        GeneralCommandType::MoveUp,
        GeneralCommandType::MoveDown,
        GeneralCommandType::MoveLeft,
        GeneralCommandType::MoveRight,
        GeneralCommandType::PageUp,
        GeneralCommandType::PageDown,
        GeneralCommandType::PreviousLetter,
        GeneralCommandType::NextLetter,
        GeneralCommandType::ToggleOsd,
        GeneralCommandType::ToggleContextMenu,
        GeneralCommandType::Select,
        GeneralCommandType::Back,
        GeneralCommandType::TakeScreenshot,
        GeneralCommandType::SendKey,
        GeneralCommandType::SendString,
        GeneralCommandType::GoHome,
        GeneralCommandType::GoToSettings,
        GeneralCommandType::VolumeUp,
        GeneralCommandType::VolumeDown,
        GeneralCommandType::Mute,
        GeneralCommandType::Unmute,
        GeneralCommandType::ToggleMute,
        GeneralCommandType::SetVolume,
        GeneralCommandType::SetAudioStreamIndex,
        GeneralCommandType::SetSubtitleStreamIndex,
        GeneralCommandType::ToggleFullscreen,
        GeneralCommandType::DisplayContent,
        GeneralCommandType::GoToSearch,
        GeneralCommandType::DisplayMessage,
        GeneralCommandType::SetRepeatMode,
        GeneralCommandType::ChannelUp,
        GeneralCommandType::ChannelDown,
        GeneralCommandType::Guide,
        GeneralCommandType::ToggleStats,
        GeneralCommandType::PlayMediaSource,
        GeneralCommandType::PlayTrailers,
        GeneralCommandType::SetShuffleQueue,
        GeneralCommandType::PlayState,
        GeneralCommandType::PlayNext,
        GeneralCommandType::ToggleOsdMenu,
        GeneralCommandType::Play,
        GeneralCommandType::SetMaxStreamingBitrate,
        GeneralCommandType::SetPlaybackOrder,
    ];

    #[test]
    fn every_honoured_verb_reads_as_a_control() {
        for name in HONOURED {
            if AS_FAMILIES.contains(&name) {
                continue;
            }
            assert!(reads(name).is_some(), "{name} read as nothing");
        }
        assert!(
            play(&jellyfin_api::types::PlayRequest {
                item_ids: Some(Vec::new()),
                play_command: Some(PlayCommand::PlayNow),
                ..Default::default()
            })
            .is_some()
        );
        assert!(
            playstate(&PlaystateRequest {
                command: Some(PlaystateCommand::PlayPause),
                ..Default::default()
            })
            .is_some()
        );
    }

    #[test]
    fn a_spatial_navigation_verb_reads_as_nothing() {
        for name in [
            GeneralCommandType::MoveUp,
            GeneralCommandType::MoveDown,
            GeneralCommandType::MoveLeft,
            GeneralCommandType::MoveRight,
            GeneralCommandType::Select,
            GeneralCommandType::Back,
        ] {
            assert!(reads(name).is_none(), "{name} read as a control");
        }
    }

    #[test]
    fn a_command_from_another_user_is_still_honoured() {
        let mut command = commanded_as(GeneralCommandType::ToggleMute, &[]);
        command.controlling_user_id = Some(uuid::Uuid::new_v4());
        assert_eq!(general(&command, true), Some(Control::ToggleMute));
    }

    #[test]
    fn the_declared_list_is_exactly_the_verbs_that_read_as_a_control() {
        for name in EVERY {
            let honoured = honoured(true).contains(&name);
            if AS_FAMILIES.contains(&name) {
                assert!(honoured, "{name} is a declared family");
                continue;
            }
            assert_eq!(reads(name).is_some(), honoured, "{name}");
        }
    }

    #[test]
    fn the_live_tv_verbs_read_as_controls_only_when_live_tv_is_available() {
        for (name, control) in [
            (GeneralCommandType::ChannelUp, Control::ChannelUp),
            (GeneralCommandType::ChannelDown, Control::ChannelDown),
            (GeneralCommandType::Guide, Control::Guide),
        ] {
            assert_eq!(
                general(&commanded_as(name, &[]), true),
                Some(control),
                "{name} with live tv"
            );
            assert_eq!(
                general(&commanded_as(name, &[]), false),
                None,
                "{name} without live tv"
            );
        }
    }

    #[test]
    fn the_declared_list_carries_the_live_tv_verbs_only_when_live_tv_is_available() {
        for name in LIVE_TV {
            assert!(honoured(true).contains(&name), "{name} with live tv");
            assert!(!honoured(false).contains(&name), "{name} without live tv");
        }
        assert_eq!(honoured(false), HONOURED.to_vec());
        assert_eq!(honoured(true).len(), HONOURED.len() + LIVE_TV.len());
    }

    #[test]
    fn a_play_mode_survives_the_round_trip() {
        let control = play(&jellyfin_api::types::PlayRequest {
            item_ids: Some(Vec::new()),
            play_command: Some(PlayCommand::PlayInstantMix),
            start_index: Some(2),
            start_position_ticks: Some(30),
            ..Default::default()
        });
        assert!(matches!(
            control,
            Some(Control::Play {
                mode: PlayMode::InstantMix,
                start_index: 2,
                start_ticks: 30,
                ..
            })
        ));
    }

    #[test]
    fn a_play_carries_its_media_source_and_stream_selections() {
        assert_eq!(
            play(&jellyfin_api::types::PlayRequest {
                item_ids: Some(Vec::new()),
                play_command: Some(PlayCommand::PlayNow),
                media_source_id: Some("source".to_owned()),
                audio_stream_index: Some(3),
                subtitle_stream_index: Some(5),
                ..Default::default()
            }),
            Some(Control::Play {
                items: Vec::new(),
                mode: PlayMode::Now,
                start_index: 0,
                start_ticks: 0,
                media_source: Some("source".to_owned()),
                audio_stream: Some(3),
                subtitle_stream: Some(5),
            })
        );
    }

    #[test]
    fn a_subtitle_index_below_zero_turns_subtitles_off() {
        assert_eq!(
            general(
                &commanded_as(
                    GeneralCommandType::SetSubtitleStreamIndex,
                    &[("Index", "-1")],
                ),
                true
            ),
            Some(Control::SetSubtitleStream { index: None })
        );
    }

    #[test]
    fn an_honoured_verb_without_its_argument_reads_as_nothing() {
        assert!(general(&commanded_as(GeneralCommandType::SetVolume, &[]), true).is_none());
        assert!(general(&commanded_as(GeneralCommandType::DisplayMessage, &[]), true).is_none());
    }

    #[test]
    fn a_drive_with_no_dedicated_route_is_issued_as_a_general_command() {
        let command = commanded(&Drive::SetVolume { level: 20 }).expect("volume is commanded");
        assert_eq!(command.name, Some(GeneralCommandType::SetVolume));
        assert_eq!(
            command.arguments.get("Volume"),
            Some(&Some("20".to_owned()))
        );
        assert!(commanded(&Drive::Stop).is_none());
    }
}
