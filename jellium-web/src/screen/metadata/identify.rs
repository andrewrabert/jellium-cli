use iced::Element;
use iced::widget::{button, checkbox, column, row, text, text_input};
use jellyfin_api::types::{BaseItemKind, RemoteSearchResult};

use crate::app::Message;
use crate::images::Foreign;
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action as Outer;

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
    pub fn query(&self, search: Search) -> serde_json::Value {
        let mut ids = serde_json::Map::new();
        if !self.provider.trim().is_empty() && !self.provider_id.trim().is_empty() {
            ids.insert(
                self.provider.trim().to_owned(),
                serde_json::Value::String(self.provider_id.trim().to_owned()),
            );
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "a year outside i32 carries no cause beyond the number itself"
        )]
        let year = self.year.trim().parse::<i32>().ok();
        #[expect(
            clippy::disallowed_methods,
            reason = "json! builds a literal body this client wrote, which cannot fail to render"
        )]
        let asked = serde_json::json!({
            "SearchInfo": {
                "Name": self.name,
                "Year": year,
                "ProviderIds": ids,
            },
            "IncludeDisabledProviders": true,
            "SearchProviderName": serde_json::Value::Null,
            "ItemType": search.segment(),
        });
        asked
    }
}

fn typed<'a>(label: Text, field: Field, held: &'a str) -> Element<'a, Message> {
    row![
        text(strings::lookup(label)),
        text_input("", held)
            .on_input(
                move |value| Message::MetadataAction(Outer::Identify(Action::Typed(field, value)))
            )
            .padding(8),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center)
    .into()
}

/// Each candidate with its poster, name, year, overview and the provider that
/// returned it; a poster is addressed by the handle the local server minted.
pub fn view<'a>(state: &'a State, foreign: &'a Foreign, read_only: bool) -> Element<'a, Message> {
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
    .spacing(theme::CARD_SPACING);

    if !read_only {
        page = page.push(
            button(text(strings::lookup(Text::MetadataIdentifyRun)))
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
                text(strings::format(Text::MetadataApplyAsk, &[&named])),
                row![
                    checkbox(state.replace_images).on_toggle(|on| Message::MetadataAction(
                        Outer::Identify(Action::SetReplaceImages(on))
                    )),
                    text(strings::lookup(Text::MetadataReplaceImages)),
                ]
                .spacing(theme::CARD_SPACING)
                .align_y(iced::Alignment::Center),
                row![
                    button(text(strings::lookup(Text::MetadataApply)))
                        .on_press(Message::MetadataAction(Outer::Identify(Action::Apply))),
                    button(text(strings::lookup(Text::MetadataCancel)))
                        .on_press(Message::MetadataAction(Outer::Identify(Action::Cancel))),
                ]
                .spacing(theme::CARD_SPACING),
            ]
            .spacing(theme::CARD_SPACING),
        );
        return page.into();
    }

    for (at, candidate) in state.candidates.iter().enumerate() {
        let poster: Element<'a, Message> = match candidate
            .image_url
            .as_deref()
            .and_then(|handle| foreign.handle(handle))
        {
            Some(held) => iced::widget::image(held).width(theme::CARD_WIDTH).into(),
            None => iced::widget::Space::new()
                .width(theme::CARD_WIDTH)
                .height(theme::CARD_WIDTH * 1.5)
                .into(),
        };

        let mut summary = column![
            text(candidate.name.clone().unwrap_or_default()).size(18),
            text(
                candidate
                    .production_year
                    .map(|year| year.to_string())
                    .unwrap_or_default()
            ),
            text(candidate.search_provider_name.clone().unwrap_or_default()).size(13),
        ]
        .spacing(4);

        if let Some(overview) = &candidate.overview {
            summary = summary.push(text(overview.as_str()).size(13));
        }
        if !read_only {
            summary = summary.push(
                button(text(strings::lookup(Text::MetadataChoose))).on_press(
                    Message::MetadataAction(Outer::Identify(Action::Choose { at })),
                ),
            );
        }

        page = page.push(row![poster, summary].spacing(theme::CARD_SPACING));
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
