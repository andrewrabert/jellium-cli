//! The trickplay a media source offers: which width a preview draws from, and
//! which tile and cell hold the frame at a position.

use std::collections::HashMap;
use std::time::Duration;

use jellium_protocol::Chapter;
use jellyfin_api::types::BaseItemDto;

use crate::appearance::card;

/// The trickplay an item offers, by media source id and then by tile width.
#[derive(Debug, Clone, Default)]
pub struct Trickplay {
    held: HashMap<String, Vec<Description>>,
}

/// One trickplay width as the server built it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Description {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub thumbnail_count: u32,
    pub interval: Duration,
}

impl Description {
    /// How many thumbnails one tile sheet holds.
    fn per_tile(self) -> u32 {
        self.tile_width.saturating_mul(self.tile_height).max(1)
    }
}

impl Trickplay {
    /// The trickplay `item` describes, empty when it describes none.
    pub fn of(item: &BaseItemDto) -> Trickplay {
        let mut held: HashMap<String, Vec<Description>> = HashMap::new();
        for (source, widths) in item.trickplay.iter().flatten() {
            let mut described: Vec<Description> = widths
                .values()
                .filter_map(|info| {
                    Some(Description {
                        width: u32::try_from(info.width?).ok()?,
                        height: u32::try_from(info.height.unwrap_or_default()).unwrap_or_default(),
                        tile_width: u32::try_from(info.tile_width?).ok()?,
                        tile_height: u32::try_from(info.tile_height?).ok()?,
                        thumbnail_count: u32::try_from(info.thumbnail_count.unwrap_or_default())
                            .unwrap_or_default(),
                        interval: Duration::from_millis(
                            u64::try_from(info.interval.unwrap_or_default()).unwrap_or_default(),
                        ),
                    })
                })
                .filter(|described| described.width > 0 && !described.interval.is_zero())
                .collect();
            described.sort_by_key(|described| described.width);
            if !described.is_empty() {
                held.insert(source.clone(), described);
            }
        }
        Trickplay { held }
    }

    /// The narrowest width at or above `rendered` for `media_source`, the widest
    /// when none reaches it, and `None` when that source has none.
    pub fn width_for(&self, media_source: &str, rendered: card::Fill) -> Option<Description> {
        let described = self.held.get(media_source)?;
        described
            .iter()
            .find(|held| held.width >= rendered.count())
            .or_else(|| described.last())
            .copied()
    }
}

/// Which tile holds the frame at `position`, and where inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub index: u32,
    pub column: u32,
    pub row: u32,
}

/// The tile covering `position`, and `None` past the last thumbnail.
pub fn tile(description: Description, position: Duration) -> Option<Tile> {
    if description.interval.is_zero() {
        return None;
    }
    let at = u32::try_from(position.as_millis() / description.interval.as_millis()).ok()?;
    if description.thumbnail_count > 0 && at >= description.thumbnail_count {
        return None;
    }
    let per_tile = description.per_tile();
    let within = at % per_tile;
    Some(Tile {
        index: at / per_tile,
        column: within % description.tile_width.max(1),
        row: within / description.tile_width.max(1),
    })
}

/// The chapter covering `position`, which is what an item with no trickplay
/// falls back to.
pub fn chapter_at(chapters: &[Chapter], position: Duration) -> Option<usize> {
    let ticks = (position.as_millis() as i64).checked_mul(10_000)?;
    chapters
        .iter()
        .enumerate()
        .rev()
        .find(|(_, chapter)| chapter.start_ticks <= ticks)
        .map(|(at, _)| at)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DESCRIBED: Description = Description {
        width: 320,
        height: 180,
        tile_width: 10,
        tile_height: 10,
        thumbnail_count: 250,
        interval: Duration::from_millis(1_000),
    };

    #[test]
    fn a_position_lands_in_the_tile_and_cell_holding_its_thumbnail() {
        let first = tile(DESCRIBED, Duration::from_secs(0)).expect("a tile");
        assert_eq!(
            first,
            Tile {
                index: 0,
                column: 0,
                row: 0
            }
        );
        let across = tile(DESCRIBED, Duration::from_secs(13)).expect("a tile");
        assert_eq!(
            across,
            Tile {
                index: 0,
                column: 3,
                row: 1
            }
        );
        let second = tile(DESCRIBED, Duration::from_secs(100)).expect("a tile");
        assert_eq!(
            second,
            Tile {
                index: 1,
                column: 0,
                row: 0
            }
        );
    }

    #[test]
    fn a_position_past_the_last_thumbnail_has_no_tile() {
        assert_eq!(tile(DESCRIBED, Duration::from_secs(250)), None);
        assert_eq!(tile(DESCRIBED, Duration::from_secs(9_999)), None);
    }

    #[test]
    fn the_chapter_covering_a_position_is_the_last_one_starting_at_or_before_it() {
        let chapters = vec![
            Chapter {
                name: "one".to_owned(),
                start_ticks: 0,
            },
            Chapter {
                name: "two".to_owned(),
                start_ticks: 60 * 10_000_000,
            },
            Chapter {
                name: "three".to_owned(),
                start_ticks: 120 * 10_000_000,
            },
        ];
        assert_eq!(chapter_at(&chapters, Duration::from_secs(0)), Some(0));
        assert_eq!(chapter_at(&chapters, Duration::from_secs(59)), Some(0));
        assert_eq!(chapter_at(&chapters, Duration::from_secs(60)), Some(1));
        assert_eq!(chapter_at(&chapters, Duration::from_secs(500)), Some(2));
        assert_eq!(chapter_at(&[], Duration::from_secs(1)), None);
    }

    fn rendered(width: f32) -> card::Fill {
        card::Fill::of(crate::appearance::Css::of(width))
    }

    fn described(widths: &[u32]) -> Trickplay {
        Trickplay {
            held: HashMap::from([(
                "source".to_owned(),
                widths
                    .iter()
                    .map(|width| Description {
                        width: *width,
                        ..DESCRIBED
                    })
                    .collect(),
            )]),
        }
    }

    #[test]
    fn the_narrowest_width_at_or_above_the_preview_is_chosen() {
        let held = described(&[160, 320, 640]);
        assert_eq!(
            held.width_for("source", rendered(200.0)).map(|d| d.width),
            Some(320)
        );
        assert_eq!(
            held.width_for("source", rendered(320.0)).map(|d| d.width),
            Some(320)
        );
        assert_eq!(
            held.width_for("source", rendered(100.0)).map(|d| d.width),
            Some(160)
        );
    }

    #[test]
    fn the_widest_stands_in_when_none_reaches_the_preview() {
        let held = described(&[160, 320]);
        assert_eq!(
            held.width_for("source", rendered(4_000.0)).map(|d| d.width),
            Some(320)
        );
    }

    #[test]
    fn a_source_with_no_trickplay_offers_none() {
        let held = described(&[160]);
        assert_eq!(held.width_for("other", rendered(200.0)), None);
        assert_eq!(
            Trickplay::default().width_for("source", rendered(200.0)),
            None
        );
    }
}
