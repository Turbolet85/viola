# Fan-out results — wrap of 2026-10-10-session-links

Seven detectors, one batch, each on the report (`viola-0.1.0/chunks/2026-10-10-session-links/report.md`) and its
own master. Detector counts in the prompts: architecture 2, security-plan 3, design-system 1, layout-templates 1,
test-plan 3, obs-plan 3, a11y-plan 2 (15, the drift-base's 15 `doc:` names). Every return decoded clean (no entity
in any value).

Each row is one proposal as parsed: detector · severity as returned · section · the change in one line. After `→`
its disposition and the check that decided it. The applied text is re-derived from the report, never pasted from
the change line. "EA n" is entry n of the plan's Expected amendments list; "SCD" is the one entry of the report's
Spec claims disproved.

Coordinate check (Validate's opening rule): every source coordinate a proposal cites is the report's or stands in
its last section. Read against `new-text.md` for the ones the report's bullets do not hold:
`crates/viola-state/src/replay.rs` 494-646; `crates/viola-state/tests/state_replay.rs` 251-327, 278-298, 300-326;
`tests/cli_links.rs` 99-281, 218-251, 253-281 and the heads 103, 141, 186, 221, 256; `tests/cli_budget.rs` 475.
**Rejected for a source read the report does not carry: 0.**

Order note: the masters whose detectors returned first (a11y-plan, security-plan, obs-plan, test-plan) were
amended while the architecture detector was still reading architecture. No detector read a master another
detector's amendment had changed.

## Verdicts

- architecture: 12 proposals (2 detectors); stripped: three header notes (no resource landed; no decision
  contradicted; line numbers only where the report states them).
- security-plan: 2 proposals; stripped: notes that `D-security-auth` and `D-security-deps` hold, that no
  security-plan key file exists, and one flag for the orchestrator (S1 below).
- design-system: 0 proposals; stripped: a note that every Coverage row reads `tokens n/a` and Part B's lines are
  not carried (raw twin: `.raw-fanout-design-system.md`).
- layout-templates: 0 proposals; stripped: a note that no surface was added and the `link` line maps to regions
  the master already describes (raw twin: `.raw-fanout-layout-templates.md`).
- test-plan: 6 proposals; stripped: notes that `D-tests-obs-harness` holds, that every new path carries its
  tests, and the list of explicit-verb sentences left standing.
- obs-plan: 5 proposals; stripped: notes that `D-obs-stack` and `D-obs-pii` hold.
- a11y-plan: 2 proposals; stripped: a note that neither invariant is literally violated and both proposals are
  filed under the nearest detector, and the sweep it ran.

Total 27.

## architecture

- A1 · D-arch-resources · warning · §Standard Contracts, Snapshot envelope · retire "`links` replays empty until
  the link kinds land"; the replay derives the set through `apply_link_line` → **applied** (check 1: accurate
  this-chunk addition; EA 1). The `since: "unknown"` clause is written as kept on the operator's word.
- A2 · D-arch-resources · warning · §Standard Contracts, Event `data` per kind, the closing sentence · the wrapper
  also appends `link` on an implicit link; no product path appends `unlink` → **applied** (check 1; EA 1).
- A3 · D-arch-resources · warning · Channel methods, `wait` · `link` joins the wrapper lines that never wake →
  **applied** (check 1; dependent of A2). The detector asked for a confirmation: read at
  `src/run/links.rs:95-104`, `change` appends through `append_event` and holds no wait feed.
- A4 · D-arch-resources · warning · Instance snapshot, the `links` bullet · the wrapper's `LinkSet`, the restore,
  the first snapshot → **applied** (check 1; EA 1).
- A5 · D-arch-resources · warning · [Session Liveness], the start chain · the restore before the endpoint serves,
  the log read twice → **applied** (check 1; dependent of A4). No position against the bind or the budget gate
  is claimed.
- A6 · D-arch-resources · warning · [Database / State Store], the one reader's list · gains the restore; `link` /
  `unlink` are known kinds → **applied** (check 1).
- A7 · D-arch-resources · warning · Channel methods, `send` · the implicit link before `send-issued`, `-32603`
  → **applied** (check 1; EA 2).
- A8 · D-arch-resources · warning · Channel methods, `answer` · the implicit link before the answer is taken,
  `-32603`, the dialog stays held → **applied** (check 1; EA 2), as a fourth sub-bullet of the row.
- A9 · D-arch-resources · warning · §Infrastructure Patterns → Project directory structure · `src/run/links.rs` →
  **applied** in the key file (check 1; the file lists every `run/` module, so this is its own grain, not
  "Registry over-reach"). `registry.py check`: 0 defects.
- A10 · D-arch-resources · warning · the same key · `viola-state` gains the link rule → **applied** (dependent of
  A9).
- A11 · D-arch-decisions · warning · [Links] · the as-landed passage → **applied** (check 1 escalates under
  "Held items landed on the operator's word"; resolved by the operator's direction naming the items, `inputs#I6`
  item 3: the five shapes stand, written as built, marked not ruled by the founder). The explicit verbs are
  written as NOT built and held for the founder's word; no widening landed, so there is nothing to ratify and no
  card (`inputs#I6` item 1).
- A12 · D-arch-decisions · warning · [Budget Governor] · the four verbs' case exists → **applied** (check 1; EA 2).
  The residual "no case drives `hook.dialog` under a closed gate" is kept from the old sentence.
- Raised by the orchestrator (check 5, `inputs#I6` item 1): Channel methods, the `link` / `unlink` row gains
  "neither is built … the wrapper answers both `-32601`" → **applied**; the report's "No channel method, no CLI
  verb" substantiates it.

## security-plan

- S1 · D-security-input · warning · §Input Validation, row "Own state files on read" · the reader's list gains
  `LinkSet::restore`; the log is read twice at start; the rule takes names as strings → **applied** (check 1;
  EA 3). The detector's flag is written into the row as a fact: the rule does not parse a logged `driver` /
  `driven` as a name again. Not a boundary widening: the same reader, the same file, the same point of the
  start as the wait feed's rebuild; no order against the strict-modes check is claimed.
- S2 · D-security-input · warning · the same row · the one-product-caller sentence is of the replay; the rule has
  a second caller → **applied** (dependent of S1).

## design-system

No proposals.

## layout-templates

No proposals.

## test-plan

- T1 · D-tests-coverage · warning · §4, viola-state · retire "`links` replays empty"; the rule and `link_rule_*`
  → **applied** (check 1; EA 4).
- T2 · D-tests-coverage · warning · §1, the chaos-test trigger · the same claim → **applied** (dependent of T1).
- T3 · D-tests-coverage · warning · §6 Scenario E5, As landed · the link leg landed; `list --json` still owed →
  **applied** (dependent of T1; EA 4). Read at `tests/cli_links.rs:218-281` before writing.
- T4 · D-tests-coverage · warning · §5, On-disk · 13 → 15 kinds; two link cases → **applied** (check 1, the
  playbook rule "A count in a master that carries no rule": no sidecar entry wrote "13 kinds" with a rule, the
  plan's approved entry and the report give it, the length of `EventKind::ALL`, and it is written beside the
  number at the one site; EA 4).
- T5 · D-tests-coverage · warning · §6 Path 6, As landed · the four verbs' case is no longer owed → **applied**
  (check 1; EA 4).
- T6 · D-tests-framework · warning · §3 → Bootstrap phases, the kill filter line · fourteen → fifteen,
  `cli_links` → **applied** in the key file (check 1, the same count rule: the sidecar entries of the two earlier
  moves name the rule, the `binary(…)` names on that filter line; EA 4). `registry.py check`: 0 defects.
- Raised by the orchestrator (check 5): §6 Scenario E3 gains an As landed bullet (the implicit link's four root
  cases; steps 1 and 3 not built and held) → **applied**. EA 7's E3 entry is not carried; this bullet claims
  nothing of Part B.

## obs-plan

- O1 · D-obs-instrumentation · warning · §4 Edge flows, E3 · the implicit link as landed, no line, no span; the
  method pair belongs to the explicit methods → **applied** (check 1; EA 6), the operator-word mark pointing to
  architecture [Links].
- O2 · D-obs-instrumentation · warning · §4 Scenario: Confirmed `send` · the link record carries no span and no
  line → **applied** (dependent of O1).
- O3 · D-obs-instrumentation · warning · §4 Scenario: Dialog → `answer` · the same → **applied** (dependent of
  O1).
- O4 · D-obs-instrumentation · warning · §4 Scenario: `viola run` start · a third start step with no span and no
  line, the restore → **applied** (check 1).
- O5 · D-obs-instrumentation · warning · §4 Edge flows, E5 · the replay derives the link set, no line, no span →
  **applied** (check 1; EA 6).

## a11y-plan

- Y1 · D-a11y-surface · warning · §8 Timeout extensions, CLI · the four verbs' case exists → **applied** (check
  1; EA 5).
- Y2 · D-a11y-surface · warning · §3 → Keyboard test harness, Tooling · the second closed-gate case → **applied**
  in the key file (dependent of Y1; EA 5), with the fact read in the case's body that it types no human key and
  reads no terminal stream. `registry.py check`: 0 defects.

## The checks, whole

1. Playbook: 26 routine ("Accurate this-chunk addition"; two under the count rule); 1 escalated class (A11,
   "Held items landed on the operator's word"), resolved by the recorded direction `inputs#I6` item 3, no halt.
   No proposal records a landed boundary widening.
2. Cross-contradiction: none. A12, T5 and Y1 state one fact; A7, A8, O2 and O3 state one order.
3. Intent-consistency: the report diverges from the entry in one place, Part B is not built. Justified by the
   plan's step 11 and the operator's directions (`inputs#I4`, `inputs#I6`): the master record's description is
   rewritten at the flip to what landed. Scope record: none (`scope: clean`, recorded 0).
4. Absence needs evidence: "no product path appends `unlink`" rests on `grep -rn -E '\.remove\(' src/run
   src/cmd/run.rs` (the only `LinkSet::remove` calls are in the test module of `src/run/links.rs`, lines 301 to
   316 of a module that opens at 107); "no master says `link` or `unlink` is served" rests on the `served`
   pattern of the cascade sweep, every row dispositioned in `cascade-dispositions.md`.
5. Expected amendments: EA 1 → A1, A2, A4; EA 2 → A7, A8, A11, A12; EA 3 → S1, S2; EA 4 → T1 to T6; EA 5 → Y1,
   Y2; EA 6 → O1, O5; EA 7 ("only when Part B lands") → **not carried**: Part B is held, the report says so, and
   no master was amended as if it had landed. No citation row was dispositioned `claim false`.
6. Disproved claims: SCD (the plan's note that its `schema-check` entry reads the new cases' lines) → **DISPOSED:
   plan note only, nothing to amend.** Searched: `schema-check|G4` beside `home|kept|keep|dev host` over
   test-plan, obs-plan and their key files, 18 hits read. Every one says the homes are read where they are kept
   (`AGENT_RUN_KEEP_HOMES=1`, CI), and the key file `test-data-bootstrap.md` says a `TestHome`'s drop deletes on
   the dev host. No master says the dev host's `schema-check` reads a root test's home. Routed to curation
   (Phase 3, candidate 3).

Applied: 27 proposals and 2 orchestrator-raised amendments. Escalations halted: 0 (1 class resolved by a recorded
direction). Rejected: 0.
