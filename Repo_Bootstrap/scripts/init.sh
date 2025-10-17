#!/bin/bash
cargo new --lib crates/dag-core
cargo new --lib crates/zkp-lib
cargo new --lib crates/mastodon-ext
# Add workspace in root Cargo.toml
echo '[workspace]\nmembers = ["crates/*"]' > Cargo.toml
git add . && git commit -m "Genesis Commit" && git push origin main
echo "Repo live! Build app: cd android && ./gradlew assembleDebug"
