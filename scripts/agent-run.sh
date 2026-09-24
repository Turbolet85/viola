#!/usr/bin/env bash
# scripts/agent-run.sh - agent-driven verification harness driver (POSIX shim)
# Generated once by /andromeda-setup-project Phase 4 from test-plan §3 + obs-plan §3.
# Project-evolvable: setup never overwrites this file on a re-run (drift is backed up and surfaced instead).
#
# Contract (test-plan §3 "Harness implementation"): agent-run.sh and agent-run.ps1 are identical THIN
# shims. Each runs `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forwards its
# exit code and stdout unchanged. All logic lives once, in Rust (crates/viola-e2e), so both shells have
# byte-identical semantics on Windows, macOS and Linux.
#
# 5-command discipline (the agent's surface):
#   boot     - build, create a fresh home under target/e2e-home/, stamp it via `viola verify` against the
#              fake agent, start the wrappers (+ `viola ui`), wait for readiness
#   run      - invoke the suites (--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|
#              --fuzz-replay|--all), one JSON summary
#   status   - `viola list --json` + /ready + cookie-gated /api/sessions, api_sessions_equal_list
#   cleanup  - graceful stop, endpoint / port / url-file checks, home removal (idempotent)
#   logs     - merged ndjson of events.ndjson + diagnostics role files + instance detail files
# Internal subcommands forwarded unchanged (not agent-facing): supervise, ui-restart, gate, and the
# CI gate bodies schema-check (G4) and secret-scan.
#
# Every command prints exactly one JSON document on stdout: {"v":1,"cmd":"<command>","ok":<bool>,...}
# Exit codes: 0 ok, 1 failure, 2 usage error.
#
# Bound contracts:
#   - test-plan §3: 5-command discipline, status shape, session record, log format
#   - obs-plan §3: log sinks under <home>/diagnostics/, the closed `event` enum, corr / conn

set -euo pipefail

# UTF-8 for every Python-mediated leg - legacy-codepage hosts corrupt em-dash/arrow-dense
# project text otherwise (harmless where the default is already UTF-8)
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Embedded-asset freshness: test-plan §3 puts the refresh INSIDE `viola-harness boot` - step 1 always
# runs `cargo build --workspace --features fake-agent` before any process starts (the embedded page is
# rebuilt whenever crates/viola-ui assets change), and after UI readiness boot byte-compares every
# embedded /assets/* path with the repo file (reason "stale-embedded-assets"). The shim therefore adds
# no step of its own; extend here only if a future plan names a pre-build step outside the harness.
ensure_fresh_artifacts() {
  :
}

harness() {
  cd "$ROOT"
  exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"
}

usage() {
  printf '%s\n' '{"v":1,"cmd":null,"ok":false,"reason":"usage"}'
  echo "Usage: $0 {boot|run|status|cleanup|logs} [flags]" >&2
  exit 2
}

case "${1:-}" in
  boot)
    ensure_fresh_artifacts
    harness "$@"
    ;;
  run)
    harness "$@"
    ;;
  status)
    harness "$@"
    ;;
  cleanup)
    harness "$@"
    ;;
  logs)
    harness "$@"
    ;;
  supervise|ui-restart|gate|schema-check|secret-scan)
    harness "$@"
    ;;
  *)
    usage
    ;;
esac
