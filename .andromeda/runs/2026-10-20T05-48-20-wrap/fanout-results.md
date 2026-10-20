# Fan-out results — 2026-10-19-newer-claude-cli-rows

The batch ran inside one snapshot pair:
- before: `tree 0805d7afb4f43ccd2a5d82182ce7e528d6ce1cd1 index 7f83e0cffcc61688ccba4ea05b9b954d7a0cb5b7 head
  4eaca1d3af32b3562ef00f58d41684e559100b5d`;
- after: `tree e63ee70bbbe9e497b111fa11b67bccfdd376550c`, with the index and head unchanged.

`git diff-tree` printed exactly seven `A` lines, one `fanout-{doc}.md` per agent. No fix was owed, so there is no
twin. Every receipt's count equals its file's.

architecture: 5 proposals · fanout-architecture.md
  note: no new library, runtime, pattern or resource; the `CLAUDE.md:42` leaf is the cascade's.
  1. apply — check 1, playbook "Accurate this-chunk addition" (routine). [Screen Model] signatures sentence: the two
     input-box literals and the measured 2.1.302 resting footer (expected amendment 1; disproved claim 1).
  2. apply — check 1, the same rule (routine). [Screen Model]: `input-box-signature` reads ready, turn and the long
     turn's settled screen (expected amendment 1; disproved claim 2).
  3. apply — check 1, the same rule (routine). [CLI Version Compatibility] @c19987: stamped versions 2.1.287 and
     2.1.302 (expected amendment 1).
  4. apply — check 1, the same rule and "A count in a master": the count's rule is the list itself at
     `architecture.md:315` ("the count's rule is this list"), read. Fake agent argv options 12 → 13, verify-serving
     8 → 9, `--long-turn-screen`.
  5. apply — check 1, the same rule (routine). §Occupied Resources → Repository (`:374`): committed sets gain
     `2.1.302`, stamped beside `2.1.287`. Its 29 = 4 spine + 3 screens + 12 dialog + 6 framing + 2 status line + 1
     resume + 1 agents, counted from the report's New text, by line.
security-plan: 0 proposals · fanout-security-plan.md
  note: expected amendment 3 still awaits its owner (raised below as S1).
  S1. escalate — check 5 (expected amendment 3) and check 1 ("Boundary widening the founder answered himself to a P4
      fork": escalate, one answer card). §Data Protection, next to "Accepted risk (verify's trusted runs …)": the
      footer reading's one scratch `viola run` start of 2.1.302, its pane's bottom three rows written to the chunk's
      `evidence/`, 0 transcripts measured. His own answer to Fork 2 is `inputs#I6`.
design-system: 0 proposals · fanout-design-system.md
layout-templates: 0 proposals · fanout-layout-templates.md
test-plan: 9 proposals · fanout-test-plan.md
  note: §10's zero-flakiness budget against the macOS criterion's routing to a WATCH (raised below as T10).
  1. apply — check 1, "Accurate this-chunk addition" (routine). §4 `:508`: the two literals and the three
     `resting_footer` cases (disproved claim 1).
  2. apply — routine. §4: the `input-box-signature` arm reads three screens; the three `long_turn` cases.
  3. apply — routine. §5 `:589`: the long-turn modal case and `--long-turn-screen`.
  4. apply — routine. §5 `:601`: stamped sets 2.1.287 and 2.1.302.
  5. apply — routine, with "A count in a master": the rule is architecture's list (`test-plan.md:960`, "the count's
     rule is architecture's list of them"). Twelve → thirteen argv options, the ten → eleven that follow, eight →
     nine for verify's interactive runs; `--long-turn-screen`.
  6. apply — routine. §7 `:968` fixture-hygiene walk: the 2.1.302 set.
  7. apply — routine, dependent of 6. §7 Seed strategies `:934`: the framing, status line, resume and agents
     variants stand under `2.1.287` and `2.1.302`. The 2.1.287 count of 34 is unchanged.
  8. apply — routine, dependent of 6. §2 Contract row `:379`: the dialog variants are 12 under each of `2.1.288`,
     `2.1.287` and `2.1.302`, and the framing variants six under each of `2.1.287` and `2.1.302`.
  9. apply — routine. §5: `wait_ready`'s test-side stall report, the macOS stall's witness (expected amendment 2).
  T10. escalate — check 3 (intent-consistency).
       - `test-plan.md:1121` (§10 Zero-flakiness budget): "For viola, quarantine means the chunk stays red until the
         root cause is fixed within that chunk."
       - The report's macOS criterion is UNMET (cause not named; 1 of 4 attempts).
       - The directive (`inputs#I8` item 4) routes it to a WATCH on «Epoch 5 cleanup» while this chunk flips
         complete.
       - A direction written for the route does not settle a quality-gate rule it does not name. So: escalate.
       - **Resolved** (`inputs#I9`): the founder's own word, «Закрыть + WATCH» (2026-10-20T06:33Z, relayed verbatim).
         Flip complete; the criterion stays UNMET; a WATCH on «Epoch 5 cleanup», the `wait_ready` stall report its
         witness. The departure from §10 is his word, and §10 is not amended (no body edit).

S1 resolved (`inputs#I9`): the answer card confirmed by the founder's own word, «Да, внутри» → apply.
obs-plan: 0 proposals · fanout-obs-plan.md
a11y-plan: 0 proposals · fanout-a11y-plan.md

Check 4 (absence needs evidence): every "no change" disposition in the report's Counts names its rows from the pre-pass
listing (`cascade-2026-10-19-newer-claude-cli-rows-pre-pass.json`). Lines over 2 000 chars were read by offset where a
fact stood on them (`architecture.md:45`, `:101`, `:110`, `:315`, `:374`; `test-plan.md:589`, `:960`).

Check 6 (disproved claims):
- 1 → architecture 1 and test-plan 1;
- 2 → architecture 2;
- 3 (the footer's 0 transcripts) → S1.

Rejected for a source the report does not carry: 0.

Raised in the cascade (same claim, other wording): `architecture.md:101` @c6385 and `security-plan.md:115` @c2816,
"rows that hold **an** input-box literal" — applied (routine, a follower of architecture 1;
`cascade-dispositions.md`).
