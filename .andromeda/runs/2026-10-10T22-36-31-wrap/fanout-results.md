# Fan-out results — wrap of 2026-10-10-budget-governor

Seven detectors, one batch, each on the report (`viola-0.1.0/chunks/2026-10-10-budget-governor/report.md`) and its
own master. Detector counts in the prompts: architecture 2, security-plan 3, design-system 1, layout-templates 1,
test-plan 3, obs-plan 3, a11y-plan 2 (15, the drift-base's 15 `doc:` names). Every return decoded clean (no entity
in any value).

Each row is one proposal as parsed: detector · severity as returned · section · the change in one line. After `→`
its disposition and the check that decided it. The applied text is re-derived from the report, never pasted from
the change line. "EA n" is entry n of the plan's Expected amendments list; "SCD n" is entry n of the report's Spec
claims disproved.

Coordinate check (Validate's opening rule): every source coordinate a proposal cites is the report's or stands in
its last section. Read against `new-text.md` for the ones the report's bullets do not hold: `src/cmd/release.rs`
63-69; `src/cmd/run.rs` 257-259; `tests/cli_budget.rs` 107-184, 175-177, 179-181, 186-217, 219-246, 248-322,
324-382, 398-429, 431-466, 474-503; `crates/viola-state/tests/state_replay.rs` 277-285, 287-300;
`src/run/budget.rs` 959-987. **Rejected for a source read the report does not carry: 0.**

Order note: the three masters whose detectors returned first (design-system, layout-templates, a11y-plan) and
security-plan were amended while the other detectors were still reading their own masters. No detector read a
master another detector's amendment had changed.

## Verdicts

- architecture: 22 proposals (2 detectors; 1 returned `escalate`).
- security-plan: 6 proposals (1 returned `escalate`); stripped: three notes of sites it did not propose (the
  Driver-originated `release` row; the `MAX_FRAME` reader list; the anti-pattern ban on a config switch).
- design-system: 0 proposals; stripped: a note that EA 14's two lines are wording drift outside its token detector
  (raw twin: `.raw-fanout-design-system.md`).
- layout-templates: 2 proposals.
- test-plan: 21 proposals (3 returned `escalate`).
- obs-plan: 23 proposals; stripped: a note that every "no line" as landed rests on the operator's word.
- a11y-plan: 2 proposals; stripped: a note that `D-a11y-obs-schema` holds (neither schema changed).

Total 76.

## architecture

- A1 · D-arch-resources · warning · §Standard Contracts, the `release` method · result gains `override_until?`;
  `budget:true` lifts the gate; a plain `release` answers the gate's recorded state → apply · check 1, "Accurate
  this-chunk addition"; EA 2.
- A2 · D-arch-resources · warning · §Standard Contracts, "answered only after the move's `wheel` record" · a
  `budget:true` release is answered after its `budget-gate` line and the snapshot land → apply · check 1, dependent
  of A1.
- A3 · D-arch-resources · warning · §Standard Contracts, the `send` method · the gate read at arrival; a change that
  cannot be recorded is `-32603` → apply · check 1.
- A4 · D-arch-resources · warning · §Occupied Resources, `config.json` · the two keys, the fallback, one line per bad
  key, read once at start → apply · check 1; EA 3.
- A5 · D-arch-resources · warning · §Occupied Resources, `budget.json` · the reader `read_budget` and its order; the
  refusal resting on it NOT ratified → **escalated, resolved** · check 1, "Boundary widening" and "what ratifies it":
  the operator's direction at this wrap (`inputs#I10` item 1) is the resolution: written as NOT ratified, the
  founder's word owed, no card confirmed. Applied in that wording; EA 3.
- A6 · D-arch-resources · warning · §Occupied Resources, the instance files, the statusline sentence · a refused
  check leaves no trace → apply · check 6, SCD 4; on the operator's word (`inputs#I10` item 1), the founder not
  having ruled.
- A7 · D-arch-resources · warning · §Standard Contracts, Hook contract, the arm's order · a refused check ends the arm
  before anything is opened → apply · dependent of A6; EA 2.
- A8 · D-arch-resources · warning · §Occupied Resources, the instance files, the `revive` sentences · the check before
  the wrapper log, the owed-order sentence discharged → apply · check 1; EA 3; the two qualifiers written "as
  measured at" their evidence.
- A9 · D-arch-resources · warning · §Conventions, exit 1, the `revive` clause · the strict-modes refusal writes no
  line; three readings → apply · check 6, SCD 5; EA 4. The `--list` parenthesis of the same line re-worded by the
  orchestrator (its "two of them" counted the retired set).
- A10 · D-arch-resources · warning · §Established Decisions [Session Liveness] · "second of its four preflight
  readings" → first of three, the check ahead → apply · dependent of A9 (a site the report's `four readings` search
  did not reach).
- A11 · D-arch-resources · warning · §Infrastructure Patterns → Project directory structure · `revive.rs` line →
  apply · key file edited; dependent of A9.
- A12 · D-arch-resources · warning · §Standard Contracts, Instance snapshot, "its third reading" · the ordinal →
  apply · the reading was read by the orchestrator in `src/cmd/revive.rs` (`preflight`: `read_snapshot_or_replay`
  sits in the session reading, the second of three); written with the reading named.
- A13 · D-arch-resources · warning · §Occupied Resources, CLI surface · exit 1 covers the refused check → apply.
- A14 · D-arch-resources · warning · Project directory structure · `src/run/budget.rs` listed → apply · check 1 (the
  key enumerates each `run/` module; not registry over-reach).
- A15 · D-arch-resources · warning · Project directory structure · `read_budget` beside `write_budget` → apply.
- A16 · D-arch-resources · warning · Project directory structure · `BudgetWindowKind` in `viola-core` → apply.
- A17 · D-arch-resources · warning · §Infrastructure Patterns → Crate dependency direction · serde's use names
  `BudgetWindowKind` → apply · key file edited; no edge changed.
- A18 · D-arch-resources · warning · §Occupied Resources, the root-watch bullet · waits 26 → 28 → apply · check 1,
  "A count in a master that carries no rule": the site states its rule (the pattern). The file count was measured
  by the orchestrator at this wrap (`grep -rn -c 'Instant::now() + WITHIN' tests`: 28 lines in 20 files; the two
  new sites by the base's counts: `tests/cli_budget.rs`, new, and `tests/cli_revive.rs`, 1 → 2). Both sites that
  hold the count amended (this one and the test-plan `run` key, T8).
- A19 · D-arch-decisions · warning · §Established Decisions [Budget Governor] · the gate as landed → apply · check 1;
  EA 1. The orchestrator added the resume point's finding 2 (a snapshot can read paused after the reset until the
  next point) and the fail-open list.
- A20 · D-arch-decisions · warning · §Established Decisions [Session Liveness], the start order · the gate's start
  evaluation between the bind and the first snapshot → apply · check 6, SCD 1 (architecture's own list).
- A21 · D-arch-decisions · escalate · §Cross-cutting Patterns, Capability ledger, "The rows are owed" · the
  `rate_limits` row's owner → **escalated, resolved** · the owner is the operator's direction at this wrap
  (`inputs#I10` item 3): "Paste newline ledger row". The reliance written as landed on the operator's word, the
  founder not having ruled.
- A22 · D-arch-decisions · warning · §Established Decisions, the capability-ledger bullet · the gate's refusal rests
  on the unrowed shape → apply · dependent of A21.

Raised by the orchestrator (check 5): §Occupied Resources, `diagnostics/` — `revive`'s start arm logs as `run` once
its check has passed. Applied.

## security-plan

- S1 · D-security-auth · warning · §Authentication & Authorization, `~/.viola/` access control, the `revive` clause ·
  the refusal writes no line → apply · check 6, SCD 5; EA 5.
- S2 · D-security-auth · warning · same clause · the check before the log, the debt to "Budget governor" retired →
  apply · EA 5; "as measured at" the evidence.
- S3 · D-security-auth · warning · same row, the `hook statusline` clause · a refused instance leaves no trace →
  apply · check 6, SCD 4.
- S4 · D-security-auth · escalate · same row, beside the trusted-file list · `budget.json` off the list, the gate
  refusing a driver on it, NOT ratified → **escalated, resolved** · as A5: `inputs#I10` item 1. No card confirmed.
- S5 · D-security-input · warning · §Input Validation, Configuration values · the threshold rule as landed → apply ·
  EA 6.
- S6 · D-security-input · warning · §Input Validation, Own state files on read · `read_budget`'s order → apply ·
  EA 6.

Raised by the orchestrator from the detector's stripped note (a): §Authentication & Authorization, Driver-originated
`release` — the string-`from` refusal comes before `budget` is acted on (report, Outcome criterion 5). Applied. Notes
(b) and (c): no claim retired, no change.

## design-system

`proposals: []`. EA 14 raised by the orchestrator (check 5): §Surface: cli → Component Patterns 4, the
`release --budget` line: the date form of `<end>` and the `budget gate open` line recorded as planning forms on the
operator's word, the founder's wording owed. Applied. The hint (`:760`) holds as landed: no change.

## layout-templates

- L1 · D-layout-surface · warning · §Output structure — wheel, handoff and dialog verbs · the `budget gate open`
  line → apply · EA 15; planning form, the founder's wording owed.
- L2 · D-layout-surface · warning · same section · the form of `<end>` → apply · EA 15.

## test-plan

- T1 · D-tests-framework · warning · §4, root bin, Budget governor · expiry is a pure-function case, the unit
  families as landed → apply · check 6, SCD 3; EA 9.
- T2 · D-tests-framework · warning · §6 Path 6 step 7 · the expiry pointer, mock_instant wording removed → apply ·
  dependent of T1.
- T3 · D-tests-framework · warning · §8, the Time bullet · `budget_override_until` leaves the clock-seam list; the
  `Utc::now()` sentence → apply · the orchestrator read the gate's three call sites (`src/cmd/run.rs`,
  `src/run/send.rs`): each reads `Utc::now()` directly; written as an exception read at those sites, no census.
- T4 · D-tests-framework · warning · §7, Time-sensitive data · mock_instant in no manifest → apply · measured by the
  orchestrator (`mock_instant` in no `Cargo.toml`); the claim's sites all lie in test-plan and all are amended, so
  the "Not this chunk's drift" caution (one artifact of several) does not apply.
- T5 · D-tests-framework · warning · §8, the Time mocking row · the same → apply.
- T6 · D-tests-framework · warning · §3 → Bootstrap phases, Dev-deps · the same → apply · key file edited.
- T7 · D-tests-framework · warning · §3 → Bootstrap phases · thirteen → fourteen verify-driven binaries → apply ·
  the site carries its rule (the `binary(…)` names on the filter line); the fourteen names read from
  `.config/nextest.toml:32`.
- T8 · D-tests-framework · warning · §3 → 5-command implementation, `run` · waits 26 → 28 → apply · with A18.
- T9 · D-tests-framework · escalate · §2 Agent-runnable invariants · the `budget.json` provenance rule scoped →
  **escalated, resolved** · the operator's answer in this wrap's dialog (`inputs#I11`, question 1): scope the rule.
- T10 · D-tests-framework · escalate · §7, the `budget.json` row → as T9, dependent.
- T11 · D-tests-framework · escalate · §11 Test Data, the NEVER hand-write ban → as T9, dependent.
- T12 · D-tests-coverage · warning · §6 Path 6 · the as-landed bullet → apply · EA 7. The clause that holds by the
  diff alone is named there, its case owed to the route.
- T13 · D-tests-coverage · warning · §6 Path 6 step 1 · `boot --statusline-echo` re-pointed to "The board: viola
  list" → apply · EA 8; the owner is the operator's direction (`inputs#I10` item 4).
- T14 · D-tests-coverage · warning · §3 → 5-command implementation, `boot` · the same re-point;
  `statusline-source-unresolved` dropped → apply · EA 8.
- T15 · D-tests-coverage · warning · §6 E5, the as-landed bullet · the budget leg landed → apply · EA 7.
- T16 · D-tests-coverage · warning · §5, out-of-range `Percent` · which arm landed → apply.
- T17 · D-tests-coverage · warning · §5, the controls bullet · the budget-gate negative joined → apply.
- T18 · D-tests-obs-harness · warning · §5, strict-modes refusals · `revive`'s start arm writes no line → apply ·
  check 6, SCD 5; the obs side is O6, O7, O16 (the bind holds on both sides).
- T19 · D-tests-obs-harness · warning · §6 exit-cause matrix, `revive` · the strict-modes case asserts no line →
  apply · EA 7.
- T20 · D-tests-obs-harness · warning · §4, viola-state, "its third preflight reading" → apply · as A12.
- T21 · D-tests-obs-harness · warning · §1, the CLI verbs entity · exit 1 wording → apply.

Raised by the orchestrator: §6 Path 6, the `release --budget` signal — the result holds `override_until` when a
pause stood (the resume point's finding 3). Applied.

## obs-plan

All 23 → apply. O13, O17 and O18 land in or beside §1 (a verbatim upstream copy): judged as any body amendment (the
playbook's "Verbatim upstream copy kept current"). Every "no line" is written as landed on the operator's answer,
the founder not having ruled (`inputs#I10` item 1).

- O1 · §6, `process-exit` exit 1 · `strict-modes-failed` left `revive`'s codes; three readings · SCD 5; EA 12.
- O2 · §1, the start path's log fields · dependent.
- O3 · §1, the Founder Direction 4 trigger · dependent (the claim without its token).
- O4 · §4 Scenario 1, log fields · dependent.
- O5 · §6, Filesystem refusals · both arms · SCD 4 and 5; EA 12.
- O6 · §3 → OTel SDK init, the `run` anchor · EA 10.
- O7 · §3 → Logging stack · EA 10.
- O8 · §10, bounded exemptions · exemption 1 widened, exemption 7 added, exemption 6's last sentence · EA 13.
- O9 · §2, agent-readable invariants · the exemption list · EA 13.
- O10 · §4, the surface table, cli · EA 13.
- O11 · §7, the main-thread catch site · EA 13.
- O12 · §7, the pre-init window.
- O13 · §1, Vector 7 · three strict-modes refusals with no line.
- O14 · §6, `hook-decision` fail-open codes · four → three of the seven, the rule named beside the count · EA 12.
- O15 · §4 Scenario: Budget governor, log fields · EA 11.
- O16 · §3 → OTel SDK init, the `hook` anchor · SCD 4; EA 10.
- O17 · §1, Vector 4.
- O18 · §2, the coverage map, Vector 4.
- O19 · §4 Scenario 1, Must-trace spans · `run.budget_eval` after `channel.bind` · SCD 1; the operator's direction
  (`inputs#I10` item 2).
- O20 · §1, the start path's Must-trace spans · dependent of O19.
- O21 · §4 Scenario: Budget governor, spans · the span as landed; two operations with no span or line · EA 11.
- O22 · §4, the wheel scenario · `release` with `budget: true`.
- O23 · D-obs-stack · §3 → OTel SDK init, steps 5 and 6 · `run`'s second read of `config.json`; one line per bad
  key; `CONFIG_KEYS` five names.

## a11y-plan

- Y1 · D-a11y-surface · warning · §8, Timeout extensions, CLI · the budget pause as a driver-only bound → apply ·
  EA 16. The not-gated clause written as read in the diff, no case yet.
- Y2 · D-a11y-surface · warning · §3 → Keyboard test harness, Tooling · the closed gate's own reading → apply · key
  file edited.

## Check 5 and check 6, closed

Expected amendments: 16 of 16 matched or raised (EA 14 by the orchestrator; the rest by proposals, as marked).
Spec claims disproved: 1 → O19, O20, A20 · 2 → a plan sentence, no master (the report's Deviations 1; curation) ·
3 → T1 to T6 · 4 → A6, A7, S3, O5, O14, O15, O16, O17, O18 · 5 → A9, A10, S1, T18, T19, O1 to O4, O6, O7.

## Escalations

Three, all resolved with the operator, none open:
1. the `budget.json` boundary widening (A5, S4): recorded NOT ratified, no card (`inputs#I10` item 1);
2. the `rate_limits` row's owner (A21): "Paste newline ledger row" (`inputs#I10` item 3);
3. the `budget.json` test-data rule (T9, T10, T11): scoped (`inputs#I11`, question 1).

One playbook rule appended on the operator's word (`inputs#I11`, question 2): "Held items landed on the operator's
word", verdict escalate, resting on the founder's standing rule of 2026-10-06 as the operator names it.

Totals: 76 proposed · 76 applied (70 routine, 6 after an escalation resolved) · 0 rejected · 5 sites raised by the
orchestrator.
