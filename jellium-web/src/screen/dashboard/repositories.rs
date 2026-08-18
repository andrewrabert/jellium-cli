//! The plugin repositories the server is configured with.

use iced::Element;
use iced::widget::{button, column, container, row, text_input};

use super::frame;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::style::{self, Layout, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};

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

/// The two fields and the control that add a repository, over the repositories
/// as MUI's own list, and the two sentences the reference writes where the
/// server holds none.
// a repository's row carries no press, where the reference draws it as a
// control whose disc opens the url in another tab
// reference: repositories-page
// reference: repositories-row
pub fn view<'a>(state: &'a State, read_only: bool, layout: Layout) -> frame::Filling<'a> {
    let mut page: Vec<Element<'a, Message>> = Vec::new();

    if !read_only {
        page.push(
            row![
                text_input(strings::lookup(Text::UsersName), &state.naming)
                    .style(style::input)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                text_input(strings::lookup(Text::RepositoriesUrl), &state.url)
                    .style(style::input)
                    .on_input(|typed| {
                        Message::DashboardAction(super::Action::TypedPassword(typed))
                    }),
                button(prose(
                    strings::lookup(Text::RepositoriesAdd),
                    typeface::BODY
                ))
                .style(style::submit)
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
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .into(),
        );
    }

    if state.repositories.is_empty() {
        page.push(
            container(
                column![
                    widget::mui::heading(
                        typeface::Rank::Second,
                        strings::lookup(Text::RepositoriesEmpty),
                    ),
                    prose(strings::lookup(Text::RepositoriesEmptyHelp), typeface::BODY),
                ]
                .align_x(iced::Center)
                .spacing(style::drawn(space::REPOSITORIES_EMPTY_GAP.drawn(layout)))
                .max_width(style::drawn(space::REPOSITORIES_EMPTY.drawn(layout))),
            )
            .center_x(iced::Fill)
            .into(),
        );
        return frame::Filling::Stacked(page);
    }

    page.push(widget::mui::listed(
        state.repositories.iter().map(|repository| {
            let url = repository.url.clone().unwrap_or_default();
            widget::mui::Row {
                lead: Some(widget::mui::Lead::Avatar(Icon::OpenInNew)),
                primary: widget::mui::Primary::Headed(
                    typeface::Rank::Third,
                    repository.name.clone().unwrap_or_default().into(),
                ),
                beneath: Some(widget::mui::Beneath::Said(url.clone().into())),
                within: None,
                showing: None,
                trailing: (!read_only).then(|| widget::mui::Trailing {
                    glyph: Icon::Delete,
                    label: Some(Text::RepositoriesRemove),
                    press: Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::RemoveRepository {
                                url: url.clone(),
                            },
                            url,
                        ),
                    )),
                }),
            }
        }),
        layout,
    ));

    frame::Filling::Stacked(page)
}
