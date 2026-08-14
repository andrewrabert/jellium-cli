#![allow(clippy::too_many_arguments)]

mod client;
pub mod error;
pub mod pagination;
mod request;
pub mod types;
mod util;

// the Jellyfin server version of the OpenAPI document this crate was
// generated from; re-record it whenever the crate is regenerated
pub const SNAPSHOT_VERSION: &str = "12.0.0";

pub use client::Client;

#[cfg(not(target_arch = "wasm32"))]
pub use client::RawResponse;
