use jellium_protocol::{Plan, PlayRequest, PlaybackRefused, Progress, Standing, Stopped};

use crate::error::{self, Answer};

/// What a play request answered.
#[derive(Debug, Clone)]
pub enum Planned {
    Plan(Box<Plan>),
    /// The local server named why it will not play this.
    Refused(PlaybackRefused),
}

pub fn endpoint(path: &str) -> String {
    let origin = web_sys::window()
        .expect("a browser window")
        .location()
        .origin()
        .expect("the page has an origin");
    format!("{origin}{path}")
}

pub async fn start(request: PlayRequest) -> Answer<Planned> {
    Answer::of(async {
        let response = reqwest::Client::new()
            .post(endpoint(jellium_protocol::PLAYBACK_PATH))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(Planned::Plan(Box::new(response.json::<Plan>().await?)));
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
