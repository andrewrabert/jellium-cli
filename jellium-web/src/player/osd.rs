use std::collections::HashSet;
use std::time::Duration;

use iced::widget::Space;
use iced::widget::{button, column, container, image, row, scrollable, slider, text};
use iced::{Element, Fill, Length};
use jellium_protocol::{Method, Quality, Repeat};

use crate::app::Message;
use crate::images::{self, Cache, Kind as ImageKind};
use crate::player::group::Joined;
use crate::player::live::Live;
use crate::player::remote::{self, Bound};
use crate::player::scrub::scrub;
use crate::player::{Action, Menu, Playing};
use crate::route::Route;
use crate::text::{self as strings, Text};
use crate::theme;

const VOLUME_WIDTH: f32 = 96.0;

fn clock(position: Duration) -> String {
    let total = position.as_secs();
    let (hours, minutes, seconds) = (total / 3600, (total % 3600) / 60, total % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

fn control<'a>(label: Text, action: Action) -> Element<'a, Message> {
    button(text(strings::lookup(label)))
        .on_press(Message::PlayerAction(action))
        .into()
}

fn art_key(playing: &Playing) -> Option<images::Key> {
    Some(images::Key {
        item: playing.item.id?,
        kind: ImageKind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    })
}

pub fn images(playing: &Playing) -> HashSet<images::Key> {
    art_key(playing).into_iter().collect()
}

fn title(playing: &Playing) -> String {
    let name = playing.item.name.clone().unwrap_or_default();
    match (
        &playing.item.series_name,
        playing.item.parent_index_number,
        playing.item.index_number,
    ) {
        (Some(series), Some(season), Some(episode)) => strings::format(
            Text::PlayerEpisode,
            &[series, &season.to_string(), &episode.to_string()],
        ),
        _ => name,
    }
}

fn method(playing: &Playing) -> &'static str {
    match playing.plan.method {
        Method::DirectPlay | Method::DirectStream => strings::lookup(Text::PlayerDirect),
        Method::Transcode {
            subtitle_burn_in: true,
        } => strings::lookup(Text::PlayerTranscodingForSubtitles),
        Method::Transcode {
            subtitle_burn_in: false,
        } => strings::lookup(Text::PlayerTranscoding),
    }
}

fn quality_label(quality: Quality) -> String {
    match quality {
        Quality::Auto => strings::lookup(Text::PlayerQualityAuto).to_string(),
        Quality::Limit { bits_per_second } => strings::format(
            Text::PlayerQualityLimit,
            &[&format!("{:.0}", f64::from(bits_per_second) / 1_000_000.0)],
        ),
    }
}

fn repeat_label(repeat: Repeat) -> Text {
    match repeat {
        Repeat::Off => Text::QueueRepeatOff,
        Repeat::One => Text::QueueRepeatOne,
        Repeat::All => Text::QueueRepeatAll,
    }
}

/// The chapter menu: each chapter's name over its chapter image.
pub fn chapters<'a>(playing: &'a Playing, images: &'a Cache) -> Vec<Element<'a, Message>> {
    playing
        .plan
        .chapters
        .iter()
        .enumerate()
        .map(|(at, chapter)| {
            let numbered = crate::failure::narrowed::<i32, _>(Text::FailureChapterIndex, at);
            let handle = playing.item.id.and_then(|item| {
                images.handle(crate::images::Key {
                    item,
                    kind: crate::images::Kind::Chapter,
                    index: numbered,
                    width: theme::IMAGE_WIDTH,
                })
            });
            let thumbnail: Element<'a, Message> = match handle {
                Some(held) => iced::widget::image(held).width(theme::CARD_WIDTH).into(),
                None => iced::widget::Space::new()
                    .width(theme::CARD_WIDTH)
                    .height(theme::CARD_WIDTH * 0.56)
                    .into(),
            };
            button(iced::widget::column![thumbnail, text(chapter.name.clone())].spacing(4))
                .style(button::text)
                .on_press(Message::PlayerAction(Action::SelectChapter(
                    chapter.start_ticks,
                )))
                .into()
        })
        .collect()
}

fn menu<'a>(playing: &'a Playing, images: &'a Cache) -> Option<Element<'a, Message>> {
    let entries: Vec<Element<'a, Message>> = match playing.menu? {
        Menu::Audio => playing
            .plan
            .audio_streams
            .iter()
            .map(|stream| {
                button(text(stream.label.clone()))
                    .on_press(Message::PlayerAction(Action::SelectAudio(stream.index)))
                    .into()
            })
            .collect(),
        Menu::Subtitle => std::iter::once(
            button(text(strings::lookup(Text::PlayerSubtitlesOff)))
                .on_press(Message::PlayerAction(Action::SelectSubtitle(None)))
                .into(),
        )
        .chain(playing.plan.subtitle_streams.iter().map(|stream| {
            button(text(stream.label.clone()))
                .on_press(Message::PlayerAction(Action::SelectSubtitle(Some(
                    stream.index,
                ))))
                .into()
        }))
        .collect(),
        Menu::Quality => Quality::LADDER
            .iter()
            .map(|quality| {
                button(text(quality_label(*quality)))
                    .on_press(Message::PlayerAction(Action::SelectQuality(*quality)))
                    .into()
            })
            .collect(),
        Menu::Chapters => chapters(playing, images),
        Menu::Version => playing
            .plan
            .sources
            .iter()
            .map(|source| {
                button(text(source.name.clone()))
                    .on_press(Message::PlayerAction(Action::SelectVersion(
                        source.id.clone(),
                    )))
                    .into()
            })
            .collect(),
    };

    Some(
        container(
            column![
                scrollable(column(entries).spacing(4)),
                control(Text::PlayerLeave, Action::CloseMenu),
            ]
            .spacing(4),
        )
        .padding(theme::OSD_SPACING)
        .into(),
    )
}

fn volume<'a>(device: crate::prefs::Device) -> Element<'a, Message> {
    slider(0.0..=1.0_f32, device.volume, |value| {
        Message::PlayerAction(Action::SetVolume(value))
    })
    .step(0.01_f32)
    .width(VOLUME_WIDTH)
    .into()
}

/// What the transport controls are pointed at.
pub enum Transport<'a> {
    Local(&'a Playing),
    Remote(&'a Bound),
    /// Playing here, with every control the group owns issuing the group's
    /// equivalent.
    Group(&'a Playing, &'a Joined),
}

fn remote_control<'a>(label: Text, action: remote::Action) -> Element<'a, Message> {
    button(text(strings::lookup(label)))
        .on_press(Message::RemoteAction(action))
        .into()
}

/// The transport pointed at a bound target, which offers no audio, subtitle
/// or quality selection.
fn remote_transport<'a>(bound: &'a Bound) -> Element<'a, Message> {
    let playing = bound.target.now_playing.as_ref();
    row![
        remote_control(Text::PlayerPrevious, remote::Action::Previous),
        remote_control(Text::PlayerSkipBack, remote::Action::SkipBack),
        remote_control(
            if playing.is_some_and(|playing| playing.paused) {
                Text::PlayerPlay
            } else {
                Text::PlayerPause
            },
            remote::Action::TogglePlay
        ),
        remote_control(Text::PlayerSkipForward, remote::Action::SkipForward),
        remote_control(Text::PlayerNext, remote::Action::Next),
        remote_control(Text::PlayerStop, remote::Action::Stop),
        text(strings::format(
            Text::PlayerPosition,
            &[&clock(bound.shown()), &clock(bound.duration())],
        )),
        Space::new().width(Fill),
        remote_control(Text::QueueShuffle, remote::Action::ToggleShuffle),
        remote_control(
            repeat_label(playing.map_or(Repeat::Off, |playing| playing.repeat)),
            remote::Action::CycleRepeat
        ),
        slider(
            0.0..=1.0,
            playing.map_or(1.0, |playing| playing.volume as f32 / 100.0),
            |level| Message::RemoteAction(remote::Action::SetVolume(level)),
        )
        .step(0.01_f32)
        .width(VOLUME_WIDTH),
        remote_control(
            if playing.is_some_and(|playing| playing.muted) {
                Text::PlayerUnmute
            } else {
                Text::PlayerMute
            },
            remote::Action::ToggleMute
        ),
        remote_control(Text::RemoteLeave, remote::Action::Leave),
    ]
    .spacing(theme::OSD_SPACING)
    .align_y(iced::Center)
    .into()
}

/// The scrub bar pointed at a bound target.
fn remote_scrub<'a>(bound: &'a Bound) -> Element<'a, Message> {
    let seconds = bound.duration().as_secs_f32().max(0.001);
    slider(
        0.0..=seconds,
        bound.shown().as_secs_f32().min(seconds),
        |value| Message::RemoteAction(remote::Action::Scrub(Duration::from_secs_f32(value))),
    )
    .on_release(Message::RemoteAction(remote::Action::ScrubReleased))
    .into()
}

fn transport<'a>(
    playing: &'a Playing,
    full: bool,
    sync_play: bool,
    device: crate::prefs::Device,
) -> Element<'a, Message> {
    let mut controls = row![
        control(Text::PlayerPrevious, Action::Previous),
        control(Text::PlayerSkipBack, Action::SkipBack),
        control(
            if playing.paused {
                Text::PlayerPlay
            } else {
                Text::PlayerPause
            },
            Action::TogglePlay
        ),
        control(Text::PlayerSkipForward, Action::SkipForward),
        control(Text::PlayerNext, Action::Next),
        text(strings::format(
            Text::PlayerPosition,
            &[&clock(playing.shown()), &clock(playing.duration)],
        )),
        Space::new().width(Fill),
        control(Text::QueueShuffle, Action::ToggleShuffle),
        control(repeat_label(playing.queue.repeat()), Action::CycleRepeat),
        volume(device),
        control(
            if device.muted {
                Text::PlayerUnmute
            } else {
                Text::PlayerMute
            },
            Action::ToggleMute
        ),
    ]
    .spacing(theme::OSD_SPACING)
    .align_y(iced::Center);

    controls = controls.push(
        button(text(strings::lookup(Text::PlayerRemote)))
            .on_press(Message::Navigated(Route::Remote)),
    );
    if sync_play {
        controls = controls.push(
            button(text(strings::lookup(Text::PlayerSyncPlay)))
                .on_press(Message::Navigated(Route::SyncPlay)),
        );
    }

    if !playing.plan.audio_streams.is_empty() {
        controls = controls.push(control(Text::PlayerAudio, Action::OpenMenu(Menu::Audio)));
    }
    if playing.plan.sources.len() > 1 {
        controls = controls.push(control(
            Text::PlayerVersion,
            Action::OpenMenu(Menu::Version),
        ));
    }
    if !playing.plan.chapters.is_empty() {
        controls = controls.push(control(
            Text::PlayerChapters,
            Action::OpenMenu(Menu::Chapters),
        ));
    }
    if full {
        controls = controls
            .push(control(
                Text::PlayerSubtitles,
                Action::OpenMenu(Menu::Subtitle),
            ))
            .push(control(
                Text::PlayerQuality,
                Action::OpenMenu(Menu::Quality),
            ))
            .push(control(
                if playing.fullscreen {
                    Text::PlayerExitFullscreen
                } else {
                    Text::PlayerFullscreen
                },
                Action::ToggleFullscreen,
            ));
    }

    controls
        .push(
            button(text(strings::lookup(Text::PlayerQueue)))
                .on_press(Message::Navigated(Route::Queue)),
        )
        .push(control(Text::PlayerLeave, Action::Leave))
        .into()
}

fn upcoming(playing: &Playing) -> Option<String> {
    let next = playing.queue.peek_next()?;
    Some(strings::format(
        Text::PlayerNextItem,
        &[&next.name.clone().unwrap_or_default()],
    ))
}

/// The full-viewport on-screen display drawn over the video element; `group`
/// present draws the waiting indicator and rebinds the controls it owns.
/// The live display: a LIVE badge, the channel's name and number, the current
/// program's title with its start and end and an elapsed bar, audio track and
/// quality selection, Record and whether a timer already covers the program,
/// previous and next channel, and no scrub bar, no skip, no chapter list, no
/// queue, no subtitle selection and no version selection.
fn live_transport<'a>(
    playing: &'a Playing,
    live: &'a Live,
    now: chrono::DateTime<chrono::Utc>,
) -> Element<'a, Message> {
    let mut named = column![
        row![
            container(text(strings::lookup(Text::PlayerLive)).size(12)).padding(2),
            text(crate::text::format(
                Text::PlayerChannel,
                &[&live.channel.name, &live.channel.number]
            ))
            .size(20),
        ]
        .spacing(8)
        .align_y(iced::Center),
    ]
    .spacing(4);

    if let Some(program) = &live.program {
        named = named.push(text(program.title.clone()).size(15));
        named = named.push(text(crate::livetv::airtime(program)).size(13));
        named = named.push(crate::widget::elapsed_bar(program.elapsed(now)));
    }

    if live.tuning {
        named = named.push(
            text(crate::text::format(
                Text::PlayerTuning,
                &[&live.channel.name],
            ))
            .size(13),
        );
    }

    let recording = live
        .program
        .as_ref()
        .is_some_and(|program| program.timer.is_some() || program.series_timer.is_some());

    let record: Element<'a, Message> = if recording {
        text(strings::lookup(Text::PlayerRecording)).size(13).into()
    } else {
        button(text(strings::lookup(Text::PlayerRecord)))
            .on_press(Message::PlayerAction(Action::Record))
            .into()
    };

    let controls = row![
        control(Text::PlayerChannelPrevious, Action::Previous),
        control(
            if playing.paused {
                Text::PlayerPlay
            } else {
                Text::PlayerPause
            },
            Action::TogglePlay
        ),
        control(Text::PlayerChannelNext, Action::Next),
        record,
        control(Text::PlayerAudio, Action::OpenMenu(Menu::Audio)),
        control(Text::PlayerQuality, Action::OpenMenu(Menu::Quality)),
        Space::new().width(Fill),
        control(Text::PlayerLeave, Action::Leave),
    ]
    .spacing(theme::OSD_SPACING)
    .align_y(iced::Center);

    column![named, controls].spacing(theme::OSD_SPACING).into()
}

/// The full-viewport on-screen display drawn over the video element; `group`
/// present draws the waiting indicator and rebinds the controls it owns, and a
/// live playback draws the live display and the tuning indicator instead of the
/// scrub bar and the item transport.
pub fn view<'a>(
    playing: &'a Playing,
    group: Option<&'a Joined>,
    sync_play: bool,
    device: crate::prefs::Device,
    quality: Quality,
    now: chrono::DateTime<chrono::Utc>,
    images: &'a Cache,
) -> Element<'a, Message> {
    if playing.idle >= theme::IDLE_HIDE && !playing.paused && playing.menu.is_none() {
        return container(Space::new().width(Fill))
            .width(Fill)
            .height(Fill)
            .into();
    }

    let mut body = column![
        text(title(playing)).size(24),
        text(method(playing)).size(13),
    ]
    .spacing(4);

    if let Some(trouble) = playing.trouble {
        body = body.push(text(strings::lookup(trouble)));
    }
    if let Some(next) = upcoming(playing) {
        body = body.push(text(next).size(13));
    }
    if group.is_some_and(Joined::waiting) {
        body = body.push(text(strings::lookup(Text::SyncPlayWaiting)).size(13));
    }

    let mut page = column![body, Space::new().height(Fill)].height(Fill);

    if let Some(menu) = menu(playing, images) {
        page = page.push(menu);
    } else {
        page = page.push(text(quality_label(quality)).size(13));
    }

    page = match playing.live.as_ref() {
        Some(live) => page.push(live_transport(playing, live, now)),
        None => page
            .push(scrub(
                playing.shown(),
                playing.duration,
                playing.buffered,
                &playing.plan.chapters,
                playing.preview.as_ref(),
            ))
            .push(transport(playing, true, sync_play, device)),
    };

    container(page.spacing(theme::OSD_SPACING))
        .padding(theme::OSD_SPACING)
        .width(Fill)
        .height(Fill)
        .into()
}

/// The bar drawn under every screen while audio plays.
/// The bar drawn under every screen while audio plays here, and while a
/// remote target is bound.
/// The bar drawn under every screen while audio plays here, while a radio
/// channel plays, and while a remote target is bound.
pub fn bar<'a>(
    transport: Transport<'a>,
    sync_play: bool,
    device: crate::prefs::Device,
    quality: Quality,
    now: chrono::DateTime<chrono::Utc>,
    images: &'a Cache,
) -> Element<'a, Message> {
    let _ = now;
    let _ = quality;
    let (art_key, heading, scrubber, controls) = match transport {
        Transport::Local(playing) => (
            self::art_key(playing),
            title(playing),
            scrub(
                playing.shown(),
                playing.duration,
                playing.buffered,
                &playing.plan.chapters,
                None,
            ),
            self::transport(playing, false, sync_play, device),
        ),
        Transport::Remote(bound) => (
            remote_art_key(bound),
            remote_title(bound),
            remote_scrub(bound),
            remote_transport(bound),
        ),
        Transport::Group(playing, joined) => (
            self::art_key(playing),
            if joined.waiting() {
                strings::lookup(Text::SyncPlayWaiting).to_string()
            } else {
                title(playing)
            },
            scrub(
                playing.shown(),
                playing.duration,
                playing.buffered,
                &playing.plan.chapters,
                None,
            ),
            self::transport(playing, false, sync_play, device),
        ),
    };

    let art: Element<'a, Message> = match art_key.and_then(|key| images.handle(key)) {
        Some(handle) => image(handle).width(theme::BAR_ART_WIDTH).into(),
        None => Space::new().width(theme::BAR_ART_WIDTH).into(),
    };

    let details = column![text(heading).size(15), scrubber]
        .spacing(4)
        .width(Fill);

    container(
        row![art, details, controls]
            .spacing(theme::OSD_SPACING)
            .align_y(iced::Center),
    )
    .padding(theme::OSD_SPACING)
    .width(Length::Fill)
    .height(theme::BAR_HEIGHT)
    .into()
}

fn remote_art_key(bound: &Bound) -> Option<images::Key> {
    Some(images::Key {
        item: bound.target.now_playing.as_ref()?.item,
        kind: ImageKind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    })
}

fn remote_title(bound: &Bound) -> String {
    match bound.target.now_playing.as_ref() {
        Some(playing) if playing.subtitle.is_empty() => playing.title.clone(),
        Some(playing) => strings::format(Text::RemoteHeading, &[&playing.subtitle, &playing.title]),
        None => strings::lookup(Text::RemoteNothingPlaying).to_string(),
    }
}

pub fn bar_images(transport: Transport<'_>) -> HashSet<images::Key> {
    match transport {
        Transport::Local(playing) => art_key(playing).into_iter().collect(),
        Transport::Remote(bound) => remote_art_key(bound).into_iter().collect(),
        Transport::Group(playing, _) => art_key(playing).into_iter().collect(),
    }
}
