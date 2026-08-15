use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, scrollable, text};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache, Kind};
use crate::player::Intent;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

#[derive(Debug, Clone)]
pub struct State {
    pub item: BaseItemDto,
    pub children: Vec<BaseItemDto>,
    /// What the server says is like this item.
    pub similar: Vec<BaseItemDto>,
}

/// The most items the similar rail shows.
pub const SIMILAR: i32 = 16;

pub async fn load(api: Rc<Api>, item: Uuid) -> Answer<State> {
    Answer::of(async {
        let id = item;
        let item = api.item(item).await.bubbled()?;
        Ok(State {
            children: api.children(&item).await.bubbled()?,
            similar: api
                .similar(id, SIMILAR)
                .await
                .or_default(Text::FailureSimilarUnread),
            item,
        })
    })
    .await
}

fn children_heading(kind: Option<BaseItemKind>) -> Text {
    match kind {
        Some(BaseItemKind::Series) => Text::DetailSeasons,
        Some(BaseItemKind::Season) => Text::DetailEpisodes,
        Some(BaseItemKind::MusicAlbum) => Text::DetailTracks,
        Some(BaseItemKind::MusicArtist) => Text::DetailAlbums,
        _ => Text::DetailEpisodes,
    }
}

fn played(item: &BaseItemDto) -> bool {
    item.user_data
        .as_ref()
        .and_then(|data| data.played)
        .unwrap_or(false)
}

fn favorite(item: &BaseItemDto) -> bool {
    item.user_data
        .as_ref()
        .and_then(|data| data.is_favorite)
        .unwrap_or(false)
}

fn heading(item: &BaseItemDto) -> String {
    match (
        &item.series_name,
        item.parent_index_number,
        item.index_number,
    ) {
        (Some(series), Some(season), Some(episode)) => {
            format!("{series} S{season}E{episode}")
        }
        (Some(series), _, _) => series.clone(),
        (None, _, _) => item
            .album_artist
            .clone()
            .or(item.production_year.map(|year| year.to_string()))
            .unwrap_or_default(),
    }
}

/// True when the server stored a position for an item that is not marked
/// played, which is what Resume begins from.
fn resumable(item: &BaseItemDto) -> bool {
    !played(item)
        && item
            .user_data
            .as_ref()
            .and_then(|data| data.playback_position_ticks)
            .is_some_and(|ticks| ticks > 0)
}

fn intent_button<'a>(label: Text, intent: Intent) -> Element<'a, Message> {
    button(text(strings::lookup(label)))
        .on_press(Message::PlayPressed(intent))
        .into()
}

/// Play, and Resume when the item has a stored position and is not marked
/// played, for a movie, episode, music video or song.
/// Play All and Shuffle for a series, season, album or artist.
/// Instant Mix for a song, album or artist.
fn play_controls<'a>(item: &BaseItemDto) -> Vec<Element<'a, Message>> {
    let Some(id) = item.id else {
        return Vec::new();
    };
    let mut controls = Vec::new();
    match item.type_ {
        Some(
            BaseItemKind::Movie
            | BaseItemKind::Episode
            | BaseItemKind::MusicVideo
            | BaseItemKind::Audio,
        ) => {
            controls.push(intent_button(
                Text::DetailPlay,
                Intent::Item {
                    item: id,
                    resume: false,
                },
            ));
            if resumable(item) {
                controls.push(intent_button(
                    Text::DetailResume,
                    Intent::Item {
                        item: id,
                        resume: true,
                    },
                ));
            }
        }
        Some(
            BaseItemKind::Series
            | BaseItemKind::Season
            | BaseItemKind::MusicAlbum
            | BaseItemKind::MusicArtist,
        ) => {
            controls.push(intent_button(
                Text::DetailPlayAll,
                Intent::All {
                    item: id,
                    shuffle: false,
                },
            ));
            controls.push(intent_button(
                Text::DetailShuffle,
                Intent::All {
                    item: id,
                    shuffle: true,
                },
            ));
        }
        _ => {}
    }

    if matches!(
        item.type_,
        Some(BaseItemKind::Audio | BaseItemKind::MusicAlbum | BaseItemKind::MusicArtist)
    ) {
        controls.push(intent_button(
            Text::DetailInstantMix,
            Intent::Mix { item: id },
        ));
    }

    controls
}

/// The item, its controls, and — for an administrator alone — Refresh
/// Metadata with its replace and scan mode options.
pub fn view<'a>(
    state: &'a State,
    images: &'a Cache,
    session: &'a jellium_protocol::Session,
) -> Element<'a, Message> {
    let item = &state.item;

    let poster: Element<'a, Message> = match item.id.and_then(|id| {
        images.handle(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
            width: theme::IMAGE_WIDTH,
        })
    }) {
        Some(handle) => iced::widget::image(handle)
            .width(theme::CARD_WIDTH * 1.5)
            .into(),
        None => text("").into(),
    };

    let mut actions = row![].spacing(theme::CARD_SPACING);
    for control in play_controls(item) {
        actions = actions.push(control);
    }
    if let Some(id) = item.id {
        let mark = if played(item) {
            Text::DetailMarkUnplayed
        } else {
            Text::DetailMarkPlayed
        };
        let star = if favorite(item) {
            Text::DetailUnfavorite
        } else {
            Text::DetailFavorite
        };
        actions = actions
            .push(
                button(text(strings::lookup(mark)))
                    .on_press(Message::PlayedToggled(id, !played(item))),
            )
            .push(
                button(text(strings::lookup(star)))
                    .on_press(Message::FavoriteToggled(id, !favorite(item))),
            );

        if session.administrator && !session.read_only {
            actions = actions
                .push(
                    button(text(strings::lookup(Text::DetailRefreshMetadata))).on_press(
                        Message::RefreshItem {
                            item: id,
                            replace: false,
                            recursive: true,
                        },
                    ),
                )
                .push(
                    button(text(strings::lookup(Text::DetailRefreshReplace))).on_press(
                        Message::RefreshItem {
                            item: id,
                            replace: true,
                            recursive: true,
                        },
                    ),
                )
                .push(
                    button(text(strings::lookup(Text::DetailRefreshScanMode))).on_press(
                        Message::RefreshItem {
                            item: id,
                            replace: false,
                            recursive: false,
                        },
                    ),
                );
        }
    }

    let mut summary = column![
        text(item.name.clone().unwrap_or_default()).size(30),
        text(heading(item)).size(16),
        actions,
    ]
    .spacing(theme::CARD_SPACING);

    if let Some(overview) = &item.overview {
        summary = summary
            .push(text(strings::lookup(Text::DetailOverview)).size(20))
            .push(text(overview.as_str()));
    }

    let mut page = column![row![poster, summary].spacing(theme::CARD_SPACING)]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    if !state.children.is_empty() {
        page = page
            .push(text(strings::lookup(children_heading(item.type_))).size(22))
            .push(widget::grid(&state.children, images, !session.read_only));
    }

    if session.administrator
        && !session.read_only
        && let Some(id) = item.id
    {
        page = page.push(button(text(strings::lookup(Text::MetadataOpen))).on_press(
            Message::Navigated(crate::route::Route::Metadata {
                item: id,
                part: crate::screen::metadata::Part::Fields,
            }),
        ));
    }

    if !state.similar.is_empty() {
        page = page.push(widget::rail(
            Text::DetailSimilar,
            &state.similar,
            images,
            !session.read_only,
        ));
    }

    scrollable(page).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.children);
    keys.extend(widget::card_images(&state.similar));
    if let Some(id) = state.item.id {
        keys.insert(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
            width: theme::IMAGE_WIDTH,
        });
    }
    keys
}
