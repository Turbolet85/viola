# Session Handoff

**Last Updated:** 2026-11-24T11:54:43Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** feat(2026-11-24-viola-adopt): `viola adopt` — gated on R5

## Position
- Done: 2026-11-24-viola-adopt (`chk-ep3sqyx7`), wrap `2026-11-24T11-08-28Z-wrap`. The record goes pending →
  GATED. `viola adopt` is built and proven over the fakes. R5 has not run. v2-13 stays claimed GATED: its gated
  entry is deferred and not flipped.
- Still GATED, STANDING:
  - «`viola focus`» (`chk-fw7hhw4r`): R3;
  - «`viola up`» (`chk-eq4ejvc6`): R4a;
  - this chunk: R5 (carry `cry-z6ucncyv`, true as written).
  All three wait on the operator's evening at the host.
- Next: «Switching a session's model and effort» (`chk-ke32dhty`, v2-30), named, not started (delegate overseer,
  `inputs#I8`).

## Work done
- P1: the close landed on `report` (tx 4699–4700). Directive `inputs#I6`. Scope clean. The CI fold was closed by
  measurement: 11.520 s and 14.588 s of 80 s under coverage (ci#45378879061).
- P2: 92 row operations landed over 7 masters (tx 4704–4713 and the cascade's three). 45 duplicate adds were
  rejected (check 2), 2 not_master and 2 stands (fit). 76 leaf findings were applied. Two escalations were answered
  by the delegate overseer, PROVISIONAL (`inputs#I7`):
  - Q1, W2: the typed id onto the argv, as an except on `decision/session-id-argv-closed-shape`;
  - Q2: adopt's start inherits run's trust step (the trust-flag write and the kept `CLAUDE_CONFIG_DIR` read), set on
    the security-plan, architecture and obs-plan rows.
- P3: T2 security.md, one entry: a verb that starts through `run::start` inherits its effects.

## Drift resolved
- The report missed adopt's trust step. It was found by reading the named lists against the code (directive item
  3), and 21 rows were amended.
- Disproved plan claims: an upper-case id is accepted (architecture `contract/cli-verbs/adopt` now says either-case
  hex). The fake receipt holds no argv (test-plan `decision/child-argv-read-from-proc`).

## Notes
- **Founder forks still owed:**
  - W1–W4 of this chunk, each on its conservative arm by the delegate's word;
  - W5 (new: adopt's start writes the Claude trust flag; arm 1 landed, arm 2 would be code for a later entry);
  - Q1 and Q2 of this wrap;
  - W2 of the «`viola up`» wrap.
  All are in `.andromeda/runs/2026-11-24T11-08-28Z-wrap/founder-forks.md` and the phase run's.
- **Deferred learnings** (carried from the «`viola up`» wrap): 2 learnings analyzed but not applied:
  - recurrence-despite-learning: CLAUDE.md «A timestamp written into any record … is read from `date -u` at the moment
    it is written»;
  - recurrence-despite-learning: testing.md 2026-09-24 «wait on the exact line asserted».
  Review with `/andromeda-wrap-session --review` if any should be applied.
- **Carries:** `cry-z6ucncyv` (R5), `cry-tzxxzdkd` (R4) and `cry-fqzcimun` (R3) stand. `cry-jeokvkdk` stays on the
  fleet entry.
- **Desk (outside the tree, to remove by name):**
  - the rehearsal base `$TMPDIR/tmp.U4WKIVmjt0/r5`;
  - `$TMPDIR/tmp.6JQv5Ve6YK`, `$TMPDIR/tmp.meET73vc55` (empty);
  - the `*.lock` files in `$XDG_RUNTIME_DIR/viola/`;
  - the R2 worktree `~/dev/projects/viola-mutants-scratch/r2` and `$XDG_RUNTIME_DIR/viola-r2/`;
  - every earlier desk item as before.
- **Last failed command:** none.
- **Host:** PATH's claude is 2.1.330. Read `hostwatch.py read` before any stall red.

## Session End Status
Completed normally at 2026-11-24 14:14:57
