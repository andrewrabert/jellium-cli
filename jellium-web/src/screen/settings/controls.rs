//! The keyboard controls screen: every key the player honours, read from the
//! one table the player matches against.

use iced::Element;
use iced::widget::{column, row};

use crate::app::Message;
use crate::player::binding::BINDINGS;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// Every entry of `player::binding::BINDINGS`, each naming its key and what it
/// does, and no control that changes one.
pub fn view<'a>() -> Element<'a, Message> {
    let mut shown = column![
        row![
            prose(
                strings::lookup(Text::ControlsKey).to_owned(),
                typeface::BODY
            ),
            prose(
                strings::lookup(Text::ControlsDoes).to_owned(),
                typeface::BODY
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for binding in BINDINGS {
        shown = shown.push(
            row![
                prose(strings::lookup(binding.named).to_owned(), typeface::BODY),
                prose(
                    strings::lookup(binding.does.text()).to_owned(),
                    typeface::BODY
                ),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        );
    }

    shown.into()
}
