#!/usr/bin/env bash
# Installs the official Node.js release build the browser suite runs on, at the version and sha256
# pinned in ci.yml's workflow `env:` block. The pins are parsed from ci.yml's text, never read from the
# environment and never restated here, so CI (every OS) and the native Linux pre-push gate host
# (`~/.local/viola-node`) install one Node. No toolchain Action is used; the sha256 check is the
# integrity gate.
#   install-node.sh <linux-x64|darwin-arm64|win-x64> <dest-dir>
#       prints `install-node: <bin dir>` (<dest-dir>/bin on unix, <dest-dir> on win-x64)
#   install-node.sh --probe
#       proves the checksum refusal on a tampered file and the pin parse against the real ci.yml
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
ci="$root/.github/workflows/ci.yml"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "tool-missing: $1"
    exit 1
  fi
}

if command -v sha256sum >/dev/null 2>&1; then
  digest() { sha256sum "$1" | cut -d' ' -f1; }
else
  digest() { shasum -a 256 "$1" | cut -d' ' -f1; }
fi

# pin <NAME>: the quoted value of the workflow-level `  NAME: "value"` line in ci.yml.
pin() {
  sed -n "s/^  $1: *\"\([^\"]*\)\".*/\1/p" "$ci" | head -1 | tr -d '\r'
}

# pins <os-key>: sets `ver` and `want` (that key's sha256), or refuses.
pins() {
  ver=$(pin NODE_PIN_VERSION)
  want=$(pin "NODE_PIN_SHA256_$(printf '%s' "$1" | tr 'a-z-' 'A-Z_')")
  if ! [[ $ver =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && $want =~ ^[0-9a-f]{64}$ ]]; then
    echo "install-node: pin-unreadable"
    exit 1
  fi
}

# verify <file> <sha256>
verify() {
  if [ "$(digest "$1")" != "$2" ]; then
    echo "install-node: checksum mismatch for $(basename "$1")"
    return 1
  fi
}

install() {
  local key=$1 dest ext bin top file work
  dest=$(printf '%s' "$2" | tr '\\' '/')
  case "$key" in
    linux-x64) ext=tar.xz ;;
    darwin-arm64) ext=tar.gz ;;
    win-x64) ext=zip ;;
    *)
      echo "usage: install-node.sh <linux-x64|darwin-arm64|win-x64> <dest-dir> | --probe"
      exit 2
      ;;
  esac
  pins "$key"
  bin="$dest/bin"
  [ "$key" = win-x64 ] && bin=$dest

  if [ -x "$bin/node" ] || [ -x "$bin/node.exe" ]; then
    if [ "$("$bin/node" --version 2>/dev/null | tr -d '\r')" = "v$ver" ]; then
      echo "install-node: $bin"
      return 0
    fi
  fi

  need curl
  if [ "$ext" = zip ]; then need unzip; else need tar; fi
  local curl_flags=(-fsSL --retry 5 --retry-delay 2 --retry-all-errors)
  if curl --version | grep -qi schannel; then
    curl_flags+=(--ssl-revoke-best-effort)
  fi

  top="node-v$ver-$key"
  file="$top.$ext"
  work=$(mktemp -d)
  curl "${curl_flags[@]}" -o "$work/$file" "https://nodejs.org/dist/v$ver/$file"
  if ! verify "$work/$file" "$want"; then
    rm -rf "$work"
    exit 1
  fi
  case "$ext" in
    tar.xz) tar -xJf "$work/$file" -C "$work" ;;
    tar.gz) tar -xzf "$work/$file" -C "$work" ;;
    zip) unzip -q "$work/$file" -d "$work" ;;
  esac
  mkdir -p "$dest"
  cp -R "$work/$top/." "$dest/"
  rm -rf "$work"
  echo "install-node: $bin"
}

probe() {
  local tmp refused=0 key
  for key in darwin-arm64 win-x64 linux-x64; do
    pins "$key"
  done
  tmp=$(mktemp -d)
  printf 'tampered\n' > "$tmp/node-v$ver-linux-x64.tar.xz"
  if [ "$(verify "$tmp/node-v$ver-linux-x64.tar.xz" "$want" || true)" \
    = "install-node: checksum mismatch for node-v$ver-linux-x64.tar.xz" ]; then
    refused=$((refused + 1))
  fi
  rm -rf "$tmp"
  if [ "$refused" -ne 1 ]; then
    echo "install-node --probe: $refused/1 refused"
    exit 1
  fi
  echo "install-node --probe: 1/1 refused, pins ok v$ver"
}

case "${1:-}" in
  --probe) probe ;;
  linux-x64 | darwin-arm64 | win-x64)
    if [ $# -ne 2 ]; then
      echo "usage: install-node.sh <linux-x64|darwin-arm64|win-x64> <dest-dir> | --probe"
      exit 2
    fi
    install "$1" "$2"
    ;;
  *)
    echo "usage: install-node.sh <linux-x64|darwin-arm64|win-x64> <dest-dir> | --probe"
    exit 2
    ;;
esac
