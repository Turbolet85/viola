#!/usr/bin/env bash
# Installs the official ripgrep release binary (built with PCRE2) that gates G1 and G3 need
# (obs-plan §9): runner images do not guarantee `rg`. One pinned version, each asset checked against
# the sha256 its release publishes, so CI and a local gate run the same binary. No Action is used.
set -euo pipefail

version=15.2.0
root=$(cd "$(dirname "$0")/.." && pwd)
bin="$root/target/tools/ripgrep/bin"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)
    asset="ripgrep-$version-x86_64-unknown-linux-musl.tar.gz"
    sha=33e15bcf1624b25cdd2a55813a47a2f95dbe126268203e76aa6a585d1e7b149c
    ;;
  Darwin-arm64)
    asset="ripgrep-$version-aarch64-apple-darwin.tar.gz"
    sha=3750b2e93f37e0c692657da574d7019a101c0084da05a790c83fd335bad973e4
    ;;
  MINGW*-x86_64 | MSYS*-x86_64 | CYGWIN*-x86_64)
    asset="ripgrep-$version-x86_64-pc-windows-msvc.zip"
    sha=71b2fef860abe467217a538ff31de02f5258807c0129f771846f87bd029aafc5
    ;;
  *)
    echo "install-ripgrep: unsupported host $(uname -s)-$(uname -m)"
    exit 1
    ;;
esac

exe=rg
[[ $asset == *.zip ]] && exe=rg.exe

if [ -x "$bin/$exe" ] && "$bin/$exe" --version 2>/dev/null | grep -qF "ripgrep $version"; then
  echo "install-ripgrep: ripgrep $version already at $bin"
  exit 0
fi

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "tool-missing: $1"
    exit 1
  fi
}
need curl
if command -v sha256sum >/dev/null 2>&1; then
  digest() { sha256sum "$1" | cut -d' ' -f1; }
else
  need shasum
  digest() { shasum -a 256 "$1" | cut -d' ' -f1; }
fi
if [[ $asset == *.zip ]]; then need unzip; else need tar; fi

work="$root/target/tools/ripgrep/download-$$"
rm -rf "$work"
mkdir -p "$work" "$bin"
curl -fsSL --retry 3 -o "$work/$asset" \
  "https://github.com/BurntSushi/ripgrep/releases/download/$version/$asset"
actual=$(digest "$work/$asset")
if [ "$actual" != "$sha" ]; then
  echo "install-ripgrep: checksum mismatch for $asset"
  exit 1
fi
if [[ $asset == *.zip ]]; then
  unzip -q "$work/$asset" -d "$work"
else
  tar -xzf "$work/$asset" -C "$work"
fi
unpacked=${asset%.tar.gz}
unpacked=${unpacked%.zip}
cp "$work/$unpacked/$exe" "$bin/$exe"
chmod +x "$bin/$exe"
rm -rf "$work"
echo "install-ripgrep: ripgrep $version installed at $bin"
