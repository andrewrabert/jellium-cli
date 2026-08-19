//! The server's UI culture and its name.

use iced::widget::{column, pick_list, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::widget::Choice;

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    /// The whole startup configuration, read on entry and written back whole.
    pub configuration: jellium_protocol::SetupConfiguration,
    /// The UI cultures the server reports.
    pub cultures: Vec<Choice<String>>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        let configuration = crate::control::setup_configuration().await.bubbled()?;
        let cultures = api
            .localization_options()
            .await
            .bubbled()?
            .into_iter()
            .map(|option| Choice {
                label: option.name.unwrap_or_default(),
                value: option.value.unwrap_or_default(),
            })
            .collect();
        Ok(State {
            configuration,
            cultures,
        })
    })
    .await
}

/// The culture picker, the server-name field, and the sentence stating that
/// this sets the Jellyfin server's language and leaves Jellium Web's own
/// unaffected.
pub fn view(state: &State) -> Element<'_, Message> {
    let chosen = state
        .cultures
        .iter()
        .find(|choice| choice.value == state.configuration.ui_culture)
        .cloned();
    column![
        prose(strings::lookup(Text::SetupLanguage), typeface::HEADING_3),
        prose(strings::lookup(Text::SetupLanguageCulture), typeface::BODY),
        pick_list(state.cultures.clone(), chosen, |choice| {
            Message::SetupAction(Action::Edited(Edit::Culture(choice)))
        })
        .width(Fill),
        prose(
            strings::lookup(Text::SetupLanguageServerName),
            typeface::BODY
        ),
        text_input(
            strings::lookup(Text::SetupLanguageServerName),
            &state.configuration.server_name,
        )
        .style(style::input)
        .on_input(|typed| Message::SetupAction(Action::Edited(Edit::ServerName(typed)))),
        prose(
            strings::lookup(Text::SetupLanguageScope),
            typeface::SECONDARY
        ),
    ]
    .spacing(style::drawn(space::FIELD_GAP.drawn()))
    .into()
}
