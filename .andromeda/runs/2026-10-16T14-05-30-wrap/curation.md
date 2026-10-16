# Curation — 2026-10-14-resume-and-compact-ledger-rows

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none new)
  Tier 2 (.claude/rules/*):                   + testing.md: "a test fixture's synthetic session id has the real session-id shape"
  Tier 3 (.claude/docs/session-learnings.md): (none new)
  Filters: 2 dup (→ 1 recurrence logged to the handoff) · 0 task-specific · 0 conflict · 0 deferred (cap held at 3)
  Extended: T1/CLAUDE.md: "A count written into a master carries the rule that produced it" + "a master states no test-case count at all"
  Extended: T3/session-learnings.md: "gate.py hygiene reads `/home/<x>/` in prose" + "a temp-dir glob in prose, and a `<` token passes"
  No-other-home: "a test fixture's synthetic session id has the real session-id shape"
  No-other-home: "a temp-dir glob in prose is read as a host path; a `<` token passes"
  CLAUDE.md size: 128/200 · T1 2.8 KB, 0 over 600 B
```

The tier counts count WRITES: T1 1 (an extension) · T2 1 (new) · T3 1 (an extension).

## Applied
- **T1, extended** — the CLAUDE.md count entry gains "a master states no test-case count at all — it names the
  functions, and the code's `#[case]` rows are the count".
  - Proof: `inputs#I7` (`relay-3.md`), the overseer's founder-delegated word.
  - The measurement: test-plan §4's "eight functions, 15 cases" had no recoverable rule. By `#[case]` rows the count
    read 14 at commit 2861e19, 24 at 880b0563 and 32 at this chunk (`src/run/send/classify.rs`).
  - Score 0.8: an explicit operator correction +0.4; verified by measurement +0.4.
- **T2, new** — `testing.md`: "a test fixture's synthetic session id has the real session-id shape".
  - Proof: the stopped implement run's journal, step 9.
  - The measurement: under `tests/support/verify.rs`'s `s-verify-1` Run E never started, because `resume_target`
    drops an off-shape id, and the row read `fail` with nothing louder. The literal became `SESSION_ID`, which has the
    shape.
  - Score 0.8: verified by measurement +0.4 (it changed the chunk's tests); specific technical detail +0.2;
    no-other-home +0.2. No master, route entry or rule carries it.
- **T3, extended** — `session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose…" gains the temp-dir glob
  facet.
  - Proof: `evidence/operator-pass.md`. The first `gate.py hygiene` read refused 3 files (`P1 … · tmp`), each a prose
    `/tmp/viola-verify-<glob>`. Rewritten as `/tmp/viola-verify-<suffix>`, the files read clean.
  - Score 0.8: a real gate refusal +0.4; specific technical detail +0.2; no-other-home +0.2.

## Every Decisions & corrections entry, weighed
1. The founder's STOP for an account switch, and the resume → not a learning (an event of the run).
2. The overseer's `inputs#I3` (re-plan the guard) → not a learning (a decision for this chunk).
3. The operator's `inputs#I4` (the plan revision) → not a learning (a decision for this chunk).
4. The operator's `inputs#I5` (the operator pass by the session) → not a learning (a decision for this chunk).
5. The operator's wrap invocation word (context alarms; re-read from disk) → not a learning (procedural for this
   window).
6. The journal time written without reading the clock → rejected, Filter 4. A one-off self-correction, -0.3. The rule
   (a `ts` from `date -u`) already stands in the evolve contract.
7. The recurrence of `testing.md` 2026-10-07 (list the cases by the function the bound decides), with its
   integer-built summary-line facet → Filter 1 dedup, and a recurrence of a defect record. Logged to the handoff as
   `recurrence-despite-learning` (the third), not extended: a further entry is not the remedy.
8. The test-data hazard (an off-shape synthetic session id) → applied (T2, new).
9. The deterministic hold of Run E (a set with no default SessionStart) → rejected, Filter 4. A one-off test design,
   0.2.
10. The hygiene hazard (a prose temp-dir glob) → applied (T3, extended).
11. The sweep hazard (`-plan/` matched the registries' `contracts/{doc}-plan/`) → rejected, Filter 4. Caught by
    reading the rows, no gate failure, 0.2.
12. The guard refusals (a `cd` that moves the cwd, four in this window and two in the stopped run) → Filter 1 dedup
    against `host-linux.md` "Paths & the working directory". Not a recurrence: each refusal cost its one re-issue, so
    the guard held the rule (`guard-refused`).
13. The live measurement (`--resume` across project dirs; Run E adds no transcript) → not curated. It is amended into
    the architecture and security-plan masters this wrap.
14. The disproved obs claim (a killed PTY child's `child_exit_status`) → corrected at its source: a P2 amendment
    (architecture, obs-plan) and its cascade.
15. The case-count ruling (`inputs#I7`) → applied (T1, extended).
