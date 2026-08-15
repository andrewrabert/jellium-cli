//! Remote access and automatic port mapping, both written on every save.

use iced::Element;
use iced::widget::{checkbox, column, row, text};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;

use super::{Action, Edit};

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
        text(strings::lookup(Text::SetupRemoteAccess)).size(20),
        row![
            checkbox(state.access.enable_remote_access)
                .on_toggle(|on| Message::SetupAction(Action::Edited(Edit::RemoteAccess(on)))),
            text(strings::lookup(Text::SetupRemoteAccessEnable)),
        ]
        .spacing(theme::CARD_SPACING),
        row![
            checkbox(state.access.enable_automatic_port_mapping)
                .on_toggle(|on| Message::SetupAction(Action::Edited(Edit::PortMapping(on)))),
            text(strings::lookup(Text::SetupRemoteAccessPortMapping)),
        ]
        .spacing(theme::CARD_SPACING),
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}
