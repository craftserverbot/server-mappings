#!/bin/bash
set -e
curl -fsSL https://servermappings.lunarclientcdn.com/servers.json > servers.json
cargo build
cargo test
git add .
git commit -m "chore: update submodules"
