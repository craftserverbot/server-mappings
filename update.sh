#!/bin/bash
set -e
git submodule update --remote
cargo build
cargo test
git add .
git commit -m "chore: update submodules"
