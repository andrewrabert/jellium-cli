//! The bridge honours nine verbs and nothing else, and the plugin it addresses
//! is the application's to decide.

use serde::Deserialize;

/// The nine verbs a configuration page may ask the application to perform.
#[derive(Debug, Clone, PartialEq)]
pub enum Verb {
    ReadConfiguration,
    WriteConfiguration { body: serde_json::Value },
    SystemInfo,
    Users,
    VirtualFolders,
    Notice { text: String },
    SaveOutcome,
    Busy,
    Idle,
}

/// One request a configuration frame made.
#[derive(Debug, Clone, PartialEq)]
pub struct Asked {
    /// The frame's own correlation id, echoed in the answer.
    pub call: u64,
    pub verb: Verb,
}

/// A payload the bridge will not act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    /// The verb the payload named, as it named it.
    pub verb: String,
}

/// The json one bridge payload carries; every other key it holds, including
/// any plugin id, is dropped, because the plugin is fixed by the frame that
/// opened.
#[derive(Debug, Deserialize)]
struct Payload {
    call: u64,
    verb: String,
    #[serde(default)]
    body: serde_json::Value,
}

/// Reads one bridge payload.
/// A payload naming a verb outside the nine reads as `Refused`; a plugin id the
/// payload carries is dropped, because the plugin is fixed by the frame that
/// opened.
/// A payload that is not json, or that names no call and no verb, reads as
/// `Refused` naming what it did say.
pub fn read(payload: &str) -> Result<Asked, Refused> {
    let Ok(read) = serde_json::from_str::<Payload>(payload) else {
        return Err(Refused {
            verb: String::new(),
        });
    };
    let verb = match read.verb.as_str() {
        "readConfiguration" => Verb::ReadConfiguration,
        "writeConfiguration" => Verb::WriteConfiguration { body: read.body },
        "systemInfo" => Verb::SystemInfo,
        "users" => Verb::Users,
        "virtualFolders" => Verb::VirtualFolders,
        "notice" => Verb::Notice {
            text: read
                .body
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| read.body.to_string()),
        },
        "saveOutcome" => Verb::SaveOutcome,
        "busy" => Verb::Busy,
        "idle" => Verb::Idle,
        _ => return Err(Refused { verb: read.verb }),
    };
    Ok(Asked {
        call: read.call,
        verb,
    })
}

/// The answer sent back down the frame's channel.
/// A `None` value is the refusal the frame's own promise rejects on.
pub fn answer(call: u64, value: Option<&serde_json::Value>) -> String {
    let answer = serde_json::json!({
        "call": call,
        "ok": value.is_some(),
        "value": value.cloned().unwrap_or(serde_json::Value::Null),
    });
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_of_the_nine_verbs_is_read() {
        let nine = [
            ("readConfiguration", Verb::ReadConfiguration),
            ("systemInfo", Verb::SystemInfo),
            ("users", Verb::Users),
            ("virtualFolders", Verb::VirtualFolders),
            ("saveOutcome", Verb::SaveOutcome),
            ("busy", Verb::Busy),
            ("idle", Verb::Idle),
        ];
        for (named, verb) in nine {
            let payload = format!(r#"{{"call":7,"verb":"{named}","body":null}}"#);
            assert_eq!(read(&payload), Ok(Asked { call: 7, verb }), "{named}");
        }
        assert_eq!(
            read(r#"{"call":1,"verb":"writeConfiguration","body":{"A":1}}"#),
            Ok(Asked {
                call: 1,
                verb: Verb::WriteConfiguration {
                    body: serde_json::json!({"A": 1})
                }
            })
        );
        assert_eq!(
            read(r#"{"call":2,"verb":"notice","body":"saved"}"#),
            Ok(Asked {
                call: 2,
                verb: Verb::Notice {
                    text: "saved".to_owned()
                }
            })
        );
    }

    #[test]
    fn a_verb_outside_the_nine_is_refused_by_name() {
        assert_eq!(
            read(r#"{"call":1,"verb":"deleteUser","body":null}"#),
            Err(Refused {
                verb: "deleteUser".to_owned()
            })
        );
    }

    #[test]
    fn a_plugin_id_the_payload_carries_reaches_nothing() {
        let asked = read(
            r#"{"call":3,"verb":"readConfiguration","pluginId":"00000000-0000-0000-0000-000000000001","body":null}"#,
        )
        .expect("a read");
        assert_eq!(asked.verb, Verb::ReadConfiguration);
    }

    #[test]
    fn a_payload_that_is_not_json_is_refused() {
        assert_eq!(
            read("not json"),
            Err(Refused {
                verb: String::new()
            })
        );
    }

    #[test]
    fn an_answer_echoes_the_call_and_says_whether_it_carried_a_value() {
        let carried = answer(9, Some(&serde_json::json!({"A": 1})));
        assert!(carried.contains(r#""call":9"#));
        assert!(carried.contains(r#""ok":true"#));
        let refused = answer(9, None);
        assert!(refused.contains(r#""ok":false"#));
    }
}
