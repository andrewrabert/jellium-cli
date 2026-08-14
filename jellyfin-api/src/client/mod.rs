mod artists;
mod audio;
mod auth;
mod backup;
mod branding;
mod channels;
mod collections;
mod devices;
mod display_preferences;
mod environment;
mod genres;
mod images;
mod items;
mod library;
mod live_tv;
mod localization;
mod music_genres;
mod packages;
mod persons;
mod playback;
mod playlists;
mod plugins;
mod providers;
mod quick_connect;
mod scheduled_tasks;
mod sessions;
mod shows;
mod startup;
mod studios;
mod sync_play;
mod system;
mod user_items;
mod users;
mod videos;
mod web;

pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
    pub(crate) verbose: bool,
}

impl Client {
    pub fn new(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

/// Response from a raw API request.
#[cfg(not(target_arch = "wasm32"))]
pub struct RawResponse {
    pub status: u16,
    pub http_version: &'static str,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Client {
    /// Send a raw API request, returning the full response without status checking.
    pub async fn raw_request(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: &[(reqwest::header::HeaderName, String)],
        body: Option<&[u8]>,
    ) -> Result<RawResponse, reqwest::Error> {
        let url = if path.starts_with(&self.baseurl) {
            path.to_string()
        } else {
            format!("{}{}", self.baseurl, path)
        };
        if self.verbose {
            eprintln!("{} {url}", method);
        }
        let mut req = self.client.request(method, &url);
        for (name, value) in headers {
            req = req.header(name, value);
        }
        if let Some(body) = body {
            if self.verbose {
                eprintln!("{}", String::from_utf8_lossy(body));
            }
            req = req.body(body.to_vec());
        }
        let response = req.send().await?;
        let status = response.status().as_u16();
        let http_version = match response.version() {
            reqwest::Version::HTTP_09 => "HTTP/0.9",
            reqwest::Version::HTTP_10 => "HTTP/1.0",
            reqwest::Version::HTTP_11 => "HTTP/1.1",
            reqwest::Version::HTTP_2 => "HTTP/2",
            reqwest::Version::HTTP_3 => "HTTP/3",
            _ => "HTTP/?",
        };
        let resp_headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
            .collect();
        if self.verbose {
            eprintln!("< {status}");
        }
        let bytes = response.bytes().await?.to_vec();
        if self.verbose && !bytes.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&bytes));
        }
        Ok(RawResponse {
            status,
            http_version,
            headers: resp_headers,
            body: bytes,
        })
    }
}
