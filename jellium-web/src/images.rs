use std::collections::{HashMap, HashSet};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code, reason = "Backdrop is part of the image key surface")]
pub enum Kind {
    Primary,
    Backdrop,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Primary => "Primary",
            Kind::Backdrop => "Backdrop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub item: Uuid,
    pub kind: Kind,
    pub width: u16,
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
