//! What the browser will and will not send as a user image.

/// The image types an upload sends.
pub const TYPES: [&str; 4] = ["image/jpeg", "image/png", "image/webp", "image/gif"];

/// The largest image an upload sends, which is the cap the relay's user image
/// route declares.
pub const LIMIT: u64 = 4 * 1024 * 1024;

/// Why a chosen file is not sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    Type { mime: String },
    TooLarge { bytes: u64, cap: u64 },
}

/// The refusal a file of `mime` and `bytes` earns, and `None` for a file the
/// browser sends.
pub fn refused(mime: &str, bytes: u64) -> Option<Refused> {
    if !TYPES.contains(&mime) {
        return Some(Refused::Type {
            mime: mime.to_owned(),
        });
    }
    if bytes > LIMIT {
        return Some(Refused::TooLarge { bytes, cap: LIMIT });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_type_outside_the_list_is_refused_by_type() {
        assert_eq!(
            refused("text/plain", 10),
            Some(Refused::Type {
                mime: "text/plain".to_owned()
            })
        );
    }

    #[test]
    fn a_file_over_the_cap_is_refused_naming_its_size_and_the_cap() {
        assert_eq!(
            refused("image/jpeg", LIMIT + 1),
            Some(Refused::TooLarge {
                bytes: LIMIT + 1,
                cap: LIMIT
            })
        );
    }

    #[test]
    fn every_listed_type_at_the_cap_is_sent() {
        for mime in TYPES {
            assert_eq!(refused(mime, LIMIT), None);
        }
    }
}
