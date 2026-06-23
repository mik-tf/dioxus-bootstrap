#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
lockfile="$repo_root/Cargo.lock"

dioxus_version="$(
  awk '
    $0 == "[[package]]" {
      in_package = 1
      name = ""
      version = ""
      next
    }
    in_package && $1 == "name" && $3 == "\"dioxus\"" {
      name = "dioxus"
      next
    }
    in_package && $1 == "version" {
      gsub(/"/, "", $3)
      version = $3
    }
    in_package && name == "dioxus" && version != "" {
      print version
      exit
    }
  ' "$lockfile"
)"

if [ -z "$dioxus_version" ]; then
  echo "Could not find resolved dioxus version in $lockfile" >&2
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
