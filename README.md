# jellyfin-cli
A command-line client for [Jellyfin](https://jellyfin.org/) media servers.

## Usage
```
$ jellyfin-cli --help
Usage: jellyfin-cli [OPTIONS] <COMMAND>

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
jellyfin-cli login
```

### Server info and ping:
```sh
jellyfin-cli system info
jellyfin-cli system ping
```

### Current user:
```sh
jellyfin-cli users me
```

### Refresh libraries:
Start a refresh of the library named "Music Videos" and exit when complete
```sh
jellyfin-cli libraries refresh --wait --name "Music Videos"
```

### Search:
```sh
jellyfin-cli search hints "big buck bunny"
```

### Make a raw authenticated API call (output body to stdout, headers to stderr):
```sh
jellyfin-cli api /System/Info
jellyfin-cli api /Users/Me
```

## Development

This project uses [just](https://github.com/casey/just) as a command runner.
```
Available recipes:
    build     # Build the debug release
    list      # List available recipes
    run *args # Run the debug release. Sets JELLYFIN_ENV_FILE to .env if it exists in the repo root
```
