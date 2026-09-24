#!/usr/bin/env bash
# Proves the print/dbg and raw-log bans fire (obs-plan §3 obs-ci-gate-wire). A clean clippy run on
# viola cannot tell a working ban from a missing one, so each probe is a throwaway crate carrying the
# repo's own clippy.toml and [workspace.lints.clippy] table: a banned call must fail with that lint's
# name, and two controls (a locally allowed print, the real obs_event!) must pass.
#
# `tracing::event` cannot sit in clippy.toml: clippy reports a disallowed macro expanded inside
# obs_event! at the caller crate's level, so no allow inside the macro exempts it. A raw `event!` is
# instead caught by a fail-closed grep over the tree, itself probed both ways here.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
# The path cargo reads inside a manifest: MSYS bash's /d/... is not one on Windows.
native_root=$(cd "$root" && { pwd -W 2>/dev/null || pwd; })

if ! cargo clippy --version >/dev/null 2>&1; then
  echo "tool-missing: clippy"
  exit 1
fi

scratch="$root/target/lint-probes/run-$(date -u +%Y%m%dT%H%M%SZ)-$$"
mkdir -p "$scratch"
export CARGO_TARGET_DIR="$root/target/lint-probes/target"

# The repo's table, never a hand copy: the lines after the header up to the next one.
lints=$(awk '/^\[workspace\.lints\.clippy\]$/ {on=1; next} /^\[/ {on=0} on && NF' "$root/Cargo.toml")
if [ -z "$lints" ]; then
  echo "lint-probes: FAILED — no [workspace.lints.clippy] table in Cargo.toml"
  exit 1
fi

# The empty [workspace] table keeps each probe out of the viola workspace that encloses target/.
make_project() {
  local dir=$1 body=$2
  mkdir -p "$dir/src"
  cp "$root/clippy.toml" "$dir/clippy.toml"
  cp "$root/Cargo.lock" "$dir/Cargo.lock"
  printf '%s\n' "$body" > "$dir/src/lib.rs"
  cat > "$dir/Cargo.toml" <<EOF
[package]
name = "lint-probe"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
viola-core = { path = "$native_root/crates/viola-core" }
tracing = { version = "=0.1.44", default-features = false, features = ["std"] }

[lints]
workspace = true

[workspace]

[workspace.lints.clippy]
$lints
EOF
}

run_clippy() {
  local dir=$1
  if (cd "$dir" && cargo clippy --quiet -- -D warnings) > "$dir/clippy.log" 2>&1; then
    return 0
  else
    return $?
  fi
}

# id | lint name the failure must carry | probe body
bans=(
  "println|print_stdout|pub fn probe() { println!(\"x\"); }"
  "eprintln|print_stderr|pub fn probe() { eprintln!(\"x\"); }"
  "dbg|dbg_macro|pub fn probe() -> i32 { dbg!(1) }"
  "tracing-info|disallowed_macros|pub fn probe() { tracing::info!(\"x\"); }"
)
controls=(
  "allowed-print|#[allow(clippy::print_stdout, clippy::print_stderr)]
pub mod output {
    pub fn probe() {
        println!(\"x\");
        eprintln!(\"x\");
    }
}"
  "obs-event|pub fn probe() {
    viola_core::obs_event!(INFO, viola_core::obs::ObsEvent::ProcessStart);
}"
)

fired=0
failing=()
for probe in "${bans[@]}"; do
  IFS='|' read -r id lint body <<< "$probe"
  dir="$scratch/$id"
  make_project "$dir" "$body"
  rc=0
  run_clippy "$dir" || rc=$?
  if [ "$rc" -ne 0 ] && grep -qF "$lint" "$dir/clippy.log"; then
    fired=$((fired + 1))
    echo "fired   $id ($lint)"
  else
    failing+=("$id")
    echo "FAILED  $id (exit $rc, expected: $lint) — $dir/clippy.log"
  fi
done

clean=0
for control in "${controls[@]}"; do
  id=${control%%|*}
  body=${control#*|}
  dir="$scratch/$id"
  make_project "$dir" "$body"
  rc=0
  run_clippy "$dir" || rc=$?
  if [ "$rc" -eq 0 ]; then
    clean=$((clean + 1))
    echo "clean   $id"
  else
    failing+=("$id")
    echo "FAILED  control $id (exit $rc) — $dir/clippy.log"
  fi
done

# Every raw `event!` invocation under $1 outside viola_core::obs, one `path:line:` per hit; grep's
# own error (exit 2) fails the whole script rather than reading as "no hits".
raw_event_hits() {
  local out rc=0
  out=$(cd "$1" && grep -rnE --include='*.rs' --exclude-dir=target --exclude-dir=.git \
    '\bevent!\s*[({[]' .) || rc=$?
  if [ "$rc" -gt 1 ]; then
    echo "lint-probes: FAILED — raw-event grep exited $rc" >&2
    exit 1
  fi
  printf '%s\n' "$out" | grep -v '^\./crates/viola-core/src/obs\.rs:' | grep -v '^$' || true
}

grep_ok=1
positive="$scratch/raw-event-positive"
mkdir -p "$positive/src"
printf 'fn f() {\n    tracing::event!(tracing::Level::INFO, "x");\n}\n' > "$positive/src/lib.rs"
if [ -z "$(raw_event_hits "$positive")" ]; then
  grep_ok=0
  echo "FAILED  raw-event grep missed a planted tracing::event!"
fi
negative="$scratch/raw-event-negative"
mkdir -p "$negative/src"
printf 'fn f() {\n    viola_core::obs_event!(INFO, ObsEvent::ProcessStart);\n}\n' > "$negative/src/lib.rs"
if [ -n "$(raw_event_hits "$negative")" ]; then
  grep_ok=0
  echo "FAILED  raw-event grep flagged obs_event!"
fi
tree_hits=$(raw_event_hits "$root")
if [ -n "$tree_hits" ]; then
  grep_ok=0
  echo "FAILED  raw event! outside viola_core::obs:"
  printf '%s\n' "$tree_hits"
fi
[ "$grep_ok" -eq 1 ] && echo "clean   raw-event grep (planted hit found, obs_event! passed, tree clean)"

if [ "$fired" -eq "${#bans[@]}" ] && [ "$clean" -eq "${#controls[@]}" ] && [ "$grep_ok" -eq 1 ]; then
  echo "lint-probes: $fired bans fired, $clean controls clean"
  exit 0
fi
echo "lint-probes: FAILED — $fired/${#bans[@]} bans fired, $clean/${#controls[@]} controls clean, raw-event grep $([ "$grep_ok" -eq 1 ] && echo clean || echo red); failing: ${failing[*]:-none}"
exit 1
