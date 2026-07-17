use std::path::{Path, PathBuf};

use uuid::Uuid;

pub struct Session {
    pub server: String,
    pub token: String,
    pub user_id: Uuid,
}

pub fn session_path() -> PathBuf {
    dirs::data_dir()
        .expect("could not determine data directory")
        .join("jellium-cli")
        .join("session.env")
}

pub fn load_session() -> Result<Session, Box<dyn std::error::Error>> {
    let path = session_path();
    let data = std::fs::read_to_string(&path)
        .map_err(|_| "no saved session; run `jellium-cli login` first")?;

    let mut server = None;
    let mut token = None;
    let mut user_id = None;

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("invalid session line: {line}"))?;
        let value = value.trim().trim_matches('"');
        match key.trim() {
            "JELLYFIN_URL" => server = Some(value.to_string()),
            "JELLYFIN_TOKEN" => token = Some(value.to_string()),
            "JELLYFIN_USER_ID" => user_id = Some(value.parse()?),
            _ => {}
        }
    }

    Ok(Session {
        server: server.ok_or("session missing JELLYFIN_URL")?,
        token: token.ok_or("session missing JELLYFIN_TOKEN")?,
        user_id: user_id.ok_or("session missing JELLYFIN_USER_ID")?,
    })
}

pub fn save_session_to(path: &Path, session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let data = format!(
        "JELLYFIN_URL={}\nJELLYFIN_TOKEN={}\nJELLYFIN_USER_ID={}\n",
        session.server, session.token, session.user_id,
    );
    std::fs::write(path, data)?;
    Ok(())
}
