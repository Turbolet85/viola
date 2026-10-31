CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings): none
  Tier 2 (.claude/rules/*): + testing.md: "A case that runs on every OS must not call a helper gated off one of them"
  Proof: `ci#41225642322` read red on `lint (macos-latest)` and `test (macos-latest)`, `error[E0425]` at `crates/viola-mcp/tests/handlers.rs:539`; the recipe reads exit 0 after the fix (`evidence/operator-pass.md`). Score 0.8: measurement 0.4, technical detail 0.2, no other home 0.2 (test-plan's sampler row on it was rejected at P2 and routed here; `inputs#I5` item 5).
  Tier 3 (.claude/docs/session-learnings.md): + "A count sweep misses a count written in words with a noun phrase after it"
  Proof: the report's `kill-count` row against `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`, caught by `fanout-test-plan.md` 4. Score 0.8: measurement 0.4, technical detail 0.2, no other home 0.2.
  Filters: 3 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "A case that runs on every OS must not call a helper gated off one of them" · "A count sweep misses a count written in words with a noun phrase after it"

The report's Decisions & corrections, each entry's disposition:
- The directive to this wrap, `inputs#I5`: not a learning, a one-wrap direction.
- The founder's own words, `inputs#I2` Q1 and Q4: not a learning, amended into the masters and their sidecars.
- The delegate's words, `inputs#I2` Q2, Q3, `inputs#I3`: not a learning, carried PROVISIONAL on the handoff.
- The operator's directive at implement, `inputs#I4`: rejected, duplicate of the testing.md entry of 2026-10-27 on kill deadlines and of the operator pass's standing form.
- Sweep hazard, a helper's own `cfg` gate: applied, Tier 2.
- Sweep hazard, an oracle moved by a shared body hides a test that never reached its subject: rejected at Filter 4, exact 0.6 (measurement 0.4, technical detail 0.2); P2 amended it into `integration-test-strategy.md`, so no no-other-home signal.
- Correction, implement's own wrong `strict-modes-failed` line: rejected, task-specific, a one-off fixed on its first read.
- Recurrence of the timestamp learning: rejected, duplicate of a defect entry, logged `recurrence-despite-learning: CLAUDE.md, timestamps from date -u` on the handoff, already carried.
- Guard refusal, the `cat` heredoc and the `rec.py` pipe: rejected, duplicate; the hook refuses the act, and host-linux.md holds it.
- Hygiene, two phase files' temp path respelled: rejected, task-specific; the tool's own refusal states it.
- A P5 slip, a stray `rm -f /dev/null`: rejected, task-specific, a one-off with no rule in it.
- A difference left untraced at phase, yes 497 against 496: not a learning, a tool reading already on the phase's card.
- The operator pass's fix commit: not a learning, the pass's record.

From this window:
- `kill-count` missed a spelled-out count: applied, Tier 3.
- A writer's chunk marker passing `[NO_PROVENANCE]` and failing classify's read-back: not a learning for this project, telemetry of the tool; its friction record stands.
- The directive's check 7 against the installed letter: not a learning, the operator withdrew it (`inputs#I6`).

CLAUDE.md size: read at P7.
