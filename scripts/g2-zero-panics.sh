#!/usr/bin/env bash
# G2 zero panics (obs-plan §9): no `event:"panic"` line in any codes-only role file (`diagnostics/*.ndjson`, never
# `detail-*`) of the homes kept under target/e2e-home. One exemption: the forced panic of the fake-agent seam, a line
# whose `panic_location` is exactly the seam's file plus `:<line>` — the file compared as a whole string, never a
# prefix, suffix or pattern over the path. `--probe` proves the exemption holds for that file only (another file,
# look-alike paths and a missing location read red) and that an empty scope reads red, before the real run is trusted.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
seam_file="src/cmd/hook/seam.rs"

if ! jq --version >/dev/null 2>&1; then
  echo "tool-missing: jq"
  exit 1
fi

# Prints the count of non-exempt panic lines in the role files under $1. Returns 2 when the scope holds no role file.
count_panics() {
  local scope=$1
  if [ -z "$(find "$scope/" -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -print -quit 2>/dev/null)" ]; then
    return 2
  fi
  find "$scope/" -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -exec awk 1 {} + \
    | jq -R -n --arg seam "$seam_file" '
        def exempt:
          (.panic_location | type) == "string"
          and (.panic_location | test(":[0-9]+$"))
          and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
        [inputs | fromjson? | select(.event == "panic") | select(exempt | not)] | length' \
    | tr -d '\r'
}

check() {
  local n rc=0
  n=$(count_panics "$root/target/e2e-home") || rc=$?
  if [ "$rc" -eq 2 ]; then
    echo "g2: empty scope — no role file under target/e2e-home"
    return 1
  fi
  if [ "$rc" -ne 0 ] || [ -z "$n" ]; then
    echo "g2: FAILED — the scan did not complete"
    return 1
  fi
  if [ "$n" -ne 0 ]; then
    echo "g2: $n panic line(s)"
    return 1
  fi
  echo "g2: clean"
}

# One role file per case, under its own home in the probe scope.
plant() {
  local dir="$probe/$1/h/diagnostics"
  mkdir -p "$dir"
  printf '%s\n' "$3" >> "$dir/$2"
}

panic_at() {
  printf '{"event":"panic","level":"ERROR","process":"hook","panic_location":"%s","thread":"main"}' "$1"
}

probe() {
  local failed=0 case want n rc
  probe="$root/target/g2-probe"
  rm -rf "$probe"
  mkdir -p "$probe"
  plant seam-alone hook-builder.ndjson "$(panic_at "$seam_file:12")"
  plant other-file hook-builder.ndjson "$(panic_at "src/cmd/hook.rs:9")"
  plant prefixed-path hook-builder.ndjson "$(panic_at "x/$seam_file:3")"
  plant bak-suffix hook-builder.ndjson "$(panic_at "$seam_file.bak:3")"
  plant x-suffix hook-builder.ndjson "$(panic_at "${seam_file}x:3")"
  plant no-line hook-builder.ndjson "$(panic_at "$seam_file")"
  plant no-location hook-builder.ndjson '{"event":"panic","level":"ERROR","process":"hook","thread":"main"}'
  plant detail-only hook-builder.ndjson '{"event":"hook-invoked","level":"INFO","process":"hook"}'
  plant detail-only detail-hook.ndjson "$(panic_at "src/cmd/hook.rs:9")"
  mkdir -p "$probe/empty-scope/h"
  for case in seam-alone:clean other-file:red prefixed-path:red bak-suffix:red x-suffix:red no-line:red \
    no-location:red detail-only:clean empty-scope:red; do
    want=${case#*:}
    case=${case%%:*}
    rc=0
    n=$(count_panics "$probe/$case") || rc=$?
    if [ "$want" = clean ] && [ "$rc" -eq 0 ] && [ "$n" = 0 ]; then
      continue
    fi
    if [ "$want" = red ] && { [ "$rc" -eq 2 ] || { [ "$rc" -eq 0 ] && [ -n "$n" ] && [ "$n" -ne 0 ]; }; }; then
      continue
    fi
    echo "g2-probe: $case read ${n:-no count} (rc $rc), expected $want"
    failed=1
  done
  rm -rf "$probe"
  if [ "$failed" -ne 0 ]; then
    echo "g2-probe: FAILED"
    return 1
  fi
  echo "g2-probe: all cases as expected"
}

case "${1:-}" in
  --probe) probe ;;
  "") check ;;
  *)
    echo "usage: g2-zero-panics.sh [--probe]"
    exit 2
    ;;
esac
