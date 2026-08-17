use std::collections::HashSet;
use std::time::Duration;

use iced::widget::Space;
use iced::widget::{button, column, container, image, row, scrollable, slider};
use iced::{Element, Fill, Length};
use jellium_protocol::{Method, Quality, Repeat, Subtitles, SyncAccess};

use crate::app::Message;
use crate::icon::Icon;
use crate::images::{self, Cache, Kind as ImageKind};
use crate::player::group::Joined;
use crate::player::live::Live;
use crate::player::remote::{self, Bound};
use crate::player::scrub::scrub;
use crate::player::{Action, Menu, Playing};
use crate::route::Route;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

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

/// Which surface a transport row is drawn on, which is what decides the
/// controls it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Osd,
    Bar,
}

fn acting<'a>(glyph: crate::icon::Icon, label: Text, action: Action) -> Element<'a, Message> {
    crate::widget::icon_button(glyph, label, Message::PlayerAction(action))
}

fn opening<'a>(glyph: crate::icon::Icon, label: Text, menu: Menu) -> Element<'a, Message> {
    acting(glyph, label, Action::OpenMenu(menu))
}

fn art_key(playing: &Playing) -> Option<images::Key> {
    Some(images::Key {
        item: playing.item.id?,
        kind: ImageKind::Primary,
        index: None,
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
    match playing.plan.playable.method {
        Method::DirectPlay | Method::DirectStream => strings::lookup(Text::PlayerDirect),
        Method::Transcode {
            subtitle_burn_in: true,
        } => strings::lookup(Text::PlayerTranscodingBurningInSubtitles),
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
            &[&format!(
                "{:.0}",
                bits_per_second.bits_per_second() as f64 / 1_000_000.0
            )],
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
pub fn chapters<'a>(
    playing: &'a Playing,
    images: &'a Cache,
    viewport: Viewport,
) -> Vec<Element<'a, Message>> {
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
                })
            });
            let thumbnail = crate::widget::tile(
                card::Card::Rail(card::Rail::Backdrop),
                Room::content(viewport),
                handle,
            );
            button(
                iced::widget::column![thumbnail, prose(chapter.name.clone(), typeface::BODY)]
                    .spacing(style::drawn(space::BLOCK_GAP.drawn())),
            )
            .style(style::flat)
            .on_press(Message::PlayerAction(Action::SelectChapter(
                chapter.start_ticks,
            )))
            .into()
        })
        .collect()
}

fn menu<'a>(
    playing: &'a Playing,
    images: &'a Cache,
    viewport: Viewport,
) -> Option<Element<'a, Message>> {
    let entries: Vec<Element<'a, Message>> = match playing.menu? {
        Menu::Settings => [
            (Text::PlayerQuality, Menu::Quality),
            (Text::PlayerChapters, Menu::Chapters),
            (Text::PlayerVersion, Menu::Version),
        ]
        .into_iter()
        .map(|(label, menu)| {
            button(prose(strings::lookup(label), typeface::BODY))
                .style(style::flat)
                .on_press(Message::PlayerAction(Action::OpenMenu(menu)))
                .into()
        })
        .collect(),
        Menu::Audio => playing
            .plan
            .audio_streams
            .iter()
            .map(|stream| {
                button(prose(stream.label.clone(), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::PlayerAction(Action::SelectAudio(stream.index)))
                    .into()
            })
            .collect(),
        Menu::Subtitle => std::iter::once(
            button(prose(
                strings::lookup(Text::PlayerSubtitlesOff),
                typeface::BODY,
            ))
            .style(style::flat)
            .on_press(Message::PlayerAction(Action::SelectSubtitle(
                Subtitles::Off,
            )))
            .into(),
        )
        .chain(playing.plan.subtitle_streams.iter().map(|stream| {
            button(prose(stream.label.clone(), typeface::BODY))
                .style(style::flat)
                .on_press(Message::PlayerAction(Action::SelectSubtitle(
                    Subtitles::Stream {
                        index: stream.index,
                    },
                )))
                .into()
        }))
        .collect(),
        Menu::Quality => Quality::LADDER
            .iter()
            .map(|quality| {
                button(prose(quality_label(*quality), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::PlayerAction(Action::SelectQuality(*quality)))
                    .into()
            })
            .collect(),
        Menu::Chapters => chapters(playing, images, viewport),
        Menu::Version => playing
            .plan
            .sources
            .iter()
            .map(|source| {
                button(prose(source.name.clone(), typeface::BODY))
                    .style(style::flat)
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
                scrollable(column(entries).spacing(style::drawn(space::BLOCK_GAP.drawn()))),
                acting(Icon::ArrowBack, Text::PlayerLeave, Action::CloseMenu),
            ]
            .spacing(style::drawn(space::BLOCK_GAP.drawn())),
        )
        .padding(style::drawn(space::ICON_GAP.drawn()))
        .into(),
    )
}

/// The volume control and its slider, which the panel drops on a narrow page.
// reference: osd-volume
fn volume<'a>(device: crate::prefs::Device, _viewport: Viewport) -> Element<'a, Message> {
    let level = slider(0.0..=1.0_f32, device.volume, |value| {
        Message::PlayerAction(Action::SetVolume(value))
    })
    .step(0.01_f32)
    .width(style::drawn(space::OSD_VOLUME_SLIDER.drawn()));
    let muted = match device.muted {
        true => acting(Icon::VolumeOff, Text::PlayerUnmute, Action::ToggleMute),
        false => acting(Icon::VolumeUp, Text::PlayerMute, Action::ToggleMute),
    };
    container(row![muted, level].align_y(iced::Center))
        .padding(style::padding(space::OSD_VOLUME_MARGIN))
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
    button(prose(strings::lookup(label), typeface::BODY))
        .style(style::flat)
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
    .spacing(style::drawn(space::ICON_GAP.drawn()))
    .align_y(iced::Center)
    .into()
}

/// The scrub bar pointed at a bound target, between the same two clocks the
/// panel's own slider stands between.
fn remote_scrub<'a>(bound: &'a Bound) -> Element<'a, Message> {
    let seconds = bound.duration().as_secs_f32().max(0.001);
    let handle = slider(
        0.0..=seconds,
        bound.shown().as_secs_f32().min(seconds),
        |value| Message::RemoteAction(remote::Action::Scrub(Duration::from_secs_f32(value))),
    )
    .on_release(Message::RemoteAction(remote::Action::ScrubReleased));
    between(bound.shown(), bound.duration(), handle.into())
}

/// A slider between the clock it has reached and the clock it ends at.
// reference: osd-markup
fn between<'a>(
    shown: Duration,
    duration: Duration,
    handle: Element<'a, Message>,
) -> Element<'a, Message> {
    let gap = style::drawn(space::OSD_TIME_GAP.drawn());
    row![
        container(prose(clock(shown), typeface::BODY)).padding(iced::Padding {
            top: 0.0,
            right: gap,
            bottom: 0.0,
            left: 0.0,
        }),
        container(handle)
            .padding(style::padding(space::OSD_SLIDER_PAD))
            .width(Fill),
        container(prose(clock(duration), typeface::BODY)).padding(iced::Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: gap,
        }),
    ]
    .align_y(iced::Center)
    .into()
}

/// The row of controls the panel carries, in the reference's own order.
// reference: osd-previous
// reference: osd-transport
// reference: osd-next
// reference: osd-buttons
// reference: osd-fullscreen
fn transport<'a>(
    playing: &'a Playing,
    surface: Surface,
    sync_play: SyncAccess,
    device: crate::prefs::Device,
    viewport: Viewport,
) -> Element<'a, Message> {
    let paused = match playing.paused {
        true => Icon::PlayArrow,
        false => Icon::Pause,
    };
    let shoulders = match viewport.matches(space::OSD_MARGINS_AT) {
        true => 0.0,
        false => style::drawn(space::ICON_GAP.drawn()),
    };
    let mut controls = row![acting(
        Icon::SkipPrevious,
        Text::PlayerPrevious,
        Action::Previous
    )]
    .spacing(shoulders)
    .align_y(iced::Center);

    if !viewport.matches(space::OSD_SEEK_AT) {
        controls = controls.push(acting(
            Icon::FastRewind,
            Text::PlayerSkipBack,
            Action::SkipBack,
        ));
    }
    controls = controls.push(acting(paused, Text::PlayerPlay, Action::TogglePlay));
    if !viewport.matches(space::OSD_SEEK_AT) {
        controls = controls.push(acting(
            Icon::FastForward,
            Text::PlayerSkipForward,
            Action::SkipForward,
        ));
    }
    controls = controls.push(acting(Icon::SkipNext, Text::PlayerNext, Action::Next));

    if surface == Surface::Osd && !viewport.matches(space::OSD_ENDS_AT) {
        controls = controls.push(
            container(prose(
                strings::format(Text::PlayerEndsAt, &[&clock(ends_at(playing))]),
                typeface::BODY,
            ))
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: style::drawn(space::OSD_ENDS_GAP.drawn()),
            }),
        );
    }

    controls = controls.push(Space::new().width(Fill));

    if !playing.plan.subtitle_streams.is_empty() {
        controls = controls.push(opening(
            Icon::ClosedCaption,
            Text::PlayerSubtitles,
            Menu::Subtitle,
        ));
    }
    if !playing.plan.audio_streams.is_empty() {
        controls = controls.push(opening(Icon::Audiotrack, Text::PlayerAudio, Menu::Audio));
    }
    if !viewport.matches(space::OSD_VOLUME_AT) {
        controls = controls.push(volume(device, viewport));
    }
    controls = controls
        .push(opening(
            Icon::Settings,
            Text::PlayerSettings,
            Menu::Settings,
        ))
        .push(acting(
            Icon::Shuffle,
            Text::QueueShuffle,
            Action::ToggleShuffle,
        ));

    let repeat = playing.queue.repeat();
    let repeating = match repeat {
        Repeat::One => Icon::RepeatOne,
        Repeat::Off | Repeat::All => Icon::Repeat,
    };
    controls = controls
        .push(acting(repeating, repeat_label(repeat), Action::CycleRepeat))
        .push(crate::widget::icon_button(
            Icon::Queue,
            Text::PlayerQueue,
            Message::Navigated(Route::Queue),
        ))
        .push(crate::widget::icon_button(
            Icon::Cast,
            Text::PlayerRemote,
            Message::Navigated(Route::Remote),
        ));
    if sync_play != SyncAccess::None {
        controls = controls.push(crate::widget::icon_button(
            Icon::Groups,
            Text::PlayerSyncPlay,
            Message::Navigated(Route::SyncPlay),
        ));
    }

    if surface == Surface::Osd {
        let (fullscreen, label) = match playing.fullscreen {
            true => (Icon::FullscreenExit, Text::PlayerExitFullscreen),
            false => (Icon::Fullscreen, Text::PlayerFullscreen),
        };
        controls = controls.push(acting(fullscreen, label, Action::ToggleFullscreen));
    }

    container(controls)
        .padding(iced::Padding {
            top: style::drawn(space::OSD_BUTTONS_TOP.drawn()),
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}

/// The time of day the item runs out at, which is what `.endsAtText` writes:
/// the reading now plus what is left to play, counted from midnight so the
/// same clock writes it.
// reference: osd-time
fn ends_at(playing: &Playing) -> Duration {
    const DAY: u64 = 24 * 60 * 60;
    let left = playing.duration.saturating_sub(playing.shown());
    let now = chrono::Local::now()
        .time()
        .signed_duration_since(chrono::NaiveTime::MIN)
        .num_seconds()
        .unsigned_abs();
    Duration::from_secs((now + left.as_secs()) % DAY)
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
            container(prose(
                strings::lookup(Text::PlayerLive),
                typeface::SECONDARY
            ))
            .padding(style::drawn(space::BLOCK_GAP.drawn())),
            prose(
                crate::text::format(
                    Text::PlayerChannel,
                    &[&live.channel.name, &live.channel.number]
                ),
                typeface::HEADING_3
            ),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .align_y(iced::Center),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    if let Some(program) = &live.program {
        named = named.push(prose(program.title.clone(), typeface::BODY));
        named = named.push(prose(crate::livetv::airtime(program), typeface::SECONDARY));
        named = named.push(crate::widget::elapsed_bar(program.elapsed(now)));
    }

    if live.tuning {
        named = named.push(prose(
            crate::text::format(Text::PlayerTuning, &[&live.channel.name]),
            typeface::SECONDARY,
        ));
    }

    let recording = live
        .program
        .as_ref()
        .is_some_and(|program| program.timer.is_some() || program.series_timer.is_some());

    let record: Element<'a, Message> = if recording {
        prose(strings::lookup(Text::PlayerRecording), typeface::SECONDARY)
    } else {
        button(prose(strings::lookup(Text::PlayerRecord), typeface::BODY))
            .style(style::flat)
            .on_press(Message::PlayerAction(Action::Record))
            .into()
    };

    let paused = match playing.paused {
        true => Icon::PlayArrow,
        false => Icon::Pause,
    };
    let controls = row![
        acting(
            Icon::SkipPrevious,
            Text::PlayerChannelPrevious,
            Action::Previous
        ),
        acting(paused, Text::PlayerPlay, Action::TogglePlay),
        acting(Icon::SkipNext, Text::PlayerChannelNext, Action::Next),
        record,
        opening(Icon::Audiotrack, Text::PlayerAudio, Menu::Audio),
        opening(Icon::Settings, Text::PlayerQuality, Menu::Quality),
        Space::new().width(Fill),
        acting(Icon::ArrowBack, Text::PlayerLeave, Action::Leave),
    ]
    .spacing(style::drawn(space::ICON_GAP.drawn()))
    .align_y(iced::Center);

    column![named, controls]
        .spacing(style::drawn(space::ICON_GAP.drawn()))
        .into()
}

/// The panel's first line: the title, and the status the server's own fetching
/// puts at the end of it.
// reference: osd-text
// reference: osd-title
// reference: osd-status
fn heading<'a>(playing: &'a Playing, viewport: Viewport) -> Element<'a, Message> {
    let mut line = row![
        container(prose(title(playing), typeface::HEADING_3)).padding(iced::Padding {
            top: 0.0,
            right: style::drawn(space::OSD_TITLE_GAP.drawn()),
            bottom: 0.0,
            left: 0.0,
        })
    ]
    .align_y(iced::Center);

    if playing.changing {
        let mut status = row![crate::icon::icon(Icon::Autorenew, typeface::BODY)]
            .spacing(style::drawn(space::OSD_STATUS_GAP.drawn()))
            .align_y(iced::Center);
        if !viewport.matches(space::OSD_VOLUME_AT) {
            status = status.push(prose(strings::lookup(Text::PlayerFetching), typeface::BODY));
        }
        line = line.push(Space::new().width(Fill)).push(status);
    }

    osd_text(line.into(), space::OSD_TEXT_INSET)
}

/// One `.osdTextContainer`: its own inset, and the gap it leaves under itself.
// reference: osd-text
fn osd_text<'a>(held: Element<'a, Message>, inset: style::Length) -> Element<'a, Message> {
    container(held)
        .width(Fill)
        .padding(iced::Padding {
            top: 0.0,
            right: 0.0,
            bottom: style::drawn(space::OSD_TEXT_GAP.drawn()),
            left: style::drawn(inset.drawn()),
        })
        .into()
}

/// The slider between the clock it has reached and the clock it ends at.
// reference: osd-time-row
// reference: osd-markup
fn elapsed<'a>(playing: &'a Playing, viewport: Viewport) -> Element<'a, Message> {
    between(
        playing.shown(),
        playing.duration,
        scrub(
            playing.shown(),
            playing.duration,
            playing.buffered,
            &playing.plan.chapters,
            playing.preview.as_ref(),
            viewport,
        ),
    )
}

/// The full-viewport on-screen display drawn over the video element; `group`
/// present draws the waiting indicator and rebinds the controls it owns, and a
/// live playback draws the live display and the tuning indicator instead of the
/// scrub bar and the item transport.
pub fn view<'a>(
    playing: &'a Playing,
    group: Option<&'a Joined>,
    sync_play: SyncAccess,
    device: crate::prefs::Device,
    quality: Quality,
    images: &'a Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    if playing.idle >= crate::player::IDLE_HIDE && !playing.paused && playing.menu.is_none() {
        return container(Space::new().width(Fill))
            .width(Fill)
            .height(Fill)
            .into();
    }

    let mut body = column![heading(playing, viewport)];

    if !viewport.matches(space::OSD_INFO_AT) {
        body = body.push(osd_text(
            prose(method(playing).to_owned(), typeface::SECONDARY),
            space::OSD_SECONDARY_INSET,
        ));
    }

    if let Some(trouble) = playing.trouble {
        body = body.push(prose(strings::lookup(trouble), typeface::BODY));
    }
    if let Some(next) = upcoming(playing) {
        body = body.push(prose(next, typeface::SECONDARY));
    }
    if group.is_some_and(Joined::waiting) {
        body = body.push(prose(
            strings::lookup(Text::SyncPlayWaiting),
            typeface::SECONDARY,
        ));
    }

    let mut panel = body;

    if let Some(menu) = menu(playing, images, viewport) {
        panel = panel.push(menu);
    } else {
        panel = panel.push(prose(quality_label(quality), typeface::SECONDARY));
    }

    panel = match playing.live.as_ref() {
        Some(live) => panel.push(live_transport(playing, live, chrono::Utc::now())),
        None => panel.push(elapsed(playing, viewport)).push(transport(
            playing,
            Surface::Osd,
            sync_play,
            device,
            viewport,
        )),
    };

    column![
        crate::widget::osd_header(sync_play),
        Space::new().height(Fill),
        bottom(panel.spacing(style::drawn(space::ICON_GAP.drawn())).into()),
    ]
    .height(Fill)
    .into()
}

/// `.videoOsdBottom`: the panel along the foot of the page, under the scrim
/// that stands above its own controls, with `.osdControls`' inset inside it.
// reference: osd-bottom
// reference: osd-controls
fn bottom<'a>(controls: Element<'a, Message>) -> Element<'a, Message> {
    let inset = style::drawn(space::OSD_CONTROLS.drawn());
    container(container(controls).padding(iced::Padding {
        top: 0.0,
        right: inset,
        bottom: 0.0,
        left: inset,
    }))
    .padding(iced::Padding {
        top: style::drawn(space::OSD_SCRIM.drawn()),
        right: 0.0,
        bottom: style::drawn(space::OSD_BOTTOM.drawn()),
        left: 0.0,
    })
    .width(Fill)
    .style(style::osd_bottom)
    .into()
}

/// The bar drawn under every screen while audio plays here, while a radio
/// channel plays, and while a remote target is bound.
pub fn bar<'a>(
    transport: Transport<'a>,
    sync_play: SyncAccess,
    device: crate::prefs::Device,
    images: &'a Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
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
                viewport,
            ),
            self::transport(playing, Surface::Bar, sync_play, device, viewport),
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
                viewport,
            ),
            self::transport(playing, Surface::Bar, sync_play, device, viewport),
        ),
    };

    let art: Element<'a, Message> = match art_key.and_then(|key| images.handle(key)) {
        Some(handle) => image(handle)
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
        None => Space::new()
            .width(style::drawn(space::BAR_ART.drawn()))
            .into(),
    };

    let details = column![prose(heading, typeface::BODY), scrubber]
        .spacing(style::drawn(space::BLOCK_GAP.drawn()))
        .width(Fill);

    container(
        row![art, details, controls]
            .spacing(style::drawn(space::ICON_GAP.drawn()))
            .align_y(iced::Center),
    )
    .padding(style::drawn(space::ICON_GAP.drawn()))
    .width(Length::Fill)
    .height(style::drawn(space::BAR.drawn()))
    .into()
}

fn remote_art_key(bound: &Bound) -> Option<images::Key> {
    Some(images::Key {
        item: bound.target.now_playing.as_ref()?.item,
        kind: ImageKind::Primary,
        index: None,
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
