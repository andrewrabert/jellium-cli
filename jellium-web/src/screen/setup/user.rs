//! The first administrator.

use iced::Element;
use iced::widget::{column, text, text_input};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;

use super::{Action, Edit};

#[derive(Debug, Clone)]
pub struct State {
    pub user: jellium_protocol::SetupUser,
    pub confirmation: String,
}

pub async fn load(_api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        let user = crate::control::setup_user().await.bubbled()?;
        Ok(State {
            confirmation: user.password.clone(),
            user,
        })
    })
    .await
}

/// The name, password and confirmation fields; Next is absent while
/// `jellium_model::setup::user_ready` is false.
pub fn view(state: &State) -> Element<'_, Message> {
    let mut page = column![
        text(strings::lookup(Text::SetupUser)).size(20),
        text(strings::lookup(Text::SetupUserName)),
        text_input(strings::lookup(Text::SetupUserName), &state.user.name)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::UserName(typed)))),
        text(strings::lookup(Text::SetupUserPassword)),
        text_input(
            strings::lookup(Text::SetupUserPassword),
            &state.user.password
        )
        .secure(true)
        .on_input(|typed| Message::SetupAction(Action::Edited(Edit::Password(typed)))),
        text(strings::lookup(Text::SetupUserConfirm)),
        text_input(strings::lookup(Text::SetupUserConfirm), &state.confirmation)
            .secure(true)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::Confirmation(typed)))),
    ]
    .spacing(theme::CARD_SPACING);

    if state.user.password != state.confirmation {
        page = page.push(text(strings::lookup(Text::SetupUserMismatch)).size(13));
    }
    page.into()
}
