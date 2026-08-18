# jellium-cli
A command-line client for [Jellyfin](https://jellyfin.org/) media servers.

## Usage
```
$ jellium-cli --help
Usage: jellium-cli [OPTIONS] <COMMAND>

Commands:
  api            Make a raw authenticated API request
  artists        Artist operations
  auth-keys      API key operations
  backup         Backup operations
  branding       Branding operations
  channels       Channel operations
  collections    Collection operations
  config         Server configuration
  devices        Device operations
  display-prefs  Display preferences
  environment    Environment/filesystem operations
  genres         Genre operations
  items          Item operations
  libraries      Library and virtual folder operations
  live-tv        Live TV operations
  localization   Localization operations
  login          Authenticate and save session
  logout         Remove saved session
  movies         Movie operations
  music-genres   Music genre operations
  packages       Package/plugin repository operations
  persons        Person operations
  playback       Playback reporting
  playlists      Playlist operations
  plugins        Plugin operations
  quick-connect  Quick connect operations
  search         Search operations
  sessions       Session operations
  shows          TV show operations
  startup        Server startup wizard
  studios        Studio operations
  sync-play      SyncPlay operations
  system         Server system commands
  tasks          Scheduled task operations
  user-data      User item data (played, favorite, rating)
  users          User operations
  web            Serve Jellium Web in a browser
  videos         Video operations (non-streaming)
  help           Print this message or the help of the given subcommand(s)

Options:
      --server <SERVER>      Jellyfin server URL (env: JELLYFIN_URL)
      --username <USERNAME>  Username (env: JELLYFIN_USERNAME)
      --password <PASSWORD>  Password (env: JELLYFIN_PASSWORD)
      --token <TOKEN>        API token (env: JELLYFIN_TOKEN)
      --user-id <USER_ID>    User ID (env: JELLYFIN_USER_ID)
      --env-file <ENV_FILE>  Load environment variables from this file (env: JELLYFIN_ENV_FILE)
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples
All examples require first logging into a server (or setting the appropriate flags or args).

```sh
jellium-cli login
```

### Server info and ping:
```sh
jellium-cli system info
jellium-cli system ping
```

### Current user:
```sh
jellium-cli users me
```

### Refresh libraries:
Start a refresh of the library named "Music Videos" and exit when complete
```sh
jellium-cli libraries refresh --wait --name "Music Videos"
```

### Search:
```sh
jellium-cli search hints "big buck bunny"
```

### Jellium Web:
Serve the browser client on loopback and open it in the default browser.
```sh
jellium-cli web
jellium-cli web --port 8096 --no-open
jellium-cli web --bind 0.0.0.0 --allow-remote --advertise media.lan
```

### Make a raw authenticated API call (output body to stdout, headers to stderr):
```sh
jellium-cli api /System/Info
jellium-cli api /Users/Me
```

## Development

Jellium Web bundles a pinned copy of [hls.js](https://github.com/video-dev/hls.js)
under `jellium-web/vendor/`; building the binary needs trunk and no Node.

This project uses [just](https://github.com/casey/just) as a command runner.
```
Available recipes:
    assets checkout     # Rewrite jellium-web/fonts, jellium-web/icons and jellium-web/branding from a checkout of the pinned revision
    build               # Build the debug release
    constructs checkout # Rewrite reference/constructs.tsv and jellium-model/src/construct.rs from a checkout of the pinned revision
    fmt                 # Check formatting in both workspaces
    list                # List available recipes
    pinned checkout     # Fail when the tree has drifted from a checkout of the pinned revision
    reference checkout  # Rewrite jellium-web/reference from a checkout of the pinned revision
    run *args           # Run the debug release. Sets JELLYFIN_ENV_FILE to .env if it exists in the repo root
    spans checkout      # Rewrite reference/spans from a checkout of the pinned revision
    static-page         # Rewrite jellium-web/boot.css and jellium-web/index.html from the ported appearance values
    suppressions        # Fail on any lint suppression or strictness-lowering configuration
    test                # Run both workspaces' tests
    web *args           # Run jellium-cli web from the debug build
    web-bundle          # Build the Jellium Web bundle
```
