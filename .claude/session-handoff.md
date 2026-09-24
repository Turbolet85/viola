# Session Handoff

**Last Updated:** 2026-09-24T07:23:46Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean. CI was red on wrap commit `b0236ca` (run 35971295434). Fix `966b7aa` is green on all 3 OS + mutants (run 35972580463); v1-06's CI witness is noted in the matrix.
**Last Commit:** 2026-09-24-three-os-ci-headless-harness-skeleton — fix: CI test tools, Unix zombie kill, mutation prebuild

## Position
- Done: 2026-09-24-three-os-ci-headless-harness-skeleton. Workspace (rust 1.98.1 pin, floor 1.96), `viola run` first slice, `viola-harness` with five commands, minimal fake agent, mutation gate, 3-OS `ci.yml`.
- Next: /andromeda-phase to promote + plan "Fake agent and test-data fixtures".
- CI witness done: run 35972580463 on `966b7aa` is `success` on all jobs. Every future push runs the `mutants` job against `github.event.before`.

## Work done
Built the chunk (25 new files). Local gates are green, including the mutation gate (286 mutants: 244 caught, 42 unviable, 0 missed, 0 timeout). The default two-instance boot was smoked through the pwsh shim.

## Drift resolved
- 46 detector proposals.
- 44 applied across architecture, security-plan, test-plan and obs-plan (4 sidecars created):
  - toolchain pin, 1.96 floor;
  - CI actions and jobs;
  - test-only crate, bins, env vars, paths;
  - `target/harness` build dir;
  - the working-tree mutation diff and the push trigger;
  - interim readiness/status/cleanup fields;
  - the fake agent's lint exemption.
- 2 rejected as sequencing deferrals (`--home` hardening, R8 strip); they are now CARRY pins on their route owners.
- 0 escalations.
- Leaves re-derived: CLAUDE.md overview, docs/stack, docs/commands, docs/conventions, docs/workflow, rules/verification-harness.

## Notes
- Operator decisions (phase P4):
  - walking skeleton, where `viola run` is the first slice of the real verb;
  - mutation CI on push + PR;
  - the harness grammar grows per chunk.
- 5 CARRY pins: Diagnostics plane, Observability gates, PTY wrapper on Windows, CLI machine contract, Home and code-bearing file integrity.
- Deferred learnings (curation cap, for review):
  - `rg` is not on the gate shell's PATH, so use `grep -rE`;
  - `grep … | grep -c` under pipefail reads green on a missing file, so count with one process that errors on a missing input.
- Host: the rust-analyzer component was added to the 1.98.1 toolchain (the code-graph rust plane needs it).
- `.andromeda/runs/2026-09-24T05-49-06-phase/control/` holds the neutralised P5 probe controls; deleting it was permission-denied.
- Last failed command: none open.
