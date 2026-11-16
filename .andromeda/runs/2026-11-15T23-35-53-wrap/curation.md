# Curation — wrap of 2026-11-15-api-error-turn-end

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A shared case table whose assertions pin one row's literal (an event name such as `"stop"`) breaks the moment a row of another subject joins it: read the subject from the case's own arguments, never a literal of the first row."
    Proof: the first gate block of implement run 2026-11-15T22-51-16-implement read red on `tests/hook_fail_open.rs` and pre-push: the spine-bound case's two `hook_event` asserts pinned `"stop"`, broken by the new `stop_failure_unreachable` row; fixed by reading `args[1]` (report §Deviations, §Decisions & corrections). Score: verified by measurement +0.4 · specific technical detail +0.2 · no other home this wrap +0.2 = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "a shared case table pinning one row's literal breaks when another subject's row joins"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Each *Decisions & corrections* entry, dispositioned
Implement's:
- The operator's run directive (no live start, no mutation, journal long runs) — not a learning: one run's directive.
- Guard refusals (two `cat` heredocs, one `cd`) — not a learning: the standing `host-linux.md` heredoc and cwd rules,
  each refusal re-issued once; a hook already blocks the act.
- Sweep hazard (a synthetic-only fake-agent fixtures root draws no input box, the send reads `input-not-ready`) —
  rejected, Filter 1 (dup): this wrap landed it into test-plan `decision/send-cases-copy-recorded-fixtures` and the
  cascade wrote it into `testing.md`'s generated body.
- Case-table hazard — applied, Tier 2 `testing.md` (above).
`## After implement`'s:
- The CI red ruled not this chunk's, pinned on «`wait` wake condition» — not a learning: the route's CARRY (P5).
- F1 / F2 PROVISIONAL, the away form — not a learning: a ruling on this chunk, carried on the handoff.
- The desk list's additions — not a learning: the handoff's.
- The operator pass stopped at the red with its cause unmeasured — not a learning: the standing «a red stays open
  until its cause is known» rule applied.
This window: no operator correction; the guard refused two `cd` calls (each re-issued once) — not a learning.
