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
        #[expect(
            clippy::disallowed_methods,
            reason = "a body that is not a playback refusal is how a plan answer is classified"
        )]
        let read = serde_json::from_str::<PlaybackRefused>(&body);
        if let Ok(refused) = read {
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
