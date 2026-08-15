set positional-arguments

# List available recipes
list:
    @just --list

# Build the debug release
build:
    cargo build
    cargo clippy --all-targets -- -D warnings
    cd jellium-web && cargo clippy --all-targets -- -D warnings

# Check formatting in both workspaces
fmt:
    cargo fmt --all --check
    cd jellium-web && cargo fmt --all --check

# Run both workspaces' tests
test: fmt
    cargo test --workspace
    cd jellium-web && cargo test

# Build the Jellium Web bundle
web-bundle:
    cd jellium-web && trunk build --release --dist dist index.html

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
