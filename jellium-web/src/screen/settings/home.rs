//! The home screen layout: which libraries the home screen shows, in what
//! order, and which of the two rows stand.

use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};

use super::{Action, Setting, toggle};
use crate::style::{self, space, typeface};
use crate::widget::prose;

/// The libraries the server offers, so the order and the exclusions are edited
/// against names rather than ids.
#[derive(Debug, Clone)]
pub struct State {
    pub libraries: Vec<jellyfin_api::types::BaseItemDto>,
}

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            libraries: api.libraries().await.bubbled()?,
        })
    })
    .await
}

fn named(state: &State, id: Uuid) -> String {
    state
        .libraries
        .iter()
        .find(|library| library.id == Some(id))
        .and_then(|library| library.name.clone())
        .unwrap_or_default()
}

/// The libraries in the configuration's order, each with its move and hide
/// controls, the two row toggles, and the save, which is absent under
/// read-only.
pub fn view<'a>(
    state: &'a State,
    held: jellium_model::prefs::Held,
    configuration: &'a jellium_model::form::Form,
    read_only: bool,
) -> Element<'a, Message> {
    let ids: Vec<Uuid> = state.libraries.iter().filter_map(|it| it.id).collect();
    let order = jellium_model::user::ids(configuration, jellium_model::user::ORDERED_VIEWS);
    let hidden = jellium_model::user::ids(configuration, jellium_model::user::MY_MEDIA_EXCLUDES);
    let arranged = jellium_model::user::arranged(&ids, &order, &[]);

    let mut shown = column![prose(strings::lookup(Text::HomeOrder), typeface::BODY)]
        .spacing(style::drawn(space::GUTTER.drawn()));

    for id in arranged {
        let is_hidden = hidden.contains(&id);
        let mut controls = row![prose(named(state, id), typeface::BODY)]
            .spacing(style::drawn(space::GUTTER.drawn()));
        if !read_only {
            controls = controls
                .push(
                    button(prose(strings::lookup(Text::HomeMoveUp), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::SettingsAction(Action::MoveLibrary {
                            id,
                            down: false,
                        })),
                )
                .push(
                    button(prose(strings::lookup(Text::HomeMoveDown), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::SettingsAction(Action::MoveLibrary {
                            id,
                            down: true,
                        })),
                )
                .push(
                    button(prose(
                        strings::lookup(if is_hidden {
                            Text::HomeShowLibrary
                        } else {
                            Text::HomeHideLibrary
                        })
                        .to_owned(),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::SettingsAction(Action::HideLibrary {
                        id,
                        hidden: !is_hidden,
                    })),
                );
        }
        shown = shown.push(controls);
    }

    shown = shown
        .push(toggle(
            Text::HomeContinueWatchingRow,
            held.continue_watching,
            Setting::ContinueWatchingRow,
        ))
        .push(toggle(
            Text::HomeNextUpRow,
            held.next_up,
            Setting::NextUpRow,
        ));

    if !read_only {
        shown = shown.push(super::save());
    }

    shown.into()
}
