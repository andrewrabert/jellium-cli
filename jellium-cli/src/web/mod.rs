use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

mod assets;
mod control;
mod foreign;
mod guard;
mod holder;
mod identity;
mod link;
mod live;
mod login;
mod manifest;
mod origin;
mod page;
pub mod playback;
mod relay;
mod route;
mod server;
mod setup;
#[cfg(test)]
mod smoke;
#[cfg(test)]
mod synthetic;
mod upstream;
mod version;
mod wire;

#[derive(Debug, clap::Args)]
pub struct WebArgs {
    /// Port to listen on (default: an ephemeral port)
    #[arg(long)]
    pub port: Option<u16>,

    /// Address to bind
    #[arg(long, default_value = "127.0.0.1")]
    pub bind: IpAddr,

    /// Permit a bind address outside loopback
    #[arg(long)]
    pub allow_remote: bool,

    /// Host browsers use to reach this server (default: the bind address)
    #[arg(long)]
    pub advertise: Option<String>,

    /// Print the URL instead of opening the default browser
    #[arg(long)]
    pub no_open: bool,

    /// Refuse every write to the Jellyfin server
    #[arg(long)]
    pub read_only: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum WebError {
    #[error("--bind {bind} is outside loopback; pass --allow-remote to serve it")]
    RemoteBindRefused { bind: IpAddr },
    #[error(
        "--bind {bind} accepts on every interface; pass --advertise <HOST> \
         naming the host browsers will use"
    )]
    AdvertiseRequired { bind: IpAddr },
    #[error("--advertise {host} is not a host name or an IP address")]
    AdvertiseMalformed { host: String },
    #[error("could not bind {address}: {source}")]
    Bind {
        address: SocketAddr,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Session(#[from] crate::session::SessionError),
    #[error(transparent)]
    Serve(std::io::Error),
}

pub struct AppState {
    /// True once this run posted `Startup/Complete`; every later setup request
    /// is refused from here rather than forwarded.
    pub completed: std::sync::atomic::AtomicBool,
    /// True when this instance was started `--read-only`; it refuses every
    /// relayed route declaring a write and every typed endpoint that writes.
    pub read_only: bool,
    /// The package names and configuration page names the relay has itself
    /// seen in a listing during this run.
    pub seen: route::Seen,
    /// The configuration page frames open now.
    pub pages: page::Grants,
    pub secret: guard::Secret,
    pub cookie: guard::Cookie,
    /// The one origin this server answers for, the host taken from
    /// `--advertise` when given and from `--bind` otherwise.
    pub origin: origin::Origin,
    /// The identity the browser announced, and nothing until one does.
    pub identity: identity::Announced,
    /// The held Jellyfin session and the session file it shares with the
    /// command line.
    pub session: holder::Holder,
    pub playback: playback::Playback,
    /// The connected tabs, the one upstream socket, and remote mode.
    pub live: live::Hub,
    /// The client every foreign image is fetched over, holding no credential
    /// and no identity.
    pub foreign: foreign::Anonymous,
}

#[cfg(test)]
impl AppState {
    // an app state holding nothing, its session file at `path`, minted purely
    // for router tests that drive the real relay route
    pub(crate) fn stub(path: std::path::PathBuf) -> AppState {
        AppState {
            completed: std::sync::atomic::AtomicBool::new(false),
            read_only: false,
            seen: route::Seen::new(),
            pages: page::Grants::new(),
            secret: guard::Secret::mint().0,
            cookie: guard::Cookie::mint(),
            origin: origin::Host::of(std::net::Ipv4Addr::LOCALHOST.into()).origin(0),
            identity: identity::Announced::announcing(jellium_protocol::Identity {
                device: "Firefox".to_owned(),
                device_id: uuid::Uuid::nil().to_string(),
            }),
            session: holder::Holder::new(path.clone()),
            playback: playback::Playback::new(path),
            live: live::Hub::new(),
            foreign: foreign::Anonymous::new(),
        }
    }

    // an app state whose session file holds one saved server naming `upstream`
    // and no credential, minted for the smoke test
    pub(crate) fn against(path: std::path::PathBuf, upstream: &str) -> AppState {
        crate::session::SessionFile::update(&path, |file| file.add_server(upstream))
            .expect("the smoke test's session file is writable");
        AppState::stub(path)
    }
}

/// Refuses a wildcard bind without advertise and a malformed advertise, each
/// before the port is bound.
fn advertised(args: &WebArgs) -> Result<origin::Host, WebError> {
    match &args.advertise {
        Some(host) => origin::Host::parse(host)
            .ok_or_else(|| WebError::AdvertiseMalformed { host: host.clone() }),
        None if args.bind.is_unspecified() => Err(WebError::AdvertiseRequired { bind: args.bind }),
        None => Ok(origin::Host::of(args.bind)),
    }
}

pub async fn run(args: WebArgs) -> Result<(), WebError> {
    if !args.bind.is_loopback() && !args.allow_remote {
        return Err(WebError::RemoteBindRefused { bind: args.bind });
    }
    let host = advertised(&args)?;

    let session_path = crate::session::SessionFile::default_path();

    let address = SocketAddr::new(args.bind, args.port.unwrap_or(0));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| WebError::Bind { address, source })?;
    let bound = listener
        .local_addr()
        .map_err(|source| WebError::Bind { address, source })?;

    let (secret, entry_secret) = guard::Secret::mint();
    let state = Arc::new(AppState {
        completed: std::sync::atomic::AtomicBool::new(false),
        read_only: args.read_only,
        seen: route::Seen::new(),
        pages: page::Grants::new(),
        secret,
        cookie: guard::Cookie::mint(),
        origin: host.origin(bound.port()),
        identity: identity::Announced::new(),
        session: holder::Holder::new(session_path.clone()),
        playback: playback::Playback::new(session_path),
        live: live::Hub::new(),
        foreign: foreign::Anonymous::new(),
    });

    let server = server::Server::on(listener, state, entry_secret).await;

    if !args.bind.is_loopback() {
        if args.read_only {
            eprintln!(
                "warning: {} is reachable from the network and is read-only: it \
                 relays Jellyfin credentials but changes nothing on the server; the \
                 saved server list and every sign-in path, Quick Connect included, \
                 are reachable from the network",
                server.url()
            );
        } else {
            eprintln!(
                "warning: {} exposes server administration to the network, \
                 including first-run setup, first-administrator creation, user \
                 deletion and plugin installation; the saved server list and every \
                 sign-in path, password reset included, are reachable from the \
                 network; pass --read-only to offer the reading half alone",
                server.url()
            );
        }
    }

    let entry = server.entry_url();
    println!("{entry}");

    if !args.no_open
        && let Err(e) = webbrowser::open(&entry)
    {
        eprintln!("warning: could not open the default browser: {e}");
    }

    server.serve().await.map_err(WebError::Serve)
}
