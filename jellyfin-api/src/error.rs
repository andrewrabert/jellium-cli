#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("failed to deserialize response: {source}\nbody: {body}")]
    Deserialization {
        source: serde_json::Error,
        body: String,
    },
    #[error("HTTP {status}: {body}")]
    Status {
        status: reqwest::StatusCode,
        body: String,
    },
}
