//! `actionSheet.show`: the sheet every menu of the reference is raised as.

use std::borrow::Cow;

use iced::widget::{Space, button, column, container, row};
use iced::{Element, Fill};

use crate::app::Message;
use crate::icon::Icon;
use crate::style::{self, Dialog, Layout, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{prose, scrolled};

/// One entry of a sheet.
pub enum Entry<'a> {
    /// One `.actionSheetMenuItem`.
    Item(Item<'a>),
    /// One `.actionsheetDivider`.
    Divider,
}

/// One `.actionSheetMenuItem`: its glyph, its name, the line under the name and
/// the text against its trailing edge.
pub struct Item<'a> {
    pub glyph: Option<Icon>,
    pub name: Cow<'a, str>,
    pub secondary: Option<Cow<'a, str>>,
    pub aside: Option<Cow<'a, str>>,
    pub press: Message,
}

/// Whether an item stands as the one already chosen, which is what the sheet
/// draws `check` for where the item names no glyph of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chosen {
    Yes,
    No,
}

impl<'a> Item<'a> {
    /// The item the sheet draws the chosen mark on.
    // reference: action-sheet-selection-glyph
    pub fn chosen(self, chosen: Chosen) -> Item<'a> {
        match (chosen, &self.glyph) {
            (Chosen::Yes, None) => Item {
                glyph: Some(Icon::Check),
                ..self
            },
            _ => self,
        }
    }
}

/// How a sheet stands its entries: centred, or at the leading edge with the
/// width of a glyph reserved before every name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrangement {
    Centred,
    Leading,
}

/// The size a sheet's own lettering takes, which a mobile layout raises.
// reference: action-sheet-large-font
fn lettering(layout: Layout) -> style::Length {
    match layout {
        Layout::Mobile => typeface::SHEET_MOBILE,
        Layout::Desktop | Layout::Television => typeface::BODY,
    }
}

/// `.actionsheetMenuItemIcon`, and the width it takes standing empty where the
/// sheet lines its names up and this item names no glyph of its own.
// reference: action-sheet-menu-item-icon
fn glyph<'a>(held: Option<Icon>) -> Element<'a, Message> {
    let drawn: Element<'a, Message> = match held {
        Some(held) => crate::icon::icon(held, typeface::LIST_ICON),
        None => Space::new()
            .width(style::drawn(typeface::LIST_ICON.drawn()))
            .into(),
    };
    container(drawn)
        .padding(style::padding(space::SHEET_GLYPH_PAD))
        .into()
}

/// One `.actionSheetMenuItem`.
// reference: action-sheet-menu-item
// reference: action-sheet-list-item-body
// reference: action-sheet-item-text
// reference: action-sheet-item-aside
// reference: action-sheet-markup
fn item<'a>(
    held: Item<'a>,
    arrangement: Arrangement,
    lined: bool,
    layout: Layout,
) -> Element<'a, Message> {
    let mut written = column![prose(held.name, lettering(layout))];
    if let Some(secondary) = held.secondary {
        written = written.push(prose(secondary, typeface::SECONDARY));
    }
    let body = container(written)
        .padding(style::padding(space::SHEET_BODY_PAD))
        .width(Fill);

    let mut inner = row![].align_y(iced::Alignment::Center);
    if arrangement == Arrangement::Leading && lined {
        inner = inner.push(glyph(held.glyph));
    }
    inner = inner.push(body);
    if let Some(aside) = held.aside {
        inner = inner.push(
            container(
                iced::widget::text(aside)
                    .size(style::drawn(typeface::SHEET_ASIDE.drawn()))
                    .style(style::sheet_aside),
            )
            .padding(style::padding(space::SHEET_ASIDE_PAD)),
        );
    }

    button(inner)
        .style(style::flat)
        .width(Fill)
        .on_press(held.press)
        .into()
}

/// One `.actionsheetDivider`, inside the margins it leaves above and below.
// reference: action-sheet-divider
fn divider<'a>() -> Element<'a, Message> {
    container(
        container(Space::new())
            .width(Fill)
            .height(style::drawn(space::SHEET_DIVIDER.drawn()))
            .style(style::sheet_divider),
    )
    .padding(
        iced::Padding::ZERO
            .top(style::drawn(space::SHEET_DIVIDER_MARGIN.drawn()))
            .bottom(style::drawn(space::SHEET_DIVIDER_MARGIN.drawn())),
    )
    .into()
}

/// The sheet, at the arrangement its layout names: centred where no entry
/// carries a glyph and a title stands, held to its own share of the viewport
/// otherwise, and filling the page on a television.
// reference: action-sheet
// reference: action-sheet-not-fullscreen
// reference: action-sheet-fullscreen
// reference: action-sheet-content
// reference: action-sheet-content-centred
// reference: action-sheet-scroller
// reference: action-sheet-scroller-tv
// reference: action-sheet-title
// reference: action-sheet-text
// reference: action-sheet-markup
pub fn sheet<'a>(
    title: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
    entries: impl IntoIterator<Item = Entry<'a>>,
    cancel: Option<Message>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let layout = viewport.layout();
    let canvas = viewport.canvas();
    let held: Vec<Entry<'a>> = entries.into_iter().collect();
    let lined = held.iter().any(|entry| match entry {
        Entry::Item(item) => item.glyph.is_some(),
        Entry::Divider => false,
    });
    let arrangement = match (title.is_some() && !lined) || layout == Layout::Television {
        true => Arrangement::Centred,
        false => Arrangement::Leading,
    };

    let mut listed = column![];
    for entry in held {
        listed = listed.push(match entry {
            Entry::Item(one) => item(one, arrangement, lined, layout),
            Entry::Divider => divider(),
        });
    }

    let scroller = match layout {
        Layout::Television => container(scrolled(listed))
            .max_width(style::drawn(
                space::SHEET_TELEVISED_WIDTH.of(canvas.width()),
            ))
            .max_height(style::drawn(
                space::SHEET_TELEVISED_HEIGHT.of(canvas.height()),
            )),
        Layout::Mobile | Layout::Desktop => container(scrolled(listed)).width(Fill),
    };

    let mut content = column![];
    if let Some(title) = title {
        content = content.push(
            container(crate::widget::heading(typeface::Rank::First, title))
                .padding(style::padding(space::SHEET_TITLE_PAD)),
        );
    }
    if let Some(text) = text {
        content = content.push(
            container(prose(text, typeface::BODY)).padding(style::padding(space::SHEET_TEXT_PAD)),
        );
    }
    content = content.push(scroller);
    if let Some(cancel) = cancel {
        content = content.push(
            button(prose(strings::lookup(Text::SheetCancel), typeface::BODY))
                .style(style::raised)
                .on_press(cancel),
        );
    }

    let content = match arrangement {
        Arrangement::Centred => content.align_x(iced::Alignment::Center),
        Arrangement::Leading => content,
    };

    let stood = container(content).padding(style::padding(space::SHEET_CONTENT_PAD));
    let stood = match viewport.dialog() {
        Dialog::Fullscreen => stood.width(Fill).height(Fill),
        Dialog::Fixed => stood
            .max_width(style::drawn(space::SHEET_WIDTH.of(canvas.width())))
            .max_height(style::drawn(space::SHEET_LOOSE_HEIGHT.of(canvas.height()))),
    };

    container(stood.style(style::sheet))
        .center_x(Fill)
        .padding(style::padding(space::PAGE_PAD))
        .into()
}
