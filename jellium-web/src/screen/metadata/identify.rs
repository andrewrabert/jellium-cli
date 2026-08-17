use iced::Element;
use iced::widget::{button, checkbox, column, row, text_input};
use std::collections::HashMap;

use jellyfin_api::types::{BaseItemKind, RemoteSearchResult};

use crate::app::Message;
use crate::images::Foreign;
use crate::text::{self as strings, Text};

use super::Action as Outer;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::widget::{self, prose};

/// The item kinds the Jellyfin server offers a remote search for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Search {
    Book,
    BoxSet,
    Movie,
    MusicAlbum,
    MusicArtist,
    MusicVideo,
    Person,
    Series,
    Trailer,
}

impl Search {
    /// The search `kind` is offered, and `None` for a kind the server offers
    /// none for, which is what takes Identify off the screen.
    pub fn of(kind: Option<BaseItemKind>) -> Option<Search> {
        Some(match kind? {
            BaseItemKind::Book => Search::Book,
            BaseItemKind::BoxSet => Search::BoxSet,
            BaseItemKind::Movie => Search::Movie,
            BaseItemKind::MusicAlbum => Search::MusicAlbum,
            BaseItemKind::MusicArtist => Search::MusicArtist,
            BaseItemKind::MusicVideo => Search::MusicVideo,
            BaseItemKind::Person => Search::Person,
            BaseItemKind::Series => Search::Series,
            BaseItemKind::Trailer => Search::Trailer,
            _ => return None,
        })
    }

    pub fn segment(self) -> &'static str {
        match self {
            Search::Book => "Book",
            Search::BoxSet => "BoxSet",
            Search::Movie => "Movie",
            Search::MusicAlbum => "MusicAlbum",
            Search::MusicArtist => "MusicArtist",
            Search::MusicVideo => "MusicVideo",
            Search::Person => "Person",
            Search::Series => "Series",
            Search::Trailer => "Trailer",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    /// Pre-filled with the item's name and year.
    pub name: String,
    pub year: String,
    /// A provider id typed directly, which searches by it alone.
    pub provider: String,
    pub provider_id: String,
    pub candidates: Vec<RemoteSearchResult>,
    /// The candidate awaiting the one question applying it asks.
    pub applying: Option<usize>,
    /// Whether applying replaces existing images; false is what the question
    /// defaults to.
    pub replace_images: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Typed(Field, String),
    Run,
    Choose { at: usize },
    SetReplaceImages(bool),
    Apply,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Name,
    Year,
    Provider,
    ProviderId,
}

impl State {
    /// The search body one run posts: the name and year typed, and the provider
    /// id when one was given.
    pub fn query(&self) -> RemoteSearch {
        let mut ids = HashMap::new();
        if !self.provider.trim().is_empty() && !self.provider_id.trim().is_empty() {
            ids.insert(
                self.provider.trim().to_owned(),
                self.provider_id.trim().to_owned(),
            );
        }
        let year = match self.year.trim() {
            "" => None,
            typed => crate::failure::read::<i32>(Text::FailureYear, typed),
        };
        RemoteSearch {
            search_info: SearchInfo {
                name: self.name.clone(),
                year,
                provider_ids: ids,
            },
            include_disabled_providers: true,
        }
    }
}

fn typed<'a>(label: Text, field: Field, held: &'a str) -> Element<'a, Message> {
    row![
        prose(strings::lookup(label), typeface::BODY),
        text_input("", held)
            .style(style::input)
            .on_input(
                move |value| Message::MetadataAction(Outer::Identify(Action::Typed(field, value)))
            )
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

/// Each candidate with its poster, name, year, overview and the provider that
/// returned it; a poster is addressed by the handle the local server minted.
pub fn view<'a>(
    state: &'a State,
    viewport: Viewport,
    foreign: &'a Foreign,
    read_only: bool,
) -> Element<'a, Message> {
    let mut page = column![
        typed(Text::MetadataIdentifyName, Field::Name, &state.name),
        typed(Text::MetadataIdentifyYear, Field::Year, &state.year),
        typed(
            Text::MetadataIdentifyProvider,
            Field::Provider,
            &state.provider
        ),
        typed(
            Text::MetadataIdentifyProviderId,
            Field::ProviderId,
            &state.provider_id
        ),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if !read_only {
        page = page.push(
            button(prose(
                strings::lookup(Text::MetadataIdentifyRun),
                typeface::BODY,
            ))
            .style(style::submit)
            .on_press(Message::MetadataAction(Outer::Identify(Action::Run))),
        );
    }

    if let Some(at) = state.applying {
        let named = state
            .candidates
            .get(at)
            .and_then(|held| held.name.clone())
            .unwrap_or_default();
        page = page.push(
            column![
                prose(
                    strings::format(Text::MetadataApplyAsk, &[&named]),
                    typeface::BODY
                ),
                row![
                    checkbox(state.replace_images).on_toggle(|on| Message::MetadataAction(
                        Outer::Identify(Action::SetReplaceImages(on))
                    )),
                    prose(strings::lookup(Text::MetadataReplaceImages), typeface::BODY),
                ]
                .spacing(style::drawn(space::GUTTER.drawn()))
                .align_y(iced::Alignment::Center),
                row![
                    button(prose(strings::lookup(Text::MetadataApply), typeface::BODY))
                        .style(style::submit)
                        .on_press(Message::MetadataAction(Outer::Identify(Action::Apply))),
                    button(prose(strings::lookup(Text::MetadataCancel), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::MetadataAction(Outer::Identify(Action::Cancel))),
                ]
                .spacing(style::drawn(space::GUTTER.drawn())),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        );
        return page.into();
    }

    for (at, candidate) in state.candidates.iter().enumerate() {
        let poster = widget::tile(
            card::Card::Wall(card::Shape::Portrait),
            Room::content(viewport),
            candidate
                .image_url
                .as_deref()
                .and_then(|handle| foreign.handle(handle)),
        );

        let mut summary = column![
            prose(
                candidate.name.clone().unwrap_or_default(),
                typeface::HEADING_3
            ),
            prose(
                candidate
                    .production_year
                    .map(|year| year.to_string())
                    .unwrap_or_default(),
                typeface::BODY
            ),
            prose(
                candidate.search_provider_name.clone().unwrap_or_default(),
                typeface::SECONDARY
            ),
        ]
        .spacing(style::drawn(space::BLOCK_GAP.drawn()));

        if let Some(overview) = &candidate.overview {
            summary = summary.push(prose(overview.clone(), typeface::SECONDARY));
        }
        if !read_only {
            summary = summary.push(
                button(prose(strings::lookup(Text::MetadataChoose), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::MetadataAction(Outer::Identify(Action::Choose {
                        at,
                    }))),
            );
        }

        page = page.push(row![poster, summary].spacing(style::drawn(space::GUTTER.drawn())));
    }

    page.into()
}

/// The handles this surface draws, so the session fetches each once.
pub fn handles(state: &State) -> std::collections::HashSet<String> {
    state
        .candidates
        .iter()
        .filter_map(|candidate| candidate.image_url.clone())
        .collect()
}

/// What `/Items/RemoteSearch/{kind}` takes.
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteSearch {
    search_info: SearchInfo,
    include_disabled_providers: bool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SearchInfo {
    name: String,
    year: Option<i32>,
    provider_ids: HashMap<String, String>,
}
