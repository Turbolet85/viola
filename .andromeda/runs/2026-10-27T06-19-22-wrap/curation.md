# Curation — 2026-10-27-bounded-inputs-at-every-boundary

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A timestamp written into any record … is read from `date -u` at the moment it is written, never estimated or carried forward …" (confidence 0.7)
  Tier 2 (.claude/rules/*):                   + testing.md: "Every test or control that spawns `viola run`, `viola mcp` or a wrapper owns its child and kills it at a stated deadline …" (confidence 0.9)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred · 9 below the threshold or homed elsewhere (below)
  CLAUDE.md size: 129/200 · T1 3.4 KB, 0 over 600 B (`health.py check`, check 1 row, after the writes)

## Applied
- T1 — the estimated-timestamp hazard.
  Proof: three events this chunk, each a hook refusal — implement's evidence stamp written +9 min ahead, implement's
  script-written stamp +3 min ahead (no hook fired; a clock read caught it), and this wrap's own report `Date` written
  14 min ahead, refused by the stamp-ahead hook and then refused again on a line quoting the estimate (report
  Decisions & corrections; `outcome-…md` Decisions & corrections). Signals: repeated 3+ (+0.3), verified by a real
  refusal (+0.4) = 0.7. Tier 1: it binds every record on every path (tiebreaker 3: spans every pattern), one
  sentence, 263 B.
- T2 testing.md — the spawn kill deadline.
  Proof: the operator's direction for the implement run ("every test or control that spawns `viola run`, `viola mcp`
  or a wrapper gets a kill deadline"), applied as the serve child's 5 s and the `mcp_tools` child's 8 s kill and the
  controls under `timeout`; the step-2 control killed the child case at its 5 s bound (`evidence/controls.md`).
  Signals: the operator's explicit direction (+0.4), "every" language (+0.3), specific technical detail (+0.2) = 0.9.
  Tier 2 by the home registry (test authoring → testing.md, whose `paths:` cover `tests/**` and the crates' tests).

## Decisions & corrections — every entry's disposition
1. The operator's implement direction (kill deadlines · long runs journalled before waited on · listings read whole) —
   applied (kill deadlines, T2); the journal clause is the implement skill's process, not a project learning; "listings
   read whole" rejected — Filter 1 duplicate of host-linux.md 2026-10-27.
2. rmcp's input bound belongs to the reader viola hands it, never the codec — not curated: amended into architecture
   [MCP], security-plan §Input Validation and the leaves this wrap (its master is its home); 0.6 (measured +0.4, detail
   +0.2), exact-0.6 rejects and the conditional signals do not fire (a master holds it).
3. An over-long MCP line refused whole, never truncated or skipped — not curated: a master's decision (architecture
   [MCP], security-plan Anti-Patterns), amended this wrap.
4. The cut signalled as end of stream, never an I/O error — not curated: architecture [MCP] detail, amended this wrap.
5. No Windows byte oracle for the wide character under ConPTY — not curated: owned by the route entry «Windows-only
   live measurements» (the P5 card's CARRY) and an OPEN in architecture [PTY].
6. The census hazard (a statement-scoped `take(` cannot see an exception's argument past the call's `(`) — not
   curated: the census rule is written into test-plan §6's detail this wrap (its home); 0.4 alone.
7. Guard refusals (bash-guard `cd` arm, one re-issue each, implement and new-session) — not a learning: the guard put
   each call back on the documented path.
8. The stamp-ahead hook refusals — applied (T1, above).
9. The `andromeda:walks-tree` token in the census test — not a learning: a one-off of the pipeline's census mechanism.
10. The phase's P2 (design-cascade-g1's ruled re-spawn folding five sibling items) — not a project learning: pipeline
    telemetry, its friction records the phase's.
11. The P5 approval PROVISIONAL (delegate overseer1) — not a learning: a ratification status, carried on the handoff.
12. The wrap directive (check 7 escalates as written; the last wrap's rows no precedent) — not a project learning: an
    operator rule for the pipeline, applied at this wrap's P2 halt (inputs#I5).
13. The second stamp-ahead refusal at this wrap — folded into 8.
14. python `yaml` absent on the Linux dev host — rejected (Filter 4: one-off, read by eye at check 12; 0.4 − 0.3).
