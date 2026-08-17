//! The API keys the server holds.

use iced::widget::{button, column, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;

/// Every API key, and the application a new one is being named for.
#[derive(Debug, Clone)]
pub struct State {
    pub keys: Vec<jellyfin_api::types::AuthenticationInfo>,
    pub naming: String,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            keys: api.keys().await.bubbled()?,
            naming: String::new(),
        })
    })
    .await
}

/// Each key's application name and creation date, the control that creates
/// one, and its revocation behind a confirmation naming it.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::KeysTitle).to_owned(),
        typeface::HEADING_2
    )]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING);

    if !read_only {
        page = page.push(
            row![
                text_input(strings::lookup(Text::KeysApp), &state.naming)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                button(prose(
                    strings::lookup(Text::KeysCreate).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::CreateKey {
                        app: state.naming.clone(),
                    }
                ))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    for key in &state.keys {
        let Some(token) = key.access_token.clone() else {
            continue;
        };
        let app = key.app_name.clone().unwrap_or_default();
        let mut held = row![
            prose(app.clone(), typeface::BODY),
            prose(
                key.date_created
                    .map(|at| at.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
                typeface::BODY
            ),
        ]
        .spacing(theme::CARD_SPACING);
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::KeysRevoke).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::RevokeKey { key: token },
                        app,
                    ),
                ))),
            );
        }
        page = page.push(held);
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}
