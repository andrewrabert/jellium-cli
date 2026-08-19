//! What the reference decodes a BlurHash at and what it draws the decode over:
//! the square the decode fills, and the box a card stretches that square
//! across while its own image loads.

use super::{Drawn, Share, nearest};

/// The square a BlurHash is decoded into, in pixels of the decode rather than
/// of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decode(u32);

impl Decode {
    pub const fn pixels(count: u32) -> Decode {
        Decode(count)
    }

    pub fn count(self) -> u32 {
        self.0
    }
}

/// How far a BlurHash's AC components are scaled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Punch(f32);

impl Punch {
    pub const fn of(scale: f32) -> Punch {
        Punch(scale)
    }

    pub fn scale(self) -> f32 {
        self.0
    }
}

/// How much of the box behind an image a decoded BlurHash is drawn over, held
/// per axis because css writes one declaration for each.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stretch {
    wide: Share,
    tall: Share,
}

impl Stretch {
    /// The two percentages a rule writes for the box's own axes.
    pub const fn percent(wide: f64, tall: f64) -> Stretch {
        Stretch {
            wide: Share::per_ten_thousand(nearest(wide * 100.0)),
            tall: Share::per_ten_thousand(nearest(tall * 100.0)),
        }
    }

    /// The width this draws at inside a box `inside` wide.
    pub fn width(self, inside: Drawn) -> Drawn {
        self.wide.of(inside)
    }

    /// The height this draws at inside a box `inside` tall.
    pub fn height(self, inside: Drawn) -> Drawn {
        self.tall.of(inside)
    }
}

/// The square the reference decodes a BlurHash into.
// reference: blurhash-decode
pub const SIZE: Decode = Decode::pixels(20);

/// The scaling the reference decodes a BlurHash at.
// reference: blurhash-punch
pub const PUNCH: Punch = Punch::of(1.0);

/// The box the reference stretches a decoded BlurHash over, which is the whole
/// of the image container it stands in.
// reference: blurhash-stretch
pub const STRETCH: Stretch = Stretch::percent(100.0, 100.0);
