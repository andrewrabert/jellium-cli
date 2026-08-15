use std::path::{Path, PathBuf};

use uuid::Uuid;

const URL_KEY: &str = "JELLYFIN_URL";
const TOKEN_KEY: &str = "JELLYFIN_TOKEN";
const USER_ID_KEY: &str = "JELLYFIN_USER_ID";
const NAME_KEY: &str = "JELLYFIN_SERVER_NAME";
const DEVICE_ID_KEY: &str = "JELLIUM_WEB_DEVICE_ID";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub server: String,
    pub token: String,
    pub user_id: Uuid,
}

/// The credential a saved server holds, absent for one that holds none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    pub token: String,
    pub user_id: Uuid,
}

/// One saved server as the session file holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Saved {
    pub server: String,
    /// The name the server reported at its last successful probe, empty until
    /// one succeeds.
    pub name: String,
    pub credential: Option<Credential>,
}

impl Saved {
    /// The session this record authenticates with, and `None` when it holds no
    /// credential.
    pub fn session(&self) -> Option<Session> {
        let credential = self.credential.as_ref()?;
        Some(Session {
            server: self.server.clone(),
            token: credential.token.clone(),
            user_id: credential.user_id,
        })
    }
}

/// A server url in the form records are compared by: the scheme and host
/// lowercased, a default port dropped, and every trailing slash removed.
/// Text that is not an http url compares as itself, trimmed.
pub fn normalized(server: &str) -> String {
    let trimmed = server.trim();
    let Some((scheme, rest)) = trimmed.split_once("://") else {
        return trimmed.to_string();
    };
    let scheme = scheme.to_ascii_lowercase();
    let default_port = match scheme.as_str() {
        "http" => 80,
        "https" => 443,
        _ => return trimmed.to_string(),
    };
    let (authority, path) = match rest.find('/') {
        Some(cut) => (&rest[..cut], &rest[cut..]),
        None => (rest, ""),
    };
    if authority.is_empty() {
        return trimmed.to_string();
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if port.chars().all(|c| c.is_ascii_digit()) && !port.is_empty() => {
            (host, port.parse::<u16>().ok())
        }
        _ => (authority, None),
    };
    let mut out = format!("{scheme}://{}", host.to_ascii_lowercase());
    if let Some(port) = port
        && port != default_port
    {
        out.push_str(&format!(":{port}"));
    }
    out.push_str(path.trim_end_matches('/'));
    out
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("session file {path} is malformed at: {line}")]
    Malformed { path: PathBuf, line: String },
    #[error("session file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

enum Entry {
    Raw(String),
    Pair {
        key: String,
        value: String,
        /// The line as it was read, dropped once the value is set.
        source: Option<String>,
    },
}

fn needs_quotes(value: &str) -> bool {
    value.is_empty()
        || value
            .chars()
            .any(|c| c.is_whitespace() || matches!(c, '"' | '\'' | '\\' | '#'))
}

fn render(key: &str, value: &str) -> String {
    if !needs_quotes(value) {
        return format!("{key}={value}");
    }
    let mut out = format!("{key}=\"");
    for c in value.chars() {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

fn unquote(value: &str) -> String {
    let mut chars = value.chars();
    match (chars.next(), value.chars().next_back()) {
        (Some('"'), Some('"')) if value.len() >= 2 => {
            let inner = &value[1..value.len() - 1];
            let mut out = String::with_capacity(inner.len());
            let mut escaped = false;
            for c in inner.chars() {
                if escaped {
                    out.push(c);
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else {
                    out.push(c);
                }
            }
            out
        }
        (Some('\''), Some('\'')) if value.len() >= 2 => value[1..value.len() - 1].to_string(),
        _ => value.to_string(),
    }
}

async fn blocking<T, F>(work: F) -> Result<T, SessionError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, SessionError> + Send + 'static,
{
    match tokio::task::spawn_blocking(work).await {
        Ok(outcome) => outcome,
        Err(joined) => std::panic::resume_unwind(joined.into_panic()),
    }
}

/// An ordered key/value document; keys this module does not know survive a
/// load and a save unchanged.
#[derive(Default)]
pub struct SessionFile {
    entries: Vec<Entry>,
}

fn record_key(base: &str, record: usize) -> String {
    if record == 0 {
        base.to_string()
    } else {
        format!("{base}_{record}")
    }
}

impl SessionFile {
    pub fn default_path() -> PathBuf {
        dirs::data_dir()
            .expect("could not determine data directory")
            .join("jellium-cli")
            .join("session.env")
    }

    pub fn load(path: &Path) -> Result<SessionFile, SessionError> {
        let data = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(SessionFile::default());
            }
            Err(source) => {
                return Err(SessionError::Io {
                    path: path.to_path_buf(),
                    source,
                });
            }
        };

        let mut entries = Vec::new();
        for line in data.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                entries.push(Entry::Raw(line.to_string()));
                continue;
            }
            let (key, value) = trimmed
                .split_once('=')
                .ok_or_else(|| SessionError::Malformed {
                    path: path.to_path_buf(),
                    line: line.to_string(),
                })?;
            entries.push(Entry::Pair {
                key: key.trim().to_string(),
                value: unquote(value.trim()),
                source: Some(line.to_string()),
            });
        }
        Ok(SessionFile { entries })
    }

    /// Holds an exclusive lock on `<path>.lock` across the load, the change and
    /// the write, so two clients never drop each other's keys.
    pub fn update<T>(
        path: &Path,
        change: impl FnOnce(&mut SessionFile) -> T,
    ) -> Result<T, SessionError> {
        let io = |source| SessionError::Io {
            path: path.to_path_buf(),
            source,
        };

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(io)?;
        }

        let mut lock_path = path.as_os_str().to_os_string();
        lock_path.push(".lock");
        let lock = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(PathBuf::from(lock_path))
            .map_err(io)?;
        <std::fs::File as fs4::FileExt>::lock(&lock).map_err(io)?;

        let mut file = SessionFile::load(path)?;
        let outcome = change(&mut file);
        file.write(path)?;
        Ok(outcome)
    }

    /// Runs [`SessionFile::load`] on the blocking pool.
    pub async fn load_async(path: PathBuf) -> Result<SessionFile, SessionError> {
        blocking(move || SessionFile::load(&path)).await
    }

    /// Runs [`SessionFile::update`] on the blocking pool, so a flock another
    /// process holds never stalls a runtime worker. A panic inside `change` is
    /// resumed on the caller.
    pub async fn update_async<T, F>(path: PathBuf, change: F) -> Result<T, SessionError>
    where
        T: Send + 'static,
        F: FnOnce(&mut SessionFile) -> T + Send + 'static,
    {
        blocking(move || SessionFile::update(&path, change)).await
    }

    fn write(&self, path: &Path) -> Result<(), SessionError> {
        let io = |source| SessionError::Io {
            path: path.to_path_buf(),
            source,
        };

        let mut data = String::new();
        for entry in &self.entries {
            match entry {
                Entry::Raw(line) => data.push_str(line),
                Entry::Pair { key, value, source } => match source {
                    Some(source) => data.push_str(source),
                    None => data.push_str(&render(key, value)),
                },
            }
            data.push('\n');
        }

        let mut temporary = path.as_os_str().to_os_string();
        temporary.push(".tmp");
        let temporary = PathBuf::from(temporary);

        let mut options = std::fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut handle = options.open(&temporary).map_err(io)?;
        std::io::Write::write_all(&mut handle, data.as_bytes()).map_err(io)?;
        handle.sync_all().map_err(io)?;
        drop(handle);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&temporary, std::fs::Permissions::from_mode(0o600))
                .map_err(io)?;
        }

        std::fs::rename(&temporary, path).map_err(io)
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find_map(|entry| match entry {
            Entry::Pair { key: k, value, .. } if k == key => Some(value.as_str()),
            _ => None,
        })
    }

    fn set(&mut self, key: &str, value: String) {
        for entry in &mut self.entries {
            if let Entry::Pair {
                key: k,
                value: v,
                source,
            } = entry
                && k == key
            {
                *v = value;
                *source = None;
                return;
            }
        }
        self.entries.push(Entry::Pair {
            key: key.to_string(),
            value,
            source: None,
        });
    }

    fn remove(&mut self, key: &str) {
        self.entries
            .retain(|entry| !matches!(entry, Entry::Pair { key: k, .. } if k == key));
    }

    pub fn server(&self, record: usize) -> Option<Session> {
        self.saved(record)?.session()
    }

    /// Record `record`, and `None` past the last one; a record exists when its
    /// url key does, whether or not it holds a credential.
    pub fn saved(&self, record: usize) -> Option<Saved> {
        let server = self.get(&record_key(URL_KEY, record))?.to_string();
        let credential = match (
            self.get(&record_key(TOKEN_KEY, record)),
            self.get(&record_key(USER_ID_KEY, record))
                .and_then(|id| id.parse().ok()),
        ) {
            (Some(token), Some(user_id)) => Some(Credential {
                token: token.to_string(),
                user_id,
            }),
            _ => None,
        };
        Some(Saved {
            server,
            name: self
                .get(&record_key(NAME_KEY, record))
                .unwrap_or_default()
                .to_string(),
            credential,
        })
    }

    /// Every record, in file order, which is most-recently-selected order.
    pub fn records(&self) -> Vec<Saved> {
        (0..).map_while(|record| self.saved(record)).collect()
    }

    /// The record `server` is saved at, compared by [`normalized`].
    pub fn find(&self, server: &str) -> Option<usize> {
        let wanted = normalized(server);
        self.records()
            .iter()
            .position(|saved| normalized(&saved.server) == wanted)
    }

    /// The active server's session — record 0's — and `None` when record 0
    /// holds no credential, whatever later records hold.
    pub fn active(&self) -> Option<Session> {
        self.server(0)
    }

    /// Writes `records` over the keys this module knows, clearing every key of
    /// every record past the last one.
    fn rewrite(&mut self, records: &[Saved]) {
        for (record, saved) in records.iter().enumerate() {
            self.set(&record_key(URL_KEY, record), saved.server.clone());
            match &saved.credential {
                Some(credential) => {
                    self.set(&record_key(TOKEN_KEY, record), credential.token.clone());
                    self.set(
                        &record_key(USER_ID_KEY, record),
                        credential.user_id.to_string(),
                    );
                }
                None => {
                    self.remove(&record_key(TOKEN_KEY, record));
                    self.remove(&record_key(USER_ID_KEY, record));
                }
            }
            if saved.name.is_empty() {
                self.remove(&record_key(NAME_KEY, record));
            } else {
                self.set(&record_key(NAME_KEY, record), saved.name.clone());
            }
        }
        for record in records.len().. {
            if self.get(&record_key(URL_KEY, record)).is_none() {
                break;
            }
            self.remove(&record_key(URL_KEY, record));
            self.remove(&record_key(TOKEN_KEY, record));
            self.remove(&record_key(USER_ID_KEY, record));
            self.remove(&record_key(NAME_KEY, record));
        }
    }

    /// Moves `record` to the front, which is what selecting, adding and
    /// signing in do; only the keys this module knows are rewritten, so a
    /// foreign key keeps its text and its place.
    pub fn activate(&mut self, record: usize) {
        let mut records = self.records();
        if record == 0 || record >= records.len() {
            return;
        }
        let moved = records.remove(record);
        records.insert(0, moved);
        self.rewrite(&records);
    }

    /// Saves `server` at the front with no credential, and moves the record
    /// already holding it to the front rather than writing a second.
    pub fn add_server(&mut self, server: &str) {
        if let Some(record) = self.find(server) {
            self.activate(record);
            return;
        }
        let mut records = self.records();
        records.insert(
            0,
            Saved {
                server: server.to_string(),
                name: String::new(),
                credential: None,
            },
        );
        self.rewrite(&records);
    }

    /// Writes `session`'s credential at the record its server is saved at,
    /// adding a record when none holds it, and moves that record to the front.
    pub fn set_server(&mut self, session: &Session) {
        let credential = Some(Credential {
            token: session.token.clone(),
            user_id: session.user_id,
        });
        let mut records = self.records();
        match self.find(&session.server) {
            Some(record) => {
                let mut moved = records.remove(record);
                moved.server = session.server.clone();
                moved.credential = credential;
                records.insert(0, moved);
            }
            None => records.insert(
                0,
                Saved {
                    server: session.server.clone(),
                    name: String::new(),
                    credential,
                },
            ),
        }
        self.rewrite(&records);
    }

    /// Clears `record`'s credential and leaves the server saved.
    pub fn clear_credential(&mut self, record: usize) {
        let mut records = self.records();
        let Some(saved) = records.get_mut(record) else {
            return;
        };
        saved.credential = None;
        self.rewrite(&records);
    }

    /// Deletes `record` and moves every later record down one, so no reader
    /// finds a gap.
    pub fn remove_server(&mut self, record: usize) {
        let mut records = self.records();
        if record >= records.len() {
            return;
        }
        records.remove(record);
        self.rewrite(&records);
    }

    /// Writes the name `record`'s server reported, and only when it differs
    /// from the one stored; answers true when it wrote.
    pub fn set_name(&mut self, record: usize, name: &str) -> bool {
        let mut records = self.records();
        let Some(saved) = records.get_mut(record) else {
            return false;
        };
        if saved.name == name {
            return false;
        }
        saved.name = name.to_string();
        self.rewrite(&records);
        true
    }

    pub fn device_id(&mut self) -> Uuid {
        if let Some(id) = self.get(DEVICE_ID_KEY).and_then(|v| v.parse().ok()) {
            return id;
        }
        let id = Uuid::new_v4();
        self.set(DEVICE_ID_KEY, id.to_string());
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-session-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    fn saved(server: &str, name: &str, token: Option<&str>) -> Saved {
        Saved {
            server: server.to_string(),
            name: name.to_string(),
            credential: token.map(|token| Credential {
                token: token.to_string(),
                user_id: Uuid::nil(),
            }),
        }
    }

    #[test]
    fn a_multi_record_file_round_trips_with_foreign_and_unknown_per_record_keys_intact() {
        let path = scratch("multi-record");
        std::fs::write(
            &path,
            "FOREIGN=kept\nJELLYFIN_URL=https://one.test\nJELLYFIN_TOKEN=one\n\
             JELLYFIN_USER_ID=00000000-0000-0000-0000-000000000000\n\
             JELLYFIN_SERVER_NAME=One\nJELLYFIN_URL_1=https://two.test\n\
             JELLYFIN_EXTRA_1=also-kept\n",
        )
        .expect("seed");

        SessionFile::update(&path, |file| file.activate(1)).expect("activate");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(
            loaded.records(),
            vec![
                saved("https://two.test", "", None),
                saved("https://one.test", "One", Some("one")),
            ]
        );
        assert_eq!(loaded.get("FOREIGN"), Some("kept"));
        assert_eq!(loaded.get("JELLYFIN_EXTRA_1"), Some("also-kept"));
    }

    #[test]
    fn a_record_holding_no_credential_hides_no_later_record() {
        let path = scratch("credentialless");
        SessionFile::update(&path, |file| {
            file.add_server("https://two.test");
            file.set_server(&Session {
                server: "https://one.test".to_string(),
                token: "one".to_string(),
                user_id: Uuid::nil(),
            });
            file.activate(1);
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.records().len(), 2);
        assert_eq!(loaded.active(), None);
        assert_eq!(
            loaded
                .saved(1)
                .and_then(|saved| saved.session())
                .map(|s| s.server),
            Some("https://one.test".to_string())
        );
    }

    #[test]
    fn removing_a_record_moves_every_later_record_down_one() {
        let path = scratch("compaction");
        SessionFile::update(&path, |file| {
            file.add_server("https://three.test");
            file.add_server("https://two.test");
            file.add_server("https://one.test");
            file.remove_server(1);
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(
            loaded
                .records()
                .into_iter()
                .map(|saved| saved.server)
                .collect::<Vec<_>>(),
            vec![
                "https://one.test".to_string(),
                "https://three.test".to_string()
            ]
        );
    }

    #[test]
    fn selecting_a_record_makes_it_the_one_an_unmodified_reader_finds() {
        let path = scratch("selection");
        SessionFile::update(&path, |file| {
            file.set_server(&Session {
                server: "https://one.test".to_string(),
                token: "one".to_string(),
                user_id: Uuid::nil(),
            });
            file.set_server(&Session {
                server: "https://two.test".to_string(),
                token: "two".to_string(),
                user_id: Uuid::nil(),
            });
            file.activate(1);
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.active().expect("active").server, "https://one.test");
        assert_eq!(loaded.server(0).expect("record").token, "one");
    }

    #[test]
    fn a_url_normalizing_to_a_saved_one_is_found_rather_than_added_twice() {
        let path = scratch("normalizing");
        SessionFile::update(&path, |file| {
            file.add_server("https://Example.test:443/");
            file.add_server("https://example.test");
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.records().len(), 1);
        assert_eq!(loaded.find("HTTPS://EXAMPLE.TEST///"), Some(0));
    }

    #[test]
    fn a_name_is_written_only_when_it_differs_from_the_one_stored() {
        let path = scratch("naming");
        SessionFile::update(&path, |file| file.add_server("https://one.test")).expect("seed");

        assert!(SessionFile::update(&path, |file| file.set_name(0, "One")).expect("first"));
        assert!(!SessionFile::update(&path, |file| file.set_name(0, "One")).expect("second"));
        assert!(SessionFile::update(&path, |file| file.set_name(0, "Renamed")).expect("third"));

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.saved(0).expect("record").name, "Renamed");
    }

    #[test]
    fn clearing_a_credential_leaves_the_server_saved() {
        let path = scratch("clearing");
        SessionFile::update(&path, |file| {
            file.set_server(&Session {
                server: "https://one.test".to_string(),
                token: "one".to_string(),
                user_id: Uuid::nil(),
            });
            file.set_name(0, "One");
            file.clear_credential(0);
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.active(), None);
        assert_eq!(
            loaded.records(),
            vec![saved("https://one.test", "One", None)]
        );
    }

    #[test]
    fn a_quoted_foreign_key_survives_a_load_and_a_save() {
        let path = scratch("foreign-key");
        std::fs::write(&path, "FOREIGN=\"a value # with marks\"\n").expect("seed");

        SessionFile::update(&path, |file| {
            file.set_server(&Session {
                server: "https://example.test".to_string(),
                token: "token".to_string(),
                user_id: Uuid::nil(),
            })
        })
        .expect("login write");
        SessionFile::update(&path, |file| file.remove_server(0)).expect("logout write");

        let data = std::fs::read_to_string(&path).expect("read back");
        assert_eq!(data, "FOREIGN=\"a value # with marks\"\n");
    }

    #[test]
    fn a_value_needing_quotes_reloads_as_itself() {
        let path = scratch("quoting");
        let awkward = "a \"quoted\" \\ value # here";

        SessionFile::update(&path, |file| {
            file.set_server(&Session {
                server: awkward.to_string(),
                token: "token".to_string(),
                user_id: Uuid::nil(),
            })
        })
        .expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.server(0).expect("record").server, awkward);
    }

    #[cfg(unix)]
    #[test]
    fn a_save_leaves_an_existing_file_at_mode_0600() {
        use std::os::unix::fs::PermissionsExt as _;

        let path = scratch("mode");
        std::fs::write(&path, "FOREIGN=value\n").expect("seed");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).expect("chmod");

        SessionFile::update(&path, |file| file.device_id()).expect("write");

        let mode = std::fs::metadata(&path).expect("stat").permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn an_update_waiting_on_the_lock_leaves_the_runtime_running() {
        let path = scratch("blocking-lock");

        let mut lock_path = path.as_os_str().to_os_string();
        lock_path.push(".lock");
        let lock = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(PathBuf::from(lock_path))
            .expect("lock file");
        <std::fs::File as fs4::FileExt>::lock(&lock).expect("hold the lock");

        let waiting = tokio::spawn(SessionFile::update_async(path.clone(), |file| {
            file.device_id()
        }));

        for _ in 0..64 {
            tokio::task::yield_now().await;
        }
        assert!(!waiting.is_finished());

        <std::fs::File as fs4::FileExt>::unlock(&lock).expect("release the lock");
        let device = waiting.await.expect("join").expect("write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.get(DEVICE_ID_KEY), Some(device.to_string().as_str()));
    }

    #[test]
    fn two_updates_of_one_file_keep_both_keys() {
        let path = scratch("concurrent");

        let device = SessionFile::update(&path, |file| file.device_id()).expect("device write");
        SessionFile::update(&path, |file| {
            file.set_server(&Session {
                server: "https://example.test".to_string(),
                token: "token".to_string(),
                user_id: Uuid::nil(),
            })
        })
        .expect("login write");

        let loaded = SessionFile::load(&path).expect("load");
        assert_eq!(loaded.get(DEVICE_ID_KEY), Some(device.to_string().as_str()));
        assert_eq!(
            loaded.server(0).expect("record").server,
            "https://example.test"
        );
    }
}
