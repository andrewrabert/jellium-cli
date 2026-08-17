//! The `UserConfiguration` fields this client's screens and player read, as
//! `form::Field`s written through one `form::Form`.

use crate::form::{Field, Form};
use uuid::Uuid;

pub const AUDIO_LANGUAGE: Field = Field::Listed {
    key: "AudioLanguagePreference",
};

pub const SUBTITLE_LANGUAGE: Field = Field::Listed {
    key: "SubtitleLanguagePreference",
};

pub const PLAY_DEFAULT_AUDIO_TRACK: Field = Field::Flag {
    key: "PlayDefaultAudioTrack",
};

pub const SUBTITLE_MODE: Field = Field::Choice {
    key: "SubtitleMode",
    options: &["Default", "Always", "OnlyForced", "None", "Smart"],
};

pub const NEXT_EPISODE_AUTOPLAY: Field = Field::Flag {
    key: "EnableNextEpisodeAutoPlay",
};

pub const MISSING_EPISODES: Field = Field::Flag {
    key: "DisplayMissingEpisodes",
};

pub const ORDERED_VIEWS: Field = Field::Lines {
    key: "OrderedViews",
};

pub const MY_MEDIA_EXCLUDES: Field = Field::Lines {
    key: "MyMediaExcludes",
};

/// True when the configuration asks for the next episode to play; a value that
/// is neither `true` nor `false` reads as false.
pub fn next_episode_autoplay(form: &Form) -> bool {
    form.flagged(NEXT_EPISODE_AUTOPLAY)
}

/// The ids one `Field::Lines` field holds, dropping every line that is not a
/// uuid.
pub fn ids(form: &Form, field: Field) -> Vec<Uuid> {
    form.value(field)
        .lines()
        .filter_map(|line| Uuid::parse_str(line.trim()).ok())
        .collect()
}

/// `libraries` in `order`'s order with every id in `hidden` dropped; a library
/// `order` does not name keeps its server order after those it does.
pub fn arranged(libraries: &[Uuid], order: &[Uuid], hidden: &[Uuid]) -> Vec<Uuid> {
    let mut arranged: Vec<Uuid> = order
        .iter()
        .copied()
        .filter(|id| libraries.contains(id))
        .collect();
    let rest: Vec<Uuid> = libraries
        .iter()
        .copied()
        .filter(|id| !arranged.contains(id))
        .collect();
    arranged.extend(rest);
    arranged.retain(|id| !hidden.contains(id));
    arranged
}

/// Which way a library moves in the home screen's own order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toward {
    Earlier,
    Later,
}

/// `order` with `id` moved one place `toward`, over `libraries` when the order
/// names none of them.
pub fn moved(libraries: &[Uuid], order: &[Uuid], id: Uuid, toward: Toward) -> Vec<Uuid> {
    let mut moved = arranged(libraries, order, &[]);
    let Some(at) = moved.iter().position(|held| *held == id) else {
        return moved;
    };
    let to = match toward {
        Toward::Later => at + 1,
        Toward::Earlier => at.wrapping_sub(1),
    };
    if to < moved.len() {
        moved.swap(at, to);
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(byte: u8) -> Uuid {
        Uuid::from_bytes([byte; 16])
    }

    #[test]
    fn autoplay_reads_true_only_for_true() {
        let form = Form::of(serde_json::json!({"EnableNextEpisodeAutoPlay": true}));
        assert!(next_episode_autoplay(&form));
        let off = Form::of(serde_json::json!({"EnableNextEpisodeAutoPlay": false}));
        assert!(!next_episode_autoplay(&off));
        let absent = Form::of(serde_json::json!({}));
        assert!(!next_episode_autoplay(&absent));
    }

    #[test]
    fn a_line_that_is_not_a_uuid_is_dropped() {
        let form = Form::of(serde_json::json!({
            "OrderedViews": [id(1).to_string(), "not-a-uuid", id(2).to_string()],
        }));
        assert_eq!(ids(&form, ORDERED_VIEWS), vec![id(1), id(2)]);
    }

    #[test]
    fn a_library_the_order_does_not_name_keeps_its_server_order_after_those_it_does() {
        let libraries = [id(1), id(2), id(3)];
        assert_eq!(
            arranged(&libraries, &[id(3)], &[]),
            vec![id(3), id(1), id(2)]
        );
    }

    #[test]
    fn a_hidden_library_is_dropped_and_an_order_naming_an_absent_one_ignores_it() {
        let libraries = [id(1), id(2)];
        assert_eq!(arranged(&libraries, &[id(9), id(2)], &[id(1)]), vec![id(2)]);
    }

    #[test]
    fn a_move_at_either_end_changes_nothing() {
        let libraries = [id(1), id(2), id(3)];
        let order = arranged(&libraries, &[], &[]);
        assert_eq!(moved(&libraries, &order, id(1), Toward::Earlier), order);
        assert_eq!(moved(&libraries, &order, id(3), Toward::Later), order);
        assert_eq!(
            moved(&libraries, &order, id(1), Toward::Later),
            vec![id(2), id(1), id(3)]
        );
        assert_eq!(
            moved(&libraries, &order, id(3), Toward::Earlier),
            vec![id(1), id(3), id(2)]
        );
    }
}
