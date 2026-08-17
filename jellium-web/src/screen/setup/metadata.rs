//! The preferred metadata language and the metadata country.

use iced::widget::{column, pick_list};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::Choice;

use super::{Action, Edit};
use crate::style::typeface;
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    /// The whole startup configuration, read on entry and written back whole.
    pub configuration: jellium_protocol::SetupConfiguration,
    pub languages: Vec<Choice>,
    pub countries: Vec<Choice>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        let configuration = crate::control::setup_configuration().await.bubbled()?;
        let languages = api
            .cultures()
            .await
            .bubbled()?
            .into_iter()
            .map(|culture| Choice {
                label: culture.display_name.clone().unwrap_or_default(),
                value: culture
                    .two_letter_iso_language_name
                    .or(culture.three_letter_iso_language_name)
                    .unwrap_or_default(),
            })
            .collect();
        let countries = api
            .countries()
            .await
            .bubbled()?
            .into_iter()
            .map(|country| Choice {
                label: country.display_name.clone().unwrap_or_default(),
                value: country.two_letter_iso_region_name.unwrap_or_default(),
            })
            .collect();
        Ok(State {
            configuration,
            languages,
            countries,
        })
    })
    .await
}

fn chosen(options: &[Choice], held: &str) -> Option<Choice> {
    options.iter().find(|choice| choice.value == held).cloned()
}

pub fn view(state: &State) -> Element<'_, Message> {
    column![
        prose(
            strings::lookup(Text::SetupMetadata).to_owned(),
            typeface::HEADING_3
        ),
        prose(
            strings::lookup(Text::SetupMetadataLanguage).to_owned(),
            typeface::BODY
        ),
        pick_list(
            state.languages.clone(),
            chosen(
                &state.languages,
                &state.configuration.preferred_metadata_language
            ),
            |choice| Message::SetupAction(Action::Edited(Edit::MetadataLanguage(choice))),
        )
        .width(Fill),
        prose(
            strings::lookup(Text::SetupMetadataCountry).to_owned(),
            typeface::BODY
        ),
        pick_list(
            state.countries.clone(),
            chosen(&state.countries, &state.configuration.metadata_country_code),
            |choice| Message::SetupAction(Action::Edited(Edit::MetadataCountry(choice))),
        )
        .width(Fill),
    ]
    .spacing(theme::CARD_SPACING)
    .into()
}
