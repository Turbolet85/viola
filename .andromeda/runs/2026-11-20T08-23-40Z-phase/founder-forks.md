# Founder forks — «A quoted modal never blocks a send» (2026-11-20-a-quoted-modal-never-blocks-a-send), end of P3

The phase stops here, before P4, on the directive's item 3 (inputs#I1) and the operator's invocation (inputs#I2). No
compiled screen literal is printed below: they are named «the trust label», «the imports label», «the
browser-tools label» (`TRUST_LITERAL`, `IMPORTS_LITERAL`, `BROWSER_TOOLS_LITERAL`). Every code reference is
`doc:2026-11-20-a-quoted-modal-never-blocks-a-send/research` (rev 2) and the closed scope (rev 3).

## The block

**Window.** This window holds Setup, P1 (promoted, tx 3304), P2 (the row read: 775 of 4752 rows picked, 7 history
files, every check passed, one task writer re-spawned once) and P3 (research rev 2, scope closed at rev 3). P4 reads
the seven rows files whole, about 215 KB, and the seven history files, about 94 KB: it starts best in a fresh window,
which resumes this take-up at P4 (the `research` row stands; phase Setup 3 (c)), its forks answered by this file.

**The cause, as read in the code — the overseer's hypothesis confirmed.** `Screen::verdict`
(`crates/viola-agent-claude/src/screen.rs:330-359`) refuses `input-not-ready` as soon as ANY visible row CONTAINS a
modal label as a substring (`349-351`, over `Signatures::holds_modal`, `81-83`), before it reads the box. The grid is
the visible screen only (vt100 scrollback 0, `285`). So a transcript row quoting a label refuses every send until
later output scrolls it off. `pause` / `release` redraw nothing, a fresh launch clears the grid, and `--continue`
redraws the quote, which matches every measured fact of 2026-11-19T10:22Z. `input_box_up` (`105-108`) carries the
same «no row holds a modal label» clause.

**What a real modal looks like on the screens viola holds (no start):**
- the trust dialog, recorded on 2.1.287, .288, .302, .325 and .328: its label alone on its row at column 3. The
  recording blanks every other row, so the selector row and the frame are not kept. 2.1.325 and .328 also keep an
  80-char rule on row 1 with no prompt row under it. No box rows stand below the label on any of them.
- the browser-tools offer, on the rig's kept first screens (local, gitignored, 2.1.325 / .328 era): a 70-char top
  rule over a title, `❯` on the cancel row at column 2, the label alone on the next row at column 4, a confirm footer,
  and no box below.
- the imports dialog: no recorded screen at all. It exists only as synthetic unit rows (`screen.rs:1008-1016`): the
  cancel row with `❯`, the label alone at column 3, and an input-box FOOTER literal under it.
- the suite's synthetic browser-tools screen (`tests/support/fake.rs:163-178`) draws the box's OWN rule and prompt
  rows BELOW the offer. That is not what the real frame shows, but it is the test that pins «a modal refuses even
  with the box up» (`tests/cli_send.rs:1301-1345`).

**The candidate discriminators, each with the modal it could miss:**
- **(1) Option shape**: the label row is an option row — indent, an optional selector `❯ `, an optional `N. `, then
  the label ending the row. Every known modal reads standing: trust (recorded, column 3), browser-tools (rig, column
  4; synthetic, column 2), imports (synthetic, column 3), and the fake agent's selected form. Misses: a CLI build that
  puts other text on the label's row, such as a «(recommended)» tail or a shortcut hint. Defeated by: a quote that
  stands alone on its own line. An assistant message's continuation rows are indented 2, so a one-line quote or a
  code-block line holding only the label is option-shaped and stays refused.
- **(2) Box below**: the box's own rows (a rule of at least 40 directly above a row opening `❯`) stand below the
  label row. Every RECORDED and kept modal reads standing: trust has no box below on any set, and the browser-tools
  rig frame has none. Misses the suite's synthetic browser-tools screen, which has the box below by construction,
  and any build that draws a dialog above a live box. A footer literal alone never counts as the box, so the imports
  synthetic rows (footer under the dialog) still read standing. Defeated by: a message that itself draws a 40-char
  rule at column 0 over a row opening `❯` at column 0. That is UNMEASURED: no recorded screen holds a transcript, and
  that an assistant message never opens a row at column 0 with `❯` is an inference from the 2-column indent of its
  continuation rows, not a reading. If the box is down (paste hint, notice), a quote reads standing and refuses as
  today.
- **(3) Selector adjacency**: a `❯` on the label row or on the row directly above or below it. Unverifiable on the
  recorded trust screens, whose selector row is blanked, so on its own it would MISS every recorded trust screen.
  Set aside as a test of its own; usable only inside (2).
- **(4) Cell attributes** (colour, inverse, bold of the label row): no recorded screen keeps attributes. Proving
  them needs a live start. Set aside.
- **(5) The modal's frame or title**: blanked on the recorded sets; only the rig frame shows it. Set aside, for the
  same reason as (4).

**What a message can still do, under the recommendation (A below):**
- a quote at a row's start inside a sentence: goes;
- a quote as the last transcript row, the box under it: goes unless option-shaped;
- a quote scrolled to where a modal would draw: position plays no part, the rule reads the box below and the row's
  shape;
- a narrow terminal that wraps the label: no row holds it whole, so nothing is read, as today. A real modal wrapped
  the same way still refuses on «no box up» at the bound, as today;
- a code block or a one-line quote holding only the label: refused, as today. This is the residual, and the
  workaround in force (never print a label in a chat message) stays useful for that one shape.

**The conservative side when the gate cannot tell: refuse, as today.** A false refusal costs a retry and a human
look. A false send types into a dialog, where Enter takes the highlighted option (trust, imports, browser tools):
that is the dangerous side the directive names.

**`status`, `list` and the dialog relay.** There is no `status` verb in `src/cmd/`. `viola list` reads the board
(`dialog_pending` from hook events) and the dialog relay is the hooks: neither reads the screen, so the fix leaves
them apart by construction. The same `input_box_up` is also:
- verify's paste guard (`src/cmd/verify/typed/turns.rs:75-77`);
- a ledger post-condition (`ledger.rs:1062-1064`);
- the rig's box read.

They move with it, which keeps the security master's «one predicate decides for verify and for viola run's pre-send
gate». Verify's settle end (`pty.rs:139-141`) and the first-screen word (`first_screen`) stay on the raw presence:
neither types into a transcript screen.

**Ledger rows.** None changes and no probe changes. The modal-signature row reads a label's presence on verify's
recorded modal screen (`ledger.rs:1086`). `input_box_ready` moves with the predicate, and no recorded `ready` or
`turn` screen of any set holds a label, so no stamp reading moves.

**Fixtures.** No new recorded set. Each test that needs a quoting screen builds a SYNTHETIC one beside a scratch copy
of the recorded sets, named as synthetic, as `tests/cli_send.rs` already does for the browser-tools offer. Nothing
is committed into `fixtures/claude/<version>/`.

**Master rows — a widening, said as one HERE.** The fix admits a screen the gate refuses today, so it widens a
control, and these rows' words change:
- architecture `decision/screen-signatures-compiled` («a screen is ready when no row holds a modal literal …»);
- architecture `decision/vt100-readiness-gate` («requires no known modal signature»);
- architecture `contract/send-results/input-not-ready` («a modal on a verified CLI»);
- security-plan `decision/input-box-readiness-from-literals` («never while a modal literal stands»).

Each becomes «… no STANDING modal row», with the rule written as compiled shapes over compiled literals. That keeps
security-plan `decision/compiled-classification-sources` («screen signatures … come only from compiled …
ledger rows»): the shape is compiled, never read from upstream text.

**A live start: none wanted.** The fake agent's `--start-screen`, the recorded trust screens and synthetic screens
carry both directions without one. What no start-free reading can prove, named as unmeasured:
- the imports dialog's real screen on any version;
- the trust dialog's selector row and frame;
- any cell attribute.

A start could record those, but under the recommendation none of them decides the gate. So none is asked for, and
there is no cap and no spare.

**One implement window holds the chunk.** The change is:
- one predicate's body in `screen.rs`;
- tests in `screen.rs`, `src/run/gate.rs`, `tests/cli_send.rs` and `tests/support/fake.rs`;
- no signature change and no caller threading (the graph: 37 call sites, none changed).

Both directions are carried as tests:
- a quoting screen on an idle session: the send passes the gate, read from the receipt's `key` lines and the absence
  of an `input-not-ready` refusal;
- each known modal standing, the recorded trust screens and the synthetic browser-tools and imports screens: refused
  `input-not-ready` with nothing typed;
- the remove-the-guard control run on the REFUSAL side: the option-shape and box-below clauses neutralised, so the
  refusal cases go red.

**Carries.**
- `rig_rehearsal_reads_the_box_empty_after_x_and_del` (cry-zbogma7z) is acted on only at its next red.
- WATCH cry-ekw6oxvp (`stop_presses_ctrl_c_again_when_the_first_is_lost`) is read at each run. No recurrence has
  been read: CI ci#44700108922 on `9e2b7e7d` read green.

## Q1 — Which discriminator releases a quoted label? (the founder's: it decides what the gate admits)

1. **(A) Option shape AND box below (Recommended).** A label row reads as a quote only when it is NOT option-shaped
   AND the box's own rows stand below it. Every other label row is a standing modal and refuses, as today.
   - Cost: a quote standing alone on its own line (one-line quote, code block) is still refused. The workaround
     stays for that shape.
   - Why it wins: a modal is missed only if a build BOTH puts other text on the label row AND draws the box below
     it. Every recorded and kept screen fails both conditions, and the suite's synthetic box-below screen still
     refuses on shape.
2. **(B) Box below alone.**
   - Releases more: a stand-alone quote line goes too.
   - Cost: it MISSES the suite's synthetic browser-tools screen (box below) by construction, so
     `send_on_a_verified_cli_refuses_while_the_browser_tools_prompt_is_up` would turn from refused to sent. It also
     rests the imports dialog on synthetic rows alone.
3. **(C) Option shape alone.**
   - Cost: a build that adds text to the label row while a footer literal stands (the imports synthetic shape)
     would read ready and type into the dialog. This is the dangerous side, so not recommended.
4. **(D) (A) widened by selector adjacency** (option-shaped rows released when no `❯` stands on or beside them, the
   box below).
   - Releases stand-alone quotes too.
   - Cost: adjacency cannot be read on the recorded trust screens (selector row blanked). Its safety rests on (2)
     alone there, and on synthetic and rig frames for the rest.

## Q2 — Ratify the widening of the four master rows above? (a widening is the founder's)

1. **Yes, as (Q1's answer) words it (Recommended).** The four rows' wording changes at this chunk's wrap. The
   security row keeps «one predicate decides for verify and for viola run's pre-send gate». Cost: the gate admits a
   screen it refuses today, bounded by Q1's rule.
2. **No.** The chunk cannot meet its entry: the gate stays as it is, and the chunk ends with its scope unmet and is
   cancelled or re-scoped. Cost: the workaround stays the only defence.

**Stop.** The phase stops here, at the end of P3, before P4, and asks no dialog. The answer is appended to this file
in the run-dir spelling, snapped, and cited where the plan rests on it. P4 then resumes on it.

## The answers — the founder's own words, given in the overseer's dialog and read at 2026-11-20T09:08:17Z

Relayed by the overseer in `~/dev/projects/additional/viola-overseer/relay-modal-p3.md` (snapped beside this file),
appended here verbatim from it:

1. **Q1: «A: форма И поле ниже (Recommended)».** Means (A) exactly as your block words it: a label row reads as a
   quote only when it is NOT option-shaped AND the input box's own rows stand below it; every other label row is a
   standing modal and refuses `input-not-ready`, as today. He was shown the residual in these words: a quote
   standing alone on its own line, or a code-block line holding only the label, still refuses, and the workaround
   stays for that shape. (B), (C) and (D) are declined.
2. **Q2: «Да, в словах выбранного правила (Recommended)».** His ratification of the widening of the four rows your
   block names — architecture `decision/screen-signatures-compiled`, `decision/vt100-readiness-gate`,
   `contract/send-results/input-not-ready`, security-plan `decision/input-box-readiness-from-literals` — in (A)'s
   words and nothing wider: the security row keeps «one predicate decides for verify and for viola run's pre-send
   gate», and the rule stays compiled shapes over compiled literals.

Authority: the founder's own answer (operator), relayed verbatim by the overseer.

## Q3 — the further rows (A) reaches, raised at P4 (the founder's: a boundary widening)

P4 resumed on Q1 and Q2 and wrote the plan (`doc:2026-11-20-a-quoted-modal-never-blocks-a-send/plan` rev 1). The P4
reading of the masters' rows found SIX more rows whose words the option-shape clause reaches. The P3 block above
named four rows, and Q2 ratified those four «and nothing wider». So these six were never shown to him, and the
escalate rule `rul-p2zl2kgl` holds that only his own word, given after a widening is shown, ratifies it. The a11y
history file had already flagged the first one: «If it reads a row's text, position or context beyond the compiled
signatures, that is a new widening of the row, owed to the founder» (`a11y-history.md`, item 2).

**What reaches them.** (A)'s option shape reads the label row's own leading spaces, an optional selector glyph, an
optional `N.` numbering, and that nothing but spaces follows the label. That is a compiled shape over the compiled
literal, not a literal alone. Verify's paste guard reads the same predicate (Q2 kept «one predicate»), so verify's
rows are reached too. The six rows:
1. a11y-plan `decision/no-screen-content-parsing` — «parsed only for compiled signatures (footer literals, the input
   box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it»;
2. test-plan `decision/screen-signatures-only` — «evaluated only for compiled input-box and modal literals as the
   readiness gate's verdict»;
3. security-plan `decision/verify-reads-only-signatures` — «verify's PTY output is read only against the compiled
   signature literals»;
4. security-plan `decision/verify-pty-output-caught` — «… read only against compiled signature literals»;
5. security-plan `decision/verify-screen-read-against-literals` — «read only against compiled signature literals»;
6. security-plan `decision/verify-signature-only` — «read only against the compiled signature literals».

Each would read, at this chunk's wrap: «compiled signatures: the literals, and the compiled shapes over them (a modal
label row's option shape; the box's rows below it)». Nothing wider: no row text is kept or returned, no reading
leaves the agent crate, and the rule stays compiled shapes over compiled literals.

1. **Yes, these six in the same words (Recommended).** The plan stands as written. Its stop rule lifts and
   `/andromeda-implement` may run after the P5 word. The six rows are reworded at the wrap beside Q2's four.
   - Cost: no behaviour beyond Q1's rule. The masters say what the code reads.
   - Why it wins: the substance is Q1's rule, which he chose. These rows only carry the same rule's words in the
     masters that the P3 block missed.
2. **No.** The option shape cannot be built as (A) defines it. The plan returns to P4.
   - Cost: either the chunk ends with its entry unmet, or it is re-scoped to a rule that reads literals alone, which
     Q1 already declined as (B). The workaround stays the only defence.

**Stop.** The phase stops here, at the end of P4, before P5, and asks no dialog. The plan is written and held behind
its stop rule. The answer is appended to this file in the run-dir spelling, snapped, and cited. The next window
resumes at P5 (phase Setup 3 (a): a `plan` row, no `matrix-2026-11-20-a-quoted-modal-never-blocks-a-send.json`).

## The answer to Q3 — the founder's own word, given in the overseer's dialog and read at 2026-11-20T09:32:06Z

Relayed by the overseer in `~/dev/projects/additional/viola-overseer/relay-modal-q3.md` (snapped beside this file),
appended here verbatim from it:

**Q3: «Да, те же слова в шести строках (Recommended)».** He was shown the six rows by name — a11y-plan
`decision/no-screen-content-parsing`, test-plan `decision/screen-signatures-only`, security-plan
`decision/verify-reads-only-signatures`, `decision/verify-pty-output-caught`,
`decision/verify-screen-read-against-literals`, `decision/verify-signature-only` — and why the option shape reaches
them (the label row's indent, selector, numbering and nothing after the label; verify's paste guard on the same
predicate). This is his ratification of that widening for those six rows in the words of your Q3 option 1 and
nothing wider: no row text kept or returned, no reading leaving the agent crate, compiled shapes over compiled
literals.

Authority: the founder's own answer (operator), relayed verbatim by the overseer. Appended at 2026-11-20T09:32:34Z.

## Q4 — five more rows (A) reaches, found at P5 (the founder's: a boundary widening)

Written 2026-11-20T09:35:11Z. Q3's relay asked that any OTHER master row the rule reaches be named as a fork, not
folded. Before the card, P5 searched all seven masters with `rec.py find`. The search covered the modal-literal,
signature-literal and compiled-literal wordings, plus «for a modal», «a modal on» and «while a modal». Beyond Q2's
four rows and Q3's six, it found five more whose words the rule reaches. None was shown to him, so rule
`rul-p2zl2kgl` holds them for his own word. The plan stands at `doc:2026-11-20-a-quoted-modal-never-blocks-a-send/plan`
rev 2, held behind its stop rule on this question.

**The five rows:**
1. architecture `contract/human-hint-lines/input-not-ready`: the hint is printed «for a modal, a never-quiet screen,
   a poisoned screen or a hint past the bound». Q2 already rewords its twin, `contract/send-results/input-not-ready`
   («a modal on a verified CLI»), to «a standing modal». This row would read «for a standing modal» the same way. The
   hint text itself does not change.
2. test-plan `decision/no-child-screen-assertions`;
3. test-plan `decision/never-parse-rendered-child-screen`;
4. test-plan `decision/discipline-at-every-layer`;
5. test-plan `decision/no-screen-parsing`.

Rows 2–5 each carry the standing live rig's exception: the rig «reads the child's screen into closed words, by
compiled literals and the box row's cell attributes, never text». The rig's box read is `SIGNATURES.input_box_up`
(`crates/viola-e2e/src/rig/screen.rs:273`). That is the same predicate, which moves with the gate, so the rig would
read by compiled shapes too. Each exception would grow by adding the new reading beside the named ones, never by
folding them into a wider class: «by compiled literals, the compiled shapes over them (a modal label row's option
shape; the box's rows below it), and the box row's cell attributes, never text».

**Read as not reached** (named so he can add any of them):
- architecture `decision/run-never-reads-screen-content`: no row text leaves the screen, and the shape is a boolean
  over compiled characters;
- architecture `decision/verify-box-wait-for-literal`: verify's settle end stays on raw presence;
- test-plan `decision/no-assertion-beyond-compiled-literals`: the new cases assert verdicts, never content;
- obs-plan `decision/deterministic-emission`: its words hold as they stand;
- architecture `decision/screen-fixtures-signature-rows-only` and security-plan
  `decision/screen-fixture-signature-rows-only`: no fixture changes;
- security-plan `decision/compiled-classification-sources`: it is kept.

1. **Yes, these five in the same words (Recommended).** The plan stands as rev 2 writes it. Its stop rule lifts, and
   `/andromeda-implement` may run after the P5 word. The five rows are reworded at the wrap beside Q2's four and Q3's
   six.
   - Cost: no behaviour beyond Q1's rule.
   - Why it wins: row 1 is Q2's send-results twin. Rows 2–5 describe the same predicate Q2 kept as «one
     predicate», read by the rig.
2. **No.** The plan returns to P4. The rig's box read would have to stay on raw label presence, through a separate
   read in the agent crate, so a second predicate stands beside the one Q2 kept. Row 1's words stay as they are.
   - Cost: a second readiness reading, which the rule «one predicate» argues against.

**Stop.** The phase writes the P5 card with this fork on it and stops there, asking no dialog. The word on the card
approves the plan, but `/andromeda-implement` does not start until his own word on Q4 stands on record.

## The answer to Q4 — the founder's own word, given in the overseer's dialog and read at 2026-11-20T09:43:21Z

Relayed by the overseer in `~/dev/projects/additional/viola-overseer/relay-modal-q4.md` (snapped beside this file),
appended here verbatim from it:

**Q4: «Да, теми же словами (Recommended)».** He was shown the five rows by name — architecture
`contract/human-hint-lines/input-not-ready` («for a standing modal», the hint text unchanged) and test-plan
`decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`,
`decision/discipline-at-every-layer`, `decision/no-screen-parsing` (the live rig's exception growing by adding
«the compiled shapes over them» beside the named readings, because the rig's box read is the same predicate) —
and that you read seven other rows as not reached, which stay as they are. This is his ratification of the
widening for those five rows in the words of your Q4 option 1 and nothing wider. With Q2's four and Q3's six,
fifteen rows are reworded at the wrap.

Authority: the founder's own answer (operator), relayed verbatim by the overseer. Appended at 2026-11-20T09:43:24Z.
The plan's stop rule lifts on this recorded word.
