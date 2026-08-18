//! What the reference decodes a BlurHash at, which is what a card draws while
//! its own image loads.

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

/// The square the reference decodes a BlurHash into.
// reference: blurhash-decode
pub const SIZE: Decode = Decode::pixels(20);

/// The scaling the reference decodes a BlurHash at.
// reference: blurhash-punch
pub const PUNCH: Punch = Punch::of(1.0);
