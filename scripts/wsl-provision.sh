#!/usr/bin/env bash
# Provisions the Linux toolchain the local pre-push gate (`agent-run.sh pre-push`) runs in WSL2
# `Ubuntu`: rustup, the `rust-toolchain.toml` channel with llvm-tools, and the cargo tools at the
# exact versions ci.yml's `test` job pins. Runs INSIDE the distro, from a checkout of this repo, as
# the ordinary user. Pins are read from rust-toolchain.toml and ci.yml, never restated here, so a CI
# pin bump reads as `pin-mismatch` until re-provisioned. The C linker (`build-essential`) is a
# system package this script never installs; it runs as root through `wsl -u root` beforehand.
#   (no flag)  install what is missing or off-pin, then check
#   --check    check only: print the pinned versions or name the first mismatch
#   --probe    prove the checksum refusal and the pin-mismatch refusal fire, and the real pins pass
set -euo pipefail

rustup_version=1.29.1
rustup_sha=dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71
root=$(cd "$(dirname "$0")/.." && pwd)
export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "tool-missing: $1"
    if [ "$1" = cc ]; then
      echo "install: wsl.exe -d Ubuntu -u root --exec /usr/bin/apt-get install -y build-essential"
    fi
    exit 1
  fi
}

# verify <file> <sha256>
verify() {
  [ "$(sha256sum "$1" | cut -d' ' -f1)" = "$2" ]
}

channel() {
  sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' "$root/rust-toolchain.toml" | head -1
}

# The `name@version` pins on ci.yml's test-job tool line (the one that also installs llvm-cov).
pins() {
  grep -m1 -E '^[[:space:]]*tool:[[:space:]]*cargo-nextest@.*cargo-llvm-cov@' "$1" \
    | sed 's/^[[:space:]]*tool:[[:space:]]*//' | tr ',' '\n' | tr -d ' \r'
}

installed() {
  local out
  case "$1" in
    rustc) out=$(cd "$root" && rustc --version 2>/dev/null) || return 0 ;;
    *) out=$(cd "$root" && cargo "${1#cargo-}" --version 2>/dev/null) || return 0 ;;
  esac
  printf '%s\n' "$out" | awk 'NR==1 {print $2}'
}

# check <ci.yml>: prints the pins-ok line, or `pin-mismatch: <tool>` for the first mismatch.
check() {
  local want got line pin name
  want=$(channel)
  got=$(installed rustc)
  if [ -z "$want" ] || [ "$got" != "$want" ]; then
    echo "pin-mismatch: rustc"
    return 1
  fi
  line="wsl-provision: pins ok rustc $got"
  local listed
  listed=$(pins "$1")
  if [ -z "$listed" ]; then
    echo "pin-mismatch: ci.yml"
    return 1
  fi
  while IFS= read -r pin; do
    name=${pin%@*}
    got=$(installed "$name")
    if [ "$got" != "${pin#*@}" ]; then
      echo "pin-mismatch: $name"
      return 1
    fi
    line="$line $name $got"
  done <<< "$listed"
  echo "$line"
}

install_rustup() {
  if command -v rustup >/dev/null 2>&1; then
    return 0
  fi
  local work="$HOME/.cache/viola-provision"
  mkdir -p "$work"
  curl -fsSL --retry 5 --retry-delay 2 --retry-all-errors -o "$work/rustup-init" \
    "https://static.rust-lang.org/rustup/archive/$rustup_version/x86_64-unknown-linux-gnu/rustup-init"
  if ! verify "$work/rustup-init" "$rustup_sha"; then
    echo "wsl-provision: checksum mismatch for rustup-init $rustup_version"
    exit 1
  fi
  chmod 0700 "$work/rustup-init"
  "$work/rustup-init" -y --no-modify-path --profile minimal --default-toolchain none
}

install_all() {
  need cc
  need curl
  need sha256sum
  install_rustup
  (cd "$root" && rustup toolchain install && rustup component add llvm-tools-preview)
  local pin name
  while IFS= read -r pin; do
    name=${pin%@*}
    if [ "$(installed "$name")" != "${pin#*@}" ]; then
      (cd "$root" && cargo install --locked "$pin")
    fi
  done <<< "$(pins "$root/.github/workflows/ci.yml")"
  check "$root/.github/workflows/ci.yml"
}

probe() {
  local tmp refused=0
  tmp=$(mktemp -d)
  printf 'tampered\n' > "$tmp/rustup-init"
  if ! verify "$tmp/rustup-init" "$rustup_sha"; then
    refused=$((refused + 1))
  fi
  sed 's/cargo-mutants@[0-9.]*/cargo-mutants@0.0.0/' "$root/.github/workflows/ci.yml" > "$tmp/ci.yml"
  if [ "$(check "$tmp/ci.yml" || true)" = "pin-mismatch: cargo-mutants" ]; then
    refused=$((refused + 1))
  fi
  rm -rf "$tmp"
  if [ "$refused" -ne 2 ]; then
    echo "wsl-provision --probe: $refused/2 refused"
    exit 1
  fi
  if ! check "$root/.github/workflows/ci.yml" >/dev/null; then
    echo "wsl-provision --probe: 2/2 refused, control dirty"
    exit 1
  fi
  echo "wsl-provision --probe: 2/2 refused, control clean"
}

case "${1:-}" in
  "") install_all ;;
  --check) check "$root/.github/workflows/ci.yml" ;;
  --probe) probe ;;
  *)
    echo "usage: $0 [--check|--probe]"
    exit 2
    ;;
esac
