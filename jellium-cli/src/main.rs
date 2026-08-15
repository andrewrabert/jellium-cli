use std::io::{IsTerminal, Read as _, Write as _};

use base64::Engine as _;
use clap::{Parser, Subcommand};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use uuid::Uuid;

mod commands;
pub mod output;
mod session;
mod web;

#[derive(Parser)]
#[command(name = "jellium-cli", version)]
struct Cli {
    /// Jellyfin server URL (env: JELLYFIN_URL)
    #[arg(long = "server", global = true)]
    server: Option<String>,

    /// Username (env: JELLYFIN_USERNAME)
    #[arg(long, global = true)]
    username: Option<String>,

    /// Password (env: JELLYFIN_PASSWORD)
    #[arg(long, global = true)]
    password: Option<String>,

    /// API token (env: JELLYFIN_TOKEN)
    #[arg(long, global = true)]
    token: Option<String>,

    /// User ID (env: JELLYFIN_USER_ID)
    #[arg(long = "user-id", global = true)]
    user_id: Option<Uuid>,

    /// Load environment variables from this file (env: JELLYFIN_ENV_FILE)
    #[arg(long = "env-file", global = true)]
    env_file: Option<String>,

    /// Print request and response details to stderr
    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Command {
    /// Make a raw authenticated API request
    Api {
        /// API path (e.g. /System/Info)
        path: String,
        /// HTTP method
        #[arg(short = 'X', long, default_value = "GET")]
        method: String,
        /// Request body file (use - for stdin)
        #[arg(long)]
        input: Option<String>,
        /// Extra header (key:value, repeatable)
        #[arg(short = 'H', long, action = clap::ArgAction::Append)]
        header: Vec<String>,
        /// Output JSON object with status, headers, and body
        #[arg(long)]
        json: bool,
    },
    /// Artist operations
    Artists {
        #[command(subcommand)]
        command: commands::artists::ArtistsCommand,
    },
    /// API key operations
    AuthKeys {
        #[command(subcommand)]
        command: commands::auth_keys::AuthKeysCommand,
    },
    /// Backup operations
    Backup {
        #[command(subcommand)]
        command: commands::backup::BackupCommand,
    },
    /// Branding operations
    Branding {
        #[command(subcommand)]
        command: commands::branding::BrandingCommand,
    },
    /// Channel operations
    Channels {
        #[command(subcommand)]
        command: commands::channels::ChannelsCommand,
    },
    /// Collection operations
    Collections {
        #[command(subcommand)]
        command: commands::collections::CollectionsCommand,
    },
    /// Server configuration
    Config {
        #[command(subcommand)]
        command: commands::config::ConfigCommand,
    },
    /// Device operations
    Devices {
        #[command(subcommand)]
        command: commands::devices::DevicesCommand,
    },
    /// Display preferences
    DisplayPrefs {
        #[command(subcommand)]
        command: commands::display_prefs::DisplayPrefsCommand,
    },
    /// Environment/filesystem operations
    Environment {
        #[command(subcommand)]
        command: commands::environment::EnvironmentCommand,
    },
    /// Genre operations
    Genres {
        #[command(subcommand)]
        command: commands::genres::GenresCommand,
    },
    /// Item operations
    Items {
        #[command(subcommand)]
        command: commands::items::ItemsCommand,
    },
    /// Library and virtual folder operations
    Libraries {
        #[command(subcommand)]
        command: commands::libraries::LibrariesCommand,
    },
    /// Live TV operations
    LiveTv {
        #[command(subcommand)]
        command: commands::live_tv::LiveTvCommand,
    },
    /// Localization operations
    Localization {
        #[command(subcommand)]
        command: commands::localization::LocalizationCommand,
    },
    /// Authenticate and save session
    Login {
        /// Write session to this path instead of the default location
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
    },
    /// Remove saved session
    Logout,
    /// Movie operations
    Movies {
        #[command(subcommand)]
        command: commands::movies::MoviesCommand,
    },
    /// Music genre operations
    MusicGenres {
        #[command(subcommand)]
        command: commands::music_genres::MusicGenresCommand,
    },
    /// Package/plugin repository operations
    Packages {
        #[command(subcommand)]
        command: commands::packages::PackagesCommand,
    },
    /// Person operations
    Persons {
        #[command(subcommand)]
        command: commands::persons::PersonsCommand,
    },
    /// Playback reporting
    Playback {
        #[command(subcommand)]
        command: commands::playback::PlaybackCommand,
    },
    /// Playlist operations
    Playlists {
        #[command(subcommand)]
        command: commands::playlists::PlaylistsCommand,
    },
    /// Plugin operations
    Plugins {
        #[command(subcommand)]
        command: commands::plugins::PluginsCommand,
    },
    /// Quick connect operations
    QuickConnect {
        #[command(subcommand)]
        command: commands::quick_connect::QuickConnectCommand,
    },
    /// Search operations
    Search {
        #[command(subcommand)]
        command: commands::search::SearchCommand,
    },
    /// Session operations
    Sessions {
        #[command(subcommand)]
        command: commands::sessions::SessionsCommand,
    },
    /// TV show operations
    Shows {
        #[command(subcommand)]
        command: commands::shows::ShowsCommand,
    },
    /// Server startup wizard
    Startup {
        #[command(subcommand)]
        command: commands::startup::StartupCommand,
    },
    /// Studio operations
    Studios {
        #[command(subcommand)]
        command: commands::studios::StudiosCommand,
    },
    /// SyncPlay operations
    SyncPlay {
        #[command(subcommand)]
        command: commands::sync_play::SyncPlayCommand,
    },
    /// Server system commands
    System {
        #[command(subcommand)]
        command: commands::system::SystemCommand,
    },
    /// Scheduled task operations
    Tasks {
        #[command(subcommand)]
        command: commands::tasks::TasksCommand,
    },
    /// User item data (played, favorite, rating)
    UserData {
        #[command(subcommand)]
        command: commands::user_data::UserDataCommand,
    },
    /// User operations
    Users {
        #[command(subcommand)]
        command: commands::users::UsersCommand,
    },
    /// Serve Jellium Web in a browser
    Web(web::WebArgs),
    /// Video operations (non-streaming)
    Videos {
        #[command(subcommand)]
        command: commands::videos::VideosCommand,
    },
}

fn resolve_credentials(cli: &Cli) -> Option<(String, String, String)> {
    let server = cli
        .server
        .clone()
        .or_else(|| std::env::var("JELLYFIN_URL").ok());
    let username = cli
        .username
        .clone()
        .or_else(|| std::env::var("JELLYFIN_USERNAME").ok());
    let password = cli
        .password
        .clone()
        .or_else(|| std::env::var("JELLYFIN_PASSWORD").ok());

    match (server, username, password) {
        (Some(s), Some(u), Some(p)) => Some((s, u, p)),
        _ => None,
    }
}

fn prompt(message: &str) -> std::io::Result<String> {
    use std::io::{BufRead, Write};
    let mut stdout = std::io::stdout();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim_end_matches(['\r', '\n']).to_string())
}

fn auth_header(token: &str) -> HeaderValue {
    let value = format!(
        r#"MediaBrowser Client="jellium-cli", Device="cli", DeviceId="jellium-cli", Version="0.1.0", Token="{token}""#,
    );
    HeaderValue::from_str(&value).expect("invalid header value")
}

fn build_client(server: &str, token: &str, verbose: bool) -> jellyfin_api::Client {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header(token));

    let http = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("failed to build HTTP client");

    jellyfin_api::Client::new(server, http).with_verbose(verbose)
}

async fn authenticate(
    server: &str,
    username: &str,
    password: &str,
) -> Result<(String, Uuid), Box<dyn std::error::Error>> {
    let client = build_client(server, "", false);

    let body = jellyfin_api::types::AuthenticateUserByName {
        pw: Some(password.to_string()),
        username: Some(username.to_string()),
    };

    let result = client.authenticate_user_by_name(&body).await?;

    let token = result
        .access_token
        .ok_or("no access token in auth response")?;
    let user_id = result
        .user
        .and_then(|u| u.id)
        .ok_or("no user id in auth response")?;

    Ok((token, user_id))
}

async fn resolve_client(
    cli: &Cli,
) -> Result<(jellyfin_api::Client, Uuid), Box<dyn std::error::Error>> {
    let server = cli
        .server
        .clone()
        .or_else(|| std::env::var("JELLYFIN_URL").ok());
    let token = cli
        .token
        .clone()
        .or_else(|| std::env::var("JELLYFIN_TOKEN").ok());

    let user_id = cli.user_id.or_else(|| {
        std::env::var("JELLYFIN_USER_ID")
            .ok()
            .and_then(|s| s.parse().ok())
    });

    if let Some(token) = token {
        let server = server.ok_or("--token requires --server (or JELLYFIN_URL)")?;
        let client = build_client(&server, &token, cli.verbose);
        let user_id = match user_id {
            Some(id) => id,
            None => client
                .get_current_user()
                .await?
                .id
                .ok_or("no user id in /Users/Me response")?,
        };
        return Ok((client, user_id));
    }

    if let Some((server, username, password)) = resolve_credentials(cli) {
        let (token, resolved_user_id) = authenticate(&server, &username, &password).await?;
        return Ok((
            build_client(&server, &token, cli.verbose),
            user_id.unwrap_or(resolved_user_id),
        ));
    }

    let sess = session::SessionFile::load(&session::SessionFile::default_path())?
        .active()
        .ok_or("no saved session; run `jellium-cli login` first")?;
    Ok((
        build_client(&sess.server, &sess.token, cli.verbose),
        user_id.unwrap_or(sess.user_id),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(env_file) = cli
        .env_file
        .clone()
        .or_else(|| std::env::var("JELLYFIN_ENV_FILE").ok())
    {
        dotenvy::from_path(&env_file)?;
    }

    if let Command::Web(args) = cli.command {
        return Ok(web::run(args).await?);
    }

    // Handle login/logout without needing an authenticated client
    if let Command::Login { output } = &cli.command {
        let server = cli
            .server
            .clone()
            .or_else(|| std::env::var("JELLYFIN_URL").ok())
            .map(Ok)
            .unwrap_or_else(|| prompt("Server: "))?;
        let username = cli
            .username
            .clone()
            .or_else(|| std::env::var("JELLYFIN_USERNAME").ok())
            .map(Ok)
            .unwrap_or_else(|| prompt("Username: "))?;
        let password = cli
            .password
            .clone()
            .or_else(|| std::env::var("JELLYFIN_PASSWORD").ok())
            .map(Ok)
            .unwrap_or_else(|| rpassword::prompt_password("Password: "))?;
        let (token, user_id) = authenticate(&server, &username, &password).await?;
        let path = match output {
            Some(p) => p.clone(),
            None => session::SessionFile::default_path(),
        };
        session::SessionFile::update(&path, |file| {
            file.set_server(&session::Session {
                server,
                token,
                user_id,
            })
        })?;
        println!(
            "Logged in successfully. Session saved to {}.",
            path.display()
        );
        return Ok(());
    }

    if matches!(&cli.command, Command::Logout) {
        let path = session::SessionFile::default_path();
        session::SessionFile::update(&path, |file| file.clear_credential(0))?;
        println!("Logged out. Session removed.");
        return Ok(());
    }

    // All other commands need an authenticated client
    let (client, user_id) = resolve_client(&cli).await?;

    match cli.command {
        Command::Api {
            path,
            method,
            input,
            header,
            json,
        } => {
            handle_api_command(&client, &path, &method, input.as_deref(), &header, json).await?;
        }
        Command::System { command } => commands::system::execute(&client, command).await?,
        Command::Config { command } => commands::config::execute(&client, command).await?,
        Command::Users { command } => commands::users::execute(&client, &user_id, &command).await?,
        Command::UserData { command } => {
            commands::user_data::execute(&client, &user_id, &command).await?
        }
        Command::Libraries { command } => {
            commands::libraries::execute(&client, &user_id, &command).await?
        }
        Command::Shows { command } => commands::shows::execute(&client, &user_id, &command).await?,
        Command::Movies { command } => {
            commands::movies::execute(&client, &user_id, &command).await?
        }
        Command::Artists { command } => {
            commands::artists::execute(&client, &user_id, &command).await?
        }
        Command::Items { command } => commands::items::execute(&client, &user_id, &command).await?,
        Command::Genres { command } => {
            commands::genres::execute(&client, &user_id, &command).await?
        }
        Command::MusicGenres { command } => {
            commands::music_genres::execute(&client, &user_id, &command).await?
        }
        Command::Studios { command } => {
            commands::studios::execute(&client, &user_id, &command).await?
        }
        Command::Persons { command } => {
            commands::persons::execute(&client, &user_id, &command).await?
        }
        Command::Playlists { command } => {
            commands::playlists::execute(&client, &user_id, &command).await?
        }
        Command::Collections { command } => {
            commands::collections::execute(&client, command).await?
        }
        Command::Sessions { command } => {
            commands::sessions::execute(&client, &user_id, &command).await?
        }
        Command::Playback { command } => {
            commands::playback::execute(&client, &user_id, &command).await?
        }
        Command::Devices { command } => commands::devices::execute(&client, command).await?,
        Command::Tasks { command } => commands::tasks::execute(&client, command).await?,
        Command::Packages { command } => commands::packages::execute(&client, command).await?,
        Command::Plugins { command } => commands::plugins::execute(&client, command).await?,
        Command::AuthKeys { command } => commands::auth_keys::execute(&client, command).await?,
        Command::QuickConnect { command } => {
            commands::quick_connect::execute(&client, command).await?
        }
        Command::LiveTv { command } => {
            commands::live_tv::execute(&client, &user_id, &command).await?
        }
        Command::Search { command } => {
            commands::search::execute(&client, &user_id, &command).await?
        }
        Command::Channels { command } => {
            commands::channels::execute(&client, &user_id, &command).await?
        }
        Command::Backup { command } => commands::backup::execute(&client, command).await?,
        Command::Localization { command } => {
            commands::localization::execute(&client, command).await?
        }
        Command::Branding { command } => commands::branding::execute(&client, command).await?,
        Command::Environment { command } => {
            commands::environment::execute(&client, command).await?
        }
        Command::Videos { command } => {
            commands::videos::execute(&client, &user_id, &command).await?
        }
        Command::SyncPlay { command } => {
            commands::sync_play::execute(&client, &user_id, &command).await?
        }
        Command::DisplayPrefs { command } => {
            commands::display_prefs::execute(&client, command).await?
        }
        Command::Startup { command } => commands::startup::execute(&client, command).await?,
        Command::Login { .. } | Command::Logout | Command::Web(_) => unreachable!(),
    }

    Ok(())
}

async fn handle_api_command(
    client: &jellyfin_api::Client,
    path: &str,
    method: &str,
    input: Option<&str>,
    headers: &[String],
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let method: reqwest::Method = method
        .parse()
        .map_err(|_| format!("invalid HTTP method: {method}"))?;

    let body = match input {
        Some("-") => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            Some(buf)
        }
        Some(file) => Some(std::fs::read(file)?),
        None => None,
    };

    let parsed_headers: Vec<(reqwest::header::HeaderName, String)> = headers
        .iter()
        .map(|h| {
            let (key, value) = h
                .split_once(':')
                .ok_or_else(|| format!("header must be key:value: {h}"))?;
            let name = key
                .parse::<reqwest::header::HeaderName>()
                .map_err(|e| format!("invalid header name '{key}': {e}"))?;
            Ok::<_, Box<dyn std::error::Error>>((name, value.trim_start().to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let resp = client
        .raw_request(method, path, &parsed_headers, body.as_deref())
        .await?;

    if json {
        print_json_response(&resp)?;
    } else {
        let color = std::io::stderr().is_terminal();
        let pretty = std::io::stdout().is_terminal();

        print_response_headers(&resp, color);

        if pretty {
            if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&resp.body) {
                let formatted = colored_json::to_colored_json_auto(&value)?;
                println!("{formatted}");
            } else {
                std::io::stdout().write_all(&resp.body)?;
            }
        } else {
            std::io::stdout().write_all(&resp.body)?;
        }
    }

    if !(200..300).contains(&resp.status) {
        std::process::exit(1);
    }
    Ok(())
}

fn print_response_headers(resp: &jellyfin_api::RawResponse, color: bool) {
    if color {
        eprintln!(
            "\x1b[34m{}\x1b[0m \x1b[36m{}\x1b[0m",
            resp.http_version, resp.status
        );
        for (key, value) in &resp.headers {
            eprintln!("\x1b[37m{key}:\x1b[0m \x1b[36m{value}\x1b[0m");
        }
    } else {
        eprintln!("{} {}", resp.http_version, resp.status);
        for (key, value) in &resp.headers {
            eprintln!("{key}: {value}");
        }
    }
    eprintln!();
}

fn encode_body(body: &[u8]) -> (String, &'static str) {
    if let Ok(text) = std::str::from_utf8(body) {
        return (text.to_string(), "text");
    }
    let encoded = base64::engine::general_purpose::STANDARD.encode(body);
    (encoded, "base64")
}

fn collect_headers(headers: &[(String, String)]) -> serde_json::Value {
    let mut grouped: indexmap::IndexMap<&str, Vec<&str>> = indexmap::IndexMap::new();
    for (k, v) in headers {
        grouped.entry(k).or_default().push(v);
    }
    let map: serde_json::Map<String, serde_json::Value> = grouped
        .into_iter()
        .map(|(k, values)| {
            let value = if values.len() == 1 {
                serde_json::Value::String(values[0].to_string())
            } else {
                serde_json::Value::Array(
                    values
                        .into_iter()
                        .map(|v| serde_json::Value::String(v.to_string()))
                        .collect(),
                )
            };
            (k.to_string(), value)
        })
        .collect();
    serde_json::Value::Object(map)
}

fn print_json_response(resp: &jellyfin_api::RawResponse) -> Result<(), Box<dyn std::error::Error>> {
    let (body_value, body_encoding) = encode_body(&resp.body);

    let mut envelope = serde_json::Map::new();
    envelope.insert("status".into(), resp.status.into());
    envelope.insert("headers".into(), collect_headers(&resp.headers));
    envelope.insert("body".into(), body_value.into());
    envelope.insert("body_encoding".into(), body_encoding.into());

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::Value::Object(envelope))?
    );
    Ok(())
}
