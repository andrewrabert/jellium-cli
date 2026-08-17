//! The reference's `.listItem`: a face or a position before `.listItemBody`'s
//! own padding, all of it inside `.listItem`'s, the row's own controls trailing
//! it at `.listItemButton`'s margin, over the rule `.listItem-border` draws.
// reference: control-list-item
// reference: control-list-border
// reference: list-body
// reference: list-body-text
// reference: list-button
// reference: list-icon
// reference: list-image
// reference: list-index
// reference: list-progress

use std::borrow::Cow;

use iced::widget::{Space, button, column, container, image};
use iced::{Element, Fill};

use crate::app::Message;
use crate::icon::Icon;
use crate::style::{self, Share, space, typeface};
use crate::widget::{elapsed_bar, line};

/// A row's one-based position in its list, which is what
/// `.listItem-indexnumberleft` writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ordinal(usize);

impl Ordinal {
    /// The row a list builds at `index`, counted from one.
    pub fn at(index: usize) -> Ordinal {
        Ordinal(index + 1)
    }
}

impl std::fmt::Display for Ordinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// What stands before a row's body.
#[derive(Debug, Clone)]
pub enum Face {
    /// `.listItemImage`, drawn at its own square, with
    /// `.listItemProgressBar` across the foot of it where a row carries one,
    /// and the square standing empty until the image arrives.
    Art {
        image: Option<image::Handle>,
        elapsed: Option<Share>,
    },
    /// `.listItemIcon`.
    Glyph(Icon),
}

/// What a press on a row opens.
#[derive(Debug, Clone)]
pub enum Press {
    /// `.listItem` itself is the control.
    Whole(Message),
    /// `.listItemBody` alone is the control, the row's own controls taking
    /// their own presses.
    Body(Message),
    /// `.listItem[data-action=none]`.
    Inert,
}

/// One `.listItem`.
pub struct Row<'a> {
    pub face: Option<Face>,
    /// `.listItem-indexnumberleft`, which the reference writes after the face.
    pub index: Option<Ordinal>,
    pub title: Cow<'a, str>,
    pub secondary: Vec<Cow<'a, str>>,
    pub press: Press,
    pub controls: Vec<Element<'a, Message>>,
}

/// `.listItemImage`, with `.listItemProgressBar` across the foot of it.
// reference: list-image
// reference: list-progress
fn art<'a>(handle: Option<image::Handle>, elapsed: Option<Share>) -> Element<'a, Message> {
    let width = style::drawn(space::LIST_IMAGE.width.drawn());
    let height = style::drawn(space::LIST_IMAGE.height.drawn());
    let square: Element<'a, Message> = match handle {
        Some(handle) => container(image(handle).width(width).height(height))
            .width(width)
            .height(height)
            .into(),
        None => Space::new().width(width).height(height).into(),
    };
    match elapsed {
        None => square,
        Some(share) => iced::widget::stack![
            square,
            container(elapsed_bar(share))
                .width(width)
                .height(height)
                .align_bottom(height),
        ]
        .into(),
    }
}

/// One line of a row's body, in `.listItemBodyText`'s own padding.
// reference: list-body-text
fn body_line<'a>(
    list: space::ListRow,
    size: style::Length,
    content: Cow<'a, str>,
) -> Element<'a, Message> {
    container(line(
        content,
        size,
        typeface::Weight::Regular,
        list.leading(),
    ))
    .padding(style::padding(list.text()))
    .into()
}

/// The cell a row stands in while the page holding it does not.
pub fn reserved<'a>(list: space::ListRow) -> Element<'a, Message> {
    Space::new()
        .height(style::drawn(list.height().drawn()))
        .into()
}

/// One row, held to the height `list` draws every row at, so a windowed list
/// pitches its rows at the cell its window counts in.
// reference: list-markup
pub fn row<'a>(list: space::ListRow, row: Row<'a>) -> Element<'a, Message> {
    let Row {
        face,
        index,
        title,
        secondary,
        press,
        controls,
    } = row;

    let mut written = column![body_line(list, list.title(), title)];
    for held in secondary {
        written = written.push(body_line(list, list.secondary(), held));
    }
    let body: Element<'a, Message> = container(written)
        .padding(style::padding(list.body()))
        .width(Fill)
        .into();

    let (body, whole) = match press {
        Press::Whole(press) => (body, Some(press)),
        Press::Inert => (body, None),
        Press::Body(press) => (
            button(body)
                .style(style::flat)
                .width(Fill)
                .on_press(press)
                .into(),
            None,
        ),
    };

    let mut inner = iced::widget::Row::new().align_y(iced::Alignment::Center);
    match face {
        None => {}
        Some(Face::Art { image, elapsed }) => inner = inner.push(art(image, elapsed)),
        Some(Face::Glyph(glyph)) => {
            inner = inner.push(
                container(crate::icon::icon(glyph, typeface::LIST_ICON))
                    .padding(iced::Padding::ZERO.right(style::drawn(space::LIST_ICON_GAP.drawn()))),
            );
        }
    }
    if let Some(index) = index {
        inner = inner.push(
            container(line(
                index.to_string(),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ))
            .padding(iced::Padding::ZERO.right(style::drawn(space::LIST_INDEX_GAP.drawn()))),
        );
    }
    inner = inner.push(body);
    for control in controls {
        inner = inner.push(control);
    }

    let standing = style::drawn(list.standing().drawn());
    let held: Element<'a, Message> = match whole {
        Some(press) => button(inner)
            .padding(style::padding(list.padding()))
            .width(Fill)
            .height(standing)
            .style(style::flat)
            .on_press(press)
            .into(),
        None => container(inner)
            .padding(style::padding(list.padding()))
            .width(Fill)
            .height(standing)
            .into(),
    };

    column![
        held,
        container(Space::new())
            .width(Fill)
            .height(style::drawn(list.rule().drawn()))
            .style(style::list_rule),
    ]
    .into()
}

/// `rows` stacked, each drawn as `row` draws one.
pub fn listed<'a>(
    list: space::ListRow,
    rows: impl IntoIterator<Item = Row<'a>>,
) -> Element<'a, Message> {
    column(rows.into_iter().map(|held| self::row(list, held))).into()
}
