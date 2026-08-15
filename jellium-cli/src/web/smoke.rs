//! The one test that boots the real bundle in a real browser: the stub
//! upstream and the local server on ephemeral ports, driven by headless
//! Chromium over WebDriver.

use std::sync::Arc;

use super::{AppState, origin, server, upstream};

/// The longest the test waits on the browser before failing.
const BOUND: std::time::Duration = std::time::Duration::from_secs(30);

/// The stub upstream, the local server serving the embedded bundle, and the
/// entry url the browser is pointed at.
struct Running {
    entry: String,
    _upstream: upstream::Answering,
    _server: tokio::task::JoinHandle<()>,
    _session: tempfile::TempDir,
}

/// Serves the synthetic upstream and a local server whose session file holds
/// one saved server naming it, both on ephemeral ports.
/// The local server keeps the listener it was given, so no port is bound,
/// released and bound again.
async fn running() -> Running {
    let answering = upstream::answering(200).await;
    let session = tempfile::tempdir().expect("a scratch directory for the session file");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a loopback listener");
    let bound = listener.local_addr().expect("the bound address");

    let mut state = AppState::against(session.path().join("session.env"), &answering.base);
    state.origin = origin::Host::of(std::net::Ipv4Addr::LOCALHOST.into()).origin(bound.port());
    let (secret, entry_secret) = super::guard::Secret::mint();
    state.secret = secret;

    let served = server::Server::on(listener, Arc::new(state), entry_secret).await;
    let entry = served.entry_url();

    Running {
        entry,
        _upstream: answering,
        _server: tokio::spawn(async move {
            let _ = served.serve().await;
        }),
        _session: session,
    }
}

/// A headless Chromium session, with chromedriver launched for the test and
/// torn down with it.
struct Driver {
    endpoint: String,
    session: String,
    client: reqwest::Client,
    chromedriver: std::process::Child,
}

impl Driver {
    /// Launches `chromedriver` on a free port and opens a headless session
    /// recording the browser log.
    /// A chromedriver that exits before answering is retried on another port
    /// five times, so a port taken between the probe and the launch does not
    /// fail the test.
    /// A `chromedriver` that is not on the path fails the test naming it.
    async fn open() -> Driver {
        let binary = std::env::var("CHROMEDRIVER").unwrap_or_else(|_| "chromedriver".to_owned());
        let client = reqwest::Client::new();
        let mut launched = None;
        for _ in 0..5 {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a free port");
            let port = listener.local_addr().expect("the bound address").port();
            drop(listener);

            let mut chromedriver = std::process::Command::new(&binary)
                .arg(format!("--port={port}"))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .unwrap_or_else(|error| {
                    panic!("could not run {binary}: {error}; set CHROMEDRIVER to its path")
                });

            let endpoint = format!("http://127.0.0.1:{port}");
            let ready = std::time::Instant::now();
            let answered = loop {
                if client
                    .get(format!("{endpoint}/status"))
                    .send()
                    .await
                    .is_ok_and(|answer| answer.status().is_success())
                {
                    break true;
                }
                if matches!(chromedriver.try_wait(), Ok(Some(_))) {
                    break false;
                }
                assert!(ready.elapsed() < BOUND, "chromedriver never answered");
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            };
            if answered {
                launched = Some((endpoint, chromedriver));
                break;
            }
        }
        let (endpoint, chromedriver) =
            launched.expect("chromedriver never held a port over five attempts");

        let opened: serde_json::Value = client
            .post(format!("{endpoint}/session"))
            .json(&serde_json::json!({
                "capabilities": {
                    "alwaysMatch": {
                        "goog:chromeOptions": {
                            "args": [
                                "--headless=new",
                                "--no-sandbox",
                                "--disable-dev-shm-usage",
                                "--enable-unsafe-swiftshader",
                                "--window-size=1280,800",
                            ],
                        },
                        "goog:loggingPrefs": { "browser": "ALL" },
                    }
                }
            }))
            .send()
            .await
            .expect("chromedriver takes the session request")
            .json()
            .await
            .expect("chromedriver answers the session request with json");
        let session = opened["value"]["sessionId"]
            .as_str()
            .unwrap_or_else(|| panic!("chromedriver opened no session: {opened}"))
            .to_owned();

        Driver {
            endpoint,
            session,
            client,
            chromedriver,
        }
    }

    fn at(&self, path: &str) -> String {
        format!("{}/session/{}{path}", self.endpoint, self.session)
    }

    /// Navigates to `url` and returns once the document has loaded.
    async fn go(&self, url: &str) {
        self.client
            .post(self.at("/url"))
            .json(&serde_json::json!({ "url": url }))
            .send()
            .await
            .expect("the browser takes the navigation");
    }

    /// What `script` evaluates to.
    async fn eval(&self, script: &str) -> serde_json::Value {
        let answered: serde_json::Value = self
            .client
            .post(self.at("/execute/sync"))
            .json(&serde_json::json!({ "script": script, "args": [] }))
            .send()
            .await
            .expect("the browser takes the script")
            .json()
            .await
            .expect("the browser answers the script with json");
        answered["value"].clone()
    }

    /// Polls `script` until it answers true, failing the test when `BOUND`
    /// passes first.
    async fn until(&self, script: &str, bound: std::time::Duration) {
        let waited = std::time::Instant::now();
        loop {
            if self.eval(script).await == serde_json::Value::Bool(true) {
                return;
            }
            assert!(
                waited.elapsed() < bound,
                "the browser never answered true to {script}; the console holds {:?}",
                self.errors().await
            );
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// The viewport as PNG bytes.
    async fn screenshot(&self) -> Vec<u8> {
        use base64::Engine;
        let answered: serde_json::Value = self
            .client
            .get(self.at("/screenshot"))
            .send()
            .await
            .expect("the browser takes the screenshot request")
            .json()
            .await
            .expect("the browser answers the screenshot with json");
        base64::engine::general_purpose::STANDARD
            .decode(answered["value"].as_str().expect("a base64 screenshot"))
            .expect("the screenshot decodes")
    }

    /// Polls the viewport until it holds more than one distinct colour,
    /// failing the test when `bound` passes first, so a shot taken before the
    /// first present never decides the outcome.
    async fn painted(&self, bound: std::time::Duration) {
        let waited = std::time::Instant::now();
        loop {
            let shot = image::load_from_memory(&self.screenshot().await)
                .expect("the screenshot is a png")
                .to_rgb8();
            let colours: std::collections::HashSet<[u8; 3]> =
                shot.pixels().map(|pixel| pixel.0).collect();
            if colours.len() > 1 {
                return;
            }
            assert!(
                waited.elapsed() < bound,
                "the canvas painted one colour alone, so nothing was drawn; \
                 the console holds {:?}",
                self.errors().await
            );
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Every browser-log entry the session recorded at SEVERE.
    async fn errors(&self) -> Vec<String> {
        let answered: serde_json::Value = self
            .client
            .post(self.at("/se/log"))
            .json(&serde_json::json!({ "type": "browser" }))
            .send()
            .await
            .expect("the browser takes the log request")
            .json()
            .await
            .expect("the browser answers the log with json");
        answered["value"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .filter(|entry| entry["level"].as_str() == Some("SEVERE"))
            .filter_map(|entry| entry["message"].as_str().map(str::to_owned))
            .collect()
    }
}

/// Kills chromedriver and reaps it; nothing else runs.
impl Drop for Driver {
    fn drop(&mut self) {
        let _ = self.chromedriver.kill();
        let _ = self.chromedriver.wait();
    }
}

/// The wasm module loads, the static boot screen is gone, a canvas exists with
/// non-zero size, the canvas has painted more than one distinct colour, and
/// the browser console recorded no error.
#[tokio::test]
async fn the_application_boots_in_a_browser() {
    let running = running().await;
    let driver = Driver::open().await;

    driver.go(&running.entry).await;
    driver
        .until(
            "const boot = document.getElementById('jellium-boot');\
             return boot === null || boot.hasAttribute('hidden');",
            BOUND,
        )
        .await;
    driver
        .until(
            "const canvas = document.querySelector('canvas');\
             return canvas !== null && canvas.width > 0 && canvas.height > 0;",
            BOUND,
        )
        .await;

    driver.painted(BOUND).await;

    let errors = driver.errors().await;
    assert!(errors.is_empty(), "the browser console recorded {errors:?}");
}
