use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{column, scrollable, text};
use jellyfin_api::types::BaseItemDto;

use crate::api::Api;
use crate::app::Message;
use crate::error::Trouble;
use crate::images::{self, Cache};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

#[derive(Debug, Clone)]
pub struct State {
    pub libraries: Vec<BaseItemDto>,
    pub continue_watching: Vec<BaseItemDto>,
    pub next_up: Vec<BaseItemDto>,
}

pub async fn load(api: Rc<Api>) -> Result<State, Trouble> {
    Ok(State {
        libraries: api.libraries().await?,
        continue_watching: api.continue_watching().await?,
        next_up: api.next_up().await?,
    })
}

pub fn view<'a>(state: &'a State, images: &'a Cache) -> Element<'a, Message> {
    if state.libraries.is_empty()
        && state.continue_watching.is_empty()
        && state.next_up.is_empty()
    {
        return widget::notice(strings::lookup(Text::HomeEmpty).to_string());
    }

    let mut page = column![].spacing(theme::CARD_SPACING);

    if !state.continue_watching.is_empty() {
        page = page.push(widget::rail(
            Text::HomeContinueWatching,
            &state.continue_watching,
            images,
        ));
    }
    if !state.next_up.is_empty() {
        page = page.push(widget::rail(Text::HomeNextUp, &state.next_up, images));
    }
    if !state.libraries.is_empty() {
        page = page.push(text(strings::lookup(Text::HomeLibraries)).size(22));
        page = page.push(widget::library_row(&state.libraries));
    }

    scrollable(page).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.continue_watching);
    keys.extend(widget::card_images(&state.next_up));
    keys
}
