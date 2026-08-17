use iced::Element;
use iced::widget::{button, column, container, row, scrollable};

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action;
use crate::style::typeface;
use crate::widget::prose;

/// One saved server's row: its stored name over its url, its url alone when no
/// probe has ever succeeded, whether it holds a credential, and whether it is
/// the active one.
fn entry<'a>(saved: &'a jellium_protocol::SavedServer, read_only: bool) -> Element<'a, Message> {
    let mut named = column![].spacing(4);
    if !saved.name.is_empty() {
        named = named.push(prose(saved.name.clone(), typeface::HEADING_3));
    }
    named = named.push(prose(saved.server.clone(), typeface::BODY));
    if saved.active {
        named = named.push(prose(
            strings::lookup(Text::LoginServersActive).to_owned(),
            typeface::SECONDARY,
        ));
    }
    if saved.credentialed {
        named = named.push(prose(
            strings::lookup(Text::LoginServersSignedIn).to_owned(),
            typeface::SECONDARY,
        ));
    }

    let mut controls = row![
        button(prose(
            strings::lookup(Text::LoginServersSelect).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::LoginAction(Action::Select {
            server: saved.server.clone(),
        })),
    ]
    .spacing(theme::CARD_SPACING);

    if !(read_only && saved.credentialed) {
        controls = controls.push(
            button(prose(
                strings::lookup(Text::LoginServersRemove).to_owned(),
                typeface::BODY,
            ))
            .on_press(Message::LoginAction(Action::Remove {
                server: saved.server.clone(),
            })),
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
        prose(
            strings::lookup(Text::LoginServersTitle).to_owned(),
            typeface::HEADING_1
        ),
        scrollable(entries),
        button(prose(
            strings::lookup(Text::LoginServersAdd).to_owned(),
            typeface::BODY
        ))
        .on_press(Message::LoginAction(Action::Add)),
    ]
    .spacing(theme::CARD_SPACING)
    .max_width(560);

    container(listed)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}
