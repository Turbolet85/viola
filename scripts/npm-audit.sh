#!/usr/bin/env bash
# Audits e2e-web/package-lock.json, which sits outside both cargo graphs (the fuzz/Cargo.lock
# precedent): npm advisories at every level, and every locked package resolved from the npm registry.
# The raw JSON goes to target/npm-audit/, never to the uploaded target/supply-chain/.
#   (no flag)          advisories + sources; exit 1 naming the failing half
#   --advisories-only  advisories only (the weekly nightly twin)
#   --probe            prove the sources check refuses a minted lockfile with a foreign `resolved` URL
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
lock="$root/e2e-web/package-lock.json"
out="$root/target/npm-audit"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "tool-missing: $1"
    exit 1
  fi
}

# sources <lockfile>: every `packages.*.resolved` starts with the npm registry.
sources() {
  jq -e '[.packages[] | .resolved // empty | select(startswith("https://registry.npmjs.org/") | not)] | length == 0' \
    "$1" >/dev/null
}

# advisories: sets `count` from npm's own report; a report npm could not produce refuses.
advisories() {
  mkdir -p "$out"
  npm audit --prefix "$root/e2e-web" --package-lock-only --audit-level=low --json > "$out/audit.json" || true
  count=$(jq -e '.metadata.vulnerabilities.total' "$out/audit.json" 2>/dev/null) || {
    echo "npm-audit: advisories unreadable"
    exit 1
  }
}

audit() {
  need npm
  need jq
  advisories
  if [ "$count" != 0 ]; then
    echo "npm-audit: advisories $count"
    exit 1
  fi
  if [ "${1:-}" = advisories-only ]; then
    echo "npm-audit: advisories 0"
    return 0
  fi
  if ! sources "$lock"; then
    echo "npm-audit: advisories 0, sources outside registry.npmjs.org"
    exit 1
  fi
  echo "npm-audit: advisories 0, sources registry.npmjs.org only"
}

probe() {
  local tmp refused=0
  need jq
  tmp=$(mktemp -d)
  jq '.packages["node_modules/@playwright/test"].resolved = "https://example.invalid/test-1.63.0.tgz"' \
    "$lock" > "$tmp/package-lock.json"
  if ! sources "$tmp/package-lock.json"; then
    refused=$((refused + 1))
  fi
  rm -rf "$tmp"
  if [ "$refused" -ne 1 ]; then
    echo "npm-audit --probe: $refused/1 refused"
    exit 1
  fi
  if ! sources "$lock"; then
    echo "npm-audit --probe: 1/1 refused, control dirty"
    exit 1
  fi
  echo "npm-audit --probe: 1/1 refused, control clean"
}

case "${1:-}" in
  "") audit ;;
  --advisories-only) audit advisories-only ;;
  --probe) probe ;;
  *)
    echo "usage: npm-audit.sh [--advisories-only|--probe]"
    exit 2
    ;;
esac
