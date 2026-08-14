use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{column, pick_list, row, text};
use jellyfin_api::types::{BaseItemDto, ItemSortBy, SortOrder};
use uuid::Uuid;

use crate::api::{Api, Page};
use crate::app::Message;
use crate::error::Trouble;
use crate::images::{self, Cache};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

pub const PAGE_SIZE: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sort {
    #[default]
    Name,
    NameDescending,
    DateAdded,
    ReleaseDate,
    CommunityRating,
    Random,
}

impl Sort {
    pub const ALL: [Sort; 6] = [
        Sort::Name,
        Sort::NameDescending,
        Sort::DateAdded,
        Sort::ReleaseDate,
        Sort::CommunityRating,
        Sort::Random,
    ];

    pub fn query(self) -> (ItemSortBy, SortOrder) {
        match self {
            Sort::Name => (ItemSortBy::SortName, SortOrder::Ascending),
            Sort::NameDescending => (ItemSortBy::SortName, SortOrder::Descending),
            Sort::DateAdded => (ItemSortBy::DateCreated, SortOrder::Descending),
            Sort::ReleaseDate => (ItemSortBy::PremiereDate, SortOrder::Descending),
            Sort::CommunityRating => (ItemSortBy::CommunityRating, SortOrder::Descending),
            Sort::Random => (ItemSortBy::Random, SortOrder::Ascending),
        }
    }

    pub fn label(self) -> Text {
        match self {
            Sort::Name => Text::SortName,
            Sort::NameDescending => Text::SortNameDescending,
            Sort::DateAdded => Text::SortDateAdded,
            Sort::ReleaseDate => Text::SortReleaseDate,
            Sort::CommunityRating => Text::SortCommunityRating,
            Sort::Random => Text::SortRandom,
        }
    }
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(strings::lookup(self.label()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Previous,
    Next,
}

#[derive(Debug, Clone)]
pub struct State {
    pub library: BaseItemDto,
    pub sort: Sort,
    pub start: i32,
    pub page: Page,
}

pub async fn load(api: Rc<Api>, library: Uuid, sort: Sort, start: i32) -> Result<State, Trouble> {
    Ok(State {
        library: api.item(library).await?,
        sort,
        start,
        page: api.page(library, sort, start, PAGE_SIZE).await?,
    })
}

pub fn view<'a>(state: &'a State, images: &'a Cache) -> Element<'a, Message> {
    let last = (state.start + state.page.items.len() as i32).max(state.start);
    let position = strings::format(
        Text::PagePosition,
        &[
            &(state.start + 1).to_string(),
            &last.to_string(),
            &state.page.total.to_string(),
        ],
    );

    let controls = row![
        text(strings::lookup(Text::LibrarySort)),
        pick_list(Sort::ALL, Some(state.sort), Message::SortSelected),
        text(position),
        widget::step_button(Text::PagePrevious, Step::Previous, state.start > 0),
        widget::step_button(
            Text::PageNext,
            Step::Next,
            last < state.page.total,
        ),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center);

    column![
        text(state.library.name.clone().unwrap_or_default()).size(28),
        controls,
        widget::grid(&state.page.items, images),
    ]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING)
    .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    widget::card_images(&state.page.items)
}
