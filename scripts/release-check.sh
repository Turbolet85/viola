#!/usr/bin/env bash
# Proves the release build ships only the product binary. The verdict comes from the build's own
# compiler-artifact records, never from a listing of target/release/: a shared or cached target dir keeps
# stale test-only executables (viola-harness from a --workspace build) that this build never produced.
# `--probe` feeds synthetic records that must be refused, so a judge that cannot fail is caught first.
set -euo pipefail

if ! jq --version >/dev/null 2>&1; then
  echo "tool-missing: jq"
  exit 1
fi

# Reads cargo --message-format=json lines on stdin; passes only when `viola` is the sole executable.
judge() {
  local names name
  # Native jq on Windows writes CRLF, which would make every name differ from "viola".
  names=$(jq -r 'select(.reason == "compiler-artifact" and .executable != null) | .target.name' | tr -d '\r')
  if [ -z "$names" ]; then
    echo "release-check: FAILED — no executable in the build output"
    return 1
  fi
  while IFS= read -r name; do
    if [ "$name" != "viola" ]; then
      echo "release-check: FAILED — test-only binary $name"
      return 1
    fi
  done <<< "$names"
  echo "release-check: viola only"
}

record() {
  printf '{"reason":"compiler-artifact","target":{"name":"%s","kind":["bin"]},"executable":"/probe/%s"}\n' "$1" "$1"
}

lib_record='{"reason":"compiler-artifact","target":{"name":"viola_core","kind":["lib"]},"executable":null}'
finished='{"reason":"build-finished","success":true}'

# id | expected verdict line | input stream
refused_by() {
  local id=$1 expected=$2 input=$3 out rc=0
  out=$(printf '%s' "$input" | judge) || rc=$?
  if [ "$rc" -ne 0 ] && [ "$out" = "$expected" ]; then
    echo "refused  $id"
    return 0
  fi
  echo "FAILED   $id (exit $rc, got: $out, expected: $expected)"
  return 1
}

probe() {
  local total=3 refused=0 control_ok=1 out rc=0
  refused_by viola-harness "release-check: FAILED — test-only binary viola-harness" \
    "$(record viola)"$'\n'"$(record viola-harness)"$'\n'"$finished"$'\n' && refused=$((refused + 1))
  refused_by viola-fake-agent "release-check: FAILED — test-only binary viola-fake-agent" \
    "$lib_record"$'\n'"$(record viola)"$'\n'"$(record viola-fake-agent)"$'\n' && refused=$((refused + 1))
  refused_by empty "release-check: FAILED — no executable in the build output" "" && refused=$((refused + 1))

  out=$(printf '%s' "$lib_record"$'\n'"$(record viola)"$'\n'"$finished"$'\n' | judge) || rc=$?
  if [ "$rc" -ne 0 ] || [ "$out" != "release-check: viola only" ]; then
    control_ok=0
    echo "FAILED   control (exit $rc, got: $out)"
  fi

  if [ "$refused" -eq "$total" ] && [ "$control_ok" -eq 1 ]; then
    echo "release-check probes: $refused/$total refused, control clean"
    return 0
  fi
  echo "release-check probes: FAILED — $refused/$total refused, control $([ "$control_ok" -eq 1 ] && echo clean || echo red)"
  return 1
}

build() {
  local json rc=0
  json=$(cargo build --release --locked --bin viola --message-format=json) || rc=$?
  if [ "$rc" -ne 0 ]; then
    echo "release-check: FAILED — cargo build exited $rc"
    return 1
  fi
  printf '%s\n' "$json" | judge
}

case "${1:-}" in
  "") build ;;
  --probe) probe ;;
  *)
    echo "usage: release-check.sh [--probe]"
    exit 2
    ;;
esac
