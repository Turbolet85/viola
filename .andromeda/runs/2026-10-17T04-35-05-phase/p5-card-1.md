# P5 review — 2026-10-17-cli-output-discipline · CLI output discipline

Plan: `viola-0.1.0/chunks/2026-10-17-cli-output-discipline/plan.md` (559 lines, 20 steps, 21 gate entries, 13 acceptance
criteria). Mechanical checks: all pass. The fence parses, planlint reads 0 hits, the research lists parse (14 items),
the snapshot citations read 0 unparsed (scope's other-chunk citations re-spelt by path), and the nine new entries are
baseline-run green on the untouched tree. Prose is 416 lines, a little past the ~400 guide (no warn band). Validation-1:
aligned with `scope.md` and the working entry.

## Goal
Every human-mode CLI output follows the terminal discipline. It is linear and static. Colour appears only beside its
word and only on a terminal that takes it. `--help` is grouped. Long unwrapped names are clipped on a narrow terminal.
Under `viola run` there are zero wrapper bytes and human keys pass. Half of this already stands and is re-run, not
rebuilt.

## What it builds
- **Grouped `viola --help`** (steps 1–4): `Usage: viola [--home <dir>] <verb> [args]`, then `board: list` ·
  `traffic: send, wait, last, answer` · `wheel: pause, release` · `setup: run, revive, verify`, then `Options:`.
  Served verbs only (`inputs#I1` Q3, the overseer's technical answer, provisional). Pinned by literal.
- **Unwrapped-NAME truncation** (steps 5–7, 17): the width comes from `viola_pty::host_size()` with no env read
  (`inputs#I1` Q2, provisional). Only unwrapped names are cut, with `...`, never inside a `\xHH` escape. Piped output
  and `--json` stay whole.
- **The colour layer** (steps 8–16; CARRY 2), on the founder's own «Читать их» (`inputs#I2`):
  - the four SGR tokens at three depths;
  - the seven-rung decision as a pure function;
  - reads of `NO_COLOR` (present, empty included), `TERM` and `COLORTERM`, and nothing else;
  - the Windows VT enable (`0x0004` alone, saved and restored);
  - on the board: bold NAME, the amber `DIALOG` word with its kind word plain, and a dim stale row with the cocked
    composition.
  The colour facts never touch escaping, a refusal, a log or a control.
- **Linear readings** (step 18): no ESC and no bare CR on `wait`'s and `--help`'s terminal streams (Unix).
- **Re-run** (step 19): zero wrapper bytes, the key during a send's readiness wait, focus/mouse/resize, `waiting:`
  once, the `wait`/`last` escaping, verify's step lines.

## Top acceptance criteria
- `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`. The completeness case stays green over
  `Options:`.
- `cross_list_rows_style_each_span_beside_its_word_on_a_terminal`: SGR only on NAME (bold) and `DIALOG` (attention).
  No dim on a live board. Depth by `COLORTERM` / `TERM`.
- `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json`.
- The `colour_` and `board_row_style_` units: each rung, each depth, the design bytes, the stale × cock composition,
  and escaping that is the same at every depth.
- `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it`, plus the `board_row_width_`
  units.
- Every gate green here, and CI `verdict: green` on all three OSes.

## Touchpoints
- New: `src/human/style.rs`, `tests/support/sgr.rs`.
- Modified: `src/cmd/mod.rs`, `src/human.rs`, `src/human/board.rs`, `src/cmd/list.rs`, `tests/cli_output_plain.rs`,
  `tests/support/mod.rs`, `tests/support/events.rs`, `tests/cli_answer.rs`, `tests/cross_list_rows.rs`,
  `tests/cli_wait_last.rs`, `tests/cli_controls_not_disableable.rs`, `.github/workflows/windows-mutants.yml`.

## Test commands
fmt · clippy · the Windows-target clippy · G1 · lint-probes · `run --unit` and its `board_row_|colour_` filter ·
`run --integration` and its five-binary filter · `run --e2e` and `binary(cross_list_rows)` · the boot smoke · G2 · G4 ·
pre-push. Then, for the operator: hygiene, the clean-tree push, and the CI read.

## Capability claim — concretization preview
**`v1-40` CLI and terminal output discipline.**

Current acceptance: "On all three OSes CLI output is linear and static (one waiting line on a TTY only, no redraw or
cursor movement) and carries no colour or glyphs under non-TTY, no-colour, dumb-terminal or machine-readable output;
every colour span sits beside its word; injected control characters in wait, last and list output appear only
escaped; verify appends step lines and ui prints its launch line once; under run viola adds zero bytes to the
terminal, human keys pass unblocked during a driver send, and focus, mouse and resize never move the wheel."

Proposed concrete form: on ubuntu-latest, macos-latest and windows-2025, each clause is held by the named witness.
- **Linear and static:** `wait_human_lines_are_linear` shows `waiting: <name>` exactly once on a TTY and absent
  piped. On Unix, no ESC and no CR not followed by LF on `wait`'s and `--help`'s terminal streams.
- **No colour or glyphs** under a non-TTY stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or `--json`:
  `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json`, and the `colour_decision_*` units for each rung.
- **Every colour span beside its word:** `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (bold NAME,
  the attention token on `DIALOG` alone, no dim on a live board), plus the `board_row_style_*` units (dim only on a
  stale row, the cocked composition).
- **Injected controls only escaped:** `last`'s case in `tests/cli_wait_last.rs`,
  `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`, and the `escape_*` and `board_row_style_`
  (g) units.
- **Verify appends step lines:** the `tests/cli_verify.rs` `[NN/MM]` literals.
- **Zero bytes under `run`:** the `tests/tui_passthrough.rs` cases.
- **Human keys pass during a send:** `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` and
  `paste_human_bytes_during_a_paste_land_wholly_after_it`.
- **Focus / mouse / resize never move the wheel:** `tui_focus_mouse_and_resize_never_take_the_wheel`.
- **The `ui` launch-line clause** is out of 0.1.0 by the founder's 2026-10-16 ruling (the cap's own note).

Where this is narrower than "on all three OSes", stated for your never-weaken check:
- On windows-2025, ConPTY re-renders the stream, so the terminal readings are read by literal absence or by parsed
  SGR attributes, not byte for byte. That is the same platform form the zero-bytes clause already takes.
- Focus and mouse on Windows follow the founder's F-W3 platform fact, already in the cap's note.
- The dim span on a real stale row is proven at the unit tier over the writer's own bytes. Under the outer PTY only
  its absence on a live board is asserted: a stale heartbeat (>5 s) does not fit the `mutants` profile's 10 s kill.

Method `integration`.

## Expected amendments (wrap)
- **The founder's own widening (`inputs#I2`)**, pattern "Boundary widening the founder answered himself to a P4 fork",
  so the wrap halts once on the answer card:
  - architecture Config management (`:403`), §Occupied Resources → Environment variables, §Naming (`:104`) and
    §Conventions (`:157`);
  - security-plan §Secret Management (`:410`), §Security Anti-Patterns Universal (`:542`) and an Input Validation row.
- **Provisional, the overseer's (`inputs#I1`):**
  - design-system Width (`host_size()`, not `COLUMNS`);
  - design-system §Navigation Pattern and layout-templates `--help` (served verbs only).
- **As built:**
  - design-system pattern 1 and §Tokens (`NO_COLOR` read as present; NAME on a stale row dim and bold);
  - layout-templates `viola list` As built and block 1;
  - a11y-plan P6 and the cli landmark row;
  - test-plan §5 / §6 / §7 (help pinned by literal; the parser in support);
  - obs-plan §3 intro and §11 Logs (CARRY 1: `human::refuse` called by `run` 5, `verify` 5, `list` 2, `revive` 1).

## Beyond the plan
- The `handoff:` help row (`link`, `unlink`) is owned by the existing entry "Explicit session links", which adds the
  row when it serves the verbs.
- `plugin install` in the `setup:` help row is owned by the existing entry "Optional plugin install", which adds the
  word when it serves the verb.
- The dim span on a real stale row under the outer PTY is not witnessed at the integration tier in this chunk or by any
  entry: it is proven at the unit tier only, with no owner minted (the 10 s `mutants` kill against the 5 s stale
  threshold plus the ~4 s stamp).
- The overseer's provisional answers Q2 (the width source) and Q3 (served verbs only) are carried to the founder's next
  batch for his own word, as delegate answers (`inputs#I1`).
