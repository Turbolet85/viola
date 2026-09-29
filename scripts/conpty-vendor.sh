#!/usr/bin/env bash
# Vendors the sideloaded ConPTY: `conpty.dll` and `OpenConsole.exe` (x64) from Microsoft's
# `Microsoft.Windows.Console.ConPTY` NuGet package, at the version and SHA-256s pinned in
# src/conpty.rs. The pins are parsed from that file's text, never restated here. The .nupkg is
# checked before anything is extracted, each file after, and each file's Authenticode signer on
# Windows (Get-AuthenticodeSignature: Valid, Microsoft Corporation, a Microsoft Code Signing PCA).
#   conpty-vendor.sh            fetch, check and write vendor/conpty/<version>/x64/
#   conpty-vendor.sh --verify   fetch outside the tree and compare with the committed files byte for
#                               byte, then check the committed files' pins and signer
#   conpty-vendor.sh --probe    prove every refusal on minted inputs, and a clean control
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
pins_file="$root/src/conpty.rs"
package=microsoft.windows.console.conpty
retries=5

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

# file_pin <name>: the `sha256_hex` of the `Companion` whose `name` is <name>.
file_pin() {
  awk -v want="$1" '
    /name: "/ { split($0, a, "\""); cur = a[2] }
    /sha256_hex: "/ { split($0, a, "\""); if (cur == want) print a[2] }
  ' "$pins_file" | tr -d '\r'
}

# Sets `ver`, `nupkg_sha`, `dll_sha`, `exe_sha`, or refuses.
read_pins() {
  ver=$(sed -n '/^macro_rules! package_version/,/^}/s/^ *"\([0-9.]*\)"$/\1/p' "$pins_file" | tr -d '\r')
  nupkg_sha=$(sed -n 's/^const NUPKG_SHA256: &str = "\([0-9a-f]*\)";$/\1/p' "$pins_file" | tr -d '\r')
  dll_sha=$(file_pin conpty.dll)
  exe_sha=$(file_pin OpenConsole.exe)
  local sha='^[0-9a-f]{64}$'
  if ! [[ $ver =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && $nupkg_sha =~ $sha && $dll_sha =~ $sha && $exe_sha =~ $sha ]]; then
    echo "conpty-vendor: pin-unreadable"
    exit 1
  fi
}

url() {
  echo "https://api.nuget.org/v3-flatcontainer/$package/$ver/$package.$ver.nupkg"
}

# fetch <url> <file>: TLS only, failing closed.
fetch() {
  local flags=(-fsSL --proto '=https' --tlsv1.2 --retry "$retries" --retry-delay 2 --retry-all-errors)
  if curl --version | grep -qi schannel; then
    flags+=(--ssl-revoke-best-effort)
  fi
  if ! curl "${flags[@]}" -o "$2" "$1"; then
    echo "conpty-vendor: fetch failed"
    return 1
  fi
}

# unpack <nupkg> <dir>: the two x64 files into <dir>, only after the .nupkg matches its pin.
unpack() {
  if [ "$(digest "$1")" != "$nupkg_sha" ]; then
    echo "conpty-vendor: nupkg checksum mismatch"
    return 1
  fi
  mkdir -p "$2"
  unzip -p "$1" runtimes/win-x64/native/conpty.dll > "$2/conpty.dll"
  unzip -p "$1" build/native/runtimes/x64/OpenConsole.exe > "$2/OpenConsole.exe"
}

# check_hash <file> <sha256>
check_hash() {
  if [ "$(digest "$1")" != "$2" ]; then
    echo "conpty-vendor: checksum mismatch for $(basename "$1")"
    return 1
  fi
}

# check_signer <file>: Valid Authenticode, signed by Microsoft Corporation through a Microsoft Code
# Signing PCA. Windows only; anywhere else it refuses.
check_signer() {
  local got status subject issuer
  if ! command -v powershell.exe >/dev/null 2>&1; then
    echo "conpty-vendor: signer check needs Windows"
    return 1
  fi
  got=$(CONPTY_VENDOR_FILE="$(cygpath -w "$1")" powershell.exe -NoProfile -NonInteractive -Command \
    '$s = Get-AuthenticodeSignature -LiteralPath $env:CONPTY_VENDOR_FILE; "{0}|{1}|{2}" -f $s.Status, $s.SignerCertificate.Subject, $s.SignerCertificate.Issuer' |
    tr -d '\r')
  IFS='|' read -r status subject issuer <<< "$got"
  if [ "$status" != Valid ] ||
    [[ $subject != "CN=Microsoft Corporation, "* ]] ||
    [[ $issuer != "CN=Microsoft Code Signing PCA"* ]]; then
    echo "conpty-vendor: signer refused for $(basename "$1")"
    return 1
  fi
}

# check_files <dir>: both files' pins, then their signer.
check_files() {
  check_hash "$1/conpty.dll" "$dll_sha" || return 1
  check_hash "$1/OpenConsole.exe" "$exe_sha" || return 1
  check_signer "$1/conpty.dll" || return 1
  check_signer "$1/OpenConsole.exe" || return 1
}

committed_dir() {
  echo "$root/vendor/conpty/$ver/x64"
}

vendor() {
  need curl
  need unzip
  work=$(mktemp -d)
  trap 'rm -rf "$work"' EXIT
  fetch "$(url)" "$work/package.nupkg" || exit 1
  unpack "$work/package.nupkg" "$work/x64" || exit 1
  check_files "$work/x64" || exit 1
  local dest
  dest=$(committed_dir)
  mkdir -p "$dest"
  cp "$work/x64/conpty.dll" "$work/x64/OpenConsole.exe" "$dest/"
  echo "conpty-vendor: vendored $ver"
}

verify() {
  need curl
  need unzip
  work=$(mktemp -d)
  trap 'rm -rf "$work"' EXIT
  local committed name
  committed=$(committed_dir)
  fetch "$(url)" "$work/package.nupkg" || exit 1
  unpack "$work/package.nupkg" "$work/x64" || exit 1
  for name in conpty.dll OpenConsole.exe; do
    if ! cmp -s "$work/x64/$name" "$committed/$name"; then
      echo "conpty-vendor: committed $name differs from the package"
      exit 1
    fi
  done
  check_files "$committed" || exit 1
  echo "conpty-vendor: verified $ver"
}

# expect_refusal <want> <got>: counts one refusal when the refusing step printed exactly <want>.
expect_refusal() {
  total=$((total + 1))
  if [ "$2" = "$1" ]; then
    refused=$((refused + 1))
  else
    echo "conpty-vendor probe: expected '$1', got '$2'"
  fi
}

probe() {
  local committed out control=failed
  refused=0
  total=0
  retries=0
  committed=$(committed_dir)
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT

  printf 'not the package\n' > "$tmp/minted.nupkg"
  out=$(unpack "$tmp/minted.nupkg" "$tmp/minted-out" || true)
  [ -e "$tmp/minted-out" ] && out="$out (extracted)"
  expect_refusal "conpty-vendor: nupkg checksum mismatch" "$out"

  mkdir "$tmp/tampered"
  cp "$committed/conpty.dll" "$tmp/tampered/conpty.dll"
  printf 'x' >> "$tmp/tampered/conpty.dll"
  out=$(check_hash "$tmp/tampered/conpty.dll" "$dll_sha" || true)
  expect_refusal "conpty-vendor: checksum mismatch for conpty.dll" "$out"

  mkdir "$tmp/unsigned"
  printf 'MZ minted, unsigned\n' > "$tmp/unsigned/OpenConsole.exe"
  out=$(check_signer "$tmp/unsigned/OpenConsole.exe" || true)
  expect_refusal "conpty-vendor: signer refused for OpenConsole.exe" "$out"

  out=$(fetch "https://127.0.0.1:9/$package.$ver.nupkg" "$tmp/unreachable.nupkg" 2>/dev/null || true)
  expect_refusal "conpty-vendor: fetch failed" "$out"

  if out=$(check_files "$committed") && [ -z "$out" ]; then
    control=clean
  fi
  echo "conpty-vendor probe: $refused/$total refused, control $control"
  if [ "$refused" -ne "$total" ] || [ "$control" != clean ]; then
    exit 1
  fi
}

read_pins
case "${1:-}" in
  "") vendor ;;
  --verify) verify ;;
  --probe) probe ;;
  *)
    echo "usage: conpty-vendor.sh [--verify | --probe]"
    exit 2
    ;;
esac
