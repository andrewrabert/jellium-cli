use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space};
use crate::text::{self as strings, Text};
use crate::widget;

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

pub async fn load(api: Rc<Api>, library: Uuid, movies: bool) -> Answer<State> {
    Answer::of(async {
        let suggestions = api
            .suggestions(LIMIT)
            .await
            .or_default(Text::FailureSuggestionsUnread);
        let recommendations = if movies {
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
    overflow: widget::Overflow,
) -> Element<'a, Message> {
    let mut page = column![widget::section(
        strings::lookup(Text::LibraryTabSuggestions),
        widget::rail(
            card::Card::Rail(card::Rail::Portrait),
            state.suggestions.iter(),
            Room::content(viewport),
            images,
            overflow,
        ),
    )]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for rail in &state.recommendations {
        page = page.push(widget::section(
            rail.heading.as_str(),
            widget::rail(
                card::Card::Rail(card::Rail::Portrait),
                rail.items.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
        ));
    }

    page.into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut wanted = widget::card_images(&state.suggestions);
    for rail in &state.recommendations {
        wanted.extend(widget::card_images(&rail.items));
    }
    wanted
}
