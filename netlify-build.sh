#!/usr/bin/env bash
set -euo pipefail

# install rustup non-interactively
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# load cargo env for the current shell
source "$HOME/.cargo/env"
# add wasm target
rustup target add wasm32-unknown-unknown
# install trunk if missing
if ! command -v trunk >/dev/null 2>&1; then
  cargo install trunk
fi
# build
trunk build --release