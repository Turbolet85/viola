# P5 review card — 2026-11-28-console-actions, revision 1 (Part 2 on his Q8 and Q9)

**Plan:** `doc:2026-11-28-console-actions/plan` rev 5, rendered to `.andromeda/cache/docs/2026-11-28-console-actions/plan.md`
to open in an editor. No plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-28-console-actions` diffs the
chunk's records.

**Inputs:** the stop, inputs#I4 (the invocation); his answers to Q8 and Q9 and the revision's six orders, inputs#I5
(`../additional/viola-overseer/relay-console-actions-q8q9.md`, copy). The operator is away: no dialog was raised.

## Goal
`viola console` runs every verb of matrix#v2-15 plus switch from a selected row, each as a child `viola` of its own exe.
Part 1 (steps 1–9) stands BUILT and green in the tree, uncommitted; `/andromeda-implement` re-enters at **step 10**.
Part 2: the editing line, send, switch, open a child, answer — on his Q8 (permission and plan by `y`/`n`, a question
left to the session) and his Q9 (the echo as typed, UTF-8, under one width rule).

## Part 2, step by step (plan steps 10–17)
- **10 editing mode** — `Mode::Editing`; Char appends, Backspace removes one scalar, Enter submits, Escape cancels,
  Ctrl-C ends; `q` `j` `k` and the verb letters are text (Part 1's departure: they decode as `Key::Char`,
  `src/console/keys.rs:117`, and only the list mode gives them meaning, `src/console/mod.rs:219-221`); bounded at
  `MAX_FRAME`; Enter, Escape or Ctrl-C ends the read, so a CR in a paste ends the line.
- **11 one `console-key` line per read** — the editing read's keys fold by class: at most one `text` line with
  `count`, one `backspace` with `count`, one `unbound` with `count`, then the line of the key that ended the read: at
  most four lines a read, never one per character. `count` lands in the `console-key` arm of
  `schemas/diag-line.v1.json` (line 99, `$ref` the existing `#/$defs/count`), `key` `text` requiring it; the event
  doc at `crates/viola-core/src/obs.rs:12` names it. `diag-detail.v1.json` (no per-event field arm) and
  `tests/contract_diag_schema.rs` (event list only) do not move; obs-plan `contract/diag-line-fields/console-key` is
  the wrap's.
- **12 the echo and its width rule** — `unicode-width =0.2.2`, already in `Cargo.lock` under `vt100` alone, made a
  direct dependency (no crate added). The cases:
  | typed | drawn | sent |
  |---|---|---|
  | Cyrillic `привет` | as typed, 6 columns | the 12 bytes as typed |
  | CJK `漢字` | as typed, 4 columns (2 a scalar) | as typed |
  | `e` + U+0301 | one column, the mark on the `e`; Backspace removes the mark alone | as typed |
  | past the right edge | the prompt, `...`, the longest tail that fits; a wide scalar straddling the cut left out whole, a leading width-0 scalar with it | the whole text |
  | an invalid byte sequence | nothing (the decoder reads it `Unbound`) | nothing; it counts in the read's `unbound` line |
  The editing line is drawn as fitted; every other line keeps the board's cut unchanged.
- **13 send** — `s`, `send <name>: `, Enter → `viola send <name>`, the text on stdin; its mirror lines drawn as its readback.
- **14 switch and open** — `m`: each word an effort word → `--effort=<w>`, else `--model=<w>` (the `=` form keeps a
  typed word a value); `o`: `viola open <typed> --parent <name> -- claude`, the typed text one argv element (a
  flag-shaped one leaves the required name missing: clap's usage error, exit 2).
- **15 answer** — the board item gains `dialog_id`, `#[serde(skip)]` as `dialog_kind` is (`crates/viola-state/src/board.rs:208-209`):
  `list`, `list --json` and the MCP list item stay byte-identical. **What the console reads to know the kind: the
  board's `dialog_kind`, the snapshot's word `permission` · `plan` · `question` (`snapshot.rs:109-112`), and the id
  from the same snapshot record.** `a` on a permission → `answer <name> permission? y allow  n deny`; on a plan →
  `answer <name> plan? y approve  n revise`; `y`/`n` run `viola answer <name> <id>` with `{"behavior":"allow"|"deny"|
  "approve"|"revise"}` on stdin; Escape answers nothing. On a question → one line `<name>  question - answer it in the
  session (f)`, key word `question`, no child. **No question, option, plan or tool text reaches the screen or a line:
  the console never reads `events.ndjson`; it reads the kind word and the id alone.**
- **16 key line** — `q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open`.
- **17 cases** — units `editing_…`, `editing_width_…`, `action_(send|switch|open|answer)_…`, the board case asserting
  the id; twelve `tui_console` cases (send ×3, editing line ×2, switch ×2, open, answer ×4); the Ctrl-C case gains the
  editing and answer modes.

**Part 1's four departures, read in as the tree stands:** the refused-action case over a frozen stale wrapper
(`tests/tui_console.rs:677`; a gone instance draws no row, `board.rs:113`); the scroll case at 80x5
(`tests/tui_console.rs:542-548`); `q` `j` `k` as plain `Key::Char`; the two-column gutter `SELECTED_GUTTER` / `GUTTER`
(`src/human/fleet.rs:30-37`).

## Top acceptance criteria
- `a` answers a permission and a plan through `viola answer`, and on a question draws one line and starts no child,
  no question or option text in any line: `console_answer_permission_by_y`, `console_answer_plan_by_n`,
  `console_answer_question_draws_one_line_and_runs_nothing`.
- The echo under the width rule and the text sent unchanged: `editing_width_…`, `console_send_a_cyrillic_line_as_typed`.
- One `console-key` line per read: `editing_a_read_of_text_writes_one_text_line`,
  `console_editing_line_logs_one_text_line_per_read`; the console-key probe reads `count,key`.
- `s` · `m` · `o` through their verbs: the part-2 e2e filter. No key trap, every mode: the Ctrl-C and escape cases.
- The harness's unit, integration and e2e layers `"ok":true`.

## Touchpoints (delta +3 −0 against rev 3)
Added: `Cargo.toml`, `Cargo.lock` (the `unicode-width` edge), `src/human.rs` (companion re-exports, already touched by
Part 1). Part 2 also edits `src/console/{mod,action,view}.rs`, `src/human/{console,fleet}.rs`,
`crates/viola-state/src/board.rs`, `crates/viola-core/src/obs.rs`, `schemas/diag-line.v1.json`, `tests/tui_console.rs`.

**Research:** lists synced to the revision's delta (+3 −0), rev 3; not re-derived since the take-up's rev 2.

## Test commands (23 entries; P5 checks)
New at this revision, baselines run on the tree the stopped run left (Part 1 in it):
- 4 `cargo nextest run -p viola-state` — green, 334 / 334.
- 6 the Part-1 unit filter (its exclusion now `action_(send|switch|open|answer)_`) — green, 47 / 47.
- 7 the Part-2 unit filter (`console::(tests::editing_|action::tests::action_(send|switch|open|answer)_)|human::console::tests::editing_width_`) — red, exit 4, 0 selected: vacuous until written.
- 14 the console-key `count` probe — red (`key` alone); green over a copy carrying step 11's field
  (`.andromeda/runs/2026-11-28T15-25-27Z-phase/controls-p5.md`; the copy stands in the scratchpad, `ctl-count/`).
- 15 `cargo deny check` — green, 0.75 s.
Carried with their take-up baselines: 1–3, 5, 8–13, 16; 17–19 seen green; 20–23 the operator leg.
Checks: `gate.py run --dry-run` 23 parsed · `gate.py baseline` not seen 0 · `planlint` 0 hits (12 (d): six «unchanged»
claims, each read: they name what stays as drawn or serialised while the file changes — consistent) · `inputs.py verify`
5 entries, 0 drifted, 0 uncited · `scope --lists-only` parsed 16 · `scope --sync` PASS +3 −0. Controls: Part 1's four
held; Part 2 adds `text-fold` and `flag-value`.

## The claim — matrix#v2-15, GATED, with his dated note (concretization preview; written at step 4 after the word)
- **Now:** «Every viola verb (open a child, focus, send, answer, last, pause, release, revive) is reachable from a
  selected session by keyboard alone, each acting through the same CLI verb with the same outcome and refusal as when
  typed at a shell.»
- **Proposed concrete form:** «From a selected wrapped row of `viola console`, by key bytes alone into a real PTY: `o`
  and a typed name open a child (`viola open <name> --parent <row> -- claude`), `f` focus, `s` and a typed line send,
  `a` answer (a permission `y` allow / `n` deny, a plan `y` approve / `n` revise; on a question one line naming the
  session and no child — the dated note), `l` last, `p` pause, `r`·`y` release, `v`·`y` revive (`viola up <name>`), and
  `m` and words switch; each runs that CLI verb as a direct child of the console's own exe, with the verb's own record,
  exit and refusal equal to the verb run at a shell, and one `console-action{verb, exit_code}` line: the part-1 and
  part-2 e2e filters of `binary(tui_console)` green over the fakes. GATED: the same on his Hyprland host, against his
  real claude and terminal, at a moment he names.» The question's narrowing is his note, not a weakening the plan
  chose; the requirement line is not reworded.
- **The note's exact text** (`matrix.py note`, after the word):
  `2026-11-28 (2026-11-28-console-actions, phase P5): claimed on the operator's own answer to Q8 (chunks/2026-11-28-console-actions inputs#I5, the arm «y/n, вопросы — в сессии»): the console answers a pending permission (y allow, n deny) and a pending plan (y approve, n revise) through viola answer; a pending question is left to the session, where a draws one line naming the session and f focuses it. So v2-15 is not fully met by the console in 0.2.0: its «answer a pending dialog» holds for permissions and plans, not for questions. The requirement line is unchanged: his answer chose this dated note, not a rewording. GATED on his live reading.`
- **What the fakes prove:** every key decoded from bytes into a real PTY; each of the nine actions run as a child of the
  console's exe under a test-owned home, its record and exit equal to the shell verb's over the fake agent, the fake
  foot and the fake hyprctl; every refusal drawn as captured; one `console-action` line each; the question line with no
  child and no text; the echo's width rule (units); no key trap.
- **What needs his live reading (GATED):** focus raising his real window; open and revive opening real terminals on
  his desktop; send's readback against his real claude (up to 53.5 s); answer on a real permission and a real plan;
  switch restarting his real claude; the echo as his terminal draws Cyrillic and CJK.
- **The block, verbatim** (`--gated-file`, and the route's `BLOCKED-ON` on this chunk's line at step 4):
  `BLOCKED-ON: the operator's live reading of viola console's actions on his Hyprland host — open a child, focus, send, answer (a permission and a plan), last, pause, release, revive and switch from a selected row, against his real claude and terminal — at a moment he names (chunks/2026-11-28-console-actions/inputs/I1-phase-console-actions-directive.md.txt)`
- Affordance honesty: met — the acceptance presses real keys into a real PTY, never a programmatic proxy.
- **matrix#v2-31** stays unclaimed (its «takes effect for its next turn» is the real CLI's, «The fleet on the founder's
  host»); its take-up note stands. No earlier claim of this chunk exists to un-claim (`coverage` lists 0 rows).

## Wrap amendments — whose word each lands on
| master · row | change | word |
|---|---|---|
| design-system · decision/list-is-the-board; layout-templates · decision/board-vs-strip | console acts on the selected strip | his own, F1/F2 |
| security-plan · + decision/console-runs-verbs-as-children | the child runner (a boundary widening) | his own, F1 (the wrap halts once, his card) |
| security-plan · + decision/console-action-child-outlives-quit | a child may outlive a quit | his own, F1 |
| design-system · contract/fleet-console-keys/* (unbound, the F3 letters, confirm) | Part 1's keys | his own, F3/F5 |
| design-system · + contract/fleet-console-keys/answer | permission and plan by y/n, question one line | behaviour his own, Q8; letter `a` and prompt copy **his, owed** (P1, P4) |
| design-system · + /open · /send · /switch | the three editing verbs | send his own, F4; letters `o`, open's form, switch's word rule **his, owed** (P1–P3) |
| design-system · contract/fleet-console-view/more-line + /above-line | the body follows the selection | his own, F6 |
| a11y-plan · decision/cli-no-interactive-input · cli-not-keyboard-interactive · static-linear-output; obs-plan · decision/no-interactive-terminal-interface | the console's except: selection, confirm, answer prompt, editing line | his own, F3–F6 and Q8 |
| layout-templates · decision/console-frame-floor | `q quit` first in the key line | builder's lean, PROVISIONAL |
| layout-templates · decision/console-own-surface + one width row | the echo as typed, the named exception, the width rule | his own, Q9; the `...` lead **his, owed** (P5) |
| security-plan · decision/console-key-reader-bounded | the decoder classes, the ESC wait, the editing bound, a CR ending the read | delegate overseer (technical), PROVISIONAL |
| obs-plan · contract/diag-line-fields/console-key + /console-action, obs Decisions Log entry | the key words (`question` among them), `count`, one line a read; the new event | one line a read: the overseer's order (inputs#I5 item 3); the `backspace`/`unbound` fold and `question`: delegate (technical), PROVISIONAL |
| test-plan · decision/harness-log-fields-owned-by-test-plan | console-action and the count among the grepped fields | delegate (technical), PROVISIONAL |
No amendment is written as his where his word is owed; none of the five is taken provisionally.

## Log volume per hour
No writer loop or timer is added; nothing scales with the fleet (33 instances or one).
- `console-key`: one line a key read in the list, confirm and answer modes; at most four a read in the editing mode.
  A typed key is its own read, so unbroken typing at 5 keys a second writes about 18 000 lines an hour, about 2.9 MB
  at about 160 B a line (estimated from the line's fields, not measured). A 10 KiB paste: at least 160 reads, at most
  640 lines (it was 10 240 at a line a character).
- `console-action`: one a finished child, one child at a time, bounded by the human's keys; the question line writes none.
- Each child verb's own `cli-<name>.ndjson` lines, as at a shell. The agents thread's 720 pairs an hour unchanged.

## Widening and product choices outside his answers
- **No widening** is met: every child is F1's crossing, the echo his Q9 exception, the answer his Q8 arm; architecture
  `decision/option-text-never-in-human-line` and security-plan `decision/question-option-text-crosses-boundary` do not
  move. (Escalate rows read: `rul-r74hshif`, `rul-p2zl2kgl`, `rul-o2tr55aj` — the Q8/Q9 hold was lifted by his own
  word, inputs#I5 —, `rul-bznresq5`.)
- **Five product choices, his, owed** — `.andromeda/runs/2026-11-28T15-25-27Z-phase/founder-forks.md`: P1 the
  letters `a` and `o`; P2 open's form; P3 switch's word rule; P4 the answer prompts' words and `a` with nothing pending
  reading `unbound`; P5 the `...` lead of a cut echo. Each is planned as its recommended arm and built so; none is
  written as answered by him.
- **Technical, the delegate's:** the direct `unicode-width` dependency (alternative: a width table in viola's code,
  rejected as a second rule beside vt100's); the key word `question`; the `backspace`/`unbound` fold.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`) No route entry is inserted (his F2/F7: one entry).
- (standing — `.andromeda/runs/2026-11-28T13-46-51Z-phase/p5-card-1.md`) adopt from the console is not owned by any entry. If he wants it, it is a new route entry on his word: its proposed
  title would be «Console adopt», placed after «Console actions».
- ENDED — the earlier card's row «Q8 and Q9 (`founder-forks.md`, the P4 block) — his to answer. Part 2 waits on both,
  and so does v2-15's claim.»: answered by him, inputs#I5.
- The five product choices P1–P5 of this run's `founder-forks.md` stay his, owed: the wrap writes each as built, not
  ratified, and carries it on the handoff for him.

Apply? (yes / review / cancel)
