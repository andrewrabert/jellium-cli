use jellyfin_api::types::{BaseItemDto, BaseItemDtoQueryResult, BaseItemKind, MediaType};
use uuid::Uuid;

use crate::web::upstream::Upstream;
use crate::web::wire::{self, Query};

/// The id of an item intros play before, and nothing for an item they do not:
/// the id is what an item the Jellyfin server holds is recognised by.
// reference: enable-intros — playbackmanager.js:246-259
fn eligible(item: &BaseItemDto) -> Option<Uuid> {
    if item.media_type != Some(MediaType::Video) {
        return None;
    }
    if item.type_ == Some(BaseItemKind::TvChannel) {
        return None;
    }
    // an in-progress recording plays no intro
    if item.status.as_deref() == Some("InProgress") {
        return None;
    }
    item.id
}

/// The intros the Jellyfin server names for `item`, and an empty list wherever
/// the five-way gate closes: a start position, a start index, a start that did
/// not ask for the full screen, an ineligible item, or cinema mode off.
/// A failed request answers an empty list, the way the reference swallows one.
/// `fullscreen`, `cinema_mode` and `start_index` are browser facts and reach
/// the relay on the request, the way `grants` does.
/// Its one caller is `Playback::enter`.
// reference: get-intros — playbackmanager.js:261-275
pub async fn intros(
    upstream: &Upstream,
    item: &BaseItemDto,
    start_ticks: i64,
    start_index: Option<usize>,
    fullscreen: bool,
    cinema_mode: bool,
) -> Vec<Uuid> {
    if start_ticks != 0
        || start_index.is_some_and(|index| index != 0)
        || !fullscreen
        || !cinema_mode
    {
        return Vec::new();
    }
    let Some(id) = eligible(item) else {
        return Vec::new();
    };
    let path = format!("Users/{}/Items/{id}/Intros", upstream.user_id());
    let found: Result<BaseItemDtoQueryResult, _> = wire::got(upstream, &path, &Query::new()).await;
    match found {
        Ok(found) => found
            .items
            .into_iter()
            .filter_map(|intro| intro.id)
            .collect(),
        Err(_) => Vec::new(),
    }
}
