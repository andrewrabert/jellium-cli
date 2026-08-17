use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::style::card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Primary,
    Backdrop,
    Thumb,
    Logo,
    Banner,
    Art,
    Chapter,
    /// A user's profile image, fetched from the user image route.
    User,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Primary | Kind::User => "Primary",
            Kind::Backdrop => "Backdrop",
            Kind::Thumb => "Thumb",
            Kind::Logo => "Logo",
            Kind::Banner => "Banner",
            Kind::Art => "Art",
            Kind::Chapter => "Chapter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub item: Uuid,
    pub kind: Kind,
    /// The index within the kind, and `None` for the first.
    pub index: Option<i32>,
}

/// The card a kind is drawn on, which is what decides how wide it is asked for.
/// This client's own mapping, the reference resolving a shape from an item's
/// type at each list it builds.
pub fn card(kind: Kind) -> card::Card {
    match kind {
        Kind::Primary => card::Card::Wall(card::Shape::Portrait),
        Kind::Thumb | Kind::Backdrop | Kind::Chapter => card::Card::Wall(card::Shape::Backdrop),
        Kind::Logo | Kind::Banner => card::Card::Wall(card::Shape::Banner),
        Kind::Art | Kind::User => card::Card::Wall(card::Shape::Square),
    }
}

/// The same-origin relay url an image key is fetched from, at `fill` wide: a
/// `Kind::User` key from the user image route, and every other key from the
/// item image route.
pub fn url(key: Key, fill: card::Fill) -> String {
    let origin = crate::page::origin();
    let prefix = jellium_protocol::RELAY_PREFIX;
    let collection = if key.kind == Kind::User {
        "Users"
    } else {
        "Items"
    };
    let index = match key.index {
        Some(index) => format!("/{index}"),
        None => String::new(),
    };
    format!(
        "{origin}{prefix}/{collection}/{}/Images/{}{index}?fillWidth={}",
        key.item,
        key.kind.as_str(),
        fill.count(),
    )
}

/// The same-origin url a foreign image is fetched from.
pub fn foreign_url(handle: &str) -> String {
    format!(
        "{}{}/{handle}",
        crate::page::origin(),
        jellium_protocol::FOREIGN_PREFIX
    )
}

/// The images the local server minted handles for, held by handle.
#[derive(Default)]
pub struct Foreign {
    held: HashMap<String, iced::widget::image::Handle>,
    in_flight: HashSet<String>,
}

impl Foreign {
    pub fn new() -> Foreign {
        Foreign::default()
    }

    /// The image `handle` names, and `None` while it is missing or in flight.
    pub fn handle(&self, handle: &str) -> Option<iced::widget::image::Handle> {
        self.held.get(handle).cloned()
    }

    /// Records a fetch as started; false when one is in flight or the image is
    /// held.
    pub fn begin(&mut self, handle: &str) -> bool {
        !self.held.contains_key(handle) && self.in_flight.insert(handle.to_owned())
    }

    pub fn store(&mut self, handle: &str, bytes: Vec<u8>) {
        if !self.in_flight.remove(handle) {
            return;
        }
        self.held.insert(
            handle.to_owned(),
            iced::widget::image::Handle::from_bytes(bytes),
        );
    }

    /// Clears the in-flight mark; a handle the local server does not hold is
    /// drawn as a missing image and is not asked for again.
    pub fn missing(&mut self, handle: &str) {
        self.in_flight.remove(handle);
    }

    pub fn retain(&mut self, keep: &HashSet<String>) {
        self.held.retain(|handle, _| keep.contains(handle));
        self.in_flight.retain(|handle| keep.contains(handle));
    }
}

#[derive(Default)]
pub struct Cache {
    held: HashMap<Key, iced::widget::image::Handle>,
    in_flight: HashSet<Key>,
    attempts: HashMap<Key, u32>,
}

impl Cache {
    /// A key is fetched at most this many times before the view stops asking.
    pub const ATTEMPTS: u32 = 3;

    pub fn new() -> Cache {
        Cache::default()
    }

    pub fn handle(&self, key: Key) -> Option<iced::widget::image::Handle> {
        self.held.get(&key).cloned()
    }

    /// Records a fetch as started; false when one is in flight, when the image
    /// is held, or when the key is out of attempts.
    pub fn begin(&mut self, key: Key) -> bool {
        !self.held.contains_key(&key)
            && self.attempts.get(&key).copied().unwrap_or(0) < Cache::ATTEMPTS
            && self.in_flight.insert(key)
    }

    /// Held only while the key is still in flight, so an image arriving after
    /// its view was left is dropped.
    pub fn store(&mut self, key: Key, bytes: Vec<u8>) {
        if !self.in_flight.remove(&key) {
            return;
        }
        self.attempts.remove(&key);
        self.held
            .insert(key, iced::widget::image::Handle::from_bytes(bytes));
    }

    /// Clears the in-flight mark and counts the attempt; true when attempts
    /// remain, which is when the caller re-issues the fetch.
    pub fn fail(&mut self, key: Key) -> bool {
        if !self.in_flight.remove(&key) {
            return false;
        }
        let attempts = self.attempts.entry(key).or_insert(0);
        *attempts += 1;
        *attempts < Cache::ATTEMPTS
    }

    /// Drops every handle, in-flight mark and attempt count whose key is absent
    /// from `keep`.
    pub fn retain(&mut self, keep: &HashSet<Key>) {
        self.held.retain(|key, _| keep.contains(key));
        self.in_flight.retain(|key| keep.contains(key));
        self.attempts.retain(|key, _| keep.contains(key));
    }
}
