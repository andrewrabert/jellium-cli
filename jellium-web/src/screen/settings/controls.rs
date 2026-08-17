//! The keyboard controls screen: every key the player honours, read from the
//! one table the player matches against.

use iced::Element;

use crate::app::Message;
use crate::player::binding::BINDINGS;
use crate::style::space;
use crate::text::{self as strings, Text};
use crate::widget;

/// Every entry of `player::binding::BINDINGS` as a row of the reference's own
/// list, the key it names over what it does, and no control that changes one.
// reference: settings-controls-form
pub fn sections<'a>() -> Vec<Element<'a, Message>> {
    vec![widget::fields(
        Text::SettingsControls,
        [widget::list::listed(
            space::ListRow::bare(space::Lines::Two),
            BINDINGS.iter().map(|binding| widget::list::Row {
                face: None,
                index: None,
                title: strings::lookup(binding.named).into(),
                secondary: vec![strings::lookup(binding.does.text()).into()],
                press: widget::list::Press::Inert,
                controls: Vec::new(),
            }),
        )],
    )]
}
