# Fan-out results — wrap of 2026-10-11-the-board-viola-list

Seven Explore doc-agents, one batch, prompts per amendment-flow.md (detector counts 2·3·1·1·3·3·2 = 15, the drift-base's
`doc:` names). Every return was YAML; no entity was found to decode (no `&lt;` `&gt;` `&amp;` in any return). Stripping
removed only the design-system return's trailing `#` commentary (its substance: every new surface's `tokens` flag reads
`n/a`, no `hardcoded✗`).

## Verdicts

- architecture — 14 proposals
- security-plan — 11 proposals
- design-system — 0 proposals (`proposals: []`; commentary stripped, substance above)
- layout-templates — 3 proposals
- test-plan — 12 proposals
- obs-plan — 10 proposals
- a11y-plan — 11 proposals

Total 61. Coordinates: every line number a proposal carries is either stated in the report or a row / `@` line / span of
its `## New text, by line` section (checked row by row against `new-text.md`). 0 rejected for a source the report does
not carry.

## Validate — the six checks, summary

- Check 1 (playbook): the routine rule "Accurate this-chunk addition" governs every proposal below marked `apply`.
  The proposals that write the exit-21 whole-board refusal, its no-log shape, the home pair or the five board forms
  fall under "Held items landed on the operator's word" (escalate): items 1–3 of the card. A11 (subcommand
  registration) is a CLI-verb registration ("Registry over-reach" names CLI verbs), but the bullet already registers
  `revive`'s arguments and exits and the plan's expected amendment 1 names "the subcommand and tree lines" → apply.
  S1 (home not canonicalised): the gap is pre-existing and owned — "CLI machine contract" carries "resolves `--home`
  as given … without `std::fs::canonicalize`" (chunk 2026-09-24) — so the "Sequencing deferral" rule governs: the row
  states `list` resolves like every `cli` verb today, the owner named; no widening. The board's read of `budget.json`
  (S5, S6, A7) is judged no boundary widening: the same checked read, nothing decided on it by `list`, the values
  shown as data; the unratified widening stays the gate's refusal alone.
- Check 2 (cross-contradiction): none. S11 and T5 both narrow the "keeping `\n` and `\t`" claim to `wait` / `last`,
  agreeing with design-system:747 and layout-templates:580.
- Check 3 (intent-consistency): the report's Deviations 1 (WHEEL 23) and 4 (the two failure paths) are decisions no
  input words → card item 1. Scope record: none (`gate.py scope` clean). Everything else is inside the plan.
- Check 4 (absence needs evidence): the "not built" proposals (A1–A3, S9–S10, Y6–Y11) rest on the report's Not built
  bullet; the cascade sweep covers the restatements.
- Check 5 (expected amendments): entries 1, 2, 4, 5, 6, 7 matched by proposals; raised by the orchestrator: R1
  (design-system §Surface: cli — `list` in the exit-21 / exit-1 phraseology, the five planning forms, the row mode's
  reach: card items 1–2) and R2 (layout-templates Output structure `viola list` as landed — the styled form and the
  unwrapped rack not built, each with its owner by title; the NAME width; the planning forms: card items 1 and 3).
  Root waits: the report re-counts none and the new cases add none → no change. 0 claim-false citation rows.
- Check 6 (disproved claims): 1 → T5 (+ S11); 2 → no master states 22 (layout-templates' sample already pads WHEEL to
  23) — plan text, card item 1 (implement's decision); 3 → no master ties the `budget.json` reading to Unix (the
  sweep of `Unix only|on Unix` near `budget` over the masters and keys: one hit, architecture:79, which is about the
  pass-through's shell, true) — research text only, routed to curation; 4 → T11.

## Proposals, parsed, with dispositions

### architecture
- A1 · D-arch-resources · §Standard Contracts → Channel methods (`list`) · `list` as landed: caller-side read of
  `instances/` and `budget.json`, no endpoint, no `claude agents --json`; `{items, budget, skipped}`; `wrapped: true`,
  `status: "unknown"`; enrichment, unwrapped rows and colour not built → apply (check 1, routine), owners by title
  per card item 3.
- A2 · dependent-of A1 · §Established Decisions → [API Style] · `list` reads the disk only as landed → apply.
- A3 · dependent-of A1 · §Standard Contracts → Session liveness · `mcp` / `ui` spawn `claude agents --json`; `list`
  as landed does not, status always `unknown` → apply.
- A4 · §Established Decisions → [Database / State Store] (basis architecture:50) · `read_from` gains
  `board::log_pass`; `list` is the first surface that shows the skip counts → apply.
- A5 · §Standard Contracts → Snapshot envelope (basis :247) · the replay has two product readers → apply.
- A6 · §Occupied Resources → Filesystem `instances/` (basis :407) · three readers check these files; the scan's
  refusal, exit 21, no log, the operator's answer → escalate (card item 1), then apply as answered.
- A7 · §Occupied Resources → Filesystem `budget.json` (basis :408) · `read_budget_stamped` the one checked read,
  `read_budget` wraps it, two product paths → apply.
- A8 · §Occupied Resources → Filesystem `config.json` (basis :400) · thresholds gain `list` as second reader → apply.
- A9 · §Occupied Resources → Filesystem `diagnostics/` · `list` a `cli` producer under `VIOLA_NAME` only → apply.
- A10 · dependent-of A9 · §Conventions → exit `1` · `list` among the `cli` verbs of the catch site → apply.
- A11 · §Occupied Resources → Binary, subcommands and exit codes · `list [--json]`, exits 0 / 21 / 1 → apply (check 1
  note), its exit-21 wording per card item 1.
- A12 · dependent-of A11 · §Conventions → exit `21` · also `list`'s strict-modes refusal, the operator's answer →
  escalate (card item 1), then apply.
- A13 · §Infrastructure Patterns → Project directory structure (key) · `viola-state` `board.rs`,
  `read_budget_stamped`, the replay's two readers → apply.
- A14 · same key · `src/human.rs` gains `escape_row`, the two state pairs, the board writers; `cmd/list.rs` → apply.

### security-plan
- S1 · D-security-input · §Input Validation → CLI arguments / stdin · `list` makes the home absolute, not
  canonicalised → apply under "Sequencing deferral" (owner "CLI machine contract", its CARRY), no widening.
- S2 · §Input Validation → Own state files on read (:241) · the replay's two product callers → apply.
- S3 · dependent-of S2 · §Threat Model Summary → Filesystem vector (:122) · same → apply ("Verbatim upstream copy kept
  current").
- S4 · §Input Validation → Own state files on read · `read_from` gains `board::log_pass`; first product reader of
  the counts → apply.
- S5 · same row · `budget.json` one checked read, two product paths → apply (check 1: no widening).
- S6 · dependent-of S5 · §Authentication & Authorization → `~/.viola/` access control (:213) · same → apply.
- S7 · §Input Validation → Configuration values · thresholds gain `list` → apply.
- S8 · D-security-auth · §Authentication & Authorization → `~/.viola/` access control · `list` joins the
  strict-modes entry points; whole-board refusal exit 21, no log, the operator's answer → escalate (card item 1),
  then apply.
- S9 · §Input Validation → Child executable resolution (:245) · CLI `list` as landed resolves no `claude`; the
  contract's `list` leg waits on its owner → apply, owner per card item 3.
- S10 · dependent-of S9 · §Threat Model Summary → child-spawn vector (:127) → apply.
- S11 · dependent-of S9 · §Bootstrap phases → logging-redaction-wire (:437) · `list` rows escape `\n` `\t` too;
  `wait` / `last` keep them → apply.

### design-system
- none proposed; R1 raised by the orchestrator (check 5) → card items 1–2.

### layout-templates
- L1 · D-layout-surface · §Surface: cli → Primary content block 2 (:587) · `list`'s two exit-21 pairs and the
  catch-site verb → escalate (card item 1), then apply.
- L2 · dependent-of L1 · same block, exit-code tail (:588) · `list`'s exit 21 human form is the state pair →
  escalate (card item 1), then apply.
- L3 · dependent-of L1 · Output structure `viola list --json` (:397, :400) · the refusal document → escalate (card
  item 1), then apply.
- R2 raised by the orchestrator (check 5).

### test-plan
- T1 · D-tests-coverage · §6 Scenario E5 (:952) · the `list` leg landed (`e5_…` ×2); two product readers → apply.
- T2 · dependent-of T1 · §6 Path 6 · the `list --json` / board leg landed; the harness-session form and step 7's
  two-instance leg owed to a new owner → apply, owner per card item 3.
- T3 · dependent-of T1 · §6 Chaos (torn append) (:1022) · `torn_lines` on `list --json` landed → apply.
- T4 · §6 Chaos (forward-compat) (:1027) · `chaos_forward_compat.rs` landed for the `list` leg → apply.
- T5 · §4 root bin (:591) · the escape line split: `list` rows escape `\n` `\t` → apply (disproved claim 1).
- T6 · D-tests-framework · §5 CLI trycmd bullet (:655) · the board pinned by literal in `tests/cli_list.rs` → apply.
- T7 · dependent-of T6 · §5 Drivers (:611) → apply.
- T8 · dependent-of T6 · §5 driver table (:689) → apply.
- T9 · dependent-of T6 · §6 Cross-surface parity · the CLI caption row is the `CAPTION` literal → apply.
- T10 · §3 → Bootstrap phases (key) · the kill line 15 → 18 binaries, rule "every binary that drives verify" → apply
  (playbook "A count in a master that carries no rule": the rule is named; every site amended).
- T11 · D-tests-obs-harness · §3 → Status endpoint shape (key, :19) · the interim's premise: `status` runs no
  `viola list` yet → apply, owner per card item 3 (disproved claim 4).
- T12 · dependent-of T11 · §3 → 5-command implementation (key) · `--statusline-echo` owner → apply, owner per card
  item 3.

### obs-plan
- O1 · D-obs-instrumentation · §6 Filesystem refusals · `list`'s refused scan writes no line, the operator's answer →
  escalate (card item 1), then apply.
- O2 · dependent-of O1 · §1 Vector 7 trigger · the no-line refusals gain `list`; `read_budget_stamped` → escalate
  (card item 1), then apply.
- O3 · dependent-of O1 · §4 cli row · `list` opens its log only after the scan, only under a name → escalate (card
  item 1), then apply.
- O4 · dependent-of O1 · §10 exemption 1 · the pre-init window names `list`'s scan → apply.
- O5 · dependent-of O1 · §10 exemption 7 · `list`'s refused scan beside revive and statusline, the operator's answer →
  escalate (card item 1), then apply.
- O6 · dependent-of O1 · §2 agent-readable invariants · same restatement → escalate (card item 1), then apply.
- O7 · §4 Edge flows E5 (:731) · the replay's second product reader → apply.
- O8 · §6 Filesystem refusals · the `state-entry` line's call site `board::read` → apply.
- O9 · §3 → OTel SDK init (key) step 6 · `list` the second reader of the thresholds → apply.
- O10 · dependent-of O9 · same key, step 5 → apply.

### a11y-plan
- Y1 · D-a11y-surface · §4 P1 cli mirror (:543) · the caption pinned by literal, not trycmd → apply.
- Y2 · dependent-of Y1 · §8 Consistent identification (:859) → apply.
- Y3 · dependent-of Y1 · §8 Plain language → Verification · `list` strings pinned by literals → apply.
- Y4 · dependent-of Y1 · §8 Error recovery → cli · the `list` refusal pairs pinned by literal → apply (wording per
  card item 1).
- Y5 · §4 P6 (:594) · `list`'s row escape checked by unit cases on a recorded version; no `claude agents --json` →
  apply.
- Y6 · §1 Surface cli → ARIA roles inventory (:173) · the board as landed; separator and grouped help not built →
  apply, owners per card item 3.
- Y7 · dependent-of Y6 · §4 landmark roles (:498) → apply.
- Y8 · dependent-of Y6 · §1 Critical path 6 → apply.
- Y9 · §6 CLI equivalent · colour not built; states carried by words → apply, owner per card item 3.
- Y10 · dependent-of Y9 · §1 cli notes → apply.
- Y11 · dependent-of Y9 · §4 P6 outer-PTY SGR → apply.
