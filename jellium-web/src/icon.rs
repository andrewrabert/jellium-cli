//! The Material Icons glyphs the client draws, and the one site a ligature name
//! becomes a codepoint.

use iced::Element;
use jellyfin_api::types::CollectionType;

use crate::app::Message;
use crate::failure::unraised;
use crate::fonts::Codepoint;
use crate::style::{self, Length};

/// The table `just assets` writes from the reference's own icon metadata: one
/// ligature per row, with the codepoint it draws in base sixteen.
const TABLE: &str = include_str!("../icons/material.tsv");

/// Declares the glyph enum, the roll the gate reads and the ligature each
/// variant draws under, from one list. A variant named here is in all three; a
/// variant named nowhere else does not exist.
macro_rules! icons {
    ($($variant:ident => $ligature:literal,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Icon {
            $($variant,)*
        }

        impl Icon {
            /// Every variant, which is what the icon table is checked against.
            #[cfg(test)]
            pub const ALL: &'static [Icon] = &[$(Icon::$variant,)*];

            /// The ligature the reference writes for this glyph.
            pub fn ligature(self) -> &'static str {
                match self {
                    $(Icon::$variant => $ligature,)*
                }
            }
        }
    };
}

icons! {
    ArrowBack => "arrow_back",
    Audiotrack => "audiotrack",
    Autorenew => "autorenew",
    Book => "book",
    Cast => "cast",
    ClosedCaption => "closed_caption",
    FastForward => "fast_forward",
    FastRewind => "fast_rewind",
    FilterAlt => "filter_alt",
    Folder => "folder",
    Fullscreen => "fullscreen",
    FullscreenExit => "fullscreen_exit",
    Groups => "groups",
    LiveTv => "live_tv",
    Movie => "movie",
    MusicNote => "music_note",
    MusicVideo => "music_video",
    Pause => "pause",
    Person => "person",
    Photo => "photo",
    PlayArrow => "play_arrow",
    Queue => "queue",
    Quiz => "quiz",
    Repeat => "repeat",
    RepeatOne => "repeat_one",
    Search => "search",
    Settings => "settings",
    Shuffle => "shuffle",
    SkipNext => "skip_next",
    SkipPrevious => "skip_previous",
    SortByAlpha => "sort_by_alpha",
    Storage => "storage",
    Theaters => "theaters",
    Tv => "tv",
    VideoLibrary => "video_library",
    VolumeOff => "volume_off",
    VolumeUp => "volume_up",
}

impl Icon {
    /// The glyph a library draws, which its own collection type decides.
    // reference: library-icon
    // reference: library-icon-unknown
    pub fn library(collection: Option<CollectionType>) -> Icon {
        match collection {
            Some(CollectionType::Movies) => Icon::Movie,
            Some(CollectionType::Music) => Icon::MusicNote,
            Some(CollectionType::Homevideos | CollectionType::Photos) => Icon::Photo,
            Some(CollectionType::Livetv) => Icon::LiveTv,
            Some(CollectionType::Tvshows) => Icon::Tv,
            Some(CollectionType::Trailers) => Icon::Theaters,
            Some(CollectionType::Musicvideos) => Icon::MusicVideo,
            Some(CollectionType::Books) => Icon::Book,
            Some(CollectionType::Boxsets) => Icon::VideoLibrary,
            Some(CollectionType::Playlists) => Icon::Queue,
            None => Icon::Quiz,
            Some(CollectionType::Folders | CollectionType::Unknown) => Icon::Folder,
        }
    }

    /// The codepoint the table records for the ligature, and None where it
    /// holds no row for it.
    pub fn glyph(self) -> Option<Codepoint> {
        TABLE.lines().find_map(|line| {
            let (ligature, scalar) = line.split_once('\t')?;
            if ligature != self.ligature() {
                return None;
            }
            let Ok(codepoint) = unraised::read::<Codepoint>(scalar) else {
                return None;
            };
            Some(codepoint)
        })
    }
}

/// The glyph drawn in the Material Icons face, and nothing where the table
/// holds no character for it.
pub fn icon<'a>(icon: Icon, size: Length) -> Element<'a, Message> {
    let Some(drawn) = icon.glyph().and_then(Codepoint::character) else {
        return iced::widget::Space::new().into();
    };
    iced::widget::text(drawn.to_string())
        .font(style::ICONS)
        .size(style::drawn(size.drawn()))
        .into()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{Icon, TABLE};

    /// Every variant's ligature has a row in the icon table and every row of
    /// the table has a variant.
    #[wasm_bindgen_test]
    fn the_icon_table_and_the_variants_agree() {
        let named: HashSet<&str> = Icon::ALL.iter().map(|icon| icon.ligature()).collect();
        let held: HashSet<&str> = TABLE
            .lines()
            .filter_map(|line| line.split('\t').next())
            .filter(|ligature| !ligature.is_empty())
            .collect();
        let mut missing: Vec<&str> = named.difference(&held).copied().collect();
        let mut stray: Vec<&str> = held.difference(&named).copied().collect();
        missing.sort_unstable();
        stray.sort_unstable();
        assert!(
            missing.is_empty() && stray.is_empty(),
            "the icon table lacks {missing:?} and holds {stray:?} under no variant"
        );
    }
}
