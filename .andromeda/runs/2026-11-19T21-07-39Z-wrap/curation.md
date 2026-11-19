# Curation — 2026-11-19-context-as-a-local-command (wrap 2026-11-19T21-07-39Z)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "When a counted set grows, a retired-shape probe for its old figure … misses a bare count asserted over a derived set …" · + testing.md: "`tests/contract_verify_home.rs` reads any `\"verify\"` string literal in a test source as a verify start …"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred
  No-other-home: "a retired-shape probe misses a bare count over a derived set" · "contract_verify_home reads any verify literal as a verify start"
  CLAUDE.md size: 135/200 · T1 4.2 KB, 0 over 600 B

## Applied
- testing.md — the derived-set count hazard (confidence 0.8: real gate failure +0.4 · specific technical detail +0.2 · reached no other durable home +0.2).
  Proof: implement's first firing of entries 3 and 4 read three stamp tests red on bare counts over derived sets (`rows.len(), 25` relied + retired; `results.len(), 23` a pre-chunk subset) that the retired-shape probe had not listed; the report's Decisions & corrections, «Sweep hazard: the retired-shape probe …», and the outcome file's reds-before-green line. Dedup: session-learnings.md 2026-10-31 (a word-form count with a noun after it) and 2026-10-16 (the summary literals masters quote) are other facets; no entry names a count asserted over a derived set.
- testing.md — the `contract_verify_home` literal hazard (confidence 0.8: real gate failure +0.4 · specific technical detail +0.2 · reached no other durable home +0.2).
  Proof: implement's entry 27 (`run --e2e`) read red once — `contract_verify_home` took a rehearsal's writer-field literal as a verify start — and went green after the assertion was rewritten without the literal (report, Decisions & corrections; outcome file, reds before green). No entry or master names it.

## Every Decisions & corrections entry, dispositioned
Implement's:
- The operator's word (inputs#I4) governed the round, hostwatch before each start — not a learning: this chunk's run of a standing rule (the round recipe), rejected task-specific (Filter 2).
- The relay on the overseer's hand read of the credentials token window — not a builder learning: the overseer's own conduct, its correction of record carried in the close; the rule it departs from stands (verification-harness.md 2026-11-18) — rejected duplicate (Filter 1).
- Sweep hazard: retired-shape probe vs a bare count over a derived set — applied (above).
- Sweep hazard: `contract_verify_home` and a `"verify"` literal — applied (above).
- The fake agent writes `--transcript` files beside its own exe — not curated: this wrap landed it in a master (test-plan `decision/private-fake-for-no-transcript`, tx 3195) and the cascade carried it into testing.md and verification-harness.md — rejected duplicate (Filter 1, a master home).
- Guard refusals, each re-issued once — not a learning: a guard's refusal that cost its one re-issue is telemetry (evolve `guard-refused`), rejected task-specific.
- The `code` evolve playbook read early — not a learning: friction telemetry, never curated.
The close's (`## After implement`):
- The wrap directive's away form — not a learning: this wrap's standing procedure (the skill's own letter).
- The overseer's correction of record (the credentials read) — not a builder learning: named in the report as a note, as the directive asks; no row or rule changes for it.
- The answer card's four founder answers — landed in the masters with their sidecar entries (P2), not curation.
- Item 6's except wording — landed in the masters (P2).
- Leaves hold no figure the code states — not a new learning: the curation guide and Filter 2 already state it; applied in this wrap's cascade (8 leaf findings held and hand-written without their counts).
- The operator pass: no correction or ruling — nothing to weigh.
