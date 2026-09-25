#!/usr/bin/env bash
# Installs the official ripgrep release binary (built with PCRE2) that gates G1 and G3 need
# (obs-plan §9): runner images do not guarantee `rg`. One pinned version, each asset checked against
# the sha256 its release publishes, so CI and a local gate run the same binary. No Action is used.
# The download retries every failure, not only curl's "transient" ones: a Windows runner's Schannel
# curl fails the TLS handshake (exit 35) when the certificate revocation server is offline
# (CRYPT_E_REVOCATION_OFFLINE), which plain `--retry` never retries (measured on curl 8.18); there it
# also treats an unreachable revocation server as best-effort. The sha256 check stays the integrity gate.
# `--probe` proves the retry, the checksum refusal and the checksum pass with the host's own curl.
set -euo pipefail

version=15.2.0
root=$(cd "$(dirname "$0")/.." && pwd)
bin="$root/target/tools/ripgrep/bin"

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

curl_flags=(-fsSL --retry 5 --retry-delay 2 --retry-all-errors)
if curl --version | grep -qi schannel; then
  curl_flags+=(--ssl-revoke-best-effort)
fi

# download <url> <out> [extra curl args...]
download() {
  local url=$1 out=$2
  shift 2
  curl "${curl_flags[@]}" "$@" -o "$out" "$url"
}

# verify <file> <sha256> <asset name>
verify() {
  local actual
  actual=$(digest "$1")
  if [ "$actual" != "$2" ]; then
    echo "install-ripgrep: checksum mismatch for $3"
    return 1
  fi
}

install() {
  local asset sha exe work unpacked
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

  if [[ $asset == *.zip ]]; then need unzip; else need tar; fi

  work="$root/target/tools/ripgrep/download-$$"
  rm -rf "$work"
  mkdir -p "$work" "$bin"
  download "https://github.com/BurntSushi/ripgrep/releases/download/$version/$asset" "$work/$asset"
  verify "$work/$asset" "$sha" "$asset" || exit 1
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
}

probe() {
  local scratch native url rc tries fired=0
  scratch="$root/target/tools/ripgrep/probe-$(date -u +%Y%m%dT%H%M%SZ)-$$"
  mkdir -p "$scratch"

  # (a) a failure plain `--retry` would not retry (connection refused) is retried, then fails closed.
  rc=0
  download "https://127.0.0.1:1/asset" "$scratch/none" --retry-delay 0 --trace-ascii "$scratch/trace.txt" \
    > "$scratch/retry.log" 2>&1 || rc=$?
  tries=$(grep -c 'Trying 127.0.0.1' "$scratch/trace.txt" 2>/dev/null || true)
  if [ "$rc" -ne 0 ] && [ "${tries:-0}" -ge 2 ]; then
    fired=$((fired + 1))
    echo "fired    retry (exit $rc after $tries attempts)"
  else
    echo "FAILED   retry (exit $rc, $tries attempts; want non-zero after >= 2) — $scratch/retry.log"
  fi

  # (b) + (c) the same local asset refused on a wrong digest and accepted on its own.
  printf 'probe asset\n' > "$scratch/asset"
  native=$(cd "$scratch" && { pwd -W 2>/dev/null || pwd; })
  url="file://$([[ $native == /* ]] || printf /)$native/asset"
  download "$url" "$scratch/fetched"
  rc=0
  verify "$scratch/fetched" 0000000000000000000000000000000000000000000000000000000000000000 probe-asset \
    > "$scratch/mismatch.log" 2>&1 || rc=$?
  if [ "$rc" -ne 0 ] && grep -qF 'install-ripgrep: checksum mismatch for probe-asset' "$scratch/mismatch.log"; then
    fired=$((fired + 1))
    echo "fired    checksum mismatch refused"
  else
    echo "FAILED   checksum mismatch (exit $rc) — $scratch/mismatch.log"
  fi
  rc=0
  verify "$scratch/fetched" "$(digest "$scratch/asset")" probe-asset > "$scratch/match.log" 2>&1 || rc=$?
  if [ "$rc" -eq 0 ]; then
    fired=$((fired + 1))
    echo "fired    checksum match accepted"
  else
    echo "FAILED   checksum match (exit $rc) — $scratch/match.log"
  fi

  if [ "$fired" -eq 3 ]; then
    echo "install-ripgrep probes: 3/3 fired"
    return 0
  fi
  echo "install-ripgrep probes: FAILED — $fired/3 fired"
  return 1
}

case "${1:-}" in
  "") install ;;
  --probe) probe ;;
  *)
    echo "usage: install-ripgrep.sh [--probe]"
    exit 2
    ;;
esac
