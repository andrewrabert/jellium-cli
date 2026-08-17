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
use crate::style::{Drawn, Viewport};
use crate::theme;
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
    Answer::of(async {
        let answered = api
            .hub(facet, library, sort, 0, Paged::<BaseItemDto>::PAGE as i32)
            .await
            .bubbled()?;
        let mut entries = Paged::new(answered.total.max(0) as usize);
        entries.filled(0..answered.items.len(), answered.items);

        Ok(State {
            facet,
            library,
            sort,
            grid: window::Grid::new(
                window::Id::Browse,
                Drawn::of(theme::CARD_WIDTH + theme::CARD_SPACING),
                Drawn::of(theme::CARD_HEIGHT),
                viewport.canvas(),
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
pub fn view<'a>(state: &'a State, images: &'a Cache) -> Element<'a, Message> {
    let count = state.entries.len();
    column![crate::window::grid(state.grid, count, move |index| {
        match state.entries.row(index) {
            Some(entry) => {
                let card = widget::card(
                    entry,
                    widget::poster_key(entry).and_then(|key| images.handle(key)),
                    false,
                );
                match opens(state, entry) {
                    Some(route) => button(card)
                        .style(button::text)
                        .on_press(Message::Navigated(route))
                        .into(),
                    None => card,
                }
            }
            None => iced::widget::Space::new()
                .width(theme::CARD_WIDTH)
                .height(theme::CARD_HEIGHT)
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
