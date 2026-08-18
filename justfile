set positional-arguments

# List available recipes
list:
    @just --list

# Fail on any lint suppression or strictness-lowering configuration
suppressions:
    cargo test -p jellium-reference --test suppressions

# Build the debug release
build: suppressions
    cargo build
    cargo clippy --all-targets -- -D warnings
    cd jellium-web && cargo clippy --all-targets -- -D warnings

# Check formatting in both workspaces
fmt:
    cargo fmt --all --check
    cd jellium-web && cargo fmt --all --check

# Run both workspaces' tests
test: fmt suppressions
    cargo test --workspace
    cd jellium-web && cargo test

# Rewrite jellium-web/reference from a checkout of the pinned revision
reference checkout:
    node tools/reference/slice.mjs "$1"

# Rewrite jellium-web/fonts, jellium-web/icons and jellium-web/branding from a checkout of the pinned revision
assets checkout:
    node tools/reference/assets.mjs "$1"

# Rewrite jellium-web/boot.css and jellium-web/index.html from the ported appearance values
static-page:
    cargo run -p jellium-model --example boot-css > jellium-web/boot.css
    cargo run -p jellium-model --example index-html > jellium-web/index.html

# Rewrite reference/spans from a checkout of the pinned revision
spans checkout:
    node tools/reference/spans.mjs "$1"

# Fail when the tree has drifted from a checkout of the pinned revision
pinned checkout: (reference checkout) (spans checkout)
    git ls-files --error-unmatch jellium-web/reference/jellyfin-web.mjs
    git ls-files --error-unmatch reference/spans
    git diff --exit-code jellium-web/reference reference/spans
    cargo test -p jellium-reference

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
