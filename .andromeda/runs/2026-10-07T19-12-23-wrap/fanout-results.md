# Fan-out results — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

Seven doc-agents, one batch, each sent the prompt of `amendment-flow.md` verbatim with its detectors (2 · 3 · 1 ·
1 · 3 · 3 · 2 = 15, the drift-base's fifteen). Every return was read for HTML entities after the strip: none
(`<runs>`, `&str` and `<name>` arrived as written). Change lines below are condensed from the returns; the applied
text is re-derived from the report, not pasted from them.

## Verdicts
- architecture — 10 proposals (8 under D-arch-decisions, 2 under D-arch-resources); a trailing comment block of
  sweep notes was stripped (sites read and left true: `:136`, `:89`, `:4`, `:458`, `:88`, `:275`, `:302` to `:306`).
- security-plan — 4 proposals under D-security-input (3 marked `escalate`, 1 `warning`); D-security-auth and
  D-security-deps clean. A leading comment block was stripped.
- design-system — `proposals: []`; two comment lines stripped (twin `.raw-fanout-design-system.md`).
- layout-templates — `proposals: []`; a comment block stripped (twin `.raw-fanout-layout-templates.md`).
- test-plan — 6 proposals under D-tests-coverage; D-tests-framework and D-tests-obs-harness clean. A leading
  comment block was stripped (sites read and left true: `:314`, `:539`, `:564`, `:588`, `:1061`, `:1078`).
- obs-plan — 3 proposals under D-obs-instrumentation; D-obs-stack and D-obs-pii clean. A trailing comment block
  was stripped (sites read and left true: `:865`, `:937`, `:314`, `:648`, `:645`).
- a11y-plan — `proposals: []`; a comment block stripped (twin `.raw-fanout-a11y-plan.md`).

## architecture
- A1 · D-arch-decisions · warning · §Established Decisions → [Delivery Confirmation], the matching sentence
  (`:49` c896) · change: the match compares the typed text (the sent text without its trailing LF,
  `hook::typed_text`), still exact · **apply** (check 1: accurate this-chunk addition; check 5: the plan's entry
  names the change).
- A2 · dependent-of A1 · [Delivery Confirmation], the exception sentence (`:49` c1628 to c2335) · change: the
  "one measured exception, unfixed … No remedy is built" sentence goes; the CLI measurement stands and no `send`
  relies on it; the outcome for a text ending in LF; what stays unmeasured · **apply** (checks 1, 5).
- A3 · dependent-of A1 · [Delivery Confirmation], the local-command sentence (`:49` c3955) · change: the typed
  text is classified; a listed command followed by newlines is that command · **apply** (checks 1, 5).
- A4 · dependent-of A1 · [Human Takeover / Wheel], the exception sentence (`:70` c2192) · change: the sentence
  goes; such a send is relabelled `driver` and moves no wheel; a prompt that is not the typed text claims nothing
  · **apply** (checks 1, 5).
- A5 · dependent-of A1 · §Standard Contracts → Event `data` per kind, `prompt-submitted`, the definition (`:292`
  c40) · change: `text` is the prompt as typed; matching compares it with the typed text · **apply** (checks 1, 5).
- A6 · dependent-of A1 · the same row, the exception sentence (`:292` c526) · change: the dropped newline stays
  as a CLI measurement; "does not match the sent text" goes · **apply** (checks 1, 5).
- A7 · dependent-of A1 · §Standard Contracts → Channel methods, `hook.event` (`:286` c520) · change: "exactly
  equals the in-flight `send`'s typed text" · **apply** (check 1; the plan's list does not name it, the report
  does).
- A8 · dependent-of A1 · [CLI Version Compatibility], the long-paste wrapper row (`:81` c1632) · change: the
  measurement stays; no `send` relies on it · **apply** (checks 1, 5).
- A9 · D-arch-resources · warning · §Occupied Resources → Repository (a row beside `target/deny-probes/`, `:428`)
  · change: register `target/profraw-census/` and its writer `scripts/profraw-census.sh` · **apply** (check 1: a
  new resource of a kind the registry enumerates; check 4: the absence claim rests on `profraw|census` over
  architecture.md and its key files, 0 hits, which the report's own pattern run confirms; check 5).
- A10 · dependent-of A9 · §Infrastructure Patterns → Project directory structure (the `scripts/` tree, key file
  `project-directory-structure.md:74`) · change: one tree line for the script · **apply** (check 1: the tree
  lists every other script).

## security-plan
The three `escalate` proposals are escalated and resolved without a halt, on two words already on record:
- the founder's live ruling of 2026-10-07T15:21Z (strip every trailing LF), given through the overseer's dialog
  after every option and the `/clear`-plus-newline consequence were shown to him, relayed by the overseer
  (inputs#I10). The playbook's "what ratifies it" rule counts a founder answer on disk when it was given after
  the specific change was shown to him; this one was;
- the operator's word at this wrap (`directive.md`): the security-plan amendment is not held for him.
Judged by subject, the change is also not the playbook's "Boundary widening" class: `validate_paste_text` is
unchanged and still runs first on the text as received, no new input class is admitted, and fewer bytes cross
into the PTY than before. It is an exception to the row's "don't strip" reason, which is the locked wording the
"Accurate this-chunk addition" rule sends to a ratification once. That ratification is the founder's above. No
playbook rule is proposed.
- S1 · D-security-input · escalate · §Input Validation → Paste text row (`:223`) · change: "Reject, don't strip"
  keeps its rule for refused characters and loses its reason "stripping would break the … exact match"; the
  trailing LF is removed after validation and the typed text is the one text `send` uses · **escalated, resolved
  as above → apply** (check 5).
- S2 · dependent-of S1 · escalate · §Security Anti-Patterns → Input (`:540`) · change: the three bans stay; a
  note that the trailing-LF removal is not the banned stripping; answer free text is untouched · **escalated,
  resolved as above → apply** (check 5).
- S3 · dependent-of S1 · escalate · §Threat Model Summary → CLI input vector → Trust boundary (`:74`) · change:
  delivery confirmation compares with the typed text · **escalated, resolved as above → apply** (the playbook's
  "Verbatim upstream copy kept current" rule: judged like any body amendment, the label stays).
- S4 · dependent-of S1 · warning · the same vector → Entry point (`:73`) · change: the bridge send writes the
  typed text · **apply** (the same rule; the sentence says what is written into the PTY).

## test-plan
Three returns carry a `basis` naming source lines the report does not carry (`src/run/send.rs:1353-1568`,
`crates/viola-agent-claude/src/hook.rs:666`, `tests/cli_send.rs:286`): the re-derivation tell. They are rejected
as proposals and their facts, which the report does carry, are raised by the orchestrator.
- T1 · D-tests-coverage · warning · §4 → root bin, the matcher bullet (`:581`) · **rejected as a proposal
  (source-line basis); raised by the orchestrator under check 5 → apply**: the matcher compares the typed text;
  the count is eight functions, 15 cases (the report's Counts bullet).
- T2 · dependent-of T1 · §4 → viola-agent-claude, a bullet for `typed_text` · **rejected as a proposal
  (source-line basis); raised by the orchestrator → apply** (check 1: the function and its nine cases are in the
  report's Symbols and Coverage bullets; the crate's list names every other rule).
- T3 · dependent-of T1 · §1 Test Scope Summary → the confirmed-send path sentence (`:233`) · change: the list is
  consumed by exact equality of the typed text; a send ending in LF is confirmed · **apply** (check 1; the
  kept-current rule for a scope copy).
- T4 · dependent-of T1 · §1, the same path's receipt bullet (`:238`) · change: "newlines intact" narrows to inner
  newlines · **apply** (check 1).
- T5 · dependent-of T1 · §6 Scenario: Path 2 → Verification signal · **rejected as a proposal (source-line
  basis); raised by the orchestrator under check 5 → apply**: the cross-process case and its two cases; the
  `/clear`-plus-newline consequence is unit-tier only; the live cases are owed to "First live test and
  self-drive".
- T6 · dependent-of T1 · §10 → Stack adjustments, the corrupt-profile sentence (`:1207`) · change: the two
  earlier runs stay recorded, not established; the mechanism measured here, the start test's host child,
  ci#37627485806 attempt 1 closed by mechanism with its limit, the witness script and its tier · **apply**
  (checks 1, 5; its basis is the chunk's evidence file, which the report names).

## obs-plan
- O1 · D-obs-instrumentation · warning · §4 Scenario: Confirmed `send` (CL-1) → Required span attributes (`:636`)
  · change: `text_bytes` is the typed text's length · **apply** (checks 1, 5).
- O2 · dependent-of O1 · the same scenario → Required log fields (`:641`) · **apply** (checks 1, 5).
- O3 · dependent-of O1 · §6, the field catalog's `send-*` row (`:819`) · change: the same qualifier in the row's
  parenthetical style · **apply** (check 1; the catalog is the place a reader looks the field up).

## The six checks
1. Playbook: every applied proposal matches "Accurate this-chunk addition", the security group through its
   ratification clause (above). No "Not this chunk's drift" and no "Registry over-reach": A9 and A10 register a
   script and a `target/` directory of kinds both registries enumerate.
2. Cross-contradiction: none. No two proposals edit one passage in opposing directions.
3. Intent-consistency: the report's deviations are each justified by a recorded word (inputs#I5 to inputs#I10).
   The scope record's one line is `in-intent`, serves step 7, and carries the overseer's word on a technical
   fork; its `serves` holds (the edit is the step 7 test).
4. Absence needs evidence: A9's "neither appears" is the `profraw` pattern run of the report (hits in test-plan
   only). T-group and O-group "left true" sites were re-read here by bounded window for `:314`, `:564`, `:1061`
   (test-plan) and `:136` (architecture).
5. Expected amendments: all eight entries of the plan's list are matched — [Delivery Confirmation] A1 to A3 ·
   [Human Takeover / Wheel] A4 · `prompt-submitted` A5, A6 · the long-paste wrapper row A8 · Repository A9 ·
   security-plan S1, S2 · test-plan T1, T5, T6 · obs-plan O1, O2.
6. Disproved claims: the report lists none in a master. The two plan-level claims it names were corrected by the
   plan's own revision before this wrap: DISPOSED.

## Totals
23 proposals: 23 applied (3 of them through the orchestrator's raise after a rejected basis, 3 escalated and
resolved on recorded words). 0 rejected outright. 0 escalations open.
