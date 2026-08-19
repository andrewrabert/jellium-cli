use iced::widget::container;
use iced::{Element, Fill};

use crate::app::Message;
use crate::style::{self, Dialog, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, Emphasis};

use super::Action;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    /// The code shown; the secret is never here and never arrives.
    pub code: Option<String>,
    /// Where the request stands, which is what decides the screen's text and
    /// whether it offers a retry.
    pub standing: Option<jellium_model::quickconnect::SignIn>,
}

/// The reference's alert dialog: the code under its title on the dialog's own
/// surface, over the backdrop, and filling the page where the page is too
/// narrow or too short to hold a dialog.
// reference: dialog-fullscreen
pub fn view<'a>(state: &'a super::State, viewport: Viewport) -> Element<'a, Message> {
    let held = &state.quick_connect;
    let code = held.code.iter().flat_map(|code| {
        [
            widget::prose(strings::lookup(Text::LoginQuickConnectCode), typeface::BODY),
            widget::prose(code.clone(), typeface::HEADING_1),
        ]
    });
    let standing: Option<Element<'a, Message>> = match held.standing {
        None | Some(jellium_model::quickconnect::SignIn::Pending) => Some(widget::prose(
            strings::lookup(Text::LoginQuickConnectWaiting),
            typeface::BODY,
        )),
        Some(jellium_model::quickconnect::SignIn::Expired) => Some(widget::block(
            strings::lookup(Text::LoginQuickConnectRetry),
            Some(Message::LoginAction(Action::QuickConnectRetry)),
            Emphasis::Raised,
        )),
        Some(
            jellium_model::quickconnect::SignIn::Disabled
            | jellium_model::quickconnect::SignIn::Authorized,
        ) => None,
    };
    let panel = container(widget::capped(
        viewport,
        space::FIELD_GAP,
        [
            widget::heading(
                typeface::Rank::First,
                strings::lookup(Text::LoginQuickConnectTitle),
            ),
            widget::prose(
                strings::lookup(Text::LoginQuickConnectInstruction),
                typeface::BODY,
            ),
        ]
        .into_iter()
        .chain(code)
        .chain(standing)
        .chain([widget::block(
            strings::lookup(Text::LoginBack),
            Some(Message::LoginAction(Action::Back)),
            Emphasis::Raised,
        )]),
    ))
    .padding(style::padding(space::PAGE_PAD))
    .style(style::dialog);
    let shown = match viewport.dialog() {
        Dialog::Fullscreen => panel.width(Fill).height(Fill),
        Dialog::Fixed => panel,
    };

    container(shown)
        .center_x(Fill)
        .center_y(Fill)
        .style(style::scrim)
        .into()
}
