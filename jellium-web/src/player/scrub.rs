use std::time::Duration;

use iced::widget::Space;
use iced::widget::{container, row, slider, stack};
use iced::{Element, Fill, Length};
use jellium_protocol::Chapter;

use crate::app::Message;
use crate::player::Action;
use crate::theme;

const TICKS_PER_SECOND: f64 = 10_000_000.0;

fn fraction(part: Duration, whole: Duration) -> f32 {
    if whole.is_zero() {
        return 0.0;
    }
    (part.as_secs_f32() / whole.as_secs_f32()).clamp(0.0, 1.0)
}

fn portion(fraction: f32) -> u16 {
    (fraction * 1000.0).round().clamp(0.0, 1000.0) as u16
}

fn bar<'a>(filled: f32, style: fn(&iced::Theme) -> container::Style) -> Element<'a, Message> {
    let taken = portion(filled);
    row![
        container(Space::new().width(Fill))
            .width(Length::FillPortion(taken.max(1)))
            .height(theme::SCRUB_HEIGHT / 4.0)
            .style(style),
        Space::new().width(Length::FillPortion(1000 - taken.min(999))),
    ]
    .height(theme::SCRUB_HEIGHT / 4.0)
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
fn ticks<'a>(duration: Duration, chapters: &'a [Chapter]) -> Element<'a, Message> {
    let mut lane = row![].height(theme::SCRUB_HEIGHT / 4.0);
    let mut taken = 0u16;
    for chapter in chapters {
        let start = Duration::from_secs_f64(chapter.start_ticks as f64 / TICKS_PER_SECOND);
        let at = portion(fraction(start, duration));
        if at <= taken {
            continue;
        }
        lane = lane
            .push(Space::new().width(Length::FillPortion(at - taken)))
            .push(
                container(Space::new().width(Fill))
                    .width(theme::CHAPTER_TICK_WIDTH)
                    .height(theme::SCRUB_HEIGHT / 4.0)
                    .style(tick_style),
            );
        taken = at;
    }
    lane.push(Space::new().width(Length::FillPortion(1000u16.saturating_sub(taken).max(1))))
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
) -> Element<'a, Message> {
    let seconds = duration.as_secs_f32().max(0.001);
    let handle = slider(
        0.0..=seconds,
        position.as_secs_f32().min(seconds),
        |value| Message::PlayerAction(Action::Scrub(Duration::from_secs_f32(value))),
    )
    .on_release(Message::PlayerAction(Action::ScrubReleased))
    .step(0.1_f32)
    .height(theme::SCRUB_HEIGHT);

    let hovered = {
        move |point: iced::Point| {
            let across = (point.x / theme::CARD_WIDTH.max(1.0)).clamp(0.0, 1.0);
            Message::PlayerAction(Action::Hovered(Duration::from_secs_f32(across * seconds)))
        }
    };

    let bar = stack![
        bar(fraction(buffered, duration), buffered_style),
        bar(fraction(position, duration), elapsed_style),
        ticks(duration, chapters),
        handle,
    ]
    .width(Fill)
    .height(theme::SCRUB_HEIGHT);

    let bar = iced::widget::mouse_area(bar)
        .on_move(hovered)
        .on_exit(Message::PlayerAction(Action::Unhovered));

    let Some(shown) = preview.and_then(|preview| preview.frame.clone()) else {
        return bar.into();
    };

    iced::widget::column![iced::widget::image(shown).width(theme::CARD_WIDTH), bar,]
        .spacing(4)
        .into()
}
