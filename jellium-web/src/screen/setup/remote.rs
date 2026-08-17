//! Remote access and automatic port mapping, both written on every save.

use iced::Element;
use iced::widget::{checkbox, column, row};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    pub access: jellium_protocol::SetupRemoteAccess,
}

pub async fn load() -> Answer<State> {
    Answer::of(async {
        Ok(State {
            access: crate::control::setup_remote_access().await.bubbled()?,
        })
    })
    .await
}

pub fn view(state: &State) -> Element<'_, Message> {
    column![
        prose(
            strings::lookup(Text::SetupRemoteAccess).to_owned(),
            typeface::HEADING_3
        ),
        row![
            checkbox(state.access.enable_remote_access)
                .on_toggle(|on| Message::SetupAction(Action::Edited(Edit::RemoteAccess(on)))),
            prose(
                strings::lookup(Text::SetupRemoteAccessEnable).to_owned(),
                typeface::BODY
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
        row![
            checkbox(state.access.enable_automatic_port_mapping)
                .on_toggle(|on| Message::SetupAction(Action::Edited(Edit::PortMapping(on)))),
            prose(
                strings::lookup(Text::SetupRemoteAccessPortMapping).to_owned(),
                typeface::BODY
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .into()
}
