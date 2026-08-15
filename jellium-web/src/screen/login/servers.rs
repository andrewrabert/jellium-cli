use iced::Element;
use iced::widget::{button, column, container, row, scrollable, text};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;

/// One saved server's row: its stored name over its url, its url alone when no
/// probe has ever succeeded, whether it holds a credential, and whether it is
/// the active one.
fn entry<'a>(saved: &'a jellium_protocol::SavedServer, read_only: bool) -> Element<'a, Message> {
    let mut named = column![].spacing(4);
    if !saved.name.is_empty() {
        named = named.push(text(saved.name.clone()).size(18));
    }
    named = named.push(text(saved.server.clone()).size(14));
    if saved.active {
        named = named.push(text(strings::lookup(Text::LoginServersActive)).size(13));
    }
    if saved.credentialed {
        named = named.push(text(strings::lookup(Text::LoginServersSignedIn)).size(13));
    }

    let mut controls = row![
        button(text(strings::lookup(Text::LoginServersSelect))).on_press(Message::LoginAction(
            Action::Select {
                server: saved.server.clone(),
            }
        )),
    ]
    .spacing(theme::CARD_SPACING);

    if !(read_only && saved.credentialed) {
        controls = controls.push(
            button(text(strings::lookup(Text::LoginServersRemove))).on_press(Message::LoginAction(
                Action::Remove {
                    server: saved.server.clone(),
                },
            )),
        );
    }

    container(column![named, controls].spacing(theme::CARD_SPACING))
        .padding(theme::CARD_SPACING)
        .width(iced::Fill)
        .into()
}

/// The list, drawn from the records the last read answered.
pub fn view<'a>(state: &'a super::State) -> Element<'a, Message> {
    let entries = column(
        state
            .servers
            .iter()
            .map(|saved| entry(saved, state.read_only)),
    )
    .spacing(theme::CARD_SPACING);

    let listed = column![
        text(strings::lookup(Text::LoginServersTitle)).size(28),
        scrollable(entries),
        button(text(strings::lookup(Text::LoginServersAdd)))
            .on_press(Message::LoginAction(Action::Add)),
    ]
    .spacing(theme::CARD_SPACING)
    .max_width(560);

    container(listed)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
