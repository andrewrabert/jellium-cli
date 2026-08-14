use std::path::{Path, PathBuf};

use uuid::Uuid;

const URL_KEY: &str = "JELLYFIN_URL";
const TOKEN_KEY: &str = "JELLYFIN_TOKEN";
const USER_ID_KEY: &str = "JELLYFIN_USER_ID";
const DEVICE_ID_KEY: &str = "JELLIUM_WEB_DEVICE_ID";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub server: String,
    pub token: String,
    pub user_id: Uuid,
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
        Some(Session {
            server: self.get(&record_key(URL_KEY, record))?.to_string(),
            token: self.get(&record_key(TOKEN_KEY, record))?.to_string(),
            user_id: self.get(&record_key(USER_ID_KEY, record))?.parse().ok()?,
        })
    }

    #[allow(dead_code, reason = "the session file holds records for every client")]
    pub fn servers(&self) -> Vec<Session> {
        (0..).map_while(|record| self.server(record)).collect()
    }

    pub fn set_server(&mut self, record: usize, session: &Session) {
        self.set(&record_key(URL_KEY, record), session.server.clone());
        self.set(&record_key(TOKEN_KEY, record), session.token.clone());
        self.set(
            &record_key(USER_ID_KEY, record),
            session.user_id.to_string(),
        );
    }

    pub fn remove_server(&mut self, record: usize) {
        self.remove(&record_key(URL_KEY, record));
        self.remove(&record_key(TOKEN_KEY, record));
        self.remove(&record_key(USER_ID_KEY, record));
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

    #[test]
    fn a_quoted_foreign_key_survives_a_load_and_a_save() {
        let path = scratch("foreign-key");
        std::fs::write(&path, "FOREIGN=\"a value # with marks\"\n").expect("seed");

        SessionFile::update(&path, |file| {
            file.set_server(
                0,
                &Session {
                    server: "https://example.test".to_string(),
                    token: "token".to_string(),
                    user_id: Uuid::nil(),
                },
            )
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
            file.set_server(
                0,
                &Session {
                    server: awkward.to_string(),
                    token: "token".to_string(),
                    user_id: Uuid::nil(),
                },
            )
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
            file.set_server(
                0,
                &Session {
                    server: "https://example.test".to_string(),
                    token: "token".to_string(),
                    user_id: Uuid::nil(),
                },
            )
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
