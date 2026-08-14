use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

mod assets;
mod control;
mod guard;
mod holder;
mod identity;
mod origin;
mod relay;
mod route;
mod server;
mod upstream;
mod version;

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
    pub secret: guard::Secret,
    pub cookie: guard::Cookie,
    /// The one origin this server answers for, the host taken from
    /// `--advertise` when given and from `--bind` otherwise.
    pub origin: origin::Origin,
    pub device: identity::Device,
    /// The held Jellyfin session and the session file it shares with the
    /// command line.
    pub session: holder::Holder,
}

#[cfg(test)]
impl AppState {
    // an app state holding nothing, its session file at `path`, minted purely
    // for router tests that drive the real relay route
    pub(crate) fn stub(path: std::path::PathBuf) -> AppState {
        AppState {
            secret: guard::Secret::mint().0,
            cookie: guard::Cookie::mint(),
            origin: origin::Host::of(std::net::Ipv4Addr::LOCALHOST.into()).origin(0),
            device: identity::Device::new(uuid::Uuid::nil()),
            session: holder::Holder::new(path),
        }
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
    let device_id =
        crate::session::SessionFile::update_async(session_path.clone(), |file| file.device_id())
            .await?;

    let address = SocketAddr::new(args.bind, args.port.unwrap_or(0));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| WebError::Bind { address, source })?;
    let bound = listener
        .local_addr()
        .map_err(|source| WebError::Bind { address, source })?;
    drop(listener);

    let (secret, entry_secret) = guard::Secret::mint();
    let state = Arc::new(AppState {
        secret,
        cookie: guard::Cookie::mint(),
        origin: host.origin(bound.port()),
        device: identity::Device::new(device_id),
        session: holder::Holder::new(session_path),
    });

    let server = server::Server::bind(bound, state, entry_secret)
        .await
        .map_err(|source| WebError::Bind {
            address: bound,
            source,
        })?;

    if !args.bind.is_loopback() {
        eprintln!(
            "warning: {} is reachable from the network, and it relays Jellyfin credentials",
            server.url()
        );
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
