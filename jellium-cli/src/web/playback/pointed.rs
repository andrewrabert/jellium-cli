//! The foreign origins the current plan has been pointed at.

/// Every url outside the Jellyfin server that the current plan, or a response
/// to the current plan's own request, named, each behind an opaque handle.
/// A register belongs to one plan and dies with it, so a handle a replaced
/// plan minted resolves to nothing.
// reference: get-text-tracks — playbackmanager.js:2908-2939
pub struct Pointed {
    held: std::sync::RwLock<Held>,
}

#[derive(Default)]
struct Held {
    minted: u64,
    by_url: std::collections::HashMap<String, String>,
    by_handle: std::collections::HashMap<String, String>,
}

impl Pointed {
    pub fn new() -> Pointed {
        Pointed {
            held: std::sync::RwLock::new(Held::default()),
        }
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, Held> {
        match self.held.write() {
            Ok(held) => held,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// The handle `url` is minted under for this plan, minting one when the
    /// plan holds none.
    pub fn mint(&self, url: &str) -> String {
        let mut held = self.write();
        if let Some(handle) = held.by_url.get(url) {
            return handle.clone();
        }
        held.minted += 1;
        let handle = format!("p{:016x}", held.minted);
        held.by_url.insert(url.to_owned(), handle.clone());
        held.by_handle.insert(handle.clone(), url.to_owned());
        handle
    }

    /// The url `handle` names, and `None` for a handle this plan did not mint.
    pub fn resolve(&self, handle: &str) -> Option<String> {
        match self.held.read() {
            Ok(held) => held.by_handle.get(handle).cloned(),
            Err(poisoned) => poisoned.into_inner().by_handle.get(handle).cloned(),
        }
    }

    /// Drops every handle, which is what replacing the plan does.
    pub fn clear(&self) {
        *self.write() = Held::default();
    }
}

/// The same-origin path `handle` is fetched from.
pub fn path(handle: &str) -> String {
    format!("{}/{handle}", jellium_protocol::POINTED_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_url_keeps_the_handle_it_was_first_minted_under() {
        let pointed = Pointed::new();
        let first = pointed.mint("https://elsewhere.test/a.vtt");
        assert_eq!(pointed.mint("https://elsewhere.test/a.vtt"), first);
        assert_ne!(pointed.mint("https://elsewhere.test/b.vtt"), first);
        assert_eq!(
            pointed.resolve(&first).as_deref(),
            Some("https://elsewhere.test/a.vtt")
        );
    }

    #[test]
    fn a_handle_a_replaced_plan_minted_resolves_to_nothing() {
        let pointed = Pointed::new();
        let handle = pointed.mint("https://elsewhere.test/a.vtt");
        pointed.clear();
        assert_eq!(pointed.resolve(&handle), None);
    }
}
