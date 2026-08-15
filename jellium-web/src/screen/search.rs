use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, text, text_input};
use jellium_model::paged::Paged;
use jellium_model::window;
use jellyfin_api::types::BaseItemDto;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::screen::browse::{self, Browse};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

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
}

/// The most entries one search section shows.
pub const SECTION: i32 = 12;

pub async fn load(
    api: Rc<Api>,
    term: String,
    listing: Listing,
    viewport: iced::Size,
) -> Answer<State> {
    Answer::of(async {
        let heading = strings::lookup(Text::NavSearch).to_string();
        let mut browse = Browse::new(window::Id::Browse, heading, listing.clone(), viewport);

        let mut people = Vec::new();
        let mut studios = Vec::new();
        let mut programs = Vec::new();

        if !term.trim().is_empty() {
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
        })
    })
    .await
}

/// One search section: a bounded strip of cards, each opening the route its
/// entry names.
fn section<'a>(
    title: Text,
    items: &'a [BaseItemDto],
    images: &'a Cache,
    opens: impl Fn(uuid::Uuid) -> crate::route::Route + 'a,
) -> Element<'a, Message> {
    let cards = items.iter().filter_map(|item| {
        let id = item.id?;
        Some(
            iced::widget::button(widget::card(
                item,
                widget::poster_key(item).and_then(|key| images.handle(key)),
                false,
            ))
            .style(iced::widget::button::text)
            .on_press(Message::Navigated(opens(id)))
            .into(),
        )
    });
    column![
        text(strings::lookup(title)).size(22),
        iced::widget::scrollable(iced::widget::row(cards).spacing(theme::CARD_SPACING)).direction(
            iced::widget::scrollable::Direction::Horizontal(
                iced::widget::scrollable::Scrollbar::default()
            )
        ),
    ]
    .spacing(8)
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

pub fn view<'a>(state: &'a State, images: &'a Cache, read_only: bool) -> Element<'a, Message> {
    let bar = row![
        text_input(strings::lookup(Text::SearchPlaceholder), &state.term)
            .on_input(Message::SearchEdited)
            .on_submit(Message::SearchSubmitted)
            .padding(8),
        button(text(strings::lookup(Text::SearchSubmit))).on_press(Message::SearchSubmitted),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center);

    let mut page = column![bar]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    if state.browse.items.is_empty() {
        page = page.push(widget::banner(
            strings::lookup(Text::SearchEmpty).to_string(),
        ));
        return page.into();
    }

    page = page.push(browse::view(&state.browse, images, read_only));

    if !state.people.is_empty() {
        page = page.push(section(Text::SearchPeople, &state.people, images, |id| {
            narrowed(jellium_model::facets::Facet::Person, id)
        }));
    }
    if !state.studios.is_empty() {
        page = page.push(section(Text::SearchStudios, &state.studios, images, |id| {
            narrowed(jellium_model::facets::Facet::Studio, id)
        }));
    }
    if !state.programs.is_empty() {
        page = page.push(section(
            Text::SearchPrograms,
            &state.programs,
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
