use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, scrollable};
use jellium_model::item::{self, Mark};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache, Kind};
use crate::player::Intent;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::prose;

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
    item::played(item) == Mark::Cleared
        && item
            .user_data
            .as_ref()
            .and_then(|data| data.playback_position_ticks)
            .is_some_and(|ticks| ticks > 0)
}

fn intent_button<'a>(label: Text, intent: Intent) -> Element<'a, Message> {
    button(prose(strings::lookup(label), typeface::BODY))
        .style(style::raised)
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
    viewport: Viewport,
    images: &'a Cache,
    session: &'a jellium_protocol::Session,
) -> Element<'a, Message> {
    let item = &state.item;
    let overflow = match session.read_only {
        true => widget::Overflow::Withheld,
        false => widget::Overflow::Offered,
    };

    let poster = widget::tile(
        card::Card::Wall(card::Shape::Portrait),
        Room::content(viewport),
        item.id.and_then(|id| {
            images.handle(images::Key {
                item: id,
                kind: Kind::Primary,
                index: None,
            })
        }),
    );

    let mut actions = row![].spacing(style::drawn(space::GUTTER.drawn()));
    for control in play_controls(item) {
        actions = actions.push(control);
    }
    if let Some(id) = item.id {
        let mark = match item::played(item) {
            Mark::Set => Text::DetailMarkUnplayed,
            Mark::Cleared => Text::DetailMarkPlayed,
        };
        let star = match item::favorited(item) {
            Mark::Set => Text::DetailUnfavorite,
            Mark::Cleared => Text::DetailFavorite,
        };
        actions = actions
            .push(
                button(prose(strings::lookup(mark), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::PlayedToggled(id, item::played(item).flipped())),
            )
            .push(
                button(prose(strings::lookup(star), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::FavoriteToggled(
                        id,
                        item::favorited(item).flipped(),
                    )),
            );

        if session.administrator && !session.read_only {
            actions = actions
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshMetadata),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: false,
                        recursive: true,
                    }),
                )
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshReplace),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: true,
                        recursive: true,
                    }),
                )
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshScanMode),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: false,
                        recursive: false,
                    }),
                );
        }
    }

    let mut summary = column![
        prose(item.name.clone().unwrap_or_default(), typeface::HEADING_1),
        prose(heading(item), typeface::BODY),
        actions,
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if let Some(overview) = &item.overview {
        summary = summary
            .push(prose(
                strings::lookup(Text::DetailOverview),
                typeface::HEADING_3,
            ))
            .push(prose(overview.clone(), typeface::BODY));
    }

    let mut page = column![row![poster, summary].spacing(style::drawn(space::GUTTER.drawn()))]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    if !state.children.is_empty() {
        page = page
            .push(prose(
                strings::lookup(children_heading(item.type_)),
                typeface::HEADING_2,
            ))
            .push(widget::posters(
                card::Card::Wall(card::Shape::Portrait),
                state.children.iter(),
                Room::content(viewport),
                images,
                overflow,
            ));
    }

    if session.administrator
        && !session.read_only
        && let Some(id) = item.id
    {
        page = page.push(
            button(prose(strings::lookup(Text::MetadataOpen), typeface::BODY))
                .style(style::raised)
                .on_press(Message::Navigated(crate::route::Route::Metadata {
                    item: id,
                    part: crate::screen::metadata::Part::Fields,
                })),
        );
    }

    if !state.similar.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::DetailSimilar),
            widget::rail(
                card::Card::Rail(card::Rail::Portrait),
                state.similar.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
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
        });
    }
    keys
}
