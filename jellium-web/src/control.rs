use jellium_protocol::{Credentials, SessionStatus};

use crate::error::{self, Trouble};

fn origin() -> String {
    web_sys::window()
        .expect("a browser window")
        .location()
        .origin()
        .expect("the page has an origin")
}

fn endpoint() -> String {
    format!("{}{}", origin(), jellium_protocol::SESSION_PATH)
}

/// A 2xx answers with the status document, a `Failure` body becomes
/// `SessionStatus::Failed`, and every other refusal is an error.
async fn read(response: reqwest::Response) -> Result<SessionStatus, Trouble> {
    if response.status().is_success() {
        return Ok(response.json::<SessionStatus>().await?);
    }
    match error::classify(response).await {
        Trouble::Upstream(failure) => Ok(SessionStatus::Failed(failure)),
        trouble => Err(trouble),
    }
}

pub async fn status() -> Result<SessionStatus, Trouble> {
    let response = reqwest::Client::new().get(endpoint()).send().await?;
    read(response).await
}

pub async fn login(credentials: Credentials) -> Result<SessionStatus, Trouble> {
    let response = reqwest::Client::new()
        .post(endpoint())
        .json(&credentials)
        .send()
        .await?;
    read(response).await
}

pub async fn logout() -> Result<(), Trouble> {
    let response = reqwest::Client::new().delete(endpoint()).send().await?;
    if response.status().is_success() {
        return Ok(());
    }
    Err(error::classify(response).await)
}
