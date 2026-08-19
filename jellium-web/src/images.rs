use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use jellium_model::appearance::blur;

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
    /// The card whose request ladder decides how wide the server is asked to
    /// fill this image.
    pub card: card::Card,
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

/// One image tag, as the server mints one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl Tag {
    /// A tag of the letters and digits the server mints one from, and None for
    /// any other text.
    pub fn read(text: &str) -> Option<Tag> {
        let named = !text.is_empty() && text.chars().all(|value| value.is_ascii_alphanumeric());
        named.then(|| Tag(text.to_owned()))
    }
}

/// One BlurHash, as the wire carries it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash(String);

impl Hash {
    /// A hash of the base-83 alphabet BlurHash is written in, and None for any
    /// other text.
    pub fn read(text: &str) -> Option<Hash> {
        let named = !text.is_empty() && text.chars().all(|value| !value.is_whitespace());
        named.then(|| Hash(text.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The BlurHashes one item carries, held by image kind and image tag.
/// This is the one site that reads `BaseItemDtoImageBlurHashes`, whose two
/// levels of map `jellyfin-api` carries as `String`.
#[derive(Debug, Clone, Default)]
pub struct Hashes {
    held: HashMap<(Kind, Tag), Hash>,
}

impl Hashes {
    pub fn of(item: &jellyfin_api::types::BaseItemDto) -> Hashes {
        let mut held = HashMap::new();
        let Some(hashes) = item.image_blur_hashes.as_ref() else {
            return Hashes { held };
        };
        for (named, tags) in [
            (Kind::Primary, &hashes.primary),
            (Kind::Backdrop, &hashes.backdrop),
            (Kind::Thumb, &hashes.thumb),
            (Kind::Logo, &hashes.logo),
            (Kind::Banner, &hashes.banner),
            (Kind::Art, &hashes.art),
            (Kind::Chapter, &hashes.chapter),
        ] {
            for (tag, hash) in tags {
                let (Some(tag), Some(hash)) = (Tag::read(tag), Hash::read(hash)) else {
                    continue;
                };
                held.insert((named, tag), hash);
            }
        }
        Hashes { held }
    }

    pub fn hash(&self, kind: Kind, tag: &Tag) -> Option<&Hash> {
        self.held.get(&(kind, tag.clone()))
    }
}

/// What one card asks the server for: the key the image is fetched under, and
/// the BlurHash the wire carries for that key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poster {
    pub key: Key,
    pub hash: Option<Hash>,
}

impl Poster {
    /// The key alone, for an image the wire carries no hash for.
    pub fn of(key: Key) -> Poster {
        Poster { key, hash: None }
    }
}

/// The images one screen asks for: the keys it fetches, and the BlurHash the
/// wire carries for each key it has one for.
#[derive(Debug, Clone, Default)]
pub struct Wanted {
    keys: HashSet<Key>,
    hashes: HashMap<Key, Hash>,
}

impl Wanted {
    pub fn new() -> Wanted {
        Wanted::default()
    }

    pub fn want(&mut self, posted: Poster) {
        if let Some(hash) = posted.hash {
            self.hashes.insert(posted.key, hash);
        }
        self.keys.insert(posted.key);
    }

    pub fn extend(&mut self, posted: impl IntoIterator<Item = Poster>) {
        for one in posted {
            self.want(one);
        }
    }

    pub fn keys(&self) -> &HashSet<Key> {
        &self.keys
    }

    pub fn hash(&self, key: Key) -> Option<&Hash> {
        self.hashes.get(&key)
    }
}

impl IntoIterator for Wanted {
    type Item = Poster;
    type IntoIter = std::vec::IntoIter<Poster>;

    fn into_iter(mut self) -> Self::IntoIter {
        let held: Vec<Poster> = self
            .keys
            .iter()
            .map(|key| Poster {
                key: *key,
                hash: self.hashes.remove(key),
            })
            .collect();
        held.into_iter()
    }
}

impl Extend<Wanted> for Wanted {
    fn extend<T: IntoIterator<Item = Wanted>>(&mut self, held: T) {
        for one in held {
            Wanted::extend(self, one);
        }
    }
}

impl FromIterator<Key> for Wanted {
    fn from_iter<T: IntoIterator<Item = Key>>(keys: T) -> Wanted {
        keys.into_iter().map(Poster::of).collect()
    }
}

impl Extend<Poster> for Wanted {
    fn extend<T: IntoIterator<Item = Poster>>(&mut self, posted: T) {
        Wanted::extend(self, posted);
    }
}

impl Extend<Key> for Wanted {
    fn extend<T: IntoIterator<Item = Key>>(&mut self, keys: T) {
        Wanted::extend(self, keys.into_iter().map(Poster::of));
    }
}

impl FromIterator<Poster> for Wanted {
    fn from_iter<T: IntoIterator<Item = Poster>>(posted: T) -> Wanted {
        let mut held = Wanted::new();
        held.extend(posted);
        held
    }
}

#[derive(Default)]
pub struct Cache {
    held: HashMap<Key, iced::widget::image::Handle>,
    in_flight: HashSet<Key>,
    attempts: HashMap<Key, u32>,
    /// One entry per hash decoded, holding nothing for a hash that would not
    /// decode, so a refusal is not attempted twice.
    blurred: HashMap<Hash, Option<iced::widget::image::Handle>>,
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

    /// The placeholder `hash` decodes to, and None while it has not been
    /// decoded or would not decode.
    pub fn placeholder(&self, hash: &Hash) -> Option<iced::widget::image::Handle> {
        self.blurred.get(hash).cloned().flatten()
    }

    /// Decodes every hash `wanted` carries that is not held, at `blur::SIZE`
    /// square and `blur::PUNCH`; a hash that will not decode is recorded and
    /// held as nothing.
    pub fn blur(&mut self, wanted: &Wanted) {
        for hash in wanted.keys().iter().filter_map(|key| wanted.hash(*key)) {
            if self.blurred.contains_key(hash) {
                continue;
            }
            let size = blur::SIZE.count();
            let drawn = crate::failure::unblurred(
                crate::text::Text::FailureImageUnread,
                hash,
                blur::SIZE,
                blur::PUNCH,
            )
            .map(|pixels| iced::widget::image::Handle::from_rgba(size, size, pixels));
            self.blurred.insert(hash.clone(), drawn);
        }
    }

    /// Drops every handle, in-flight mark and attempt count whose key is absent
    /// from `keep`, and every placeholder no key it holds carries.
    pub fn retain(&mut self, keep: &Wanted) {
        self.held.retain(|key, _| keep.keys.contains(key));
        self.in_flight.retain(|key| keep.keys.contains(key));
        self.attempts.retain(|key, _| keep.keys.contains(key));
        let held: HashSet<&Hash> = keep.hashes.values().collect();
        self.blurred.retain(|hash, _| held.contains(hash));
    }
}
