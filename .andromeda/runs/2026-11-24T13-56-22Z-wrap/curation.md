CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "`tests/contract_error_bodies.rs` holds two rows of `code: "unknown"` (CLI and MCP) with the same detail line, so an edit anchored on the code and the detail alone matches both: anchor on the row's `exit` line too." (confidence 0.8)
    Proof: implement's report, Decisions & corrections, the sweep hazard (two `code: "unknown"` rows, CLI and MCP, one detail line) met while editing `CLI_DOCUMENTS` (+3 rows, `tests/contract_error_bodies.rs`); signals: verified by measurement +0.4 · specific technical detail +0.2 · no other home +0.2.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 not a learning · 0 task-specific · 0 conflict · 1 deferred (→ handoff, recurrence-despite-learning)
  No-other-home: "contract_error_bodies.rs code/detail anchor hazard"; "an unscripted booted wrapper refuses a send"
  Extended: T2/testing.md: "2026-10-17: A harness-session case never assumes the fake agent's defaults …" + "an unscripted booted wrapper also refuses a send `not-delivered`, so a case that needs a running turn boots over a gated script" (confidence 0.8)
    Proof: implement's report, Decisions & corrections and Deviations: `switch_mid_turn_is_refused_turn_running` boots over `fixtures/fake-scripts/path3.json` because an unscripted booted wrapper refused the setup send `not-delivered`; signals: verified by measurement +0.4 · specific technical detail +0.2 · no other home +0.2.
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Dispositions of every Decisions & corrections entry:
- implement: «the implement directive (inputs#I5): the PTY core lands as its own layer first» — not a learning: a directive for this chunk, held and recorded (task-specific to the chunk's order).
- implement: «recurrence of CLAUDE.md's timestamp learning: two record stamps typed estimated» — rejected by Filter 1 against the CLAUDE.md entry «A timestamp written into any record … is read from `date -u`», a DEFECT record → recurrence-despite-learning, deferred to the handoff.
- implement: «guard refusal: `rec.py query … | cut`» — not a learning: the bash guard refuses the act itself (its one re-issue); this wrap met the same guard once on `rows-land | grep`, re-issued bare.
- implement: «sweep hazard: two `code: "unknown"` rows in `tests/contract_error_bodies.rs`» — applied, T2 testing.md.
- implement: «a booted wrapper with no script refuses a send; a case needing a running turn boots over a gated script» — extended, T2 testing.md (the 2026-10-17 entry).
- after implement: the wrap directive (inputs#I6), whose forks stand on a delegate's word — not a learning: the chunk's own provenance, carried in the sidecar entries and the handoff's founder forks.
- after implement: the pass record's corrections or rulings — none.
- this wrap: the leaf comparer's figure (a11y-summary.md:53#1) held under the standing learning «a leaf names a time bound by its constant» — not a learning: the entry worked (no recurrence).
- this wrap: one fact proposed by up to five proposers, kept in one home by check 2 — not a learning: the pipeline's own behaviour (telemetry, not curated).
