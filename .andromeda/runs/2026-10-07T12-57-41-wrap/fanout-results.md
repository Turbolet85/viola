# Fan-out results — 2026-10-07-send-waits-out-the-paste-hint

Seven doc-agents, one batch, 15 detectors (arch 2 · security-plan 3 · design-system 1 · layout-templates 1 ·
test-plan 3 · obs-plan 3 · a11y-plan 2 = the 15 `doc:` names of `drift-base.md`). Every return was stripped of its
`#` commentary before the parse; the entity probe over this file reads 0 (`&lt;` · `&gt;` · `&amp;`).

Validation legend: C1 playbook · C2 cross-contradiction · C3 intent · C4 absence needs evidence · C5 expected
amendments · C6 disproved claims. "Tell" = the re-derivation tell of §Validate: a `basis` naming a source location
the report does not carry; the proposal is rejected and its fact, carried by the report, is raised by the
orchestrator under C5.

Authority recorded with every applied amendment (the overseer's disposition 1, `directive.md`): the wait and the
8.5 s bound are the founder's live ruling of 2026-10-07T10:29Z, relayed by the overseer; the second read of the
wheel and the turn is the overseer's technical answer of 2026-10-07T12:10Z (inputs#I2), not the founder's.

## Verdict lines
- architecture — 9 proposals. Stripped: its scope note, the two detector readings, and the list of sites read and
  left (`:4`, `:48` partial-gate sentence, `:91` verify settle rule and the `--version` kill, `:174` / `:455`,
  `:229`, `:326`).
- security-plan — `proposals: []`. Stripped: a per-detector basis, a sweep reading 0 hits for the retired claims,
  and one site read and left (`:126`, a dated fact about a card). Raw twin kept.
- design-system — `proposals: []`. Stripped: its basis and a coverage note. Raw twin kept.
- layout-templates — `proposals: []`. Stripped: its basis and a sweep note. Raw twin kept.
- test-plan — 7 proposals. Stripped: the D-tests-obs-harness reading (no drift) and the sites swept and left.
- obs-plan — 2 proposals. Stripped: the three detector readings (all hold) and its note that the two proposals
  are the plan's expected item, not an invariant violation.
- a11y-plan — 3 proposals. Stripped: the two detector readings (the strict trigger did not fire; the proposals are
  omissions the report's expected list names) and the sites swept and left.

## architecture (9)
| # | detector | section | change (as proposed, condensed) | basis | disposition |
|---|---|---|---|---|---|
| A1 | D-arch-decisions | [Screen Model] | `GATE_MAX_WAIT` = 8.5 s (was 5 s), raised on the founder's ruling; "their values are unchanged" holds for `QUIET_PERIOD` only; re-verified settles 1 103 / 617 ms | `architecture.md:48` | apply — C1 accurate this-chunk addition; the recorded ruling settles the locked value's move |
| A2 | D-arch-decisions (dep. A1) | [Screen Model] | the verdict's waiting arm in place of "or either check fails … reports `input-not-ready`" | `architecture.md:48` | apply — C1; also the decision's opening sentence, the same claim's site (orchestrator, C4 window read) |
| A3 | D-arch-decisions (dep. A1) | [Human Takeover / Wheel] | the second wheel and turn reads; the 2026-10-04 residual retired; 8.5 s | `architecture.md:70` | apply — C1; recorded as the overseer's technical answer, not the founder's |
| A4 | D-arch-decisions (dep. A1) | §Conventions, refusal order | the second `human-typing` / `turn-running` read after the gate | `architecture.md:136` | apply — C1 |
| A5 | D-arch-decisions (dep. A1) | [CLI Version Compatibility] | "It is not built …" replaced by the built state; cap 10 000 ms; hint line unchanged | `architecture.md:91` | apply — C1, C5 |
| A6 | D-arch-decisions (dep. A1) | [CLI Version Compatibility] | the refused-at-0.63 s outcome dated as before the chunk; the delivered outcome added | `architecture.md:91` | apply — C1; the body states the scope of the measurement (fake agent; not the real CLI) |
| A7 | D-arch-decisions (dep. A1) | [CLI Version Compatibility] | "the readiness gate … is unchanged by this" retired | `architecture.md:91` | apply — C1 |
| A8 | D-arch-decisions (dep. A1) | [Delivery Confirmation] | the longest block stated, 18.5 s | `architecture.md:49` | apply — C5 (the plan's entry names the change) |
| A9 | D-arch-resources | §Occupied Resources, fake agent | `--paste-hint-ms` capped at 10 000 ms | `architecture.md:355` | apply — C1, C5 |

## test-plan (7)
| # | detector | section | change (condensed) | basis | disposition |
|---|---|---|---|---|---|
| T1 | D-tests-framework | §3 → Bootstrap phases, `[profile.ci]` | `verify_window_` 20 s × 3 (60 s); 8.5 s maximum; floor 34.3 s / 34.3–35.3 s; hold 9 s, 13.1–13.7 s | key file `:10`; `.config/nextest.toml:23-27` | REJECT — the tell (`nextest.toml` lines). Raised by the orchestrator, C5 (test-plan §3): apply |
| T2 | D-tests-framework (dep. T1) | §3 → Bootstrap phases, `[profile.mutants]` | `verify_window_` 15 s × 3 (45 s); the three overrides no longer share one line | key file `:11`; `.config/nextest.toml:54-58` | REJECT with its primary. Raised by the orchestrator, C5: apply |
| T3 | D-tests-framework (dep. T1) | §5, the `cli_verify` bullet | the hold is 9 s (was 6 s) | `test-plan.md:654` | REJECT with its primary. Raised by the orchestrator from the report's Counts bullet: apply |
| T4 | D-tests-coverage | §6 Path 2, the `local` bullet | the pair delivered in both cases; the keystroke case; the bound's refusal unit-only; the stated limit | `test-plan.md:750`; `tests/cli_send.rs:554`, `:614` | REJECT — the tell (`cli_send.rs` lines). Raised by the orchestrator, C5 (test-plan §6): apply |
| T5 | D-tests-coverage (dep. T4) | §4, "Refusal ordering" | the second wheel and turn reads after the gate | `test-plan.md:588` | REJECT with its primary. Raised by the orchestrator as the same claim's site: apply |
| T6 | D-tests-coverage (dep. T4) | §4, "Screen signatures" | the waiting arm and the 8 500 ms pin | `test-plan.md:566` | REJECT with its primary. Raised by the orchestrator (the two-valued verdict is now three-valued): apply |
| T7 | D-tests-coverage | §7 Fake agent, Modes | cap 10 000 ms (was 8 000 ms) | `test-plan.md:1077` | apply — C1, C5 |

## obs-plan (2)
| # | detector | section | change (condensed) | basis | disposition |
|---|---|---|---|---|---|
| O1 | D-obs-instrumentation | §5, the `Send → readback latency` row | `duration_ms` starts at the send's arrival and spans the gate's wait, named by its constant | `obs-plan.md:763`; `src/run/send.rs:312`, `:341`, `:436` | REJECT — the tell (`send.rs` lines). Raised by the orchestrator, C5 (obs-plan §5): apply as a qualifier |
| O2 | D-obs-instrumentation (dep. O1) | §4 Scenario 2, the `send-confirmed` bullet | the same start instant on the field's own line | `obs-plan.md:643`; `src/run/send.rs` | REJECT with its primary. Raised by the orchestrator as the same field's second site: apply |

## a11y-plan (3)
| # | detector | section | change (condensed) | basis | disposition |
|---|---|---|---|---|---|
| Y1 | D-a11y-surface | §3 → Keyboard test harness | the keystroke case during the gate's wait, with its limit | key file `keyboard-test-harness.md:8` | apply — C1, C5 |
| Y2 | D-a11y-surface | §8, the CLI timing clause | the gate's bound as a driver-only bound | `a11y-plan.md:823` | apply — C5 (outside the detector's §5–§7 wording; it rides the plan's entry) |
| Y3 | D-a11y-surface (dep. Y1) | §4 P4, the tui bullet | the further case named beside the three boundary cases | `a11y-plan.md:573` | apply — the primary applied |

## Orchestrator, check 5 — the plan's `Expected amendments (wrap)` list, entry by entry
- architecture [Screen Model] → A1, A2. [CLI Version Compatibility] → A5, A6, A7. [Human Takeover / Wheel] → A3 (and
  A4, its second site). [Delivery Confirmation] → A8. §Occupied Resources → A9.
- test-plan §3 → T1, T2 raised. §6 → T4 raised. §7 → T7.
- obs-plan §5 → O1, O2 raised.
- a11y-plan §8 → Y2. §3 → Y1.
- security-plan §Input Validation, the `send` rungs read again → NOT raised. The report's search reads 0 hits for
  the `send` rungs in security-plan, and the security-plan detector's own sweep agrees: `:223` is the paste-text
  row (it places `validate_paste_text` first and says nothing of the wheel or the turn), `:239` is the PTY-output
  row. No sentence of security-plan is made false, and the fact is carried where the order is stated
  (architecture [Human Takeover / Wheel] and §Conventions). No boundary is widened: two refusals were added. The
  entry under-ran by evidence, not silently.
- The `v1-21` note is phase P5's, already written; no P7.3 write is owed.

## Checks over the whole set
- C2: no two proposals edit one section in opposing directions.
- C3: the report's deviations are each justified; the scope record holds no line; no intent amendment is needed.
- C4: every site was read by bounded window before its disposition (the architecture lines run to 9 545 chars).
- C6: the report's two disproved claims are plan and research statements, not a master's; both are routed to
  curation (the sweep hazard and the fake agent's `key` receipt).
- Boundary widening: none met. The moved bound loosens a stamped row's check; the founder ruled it on a card that
  showed the price (2026-10-07T10:29Z), and the body already recorded that decision.

## Escalations
None. No card of the founder's arose (the overseer's disposition 5 had nothing to hold).

## Totals
21 proposals: 13 applied as proposed (A1–A9, T7, Y1–Y3) · 8 rejected on the tell or with a rejected primary
(T1–T6, O1, O2), each of their facts raised by the orchestrator and applied · 0 escalated.
