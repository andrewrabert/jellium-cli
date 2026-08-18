use iced::advanced::widget::Id as WidgetId;
use iced::widget::operation::scroll_to;
use iced::widget::scrollable::AbsoluteOffset;
use iced::widget::{Space, column};
use iced::{Element, Fill, Task};

use crate::app::Message;
use crate::style::{self, Drawn, card, space};

pub use jellium_model::window::{Grid, Id, Scrolled, Window};

/// The scrollable a window drives, named so a scroll routes back to the
/// surface it came from.
fn scrollable_id(id: Id) -> WidgetId {
    match id {
        Id::Guide => WidgetId::new("window-guide"),
        Id::Channels => WidgetId::new("window-channels"),
        Id::Queue => WidgetId::new("window-queue"),
        Id::Recordings => WidgetId::new("window-recordings"),
        Id::Series => WidgetId::new("window-series"),
        Id::Activity => WidgetId::new("window-activity"),
        Id::Log => WidgetId::new("window-log"),
        Id::Catalog => WidgetId::new("window-catalog"),
        Id::Devices => WidgetId::new("window-devices"),
        Id::Keys => WidgetId::new("window-keys"),
        Id::Browse => WidgetId::new("window-browse"),
        Id::Entries => WidgetId::new("window-entries"),
        Id::Section(section) => WidgetId::from(format!("window-section-{section:?}")),
    }
}

/// Scrolls `window` so `index` is the first row shown.
pub fn showing(window: Window, index: usize) -> Task<Message> {
    scroll_to(
        scrollable_id(window.id()),
        AbsoluteOffset {
            x: Some(0.0),
            y: Some(index as f32 * style::drawn(window.cell())),
        },
    )
}

/// Scrolls the surface `id` names to `offset`, which is what a sort change and
/// a letter jump restore.
pub fn resting(id: Id, offset: Drawn) -> Task<Message> {
    scroll_to(
        scrollable_id(id),
        AbsoluteOffset {
            x: Some(0.0),
            y: Some(style::drawn(offset)),
        },
    )
}

/// A windowed grid: a scrollable whose content stands `Grid::rows` rows tall,
/// with only `Grid::built` cells constructed and the rest standing as space.
pub fn grid<'a>(
    grid: Grid,
    wrap: card::Wrap,
    count: usize,
    build: impl Fn(usize) -> Element<'a, Message> + 'a,
) -> Element<'a, Message> {
    let built = grid.built(count);
    let columns = grid.columns();
    let above = (built.start / columns) as f32 * style::drawn(grid.row());
    let below = grid.rows(count).saturating_sub(built.end.div_ceil(columns)) as f32
        * style::drawn(grid.row());

    let cells = built.collect::<Vec<_>>();
    let rows = cells
        .chunks(columns)
        .map(|row| {
            let laid = iced::widget::row(row.iter().map(|index| build(*index)))
                .spacing(style::drawn(space::GUTTER.drawn()));
            match wrap {
                card::Wrap::Leading => laid.into(),
                card::Wrap::Centred => iced::widget::container(laid).center_x(Fill).into(),
            }
        })
        .collect::<Vec<Element<'a, Message>>>();

    let content = column![
        Space::new().height(above),
        column(rows).spacing(style::drawn(space::GUTTER.drawn())),
        Space::new().height(below),
    ]
    .width(Fill);

    let id = grid.id();
    crate::widget::scrolled(content)
        .id(scrollable_id(id))
        .on_scroll(move |viewport| {
            Message::Scrolled(Scrolled {
                id,
                offset: style::measured(viewport.absolute_offset().y),
                extent: style::measured(viewport.bounds().height),
            })
        })
        .height(Fill)
        .into()
}

/// A windowed list: a scrollable whose content stands `count` rows tall, with
/// only `Window::built` constructed and the rest standing as space.
/// The rows built are computed from the extent the layout measures, so a
/// resize needs no message; `Window`'s own extent follows scrolls and page
/// resizes and is what `shown` answers from.
pub fn list<'a>(
    window: Window,
    count: usize,
    build: impl Fn(usize) -> Element<'a, Message> + 'a,
) -> Element<'a, Message> {
    let built = window.built(count);
    let above = built.start as f32 * style::drawn(window.cell());
    let below = count.saturating_sub(built.end) as f32 * style::drawn(window.cell());

    let rows = built.map(build).collect::<Vec<_>>();
    let content = column![
        Space::new().height(above),
        column(rows),
        Space::new().height(below),
    ]
    .width(Fill);

    let id = window.id();
    crate::widget::scrolled(content)
        .id(scrollable_id(id))
        .on_scroll(move |viewport| {
            Message::Scrolled(Scrolled {
                id,
                offset: style::measured(viewport.absolute_offset().y),
                extent: style::measured(viewport.bounds().height),
            })
        })
        .height(Fill)
        .into()
}

/// A windowed row: a horizontal scrollable whose content stands `count` cells
/// wide, with only `Window::built` constructed and the rest standing as space.
pub fn rail<'a>(
    window: Window,
    count: usize,
    build: impl Fn(usize) -> Element<'a, Message> + 'a,
) -> Element<'a, Message> {
    let built = window.built(count);
    let before = built.start as f32 * style::drawn(window.cell());
    let after = count.saturating_sub(built.end) as f32 * style::drawn(window.cell());

    let cells = built.map(build).collect::<Vec<_>>();
    let content = iced::widget::row![
        Space::new().width(before),
        iced::widget::row(cells).spacing(style::drawn(space::GUTTER.drawn())),
        Space::new().width(after),
    ];

    let id = window.id();
    crate::widget::sideways(content)
        .id(scrollable_id(id))
        .on_scroll(move |viewport| {
            Message::Scrolled(Scrolled {
                id,
                offset: style::measured(viewport.absolute_offset().x),
                extent: style::measured(viewport.bounds().width),
            })
        })
        .into()
}
