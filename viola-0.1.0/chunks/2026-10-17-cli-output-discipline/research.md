# Codebase Research — 2026-10-17-cli-output-discipline

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (71 lines, 17 Session Additions); `.claude/rules/testing.md` (loaded on `tests/**`, 48 Session Additions) — applied: the remove-the-guard run, oracle literals copied from the first run (2026-10-16), the Windows lint of `cfg` items (2026-10-03), the `windows-mutants.yml` line a new `cfg(windows)` product file owes (2026-10-13 via architecture-amendments), a kill only at a process the test started (2026-10-10)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's run `ci#39002965787` re-read green before P4) and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `src/human.rs` (full) — the `human` module root: `write_refusal` / `refuse`, `write_internal_error` / `internal_error`, `write_result` / `result`, the `Recorder` / `Broken` / `one_write` test helpers; re-exports the six submodules. No SGR, no `#[allow]`, no print macro.
- `src/human/escape.rs` (full) — `escape_message` (message mode, keeps `\n` / `\t`) and `escape_row` (row mode, every control) with rstest tables; both built.
- `src/human/wait.rs` (full) — `write_waiting` and the `wait` / `last` lines; `write_woken`, `write_woken_dialog`, `write_last_turn`, `write_message` all pass through `escape_message`; unit `woken_lines_escape_what_they_are_handed`.
- `src/human/verbs.rs` (full) — `answer` / `pause` / `release` / `release --budget` lines, uncoloured.
- `src/human/board.rs` (1–240 of 632) — `bay_line`, `wheel_cell`, `dialog_cell` (`DIALOG <kind>` as one string), `cli_cell`, `board_line` (fixed widths LIVE 5 · STATUS 7 · WHEEL 23 · DIALOG 17, NAME width = max(4, longest escaped name in chars)), `board_rows`, `write_board` (ONE `write_all`); `item.liveness.as_str()` is the LIVE cell; units `board_row_*` from line 216.
- `src/cmd/list.rs` (1–100) — `list` calls `human::write_board(&mut io::stdout().lock(), …)` at `src/cmd/list.rs:60` only in human mode; `--json` goes through `client::document`; the exit-21 `refuse` pairs.
- `src/cmd/mod.rs` (1–75) — `Cli` (`#[command(name = "viola", version, about = …)]`, global `--home`), `Command` enum in clap's declared order: run, revive, send, wait, last, answer, list, pause, release, verify, hook (hidden).
- `src/cmd/wait.rs:56`, `src/cmd/last.rs:65` — `waiting:` / `last` context lines gated on `!args.json && io::stderr().is_terminal()`.
- `crates/viola-pty/src/lib.rs` (grep) — `host_size()` at `:466` (Unix: `ioctl(STDOUT_FILENO, TIOCGWINSZ)`; Windows: `GetConsoleScreenBufferInfo(STD_OUTPUT_HANDLE)`); `HostTerminal::enter` saves and restores console modes with `0x000c` on output (CARRY 2's "never viola-pty's").
- `crates/viola-pty/src/pump.rs` (outline) — `PasteHandle::paste` one `write_all` under the input lock; unit `paste_human_bytes_during_a_paste_land_wholly_after_it` at `:814`.
- `crates/viola-state/src/liveness.rs:15` — `Liveness { Live, Stale, Gone }`.
- `Cargo.toml` — clap `=4.6.7`, `default-features = false`, no `color` (comment at `:189`); workspace windows-sys `=0.61.2` already carries `Win32_System_Console` (`:237`); root `[dev-dependencies]` hold NO `assert_cmd`, `predicates` or `trycmd` (`:150`–`:162`).
- `tests/tui_passthrough.rs` (outline) — the zero-viola-bytes cases (`tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it`, `tui_hooks_firing_add_no_viola_bytes`, `passthrough_with_a_statusline_source_adds_no_viola_bytes`, `tui_child_output_passes_through_without_viola_bytes`; Unix byte-equality, Windows literal absence).
- `tests/tui_wheel.rs` (outline) — `tui_focus_mouse_and_resize_never_take_the_wheel` (case (3)), Path 5 cases, `^Z`.
- `tests/cli_send.rs:924` — `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (clause (2)'s further case).
- `tests/cli_wait_last.rs:367`–`398` — `wait_human_lines_are_linear`: under `OuterPty` `waiting: builder` exactly once; piped: result alone, stderr empty, no ESC. Escaping of `last` at `:340`–`363`.
- `tests/cli_output_plain.rs` (1–100) — `sgr_attributes` parser (a file-local fn, not in `tests/support/`), `help_is_plain_when_colour_is_forced_into_a_pipe`, `usage_error_is_plain_when_colour_is_forced_into_a_pipe`, `help_is_plain_on_a_terminal`.
- `tests/cli_controls_not_disableable.rs:422`–`434` — `long_flags` reads `viola --help` from the line `Options:` on.
- `tests/cross_list_rows.rs` (outline) — the piped / `NO_COLOR` / `TERM=dumb` / outer-PTY mode matrix; Windows reads text under 200 columns.
- `tests/cli_list.rs` (outline) — `CAPTION` literal, `list_human_board_is_the_literal_rows`; no case builds a `stale` or `DIALOG` row; `fixtures/fake-scripts/path4-permission.json` exists (a permission dialog).
- `tests/cli_verify.rs:29`–`38` — the `[NN/23]` step-line literals.
- The current `target/debug/viola --help` (read at P3): clap's `Usage: viola [OPTIONS] <COMMAND>`, a `Commands:` list in declared order plus `help`, then `Options:` (`--home`, `-h`, `-V`).

## Graph impact (rust plane; trace `.andromeda/runs/2026-10-17T04-35-05-phase/tree-query-2026-10-17-cli-output-discipline.json`, rows 33)
- **write_board** — one product caller, `list` @ `src/cmd/list.rs:60`; six `board_row_*` units in `src/human/board.rs` (`:500`, `:545`, `:591`, `:607`, `:620`, `:630`). A signature change (a style argument) threads to these seven sites only.
- **escape_row** — `cli_cell` @ `src/human/board.rs:130`, `board_rows` @ `src/human/board.rs:151`, unit @ `src/human/escape.rs:65`.
- **refuse** (`human::refuse`) — 13 call sites: `src/cmd/run.rs` 5 (`:680`, `:688`, `:693`, `:700`, `:709`), `src/cmd/verify.rs` 5 (`:74`, `:143`, `:150`, `:185`, `:608`), `src/cmd/list.rs` 2 (`:75`, `:79`), `src/cmd/revive.rs` 1 (`:107`). Re-derived: `grep -rcE 'human::refuse\(' src` agrees (5 · 5 · 2 · 1).
- **result** (`human::result`) — `src/cmd/revive.rs:262`, `src/cmd/verify.rs:276`, `src/cmd/verify.rs:469`.
- **internal_error** — `main` @ `src/main.rs:73`.

## Patterns detected
- **One write per output** (`src/human/board.rs:207`, `src/human.rs:33`): every human writer builds the whole text and calls `write_all` once; units assert `calls == 1` through `one_write`.
- **Escape before layout** (`src/human/board.rs:151`): names are escaped first, then their width is counted in chars and padded; a span or a cut must act on the escaped text.
- **TTY gate at the call site** (`src/cmd/wait.rs:56`): a TTY-only line is decided in the verb with `IsTerminal` on the stream it writes; the writer itself is stream-agnostic.
- **Mode matrix per child env** (`tests/cross_list_rows.rs:58`): `piped(bin, home, env)` sets `NO_COLOR` / `TERM` per child through `Command::env`; never `set_var`.
- **Windows oracle by literal absence** (`tests/tui_passthrough.rs:25`–`68`): `assert_no_viola_bytes` searches viola's own literals; Unix compares byte for byte with an unwrapped run.

## Conventions to follow
- **Human writers only through `crate::human`**: no `print!` / `eprintln!` (workspace lints), no `#[allow]` in `src/human*` (obs-plan §11; `grep -cE 'print!|println!|eprint|#\[allow' src/human.rs` → 0 at P3).
- **Oracles are literals** (`.claude/rules/testing.md` §What to assert): the SGR bytes, the decision rungs and the grouped help text are typed in the test, copied from the first run's output (testing.md 2026-10-16), never imported from the product.
- **Shared test helpers live in `tests/support/`** (test-plan §2): a second user of `sgr_attributes` moves it there; no root test file keeps its own copy.
- **`cfg(windows)` product code**: a minimal OS reader with the decision in a plain function tested on every OS (testing.md 2026-09-24); a new `cfg(windows)` product file owes `.github/workflows/windows-mutants.yml` its line (`tests/contract_windows_mutation_scope.rs`).

## New files to create
- `src/human/style.rs` — the SGR tokens × three depths, the seven-rung decision and the depth rule as a pure function over injected facts, the span helpers (bold NAME, amber `DIALOG` word, dim stale row with the cocked composition), and the fact reader (std `IsTerminal` on stdout, the `NO_COLOR` / `TERM` / `COLORTERM` reads the founder ratified, `inputs#I2`, the Windows VT enable `ENABLE_VIRTUAL_TERMINAL_PROCESSING` alone, saved and restored)
- `tests/support/sgr.rs` — `sgr_attributes` moved out of `tests/cli_output_plain.rs`, its self-check case kept beside it

## Files to modify
- `src/human.rs` — `mod style;` and its re-exports
- `src/human/board.rs` — `write_board` takes the style (and the known width); the spans and the unwrapped-NAME truncation applied over the escaped cells; new `board_row_*` units
- `src/cmd/list.rs` — reads the style facts and the width once, passes them to `write_board`
- `src/cmd/mod.rs` — the grouped help: `override_usage` and a `help_template` carrying the group rows, `Options:` kept
- `tests/cli_output_plain.rs` — the grouped help literal; the parser moved to `tests/support/sgr.rs`
- `tests/support/mod.rs` — `pub mod sgr;`
- `tests/support/events.rs` — `raise` (a dialog raised through a gated fake-agent turn) moved here from `tests/cli_answer.rs`, its second user being `tests/cross_list_rows.rs`
- `tests/cli_answer.rs` — imports `raise` from `tests/support/events.rs`; no case changes
- `tests/cli_controls_not_disableable.rs` — `NO_COLOR`, `TERM` and `COLORTERM` cases in `setting_does_not_disable_the_send_paste_control`
- `tests/cross_list_rows.rs` — the truncation case, the two styled-board cases, and `NO_COLOR` set on the standing case's outer-PTY mode
- `tests/cli_wait_last.rs` — the no-ESC / CR-before-LF reading of `wait`'s outer-PTY stream
- `.github/workflows/windows-mutants.yml` — `src/human/style.rs` added to the root-package label whose files it belongs with: `tests/contract_windows_mutation_scope.rs` derives the Windows-gated sources from `src/` (its `is_windows_gate`, `product_lines`) and requires each in the workflow's lists

## Sweeps
- `human::` over `src/`: 12 files hit (`answer.rs`, `client.rs`, `last.rs`, `list.rs`, `pause.rs`, `release.rs`, `revive.rs`, `run.rs`, `send.rs`, `verify.rs`, `wait.rs`, `main.rs`) · 1 changed (`list.rs`) · 11 no-change (they write uncoloured lines the colour layer never touches). Re-derived: `grep -rlE 'human::' src` (CARRY 1 said ten files: corrected to 12).
- `human::refuse(` over `src/`: 13 hits · 0 changed · 13 no-change (refusals carry no SGR). CARRY 1 said `verify` has four sites: re-derived 5 (`grep -rcE 'human::refuse\(' src`).
- `trycmd|assert_cmd` over `Cargo.toml crates/*/Cargo.toml`: 0 hits — the test-plan's trycmd `viola-help` case does not exist; the help is pinned by literal (a wrap amendment of test-plan §5 / §6 / §7).
- `sgr_attributes` over `tests/`: 1 file (`tests/cli_output_plain.rs`) · moved.

## Open questions
- none — the three P4 forks are answered: the env reads by the founder's own word (`inputs#I2`), the help rows and the width source by the Viola overseer's technical answers, provisional (`inputs#I1`).

## Read at P4's resumption (the colour layer)
- `tests/cross_list_rows.rs:84`–`102` — the standing case's outer-PTY mode runs at 200 columns with only `PATH` set and asserts no ESC byte on Unix; a coloured board breaks it unless the mode sets `NO_COLOR`.
- `tests/cli_answer.rs:167` — `raise(wrapper, kind, nth)` raises a dialog through the `path4-permission.json` gated turn (used at `:377`, `:575`).
- `tests/contract_windows_mutation_scope.rs:51`–`64` — `is_windows_gate` (`cfg(windows`, `cfg!(windows)`, `target_os = "windows"`) over the lines before a file's first column-0 `#[cfg(test)]`.
- `.config/nextest.toml:23`–`25` — `binary(cross_list_rows)` and `binary(cli_answer)` run under the `ci` profile's override `period = "10s", terminate-after = 2` (a 20 s kill); `:31`–`33` the `mutants` profile's default `period = "5s", terminate-after = 2` (a 10 s kill), which they take (no `mutants` override names them), so a stamped-home case must fit 10 s (testing.md 2026-09-24, extended 2026-10-06).
- `crates/viola-pty/src/lib.rs:258` — viola-pty's output mode `0x000c` (`ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN`), the mode CARRY 2 bars the colour enable from borrowing.
- `.andromeda/architecture.md:403`, `.andromeda/security-plan.md:410` and `:542` — the env sentences the widening amends.
