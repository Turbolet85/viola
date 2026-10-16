# Curation — 2026-10-16-unwrapped-sessions-on-the-board

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + events.md: "a snapshot field a reader expects once an event line is visible is written before that line" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A moved relied-row count stales the masters' quoted summary literals too" (confidence 0.8)
  Filters: 1 dup · 4 task-specific / below threshold · 0 conflict · 0 deferred
  No-other-home: "a snapshot field is written before its event line" · "a relied-count move stales the quoted summary literals"
  Extended: T2/testing.md: "2026-10-07: When a compiled bound moves…" + "a count a shared test-support helper pins moves for every binary calling the helper; the gate filter selects them by the helper's callers" (confidence 0.8)
```
Tier counts (writes): T1 0 · T2 2 (one new, one extension) · T3 1.

## Applied
- T2 `events.md` — write the snapshot field before its event line.
  Proof: report Deviations 4 — written after the `session-start` line, `agent_session_id` raced `cli_list`'s `path1` and `e5` readers (both red); moved before the line, green. Scores: measurement +0.4, technical detail +0.2, no other home +0.2 (no master states the ordering; the architecture amendments name the field only).
- T2 `testing.md` 2026-10-07 entry, extended — the gate filter selects every caller of a helper that pins a moved count.
  Proof: report Deviations 11 — the plan's integration filter did not select `cli_answer`, whose `tests/support/home.rs` `dialogless_home` pinned `18 pass  4 fail`; only `pre-push` found it red (now `19 pass  4 fail`). Scores: measurement +0.4, detail +0.2, no other home +0.2. A facet of the 2026-10-07 entry (it names the shared-helper surface the entry lacks); the handoff's carried `recurrence-despite-learning: testing.md 2026-10-07` note gains this as its fourth recurrence.
- T3 `session-learnings.md` — a relied-count move stales the masters' quoted summary literals.
  Proof: this wrap's `cascade-dispositions.md` — the step-2 sweep's `counts-old` pattern found `test-plan.md:586`'s `18 pass  4 fail` / `20 pass  2 fail` literals stale, missed by every detector and by the P1 site sweep; folded in from `tests/cli_verify.rs`. Scores: measurement +0.4, detail +0.2, no other home +0.2. Tier 3: the artifact (the masters) has no rule file.

## Every *Decisions & corrections* entry
- The founder's four P4 answers — not a learning: a ruling, written into the masters and their sidecars this wrap.
- The operator's P5 card yes — not a learning: a ruling; its placements go to P5.
- A fake agent's registry row must replay recorded values — rejected, Filter 4 (0.6: measurement +0.4, detail +0.2; exactly 0.6 rejects; the fact is amended into architecture §Occupied Resources this wrap, so no conditional signal).
- One process-table read per listing — rejected, Filter 4 (0.4: its cause inferred, not read, −0.2).
- A snapshot field written after its event line raced readers — applied (T2 `events.md`).
- A verify probe placed after a chain of pastes inherited their failures — rejected, Filter 4 (0.6; the placement is amended into architecture [CLI Version Compatibility] and security-plan this wrap).
- A proptest oracle against the pre-serialisation value failed on a float — rejected, Filter 4 (0.6; amended into test-plan §6 this wrap).
- A chunk's integration filter missed a binary whose shared support pinned the changed count — extended (T2 `testing.md`).
- The unexplained `cli_send` red — not a learning: an open red, its owner the WATCH pin P5 places.
- The leftover dirs — not a learning: the founder's desk list (P6, handoff).
- Guard refusals (`cat` heredoc, `cd`, recursive `rm`, a pipe into a filter) — rejected, Filter 1 (`host-linux.md` and the handoff's standing rules carry each; the hook refuses the act itself).
- A matrix field path probed before the contract's schema was read — rejected, Filter 4 (one-off, −0.3).
- The P1 site sweep's three unfired controls — not a learning: the tool's designed refusal, telemetry.
- Standing learnings met (implement) — not candidates: existing entries that held.
- (This window) the cascade sweep's stale summary literals — applied (T3).
