use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::paged::Paged;
use jellium_model::window;
use jellyfin_api::types::BaseItemDto;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::screen::browse::{self, Browse};
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    pub term: String,
    pub browse: Browse,
    /// A bounded strip rather than a windowed grid; each entry opens that
    /// person's filtered list.
    pub people: Vec<BaseItemDto>,
    /// Each entry opens that studio's filtered list.
    pub studios: Vec<BaseItemDto>,
    pub programs: Vec<BaseItemDto>,
    /// What the page offers while the term is empty.
    pub suggestions: Vec<BaseItemDto>,
}

/// The most entries one search section shows.
pub const SECTION: i32 = 12;

/// The suggestions an empty term draws.
// reference: search-suggestions-listed
pub const SUGGESTIONS: i32 = 20;

pub async fn load(
    api: Rc<Api>,
    term: String,
    listing: Listing,
    viewport: Viewport,
    overflow: widget::Overflow,
) -> Answer<State> {
    Answer::of(async {
        let heading = strings::lookup(Text::NavSearch).to_string();
        let mut browse = Browse::new(
            window::Id::Browse,
            heading,
            listing.clone(),
            None,
            viewport,
            overflow,
        );

        let mut people = Vec::new();
        let mut studios = Vec::new();
        let mut programs = Vec::new();
        let mut suggestions = Vec::new();

        if term.trim().is_empty() {
            suggestions = api
                .suggestions(SUGGESTIONS)
                .await
                .or_default(Text::FailureSuggestionsUnread);
        } else {
            let answered = api
                .browse(
                    None,
                    Some(&term),
                    &listing,
                    0,
                    Paged::<BaseItemDto>::PAGE as i32,
                )
                .await
                .bubbled()?;
            browse.items = Paged::new(answered.total.max(0) as usize);
            browse.filled(0..answered.items.len(), answered.items);

            people = api
                .people(&term, SECTION)
                .await
                .or_default(Text::FailurePeopleUnread);
            studios = api
                .studios(None)
                .await
                .or_default(Text::FailureStudiosUnread)
                .into_iter()
                .filter(|studio| {
                    studio
                        .name
                        .as_deref()
                        .is_some_and(|name| name.to_lowercase().contains(&term.to_lowercase()))
                })
                .take(SECTION as usize)
                .collect();
            programs = api
                .browse(
                    None,
                    Some(&term),
                    &Listing {
                        sort: listing.sort,
                        facets: jellium_model::facets::Facets::of_kind(
                            jellyfin_api::types::BaseItemKind::Program,
                        ),
                    },
                    0,
                    SECTION,
                )
                .await
                .map(|page| page.items)
                .or_default(Text::FailureLatestUnread);
        }

        Ok(State {
            term,
            browse,
            people,
            studios,
            programs,
            suggestions,
        })
    })
    .await
}

/// One search section: a bounded strip of cards, each opening the route its
/// entry names.
fn section<'a>(
    title: Text,
    items: &'a [BaseItemDto],
    viewport: Viewport,
    images: &'a Cache,
    opens: impl Fn(uuid::Uuid) -> crate::route::Route + 'a,
) -> Element<'a, Message> {
    let cards = items.iter().filter_map(|item| {
        let id = item.id?;
        Some(
            iced::widget::button(widget::poster(
                card::Card::Wall(card::Shape::Portrait),
                item,
                Room::content(viewport),
                widget::poster_key(item).and_then(|key| images.handle(key)),
                widget::Overflow::Withheld,
            ))
            .style(style::flat)
            .on_press(Message::Navigated(opens(id)))
            .into(),
        )
    });
    column![
        prose(strings::lookup(title), typeface::HEADING_2),
        iced::widget::scrollable(
            iced::widget::row(cards).spacing(style::drawn(space::GUTTER.drawn()))
        )
        .direction(iced::widget::scrollable::Direction::Horizontal(
            iced::widget::scrollable::Scrollbar::default()
        )),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .into()
}

/// The filtered list one facet value opens, across the server.
fn narrowed(facet: jellium_model::facets::Facet, id: uuid::Uuid) -> crate::route::Route {
    crate::route::Route::Filtered(Box::new(crate::route::Filtered {
        library: None,
        header: Some(id),
        listing: Listing {
            sort: jellium_model::sort::Sort::default(),
            facets: jellium_model::facets::Facets::of(facet, id),
        },
    }))
}

/// The column an empty term draws: the reference's heading over one centred
/// link per suggestion, each padded as the reference pads it.
// reference: search-suggestions
fn suggesting(suggestions: &[BaseItemDto]) -> Element<'_, Message> {
    let links = suggestions.iter().filter_map(|item| {
        let id = item.id?;
        Some(
            iced::widget::button(prose(item.name.clone().unwrap_or_default(), typeface::BODY))
                .style(style::link)
                .padding(style::padding(space::SUGGESTION_PAD))
                .on_press(Message::Navigated(crate::route::Route::Detail { id }))
                .into(),
        )
    });
    column![
        prose(
            strings::lookup(Text::SearchSuggestions),
            typeface::HEADING_2
        ),
        column(links).align_x(iced::Alignment::Center),
    ]
    .align_x(iced::Alignment::Center)
    .width(iced::Fill)
    .into()
}

pub fn view<'a>(state: &'a State, viewport: Viewport, images: &'a Cache) -> Element<'a, Message> {
    let mut page = column![widget::searching(&state.term, viewport)]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    if state.term.trim().is_empty() {
        return page.push(suggesting(&state.suggestions)).into();
    }

    if state.browse.items.is_empty() {
        page = page.push(widget::banner(
            strings::lookup(Text::SearchEmpty).to_string(),
        ));
        return page.into();
    }

    page = page.push(browse::view(&state.browse, viewport, images));

    if !state.people.is_empty() {
        page = page.push(section(
            Text::SearchPeople,
            &state.people,
            viewport,
            images,
            |id| narrowed(jellium_model::facets::Facet::Person, id),
        ));
    }
    if !state.studios.is_empty() {
        page = page.push(section(
            Text::SearchStudios,
            &state.studios,
            viewport,
            images,
            |id| narrowed(jellium_model::facets::Facet::Studio, id),
        ));
    }
    if !state.programs.is_empty() {
        page = page.push(section(
            Text::SearchPrograms,
            &state.programs,
            viewport,
            images,
            |id| crate::route::Route::Detail { id },
        ));
    }

    page.into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut wanted = browse::images(&state.browse);
    wanted.extend(widget::card_images(&state.people));
    wanted.extend(widget::card_images(&state.studios));
    wanted.extend(widget::card_images(&state.programs));
    wanted
}
