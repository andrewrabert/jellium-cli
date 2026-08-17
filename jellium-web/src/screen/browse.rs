use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, checkbox, column, row, scrollable};
use jellium_model::facets::{SeriesState, VideoKind};
use jellium_model::paged::Paged;
use jellium_model::sort::Sort;
use jellium_model::window;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::style::typeface;
use crate::style::{Drawn, Viewport};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::widget::prose;

/// A windowed grid of items: the library grid, search results, a hub's filtered
/// list, a collection's contents, and either top-level destination.
#[derive(Debug, Clone)]
pub struct Browse {
    /// What the heading names, above the total item count.
    pub heading: String,
    pub listing: Listing,
    pub grid: window::Grid,
    pub items: Paged<BaseItemDto>,
    /// The filter choices the server offered for this parent.
    pub choices: Choices,
    /// True while the filter surface is open.
    pub filtering: bool,
    /// Where the grid rested under each sort visited, so returning to a sort
    /// returns to its place.
    rested: Vec<(Sort, Drawn)>,
}

/// One facet value: the id every query carries and the name every control
/// shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Named {
    pub id: Uuid,
    pub name: String,
}

/// What the filter surface offers, from the server's filter listing scoped to
/// the parent library and from its studio listing.
#[derive(Debug, Clone, Default)]
pub struct Choices {
    pub genres: Vec<Named>,
    pub official_ratings: Vec<String>,
    pub years: Vec<i32>,
    pub tags: Vec<String>,
    pub studios: Vec<Named>,
    /// True on a library holding series, which is what offers series status.
    pub series: bool,
}

/// One control on the shared sort and filter surfaces.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Sorted(Sort),
    Narrowed(Narrow),
    ClearFilters,
    OpenFilters,
    CloseFilters,
    /// One initial of the letter jump.
    Jumped(char),
}

/// One filter control's change.
#[derive(Debug, Clone, PartialEq)]
pub enum Narrow {
    Played(Option<bool>),
    Resumable(bool),
    Favorite(bool),
    Genre(Uuid, bool),
    Studio(Uuid, bool),
    OfficialRating(String, bool),
    Year(i32, bool),
    Tag(String, bool),
    HasSubtitles(bool),
    Hd(bool),
    Uhd(bool),
    Video(VideoKind, bool),
    Series(SeriesState, bool),
}

/// Adds or removes `value`, keeping the list a set.
fn toggled<T: PartialEq>(held: &mut Vec<T>, value: T, on: bool) {
    held.retain(|other| *other != value);
    if on {
        held.push(value);
    }
}

impl Browse {
    pub fn new(id: window::Id, heading: String, listing: Listing, viewport: Viewport) -> Browse {
        Browse {
            heading,
            listing,
            grid: window::Grid::new(
                id,
                Drawn::of(theme::CARD_WIDTH + theme::CARD_SPACING),
                Drawn::of(theme::CARD_HEIGHT),
                viewport.canvas(),
            ),
            items: Paged::new(0),
            choices: Choices::default(),
            filtering: false,
            rested: Vec::new(),
        }
    }

    /// The page the window wants that is neither held nor in flight.
    pub fn wanted(&self) -> Option<std::ops::Range<usize>> {
        self.items.wanted(self.grid.built(self.items.len()))
    }

    pub fn began(&mut self, page: std::ops::Range<usize>) {
        self.items.began(page);
    }

    pub fn filled(&mut self, page: std::ops::Range<usize>, items: Vec<BaseItemDto>) {
        self.items.filled(page, items);
    }

    /// Drops the rows `page` covers so the next fetch re-reads them; the grid's
    /// offset and the listing both stand.
    pub fn forget(&mut self, page: std::ops::Range<usize>) {
        self.items.forget(page);
    }

    pub fn scrolled(&mut self, scrolled: window::Scrolled) {
        self.grid.scrolled(scrolled);
    }

    pub fn resized(&mut self, viewport: Viewport) {
        self.grid.resized(viewport.canvas());
    }

    /// Records where the grid rests under the sort shown and answers the offset
    /// `sort` last rested at, which is what a sort change restores.
    pub fn resorting(&mut self, sort: Sort) -> Option<Drawn> {
        let leaving = self.listing.sort;
        let offset = self.grid.offset();
        toggled(&mut self.rested, (leaving, offset), true);
        self.rested
            .iter()
            .find(|(held, _)| *held == sort)
            .map(|(_, offset)| *offset)
    }

    /// Applies one narrowing, which is what re-reads the surface from the top.
    pub fn narrow(&mut self, narrow: Narrow) {
        let facets = &mut self.listing.facets;
        match narrow {
            Narrow::Played(played) => facets.played = played,
            Narrow::Resumable(on) => facets.resumable = on,
            Narrow::Favorite(on) => facets.favorite = on,
            Narrow::Genre(id, on) => toggled(&mut facets.genres, id, on),
            Narrow::Studio(id, on) => toggled(&mut facets.studios, id, on),
            Narrow::OfficialRating(rating, on) => {
                toggled(&mut facets.official_ratings, rating, on);
            }
            Narrow::Year(year, on) => toggled(&mut facets.years, year, on),
            Narrow::Tag(tag, on) => toggled(&mut facets.tags, tag, on),
            Narrow::HasSubtitles(on) => facets.has_subtitles = on,
            Narrow::Hd(on) => facets.hd = on,
            Narrow::Uhd(on) => facets.uhd = on,
            Narrow::Video(kind, on) => toggled(&mut facets.video_kinds, kind, on),
            Narrow::Series(state, on) => toggled(&mut facets.series_states, state, on),
        }
    }
}

/// The label one sort is shown under.
pub fn sort_label(sort: Sort) -> Text {
    match sort {
        Sort::Name => Text::SortName,
        Sort::NameDescending => Text::SortNameDescending,
        Sort::DateAdded => Text::SortDateAdded,
        Sort::ReleaseDate => Text::SortReleaseDate,
        Sort::CommunityRating => Text::SortCommunityRating,
        Sort::Random => Text::SortRandom,
    }
}

fn sort_surface<'a>(listing: &Listing) -> Element<'a, Message> {
    let controls = Sort::ALL.into_iter().map(|sort| {
        let mut control = button(prose(
            strings::lookup(sort_label(sort)).to_owned(),
            typeface::BODY,
        ));
        if sort != listing.sort {
            control = control.on_press(Message::BrowseAction(Action::Sorted(sort)));
        }
        control.into()
    });
    row![
        prose(
            strings::lookup(Text::LibrarySort).to_owned(),
            typeface::BODY
        ),
        row(controls).spacing(theme::CARD_SPACING),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center)
    .into()
}

fn letter_jump<'a>() -> Element<'a, Message> {
    row(window::LETTERS.into_iter().map(|letter| {
        button(prose(letter.to_string(), typeface::BODY))
            .style(button::text)
            .on_press(Message::BrowseAction(Action::Jumped(letter)))
            .into()
    }))
    .spacing(4)
    .into()
}

fn narrowing<'a>(label: String, on: bool, narrow: Narrow) -> Element<'a, Message> {
    row![
        checkbox(on).on_toggle(move |on| {
            let mut narrow = narrow.clone();
            match &mut narrow {
                Narrow::Played(played) => *played = on.then_some(true),
                Narrow::Resumable(held)
                | Narrow::Favorite(held)
                | Narrow::HasSubtitles(held)
                | Narrow::Hd(held)
                | Narrow::Uhd(held) => *held = on,
                Narrow::Genre(_, held)
                | Narrow::Studio(_, held)
                | Narrow::OfficialRating(_, held)
                | Narrow::Year(_, held)
                | Narrow::Tag(_, held)
                | Narrow::Video(_, held)
                | Narrow::Series(_, held) => *held = on,
            }
            Message::BrowseAction(Action::Narrowed(narrow))
        }),
        prose(label, typeface::BODY),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center)
    .into()
}

fn filter_surface<'a>(browse: &'a Browse) -> Element<'a, Message> {
    let facets = &browse.listing.facets;
    let choices = &browse.choices;

    let mut surface = column![
        row![
            button(prose(
                strings::lookup(Text::FilterClose).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::BrowseAction(Action::CloseFilters)),
            button(prose(
                strings::lookup(Text::FilterClear).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::BrowseAction(Action::ClearFilters)),
        ]
        .spacing(theme::CARD_SPACING),
        narrowing(
            strings::lookup(Text::FilterPlayed).to_string(),
            facets.played == Some(true),
            Narrow::Played(None),
        ),
        narrowing(
            strings::lookup(Text::FilterResumable).to_string(),
            facets.resumable,
            Narrow::Resumable(false),
        ),
        narrowing(
            strings::lookup(Text::FilterFavorite).to_string(),
            facets.favorite,
            Narrow::Favorite(false),
        ),
        narrowing(
            strings::lookup(Text::FilterHasSubtitles).to_string(),
            facets.has_subtitles,
            Narrow::HasSubtitles(false),
        ),
        narrowing(
            strings::lookup(Text::FilterHd).to_string(),
            facets.hd,
            Narrow::Hd(false),
        ),
        narrowing(
            strings::lookup(Text::FilterUhd).to_string(),
            facets.uhd,
            Narrow::Uhd(false),
        ),
    ]
    .spacing(8);

    for kind in VideoKind::ALL {
        surface = surface.push(narrowing(
            kind.query().to_owned(),
            facets.video_kinds.contains(&kind),
            Narrow::Video(kind, false),
        ));
    }
    if choices.series {
        for state in SeriesState::ALL {
            surface = surface.push(narrowing(
                state.query().to_owned(),
                facets.series_states.contains(&state),
                Narrow::Series(state, false),
            ));
        }
    }
    for genre in &choices.genres {
        surface = surface.push(narrowing(
            genre.name.clone(),
            facets.genres.contains(&genre.id),
            Narrow::Genre(genre.id, false),
        ));
    }
    for studio in &choices.studios {
        surface = surface.push(narrowing(
            studio.name.clone(),
            facets.studios.contains(&studio.id),
            Narrow::Studio(studio.id, false),
        ));
    }
    for rating in &choices.official_ratings {
        surface = surface.push(narrowing(
            rating.clone(),
            facets.official_ratings.contains(rating),
            Narrow::OfficialRating(rating.clone(), false),
        ));
    }
    for year in &choices.years {
        surface = surface.push(narrowing(
            year.to_string(),
            facets.years.contains(year),
            Narrow::Year(*year, false),
        ));
    }
    for tag in &choices.tags {
        surface = surface.push(narrowing(
            tag.clone(),
            facets.tags.contains(tag),
            Narrow::Tag(tag.clone(), false),
        ));
    }

    scrollable(surface).height(theme::RAIL_HEIGHT).into()
}

/// The heading with the total item count, the sort surface, the filter surface
/// with its active count and its one clear, the letter jump on a name sort
/// alone, and the windowed grid.
pub fn view<'a>(browse: &'a Browse, images: &'a Cache, read_only: bool) -> Element<'a, Message> {
    let active = browse.listing.facets.count();
    let mut page = column![
        prose(browse.heading.clone(), typeface::HEADING_1),
        prose(
            strings::format(Text::BrowseTotal, &[&browse.items.len().to_string()],),
            typeface::BODY
        ),
        sort_surface(&browse.listing),
        button(prose(
            strings::format(Text::FilterOpen, &[&active.to_string()],),
            typeface::BODY
        ))
        .on_press(Message::BrowseAction(Action::OpenFilters)),
    ]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING);

    if browse.filtering {
        page = page.push(filter_surface(browse));
    }
    if browse.listing.sort.by_name() {
        page = page.push(letter_jump());
    }

    let count = browse.items.len();
    page = page.push(crate::window::grid(
        browse.grid,
        count,
        move |index| match browse.items.row(index) {
            Some(item) => widget::card(
                item,
                widget::poster_key(item).and_then(|key| images.handle(key)),
                !read_only,
            ),
            None => iced::widget::Space::new()
                .width(theme::CARD_WIDTH)
                .height(theme::CARD_HEIGHT)
                .into(),
        },
    ));
    page.into()
}

pub fn images(browse: &Browse) -> HashSet<images::Key> {
    let shown = browse.grid.shown(browse.items.len());
    shown
        .filter_map(|index| browse.items.row(index))
        .filter_map(widget::poster_key)
        .collect()
}

/// One page of a browse surface, and the total the server reports.
/// `term` is what search results are paged by; every other surface names none.
pub async fn page(
    api: Rc<Api>,
    parent: Option<Uuid>,
    term: Option<String>,
    listing: Listing,
    page: std::ops::Range<usize>,
) -> Answer<(Vec<BaseItemDto>, usize)> {
    Answer::of(async {
        let answered = api
            .browse(
                parent,
                term.as_deref(),
                &listing,
                page.start as i32,
                page.len() as i32,
            )
            .await
            .bubbled()?;
        Ok((answered.items, answered.total.max(0) as usize))
    })
    .await
}
