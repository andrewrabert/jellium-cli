use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column};
use jellium_model::facets::{Facet, Facets};
use jellium_model::paged::Paged;
use jellium_model::sort::Sort;
use jellium_model::window;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::{Filtered, Listing, Route};
use crate::style::card::{Aspect, Card};
use crate::style::space::Room;
use crate::style::{self, Viewport, card};
use crate::widget;

/// A hub: the values one facet takes across the library that owns it.
#[derive(Debug, Clone)]
pub struct State {
    pub facet: Facet,
    pub library: Uuid,
    pub sort: Sort,
    pub grid: window::Grid,
    pub entries: Paged<BaseItemDto>,
}

impl State {
    /// The page the window wants that is neither held nor in flight.
    pub fn wanted(&self) -> Option<std::ops::Range<usize>> {
        self.entries.wanted(self.grid.built(self.entries.len()))
    }
}

pub async fn load(
    api: Rc<Api>,
    facet: Facet,
    library: Uuid,
    sort: Sort,
    viewport: Viewport,
) -> Answer<State> {
    let room = Room::content(viewport);
    Answer::of(async move {
        let answered = api
            .hub(facet, library, sort, 0, Paged::<BaseItemDto>::PAGE as i32)
            .await
            .bubbled()?;
        let mut entries = Paged::new(answered.total.max(0) as usize);
        entries.filled(0..answered.items.len(), answered.items);
        let wall = Card::grid(
            None,
            Aspect::shared(
                entries
                    .held()
                    .filter_map(|item| item.primary_image_aspect_ratio)
                    .map(Aspect::of),
            ),
        );

        Ok(State {
            facet,
            library,
            sort,
            grid: window::Grid::new(
                window::Id::Browse,
                wall.width(room),
                wall.row(room, card::Footer::NameAndSubtitle, card::Bottom::Padded),
                room,
            ),
            entries,
        })
    })
    .await
}

/// One page of a hub's entries, and the total the server reports.
pub async fn page(
    api: Rc<Api>,
    facet: Facet,
    library: Uuid,
    sort: Sort,
    page: std::ops::Range<usize>,
) -> Answer<(Vec<BaseItemDto>, usize)> {
    Answer::of(async {
        let answered = api
            .hub(facet, library, sort, page.start as i32, page.len() as i32)
            .await
            .bubbled()?;
        Ok((answered.items, answered.total.max(0) as usize))
    })
    .await
}

/// The filtered list one hub entry opens: that value's items, narrowed by id.
pub fn opens(state: &State, entry: &BaseItemDto) -> Option<Route> {
    let id = entry.id?;
    Some(Route::Filtered(Box::new(Filtered {
        library: Some(state.library),
        header: Some(id),
        listing: Listing {
            sort: Sort::default(),
            facets: Facets::of(state.facet, id),
        },
    })))
}

/// Each entry opens the filtered list narrowed to that value by id.
pub fn view<'a>(state: &'a State, viewport: Viewport, images: &'a Cache) -> Element<'a, Message> {
    let wall = Card::grid(
        None,
        Aspect::shared(
            state
                .entries
                .held()
                .filter_map(|item| item.primary_image_aspect_ratio)
                .map(Aspect::of),
        ),
    );
    let count = state.entries.len();
    column![crate::window::grid(state.grid, count, move |index| {
        match state.entries.row(index) {
            Some(entry) => {
                let card = widget::poster(
                    wall,
                    entry,
                    Room::content(viewport),
                    card::Footer::NameAndSubtitle,
                    card::Bottom::Padded,
                    widget::poster_key(entry).and_then(|key| images.handle(key)),
                    widget::Overflow::Withheld,
                );
                match opens(state, entry) {
                    Some(route) => button(card)
                        .style(style::flat)
                        .on_press(Message::Navigated(route))
                        .into(),
                    None => card,
                }
            }
            None => iced::widget::Space::new()
                .width(style::drawn(wall.width(Room::content(viewport))))
                .height(style::drawn(wall.row(
                    Room::content(viewport),
                    card::Footer::NameAndSubtitle,
                    card::Bottom::Padded,
                )))
                .into(),
        }
    })]
    .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    state
        .grid
        .shown(state.entries.len())
        .filter_map(|index| state.entries.row(index))
        .filter_map(widget::poster_key)
        .collect()
}
