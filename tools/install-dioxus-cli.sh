#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

dioxus_version="$(
  cargo metadata --format-version 1 --manifest-path "$repo_root/Cargo.toml" |
    python3 -c '
import json
import sys

metadata = json.load(sys.stdin)
versions = sorted({
    package["version"]
    for package in metadata["packages"]
    if package["name"] == "dioxus"
})

if len(versions) != 1:
    raise SystemExit(f"Expected exactly one resolved dioxus version, found: {versions}")

print(versions[0])
'
)"

if [ -z "$dioxus_version" ]; then
  echo "Could not find resolved dioxus version from cargo metadata" >&2
  exit 1
fi

installed_version=""
if command -v dx >/dev/null 2>&1; then
  installed_version="$(
    dx --version 2>/dev/null | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+' | head -n 1 || true
  )"
fi

if [ "$installed_version" = "$dioxus_version" ]; then
  echo "dioxus-cli $dioxus_version already installed."
  exit 0
fi

if [ -n "$installed_version" ]; then
  echo "Installing dioxus-cli $dioxus_version; replacing dx $installed_version."
else
  echo "Installing dioxus-cli $dioxus_version."
fi

cargo install dioxus-cli --version "$dioxus_version" --locked --force
