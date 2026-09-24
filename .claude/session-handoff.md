# Session Handoff

**Last Updated:** 2026-09-24T10:52:16Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-diagnostics-plane — feat: closed ObsEvent vocabulary + obs_event!, per-role sinks, diagnostics_level, owner-only detail files, diag line schemas, logs detail source, write-guard repair

## Position
- Done: 2026-09-24-diagnostics-plane.
  - `viola_core::obs`: 19-value `ObsEvent`, `ObsProcess`, `ProcessCtx`, `obs_event!`, plus `MAX_FRAME`.
  - Root `viola::obs`: role files for all 5 roles (only `run` produces today), `config.json` `diagnostics_level`, owner-only `detail-<process>.ndjson` (panic payload + backtrace), run catch-site `process-exit{internal-error}`.
  - `schemas/diag-line.v1.json` + `diag-detail.v1.json`; harness `logs` streams detail files.
  - `.claude/settings.json` write guard repaired (it blocked nothing under Git Bash); `scripts/guard-probe.py` proves `2,2,2,2,0,0`.
  - Rust gate deferral (since 2026-09-24-supply-chain-and-workflow-gates) closed: `run --unit` 135 green; mutants `counted`, 0 survived.
- Next: /andromeda-phase to promote and plan "Log redaction and never-log floor" (carries a CARRY: route chains and drift reports through the new detail sink).
- **CI witness owed (plan operator entry):** read `check-runs` for this wrap's pushed sha. Expect `success` on every check.
- **Recorded:** CI witness for `4d8be52` (the previous chunk). Run `35983992260` (ci, push) and run `35984789181` (nightly) were both `success`.

## Work done
- 6 new files, 8 modified. 21 local gates green; gate 12 green after the operator-directed `env = []` edit. Smoke ✓ (p3-smoke booted ready, cleanup exact).

## Drift resolved
- 17 amendments across 4 masters: arch 9, obs 5, security 2, a11y 1. 1 escalation was resolved with the operator: obs §3/§11 now say `corr` is a caller-supplied typed field, not attached by `obs_event!`.
- Retired:
  - arch "hook/mcp diagnostics go to the instance dir / stderr";
  - obs §8 "`additionalProperties: false` per event" (the schemas use `unevaluatedProperties: false`);
  - a11y §12 "`a11y-violation` accepted as an enum amendment".
- Leaves re-derived: `rules/observability.md`, `docs/stack.md`.

## Notes
- Operator decisions this chunk:
  - The guard fix is proven by a repo-local probe.
  - The CLAUDE.md `v` learning was narrowed; applied as a correction.
  - `env = []` was added to plan gate 12.
  - `corr`: amend obs to shipped, and close the trade-off with a CARRY.
- New route pins: CARRY on "Wrapper channel" (make `corr` `required` in diag-line for corr-bearing events + a negative test), "Hooks to normalised events" (concurrent-append check + argv role classification) and "Log redaction" (the detail-sink pointer).
- The operator's viola-lab prototype (`viola.exe` 12172, 35652) was running; it is not this project's.
- Last failed command: none open.
