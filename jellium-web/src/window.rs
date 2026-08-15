use iced::advanced::widget::Id as WidgetId;
use iced::widget::operation::scroll_to;
use iced::widget::scrollable::AbsoluteOffset;
use iced::widget::{Space, column, scrollable};
use iced::{Element, Fill, Task};

use crate::app::Message;
use crate::theme;

pub use jellium_model::window::{Grid, Id, Scrolled, Window};

/// The scrollable a window drives, named so a scroll routes back to the
/// surface it came from.
fn scrollable_id(id: Id) -> WidgetId {
    WidgetId::new(match id {
        Id::Guide => "window-guide",
        Id::Channels => "window-channels",
        Id::Queue => "window-queue",
        Id::Recordings => "window-recordings",
        Id::Schedule => "window-schedule",
        Id::Series => "window-series",
        Id::Activity => "window-activity",
        Id::Log => "window-log",
        Id::Catalog => "window-catalog",
        Id::Users => "window-users",
        Id::Tasks => "window-tasks",
        Id::Devices => "window-devices",
        Id::Plugins => "window-plugins",
        Id::Browse => "window-browse",
        Id::Entries => "window-entries",
    })
}

/// Scrolls `window` so `index` is the first row shown.
pub fn showing(window: Window, index: usize) -> Task<Message> {
    scroll_to(
        scrollable_id(window.id()),
        AbsoluteOffset {
            x: Some(0.0),
            y: Some(index as f32 * window.row()),
        },
    )
}

/// Scrolls the surface `id` names to `offset`, which is what a sort change and
/// a letter jump restore.
pub fn resting(id: Id, offset: f32) -> Task<Message> {
    scroll_to(
        scrollable_id(id),
        AbsoluteOffset {
            x: Some(0.0),
            y: Some(offset),
        },
    )
}

/// A windowed grid: a scrollable whose content stands `Grid::rows` rows tall,
/// with only `Grid::built` cells constructed and the rest standing as space.
pub fn grid<'a>(
    grid: Grid,
    count: usize,
    build: impl Fn(usize) -> Element<'a, Message> + 'a,
) -> Element<'a, Message> {
    let built = grid.built(count);
    let columns = grid.columns();
    let above = (built.start / columns) as f32 * grid.row();
    let below = grid.rows(count).saturating_sub(built.end.div_ceil(columns)) as f32 * grid.row();

    let cells = built.collect::<Vec<_>>();
    let rows = cells
        .chunks(columns)
        .map(|row| {
            iced::widget::row(row.iter().map(|index| build(*index)))
                .spacing(theme::CARD_SPACING)
                .into()
        })
        .collect::<Vec<Element<'a, Message>>>();

    let content = column![
        Space::new().height(above),
        column(rows).spacing(theme::CARD_SPACING),
        Space::new().height(below),
    ]
    .width(Fill);

    let id = grid.id();
    scrollable(content)
        .id(scrollable_id(id))
        .on_scroll(move |viewport| {
            Message::Scrolled(Scrolled {
                id,
                offset: viewport.absolute_offset().y,
                height: viewport.bounds().height,
            })
        })
        .height(Fill)
        .into()
}

/// A windowed list: a scrollable whose content stands `count` rows tall, with
/// only `Window::built` constructed and the rest standing as space.
/// The rows built are computed from the height the layout measures, so a
/// resize needs no message; `Window`'s own height follows scrolls and page
/// resizes and is what `shown` answers from.
pub fn list<'a>(
    window: Window,
    count: usize,
    build: impl Fn(usize) -> Element<'a, Message> + 'a,
) -> Element<'a, Message> {
    let built = window.built(count);
    let above = built.start as f32 * window.row();
    let below = count.saturating_sub(built.end) as f32 * window.row();

    let rows = built.map(build).collect::<Vec<_>>();
    let content = column![
        Space::new().height(above),
        column(rows),
        Space::new().height(below),
    ]
    .width(Fill);

    let id = window.id();
    scrollable(content)
        .id(scrollable_id(id))
        .on_scroll(move |viewport| {
            Message::Scrolled(Scrolled {
                id,
                offset: viewport.absolute_offset().y,
                height: viewport.bounds().height,
            })
        })
        .height(Fill)
        .into()
}
