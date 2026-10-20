CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   testing.md: the 2026-09-25 remove-the-guard entry extended
  Tier 3 (.claude/docs/session-learnings.md): + "A live round's census entry sits right after its own leg" · + "A site-sweep pattern on `stamped` reads identifiers, not claims"
  Filters: 3 dup (2 of them recurrences → handoff) · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold · 2 home exists · 4 not a learning
  No-other-home: "A live round's census entry sits right after its own leg" · "A site-sweep pattern on `stamped` reads identifiers, not claims" · "a classifier-refused remove-the-guard control runs over a scratch clone"
  Extended: T2/testing.md: "2026-09-25: Every new guard test carries its remove-the-guard run…" + "one the permission classifier refuses in the tree runs over a scratch clone of HEAD"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Applied
- **T2 extension, testing.md (the remove-the-guard entry), confidence 0.8.** Signals: the operator's wrap directive
  (`inputs#I8` item 3) +0.4, measured +0.4. Nothing else carries it, though the conditional +0.2 was not needed.
  Proof: implement's in-tree control was refused (`[Security Test Removal]`). The clone form
  (`gate.py run --entry 8 --project-root {clone}`, its target dir on disk) read 8 passed and 2 failed, exactly the
  two cases resting on the literal (`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/evidence/remove-the-guard.md`).
- **T3, "A live round's census entry sits right after its own leg", confidence 0.8.** Signals: measured +0.4,
  specific detail +0.2, no other durable home +0.2 (only the friction log, which is telemetry). Proof: entry 17 of
  the chunk's plan was unreachable through `--live-legs --entry` without leg 18 (option 2, not taken); its loop ran
  by hand (`evidence/live-round.md`).
- **T3, "A site-sweep pattern on `stamped` reads identifiers, not claims", confidence 0.8.** Signals: measured +0.4,
  specific detail +0.2, no other durable home +0.2. Proof: the pre-pass F3 pattern gave 80 master rows, 77 of them
  true claims sharing the token (`cascade-2026-10-19-newer-claude-cli-rows-pre-pass.json`). The detectors found
  three restatements the pattern missed (`fanout-results.md`, architecture 5 and test-plan 7, 8).

## Each Decisions & corrections entry
- The founder's own answers (`inputs#I6`) — not a learning: a ruling, recorded in the sidecars and inputs.
- The operator's implement invocation (`inputs#I7`) — not a learning: a one-run direction.
- The overseer's wrap directive (`inputs#I8`) — not a learning as a whole; its item 3 is the source of the applied
  T2 extension.
- Guard refusal, the classifier, in-tree remove-the-guard — applied (the T2 extension).
- Guard refusal, recursive `rm` of the clone and its target — rejected, duplicate: host-linux.md 2026-10-07 already
  says a refused removal is never done another way and a scratch home moves into `target/e2e-home.disk/`, which
  this wrap did.
- Guard refusal, a `cd` into `references/` — rejected, duplicate. It is a recurrence of host-linux.md §Paths →
  handoff, `recurrence-despite-learning`.
- Guard refusal, a census piped into `head -0` — rejected, duplicate. It is a recurrence of the carried "a pipeline
  tool's stdout" learning → handoff, `recurrence-despite-learning`.
- Hook refusal, an evolve `ts` typed ahead of the clock — not a learning: the tool already refuses it, and its letter
  says to take `ts` from `date -u`.
- Measured fact, the resting footer's mode word follows the permission mode — rejected, home exists: amended into
  architecture [Screen Model] at this wrap's P2.
- Measured fact, the round census between two legs — applied (T3).
- Measured fact, a detached tmux pane reads CLI rows without a key — rejected at confidence 0.6 (measured +0.4,
  specific +0.2, exactly at the threshold). Its other home is the chunk's committed `footer-reading.sh`.
- Sweep hazard, `stamped` — applied (T3).
- The `/tmp` quota (EDQUOT) mid-wrap — rejected, home exists: the handoff's Headroom note and «Epoch 5 cleanup»
  item 9 on the route.
