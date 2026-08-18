//! A configuration section is read whole, edited by key and written whole, so
//! every key no control names survives.

/// One control of a configuration form, named by the json key it edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Text {
        key: &'static str,
    },
    Number {
        key: &'static str,
    },
    Flag {
        key: &'static str,
    },
    Choice {
        key: &'static str,
        options: &'static [&'static str],
    },
    /// A json array of strings, edited as one line per entry.
    Lines {
        key: &'static str,
    },
    /// A string chosen from a list the screen supplies.
    Listed {
        key: &'static str,
    },
    /// A json array of `{"Name": …}` objects, edited as one line per name and
    /// written back as objects carrying only a name.
    Named {
        key: &'static str,
    },
    /// A count of seconds, read and written as whole minutes.
    Minutes {
        key: &'static str,
    },
    /// A count of bits per second, read and written as megabits per second.
    Megabits {
        key: &'static str,
    },
}

impl Field {
    pub fn key(self) -> &'static str {
        match self {
            Field::Text { key }
            | Field::Number { key }
            | Field::Flag { key }
            | Field::Choice { key, .. }
            | Field::Lines { key }
            | Field::Listed { key }
            | Field::Named { key }
            | Field::Minutes { key }
            | Field::Megabits { key } => key,
        }
    }
}

/// A section as the server answered it, with the edits made against it.
#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    read: serde_json::Value,
    edited: serde_json::Map<String, serde_json::Value>,
}

impl Form {
    /// Holds `read`, the section exactly as the server answered it.
    pub fn of(read: serde_json::Value) -> Form {
        Form {
            read,
            edited: serde_json::Map::new(),
        }
    }

    /// What `field` holds now, rendered as text: the edit made against it when
    /// there is one, and what the server answered otherwise. A field carrying
    /// a unit the key does not hold is rendered in that unit, and text held
    /// where a number was expected is rendered as it was typed.
    pub fn value(&self, field: Field) -> String {
        let held = self
            .edited
            .get(field.key())
            .or_else(|| self.read.get(field.key()));
        let Some(held) = held else {
            return String::new();
        };
        match field {
            Field::Lines { .. } => held
                .as_array()
                .map(|entries| entries.iter().map(rendered).collect::<Vec<_>>().join("\n"))
                .unwrap_or_default(),
            Field::Named { .. } => held
                .as_array()
                .map(|entries| {
                    entries
                        .iter()
                        .map(|entry| match entry.get("Name") {
                            Some(name) => rendered(name),
                            None => rendered(entry),
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_default(),
            Field::Minutes { .. } => match held.as_f64() {
                Some(seconds) => ((seconds / SECONDS_A_MINUTE) as i64).to_string(),
                None => rendered(held),
            },
            Field::Megabits { .. } => match held.as_f64() {
                Some(bits) => (bits / BITS_A_MEGABIT).to_string(),
                None => rendered(held),
            },
            _ => rendered(held),
        }
    }

    /// Records an edit; no key but `field`'s changes.
    /// A number that does not parse, and a flag that is neither `true` nor
    /// `false`, are held as the text they were typed as, so nothing the user
    /// typed is silently discarded before the save reads it.
    pub fn edit(&mut self, field: Field, value: String) {
        let held = match field {
            Field::Flag { .. } => match value.as_str() {
                "true" => serde_json::Value::Bool(true),
                "false" => serde_json::Value::Bool(false),
                _ => serde_json::Value::String(value),
            },
            Field::Number { .. } => match value.parse::<serde_json::Number>() {
                Ok(number) => serde_json::Value::Number(number),
                Err(_) => serde_json::Value::String(value),
            },
            Field::Lines { .. } => serde_json::Value::Array(
                value
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| serde_json::Value::String(line.to_owned()))
                    .collect(),
            ),
            Field::Named { .. } => serde_json::Value::Array(
                value
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| serde_json::json!({ "Name": line }))
                    .collect(),
            ),
            Field::Minutes { .. } => match value.parse::<i64>() {
                Ok(minutes) => serde_json::Value::from(minutes * SECONDS_A_MINUTE as i64),
                Err(_) => serde_json::Value::String(value),
            },
            Field::Megabits { .. } => match value.parse::<f64>() {
                Ok(megabits) => serde_json::Value::from((megabits * BITS_A_MEGABIT) as i64),
                Err(_) => serde_json::Value::String(value),
            },
            Field::Text { .. } | Field::Choice { .. } | Field::Listed { .. } => {
                serde_json::Value::String(value)
            }
        };
        self.edited.insert(field.key().to_owned(), held);
    }

    /// The values `field` names now: the edit made against it when there is
    /// one, and what the server answered otherwise.
    // the values of a key holding no array read as none
    pub fn listed(&self, field: Field) -> Vec<String> {
        self.edited
            .get(field.key())
            .or_else(|| self.read.get(field.key()))
            .and_then(serde_json::Value::as_array)
            .map(|entries| entries.iter().map(rendered).collect())
            .unwrap_or_default()
    }

    /// Records an edit setting `field` to `values`.
    pub fn list(&mut self, field: Field, values: Vec<String>) {
        self.edited.insert(
            field.key().to_owned(),
            serde_json::Value::Array(values.into_iter().map(serde_json::Value::String).collect()),
        );
    }

    /// What `field`'s flag holds now, which is false unless it holds `true`.
    pub fn flagged(&self, field: Field) -> bool {
        self.edited
            .get(field.key())
            .or_else(|| self.read.get(field.key()))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    }

    /// Records an edit setting `field`'s flag to `on`.
    pub fn flag(&mut self, field: Field, on: bool) {
        self.edited
            .insert(field.key().to_owned(), serde_json::Value::Bool(on));
    }

    /// Records an edit against `key` directly, for the structured values no
    /// `Field` shape covers: the people array, the provider-id object and the
    /// locked-fields array.
    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.edited.insert(key.to_owned(), value);
    }

    /// The edits made and not yet saved, which is what a save writes over the
    /// item the server answers with.
    pub fn edits(&self) -> serde_json::Map<String, serde_json::Value> {
        self.edited.clone()
    }

    /// True once an edit has been made and not yet saved.
    pub fn dirty(&self) -> bool {
        !self.edited.is_empty()
    }

    /// The whole section as it is to be written: what was read with every edit
    /// applied, so every key no `Field` names is carried through unchanged.
    pub fn written(&self) -> serde_json::Value {
        let mut written = self.read.clone();
        let Some(object) = written.as_object_mut() else {
            return written;
        };
        for (key, value) in &self.edited {
            object.insert(key.clone(), value.clone());
        }
        written
    }

    /// Takes the section as the server answered it after a save, clearing the
    /// edits.
    pub fn saved(&mut self, read: serde_json::Value) {
        self.read = read;
        self.edited.clear();
    }

    /// Takes the section as the server answered it while keeping every edit,
    /// which is what a `UserUpdated` refresh does.
    pub fn refreshed(&mut self, read: serde_json::Value) {
        self.read = read;
    }

    /// Drops every edit, leaving what the server answered, which is what
    /// leaving a form without saving does.
    pub fn discard(&mut self) {
        self.edited.clear();
    }
}

/// How many seconds a minute holds, which is what a `Minutes` field divides by
/// to read and multiplies by to write.
const SECONDS_A_MINUTE: f64 = 60.0;

/// How many bits a megabit holds, which is what a `Megabits` field divides by
/// to read and multiplies by to write.
const BITS_A_MEGABIT: f64 = 1_000_000.0;

/// One json value as a control renders it: a string as itself, so no quoting
/// reaches a text field, and anything else as its json text.
fn rendered(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(held) => held.clone(),
        serde_json::Value::Null => String::new(),
        held => held.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: Field = Field::Text { key: "Name" };
    const PORT: Field = Field::Number { key: "Port" };
    const ENABLED: Field = Field::Flag { key: "Enabled" };
    const PATHS: Field = Field::Lines { key: "Paths" };
    const STUDIOS: Field = Field::Named { key: "Studios" };
    const PADDING: Field = Field::Minutes {
        key: "PrePaddingSeconds",
    };
    const BITRATE: Field = Field::Megabits {
        key: "RemoteClientBitrateLimit",
    };

    #[test]
    fn a_count_of_seconds_reads_and_writes_as_whole_minutes() {
        let mut form = Form::of(serde_json::json!({"PrePaddingSeconds": 600}));
        assert_eq!(form.value(PADDING), "10");
        form.edit(PADDING, "3".to_owned());
        assert_eq!(form.written()["PrePaddingSeconds"], serde_json::json!(180));
    }

    #[test]
    fn a_count_of_bits_reads_and_writes_as_megabits() {
        let mut form = Form::of(serde_json::json!({"RemoteClientBitrateLimit": 5_000_000}));
        assert_eq!(form.value(BITRATE), "5");
        form.edit(BITRATE, "0.25".to_owned());
        assert_eq!(
            form.written()["RemoteClientBitrateLimit"],
            serde_json::json!(250_000)
        );
    }

    #[test]
    fn a_unit_that_does_not_parse_is_held_as_it_was_typed() {
        let mut form = Form::of(serde_json::json!({}));
        form.edit(PADDING, "soon".to_owned());
        assert_eq!(form.value(PADDING), "soon");
        form.edit(BITRATE, "fast".to_owned());
        assert_eq!(form.value(BITRATE), "fast");
    }

    fn section() -> serde_json::Value {
        serde_json::json!({
            "Name": "held",
            "Port": 8096,
            "Enabled": true,
            "Paths": ["one", "two"],
            "Untouched": {"deep": [1, 2, 3]},
        })
    }

    #[test]
    fn a_named_array_reads_as_one_name_a_line_and_writes_back_as_objects() {
        let mut form = Form::of(serde_json::json!({
            "Studios": [{"Name": "one", "Id": "kept"}, {"Name": "two"}],
        }));
        assert_eq!(form.value(STUDIOS), "one\ntwo");
        form.edit(STUDIOS, "one\nthree".to_owned());
        assert_eq!(
            form.written()["Studios"],
            serde_json::json!([{"Name": "one"}, {"Name": "three"}])
        );
    }

    #[test]
    fn a_named_array_drops_the_blank_lines_a_user_left() {
        let mut form = Form::of(serde_json::json!({"Studios": []}));
        form.edit(STUDIOS, "one\n\n  \ntwo\n".to_owned());
        assert_eq!(
            form.written()["Studios"],
            serde_json::json!([{"Name": "one"}, {"Name": "two"}])
        );
    }

    #[test]
    fn a_field_reads_as_the_server_answered_it() {
        let form = Form::of(section());
        assert_eq!(form.value(NAME), "held");
        assert_eq!(form.value(PORT), "8096");
        assert_eq!(form.value(ENABLED), "true");
        assert_eq!(form.value(PATHS), "one\ntwo");
        assert!(!form.dirty());
    }

    #[test]
    fn a_key_no_field_names_survives_a_save() {
        let mut form = Form::of(section());
        form.edit(NAME, "renamed".to_owned());
        let written = form.written();
        assert_eq!(written["Name"], serde_json::json!("renamed"));
        assert_eq!(written["Untouched"], serde_json::json!({"deep": [1, 2, 3]}));
        assert_eq!(written["Port"], serde_json::json!(8096));
    }

    #[test]
    fn an_edit_is_typed_by_its_field() {
        let mut form = Form::of(section());
        form.edit(PORT, "9000".to_owned());
        form.edit(ENABLED, "false".to_owned());
        form.edit(PATHS, "a\n\nb\n".to_owned());
        let written = form.written();
        assert_eq!(written["Port"], serde_json::json!(9000));
        assert_eq!(written["Enabled"], serde_json::json!(false));
        assert_eq!(written["Paths"], serde_json::json!(["a", "b"]));
    }

    #[test]
    fn a_form_is_dirty_until_it_is_saved() {
        let mut form = Form::of(section());
        assert!(!form.dirty());
        form.edit(NAME, "renamed".to_owned());
        assert!(form.dirty());
        form.saved(form.written());
        assert!(!form.dirty());
        assert_eq!(form.value(NAME), "renamed");
    }

    #[test]
    fn a_discard_leaves_what_the_server_answered() {
        let mut form = Form::of(section());
        form.edit(NAME, "renamed".to_owned());
        form.discard();
        assert!(!form.dirty());
        assert_eq!(form.value(NAME), "held");
        assert_eq!(form.written()["Name"], serde_json::json!("held"));
    }

    #[test]
    fn a_refresh_takes_the_servers_copy_and_keeps_every_edit() {
        let mut form = Form::of(section());
        form.edit(NAME, "typed".to_owned());
        form.refreshed(serde_json::json!({"Name": "elsewhere", "Port": 9999}));
        assert!(form.dirty());
        assert_eq!(form.value(NAME), "typed");
        assert_eq!(form.value(PORT), "9999");
    }

    #[test]
    fn a_list_reads_as_the_values_it_names_and_writes_back_as_an_array() {
        let mut form = Form::of(serde_json::json!({"Paths": ["one", "two"]}));
        assert_eq!(form.listed(PATHS), vec!["one", "two"]);
        form.list(PATHS, vec!["three".to_owned()]);
        assert_eq!(form.listed(PATHS), vec!["three"]);
        assert_eq!(form.written()["Paths"], serde_json::json!(["three"]));
    }

    #[test]
    fn the_values_of_a_key_holding_no_array_read_as_none() {
        let form = Form::of(serde_json::json!({"Paths": "one"}));
        assert!(form.listed(PATHS).is_empty());
        assert!(Form::of(serde_json::json!({})).listed(PATHS).is_empty());
    }

    #[test]
    fn an_absent_key_reads_as_nothing() {
        let form = Form::of(serde_json::json!({}));
        assert_eq!(form.value(NAME), "");
        assert_eq!(form.value(PATHS), "");
    }
}
