use jellium_protocol::{Plan, PlayRequest, PlaybackRefused, Progress, Standing, Stopped};

use crate::error::{self, Answer};

/// What a play request answered.
#[derive(Debug, Clone)]
pub enum Planned {
    Plan(Box<Plan>),
    /// The change was not made and the stream that was playing still is.
    Unchanged,
    /// The local server named why it will not play this.
    Refused(PlaybackRefused),
}

pub fn endpoint(path: &str) -> String {
    format!("{}{path}", crate::page::origin())
}

/// Asks the local server for a plan for a user-initiated play, which is the one
/// door that requests this item's intros.
pub async fn enter(request: PlayRequest) -> Answer<Planned> {
    planned(jellium_protocol::PLAYBACK_ENTER_PATH, request).await
}

/// Asks the local server for a plan for a queue advance, an ended item or a
/// version change, none of which requests intros.
pub async fn start(request: PlayRequest) -> Answer<Planned> {
    planned(jellium_protocol::PLAYBACK_PATH, request).await
}

/// Asks the local server to swap the source under the session already playing,
/// which reports no stop and leaves that session standing.
pub async fn change(request: PlayRequest) -> Answer<Planned> {
    planned(jellium_protocol::PLAYBACK_CHANGE_PATH, request).await
}

/// The plan `path` answers for `request`, or the refusal it named.
async fn planned(path: &str, request: PlayRequest) -> Answer<Planned> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(endpoint(path))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(match response.json::<jellium_protocol::Planned>().await? {
                jellium_protocol::Planned::Started(plan) => Planned::Plan(plan),
                jellium_protocol::Planned::Unchanged => Planned::Unchanged,
            });
        }

        let status = response.status();
        let body = response.text().await?;
        if let Ok(Answered::Refused(refused)) = crate::failure::unraised::decoded::<Answered>(&body)
        {
            return Ok(Planned::Refused(refused));
        }
        Err(error::classify_body(status, &body).into())
    })
    .await
}

pub async fn progress(progress: Progress) -> Answer<Standing> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(endpoint(jellium_protocol::PLAYBACK_PROGRESS_PATH))
            .json(&progress)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(response.json::<Standing>().await?)
    })
    .await
}

pub async fn stopped(stopped: Stopped) -> Answer<()> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(endpoint(jellium_protocol::PLAYBACK_STOPPED_PATH))
            .json(&stopped)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(error::classify(response).await.into());
        }
        Ok(())
    })
    .await
}

/// What `/playback/plan` answered.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum Answered {
    Refused(PlaybackRefused),
    Other(serde::de::IgnoredAny),
}
