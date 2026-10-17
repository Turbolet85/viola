# Scope — 2026-10-17-cli-output-discipline · CLI output discipline

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry at take-up, twelfth under
`### Epoch 5 — Driver surface`, directly after "CLI machine contract"): CLI output discipline — linear static output on
three OSes, grouped --help, colour only beside its word, zero wrapper bytes, unblocked human keys during sends — plus
three CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run dir's trail: three rows on
the entry, CARRY 854 · 1149 · 189 chars; no `BLOCKED-ON`, no `PREREQ`, no `CONTEXT`, no `WATCH`).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). Every fact below is
read from the entry, from the masters (a11y-plan §1 / §3 / P4 / P6, design-system §Tokens (platform-specific),
layout-templates `--help`, obs-plan §3 / §11), from the verification matrix (`v1-40`), or from a file of this
repository. The earlier chunks' snapshots the CARRY text cites are spelt by those chunks' paths (P5 check 10). No
external input is snapped at take-up.

**CI read at Setup 5a** (the last wrap's flip `6d8b662` = HEAD, `ci.py conclusion`, this run dir's trail):
- `6d8b662cd291`: **in progress** at Setup — `CI 6d8b662cd291: verdict not yet available`; checks 15/15 registered,
  `ci#39002965787` push in_progress, 6 running, the oldest `lint (ubuntu-latest)` at 142 s when read. Not folded and
  not read as green; re-read before P4.
- Re-read at P3's end (same call, same trail): **green** — `ci#39002965787` push completed/success, checks 15/15, wall
  585 s. Nothing to fold.

**Premise closure (P3):** every `[inferred]` bullet below was read against `research.md`. A verified bullet has lost
its tag; a falsified one carries `[premise-corrected: …]`.

## What this chunk builds

The terminal discipline of every human-mode CLI output: linear, static text; colour only as a second cue beside its
word and only where a terminal can take it; injected control characters shown escaped; the grouped `--help`; and,
under `viola run`, no byte of viola's own on the outer terminal and no human key held behind a driver's send.

### 1. Linear static output on three OSes (entry)
- No spinner, no `\r` redraw, no cursor-movement or erase sequence on any stream of any verb (a11y-plan P6).
- The human-mode `viola wait` prints `waiting: <name>` exactly once on stderr when stderr is a TTY, and never under
  non-TTY (a11y-plan P6 "linear static output"). Built and witnessed: `src/cmd/wait.rs:56` gates it on
  `!json && stderr.is_terminal()`; `tests/cli_wait_last.rs` `wait_human_lines_are_linear` asserts it once under the
  outer PTY and absent piped. This chunk adds the no-`\r` / no-cursor-sequence reading across the human verbs.
- `viola verify` prints appended step lines `[NN/MM] … pass|fail` (a11y-plan P6). Built and witnessed:
  `tests/cli_verify.rs` pins the `[NN/23]` literals per OS; `MM` is `LedgerRow::relied()`'s count. Re-run, not
  rebuilt.
- The `ui` launch-line clause is out of scope: the founder moved the whole frontend to 0.2.0 (the 2026-10-16 note on
  `v1-40`, `.andromeda/runs/2026-10-16T14-58-10-wrap/inputs/` I1).

### 2. Grouped `--help` (entry; CARRY 3)
- `viola --help` is the grouped verb table (layout-templates: "`viola --help` is the grouped verb table (not built:
  the built help is clap's ordered list; owed to 'CLI output discipline'"); its group labels (board / traffic / wheel /
  handoff / …, a11y-plan §ARIA equivalents) are the terminal's structural equivalents.
- CARRY 3 (chunk 2026-10-11-the-board-viola-list, the review card the founder approved,
  `chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`,
  `chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`):
  the grouped `--help` table is not built; the built help is clap's ordered list.
- The help stays SGR-free: clap is built without `color` (`Cargo.toml:189`), held by `tests/cli_output_plain.rs`'s
  three cases; the group labels add text, not colour.
- The served verbs today are `run`, `revive`, `send`, `wait`, `last`, `answer`, `list`, `pause`, `release`, `verify`
  (and the hidden `hook`); `link`, `unlink` and `plugin install`, which the layout-templates wireframe groups, are not
  served (`src/cmd/mod.rs` `Command`). Which rows the grouped help prints is a P4 fork.
- The completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` reads `viola --help` from its
  `Options:` line on (`tests/cli_controls_not_disableable.rs:422`), so the grouped help keeps an `Options:` section.

### 3. Colour only beside its word (entry; CARRY 2)
- CARRY 2, its text as it stood when re-pinned (chunk 2026-10-11-the-board-viola-list,
  `chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`; owners
  `chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`; placement
  `chunks/2026-10-11-the-board-viola-list/inputs/I7-relay-3.md.txt`,
  `chunks/2026-10-11-the-board-viola-list/inputs/I9-relay-4.md.txt`; the founder's own answer on the env-read
  widening it names: `inputs#I2`): chunk 2026-09-28-cli-output-tokens (P4 operator ruling: no code
  without a live consumer) left the colour layer to this entry, its first consumer, in `src/human.rs`:
  - the four SGR tokens × three depths and the stale × cock composition (design-system §Tokens (platform-specific));
  - the seven-rung colour decision order and depth rule as a pure function over injected facts;
  - the fact reader (`NO_COLOR`, `TERM`, `COLORTERM`, stdout `IsTerminal`, a Windows VT enable with
    ENABLE_VIRTUAL_TERMINAL_PROCESSING only, saved and restored — never viola-pty's `0x000c`, which adds
    DISABLE_NEWLINE_AUTO_RETURN);
  - (the row-mode escaper landed with the board: `escape_row`).
- The spans (a11y-plan P6, design cli): amber only on the `DIALOG` word, dim only on stale rows, bold only on NAME
  column values (the callsign), always beside the word; `unable` / `fail` carry no SGR; no SGR under non-TTY /
  `NO_COLOR` / `TERM=dumb` / `--json`. `viola list` emits no SGR as built (a11y-plan P6, design-system §441,
  layout-templates §91), so its three spans get their outer-PTY check here.
- **Boundary widening (CARRY 2, verbatim):** "Reading `NO_COLOR` / `TERM` / `COLORTERM` is a boundary widening of
  architecture Config management's exhaustive env sentence and security-plan Secret Management: it needs a Decisions
  Log entry and a live founder ratification (playbook 'Boundary widening')." The plan cannot ship the env reads on a
  decisive lean: `.andromeda/playbook.md`'s "Boundary widening" and "Boundary widening — what ratifies it" patterns
  are `verdict: escalate`, so P4 asks it as a fork naming them.
- Width-known truncation of an unwrapped NAME on the styled TTY board (layout-templates-amendments "Line 284's
  design-target truncation of unwrapped NAMEs stays (owed to 'CLI output discipline')") rides this clause.
  design-system §Surface: cli Width names `GetConsoleScreenBufferInfo` on Windows and `COLUMNS` elsewhere; the width
  is already read without any env var by `viola_pty::host_size()` (`crates/viola-pty/src/lib.rs:466`: a stdout
  `TIOCGWINSZ` ioctl on Unix, the console buffer on Windows). Which source it reads is a P4 fork.

### 4. Injected controls shown only escaped (`v1-40`; a11y-plan P6)
- `viola list` rows escape through `escape_row` (`src/human/board.rs:151`, `tests/cross_list_rows.rs`); `viola wait`
  / `viola last` human-mode text shows every C0 / C1 byte other than `\n` / `\t` escaped through `escape_message`
  (`src/human/wait.rs`, units `woken_lines_escape_what_they_are_handed`; root `tests/cli_wait_last.rs` asserts no raw
  control on `last`'s stdout). Built and witnessed; the styled board must keep every span outside the escaped text.

### 5. Zero wrapper bytes under `viola run` (entry)
- Case (1) of a11y-plan P4: zero viola-originated bytes on the outer-PTY stream — byte for byte against an unwrapped
  fake-agent run on Linux / macOS; viola's own literals absent on Windows ConPTY.
- Held by `tests/tui_passthrough.rs` (`tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it`,
  `tui_hooks_firing_add_no_viola_bytes`, `passthrough_with_a_statusline_source_adds_no_viola_bytes`,
  `tui_child_output_passes_through_without_viola_bytes`): Unix byte for byte against an unwrapped run, Windows by
  literal absence. Re-run, not rebuilt; the colour layer adds no byte there (`run` is rung 2 of the decision order).

### 6. Unblocked human keys during sends (entry)
- Case (2) of a11y-plan P4: human keystrokes delivered unblocked past the current atomic paste; and the further case
  under it — a key written while a verified `send` waits in the readiness gate takes the wheel, the send exits 10
  `human-typing` with nothing typed and the key reaches the child.
- Held today by three witnesses: `tests/cli_send.rs` `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`
  (the readiness-gate case, outer PTY, three OSes), `crates/viola-pty/src/pump.rs` unit
  `paste_human_bytes_during_a_paste_land_wholly_after_it` (human bytes during the one-write paste land whole after it)
  and `tests/tui_passthrough.rs` `tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it`. Re-run, not rebuilt.

### 7. obs-plan's stderr-writer sentences (CARRY 1)
- CARRY 1 (chunk 2026-10-09-epoch-3-cleanup-ii's wrap, read at `632f6a7`): obs-plan §3's intro calls `src/human.rs`
  `refuse` → `write_refusal` "the root bin's only human-stderr writer, called only by `run`", and §11 Logs restates
  the `run`-only framing; `src/cmd/verify.rs` calls `human::refuse` at four sites and ten files under `src/` use
  `human::`. Both sentences are brought to what writes to stderr when this entry lands
  [premise-corrected (as carried): chunk 2026-10-10-viola-revive's wrap amended both for its own verb only — §3's intro
  now reads "the root bin's only human-stderr writer for a start refusal, called by `run` and by `revive`" and §11 Logs
  names `run`'s and `revive`'s refusals; neither names `verify` or the other verbs]
  [premise-corrected: re-derived at P3 (`grep -rlE 'human::' src`; `grep -rcE 'human::refuse\(' src`): 12 files under
  `src/` use `human::`, not ten, and `human::refuse` has 13 call sites: `verify` 5 (not four), `run` 5, `list` 2,
  `revive` 1].
- The sentences are master text: this chunk names them as `Expected amendments (wrap)` against the stderr writers the
  chunk leaves, never a phase edit.

## Boundaries — not this chunk
- The `ui` launch line and any web surface (0.2.0).
- The wheel's focus / mouse / resize clause: landed by chunk 2026-10-04-the-wheel (`tests/tui_wheel.rs`); re-run, not
  rebuilt.
- `--json` documents and typed exits (chunk 2026-10-17-cli-machine-contract) — this chunk changes no machine form.
- The Windows real-terminal readings ("Windows-only live measurements"): CI's `windows-2025` ConPTY leg is the
  Windows evidence here.
- The MCP server ("MCP server for drivers").

## Surfaces and contracts it touches
- `src/human.rs` (and `src/human/board.rs`), the human-mode writers of `wait`, `last`, `list`, `verify`, `send`; the
  clap command definition (`--help`); the `run` pump's outer-terminal path.
- a11y-plan §3 / P4 / P6, design-system §Tokens (platform-specific) and cli, layout-templates `--help` and the board's
  styled form, obs-plan §3 / §11, architecture Config management (the env sentence), security-plan Secret Management.

## Capability candidates
- `v1-40` CLI and terminal output discipline (planned, unclaimed): claimable only if every clause but the `ui` line
  is proven here, the env-read widening included; P4 reads its requirement line and the working-route for any other
  entry carrying a leg of it.
