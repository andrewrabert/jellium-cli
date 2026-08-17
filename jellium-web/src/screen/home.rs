use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{column, scrollable};
use jellyfin_api::types::{BaseItemDto, CollectionType};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;
use crate::widget::prose;

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
    /// True when the server offers a collections view, which is what puts the
    /// Collections destination in the library list.
    pub collections_view: bool,
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
            collections_view: libraries
                .iter()
                .any(|library| library.collection_type == Some(CollectionType::Boxsets)),
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

/// The on-now row above the existing rows, the library list in the arrangement's
/// order carrying a Live TV entry when `live_tv`, and a row the arrangement
/// turns off absent rather than empty.
pub fn view<'a>(
    state: &'a State,
    arrangement: &'a Arrangement,
    live_tv: bool,
    now: chrono::DateTime<chrono::Utc>,
    images: &'a Cache,
    read_only: bool,
) -> Element<'a, Message> {
    if state.libraries.is_empty() && state.continue_watching.is_empty() && state.next_up.is_empty()
    {
        return widget::banner(strings::lookup(Text::HomeEmpty).to_string());
    }

    let mut page = column![].spacing(style::drawn(space::GUTTER.drawn()));

    if live_tv {
        page = page.push(if state.on_now.is_empty() {
            widget::banner(strings::lookup(Text::ChannelsEmpty).to_string())
        } else {
            widget::on_now_row(&state.on_now, now, images)
        });
    }

    if arrangement.continue_watching && !state.continue_watching.is_empty() {
        page = page.push(widget::rail(
            Text::HomeContinueWatching,
            &state.continue_watching,
            images,
            !read_only,
        ));
    }
    if arrangement.next_up && !state.next_up.is_empty() {
        page = page.push(widget::rail(
            Text::HomeNextUp,
            &state.next_up,
            images,
            !read_only,
        ));
    }
    for row in &state.latest {
        page = page.push(widget::named_rail(
            row.library.name.as_deref().unwrap_or_default(),
            &row.items,
            images,
            !read_only,
        ));
    }
    let ids: Vec<uuid::Uuid> = state.libraries.iter().filter_map(|it| it.id).collect();
    let shown = jellium_model::user::arranged(&ids, &arrangement.order, &arrangement.hidden);
    let libraries: Vec<&BaseItemDto> = shown
        .iter()
        .filter_map(|id| state.libraries.iter().find(|it| it.id == Some(*id)))
        .collect();
    if !libraries.is_empty() {
        page = page.push(prose(
            strings::lookup(Text::HomeLibraries).to_owned(),
            typeface::HEADING_2,
        ));
        page = page.push(widget::library_row(
            libraries,
            live_tv,
            state.collections_view,
        ));
    }

    scrollable(page).into()
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
            width: theme::IMAGE_WIDTH,
        }));
    }
    keys
}
