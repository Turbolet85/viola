#!/usr/bin/env bash
# `cargo modules orphans --deny` over every lib and bin target: a .rs file that no `mod` declaration reaches is
# never compiled, so its tests and lints stop running without any error. `--probe` proves the check fires on a
# planted orphan (and stays clean on its controls) before the real run is trusted.
# Each target is analysed for all three CI target triples, and a file is an orphan only when every triple reports it:
# a `#[cfg(windows)] mod x;` file reads as an orphan when analysed for Linux, and the reverse (measured on
# cargo-modules 0.27.0), so analysing only the host's own triple fails whichever leg compiles the file out.
# `dependencies --acyclic` is deliberately absent: at cargo-modules 0.27.0 every type with an inherent method
# reads as a type<->method cycle whatever the filters, so it stays an on-demand review
# (architecture §Infrastructure Patterns, Build system).
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
triples=(x86_64-pc-windows-msvc x86_64-unknown-linux-gnu aarch64-apple-darwin)

if ! cargo modules --version >/dev/null 2>&1; then
  echo "tool-missing: cargo-modules"
  exit 1
fi
if ! jq --version >/dev/null 2>&1; then
  echo "tool-missing: jq"
  exit 1
fi

# Runs `cargo modules orphans <args> --deny` once per triple and prints the orphan lines every triple reports.
# Returns 0 when there are none, 1 when there are, and 2 when a run failed without naming an orphan (fail closed).
orphans_everywhere() {
  local triple rc log found common="" first=1
  for triple in "${triples[@]}"; do
    rc=0
    log=$(cargo modules orphans "$@" --deny --target "$triple" 2>&1) || rc=$?
    found=$(printf '%s\n' "$log" | grep -oE 'orphaned module `[^`]+` at [^[:space:]]+' | sort -u || true)
    if [ "$rc" -ne 0 ] && [ -z "$found" ]; then
      printf '%s\n' "$log"
      return 2
    fi
    if [ "$first" -eq 1 ]; then
      common=$found
      first=0
    else
      common=$(comm -12 <(printf '%s\n' "$common") <(printf '%s\n' "$found") | grep -v '^$' || true)
    fi
  done
  if [ -n "$common" ]; then
    printf '%s\n' "$common"
    return 1
  fi
  return 0
}

check() {
  local targets pkg kind name features total=0 clean=0 rc
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
    rc=0
    (cd "$root" && orphans_everywhere "${args[@]}") || rc=$?
    if [ "$rc" -eq 0 ]; then
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

# One planted case: `fire` expects the named orphan on every triple, `clean` expects none.
probe_case() {
  local want=$1 dir=$2 module=$3 rc=0 out
  out=$(orphans_everywhere --manifest-path "$dir/Cargo.toml" --lib) || rc=$?
  printf '%s\n' "$out" > "$dir.log"
  if [ "$want" = fire ] && [ "$rc" -eq 1 ] && grep -qF "orphaned module \`$module\`" "$dir.log"; then
    echo "fired    $module"
    return 0
  fi
  if [ "$want" = clean ] && [ "$rc" -eq 0 ]; then
    return 0
  fi
  echo "FAILED   $(basename "$dir") (want $want, exit $rc) — $dir.log"
  return 1
}

probe() {
  local scratch fired=0 controls=0
  scratch="$root/target/orphans-probes/run-$(date -u +%Y%m%dT%H%M%SZ)-$$"
  make_crate "$scratch/orphan"
  echo 'pub fn stray() {}' > "$scratch/orphan/src/stray.rs"
  # Gated on a target no CI triple compiles: an orphan on every one of them.
  make_crate "$scratch/unreached"
  printf '#[cfg(target_os = "haiku")]\nmod gated;\npub fn a() {}\n' > "$scratch/unreached/src/lib.rs"
  echo 'pub fn g() {}' > "$scratch/unreached/src/gated.rs"
  make_crate "$scratch/control"
  # One file per OS family: each is reached by some triple, so neither is an orphan.
  make_crate "$scratch/per-os"
  printf '#[cfg(windows)]\nmod win;\n#[cfg(unix)]\nmod nix;\npub fn a() {}\n' > "$scratch/per-os/src/lib.rs"
  echo 'pub fn w() {}' > "$scratch/per-os/src/win.rs"
  echo 'pub fn n() {}' > "$scratch/per-os/src/nix.rs"

  probe_case fire "$scratch/orphan" stray && fired=$((fired + 1))
  probe_case fire "$scratch/unreached" gated && fired=$((fired + 1))
  probe_case clean "$scratch/control" - && controls=$((controls + 1))
  probe_case clean "$scratch/per-os" - && controls=$((controls + 1))

  if [ "$fired" -eq 2 ] && [ "$controls" -eq 2 ]; then
    echo "orphans-check probes: 2/2 fired, control clean"
    return 0
  fi
  echo "orphans-check probes: FAILED — $fired/2 fired, control $([ "$controls" -eq 2 ] && echo clean || echo red)"
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
