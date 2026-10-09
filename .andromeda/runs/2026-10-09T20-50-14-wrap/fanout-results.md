# Fan-out results — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

Seven doc-agents, one batch, each sent the letter's prompt verbatim with repo-relative paths. Detector counts per
prompt: architecture 2, security-plan 3, design-system 1, layout-templates 1, test-plan 3, obs-plan 3, a11y-plan 2;
sum 15, the drift-base's 15 `doc:` names. Every return read as YAML. No HTML entity was seen in any of them (the returns hold `<` and `>` as typed, as in
`Cow<'_, str>`), read directly from the hand-backs; no scripted decode was run. No raw twin is kept (no empty
return was changed by stripping, none failed the read).

Totals: 22 proposals (architecture 10, security-plan 4, test-plan 6, obs-plan 2); 22 applied; 0 rejected; 0 rejected
for a source the report does not carry; 5 carried `escalate` (security-plan 4 by the detector's severity,
architecture 1 by the agent's own word), each resolved on a recorded founder ruling, no halt.

## Verdict lines
- architecture — 10 proposals (D-arch-decisions 10; D-arch-resources none: no resource added).
- security-plan — 4 proposals (D-security-input 4; D-security-auth, D-security-deps none).
- test-plan — 6 proposals (D-tests-coverage 6; D-tests-framework, D-tests-obs-harness none).
- obs-plan — 2 proposals (D-obs-instrumentation 2; D-obs-stack, D-obs-pii none).
- design-system — `proposals: []`. Its note: `design-system.md:765`, the `empty-text` hint, stays true.
- layout-templates — `proposals: []`. Its note: `layout-templates.md:557` stays true (an inner CR is content and
  never makes a text empty).
- a11y-plan — `proposals: []`.

## architecture (10)
| # | section | change, in short | disposition |
|---|---|---|---|
| A1 | [Delivery Confirmation], the `typed_text` definition | every CR LF pair and every other CR as one LF, the ending removed, no CR; "and nothing else … a CR … not at the very end" retired | apply — check 1, the playbook's "Accurate this-chunk addition"; check 5, the plan's first entry. Applied without the proposal's `Cow` return type and its `answer` sentence: the body states no return type, and security-plan already says answer text is typed as validated |
| A2 | the same, the matching sentence | the gloss of the typed text follows the rule | apply — dependent of A1 |
| A3 | the same, the rulings sentence | the third ruling named (the founder, 2026-10-09T16:51Z, live in the overseer's dialog, the options shown, relayed by the operator); "that ruling covers the ending only" retired | apply — dependent of A1; check 5 |
| A4 | the same, the inner-case paragraph | recast as a record of the build before the rule; the landed rule, its tiers, the six live readings on 2.1.287, the control, the not-measured list; "no fix landed, carried on the working route" retired | apply — dependent of A1; check 6 (the report's first two disproved claims). Applied without the proposal's `text_bytes` figures (obs-plan's fact) and with the owed row's route entry named by title, on `inputs#I8` |
| A5 | [Human Takeover / Wheel] | the in-flight sentence covers a text with a CR or a CR LF inside | apply — dependent of A1 |
| A6 | §Conventions → Error handling schema, `empty-text` | the gloss of the typed text follows the rule | apply — dependent of A1; check 5, the plan's second entry. The proposal's added clause on what an empty typed text is was not written: the same line states it already |
| A7 | §Standard Contracts, `hook.event` | the gloss follows the rule | apply — dependent of A1; check 5 |
| A8 | §Standard Contracts, `prompt-submitted`, the matching gloss | the gloss follows the rule | apply — dependent of A1; check 5 |
| A9 | the same row, "as typed, not as sent" | also a send whose text held a CR or a CR LF inside | apply — dependent of A1 |
| A10 | §Cross-cutting Patterns, the capability ledger pattern (`escalate`) | one shape `send` relies on has no row: an LF typed inside a paste, measured on 2.1.287, its row owed | apply — the agent escalated for two reasons, and both are answered on disk. One: the no-row outcome was the overseer's P4 answer. Since then the founder, shown that the shape is relied on and has no row or probe, gave the row a route entry of its own (`inputs#I8`, 2026-10-09T20:58Z, live in the overseer's dialog, relayed). Two: the report wrote "(no master)" beside the route item. That was the report's wording for the route question, not a direction against master text; the plan's first expected amendment, approved at P5, names the fact for [Delivery Confirmation]. The sentence states the fact and its owner. It rules nothing about the requirement's reach and widens no boundary |

## security-plan (4, each `escalate` by D-security-input's severity)
The agent's own reading: no boundary is unvalidated and no input surface is added; the drift is the plan's wording.
Check 1: not the playbook's "Boundary widening" class by its subject. The set of accepted texts is unchanged, CR
and LF were already allowed, `validate_paste_text` is unchanged and first, no refused character is typed or
removed, and no channel gains a crossing. The change itself is the founder's ruling, given live with the options
shown (2026-10-09T16:51Z, relayed), and the plan's third expected amendment, approved at P5, names it. Each is
resolved on that ruling, no halt, and the sidecar records it as the founder's with the relay named.

| # | section | change, in short | disposition |
|---|---|---|---|
| S1 | §Input Validation, Paste text row | the typed text's rule; the class argued anew; the third ruling | apply — check 1 (above); check 5. Applied without the `Cow` return type |
| S2 | §Threat Model Summary, CLI input, Entry point | the definition follows the rule | apply — dependent of S1; the playbook's "Verbatim upstream copy kept current" |
| S3 | the same, Trust boundary | the delivery-confirmation gloss follows the rule, exact equality | apply — dependent of S1; the same rule |
| S4 | §Security Anti-Patterns § Input | the not-stripping carve-out names the inner rewrite | apply — dependent of S1; check 5 |

## test-plan (6)
| # | section | change, in short | disposition |
|---|---|---|---|
| T1 | §6 Path 2, "A text ending in newlines" | the inner outcome is pinned and confirmed; the live readings; "no test at any tier … no fix has landed" retired | apply — check 6; check 5. Applied without the `text_bytes` figures |
| T2 | §1 Path 2, the Path sentence | the rule restated | apply — dependent of T1; check 5 |
| T3 | §1 Path 2, the fake-agent receipt bullet | no CR in the receipt, an inner CR or CR LF as one LF | apply — dependent of T1. The proposal named no line, as the prompt allows; the site was read at `test-plan.md:238` |
| T4 | §4, the `typed_text` bullet | the rule; the table 17 → 14 and its new name; the second table (10), the borrow test, the property (512) | apply — dependent of T1; check 5 |
| T5 | §4, the matcher bullet | the gloss follows the rule; the inner-CR send cases and the two-case no-tolerance test named | apply — dependent of T1; check 5. The case count of five is `evidence/red-green.md`'s |
| T6 | §5, wrapper-side paste validation | the accepted half named with what it pins | apply — dependent of T1; check 5, the plan's fifth entry |

## obs-plan (2)
| # | section | change, in short | disposition |
|---|---|---|---|
| O1 | §4 Scenario: Confirmed `send`, `pty.paste_write` | the `text_bytes` gloss follows the typed text | apply — check 1 "Accurate this-chunk addition"; check 5, the plan's sixth entry |
| O2 | §6 Additive field catalog, the `send-*` row | the same gloss | apply — dependent of O1 |

## The six checks
1. Playbook: every applied proposal matched "Accurate this-chunk addition" or is settled by a recorded ruling
   (above). No two rules collided.
2. Cross-contradiction: none. The four masters now state the rule in one wording.
3. Intent-consistency: the report's deviations are justified (the companion edit is the plan's revision; the
   borrow is the same value). The scope record's one line serves the file it names. No widening line.
4. Absence: no proposal claims an absence. The caught-all claim is the sweep's, in `cascade-dispositions.md`.
5. Expected amendments: seven entries. Six are matched by proposals (above). The seventh, layout-templates
   "Refusals decided before any request", left the judgment to the wrap: read at `layout-templates.md:557`, it is
   true as written and is not amended.
6. Disproved claims: the first two are A4 and T1; the third is every gloss edit above; the fourth (plan step 12 as
   first written) names no master and was closed by the plan's revision.
