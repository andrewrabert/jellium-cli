use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::facets::Facets;
use jellium_model::search::Section;
use jellium_model::sort::Sort;
use jellium_model::window;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::style::card::{Aspect, Card, Rail};
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    pub term: String,
    /// Each section that answered with items, in the reference's order.
    pub sections: Vec<Results>,
    /// What the page offers while the term is empty.
    pub suggestions: Vec<BaseItemDto>,
}

impl State {
    /// Takes the offsets the sections of a search on the same term were
    /// resting at, so a re-read leaves every rail where it stood.
    pub fn rested(&mut self, previous: &State) {
        for results in &mut self.sections {
            let Some(stood) = previous
                .sections
                .iter()
                .find(|earlier| earlier.section == results.section)
            else {
                continue;
            };
            results.window.moved(stood.window.offset());
        }
    }
}

/// One section's items and the window over them.
#[derive(Debug, Clone)]
pub struct Results {
    pub section: Section,
    pub items: Vec<BaseItemDto>,
    pub window: window::Window,
}

/// The most entries one section holds, which is the reference's own limit.
// reference: search-limit
pub const SECTION: i32 = 100;

/// The suggestions an empty term draws.
// reference: search-suggestions-listed
pub const SUGGESTIONS: i32 = 20;

/// The most items one search reads across every kind at once.
// reference: search-sections-listed
pub const RESULTS: i32 = 800;

pub async fn load(api: Rc<Api>, term: String, viewport: Viewport) -> Answer<State> {
    Answer::of(async {
        if term.trim().is_empty() {
            let suggestions = api
                .suggestions(SUGGESTIONS)
                .await
                .or_default(Text::FailureSuggestionsUnread);
            return Ok(State {
                term,
                sections: Vec::new(),
                suggestions,
            });
        }

        let listed = api
            .browse(None, Some(&term), &Listing::default(), 0, RESULTS)
            .await
            .bubbled()?
            .items;
        let people = api
            .people(&term, SECTION)
            .await
            .or_default(Text::FailurePeopleUnread);
        let studios = api
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
            .collect::<Vec<BaseItemDto>>();
        let programs = api
            .browse(
                None,
                Some(&term),
                &Listing {
                    sort: Sort::default(),
                    facets: Facets::of_kind(BaseItemKind::Program),
                },
                0,
                SECTION,
            )
            .await
            .map(|page| page.items)
            .or_default(Text::FailureLatestUnread);

        let sections = Section::ALL
            .into_iter()
            .filter_map(|section| {
                let items: Vec<BaseItemDto> = match section.kind() {
                    Some(kind) => listed
                        .iter()
                        .filter(|item| item.type_ == Some(kind))
                        .take(SECTION as usize)
                        .cloned()
                        .collect(),
                    None => match section {
                        Section::People => people.clone(),
                        Section::Studios => studios.clone(),
                        Section::Programs => programs.clone(),
                        _ => Vec::new(),
                    },
                };
                (!items.is_empty()).then(|| Results {
                    section,
                    window: windowed(section, &items, viewport),
                    items,
                })
            })
            .collect();

        Ok(State {
            term,
            sections,
            suggestions: Vec::new(),
        })
    })
    .await
}

/// The window one section's row scrolls, measured against the room the page
/// lays its cards in.
fn windowed(section: Section, items: &[BaseItemDto], viewport: Viewport) -> window::Window {
    let room = Room::content(viewport);
    window::Window::new(
        window::Id::Section(section),
        card(section, shared(items)).width(room),
        room.width(),
    )
}

/// The aspect a section's items share.
fn shared(items: &[BaseItemDto]) -> Option<Aspect> {
    Aspect::shared(
        items
            .iter()
            .filter_map(|item| item.primary_image_aspect_ratio)
            .map(Aspect::of),
    )
}

/// The heading this section writes.
// reference: search-section-title
pub fn title(section: Section) -> Text {
    match section {
        Section::Movies => Text::SearchMovies,
        Section::Shows => Text::SearchShows,
        Section::Episodes => Text::SearchEpisodes,
        Section::People => Text::SearchPeople,
        Section::Playlists => Text::SearchPlaylists,
        Section::Artists => Text::SearchArtists,
        Section::Albums => Text::SearchAlbums,
        Section::Songs => Text::SearchSongs,
        Section::Videos => Text::SearchVideos,
        Section::Programs => Text::SearchPrograms,
        Section::Channels => Text::SearchChannels,
        Section::PhotoAlbums => Text::SearchPhotoAlbums,
        Section::Photos => Text::SearchPhotos,
        Section::AudioBooks => Text::SearchAudioBooks,
        Section::Books => Text::SearchBooks,
        Section::Collections => Text::SearchCollections,
        Section::Studios => Text::SearchStudios,
    }
}

/// The card this section's items draw on, which is the shape they share except
/// where the reference fixes it.
// reference: search-section-cards
pub fn card(section: Section, aspect: Option<Aspect>) -> Card {
    match section {
        Section::Songs => Card::Rail(Rail::Square),
        _ => Card::overflowing(aspect),
    }
}

/// What this section writes under a card.
// reference: search-section-cards
pub fn footer(section: Section) -> card::Footer {
    match section {
        Section::Movies
        | Section::Shows
        | Section::Albums
        | Section::Episodes
        | Section::Songs
        | Section::Videos
        | Section::Programs => card::Footer::NameAndSubtitle,
        Section::People
        | Section::Playlists
        | Section::Artists
        | Section::Channels
        | Section::PhotoAlbums
        | Section::Photos
        | Section::AudioBooks
        | Section::Books
        | Section::Collections
        | Section::Studios => card::Footer::Name,
    }
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

/// One section's row: its heading over the windowed rail of its items.
// reference: search-results
fn sectioned<'a>(
    results: &'a Results,
    viewport: Viewport,
    images: &'a Cache,
) -> Element<'a, Message> {
    let room = Room::content(viewport);
    let drawn = card(results.section, shared(&results.items));
    let rail = crate::window::rail(
        results.window,
        results.items.len(),
        move |index| match results.items.get(index) {
            Some(item) => widget::poster(
                drawn,
                item,
                room,
                footer(results.section),
                card::Bottom::Flush,
                widget::poster_key(item).and_then(|key| images.handle(key)),
                widget::Overflow::Withheld,
            ),
            None => iced::widget::Space::new()
                .width(style::drawn(drawn.width(room)))
                .into(),
        },
    );
    widget::section(
        strings::lookup(title(results.section)),
        iced::widget::container(rail)
            .height(style::drawn(drawn.row(
                room,
                footer(results.section),
                card::Bottom::Flush,
            )))
            .into(),
    )
}

pub fn view<'a>(state: &'a State, viewport: Viewport, images: &'a Cache) -> Element<'a, Message> {
    let mut page = column![widget::searching(&state.term, viewport)]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .padding(style::padding(space::PAGE_PAD));

    if state.term.trim().is_empty() {
        return page.push(suggesting(&state.suggestions)).into();
    }

    // reference: search-empty
    if state.sections.is_empty() {
        return page
            .push(widget::centered(strings::format(
                Text::SearchResultsEmpty,
                &[&state.term],
            )))
            .into();
    }

    for results in &state.sections {
        page = page.push(sectioned(results, viewport, images));
    }
    page.into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .sections
        .iter()
        .flat_map(|results| {
            results
                .window
                .shown(results.items.len())
                .filter_map(|index| results.items.get(index))
                .filter_map(widget::poster_key)
        })
        .collect()
}
