# a11y-plan · ARIA Patterns & Roles
Read whole. The master's broad view is .andromeda/a11y-plan.md; this file holds its ARIA Patterns & Roles detail.

## 4. ARIA Patterns & Roles

<!-- detail: values · aria-landmarks -->

| Surface | Required landmarks | Optional landmarks |
|---------|-------------------|-------------------|
| cli | N/A (terminal). Structural equivalents: BAY context line → uppercase caption line → `-- UNWRAPPED - READ-ONLY --` separator; `--help` group labels (board / traffic / wheel / handoff / setup). As built (chunk 2026-10-11-the-board-viola-list; the rack, chunk 2026-10-16-unwrapped-sessions-on-the-board): BAY line → caption line → wrapped rows → (only when unwrapped items exist) the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows; the `--help` group labels built (chunk 2026-10-17-cli-output-discipline): `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the rows `board:` · `traffic:` · `wheel:` · `setup:` over the served verbs only, then `Options:`, plain ASCII, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` (`tests/cli_output_plain.rs`); `plugin install` in `setup:` built (chunk 2026-10-17-optional-plugin-install), the table's words read from the group rows alone, `plugin install` one entry and a bare `plugin` unserved, held by `help_lists_only_served_verbs`; the `handoff:` row is owned by "Explicit session links" | n/a |
| tui | N/A (vendor `claude` TUI, boundary-only) | n/a |

<!-- /detail -->

<!-- detail: procedure · path-p4-wheel -->

- **P4 — Human takes the wheel** (tui boundary + cli).
  - **cli:** `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
  - **tui:** three portable-pty outer-PTY nextest cases on all three OS legs, one per Section 1 boundary clause (Section 3 → Keyboard test harness → Tooling): (1) zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY); (2) human keystrokes delivered unblocked past the current atomic paste; (3) `\x1b[I`/`\x1b[O`, mouse and resize sequences do not move the wheel — step-wise (the resize behind its size receipt, then the focus reports with a mouse report as their read barrier), the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human); the full assertion on Linux and macOS, the platform fact on `windows-2025` (F-W3: focus reports swallowed, the injected mouse report takes the wheel), Section 3 → Keyboard test harness → Tooling. A further case sits under clause (2): a key written while a verified `send` waits in the readiness gate takes the wheel, the send exits 10 `human-typing` with nothing typed and the key reaches the child (the same Tooling paragraph, with its stated limit).
  - **SCs:** 2.1.1 (boundary stand-in on tui).

<!-- /detail -->

<!-- detail: procedure · path-p6-board -->

- **P6 — CLI board, wait/last, verify** (cli). No ARIA. trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on all three OS legs:
  - linear static output: under the outer PTY, `waiting: <name>` appears exactly once on stderr and the stream contains no `\r` redraw, cursor-movement or erase sequence (no spinner); under non-TTY the `waiting:` line is absent;
  - `[NN/NN] … pass|fail`;
  - **cli mirror:** a literal pins the caption order the deferred 0.2.0 page must share (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), not a trycmd case: the `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) and the unit case `board_row_caption_and_default_row_are_the_literals` (`src/human/board/`), chunk 2026-10-11-the-board-viola-list.
  - no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`;
  - under the outer PTY, every SGR span sits beside its word (amber only on `DIALOG`, dim only on stale rows, bold only on NAME column values), and `unable` / `fail` carry no SGR; as built (chunk 2026-10-17-cli-output-discipline) `viola list`'s spans are checked under the outer PTY by `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (bold NAME, amber only on `DIALOG`; on windows-2025 the parsed attribute set) and at the unit tier by the `board_row_style_*` cases; dim only on stale rows is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on "Linux and macOS parity"; `viola wait` and `viola --help` carry no ESC byte and no lone CR on Unix (`tests/support/sgr.rs` `redraws`);
  - C0/C1 escaped (security excerpt, C0/C1 escaping; ratified T5 for `list` rows), checked with assert_cmd on all three OS legs:
    - Setup: human-mode `viola wait` / `viola last` run against a `viola-fake-agent` event whose text carries C0 and C1 control bytes (including ESC / CSI), and `viola list` runs against a `claude agents --json` fixture row carrying the same bytes plus `\n` / `\t`. As built (chunk 2026-10-16-unwrapped-sessions-on-the-board) `viola list` makes one `claude agents --json` read and prints every unwrapped NAME (the row's `sessionId`, never a `ViolaName`) through `escape_row` on one line, held by the `board_row_*` units and by `tests/cross_list_rows.rs` in three modes (piped, `NO_COLOR` / `TERM=dumb`, the outer PTY; the no-ESC assertion Unix-only, and on windows-2025 each escaped name read as text under a 200-column terminal, because ConPTY re-renders escapes); beside them the row escape is held by unit cases on a recorded CLI version carrying ESC, C1, DEL, `\n` and `\t`, each printed as `\xHH` (`escape_row_shows_every_control_character_as_hex_text`, `board_row_cli_cell`), and the root cases `list_human_board_is_the_literal_rows` and `list_of_a_stopped_wrapper_is_an_empty_board` assert no ESC and no non-ASCII byte on either stream.
    - Pass condition: each injected control appears only in escaped form. In `wait` / `last` text `\n` / `\t` stay raw. In `list` table rows `\n` / `\t` are escaped too (`\x0A`, `\x09`), so every row stays one fixed-width line.
    - Under non-TTY, `wait` / `last` stdout contains no raw C0 / C1 byte other than `\n` / `\t`, and `list` stdout contains none inside a row. Under the outer PTY, the injected controls are escaped, and viola's own SGR stays beside its word only.
  - These stand in for SC 1.3.2, 1.4.1 and 3.3.1, with no conformance claim.

<!-- /detail -->
