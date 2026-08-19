use std::rc::Rc;

use iced::Element;
use iced::widget::{button, checkbox, column, row};
use jellium_model::facets::{SeriesState, VideoKind};
use jellium_model::paged::Paged;
use jellium_model::sort::Sort;
use jellium_model::window;
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, CollectionType};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::style::card::{Aspect, Card};
use crate::style::space::Room;
use crate::style::{self, Drawn, Letters, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
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
    /// The surface the bar has open, and `None` while it has none.
    pub opened: Option<Opened>,
    /// The library this grid is of, which decides the card it draws.
    collection: Option<CollectionType>,
    /// The card every cell draws, which the items' own aspect settles for a
    /// library whose controller writes no shape.
    card: card::Drawing,
    /// The page these measurements were taken in.
    viewport: Viewport,
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

/// Which surface the bar has open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opened {
    Sort,
    Filters,
}

/// One control on the shared sort and filter surfaces.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Sorted(Sort),
    Narrowed(Narrow),
    ClearFilters,
    Open(Opened),
    Close,
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

/// Whether a value stays in the set or leaves it.
pub enum Membership {
    Held,
    Dropped,
}

/// Adds or removes `value`, keeping the list a set.
fn toggled<T: PartialEq>(held: &mut Vec<T>, value: T, membership: Membership) {
    held.retain(|other| *other != value);
    match membership {
        Membership::Held => held.push(value),
        Membership::Dropped => {}
    }
}

/// The card a library grid draws: the shape the library's own controller
/// writes, over the two lines the grid writes under it.
// reference: grid-card
fn wall(collection: Option<CollectionType>, aspect: Option<Aspect>) -> card::Drawing {
    card::Drawing {
        card: Card::grid(collection, aspect),
        // reference: grid-card-album
        footer: match collection {
            Some(CollectionType::Music) => card::Footer::ParentAndName,
            Some(
                CollectionType::Unknown
                | CollectionType::Movies
                | CollectionType::Tvshows
                | CollectionType::Musicvideos
                | CollectionType::Trailers
                | CollectionType::Homevideos
                | CollectionType::Boxsets
                | CollectionType::Books
                | CollectionType::Photos
                | CollectionType::Livetv
                | CollectionType::Playlists
                | CollectionType::Folders,
            )
            | None => card::Footer::NameAndYear,
        },
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        // reference: grid-card
        touch: card::Touch::Plays,
    }
}

impl Browse {
    pub fn new(
        id: window::Id,
        heading: String,
        listing: Listing,
        collection: Option<CollectionType>,
        viewport: Viewport,
    ) -> Browse {
        let drawn = wall(collection, None);
        let room = Browse::laid(listing.sort, viewport);
        Browse {
            heading,
            listing,
            grid: window::Grid::new(id, drawn.card.width(room), drawn.row(room), room),
            items: Paged::new(0),
            choices: Choices::default(),
            opened: None,
            collection,
            card: drawn,
            viewport,
            rested: Vec::new(),
        }
    }

    /// The room a grid sorted by `sort` lays its cards in at `viewport`: the
    /// content box, less the letter picker's own reserve where the sort draws
    /// the picker.
    fn laid(sort: Sort, viewport: Viewport) -> Room {
        match sort.by_name() {
            true => Room::lettered(viewport),
            false => Room::content(viewport),
        }
    }

    /// The room this grid's cards are laid in, which is what the window counts
    /// its columns in.
    pub fn room(&self) -> Room {
        Browse::laid(self.listing.sort, self.viewport)
    }

    /// The card every cell of this grid draws.
    pub fn card(&self) -> card::Drawing {
        self.card
    }

    /// The rows the window shows, counted from one as the paging sentence
    /// writes them, and `None` while the surface holds no rows.
    // reference: grid-paging
    pub fn showing(&self) -> Option<std::ops::RangeInclusive<usize>> {
        let shown = self.grid.shown(self.items.len());
        (!shown.is_empty()).then(|| shown.start + 1..=shown.end)
    }

    /// Relays the grid at the card and the page standing now.
    fn relaid(&mut self) {
        let room = self.room();
        self.grid
            .resized(room, self.card.card.width(room), self.card.row(room));
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
        self.card = wall(
            self.collection,
            Aspect::shared(
                self.items
                    .held()
                    .filter_map(|item| item.primary_image_aspect_ratio)
                    .map(Aspect::of),
            ),
        );
        self.relaid();
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
        self.viewport = viewport;
        self.relaid();
    }

    /// Records where the grid rests under the sort shown and answers the offset
    /// `sort` last rested at, which is what a sort change restores.
    pub fn resorting(&mut self, sort: Sort) -> Option<Drawn> {
        let leaving = self.listing.sort;
        let offset = self.grid.offset();
        toggled(&mut self.rested, (leaving, offset), Membership::Held);
        self.rested
            .iter()
            .find(|(held, _)| *held == sort)
            .map(|(_, offset)| *offset)
    }

    /// Applies one narrowing, which is what re-reads the surface from the top.
    pub fn narrow(&mut self, narrow: Narrow) {
        let facets = &mut self.listing.facets;
        let membership = |on: bool| {
            if on {
                Membership::Held
            } else {
                Membership::Dropped
            }
        };
        match narrow {
            Narrow::Played(played) => facets.played = played,
            Narrow::Resumable(on) => facets.resumable = on,
            Narrow::Favorite(on) => facets.favorite = on,
            Narrow::Genre(id, on) => toggled(&mut facets.genres, id, membership(on)),
            Narrow::Studio(id, on) => {
                toggled(&mut facets.studios, id, membership(on));
            }
            Narrow::OfficialRating(rating, on) => {
                toggled(&mut facets.official_ratings, rating, membership(on));
            }
            Narrow::Year(year, on) => {
                toggled(&mut facets.years, year, membership(on));
            }
            Narrow::Tag(tag, on) => toggled(&mut facets.tags, tag, membership(on)),
            Narrow::HasSubtitles(on) => facets.has_subtitles = on,
            Narrow::Hd(on) => facets.hd = on,
            Narrow::Uhd(on) => facets.uhd = on,
            Narrow::Video(kind, on) => {
                toggled(&mut facets.video_kinds, kind, membership(on));
            }
            Narrow::Series(state, on) => {
                toggled(&mut facets.series_states, state, membership(on));
            }
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
        let mut control =
            button(prose(strings::lookup(sort_label(sort)), typeface::BODY)).style(style::flat);
        if sort != listing.sort {
            control = control.on_press(Message::BrowseAction(Action::Sorted(sort)));
        }
        control.into()
    });
    row![
        prose(strings::lookup(Text::LibrarySort), typeface::BODY),
        row(controls).spacing(style::drawn(space::CONTROL_GAP.drawn())),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

/// The reference's own paging bar: the sentence naming the first row shown, the
/// last and the total, then the two controls opening the sort surface and the
/// filter surface, the filter control carrying the indicator while filters
/// narrow the grid.
// reference: grid-paging
// reference: grid-paging-sort
// reference: grid-paging-filter
fn paging<'a>(browse: &Browse) -> Element<'a, Message> {
    let (first, last) = match browse.showing() {
        Some(rows) => (*rows.start(), *rows.end()),
        None => (0, 0),
    };
    let sentence = strings::format(
        Text::GridPaging,
        &[
            &first.to_string(),
            &last.to_string(),
            &browse.items.len().to_string(),
        ],
    );

    let pressing = |surface: Opened| {
        Message::BrowseAction(match browse.opened == Some(surface) {
            true => Action::Close,
            false => Action::Open(surface),
        })
    };
    let filters = widget::icon_button(
        Icon::FilterAlt,
        typeface::ICON_BUTTON,
        Some(Text::FilterIndicator),
        pressing(Opened::Filters),
    );

    row![
        prose(sentence, typeface::BODY),
        widget::icon_button(
            Icon::SortByAlpha,
            typeface::ICON_BUTTON,
            Some(Text::LibrarySort),
            pressing(Opened::Sort)
        ),
        match browse.listing.facets.is_empty() {
            true => filters,
            false => widget::filtering(filters, browse.viewport.layout()),
        },
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

/// The letter picker this surface lays over the page, and `None` where the sort
/// is not by name or the page is too short to draw it.
// reference: alpha-picker
pub fn letters<'a>(browse: &Browse, viewport: Viewport) -> Option<Element<'a, Message>> {
    if !browse.listing.sort.by_name() || viewport.letters() == Letters::Hidden {
        return None;
    }
    let size = typeface::letters(viewport);
    Some(
        column(window::LETTERS.into_iter().map(|letter| {
            button(prose(letter.to_string(), size))
                .style(style::flat)
                .on_press(Message::BrowseAction(Action::Jumped(letter)))
                .into()
        }))
        .into(),
    )
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
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

fn filter_surface<'a>(browse: &'a Browse, viewport: Viewport) -> Element<'a, Message> {
    let facets = &browse.listing.facets;
    let choices = &browse.choices;

    let mut surface = column![
        row![
            button(prose(strings::lookup(Text::FilterClose), typeface::BODY))
                .style(style::raised)
                .on_press(Message::BrowseAction(Action::Close)),
            button(prose(strings::lookup(Text::FilterClear), typeface::BODY))
                .style(style::raised)
                .on_press(Message::BrowseAction(Action::ClearFilters)),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn())),
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
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

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

    crate::widget::scrolled(surface)
        .height(style::drawn(space::filter_surface(viewport)))
        .into()
}

/// The heading, the paging bar, the surface the bar has open, and the windowed
/// grid.
pub fn view<'a>(
    browse: &'a Browse,
    viewport: Viewport,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    session: &'a Session,
    collection: Option<Uuid>,
) -> Element<'a, Message> {
    let wall = browse.card();
    let room = browse.room();
    let mut page = column![
        prose(browse.heading.clone(), typeface::HEADING_1),
        paging(browse),
    ]
    .spacing(style::drawn(space::SECTION_GAP.drawn()))
    .padding(style::padding(space::PAGE_PAD));

    match browse.opened {
        Some(Opened::Sort) => page = page.push(sort_surface(&browse.listing)),
        Some(Opened::Filters) => page = page.push(filter_surface(browse, viewport)),
        None => {}
    }
    let count = browse.items.len();
    page = page.push(crate::window::grid(
        browse.grid,
        card::Wrap::Leading,
        count,
        move |index| match browse.items.row(index) {
            Some(item) => widget::poster(wall, item, room, images, now, session, collection),
            None => iced::widget::Space::new()
                .width(style::drawn(wall.card.width(room)))
                .height(style::drawn(wall.row(room)))
                .into(),
        },
    ));
    page.into()
}

pub fn images(browse: &Browse) -> images::Wanted {
    let wall = browse.card();
    let shown = browse.grid.shown(browse.items.len());
    shown
        .filter_map(|index| browse.items.row(index))
        .filter_map(|item| widget::posted(item, wall.card))
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
