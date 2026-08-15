//! The key table the player matches and the keyboard controls screen lists, so
//! neither can name a key the other does not.

use iced::keyboard::{Key, key::Named};

use crate::text::Text;

/// What a bound key does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Does {
    TogglePlay,
    SkipBack,
    SkipForward,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    Fullscreen,
    Next,
    Previous,
    Leave,
}

impl Does {
    /// What the keyboard controls screen calls this.
    pub fn text(self) -> Text {
        match self {
            Does::TogglePlay => Text::DoesTogglePlay,
            Does::SkipBack => Text::DoesSkipBack,
            Does::SkipForward => Text::DoesSkipForward,
            Does::VolumeUp => Text::DoesVolumeUp,
            Does::VolumeDown => Text::DoesVolumeDown,
            Does::ToggleMute => Text::DoesToggleMute,
            Does::Fullscreen => Text::DoesFullscreen,
            Does::Next => Text::DoesNext,
            Does::Previous => Text::DoesPrevious,
            Does::Leave => Text::DoesLeave,
        }
    }
}

/// One key binding the player honours.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub key: Key<&'static str>,
    /// The key as the controls screen names it.
    pub named: Text,
    /// What the key does.
    pub does: Does,
}

/// Every binding the player honours, which is both what `player::keys` matches
/// and what the keyboard controls screen lists.
pub const BINDINGS: &[Binding] = &[
    Binding {
        key: Key::Named(Named::Space),
        named: Text::KeySpace,
        does: Does::TogglePlay,
    },
    Binding {
        key: Key::Character("k"),
        named: Text::KeyK,
        does: Does::TogglePlay,
    },
    Binding {
        key: Key::Named(Named::ArrowLeft),
        named: Text::KeyArrowLeft,
        does: Does::SkipBack,
    },
    Binding {
        key: Key::Named(Named::ArrowRight),
        named: Text::KeyArrowRight,
        does: Does::SkipForward,
    },
    Binding {
        key: Key::Named(Named::ArrowUp),
        named: Text::KeyArrowUp,
        does: Does::VolumeUp,
    },
    Binding {
        key: Key::Named(Named::ArrowDown),
        named: Text::KeyArrowDown,
        does: Does::VolumeDown,
    },
    Binding {
        key: Key::Character("m"),
        named: Text::KeyM,
        does: Does::ToggleMute,
    },
    Binding {
        key: Key::Character("f"),
        named: Text::KeyF,
        does: Does::Fullscreen,
    },
    Binding {
        key: Key::Character("n"),
        named: Text::KeyN,
        does: Does::Next,
    },
    Binding {
        key: Key::Character("p"),
        named: Text::KeyP,
        does: Does::Previous,
    },
    Binding {
        key: Key::Named(Named::Escape),
        named: Text::KeyEscape,
        does: Does::Leave,
    },
];

/// The action `key` raises, and `None` for a key `BINDINGS` does not name;
/// `volume` is what the two volume keys move from.
pub fn bound(key: &Key<&str>, volume: f32) -> Option<crate::player::Action> {
    use crate::player::{Action, VOLUME_STEP};

    let held = BINDINGS.iter().find(|binding| binding.key == *key)?;
    Some(match held.does {
        Does::TogglePlay => Action::TogglePlay,
        Does::SkipBack => Action::SkipBack,
        Does::SkipForward => Action::SkipForward,
        Does::VolumeUp => Action::SetVolume(volume + VOLUME_STEP),
        Does::VolumeDown => Action::SetVolume(volume - VOLUME_STEP),
        Does::ToggleMute => Action::ToggleMute,
        Does::Fullscreen => Action::ToggleFullscreen,
        Does::Next => Action::Next,
        Does::Previous => Action::Previous,
        Does::Leave => Action::Leave,
    })
}
