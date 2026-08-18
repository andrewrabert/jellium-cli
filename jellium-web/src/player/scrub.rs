use std::time::Duration;

use iced::widget::Space;
use iced::widget::{container, row, slider, stack};
use iced::{Element, Fill, Length};
use jellium_protocol::Chapter;

use crate::app::Message;
use crate::player::Action;
use crate::style::{self, Drawn, Share, Viewport, space};

const TICKS_PER_SECOND: f64 = 10_000_000.0;

/// The portions a bar is divided into, which is the resolution its fills carry.
const PORTIONS: u16 = 1_000;

/// How far through `whole` the instant `part` stands.
fn through(part: Duration, whole: Duration) -> Share {
    Share::part(part.as_millis() as i64, whole.as_millis() as i64)
}

fn portion(share: Share) -> u16 {
    style::drawn(share.of(Drawn::of(f64::from(PORTIONS)))).round() as u16
}

fn bar<'a>(filled: Share, fill: fn(&iced::Theme) -> container::Style) -> Element<'a, Message> {
    let taken = portion(filled);
    let track = style::drawn(space::SLIDER_TRACK.drawn());
    row![
        container(Space::new().width(Fill))
            .width(Length::FillPortion(taken.max(1)))
            .height(track)
            .style(fill),
        Space::new().width(Length::FillPortion(PORTIONS - taken.min(PORTIONS - 1))),
    ]
    .height(track)
    .into()
}

fn elapsed_style(theme: &iced::Theme) -> container::Style {
    container::Style::default().background(theme.palette().primary)
}

fn buffered_style(theme: &iced::Theme) -> container::Style {
    container::Style::default().background(theme.extended_palette().background.weak.color)
}

fn tick_style(theme: &iced::Theme) -> container::Style {
    container::Style::default().background(theme.palette().text)
}

/// The chapter ticks laid out across the bar, each a thin mark at the share of
/// the run time its chapter starts at.
fn ticks<'a>(
    duration: Duration,
    chapters: &'a [Chapter],
    viewport: Viewport,
) -> Element<'a, Message> {
    let layout = viewport.layout();
    let mark = style::drawn(space::SLIDER_MARKER_HEIGHT.drawn(layout));
    let mut lane = row![].height(mark);
    let mut taken = 0u16;
    for chapter in chapters {
        let start = Duration::from_secs_f64(chapter.start_ticks as f64 / TICKS_PER_SECOND);
        let at = portion(through(start, duration));
        if at <= taken {
            continue;
        }
        lane = lane
            .push(Space::new().width(Length::FillPortion(at - taken)))
            .push(
                container(Space::new().width(Fill))
                    .width(style::drawn(space::SLIDER_MARKER_WIDTH.drawn(layout)))
                    .height(mark)
                    .style(tick_style),
            );
        taken = at;
    }
    lane.push(Space::new().width(Length::FillPortion(PORTIONS.saturating_sub(taken).max(1))))
        .into()
}

/// A scrub bar drawing elapsed, buffered and a tick per chapter, seeking to the
/// position pressed or dragged to, and showing `preview` above the pointer.
/// An item with neither trickplay nor chapter images draws an otherwise
/// unchanged bar.
pub fn scrub<'a>(
    position: Duration,
    duration: Duration,
    buffered: Duration,
    chapters: &'a [Chapter],
    preview: Option<&'a crate::player::trickplay::Preview>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let row_height = style::drawn(space::SLIDER_THUMB.drawn());
    let seconds = duration.as_secs_f32().max(0.001);

    let measured = container(iced::widget::responsive(move |size| {
        let handle = slider(
            0.0..=seconds,
            position.as_secs_f32().min(seconds),
            |value| Message::PlayerAction(Action::Scrub(Duration::from_secs_f32(value))),
        )
        .on_release(Message::PlayerAction(Action::ScrubReleased))
        .step(0.1_f32)
        .height(row_height);

        let hovered = move |point: iced::Point| {
            let across = (point.x / size.width.max(1.0)).clamp(0.0, 1.0);
            Message::PlayerAction(Action::Hovered(Duration::from_secs_f32(across * seconds)))
        };

        let lanes = stack![
            bar(through(buffered, duration), buffered_style),
            bar(through(position, duration), elapsed_style),
            ticks(duration, chapters, viewport),
            handle,
        ]
        .width(Fill)
        .height(row_height);

        iced::widget::mouse_area(lanes)
            .on_move(hovered)
            .on_exit(Message::PlayerAction(Action::Unhovered))
            .into()
    }))
    .width(Fill)
    .height(row_height);

    let Some(shown) = preview.and_then(|preview| preview.frame.clone()) else {
        return measured.into();
    };

    let frame = style::drawn(space::preview(viewport).drawn(viewport.layout()));
    iced::widget::column![iced::widget::image(shown).width(frame), measured,]
        .spacing(style::drawn(space::BLOCK_GAP.drawn()))
        .into()
}
