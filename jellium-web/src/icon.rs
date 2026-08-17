//! The Material Icons glyphs the client draws, and the one site a ligature name
//! becomes a codepoint.

use iced::Element;

use crate::app::Message;
use crate::failure::unraised;
use crate::fonts::Codepoint;
use crate::style::{self, Length};

/// The table `just assets` writes from the reference's own icon metadata: one
/// ligature per row, with the codepoint it draws in base sixteen.
const TABLE: &str = include_str!("../icons/material.tsv");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Person,
    Storage,
}

impl Icon {
    /// The ligature the reference writes for this glyph.
    pub fn ligature(self) -> &'static str {
        match self {
            Icon::Person => "person",
            Icon::Storage => "storage",
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
