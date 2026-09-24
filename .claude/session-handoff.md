# Session Handoff

**Last Updated:** 2026-09-24T09:00:42Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-fake-agent-and-test-data-fixtures — feat: consumer-first fake agent, root fixture chain, hygiene walk, mutation no-rust-delta verdict

## Position
- Done: 2026-09-24-fake-agent-and-test-data-fixtures.
  - Fake agent: script/control/receipt contract, bracketed paste, hooks from `<plugin-dir>/hooks/hooks.json`, five modes.
  - Sync root fixture chain in `tests/support/`, the fixture hygiene walk, `ViolaName` property tests.
  - The mutation gate's `no-rust-delta` verdict, plus removal of stale `outcomes.json`.
- Next: /andromeda-phase to promote and plan "Supply-chain and workflow gates".
- CI witness owed: plan gates 19-20. Push (this wrap pushes the branch), then read `check-runs` for the pushed sha. Expect `success` on all 3 test legs + `mutants` (the chunk folds CI run 35973118026's mutants red).

## Work done
- 10 new files, 7 modified.
- Local gates green: mutation gate 81 mutants, 33 caught, 48 unviable, 0 missed; boot smoke with two instances ready and clean.

## Drift resolved
- 20 proposals (arch 6, test-plan 14).
  - 16 applied: architecture §Occupied Resources + tree; test-plan §2, §3, §7, §10, §12.
  - 2 escalations resolved by the operator: fixtures are named `<Event>.<variant>.json` (PascalCase); `boot` passes the parent `--fixtures`. "Code wins where the doc invented a shape."
  - Rejected: an arch §Stack test-lib row ([Deferred]), `tests/support/` as a Repository entry (over-reach).
  - T8-T10 are sequencing (the owners are CARRY pins).
- Leaves re-derived: CLAUDE.md overview, rules/testing, rules/verification-harness, docs/conventions, docs/tests-summary, docs/commands.

## Notes
- Operator decisions:
  - P1: fold the CI mutants red, with a no-vacuous-pass constraint.
  - P4: consumer-first, no invented shapes.
  - Wrap P2: code wins over an invented doc shape; no mapping tables.
- 6 new CARRY pins:
  - PTY wrapper: the `viola_e2e::fixtures` copy, and the root chain's stdin pipe → PTY.
  - viola verify: the `fixtures/claude` walk + schema, and `stamped_home` stamping.
  - Drift contract: hook matchers.
  - Readiness gate: `--vt100-panic-bytes`.
  - Statusline: `statusline-echo` + the `settings.json` read.
  - viola list: `agents --json` stub.
- `cargo deny check` has no `deny.toml` yet. "Supply-chain and workflow gates" (the next entry) owns it. The C-crate probe reads 0.
- Deferred learnings from the prior session (still for review):
  - `rg` is not on the gate shell's PATH;
  - `grep … | grep -c` under pipefail.
- `.andromeda/runs/2026-09-24T05-49-06-phase/control/` still holds the neutralised P5 probe controls (deleting it was permission-denied).
- The operator's viola-lab prototype (`viola` 12172 `run viola-builder`, 10348 `wait viola-builder`) was running on the host throughout; it is not this project's.
- Last failed command: none open.
