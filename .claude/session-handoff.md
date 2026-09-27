# Session Handoff

**Last Updated:** 2026-09-27T01:15:37Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-26-local-linux-pre-push-gate — WSL2 pre-push gate, uncommitted-promotion base rule, pump resize baseline

## Position
- Done: 2026-09-26-local-linux-pre-push-gate — `viola-harness pre-push` runs the ubuntu suites + `ubuntu-latest` leg in
  WSL2 `Ubuntu` and the `windows-2025` leg on the host before every push; the Linux resize red it found is fixed.
- Next: Instance state and start order (`/andromeda-phase` to promote + plan it).

## Work done
- Wrap resumed across a session boundary (paused mid-P2 at the context alarm): the remaining spec edits, the cascade,
  curation, route-resolve and the commit ran in this session from `.andromeda/runs/2026-09-27T00-51-08-wrap/`.

## Drift resolved
- 24 spec-body amendments (test-plan 7, architecture 8, security-plan 7, obs-plan 2), sidecar entries carrying the
  23-pattern cascade sweep; 11 leaves re-derived; CLAUDE.md GENERATED blocks recomputed, unchanged.
- 1 escalation (the test-only `FAKE_AGENT_PUMP_DELAY_MS` env seam — a boundary widening touching [Naming]) resolved by
  the operator's recorded ratification: a carve-out behind `cfg(feature="fake-agent")`, capped at 5 s, absent from
  release builds (security-plan Decisions Log 2026-09-27).

## Notes
- **Every pre-push / light gate:** stop rust-analyzer by exact ExecutablePath first (host-win32.md).
- **Operator pass now runs `pre-push` before the pre-CI commit** (arch §CI/CD approach; `docs/workflow.md`).
- **For overseer1:** `evidence/plan-template-proposal.md` (pre-push before the pre-CI commit in the plan template's
  operator pass); the planlint check-9 slot question; the local-vs-CI wall-clock pair in `evidence/operator-pass.md`.
- Epoch 2 header wording stays as is (overseer decision: the friction-log grouping keys on it).
- **Deferred learnings:** `git reset --hard` already drops files the clone's own `add -A` staged, so a `clean` guard's red
  half needs a stray clone-side file (0.8, cap); carried: a `cfg!()`-valued fn is an equivalent mutant on one OS's leg,
  so make it a const (0.8); `check-runs` by sha mixes superseded runs after a force-push (0.8).
- **recurrence-despite-learning:** host-win32.md §Paths (`MSYS2_ARG_CONV_EXCL` for a leading-`/` argument) — the
  implement work re-hit the `/mnt/…` mangling anyway.
- **Carried:** the code-metrics `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is owed at the next
  ledger-mode audit; two Windows-only unviable fake-agent `main` mutants observed, not chased.
- Last failed command: none.

## Session End Status
Completed normally at 2026-09-27 04:01:02
