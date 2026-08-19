//! `if (value)`, as the reference's own JavaScript answers it.

/// A value the reference tests with `if (value)`.
pub trait Truthy {
    /// What the value carries where that test passes.
    type Held;

    /// The value where the reference's own test passes, and none where it
    /// fails: zero, false and empty text each fail it.
    fn truthy(self) -> Option<Self::Held>;
}

impl Truthy for Option<i32> {
    type Held = i32;

    fn truthy(self) -> Option<i32> {
        self.filter(|held| *held != 0)
    }
}

impl Truthy for Option<bool> {
    type Held = bool;

    fn truthy(self) -> Option<bool> {
        self.filter(|held| *held)
    }
}

impl Truthy for Option<String> {
    type Held = String;

    fn truthy(self) -> Option<String> {
        self.filter(|held| !held.is_empty())
    }
}

impl<'held> Truthy for Option<&'held str> {
    type Held = &'held str;

    fn truthy(self) -> Option<&'held str> {
        self.filter(|held| !held.is_empty())
    }
}
