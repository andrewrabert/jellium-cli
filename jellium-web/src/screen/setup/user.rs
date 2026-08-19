//! The first administrator.

use iced::Element;
use iced::widget::{column, text_input};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

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
        prose(strings::lookup(Text::SetupUser), typeface::HEADING_3),
        prose(strings::lookup(Text::SetupUserName), typeface::BODY),
        text_input(strings::lookup(Text::SetupUserName), &state.user.name)
            .style(style::input)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::UserName(typed)))),
        prose(strings::lookup(Text::SetupUserPassword), typeface::BODY),
        text_input(
            strings::lookup(Text::SetupUserPassword),
            &state.user.password
        )
        .style(style::input)
        .secure(true)
        .on_input(|typed| Message::SetupAction(Action::Edited(Edit::Password(typed)))),
        prose(strings::lookup(Text::SetupUserConfirm), typeface::BODY),
        text_input(strings::lookup(Text::SetupUserConfirm), &state.confirmation)
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::Confirmation(typed)))),
    ]
    .spacing(style::drawn(space::FIELD_GAP.drawn()));

    if state.user.password != state.confirmation {
        page = page.push(prose(
            strings::lookup(Text::SetupUserMismatch),
            typeface::SECONDARY,
        ));
    }
    page.into()
}
