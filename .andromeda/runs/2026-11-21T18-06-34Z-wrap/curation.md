# Curation — 2026-11-21-cross-session-prefix-ledger-row (wrap 2026-11-21T18-06-34Z)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A change to the fake agent's receipt shape reads red only in the whole integration layer …"
                                              + testing.md: "A test helper that rebuilds a `Command` from `get_program()` and `get_args()` drops every env the original set …"
                                              + security.md: "A promise that a value reaches no fixture covers only what viola writes itself …"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: the three applied entries (each 0.6 + 0.2)
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Applied
- testing.md — the receipt key pin. Confidence 0.8: measured +0.4 (the after-landing `run --integration` red,
  `fake_agent_receipt_env_holds_names_never_values`, report Decisions & corrections), specific detail +0.2, no other
  home +0.2 (no master row, route annotation or rule holds it).
  Proof: report Decisions & corrections «Sweep hazard: a fake-agent receipt key pin …»; implement outcome entry 40.
- testing.md — the rebuilt `Command`'s env. Confidence 0.8: measured +0.4 (`pre-push` red, 2 of 3951: the VH and AV
  rehearsals `cross-session: not-run`), specific detail +0.2, no other home +0.2.
  Proof: report Decisions & corrections «Sweep hazard: a test helper that rebuilds a `Command` …»; evidence/operator-pass.md
  §pre-push. Tier 2 by the registry: test authoring → testing.md (paths cover `crates/viola-e2e/tests/`).
- security.md — an upstream CLI's echo into a recorded payload. Confidence 0.8: measured +0.4 (this wrap read the
  committed `UserPromptSubmit.cross-session-1.json` and `ledger::scrub` / `has_absolute_path`: the runtime-dir path and
  pid pass the scrub), specific detail +0.2, no other home +0.2 (the masters now state the scrub's reach, not the act of
  checking a CLI's echo before a promise). security.md has no `paths:`: judged at Tier 1's bar, one sentence, 246 B.
  Proof: After implement, Decisions & corrections after implement, the measured fact; founder-forks.md item 1.

## Every Decisions & corrections entry, dispositioned
Implement's:
- «A new env read in verify reaches every fake-agent stamp …» — not curated: landed as the test-plan row
  `decision/stamping-verify-test-owned-runtime-dir` (TX 3699) and re-derived into testing.md, verification-harness.md,
  tests-summary.md and commands.md by the cascade (Filter 1: the generated body holds it).
- «Sweep hazard: a fake-agent receipt key pin …» — applied (testing.md).
- «A fake-agent side process that runs plugin hooks adds hook receipt lines …» — not curated: landed in the test-plan
  rows `decision/fixture-drift-byte-compare` and `decision/fake-agent-matches-recordings` (TX 3702), re-derived into
  verification-harness.md and tests-summary.md.
- «Sweep hazard: a test helper that rebuilds a `Command` …» — applied (testing.md).
- «Standing learning met: a whole re-record moves the recorded session ids» — not a learning: the testing.md
  2026-11-21 entry prevented its recurrence (the ids read before the copy).
- «Guard refusals, each re-issued once» — not a learning: each guard held its rule at the cost of one re-issue.
- «The overseer's word to fire `XS-2` (inputs#I11)» — not a learning: a ruling of the run.
After implement's:
- The wrap directive (inputs#I12): operator away, card and forks — not a learning: this wrap's directive.
- The measured fixture fact — applied as its general form (security.md); the fact itself is founder-forks.md item 1.
- The carries — not a learning: route-resolve's (P5).
