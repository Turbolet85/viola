# Fan-out results — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

Seven doc-agents, one batch, each sent the prompt of `amendment-flow.md` with its doc, the report and its detectors
(the prompts as sent: `fanout-prompt-{doc}.md` in this dir; 15 detectors over 7 docs). Returns arrived inline.

**Transport reading.** No HTML entity was seen in any return (`&lt;` · `&gt;` · `&amp;`: 0). Three returns
(architecture, security-plan, test-plan) arrived with a harness notice that control tags were neutralised (`<` then a
backslash before tag-like text), so a tag in their `change` lines may read as escaped when the agent wrote it plain.
No applied text is taken from a `change` line: every body edit is re-derived from the report. Each `change` below is
the orchestrator's condensation of the returned line; the `sidecar` lines are not kept (the entries are authored
after the sweep).

## Verdicts
- architecture — 15 proposals (11 warning, 4 escalate)
- security-plan — 2 proposals (warning); stripped: two comment blocks (detectors with no proposal; two sites noted for the orchestrator)
- design-system — `proposals: []`; stripped: a comment naming `design-system.md:764` (the hint line's advice) as outside its detector
- layout-templates — `proposals: []`
- test-plan — 4 proposals (warning); stripped: a comment block of clean detectors and checked sites
- obs-plan — 1 proposal (warning, dependent); stripped: a header of three holding detectors
- a11y-plan — `proposals: []`; stripped: a comment naming the keystroke clause (`a11y-plan.md:99`, `:624`, `:990`) to re-check when the hint remedy is built

## architecture
| # | detector | sev | section | change (condensed) | disposition |
|---|---|---|---|---|---|
| A1 | D-arch-decisions | warning | [CLI Version Compatibility] → the long-paste wrapper (the frame; what the unwrap removes) | frame after the close: one newline at the prompt's end, two before typed text, three between pairs; the unwrap also removes the second when non-frame text follows | **apply** — check 1 routine (accurate this-chunk addition), check 5 (expected), check 6 (disproved 1). Its stay-list is corrected on apply: a second newline after the close at the prompt's end still stays (the report's Symbols bullet; the retained case) |
| A2 | D-arch-decisions | warning, dependent | [Delivery Confirmation], the normalisation parenthetical | the frame parenthetical gains the second newline | **apply** — dependent of A1 |
| A3 | D-arch-decisions | warning, dependent | §Standard Contracts → `prompt-submitted`, the frame parenthetical | the same | **apply** — dependent of A1 |
| A4 | D-arch-decisions | warning | the long-paste wrapper (the id clause; the closing "Unmeasured" sentence) | one id per session; the three shapes measured; a text's last newline never reaches the hook | **apply** — check 5, check 6 (disproved 2, 5) |
| A5 | D-arch-decisions | **escalate** | the long-paste wrapper (a new closing sentence) | the paste-then-typed and two-pair frames are held by unit cases only; no `viola verify` run types them; an open divergence from the ledger rule | **escalate** — no playbook rule; a relied-on CLI shape with no probe is the operator's call. Resolved below |
| A6 | D-arch-decisions | warning | Tag escaping (the start-of-prompt sentence; the cross-session exception) | start of prompt measured; the cross-session message arrived unescaped under three attributes; the escaped form relayed only, kept compiled | **apply** — check 5, check 6 (disproved 3, 4) |
| A7 | D-arch-decisions | warning, dependent | Harness prompt prefixes | the plain cross-session prefix measured here; the forward reference to this chunk removed | **apply** — dependent of A6 |
| A8 | D-arch-decisions | warning, dependent | [Human Takeover / Wheel], the `HARNESS_PREFIXES` parenthetical | "the escaped form is the one the CLI injects" no longer stands | **apply** — dependent of A6; check 4: read by offset, `architecture.md:70` @c2528 of 7 436 chars, the sentence stands there as quoted. A site the report's own sweep missed |
| A9 | D-arch-decisions | **escalate** | [Delivery Confirmation] (a measured exception to "every send is confirmed") | a sent text ending in a newline is delivered, runs a turn, is filed `human`, moves the wheel, and ends `not-delivered` / `no-prompt-submitted`; not fixed | **escalate** — it qualifies a locked decision's headline and is not on the plan's list. Resolved below |
| A10 | D-arch-decisions | escalate, dependent | `prompt-submitted` (the "`text` is the pasted text" clause) | the clause notes the dropped last newline | **escalate** with A9 |
| A11 | D-arch-decisions | escalate, dependent | [Human Takeover / Wheel] (the "never move it" parenthetical) | a driver send ending in a newline is taken as a human prompt | **escalate** with A9; check 4: `architecture.md:70` @c2172 read |
| A12 | D-arch-decisions | warning | the `viola verify` paragraph (the paste-hint passage) | the live readings replace "no `send` was run"; the founder's decision recorded as made and unbuilt | **apply** — check 1 (the operator's recorded direction, inputs#I6), check 5, check 6 (disproved 7, 8) |
| A13 | D-arch-decisions | warning, dependent | the `viola verify` paragraph ("owed to …") | both rows measured, unlanded, still owed, no entry minted | **apply** — check 5 |
| A14 | D-arch-resources | warning | §Occupied Resources → Environment variables, the provided-by-Claude-Code line | `CLAUDE_ENV_FILE` and `CLAUDE_PROJECT_DIR` registered as names the CLI hands a hook process | **apply** — routine: the line enumerates such names (two stand there); not registry over-reach |
| A15 | D-arch-resources | warning | §Occupied Resources → Environment variables, the identity floor's origin | the Linux reading beside the Windows origin; the floor's list unchanged | **apply** — check 5, check 6 (disproved 9); `IDENTITY_FLOOR` does not move (the overseer's disposition 2) |

## security-plan
| # | detector | sev | section | change (condensed) | disposition |
|---|---|---|---|---|---|
| S1 | D-security-auth | warning | §Secret Management → Storage (the R8 / floor sentence, `:452`) | the Linux reading beside the Windows origin; the floor unchanged | **apply** — dependent in substance of A15 |
| S2 | D-security-auth | warning | §Threat Model Summary (the inherited-credential bullet, `:42`) | the two scratch-probe measurements recorded as the founder's live answers of 2026-10-07T09:43Z, relayed by the overseer | **apply** — check 5 (expected). A boundary-widening record: ratified by the founder's own live answers, each given after its widening was shown (inputs#I3), which is what the playbook's "what ratifies it" rule asks; the sidecar names him and the relay. The verbatim-copy section is kept current (the 2026-10-04 rule) |

## test-plan
| # | detector | sev | section | change (condensed) | disposition |
|---|---|---|---|---|---|
| T1 | D-tests-coverage | warning | §4 → viola-agent-claude, the normalisation bullet (`:564`) | the frame clause and the case list (the two `live_shape` tables, the property's tail) | **reject as proposed, raised by the orchestrator** — its basis cites source lines the report does not carry (the re-derivation tell); the fact is the report's (Symbols bullet, disproved 1) and the plan's expected amendment, so check 5 raises it routine |
| T2 | D-tests-obs-harness | warning | §3 → 5-command implementation, the `--local-live` bullet | the list names all seventeen ids | **reject as proposed, raised by the orchestrator** — its basis cites a source location the report does not carry; the plan's expected amendment, raised under check 5, the three ids as the report names them |
| T3 | D-tests-coverage | warning | §4 → viola-agent-claude, the R8 strip bullet (`:559`) | the Linux reading beside the literal eleven; the literal unchanged | **apply** — routine, in short form (the full reading has one home, architecture) |
| T4 | D-tests-coverage | warning, dependent | §1 → the inherited-credentials-strip entity (`:86`) | a pointer to the Linux reading | **apply** — dependent of T3 |

## obs-plan
| # | detector | sev | section | change (condensed) | disposition |
|---|---|---|---|---|---|
| O1 | D-obs-instrumentation (a nominal slot; no invariant violated) | warning, dependent | §4 → Edge flows → E2 (`:725`) | the Linux reading beside the Windows origin | **apply** — dependent in substance of A15, in short form. The same sentence says the strip "always keeps" the floor, against architecture `:380` and security-plan `:452` ("is removed whatever the persistent set says"): a cross-master contradiction on the amended claim, corrected in the same edit under cascade step 2. The agent's source citation for it is not used |

## Not proposed, dispositioned (check 6)
- Disproved 8 (the `input-not-ready` hint's advice; `design-system.md:764`): the line is unchanged and still the shipped string; the reading goes into A12 and rides the route (P5).
- The a11y keystroke clause: unchanged; a re-check for the chunk that builds the hint remedy (P5).

## Escalations and their resolutions
Both were raised in one halt and answered by the overseer in the session.
- **A9, A10, A11 — the newline false negative in three sections.** Chosen: "Record in all three". The overseer's
  note: the bodies say what the product does today, measured and unfixed, each with its evidence pointer; the remedy
  stays the STOP 5 route card. → **applied** in [Delivery Confirmation], `prompt-submitted` and [Human Takeover /
  Wheel].
- **A5 — the frame beside typed text has no `viola verify` probe.** Chosen: "Record it, add a route card". The
  overseer's note: record the two frames as compiled on the 2026-10-07 measurement, unit-held and unprobed, and make
  it the fourth route-resolve card; probe-or-ratify is the founder's (a new Run B paste is a widening, an accepted
  limit is a ruling on the ledger rule), so the card's options go to him priced. → **applied** in the long-paste
  wrapper bullet; the card is P5's.

No playbook rule is proposed: neither resolution names a class that recurred (a locked decision's measured
exception; an unprobed relied-on shape), and the second is the founder's to rule on at its card.

## After apply
22 proposals dispositioned: 20 applied as proposed or in corrected form (A1's stay-list; T3, T4 and O1 in short
form), 2 rejected as proposed and raised by the orchestrator (T1, T2). Four masters and one key file changed:
architecture (5 sidecar entries), security-plan (1), test-plan (1), obs-plan (1). Cascade: `cascade-dispositions.md`;
three leaves re-derived. Open escalations: 0.
