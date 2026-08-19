use jellium_protocol::Quality;
use serde::{Deserialize, Serialize};

use crate::failure::Call;
use crate::failure::{self, Cause, Failure};
use crate::text::Text;

/// The `localStorage` entries this browser holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    Device,
    /// The id this browser minted for itself, kept apart from `Device` so
    /// `Device` carries no `String`.
    DeviceId,
    Shared,
    /// Keyed by the signed-in user, the way `appSettings` keys a `userSettings`
    /// entry whose `enableOnServer` is false.
    Account(uuid::Uuid),
}

impl Entry {
    fn key(self) -> std::borrow::Cow<'static, str> {
        match self {
            Entry::Device => std::borrow::Cow::Borrowed("jellium_web_prefs"),
            Entry::DeviceId => std::borrow::Cow::Borrowed("jellium_web_device_id"),
            Entry::Shared => std::borrow::Cow::Borrowed("jellium_web_app_settings"),
            Entry::Account(user) => {
                std::borrow::Cow::Owned(format!("jellium_web_user_settings-{user}"))
            }
        }
    }
}

/// The preferences this browser holds, and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub volume: f32,
    pub muted: bool,
}

impl Default for Device {
    fn default() -> Device {
        Device {
            volume: 1.0,
            muted: false,
        }
    }
}

fn storage() -> Option<web_sys::Storage> {
    failure::called(Call::LocalStorage, web_sys::window()?.local_storage())?
}

/// What `entry` holds, and `None` when it is missing or will not read.
pub fn stored<T: serde::de::DeserializeOwned>(entry: Entry) -> Option<T> {
    let raw = failure::called(Call::LocalStorageGetItem, storage()?.get_item(&entry.key()))??;
    failure::decoded(Text::FailureStored, &raw)
}

/// Writes `value` into `entry`, leaving every other entry standing.
pub fn store<T: serde::Serialize>(entry: Entry, value: &T) {
    let Some(storage) = storage() else {
        return;
    };
    let Some(raw) = failure::rendered(Text::FailureStored, value) else {
        return;
    };
    failure::called(
        Call::LocalStorageSetItem,
        storage.set_item(&entry.key(), &raw),
    );
}

impl Device {
    /// Reads `localStorage`; a missing or malformed entry reads as full volume
    /// and unmuted.
    pub fn load() -> Device {
        stored(Entry::Device).unwrap_or_default()
    }

    /// Writes volume and mute into the entry, leaving every other key it holds
    /// standing, so a ceiling an earlier version parked there survives a volume
    /// change and a mute.
    pub fn store(&self) {
        let Some(serde_json::Value::Object(mine)) = failure::encoded(Text::FailureStored, self)
        else {
            failure::raise(Failure::told(
                Text::FailureStored,
                Cause::Malformed {
                    detail: "the device preferences are not an object".to_owned(),
                },
            ));
            return;
        };
        let mut held = match stored(Entry::Device) {
            Some(serde_json::Value::Object(held)) => held,
            _ => serde_json::Map::new(),
        };
        held.extend(mine);
        store(Entry::Device, &serde_json::Value::Object(held));
    }

    /// Applies what becomes of the ceiling an earlier version parked in the
    /// entry: `Parked::Kept` leaves it, `Parked::Dropped` removes it and leaves
    /// volume and mute alone in the entry.
    /// An entry holding no ceiling is not rewritten.
    pub fn settle(parked: jellium_model::prefs::Parked) {
        if parked == jellium_model::prefs::Parked::Kept {
            return;
        }
        let Some(serde_json::Value::Object(mut held)) = stored(Entry::Device) else {
            return;
        };
        if held.remove("quality").is_none() {
            return;
        }
        store(Entry::Device, &serde_json::Value::Object(held));
    }

    /// The bitrate ceiling an earlier version parked in `localStorage`, and
    /// `None` once the entry holds the device preferences alone.
    pub fn parked() -> Option<Quality> {
        let parked = stored::<serde_json::Value>(Entry::Device)?
            .get("quality")?
            .clone();
        failure::parsed(Text::FailureStored, parked)
    }
}
