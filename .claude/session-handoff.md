# Session Handoff

**Last Updated:** 2026-10-04T05:38Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the pre-CI commit `afef92f` was pushed in the operator pass; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-04-readiness-gate-and-timing-constants — the wrap commit of Readiness gate and timing constants

## Position
- Done: **2026-10-04-readiness-gate-and-timing-constants**. The readiness gate's mechanism landed:
  - a pure vt100 `Screen` in viola-agent-claude, fed by a tee + feed thread on `run`'s pump (`src/run/gate.rs`);
  - a vt100 panic poisons the model until the size changes, with one `parse-rejected` line;
  - `viola_core::SPINE_DEADLINE` and `Clock` are named; the hook's 750 ms is `CONNECT_DEADLINE`;
  - a `vt100_feed` fuzz target (4 replayed targets);
  - CARRY 4 is fixed: fixture repos run git with `maintenance.auto=false`. Witnessed two-sided: control 4 and 5 vs
    fix 0 and 0.
  - CI ci#37179459192 on `afef92f` read green 15/15, the Windows passthrough included.
- Next: **Confirmed send with CL-1 records** (`working-route.md:74`) → `/andromeda-phase`. It now carries the unbounded
  tee → feed `mpsc` bound ("bound every input") and the `--vt100-panic-bytes` / feed-panic E2E / G2 question.

## Work done
- Code: `crates/viola-agent-claude/src/screen.rs`, `src/run/gate.rs`, `fuzz/fuzz_targets/vt100_feed.rs` + corpus,
  viola-core `SPINE_DEADLINE`/`Clock`, the perf gate's named bound, the fixture git config.
- Evidence: `evidence/{fuzz-override,guards,carry4,operator-pass}.md`.

## Drift resolved
- **22 amendments, 0 escalations:**
  - architecture: [Screen Model], [Delivery Confirmation], [Hook Transport], [CLI Version Compatibility], Stack row,
    `fuzz/`, plus 2 keyed contracts (9);
  - test-plan: §2, §6 ×2, §7, §10 ×3, plus 2 keyed contracts (10);
  - obs-plan §7 ×2;
  - security-plan PTY output row.
- 7 leaf files re-derived, CLAUDE.md modules included.
- PROVISIONAL, per the overseer's direction: the 300 ms / 5 s / 10 s values and the two-literal-list signature format.

## Notes
- **Held widening (founder morning, 2026-10-05):**
  - what it is: the PTY typed-input `viola verify` probe, the live 2.1.287 recording, and the signature / quiet-period /
    maximum-wait ledger rows;
  - it is HELD, nothing built toward it, and owed to `:82` or the founder's ruling (CARRY on `:82`);
  - its exact shape is in that chunk's `plan.md` §Held widening.
- **Epoch 3 holds 9 entries,** near the ~10 growth valve. A split is the operator's call.
- **`host-win32.md`** still describes the retired Windows host. Its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **Operator cleanup left on disk** (no later run needs any of it):
  - `../viola-mutants-scratch/c4ctl/` and `c4fix/` (873 MB each) and `wctl`, `wctl2`, `wfix`, `wfix2` (the CARRY 4
    witness);
  - the older residue the previous handoff listed. Keep the NOCOW scratch dir itself.
- **Installed `claude` is 2.1.287.** Fixtures exist only for 2.1.283, so it stays unverified until `viola verify` runs.
- **Deferred learnings** (carried): `recurrence-despite-learning: host-win32.md 2026-09-28`; the "not measured here"
  vocabulary; PID 1 as the cleanup-deadline target; the PTY master close needing no held clone; let a red CI run finish
  before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-04 08:09:38
