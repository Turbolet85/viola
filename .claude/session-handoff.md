# Session Handoff

**Last Updated:** 2026-09-24T07:23:46Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** escalation-open. CI was red on wrap commit `b0236ca` (run 35971295434); a fix commit was pushed, and its CI is being watched. The chunk counts as closed only when all 3 OS + mutants are green (operator directive).
**Last Commit:** 2026-09-24-three-os-ci-headless-harness-skeleton — fix: CI test tools, Unix zombie kill, mutation prebuild

## Position
- Done: 2026-09-24-three-os-ci-headless-harness-skeleton. Workspace (rust 1.98.1 pin, floor 1.96), `viola run` first slice, `viola-harness` with five commands, minimal fake agent, mutation gate, 3-OS `ci.yml`.
- Next: /andromeda-phase to promote + plan "Fake agent and test-data fixtures".
- Owed at the next session start: read CI for the wrap commit's sha. The plan gate `gh api repos/Turbolet85/viola/commits/<sha>/check-runs` must read `success` for the `test` (3 OS) + `mutants` jobs. That is the CI half of matrix v1-06. If it is red, disposition it before promoting.

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
