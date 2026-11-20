# Answer card — the wrap of «A quoted modal never blocks a send» (2026-11-20-a-quoted-modal-never-blocks-a-send)

Q1 of this wrap's halt, put to the overseer as a dialog (the operator away, inputs#I13). No compiled screen literal is
printed here.

## The founder's own answers it rests on (relayed verbatim by the overseer)

- Q1 «A: форма И поле ниже (Recommended)» — rule (A): a label row is a quote only when it is not option-shaped AND the
  input box's own rows stand below it; every other label row is a standing modal and refuses. Residual shown to him: a
  quote alone on its own line, or a code-block line holding only the label, still refuses (inputs#I4).
- Q2 «Да, в словах выбранного правила (Recommended)» — the four rows, in (A)'s words and nothing wider; the security row
  keeps «one predicate decides for verify and for viola run's pre-send gate» (inputs#I4).
- Q3 «Да, те же слова в шести строках (Recommended)» — the six rows: «compiled signatures: the literals, and the
  compiled shapes over them (a modal label row's option shape; the box's rows below it)» (inputs#I7).
- Q4 «Да, теми же словами (Recommended)» — the five rows: «for a standing modal»; the rig's exception grows by «the
  compiled shapes over them» beside the named readings (inputs#I10).

## The operations, each inside those answers (15 rows, 17 operations)

Old → new, each cell as it stands and as it lands.

1. ops-main-architecture.md · decision/screen-signatures-compiled · rule · fit: fits type 1 · Q2
   - old: … a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do …
   - new: … a screen is ready when no row is a standing modal and a footer literal stands or the box's own rows do …
2. ops-main-architecture.2.md · decision/screen-signatures-compiled · except (new) · fit: fits type 2 · Q1 + Q2
   - old: (none)
   - new: A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a
     quote alone on its own line, or a code-block line holding only the label, still refuses.
3. ops-main-architecture.md · decision/vt100-readiness-gate · rule · fit: fits type 1 · Q2
   - old: … requires no known modal signature and waits …
   - new: … requires no standing modal and waits …
4. ops-main-architecture.md · contract/send-results/input-not-ready · promise · fit: fits type 3 · Q2
   - old: … or a modal on a verified CLI makes send type nothing …
   - new: … or a standing modal on a verified CLI makes send type nothing …
5. ops-architecture.md · contract/human-hint-lines/input-not-ready (the proposer's) · promise · fit: fits type 3 · Q4
   - old: … for a modal, a never-quiet screen …
   - new: … for a standing modal, a never-quiet screen … (the hint text unchanged)
6. ops-main-security-plan.md · decision/input-box-readiness-from-literals · rule · fit: fits type 1 · Q2
   - old: … and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate.
   - new: … and never while a standing modal is up; one predicate decides for verify and for viola run's pre-send gate.
7. ops-main-security-plan.2.md · decision/input-box-readiness-from-literals · except (new) · fit: fits type 2 · Q1 + Q2
   - old: (none)
   - new: the same words as row 2.
8. ops-main-security-plan.3.md · decision/verify-reads-only-signatures · rule · fit: fits type 1 · Q3
   - old: verify's PTY output is read only against the compiled signature literals.
   - new: verify's PTY output is read only against the compiled signatures: the literals, and the compiled shapes over
     them.
9. ops-main-security-plan.3.md · decision/verify-pty-output-caught · rule · fit: fits type 1 · Q3
   - old: … and read only against compiled signature literals.
   - new: … and read only against compiled signatures: the literals, and the compiled shapes over them.
10. ops-main-security-plan.3.md · decision/verify-screen-read-against-literals · rule · fit: fits type 1 · Q3
    - old: verify's children's screens are read only against compiled signature literals, and their row text …
    - new: verify's children's screens are read only against compiled signatures: the literals, and the compiled shapes
      over them, and their row text …
11. ops-main-security-plan.3.md · decision/verify-signature-only · rule · fit: fits type 1 · Q3
    - old: … are read only against the compiled signature literals, their row text …
    - new: … are read only against the compiled signatures: the literals, and the compiled shapes over them, their row
      text …
12. ops-main-test-plan.2.md · decision/screen-signatures-only · rule · fit: fits type 1 · Q3
    - old: The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict;
      content beyond the compiled literals is never asserted.
    - new: The screen is evaluated only for compiled signatures as the readiness gate's verdict: the input-box and modal
      literals, and the compiled shapes over them; content beyond the compiled literals is never asserted.
13. ops-main-a11y-plan.md · decision/no-screen-content-parsing · rule · fit: fits type 1 · Q3
    - old: … compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), …
    - new: … compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled
      shapes over them), …
14. ops-main-test-plan.md · decision/no-child-screen-assertions · except · fit: fits type 2 · Q4
15. ops-main-test-plan.md · decision/never-parse-rendered-child-screen · except · fit: fits type 2 · Q4
16. ops-main-test-plan.md · decision/discipline-at-every-layer · except · fit: fits type 2 · Q4
17. ops-main-test-plan.md · decision/no-screen-parsing · except · fit: fits type 2 · Q4
    (rows 14–17: §The shortened members)

Row 5 is listed under its file: it is `ops-architecture.md`'s n=4, the proposer's own set, kept as written.

## The shortened members (a cell bound, never a dropped member)

Each except GROWS by the new reading beside the named ones (compiled literals · the compiled shapes over them · the box
row's cell attributes · never text); the bound forces the words of named members shorter:

- no-child-screen-assertions (200): old «The forced vt100 panic case byte-searches the outer terminal; the test-only
  live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never
  text.» → new «The forced vt100 panic case byte-searches the outer terminal; the test-only live rig reads the screen
  as closed words (compiled literals, their compiled shapes, box-row cell attributes), never text.»
  - forced vt100 panic case … outer terminal: kept · test-only live rig: kept · «the child's screen» → «the screen» ·
    «into closed words» → «as closed words» · compiled literals: kept · NEW «their compiled shapes» · «the box row's
    cell attributes» → «box-row cell attributes» · never text: kept.
- never-parse-rendered-child-screen and no-screen-parsing (200): old «The test-only live rig reads the child's screen
  only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal
  asserts on.» → new «The test-only live rig reads its child's screen only as closed words, by compiled literals, their
  compiled shapes and box-row cell attributes, never text, which its rehearsal asserts on.»
  - test-only live rig: kept · «the child's» → «its child's» · «into» → «as» · compiled literals: kept · NEW «their
    compiled shapes» · «the box row's cell attributes» → «box-row cell attributes» · never text: kept · which its
    rehearsal asserts on: kept.
- discipline-at-every-layer (200): old «The test-only live rig reads the child's screen into closed words, by compiled
  literals and the box row's cell attributes, never text, and passes a read trust dialog under its key grant.» → new
  «The test-only live rig reads its child's screen as closed words by compiled literals, their compiled shapes and
  box-row cell attributes, never text, and passes a read trust dialog under its key grant.»
  - as above; «passes a read trust dialog under its key grant»: kept.
- a11y no-screen-content-parsing (300): the new member «the compiled shapes over them» added in the signatures list;
  its parenthetical «(a modal label row's option shape; the box's rows below it)» left out to the bound. Every other
  member kept word for word.

Q4's words «the compiled shapes over them» read «their compiled shapes» in the three except rows; F4 of
`founder-forks.md` names the shortening.

## Named beside the card, not asked

- `not_master` type 4, rejected before the question (their shorter sets are rows 8–12 above): security-plan
  `verify-reads-only-signatures`, `verify-pty-output-caught`, `verify-screen-read-against-literals`,
  `verify-signature-only`, test-plan `screen-signatures-only` — the parenthetical «(a modal label row's option shape;
  the box's rows below it)» read as the code's. `founder-forks.md` F3.
- Two sixteenth rows, not folded, their most conservative option in force (`founder-forks.md` F1, F2): test-plan
  `decision/readiness-gate-tested-by-refusals` (unchanged) · obs-plan `decision/standing-modal-reading-uninstrumented`
  (not added).
- The seven rows the P5 card read as not reached stand as they are.

## The answer

As given, verbatim (inputs#I15, delegate overseer, PROVISIONAL):

Answer card (Q1 of the wrap's halt): «Confirm (Recommended)». Notes: Overseer's delegate confirm, read at 2026-11-20T10:42:02Z against answer-card.md and the founder's recorded answers Q1-Q4: all 17 operations on the 15 rows lie inside them; the shortened members were read old beside new, no member dropped. F1-F4 stand as their conservative options and go to the epoch batch. PROVISIONAL until the founder's own word.
