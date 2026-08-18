use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, CollectionType};
use uuid::Uuid;

use crate::api::Api;
use jellium_model::construct::Construct;

use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space};
use crate::text::{self as strings, Text};
use crate::widget;

/// The card a suggestions rail draws on.
// reference: card-box-classes
const RAIL: card::Drawing = card::Drawing {
    card: card::Card::Rail(card::Rail::Portrait),
    footer: card::Footer::NameAndSubtitle,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
    // reference: suggestions-latest-cards
    touch: card::Touch::Plays,
};

/// The rails a Suggestions tab shows: the server's suggestions, and on a movie
/// library its movie recommendations as one rail per recommendation.
/// The suggestions rail is user-scoped, because `/Items/Suggestions` takes no
/// parent; the recommendation rails are scoped to the library.
#[derive(Debug, Clone)]
pub struct State {
    pub suggestions: Vec<BaseItemDto>,
    pub recommendations: Vec<Rail>,
}

/// One recommendation rail: what the server called it and what it holds.
#[derive(Debug, Clone)]
pub struct Rail {
    pub heading: String,
    pub items: Vec<BaseItemDto>,
}

/// The most items one suggestions rail shows.
pub const LIMIT: i32 = 16;

pub async fn load(
    api: Rc<Api>,
    library: Uuid,
    collection: Option<CollectionType>,
) -> Answer<State> {
    Answer::of(async {
        let suggestions = api
            .suggestions(LIMIT)
            .await
            .or_default(Text::FailureSuggestionsUnread);
        let recommendations = if collection == Some(CollectionType::Movies) {
            api.recommendations(library)
                .await
                .or_default(Text::FailureRecommendationsUnread)
                .into_iter()
                .map(|held| Rail {
                    heading: held.baseline_item_name.unwrap_or_default(),
                    items: held.items.unwrap_or_default(),
                })
                .filter(|rail| !rail.items.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        Ok(State {
            suggestions,
            recommendations,
        })
    })
    .await
}

pub fn view<'a>(
    state: &'a State,
    viewport: Viewport,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    session: &'a Session,
) -> Element<'a, Message> {
    let mut page = column![widget::section(
        widget::prose(
            strings::lookup(Text::LibraryTabSuggestions),
            style::typeface::HEADING_2
        ),
        widget::rail(
            RAIL,
            widget::Rail::of(Construct::ItemsContainer),
            state.suggestions.iter(),
            Room::content(viewport),
            images,
            now,
            session,
        ),
    )]
    .spacing(style::drawn(space::SECTION_GAP.drawn()));

    for rail in &state.recommendations {
        page = page.push(widget::section(
            widget::prose(rail.heading.as_str(), style::typeface::HEADING_2),
            widget::rail(
                RAIL,
                widget::Rail::of(Construct::ItemsContainer),
                rail.items.iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }

    page.into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut wanted = widget::card_images(&state.suggestions, RAIL.card);
    for rail in &state.recommendations {
        wanted.extend(widget::card_images(&rail.items, RAIL.card));
    }
    wanted
}
