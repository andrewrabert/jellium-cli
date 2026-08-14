use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, scrollable, text};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Trouble;
use crate::images::{self, Cache, Kind};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

#[derive(Debug, Clone)]
pub struct State {
    pub item: BaseItemDto,
    pub children: Vec<BaseItemDto>,
}

pub async fn load(api: Rc<Api>, item: Uuid) -> Result<State, Trouble> {
    let item = api.item(item).await?;
    Ok(State {
        children: api.children(&item).await?,
        item,
    })
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
    match (&item.series_name, item.parent_index_number, item.index_number) {
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

pub fn view<'a>(state: &'a State, images: &'a Cache) -> Element<'a, Message> {
    let item = &state.item;

    let poster: Element<'a, Message> = match item.id.and_then(|id| {
        images.handle(images::Key {
            item: id,
            kind: Kind::Primary,
            width: theme::IMAGE_WIDTH,
        })
    }) {
        Some(handle) => iced::widget::image(handle).width(theme::CARD_WIDTH * 1.5).into(),
        None => text("").into(),
    };

    let mut actions = row![].spacing(theme::CARD_SPACING);
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
            .push(widget::grid(&state.children, images));
    }

    scrollable(page).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.children);
    if let Some(id) = state.item.id {
        keys.insert(images::Key {
            item: id,
            kind: Kind::Primary,
            width: theme::IMAGE_WIDTH,
        });
    }
    keys
}
