//! The reference's Favorites tab: the sixteen sections it draws, each as a rail
//! of that section's own card shape.

use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::construct::{Construct, Page};
use jellium_model::favorites::{ASKED, Section};
use jellium_protocol::Session;
use jellyfin_api::types::BaseItemDto;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{Cache, Wanted};
use crate::route::Route;
use crate::screen::arrival::Arrival;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space};
use crate::text::{self as strings, Said, Text};
use crate::widget;

/// The reference pages this screen draws.
pub const DRAWS: &[Page] = &[Page::Home];

/// One favourites rail's card: the section's own shape over the lines that
/// section's own options write under it.
// reference: favorites-shapes
fn railed(section: Section) -> card::Drawing {
    card::Drawing {
        card: section.card(),
        footer: section.footer(),
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        touch: card::Touch::Plays,
    }
}

/// What one section reads as.
pub fn label(section: Section) -> Text {
    match section {
        Section::Movies => Text::FavoritesMovies,
        Section::Shows => Text::FavoritesShows,
        Section::Seasons => Text::FavoritesSeasons,
        Section::Episodes => Text::FavoritesEpisodes,
        Section::Videos => Text::FavoritesVideos,
        Section::MusicVideos => Text::FavoritesMusicVideos,
        Section::Collections => Text::FavoritesCollections,
        Section::Playlists => Text::FavoritesPlaylists,
        Section::People => Text::FavoritesPeople,
        Section::Artists => Text::FavoritesArtists,
        Section::Albums => Text::FavoritesAlbums,
        Section::Songs => Text::FavoritesSongs,
        Section::Books => Text::FavoritesBooks,
        Section::Channels => Text::FavoritesChannels,
        Section::PhotoAlbums => Text::FavoritesPhotoAlbums,
        Section::Photos => Text::FavoritesPhotos,
    }
}

/// The list a section's title opens.
pub fn opens(section: Section) -> Route {
    match section {
        Section::Collections => Route::Collections,
        Section::Playlists => Route::Playlists,
        _ => Route::Home {
            tab: crate::screen::home::Tab::Favorites,
        },
    }
}

/// One section's items.
#[derive(Debug, Clone)]
pub struct Rail {
    pub section: Section,
    pub items: Arrival<BaseItemDto>,
}

/// Every section the reference draws, in `Section::ALL`'s order, each holding
/// what its own request answered.
#[derive(Debug, Clone)]
pub struct State {
    pub rails: Vec<Rail>,
}

impl Default for State {
    /// One awaited rail per section, in `Section::ALL`'s order.
    fn default() -> State {
        State {
            rails: Section::ALL
                .into_iter()
                .map(|section| Rail {
                    section,
                    items: Arrival::Awaited,
                })
                .collect(),
        }
    }
}

impl State {
    /// Takes one section's answer.
    pub fn took(&mut self, section: Section, items: Vec<BaseItemDto>) {
        if let Some(rail) = self.rails.iter_mut().find(|rail| rail.section == section) {
            rail.items = Arrival::Arrived(items);
        }
    }
}

/// What one section's own request answers.
pub async fn requested(api: Rc<Api>, section: Section) -> Answer<Vec<BaseItemDto>> {
    api.favorites(section, ASKED).await
}

pub fn view<'a>(
    state: &'a State,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a Session,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    for rail in state
        .rails
        .iter()
        .filter(|rail| !rail.items.held().is_empty())
    {
        page = page.push(widget::section(
            crate::construct::navigation(
                Construct::SectionTitleCards,
                Some(Said::Plain(label(rail.section))),
                Message::Navigated(opens(rail.section)),
                widget::prose(
                    strings::lookup(label(rail.section)),
                    style::typeface::HEADING_2,
                ),
            ),
            widget::rail(
                railed(rail.section),
                widget::Rail::of(Construct::ItemsContainer),
                rail.items.held().iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }
    page.into()
}

pub fn images(state: &State) -> Wanted {
    let mut held = Wanted::new();
    for rail in &state.rails {
        held.extend(widget::card_images(rail.items.held(), rail.section.card()));
    }
    held
}
