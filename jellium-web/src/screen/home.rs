use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{column, row};
use jellyfin_api::types::{BaseItemDto, CollectionType, MediaType};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::route::Route;
use crate::style::space::Room;
use crate::style::{Viewport, card};
use crate::text::{self as strings, Text};
use crate::widget;

/// The most channels the on-now row shows.
pub const ON_NOW: i32 = 20;

#[derive(Debug, Clone)]
pub struct State {
    pub libraries: Vec<BaseItemDto>,
    pub continue_watching: Vec<BaseItemDto>,
    pub next_up: Vec<BaseItemDto>,
    /// One row per library, in the library order, each carrying that library
    /// and its latest items.
    pub latest: Vec<Latest>,
    /// The user's favourite channels first and then channels in number order,
    /// capped at `ON_NOW`; the trouble stands in the row's place.
    pub on_now: Vec<Channel>,
}

/// One Latest row.
#[derive(Debug, Clone)]
pub struct Latest {
    pub library: BaseItemDto,
    pub items: Vec<BaseItemDto>,
}

/// The most items one Latest row shows.
pub const LATEST: i32 = 16;

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        let libraries = api.libraries().await.bubbled()?;
        let mut latest = Vec::new();
        for library in &libraries {
            let Some(id) = library.id else {
                continue;
            };
            let items = api
                .latest(id, LATEST)
                .await
                .or_default(Text::FailureLatestUnread);
            if items.is_empty() {
                continue;
            }
            latest.push(Latest {
                library: library.clone(),
                items,
            });
        }

        Ok(State {
            continue_watching: api.continue_watching().await.bubbled()?,
            next_up: api.next_up().await.bubbled()?,
            libraries,
            latest,
            on_now: Vec::new(),
        })
    })
    .await
}

/// The channels the on-now row shows, in one channel query carrying their
/// current programs.
pub async fn on_now(api: Rc<Api>) -> Answer<Vec<Channel>> {
    api.channels(jellyfin_api::types::ChannelType::Tv, Some(ON_NOW))
        .await
}

/// What the home screen shows: the library order, the libraries hidden, and the
/// two rows.
#[derive(Debug, Clone, PartialEq)]
pub struct Arrangement {
    pub order: Vec<uuid::Uuid>,
    pub hidden: Vec<uuid::Uuid>,
    pub continue_watching: bool,
    pub next_up: bool,
}

impl Arrangement {
    /// What the user configuration and the preference bag ask of the home
    /// screen.
    pub fn of(
        configuration: &jellium_model::form::Form,
        held: jellium_model::prefs::Held,
    ) -> Arrangement {
        Arrangement {
            order: jellium_model::user::ids(configuration, jellium_model::user::ORDERED_VIEWS),
            hidden: jellium_model::user::ids(configuration, jellium_model::user::MY_MEDIA_EXCLUDES),
            continue_watching: held.continue_watching,
            next_up: held.next_up,
        }
    }
}

/// Where a media type's resumed section stands among the reference's own
/// defaults, a media type none of the three names standing after all three.
fn sectioned(media: Option<MediaType>) -> usize {
    match media {
        Some(MediaType::Video) => 0,
        Some(MediaType::Audio) => 1,
        Some(MediaType::Book) => 2,
        Some(MediaType::Unknown | MediaType::Photo) | None => 3,
    }
}

/// The resumed items grouped the way the reference's own `Resume`,
/// `ResumeAudio` and `ResumeBook` sections group them, in that order. The
/// reference reaches those three through three requests where this client holds
/// one list, so this splits what is already fetched by the media type each item
/// carries, and a group the list holds nothing for draws no section at all.
pub fn resumed(items: &[BaseItemDto]) -> Vec<(Option<MediaType>, Vec<&BaseItemDto>)> {
    let mut groups: Vec<(Option<MediaType>, Vec<&BaseItemDto>)> = Vec::new();
    for item in items {
        match groups
            .iter_mut()
            .find(|(media, _)| *media == item.media_type)
        {
            Some((_, held)) => held.push(item),
            None => groups.push((item.media_type, vec![item])),
        }
    }
    groups.sort_by_key(|(media, _)| sectioned(*media));
    groups
}

/// The screen a library tile opens, which the view's own collection type
/// decides: a box-set view opens the Collections screen and a playlist view the
/// Playlists screen, both of which this client draws itself, and every other
/// view opens the library screen its id names. A view carrying no id opens
/// nothing.
fn opens(library: &BaseItemDto) -> Option<Route> {
    match library.collection_type {
        Some(CollectionType::Boxsets) => Some(Route::Collections),
        Some(CollectionType::Playlists) => Some(Route::Playlists),
        _ => Some(Route::Library {
            id: library.id?,
            tab: Box::new(crate::screen::library::Tab::Items(Box::default())),
        }),
    }
}

// reference: home-sections
/// The sections a default user sees, in the order the server's own defaults put
/// them: the library tiles in the arrangement's order, what is resumed, what
/// Live TV offers and what is on now, what is next up, and what is latest in
/// each library. A section the arrangement turns off is absent rather than
/// empty.
pub fn view<'a>(
    state: &'a State,
    arrangement: &'a Arrangement,
    live_tv: bool,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
    images: &'a Cache,
    overflow: widget::Overflow,
) -> Element<'a, Message> {
    if state.libraries.is_empty() && state.continue_watching.is_empty() && state.next_up.is_empty()
    {
        return widget::centered(strings::lookup(Text::HomeEmpty).to_string());
    }

    let mut page = column![];

    let ids: Vec<uuid::Uuid> = state.libraries.iter().filter_map(|it| it.id).collect();
    let shown = jellium_model::user::arranged(&ids, &arrangement.order, &arrangement.hidden);
    let libraries: Vec<&BaseItemDto> = shown
        .iter()
        .filter_map(|id| state.libraries.iter().find(|it| it.id == Some(*id)))
        .collect();
    if !libraries.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::HomeMyMedia),
            widget::wall(
                card::Card::LIBRARY,
                Room::content(viewport),
                libraries.iter().filter_map(|library| {
                    Some(widget::library_tile(
                        library,
                        Room::content(viewport),
                        Message::Navigated(opens(library)?),
                    ))
                }),
            ),
        ));
    }

    if arrangement.continue_watching {
        for (media, items) in resumed(&state.continue_watching) {
            page = page.push(widget::section(
                strings::lookup(Text::HomeContinueWatching),
                widget::rail(
                    card::Card::resumed(media),
                    items,
                    Room::content(viewport),
                    images,
                    overflow,
                ),
            ));
        }
    }
    if live_tv {
        // reference: home-live-tv
        page = page.push(widget::section(
            strings::lookup(Text::HomeLiveTv),
            // reference: home-live-tv-sections
            row(crate::screen::livetv::Tab::ALL.iter().map(|tab| {
                widget::block(
                    strings::lookup(tab.label()),
                    Some(Message::Navigated(Route::LiveTv { tab: *tab })),
                    widget::Emphasis::Raised,
                )
            }))
            .into(),
        ));
        page = page.push(widget::section(
            strings::lookup(Text::HomeOnNow),
            widget::on_now_row(&state.on_now, Room::content(viewport), now, images),
        ));
    }

    if arrangement.next_up && !state.next_up.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::HomeNextUp),
            widget::rail(
                card::Card::NEXT_UP,
                state.next_up.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
        ));
    }
    for row in &state.latest {
        page = page.push(widget::section(
            row.library.name.as_deref().unwrap_or_default(),
            widget::rail(
                card::Card::latest(row.library.collection_type),
                row.items.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
        ));
    }
    crate::widget::scrolled(page).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.continue_watching);
    keys.extend(widget::card_images(&state.next_up));
    for row in &state.latest {
        keys.extend(widget::card_images(&row.items));
    }
    {
        keys.extend(state.on_now.iter().map(|channel| images::Key {
            item: channel.id,
            kind: images::Kind::Primary,
            index: None,
        }));
    }
    keys
}
