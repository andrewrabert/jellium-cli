set positional-arguments

# List available recipes
list:
    @just --list

# Build the debug release
build:
    cargo build
    cargo clippy --all-targets -- -D warnings

# Build the Jellium Web bundle
web-bundle renderer="webgpu":
    cd jellium-web && trunk build --release --no-default-features --features {{renderer}} \
        --dist dist index.html

# Run jellium-cli web from the debug build
web *args:
    #!/usr/bin/env sh
    set -eu
    if [ -f .env ]; then
        export JELLYFIN_ENV_FILE="$(pwd)/.env"
    fi
    cargo run -p jellium-cli -- web "$@"

# Run the debug release. Sets JELLYFIN_ENV_FILE to .env if it exists in the repo root
run *args:
    #!/usr/bin/env sh
    set -eu
    if [ -f .env ]; then
        export JELLYFIN_ENV_FILE="$(pwd)/.env"
    fi
    cargo run -p jellium-cli -- "$@"
