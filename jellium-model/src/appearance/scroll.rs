//! Where one press of a scroll button carries the rail it stands beside.

use super::space::Room;
use super::{Drawn, card, space};

/// Which way a scroll button carries the rail it stands beside.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toward {
    Leading,
    Trailing,
}

/// The measurements a step stands on, which the reference reads off the DOM as
/// `frame.offsetWidth` and `items[0].offsetWidth`. This canvas has no DOM to
/// ask, so both are computed where the rail is drawn, from the room it is laid
/// in and the card drawing it holds, and travel with the press.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub window: Drawn,
    pub pitch: Drawn,
}

impl Frame {
    /// The frame a rail of `drawing` laid in `room` scrolls in: the room's own
    /// content width, and one card's width inside its pitch plus the gutter
    /// between two.
    pub fn of(drawing: card::Drawing, room: Room) -> Frame {
        Frame {
            window: room.width(),
            pitch: drawing.card.width(room).plus(space::GUTTER.drawn()),
        }
    }
}

/// Where one press carries a rail standing at `at`.
/// The trailing button anchors the first card that overflows the window to the
/// window's start.
/// The leading button anchors the last card standing before the window to the
/// window's end, and stops at zero.
// reference: scroll-window
pub fn stepped(at: Drawn, frame: Frame, toward: Toward) -> Drawn {
    if frame.pitch.count() <= 0.0 {
        return at;
    }
    let first = ((at.count() / frame.pitch.count()).floor() - 1.0).max(0.0);
    let last = ((at.count() + frame.window.count()) / frame.pitch.count()).floor();
    match toward {
        Toward::Trailing => Drawn::of(last * frame.pitch.count()),
        Toward::Leading => {
            let fits = (frame.window.count() / frame.pitch.count()).floor() - 1.0;
            Drawn::of(((first - fits) * frame.pitch.count()).max(0.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Frame, Toward, stepped};
    use crate::appearance::Drawn;

    /// A window five cards wide, which is what the reference's own arithmetic
    /// is easiest to read against.
    fn five_cards() -> Frame {
        Frame {
            window: Drawn::of(500.0),
            pitch: Drawn::of(100.0),
        }
    }

    /// The trailing press anchors the first card that overflows the window to
    /// the window's start.
    #[test]
    fn the_trailing_press_anchors_the_overflowing_card_to_the_start() {
        assert_eq!(
            stepped(Drawn::of(0.0), five_cards(), Toward::Trailing),
            Drawn::of(500.0)
        );
        assert_eq!(
            stepped(Drawn::of(250.0), five_cards(), Toward::Trailing),
            Drawn::of(700.0)
        );
    }

    /// The leading press anchors the last card standing before the window to
    /// the window's end, and stops at zero.
    #[test]
    fn the_leading_press_anchors_the_earlier_card_to_the_end_and_stops_at_zero() {
        assert_eq!(
            stepped(Drawn::of(1000.0), five_cards(), Toward::Leading),
            Drawn::of(500.0)
        );
        assert_eq!(
            stepped(Drawn::of(100.0), five_cards(), Toward::Leading),
            Drawn::of(0.0)
        );
    }

    /// A rail whose cards measure nothing stays where it stands.
    #[test]
    fn a_rail_over_cards_of_no_width_does_not_move() {
        assert_eq!(
            stepped(
                Drawn::of(40.0),
                Frame {
                    window: Drawn::of(500.0),
                    pitch: Drawn::of(0.0),
                },
                Toward::Trailing
            ),
            Drawn::of(40.0)
        );
    }
}
