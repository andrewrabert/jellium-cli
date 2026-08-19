//! What one rail of a sectioned screen holds before and after its own request
//! has answered.

/// One rail's own answer.
#[derive(Debug, Clone, Default)]
pub enum Arrival<T> {
    #[default]
    Awaited,
    Arrived(Vec<T>),
}

impl<T> Arrival<T> {
    // a rail that has not answered and a rail that answered nothing both draw
    // nothing, which is what the reference hides on
    /// The items the rail draws.
    pub fn held(&self) -> &[T] {
        match self {
            Arrival::Awaited => &[],
            Arrival::Arrived(items) => items,
        }
    }

    /// The items the rail draws, for a live refresh to mark in place.
    pub fn held_mut(&mut self) -> &mut [T] {
        match self {
            Arrival::Awaited => &mut [],
            Arrival::Arrived(items) => items,
        }
    }
}
