#!/usr/bin/env bash
# Proves every cargo-deny ban fires. A ban on a crate absent from viola's graph prints nothing, so a green
# `cargo deny check` cannot tell a working ban from a missing one: each probe is a throwaway Cargo project
# pulling one banned crate or feature, and `cargo deny check bans` with the repo's config must reject it
# with that ban's own diagnostic. A clean control project must pass both configs.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)

if ! cargo deny --version >/dev/null 2>&1; then
  echo "tool-missing: cargo-deny"
  exit 1
fi

scratch="$root/target/deny-probes/run-$(date -u +%Y%m%dT%H%M%SZ)-$$"
mkdir -p "$scratch"

# id | config | expected diagnostic (fixed string) | dependency line
probes=(
  "tokio|deny-sync.toml|error[banned]: crate 'tokio = |tokio = { version = \"1\", default-features = false }"
  "cc|deny.toml|error[banned]: crate 'cc = |cc = \"1\""
  "libsqlite3-sys|deny.toml|error[banned]: crate 'libsqlite3-sys = |libsqlite3-sys = { version = \"*\", default-features = false }"
  "openssl-sys|deny.toml|error[banned]: crate 'openssl-sys = |openssl-sys = \"*\""
  "opentelemetry-otlp|deny.toml|error[banned]: crate 'opentelemetry-otlp = |opentelemetry-otlp = { version = \"*\", default-features = false }"
  "opentelemetry-stdout|deny.toml|error[banned]: crate 'opentelemetry-stdout = |opentelemetry-stdout = { version = \"*\", default-features = false }"
  "sentry|deny.toml|error[banned]: crate 'sentry = |sentry = { version = \"*\", default-features = false }"
  "tracing-appender|deny.toml|error[banned]: crate 'tracing-appender = |tracing-appender = \"0.2\""
  "veil-toggle|deny.toml|error[feature-banned]: feature 'toggle' for crate 'veil = |veil = { version = \"0.3\", default-features = false, features = [\"toggle\"] }"
  "rmcp-transport-streamable-http-server|deny.toml|error[feature-banned]: feature 'transport-streamable-http-server' for crate 'rmcp = |rmcp = { version = \"3.4\", default-features = false, features = [\"transport-streamable-http-server\"] }"
  "rmcp-auth|deny.toml|error[feature-banned]: feature 'auth' for crate 'rmcp = |rmcp = { version = \"3.4\", default-features = false, features = [\"auth\"] }"
  "tracing-subscriber-env-filter|deny.toml|error[feature-banned]: feature 'env-filter' for crate 'tracing-subscriber = |tracing-subscriber = { version = \"0.3\", default-features = false, features = [\"env-filter\"] }"
  "serde_json-unbounded-depth|deny.toml|error[feature-banned]: feature 'unbounded_depth' for crate 'serde_json = |serde_json = { version = \"1\", features = [\"unbounded_depth\"] }"
)

# The empty [workspace] table keeps each probe out of the viola workspace that encloses target/.
make_project() {
  local dir=$1 dep=$2
  mkdir -p "$dir/src"
  : > "$dir/src/lib.rs"
  printf '[package]\nname = "deny-probe"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n[workspace]\n\n[dependencies]\n%s\n' "$dep" > "$dir/Cargo.toml"
}

run_deny() {
  local dir=$1 config=$2
  if cargo deny --config "$root/$config" --manifest-path "$dir/Cargo.toml" check bans > "$dir/deny.log" 2>&1; then
    return 0
  else
    return $?
  fi
}

total=${#probes[@]}
banned=0
failing=()
for probe in "${probes[@]}"; do
  IFS='|' read -r id config diagnostic dep <<< "$probe"
  dir="$scratch/$id"
  make_project "$dir" "$dep"
  rc=0
  run_deny "$dir" "$config" || rc=$?
  if [ "$rc" -ne 0 ] && grep -qF "$diagnostic" "$dir/deny.log"; then
    banned=$((banned + 1))
    echo "banned  $id"
  else
    failing+=("$id")
    echo "FAILED  $id (exit $rc, expected: $diagnostic) — $dir/deny.log"
  fi
done

# viola-channel leaves the deny-sync loop for its own tree form (scripts/channel-tokio-free.sh), run
# here from a throwaway workspace's copy: a normal tokio edge planted in viola-channel must fail it,
# and the same edge behind a `tokio` feature (the control) must pass it.
make_channel_workspace() {
  local dir=$1 features=$2 dep=$3
  mkdir -p "$dir/scripts" "$dir/crates/viola-channel/src"
  cp "$root/scripts/channel-tokio-free.sh" "$dir/scripts/"
  : > "$dir/crates/viola-channel/src/lib.rs"
  printf '[workspace]\nmembers = ["crates/*"]\nresolver = "3"\n' > "$dir/Cargo.toml"
  printf '[package]\nname = "viola-channel"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n[features]\n%s\n\n[dependencies]\n%s\n' "$features" "$dep" > "$dir/crates/viola-channel/Cargo.toml"
}

total=$((total + 1))
planted="$scratch/channel-tokio-edge"
make_channel_workspace "$planted" 'tokio = []' 'tokio = { version = "1", default-features = false }'
rc=0
bash "$planted/scripts/channel-tokio-free.sh" > "$planted/tree.log" 2>&1 || rc=$?
if [ "$rc" -ne 0 ] && grep -qF "channel-tokio-free: FAILED — viola-channel reaches tokio without its feature" "$planted/tree.log"; then
  banned=$((banned + 1))
  echo "banned  channel-tokio-edge"
else
  failing+=("channel-tokio-edge")
  echo "FAILED  channel-tokio-edge (exit $rc) — $planted/tree.log"
fi

control="$scratch/control"
make_project "$control" ""
control_ok=1
for config in deny.toml deny-sync.toml; do
  rc=0
  run_deny "$control" "$config" || rc=$?
  if [ "$rc" -ne 0 ]; then
    control_ok=0
    echo "FAILED  control under $config (exit $rc) — $control/deny.log"
  fi
done

channel_control="$scratch/channel-control"
make_channel_workspace "$channel_control" 'tokio = ["dep:tokio"]' 'tokio = { version = "1", default-features = false, optional = true }'
rc=0
bash "$channel_control/scripts/channel-tokio-free.sh" > "$channel_control/tree.log" 2>&1 || rc=$?
if [ "$rc" -ne 0 ] || ! grep -qxF "channel-tokio-free: ok" "$channel_control/tree.log"; then
  control_ok=0
  echo "FAILED  channel control (exit $rc) — $channel_control/tree.log"
fi

if [ "$banned" -eq "$total" ] && [ "$control_ok" -eq 1 ]; then
  echo "deny-probes: $banned/$total banned, control clean"
  exit 0
fi
echo "deny-probes: FAILED — $banned/$total banned, control $([ "$control_ok" -eq 1 ] && echo clean || echo red); failing: ${failing[*]:-none}"
exit 1
