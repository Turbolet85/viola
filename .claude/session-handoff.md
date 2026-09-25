# Session Handoff

**Last Updated:** 2026-09-25T11:43:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD d14f234 = the pushed pre-CI commit)
**Status:** clean
**Last Commit:** 2026-09-24-epoch-1-cleanup — the chunk commit on top of the pre-CI commit d14f234

## Position
- Done: `2026-09-24-epoch-1-cleanup`. The Epoch 1 problem spots are cleaned up:
  - the MAX_FRAME pin is mutation-witnessed;
  - cognitive over 15: 3 → 0;
  - run.rs 1 562 → 472 code lines, split into `harness/run/*`;
  - in-scope clone pairs 9 → 0;
  - the Rust gate deferral is closed.

  Folded CI reds: the mutation leg now streams per-mutant progress, a `cli.rs` Drop guard stops leaked sessions, and an
  unviable-swamp rule applies. CI run 36126924953 on d14f234: 15/15 success.
- Next: `/andromeda-phase` to promote and plan "Security prerequisites", the next markerless Epoch 2 entry.

## Work done
- 8 source files modified and 5 new `harness/run/*` modules, plus evidence and operator passes 1–4 (`chunks/2026-09-24-epoch-1-cleanup/evidence/`).
- Two force-pushes and one rewind of `build/viola-0.1.0` (never `main`), each on the founder's ruling, each with `--force-with-lease`.

## Drift resolved
- **test-plan:** 12 proposals plus 1 orchestrator-raised addition, all applied (§2, §3 `run` step 4 / Exit semantics / `gate` /
  quality-gate-config-emit, §9, §10 Mutation gate, §11, §12 entry).
  - The Founder Direction 1 clause "unviable … do not fail the gate" is now tightened to `unviable <= caught`, on the overseer's decision.
- **Cross-master restatements:** `architecture.md:515` and `obs-plan.md:1253` amended.
- **Leaves re-derived:** `commands.md`, `tests-summary.md`, `testing.md`, `verification-harness.md`.
- **Escalation resolved:** 1 — the playbook rule "Verbatim scope copy" (obs-plan §1 stays out of cascade sweeps), operator-approved.

## Notes
- **Operator decisions:**
  - the swamp rule is `unviable > caught`, picked by measurement across the chunk-7, chunk-8 and this chunk's legs;
  - the reduced partial-verdict upload for cancelled legs is **declined** (a decision, not a deferral);
  - every new guard test carries its remove-the-guard run.
- **Observation, not red, not chased (overseer):** two Windows-only unviable fake-agent `main` mutants (`viola-fake-agent.rs:463:5`,
  `:482:8`, each after a 1 s build); ubuntu caught both. Hypothesis, not measured: a relink lock from the `--exit-no-eof` test's grandchild
  holding the exe for 10 s.
- **CI reads by sha:** `gh api …/commits/{sha}/check-runs` returns every run on that sha, including superseded ones after a force-push.
- **The repo is now PUBLIC** (Actions minutes free). The earlier billing block on `mutants-verdict` no longer applies (verified in run 36126924953).
- **Deferred learnings (Filter 5 cap, 0.8 each):**
  - a `cfg!()`-valued fn is an equivalent mutant on one OS's leg; make it a const (an additive facet of testing.md's cfg entry);
  - `check-runs` by sha mixes superseded runs after a force-push.
- **Code-metrics ledger correction still owed:** the next ledger-mode code-audit record carries the `mutation.survivors` column correction
  (`lib.rs:9:31` / `:9:38`).
- Last failed command: none open.

## Session End Status
Completed normally at 2026-09-25 14:08:42
