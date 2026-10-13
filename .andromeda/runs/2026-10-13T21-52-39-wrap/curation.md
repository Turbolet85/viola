# Curation — 2026-10-13-paste-newline-ledger-row

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A probe for count literals a move leaves stale matches the retired shape, never a bare `N pass`"
  Tier 3 (.claude/docs/session-learnings.md): + "A sweep pattern file refuses an escaped apostrophe and a late inline flag"
  Filters: 0 dup · 4 task-specific · 0 conflict · 0 deferred (cap) · 2 below threshold · 1 recurrence → handoff
  No-other-home: "A probe for count literals a move leaves stale matches the retired shape" · "A sweep pattern file refuses an escaped apostrophe and a late inline flag"

## Applied
- T2 `.claude/rules/testing.md` — count-probe shape. Confidence 0.8: verified by measurement +0.4, specific
  technical detail +0.2, no other home +0.2 (other signals exactly 0.6; not on the route, not in a master, not
  a playbook or drift-base rule).
  Proof: plan entry 12, the literal probe, read `last line '1'` on a correct tree. The one hit,
  `tests/cli_verify.rs:1093` `"stamped 2.1.0  17 pass  1 fail"`, is the eighteen-row oracle (implement run dir's
  gate trail, entry 12 firings 2–3; report §Spec claims disproved).
- T3 `.claude/docs/session-learnings.md` — the pattern-file traps. Confidence 0.8: a real tool refusal +0.4,
  specific detail +0.2, no other home +0.2.
  Proof: `cascade.py sweep` exit 2 three times this wrap. `report-patterns.toml`: "not TOML — Expected newline or
  end of document … (at line 7, column 70)" (a doubled `''`), then "framing-variants: the regex does not compile —
  global flags not at the start of the expression at position 18". `cascade-patterns.toml`: "variants-two: … at
  position 28".

## Every Decisions & corrections entry, dispositioned
- The founder's cap kept to the letter — not a learning: a chunk decision under its own snapped word (`inputs#I5`).
- The operator's word on entries 12 and 13 — not a learning: task-specific, carried by the report's Deviations.
- The operator's word on the route (mint "Owed ledger rows") — not a learning: P5 lands it.
- Sweep hazard, a count literal can be a correct new count — **applied** (T2 `testing.md`, above).
- Sweep hazard, a row count pinned by shape as well as by literal — rejected as a new entry. It is a
  **recurrence-despite-learning** of `testing.md` 2026-10-07 ("When a compiled bound moves, a sweep keyed on the
  bound's own values misses the cases … list the cases by the function the bound decides, never by the literals"),
  logged to the handoff's Deferred learnings. Research swept literals and missed five readers of `LedgerRow::ALL`
  and of the fixture set.
- Plan defect class (a preservation guard excluding only research's lists vs the mandatory recorded companion) —
  not a project learning: a pipeline defect (phase's plan guard), recorded as friction
  `contract.jointly-contradictory-instructions` for the pipeline's owners.
- Hazard, a hand-applied mutant of a path accessor can write into the repository — rejected, Filter 4: 0.4
  (specific detail +0.2, implicit-behaviour paraphrase −0.2 offset by the "never" design +0.3, one-off −0.3, task
  specific −0.2 → below 0.6). It was reasoned, not measured: no gate failed on it.
- A mutant that turns a loop's exit condition false spins its other case — rejected, Filter 4: below 0.6. It is a
  one-off, and close to `testing.md` 2026-09-24 (a test wait a mutant can reach).
- Two Bash-guard refusals (`cd` into a subdir; a `cat` heredoc with a file target) — not a learning. Both rules
  stand in `host-linux.md`; each guard refusal cost its one re-issue (`guard-refused`), no recurrence.
- Hygiene caught a temp-dir spelling — not a learning: the run-dir spelling rule stands (inputs-contract §snap),
  and the gate did its job.
- Standing rules kept — not a learning.
- Task-specific rejections (Filter 2): the founder's cap, entries 12/13, the route mint and the standing-rules
  line — 4.

CLAUDE.md size: see the P7 report (health check 1, fired after the writes).
