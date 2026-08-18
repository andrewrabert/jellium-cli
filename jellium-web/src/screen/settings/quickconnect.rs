//! The Quick Connect screen: authorizing a code another device is showing.

use iced::Element;
use jellium_model::quickconnect::Outcome;

use crate::app::Message;
use crate::style::{space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;

use super::Action;

/// The code being typed, the codes this run authorized, and what the last
/// authorize answered.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub code: String,
    /// The codes this run authorized, which is what tells an expired code from
    /// an unknown one.
    pub authorized: Vec<String>,
    pub outcome: Option<Outcome>,
}

/// What authorizing the code now typed asks confirmation for.
fn asking(state: &State) -> crate::screen::confirm::Pending {
    crate::screen::confirm::Pending::of(
        crate::screen::confirm::Destructive::AuthorizeQuickConnect {
            code: state.code.clone(),
        },
        state.code.clone(),
    )
}

/// The description, the code field, the authorize control filling the form's
/// width — absent under read-only — and the outcome of the last authorize.
// reference: settings-quickconnect-form
pub fn sections<'a>(state: &'a State, read_only: bool) -> Vec<Element<'a, Message>> {
    let mut rows = vec![
        widget::description(Text::QuickConnectDescription, space::DESCRIPTION_INSET),
        widget::field(
            strings::lookup(Text::QuickConnectCode),
            &state.code,
            None,
            None,
            |typed| Message::SettingsAction(Action::Typed(typed)),
            match read_only {
                true => Message::Unchanged,
                false => Message::SettingsAction(Action::Ask(asking(state))),
            },
            widget::Secrecy::Shown,
        ),
    ];

    if !read_only {
        rows.push(widget::block(
            strings::lookup(Text::QuickConnectAuthorize),
            Some(Message::SettingsAction(Action::Ask(asking(state)))),
            widget::Emphasis::Submit,
        ));
    }

    if let Some(Outcome::Authorized) = state.outcome {
        rows.push(widget::description(
            Text::QuickConnectAuthorized,
            space::DESCRIPTION_INSET,
        ));
    }

    vec![widget::fields(
        typeface::Rank::Second,
        Text::SettingsQuickConnect,
        rows,
    )]
}
