use jellium_protocol::Quality;
use serde::{Deserialize, Serialize};

use crate::failure::{self, Cause, Failure};
use crate::text::Text;

const STORAGE_KEY: &str = "jellium_web_prefs";

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
    failure::called("localStorage", web_sys::window()?.local_storage())?
}

fn entry() -> Option<serde_json::Value> {
    let raw = failure::called("localStorage.getItem", storage()?.get_item(STORAGE_KEY))??;
    failure::decoded(Text::FailureStored, &raw)
}

fn write(held: &serde_json::Value) {
    let Some(storage) = storage() else {
        return;
    };
    let Some(raw) = failure::rendered(Text::FailureStored, held) else {
        return;
    };
    failure::called("localStorage.setItem", storage.set_item(STORAGE_KEY, &raw));
}

impl Device {
    /// Reads `localStorage`; a missing or malformed entry reads as full volume
    /// and unmuted.
    pub fn load() -> Device {
        let Some(held) = entry() else {
            return Device::default();
        };
        failure::parsed(Text::FailureStored, held).unwrap_or_default()
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
        let mut held = match entry() {
            Some(serde_json::Value::Object(held)) => held,
            _ => serde_json::Map::new(),
        };
        held.extend(mine);
        write(&serde_json::Value::Object(held));
    }

    /// Applies what becomes of the ceiling an earlier version parked in the
    /// entry: `Parked::Kept` leaves it, `Parked::Dropped` removes it and leaves
    /// volume and mute alone in the entry.
    /// An entry holding no ceiling is not rewritten.
    pub fn settle(parked: jellium_model::prefs::Parked) {
        if parked == jellium_model::prefs::Parked::Kept {
            return;
        }
        let Some(serde_json::Value::Object(mut held)) = entry() else {
            return;
        };
        if held.remove("quality").is_none() {
            return;
        }
        write(&serde_json::Value::Object(held));
    }

    /// The bitrate ceiling an earlier version parked in `localStorage`, and
    /// `None` once the entry holds the device preferences alone.
    pub fn parked() -> Option<Quality> {
        let parked = entry()?.get("quality")?.clone();
        failure::parsed(Text::FailureStored, parked)
    }
}
