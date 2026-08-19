//! The doors every construct of the reference is drawn through, and the one
//! place a module names what it draws.
//!
//! A construct the reference draws is drawn here under the reference's own name
//! for it, in the role the reference's markup gives it, carrying the sentence
//! the reference's own string key gives it. `reference/constructs.tsv` is the
//! register those names come from, and `jellium-reference/tests/constructs.rs`
//! is what holds this client to it.

use iced::widget::{button, container};
use iced::{Element, Fill};
use jellium_model::appearance::space;
use jellium_model::construct::{Construct, Page, PageClass};

use crate::app::Message;
use crate::style::{self, Viewport};
use crate::text::Said;

/// One construct this client draws that the reference has no counterpart for,
/// each a row of `reference/exemptions.tsv`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Own {
    FailureList,
}

/// `body` on the pages `pages` names, in the room the class those pages share
/// reserves above itself for the fixed header.
/// `pages` is the module's own `DRAWS`, which is what names the reference pages
/// that module draws.
pub fn page<'a>(
    pages: &'static [Page],
    viewport: Viewport,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    container(body)
        .padding(iced::Padding {
            top: room(pages, viewport),
            right: 0.0,
            bottom: style::drawn(space::PAGE_BOTTOM.drawn()),
            left: 0.0,
        })
        .height(Fill)
        .style(style::page)
        .into()
}

/// The room the pages one module draws reserve above themselves, which is one
/// room because one module draws pages of one class.
fn room(pages: &'static [Page], viewport: Viewport) -> f32 {
    let mut classes = pages.iter().map(|page| page.class());
    let held = classes.next().unwrap_or(PageClass::Standalone);
    assert!(
        classes.all(|other| other == held),
        "one module draws pages of two classes, which reserve two rooms"
    );
    style::drawn(space::page_top(held, viewport))
}

/// `held` under the reference's flat link face with the trailing chevron the
/// reference writes.
/// `said` is the sentence the reference's own string key gives it, and None
/// where the reference wraps the construct in a link and writes the sentence on
/// a construct inside it — a drawer link over its own `nav-menu-option-text`, a
/// section title button over its own `section-title-cards`.
pub fn navigation<'a>(
    construct: Construct,
    said: Option<Said>,
    opens: Message,
    held: Element<'a, Message>,
) -> Element<'a, Message> {
    container(button(held).style(style::flat).on_press(opens))
        .id(spoken(construct, said))
        .into()
}

/// `held` carrying the sentence the reference writes inside this construct and
/// wrapping it in neither a link nor a button.
pub fn stated<'a>(
    construct: Construct,
    said: Said,
    held: Element<'a, Message>,
) -> Element<'a, Message> {
    container(held).id(spoken(construct, Some(said))).into()
}

/// `held` under the construct's own name, which the reference's markup gives no
/// string key.
/// This is a naming layer: it answers `held` under the construct's widget id and
/// draws nothing of its own.
pub fn silent<'a>(construct: Construct, held: Element<'a, Message>) -> Element<'a, Message> {
    container(held).id(spoken(construct, None)).into()
}

/// `held` under this client's own construct's name.
/// This is a naming layer: it answers `held` under that construct's widget id
/// and draws nothing of its own.
pub fn own<'a>(own: Own, held: Element<'a, Message>) -> Element<'a, Message> {
    container(held)
        .id(iced::widget::Id::from(format!("{own:?}")))
        .into()
}

/// The widget id one construct stands under: the reference's own name for it,
/// and the sentence it carries where it carries one.
fn spoken(construct: Construct, said: Option<Said>) -> iced::widget::Id {
    match said {
        Some(said) => iced::widget::Id::from(format!("{construct:?}/{}", said.key())),
        None => iced::widget::Id::from(format!("{construct:?}")),
    }
}
