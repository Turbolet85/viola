#!/usr/bin/env bash
# Provisions the Linux toolchain the local pre-push gate (`agent-run.sh pre-push`) runs in WSL2
# `Ubuntu`: rustup, the `rust-toolchain.toml` channel with llvm-tools, the cargo tools at the
# exact versions ci.yml's `test` job pins, ci.yml's pinned Node (scripts/install-node.sh) and the
# locked Playwright's Chromium. Runs INSIDE the distro, from a checkout of this repo, as the ordinary
# user. Pins are read from rust-toolchain.toml and ci.yml, never restated here, so a CI pin bump
# reads as `pin-mismatch` until re-provisioned. The C linker (`build-essential`) and Chromium's
# system libraries are system packages the user run never installs: they run as root through
# `wsl -u root` (the latter as `--install-deps`, after the user run), never through sudo.
#   (no flag)                  install what is missing or off-pin, then check
#   --check                    check only: print the pinned versions or name the first mismatch
#   --probe                    prove the checksum refusals and the pin-mismatch refusal fire, and the
#                              real pins pass
#   --install-deps <user home> as uid 0 only: install the system libraries that user's provisioned
#                              Playwright names for Chromium
set -euo pipefail

rustup_version=1.29.1
rustup_sha=dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71
root=$(cd "$(dirname "$0")/.." && pwd)
node_dir=.local/viola-node
web_dir=.cache/viola-provision/e2e-web
export PATH="$HOME/.cargo/bin:$HOME/$node_dir/bin:/usr/local/bin:/usr/bin:/bin"

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

# The workflow-level `NODE_PIN_VERSION` of a ci.yml (install-node.sh parses the same line).
node_pin() {
  sed -n 's/^  NODE_PIN_VERSION: *"\([^"]*\)".*/\1/p' "$1" | head -1 | tr -d '\r'
}

installed() {
  local out
  case "$1" in
    node) out=$(node --version 2>/dev/null) || return 0; printf '%s\n' "${out#v}"; return 0 ;;
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
  pin=$(node_pin "$1")
  got=$(installed node)
  if [ -z "$pin" ] || [ "$got" != "$pin" ]; then
    echo "pin-mismatch: node"
    return 1
  fi
  echo "$line node v$got"
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
  bash "$root/scripts/install-node.sh" linux-x64 "$HOME/$node_dir"
  node --version
  # The lockfile installs in the distro's own scratch, never in a checkout: from /mnt/<drive> it
  # would write Linux node_modules into the Windows tree (the pre-push clone runs its own npm ci).
  mkdir -p "$HOME/$web_dir"
  cp "$root/e2e-web/package.json" "$root/e2e-web/package-lock.json" "$HOME/$web_dir/"
  (cd "$HOME/$web_dir" && npm ci && npx --no playwright install chromium)
  check "$root/.github/workflows/ci.yml"
}

# install_deps <user home>: as uid 0, the apt packages that user's locked Playwright names for
# Chromium, printed (--dry-run) and then installed. `wsl -u root` is uid 0 with no password.
install_deps() {
  local home=$1 node cli
  if [ "$(id -u)" -ne 0 ]; then
    echo "wsl-provision: install-deps needs uid 0 (wsl.exe -d Ubuntu -u root)"
    exit 2
  fi
  node="$home/$node_dir/bin/node"
  cli="$home/$web_dir/node_modules/@playwright/test/cli.js"
  if [ ! -x "$node" ] || [ ! -f "$cli" ]; then
    echo "wsl-provision: install-deps needs the user provision first"
    exit 1
  fi
  # The dry run lists the missing packages and exits non-zero while any is missing (measured:
  # Playwright 1.63.0); the install below is the verdict.
  PATH="$home/$node_dir/bin:/usr/sbin:/usr/bin:/sbin:/bin" "$node" "$cli" install-deps --dry-run chromium || true
  PATH="$home/$node_dir/bin:/usr/sbin:/usr/bin:/sbin:/bin" "$node" "$cli" install-deps chromium
  echo "wsl-provision: install-deps ok"
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
  case "$(bash "$root/scripts/install-node.sh" --probe || true)" in
    "install-node --probe: 1/1 refused, pins ok v"*) refused=$((refused + 1)) ;;
  esac
  if [ "$refused" -ne 3 ]; then
    echo "wsl-provision --probe: $refused/3 refused"
    exit 1
  fi
  if ! check "$root/.github/workflows/ci.yml" >/dev/null; then
    echo "wsl-provision --probe: 3/3 refused, control dirty"
    exit 1
  fi
  echo "wsl-provision --probe: 3/3 refused, control clean"
}

case "${1:-}" in
  "") install_all ;;
  --check) check "$root/.github/workflows/ci.yml" ;;
  --probe) probe ;;
  --install-deps)
    if [ $# -ne 2 ] || [ "${2#/}" = "$2" ]; then
      echo "usage: $0 [--check|--probe|--install-deps <user home>]"
      exit 2
    fi
    install_deps "$2"
    ;;
  *)
    echo "usage: $0 [--check|--probe|--install-deps <user home>]"
    exit 2
    ;;
esac
