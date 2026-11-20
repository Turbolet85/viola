# P5 card 1 — «A quoted modal never blocks a send» (2026-11-20-a-quoted-modal-never-blocks-a-send)

Written 2026-11-20T09:40:27Z. The plan is `doc:2026-11-20-a-quoted-modal-never-blocks-a-send/plan` rev 3 (tx 3323).
To open it in an editor, use its render at `.andromeda/cache/docs/2026-11-20-a-quoted-modal-never-blocks-a-send/plan.md`.

- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-a-quoted-modal-never-blocks-a-send`
  diffs the chunk's records.

No compiled screen literal is printed here. They are named «the trust label», «the imports label» and «the
browser-tools label».

## ⚠ A fork comes first: Q4 (the founder's)

The Q3 relay asked: «if any OTHER master row's words are reached by the rule, name it on the card as a fork — do not
fold it» (`inputs#I6`).

P5 searched all seven masters with `rec.py find`. It covered:
- the modal-literal, signature-literal and compiled-literal wordings;
- «for a modal», «a modal on» and «while a modal».

Beyond Q2's four rows and Q3's six, it found **five more** rows the rule reaches. Q4 is written to
`.andromeda/runs/2026-11-20T08-23-40Z-phase/founder-forks.md`. It is not folded.

1. architecture `contract/human-hint-lines/input-not-ready`. Its words are «… for a modal, a never-quiet screen …».
   This is the twin of Q2's `contract/send-results/input-not-ready`, and it would read «for a standing modal». The
   hint text does not change.
2. test-plan `decision/no-child-screen-assertions`
3. test-plan `decision/never-parse-rendered-child-screen`
4. test-plan `decision/discipline-at-every-layer`
5. test-plan `decision/no-screen-parsing`

Rows 2–5 hold the standing live rig's exception: «by compiled literals and the box row's cell attributes, never
text». The rig's box read is `SIGNATURES.input_box_up` (`crates/viola-e2e/src/rig/screen.rs:273`), and it moves with
the predicate. Each exception would grow by adding the new reading beside the named ones: «by compiled literals, the
compiled shapes over them (a modal label row's option shape; the box's rows below it), and the box row's cell
attributes, never text».

Q4 options:
1. **Yes, the same words (Recommended).**
2. **No.** The plan returns to P4, and the rig keeps a second, raw-presence reading.

**The plan is held behind its stop rule on Q4** (plan §Metadata). A `yes` on this card approves the plan and links
the ledger. `/andromeda-implement` still does not start until his own word on Q4 stands on record. A delegate's
answer, a direction or this card's confirm does not lift it (`rul-p2zl2kgl`).

Read as **not reached**, and named so he can add any of them:
- architecture `decision/run-never-reads-screen-content`: no row text leaves the screen; the shape is a boolean over
  compiled characters.
- architecture `decision/verify-box-wait-for-literal`: verify's settle end stays on raw presence.
- test-plan `decision/no-assertion-beyond-compiled-literals`: the new cases assert verdicts, never content.
- obs-plan `decision/deterministic-emission`: unchanged.
- the two screen-fixture rows (architecture `decision/screen-fixtures-signature-rows-only` and security-plan
  `decision/screen-fixture-signature-rows-only`): no fixture changes.
- security-plan `decision/compiled-classification-sources`: kept.

## Goal

`viola run`'s pre-send gate stops reading a quoted modal label as a standing modal. A label row reads as a quote, and
so admits the send, only when BOTH hold:
- the row is NOT option-shaped;
- the input box's own rows stand BELOW it.

Every other label row refuses `input-not-ready` exactly as today. This is the founder's rule (A) (Q1, `inputs#I4`).

The change is one predicate's body in `viola-agent-claude`: `Signatures::standing_modal`, read by `input_box_up` and
`Screen::verdict`. So the predicate moves for every reader that decides a typed byte:
- `run`'s gate;
- verify's paste guard;
- the ledger post-condition `input_box_ready`;
- the rig's box read.

Verify's settle end and `first_screen` stay on the label's raw presence. No relied ledger row, probe, refusal word,
log line or committed fixture changes.

## The option shape, as readings (`inputs#I3` item 4)

A row is option-shaped when, left to right, it reads:
1. **Indent:** any run of spaces, from zero up to the row's width.
2. **Selector (optional):** the compiled prompt glyph, then any run of spaces.
3. **Numbering (optional):** one or more ASCII digits, a `.`, then any run of spaces.
4. **The literal itself.**
5. **«The label ENDS the row»:** only spaces after it.

Any other character makes the row not option-shaped.

**Box below:** the box's own rows stand strictly below the label row. That means a rule of at least `BOX_RULE_MIN`
rule chars directly above a row opening the prompt glyph. A footer literal alone never counts (`inputs#I3` item 5).

Refusal tests for each clause boundary:
- `option_shape_reads_each_clause_boundary`: every clause, both sides.
- `option_shape_a_shaped_label_with_the_box_below_still_refuses`: a shaped row WITH the box below still refuses.
- `box_below_a_quoted_label_without_the_box_below_still_refuses`: a row with a trailing word and NO box below still
  refuses. The cases are no row below, a footer only, the box only above, and a rule with no prompt row.

## The catch test, the measured instance (`inputs#I3` item 3)

The 10:22 screen is built as closely as the tree can state it, without reading any transcript. The shape comes from
a redacting read of the sealed card that was quoted, which printed classes, never text:
- the trust label sits inside a sentence, mid-row;
- `"` stands directly before and after it;
- the idle box's own rows stand below.

The label's first word is joined to its opening `"`, so however the TUI wraps the line, the row holding the whole
label has that `"` before it. Under (A) that row is never option-shaped. **So the measured instance goes under (A),
and the chunk meets its entry: no fork on this point.**

The shape is tested at two levels:
- unit: `quoted_label_with_the_box_below_reads_ready`;
- E2E: `send_on_a_verified_cli_goes_past_a_quoted_modal_label`, which confirms the send through `send-issued`,
  `prompt-submitted{origin:"driver"}`, `send-confirmed` and no `send-refused`.

## Top acceptance criteria

- **Delivered:** the quoting screen with the box below reads ready, and the E2E send is confirmed.
- **Standing:** every known modal still refuses `input-not-ready` with nothing typed:
  - the trust label on all five recorded modal screens;
  - the fake agent's trust form;
  - the browser-tools offer with the box below;
  - the imports and browser-tools synthetic rows;
  - every shaped form × literal.
- **Controls** (implement's one-shot `gate.py control`): each reads `holds`.

  | Control | Side | Entry | Timeout |
  |---|---|---|---|
  | `shape-off` | refusal | 4 | `--timeout 1200` |
  | `below-off` | refusal | 5 | `--timeout 1200` |
  | `shape-off-e2e` | refusal | 8, builds the workspace | `--timeout 2400` |
  | `raw-presence` | delivered | 9, builds the workspace | `--timeout 2400` |

  A `timeout` reading is a red to read, never `holds`. Ids are within 16 characters (`inputs#I3` item 6).
- **No stamp moves:** every stamped recorded set stamps with zero fails over the fake, with no recording start
  (`inputs#I3` item 7). Entry 10's baseline is green: 123 passed, the ledger-probe contract included.
- **Preservation:** `gate.py guard` reads `guard: clean` over fixtures, schemas, `viola-core`, `ledger.rs` and the
  fake agent.
- **Whole layers and CI:** the integration and E2E layers return `"ok":true`, and CI reads `verdict: green`.

## Touchpoints
- `crates/viola-agent-claude/src/screen.rs`: `option_shaped`, `box_below` and `standing_modal`; `input_box_up` and
  `Screen::verdict` read them; the unit cases.
- `src/run/gate.rs`: two gate cases.
- `tests/support/fake.rs`: `quoting_screen`, synthetic.
- `tests/cli_send.rs`: the delivered E2E.
- No new file and no signature change. The 37 call sites are untouched.

## Test commands, with their baselines (P5 check 4 (9))

There are 18 `[[gate]]` entries. `gate.py baseline` read 7 `seen green`, 7 `new (key)` and 4 `leg operator`. Every
`new` entry was baseline-run on the untouched tree:

| # | Entry | Baseline |
|---|---|---|
| 4 | nextest `test(/option_shape/)` | red — vacuous: no tests to run, 0 of 663 selected (exit 4); this chunk's cases |
| 5 | nextest `test(/box_below/)` | red — vacuous: the same reading; this chunk's cases |
| 7 | integration `cli_send \| cli_verify` | green — 90 passed, 0 failed |
| 8 | integration `browser_tools_prompt_is_up` | green — 1 passed: the refusal case selected |
| 9 | integration `quoted_modal_label` | red — vacuous: 0 of 574 selected, `nextest-exit-4`; this chunk's case |
| 10 | e2e: ledger probes, fixture hygiene, drift, feed panic | green — 123 passed |
| 12 | `gate.py guard` (newly `new = true`: it read NOT SEEN) | green — `guard: clean` |

Entry 12's known-positive control was fired as the entry in a shared scratch clone at `9e2b7e7d`:
- unmodified clone: clean;
- clone with a `ledger.rs` edit: red, `guard: outside 1 files`.

It is recorded in `.andromeda/runs/2026-11-20T08-23-40Z-phase/guard-control.md`.

The reds of entries 4, 5 and 9 are on an empty input, so they prove only that each entry can read red. Each entry's
control is implement's red-before-green run.

## Validation
- **Mechanical checks:**
  - `gate.py run --dry-run` parses 18 entries, with no env line to resolve.
  - `planlint`: `hits: 0`. Check 12 (d) lists one claim (line 85, «unchanged» on `Screen::verdict`). It was read: it
    is step 5's own claim about the rest of `verdict`, and it holds.
  - `inputs.py verify`: 8 entries, drifted 0, uncited 0, unparsed 0.
  - `gate.py scope --lists-only`: `parsed 4 items`.
  - The modify paths exist, and none is a master.
  - Mechanism reach was read at `screen.rs:81-83, 105-108, 345-352`, `turns.rs:75-77`, `ledger.rs:1062-1064, 1086`,
    `rig/screen.rs:273` and `typed/pty.rs:139`.
- **The whole set was re-run** after each of this window's plan edits (rev 2, the Q3/Q4 edit; rev 3, the baseline
  keys).
- **Validation-1:** aligned with `doc:…/scope` rev 3 and the working entry.
- **Capabilities:** none claimed. None of the 17 pooled caps names a gate reading, and the entry carries no
  `CARRY: proves`.
- **Carries:**
  - cry-enjonrk7 is the substance.
  - cry-zbogma7z is acted on only at its next red.
  - WATCH cry-ekw6oxvp is read at each run, from its log before any mend.

## The directive's item-10 block (verbatim where a tool printed it)

**The union** (`rec.py rows-union`):
```
architecture: rows 968 · packs 10 · picked 151 (79 by every draw) · standing 30 · 46636 B
security-plan: rows 623 · packs 7 · picked 82 (41 by every draw) · standing 15 · 26483 B
design-system: rows 318 · packs 4 · picked 51 (30 by every draw) · standing 0 · 13569 B
layout-templates: rows 223 · packs 3 · picked 46 (26 by every draw) · standing 3 · 10087 B
test-plan: rows 1355 · packs 14 · picked 227 (122 by every draw) · standing 50 · 61170 B
obs-plan: rows 1030 · packs 11 · picked 182 (116 by every draw) · standing 27 · 45778 B
a11y-plan: rows 235 · packs 3 · picked 36 (23 by every draw) · standing 19 · 11422 B
rows-union: 7 master(s) · picked 775 of 4752 · rule any
```
The rows files total 215,145 B, under the 250,000 B bound, so P4 read them whole.

**Is every cited row in the union or its standing list?** Not quite. 36 of the 37 master rows the plan and this card
cite are in their master's rows file. One is not: architecture `decision/screen-fixtures-signature-rows-only`
(`dec-wbpw2h6h`). It is named only in the not-reached list above, and P5's `rec.py find` found it. Two more names are
not master rows but escalate rules, cited as rules: `rul-p2zl2kgl` and `rul-bznresq5`.

**`[RETYPED]` lines and re-spawns:**
```
security-pick-p2.d3 · [RETYPED] line 2: contract/strict-modes-refusal/cli-frame-verb is no row of the pack — read as contract/strict-modes-refusal/cli-frame-verbs, the pack's row 2
a11y-pick-p2 · [RETYPED] line 63: decision/time-bounds-fall-on-the-driver-only is no row of the pack — read as decision/time-bounds-fall-on-the-driver, the pack's row 63
obs-task · [NOT_NEEDED_REACHED] 'Revive preflight and the claude agents probe and its parse-rejected lines: the c… (117 chars)' names fixtures/claude
VERDICT: 6 of 7 pass · failed: obs-task [NOT_NEEDED_REACHED]
re-spawned 1 agent(s) · NOT_NEEDED_REACHED 1 · wall 67 s
```
The re-spawned task, batch `task-r1`, passed 1 of 1.

**Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
- The baseline reds of entries 4, 5 and 9 above (each tool call exited 0, and the entries read red).
- The guard control's planted red.
- `rec.py query` exit 2 on a guessed table name `row` (`Catalog Error`). It was re-read through `rec.py get`.
- The Bash guard refused three calls, each re-issued once in the form it passes:
  - a `cd` into a references directory;
  - a `cd` into the run dir;
  - one `rec.py get` loop piped into `head`.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed this window.

**Where a letter was wrong or silent:**
- The letter's §Revision says a resumed P5 in a run dir holding no `route-{marker}.json` is a revision's. This dir
  holds it, so this is a take-up resume (Setup 3 (a)), as read.
- `gate.py show` reads only the trail's last run. So the batched baseline readings were read from the entries' own
  logs under `$TMPDIR/andromeda-gate/…/phase-2026-11-20T08-23-40Z/`, not through `show`.
- Validation.md check 4 (9) asks for a known-positive control for a green guard, but says nothing on whether a
  NOT SEEN probe takes `new = true` before or after its baseline. It was set with the baseline, as the letter's «P5
  sets `new = true` on it».
- The validate evolve checkpoint and step 5's land fire on a P5 outcome. No word is taken in this window, so neither
  has fired yet.

## Beyond the plan
none

---

**Apply? (yes / review / cancel)**

A `yes` approves rev 3 and links the ledger (no cap claimed). `/andromeda-implement` then waits on Q4's own word.
