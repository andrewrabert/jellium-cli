//! The plugin repositories the server is configured with.

use iced::widget::{button, column, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;

/// Every repository, and what a new one is being named.
#[derive(Debug, Clone)]
pub struct State {
    pub repositories: Vec<jellyfin_api::types::RepositoryInfo>,
    pub naming: String,
    pub url: String,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            repositories: api.repositories().await.bubbled()?,
            naming: String::new(),
            url: String::new(),
        })
    })
    .await
}

/// Each repository with its name and url, its removal, and the control that
/// adds one behind a confirmation naming the url.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::RepositoriesTitle).to_owned(),
        typeface::HEADING_2
    )]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING);

    for repository in &state.repositories {
        let url = repository.url.clone().unwrap_or_default();
        let mut held = row![
            prose(repository.name.clone().unwrap_or_default(), typeface::BODY),
            prose(url.clone(), typeface::BODY),
        ]
        .spacing(theme::CARD_SPACING);
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::RepositoriesRemove).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::RemoveRepository { url: url.clone() },
                        url,
                    ),
                ))),
            );
        }
        page = page.push(held);
    }

    if !read_only {
        page = page.push(
            row![
                text_input(strings::lookup(Text::UsersName), &state.naming)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                text_input(strings::lookup(Text::RepositoriesUrl), &state.url).on_input(|typed| {
                    Message::DashboardAction(super::Action::TypedPassword(typed))
                }),
                button(prose(
                    strings::lookup(Text::RepositoriesAdd).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::AddRepository {
                            name: state.naming.clone(),
                            url: state.url.clone(),
                        },
                        state.url.clone(),
                    )
                ))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}
