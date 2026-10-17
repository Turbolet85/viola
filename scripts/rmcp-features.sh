#!/usr/bin/env bash
# The release graph's rmcp features are exactly `server` + `transport-io` (test-plan §9 Lint), beside
# deny.toml's [[bans.features]] gate. Non-vacuous: rmcp must first be present in viola's normal
# graph, so a graph that lost rmcp cannot read green. A feature counts as requested when a package
# other than rmcp enables it; the ones rmcp turns on itself (`transport-async-rw`, `schemars`, …)
# follow from those two. One verdict line, last.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

rc=0
out=$(cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth 2>&1) || rc=$?
out=$(printf '%s\n' "$out" | tr -d '\r')

if ! printf '%s\n' "$out" | grep -q '^0rmcp v'; then
  if [ "$rc" -eq 0 ] || printf '%s\n' "$out" | grep -qF 'did not match any packages'; then
    echo "rmcp-features: FAILED — rmcp is absent from viola's release graph"
  else
    echo "rmcp-features: FAILED — cargo tree did not run"
  fi
  exit 1
fi

# Depth 1 is each enabled rmcp feature; a depth-2 line under it that is not an rmcp feature is the
# package that requested it.
requested=$(printf '%s\n' "$out" | awk '
  /^1rmcp feature "/ { split($0, part, "\""); feature = part[2]; next }
  /^2/ && $0 !~ /^2rmcp feature "/ && feature != "" { seen[feature] = 1 }
  END { for (f in seen) print f }
' | sort | tr '\n' ' ' | sed 's/ $//')

if [ "$requested" != "server transport-io" ]; then
  echo "rmcp-features: FAILED — rmcp features requested: ${requested:-none}; expected: server transport-io"
  exit 1
fi
echo "rmcp-features: server transport-io"
