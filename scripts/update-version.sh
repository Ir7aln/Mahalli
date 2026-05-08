#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./scripts/update-version.sh <version>  (e.g. 0.4.0)"
    exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NEW_VERSION=$1

# package.json
sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" "$ROOT_DIR/package.json"

# tauri.conf.json
sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" "$ROOT_DIR/src-tauri/tauri.conf.json"

# Cargo.toml (first occurrence only)
sed -i "1,/^version = / s/version = \".*\"/version = \"$NEW_VERSION\"/" "$ROOT_DIR/src-tauri/Cargo.toml"

echo "Updated version to $NEW_VERSION in:"
echo "  - package.json"
echo "  - src-tauri/tauri.conf.json"
echo "  - src-tauri/Cargo.toml"

# Commit, tag and push to trigger the release workflow
read -p "Commit, tag v$NEW_VERSION and push? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git -C "$ROOT_DIR" add .
    git -C "$ROOT_DIR" commit -m "$NEW_VERSION"
    git -C "$ROOT_DIR" tag "v$NEW_VERSION"
    git -C "$ROOT_DIR" push && git -C "$ROOT_DIR" push origin "v$NEW_VERSION"
    echo "Pushed tag v$NEW_VERSION — release workflow will start on GitHub Actions."
fi
