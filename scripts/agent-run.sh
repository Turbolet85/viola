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
#              fake agent, start the wrappers, wait for readiness
#   run      - invoke the suites (--unit|--integration|--e2e|--mutants|--coverage|--perf|
#              --fuzz-replay|--all), one JSON summary
#   status   - the session's status document (test-plan §3 Status endpoint shape)
#   cleanup  - graceful stop, endpoint checks, home removal (idempotent)
#   logs     - merged ndjson of events.ndjson + diagnostics role files + instance detail files
# Internal subcommands forwarded unchanged (not agent-facing): supervise, gate, the
# CI gate bodies schema-check (G4) and secret-scan, and pre-push (the operator pass's local Linux gate).
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

# `viola-harness boot` builds the workspace itself (test-plan §3), so the shim adds no pre-build step.
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
  supervise|gate|schema-check|secret-scan|pre-push)
    harness "$@"
    ;;
  *)
    usage
    ;;
esac
