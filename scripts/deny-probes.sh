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
  "axum-http2|deny.toml|error[feature-banned]: feature 'http2' for crate 'axum = |axum = { version = \"0.8\", default-features = false, features = [\"http2\"] }"
  "tracing-subscriber-env-filter|deny.toml|error[feature-banned]: feature 'env-filter' for crate 'tracing-subscriber = |tracing-subscriber = { version = \"0.3\", default-features = false, features = [\"env-filter\"] }"
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

if [ "$banned" -eq "$total" ] && [ "$control_ok" -eq 1 ]; then
  echo "deny-probes: $banned/$total banned, control clean"
  exit 0
fi
echo "deny-probes: FAILED — $banned/$total banned, control $([ "$control_ok" -eq 1 ] && echo clean || echo red); failing: ${failing[*]:-none}"
exit 1
