#!/usr/bin/env bash
# `cargo modules orphans --deny` over every lib and bin target: a .rs file that no `mod` declaration reaches is
# never compiled, so its tests and lints stop running without any error. `--probe` proves the check fires on a
# planted orphan (and stays clean on a control) before the real run is trusted.
# `dependencies --acyclic` is deliberately absent: at cargo-modules 0.27.0 every type with an inherent method
# reads as a type<->method cycle whatever the filters, so it stays an on-demand review
# (architecture §Infrastructure Patterns, Build system).
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)

if ! cargo modules --version >/dev/null 2>&1; then
  echo "tool-missing: cargo-modules"
  exit 1
fi
if ! jq --version >/dev/null 2>&1; then
  echo "tool-missing: jq"
  exit 1
fi

check() {
  local targets pkg kind name features total=0 clean=0
  local -a failing=() args
  # One row per lib/bin target: package, lib|bin, target name, required features (comma-joined).
  targets=$(cargo metadata --no-deps --format-version 1 --manifest-path "$root/Cargo.toml" \
    | jq -r '.packages[] | .name as $p | .targets[]
        | select(any(.kind[]; . == "lib" or . == "bin"))
        | [$p, (if any(.kind[]; . == "lib") then "lib" else "bin" end), .name,
           ((.["required-features"] // []) | join(","))]
        | @tsv' \
    | tr -d '\r')
  if [ -z "$targets" ]; then
    echo "orphans-check: FAILED — no lib or bin target in the workspace"
    return 1
  fi
  while IFS=$'\t' read -r pkg kind name features; do
    [ -n "$pkg" ] || continue
    total=$((total + 1))
    args=(-p "$pkg")
    if [ "$kind" = lib ]; then args+=(--lib); else args+=(--bin "$name"); fi
    if [ -n "$features" ]; then args+=(--features "$features"); fi
    if (cd "$root" && cargo modules orphans "${args[@]}" --deny); then
      clean=$((clean + 1))
    else
      failing+=("$pkg/$name")
    fi
  done <<< "$targets"
  if [ "${#failing[@]}" -gt 0 ]; then
    echo "orphans-check: FAILED — ${failing[*]}"
    return 1
  fi
  echo "orphans-check: $clean/$total targets clean"
}

# The empty [workspace] table keeps each probe crate out of the viola workspace that encloses target/.
make_crate() {
  local dir=$1
  mkdir -p "$dir/src"
  printf '[package]\nname = "orphans-probe"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n[workspace]\n' > "$dir/Cargo.toml"
  echo 'pub fn a() {}' > "$dir/src/lib.rs"
}

probe() {
  local scratch rc fired=0 control_ok=0
  scratch="$root/target/orphans-probes/run-$(date -u +%Y%m%dT%H%M%SZ)-$$"
  make_crate "$scratch/orphan"
  echo 'pub fn stray() {}' > "$scratch/orphan/src/stray.rs"
  make_crate "$scratch/control"

  rc=0
  cargo modules orphans --manifest-path "$scratch/orphan/Cargo.toml" --lib --deny > "$scratch/orphan.log" 2>&1 || rc=$?
  if [ "$rc" -ne 0 ] && grep -qF 'orphaned module `stray`' "$scratch/orphan.log"; then
    fired=1
    echo "fired    orphan"
  else
    echo "FAILED   orphan (exit $rc, expected: orphaned module \`stray\`) — $scratch/orphan.log"
  fi

  rc=0
  cargo modules orphans --manifest-path "$scratch/control/Cargo.toml" --lib --deny > "$scratch/control.log" 2>&1 || rc=$?
  if [ "$rc" -eq 0 ] && grep -qF 'No orphans found.' "$scratch/control.log"; then
    control_ok=1
  else
    echo "FAILED   control (exit $rc) — $scratch/control.log"
  fi

  if [ "$fired" -eq 1 ] && [ "$control_ok" -eq 1 ]; then
    echo "orphans-check probes: 1/1 fired, control clean"
    return 0
  fi
  echo "orphans-check probes: FAILED — $fired/1 fired, control $([ "$control_ok" -eq 1 ] && echo clean || echo red)"
  return 1
}

case "${1:-}" in
  "") check ;;
  --probe) probe ;;
  *)
    echo "usage: orphans-check.sh [--probe]"
    exit 2
    ;;
esac
