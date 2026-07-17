set positional-arguments

# List available recipes
list:
    @just --list

# Build the debug release
build:
    cargo build
    cargo clippy --all-targets -- -D warnings

# Run the debug release. Sets JELLYFIN_ENV_FILE to .env if it exists in the repo root
run *args:
    #!/usr/bin/env sh
    set -eu
    if [ -f .env ]; then
        export JELLYFIN_ENV_FILE="$(pwd)/.env"
    fi
    cargo run -p jellium-cli -- "$@"
