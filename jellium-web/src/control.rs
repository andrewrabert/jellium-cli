use jellium_protocol::{
    AddServer, ChooseServer, Credentials, Framed, Identity, PageRequest, PinOutcome,
    QuickConnectCode, QuickConnectState, Removed, ResetAnswer, ResetPin, ResetRequest, SavedServer,
    SessionStatus, SetupConfiguration, SetupRemoteAccess, SetupUser,
};
use uuid::Uuid;

use crate::error::{self, Answer, Trouble};
use crate::text::Text;

fn origin() -> String {
    web_sys::window()
        .expect("a browser window")
        .location()
        .origin()
        .expect("the page has an origin")
}

fn endpoint() -> String {
    format!("{}{}", origin(), jellium_protocol::IDENTITY_PATH)
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

/// Announces what this browser is, which is what every upstream request the
/// local server then issues is identified by, and reads back the session
/// status.
pub async fn announce(identity: &Identity) -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(endpoint())
            .json(identity)
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

fn at(path: &str) -> String {
    format!("{}{path}", origin())
}

/// `{path}?target={target}`, which every login-stage request carries.
fn aimed(path: &str, target: &str) -> String {
    format!(
        "{}?{}={}",
        at(path),
        jellium_protocol::TARGET_QUERY,
        urlencoded(target)
    )
}

/// Percent-encodes the bytes a query value may not carry literally.
fn urlencoded(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// One typed read whose body the caller decodes.
async fn decoded<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, Trouble> {
    if !response.status().is_success() {
        return Err(error::classify(response).await);
    }
    Ok(response.json::<T>().await?)
}

/// One typed request answering no body.
async fn acknowledged(response: reqwest::Response) -> Result<(), Trouble> {
    if response.status().is_success() {
        return Ok(());
    }
    Err(error::classify(response).await)
}

pub async fn servers() -> Answer<Vec<SavedServer>> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .get(at(jellium_protocol::SERVERS_PATH))
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

pub async fn add_server(url: String) -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(at(jellium_protocol::SERVERS_PATH))
            .json(&AddServer { url })
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

pub async fn select_server(server: String) -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(at(jellium_protocol::SERVER_SELECT_PATH))
            .json(&ChooseServer { server })
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

pub async fn remove_server(server: String) -> Answer<Removed> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .delete(at(jellium_protocol::SERVERS_PATH))
            .json(&ChooseServer { server })
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

pub async fn switch_server() -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(at(jellium_protocol::SWITCH_PATH))
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

pub async fn sign_in(target: String, credentials: Credentials) -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(aimed(jellium_protocol::LOGIN_PATH, &target))
            .json(&credentials)
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

/// Releases the login target, which is what Back off a login screen does; a
/// trouble is raised as a failure here, and nothing is answered.
pub async fn leave_login(target: String) {
    Answer::of(async {
        let response = reqwest::Client::new()
            .delete(aimed(jellium_protocol::LOGIN_PATH, &target))
            .send()
            .await?;
        Ok(acknowledged(response).await?)
    })
    .await
    .disregarded(Text::FailureLoginLeft);
}

pub async fn quick_connect_initiate(target: String) -> Answer<QuickConnectCode> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(aimed(jellium_protocol::QUICK_CONNECT_PATH, &target))
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

pub async fn quick_connect_poll(target: String) -> Answer<QuickConnectState> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .get(aimed(jellium_protocol::QUICK_CONNECT_PATH, &target))
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

/// Abandons the Quick Connect attempt; a trouble is raised as a failure here,
/// and nothing is answered.
pub async fn quick_connect_abandon(target: String) {
    Answer::of(async {
        let response = reqwest::Client::new()
            .delete(aimed(jellium_protocol::QUICK_CONNECT_PATH, &target))
            .send()
            .await?;
        Ok(acknowledged(response).await?)
    })
    .await
    .disregarded(Text::FailureQuickConnectAbandoned);
}

pub async fn forgot_password(target: String, username: String) -> Answer<ResetAnswer> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(aimed(jellium_protocol::RESET_PATH, &target))
            .json(&ResetRequest { username })
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

pub async fn redeem_pin(target: String, pin: String) -> Answer<PinOutcome> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(aimed(jellium_protocol::RESET_PIN_PATH, &target))
            .json(&ResetPin { pin })
            .send()
            .await?;
        Ok(decoded(response).await?)
    })
    .await
}

/// One public user's primary image, fetched from the login stage's own
/// endpoint.
pub async fn public_image(target: String, user: Uuid) -> Answer<Vec<u8>> {
    Answer::of(async {
        let path = format!("{}/{user}/image", jellium_protocol::LOGIN_IMAGE_PREFIX);
        let response = reqwest::Client::new()
            .get(aimed(&path, &target))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(response.bytes().await?.to_vec())
    })
    .await
}

pub async fn logout() -> Answer<()> {
    Answer::of(async {
        let response = reqwest::Client::new().delete(endpoint()).send().await?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(error::classify(response).await.into())
    })
    .await
}

fn page_endpoint() -> String {
    format!("{}{}", origin(), jellium_protocol::PLUGIN_PATH)
}

/// Mints a grant and answers the frame's path.
pub async fn open_page(name: String) -> Answer<Framed> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(page_endpoint())
            .json(&PageRequest { name })
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(response.json::<Framed>().await?)
    })
    .await
}

/// Releases a grant.
pub async fn close_page(framed: Framed) -> Answer<()> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .delete(page_endpoint())
            .json(&framed)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(())
    })
    .await
}

fn setup_endpoint(path: &str) -> String {
    format!("{}{path}", origin())
}

/// One `/setup/*` read.
async fn setup_read<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, Trouble> {
    let response = reqwest::Client::new()
        .get(setup_endpoint(path))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(error::classify(response).await);
    }
    Ok(response.json::<T>().await?)
}

/// One `/setup/*` write, which answers no body.
async fn setup_write(path: &str, body: &impl serde::Serialize) -> Result<(), Trouble> {
    let response = reqwest::Client::new()
        .post(setup_endpoint(path))
        .json(body)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(error::classify(response).await);
    }
    Ok(())
}

pub async fn setup_configuration() -> Answer<SetupConfiguration> {
    Answer::of(async { Ok(setup_read(jellium_protocol::SETUP_CONFIGURATION_PATH).await?) }).await
}

pub async fn set_setup_configuration(configuration: SetupConfiguration) -> Answer<()> {
    Answer::of(async {
        Ok(setup_write(jellium_protocol::SETUP_CONFIGURATION_PATH, &configuration).await?)
    })
    .await
}

pub async fn setup_user() -> Answer<SetupUser> {
    Answer::of(async { Ok(setup_read(jellium_protocol::SETUP_USER_PATH).await?) }).await
}

pub async fn set_setup_user(user: SetupUser) -> Answer<()> {
    Answer::of(async { Ok(setup_write(jellium_protocol::SETUP_USER_PATH, &user).await?) }).await
}

pub async fn setup_remote_access() -> Answer<SetupRemoteAccess> {
    Answer::of(async { Ok(setup_read(jellium_protocol::SETUP_REMOTE_ACCESS_PATH).await?) }).await
}

pub async fn set_setup_remote_access(access: SetupRemoteAccess) -> Answer<()> {
    Answer::of(async {
        Ok(setup_write(jellium_protocol::SETUP_REMOTE_ACCESS_PATH, &access).await?)
    })
    .await
}

/// Posts `Startup/Complete` and answers the session the sign-in that follows
/// it installed.
pub async fn complete_setup() -> Answer<SessionStatus> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(setup_endpoint(jellium_protocol::SETUP_COMPLETE_PATH))
            .send()
            .await?;
        Ok(read(response).await?)
    })
    .await
}

/// Releases the setup upstream.
pub async fn leave_setup() -> Answer<()> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .delete(setup_endpoint(jellium_protocol::SETUP_PATH))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(())
    })
    .await
}
