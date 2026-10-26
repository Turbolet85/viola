# Cascade compare — a11y-plan · phase · 2026-10-26-explicit-session-links
E items 59 · held 6 · lost at classify 1 · lost at strong 0 · beside a placed fact - · cascade only 2 · listed 3 · body 34 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### cognitive-accessibility#5.1 · lost at classify · .andromeda/a11y-plan-detail/cognitive-accessibility.md:45
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:45) The board's caption set and order must not change: the chunk adds no link column to `viola list`.
  > - The `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) pins the caption order `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI` for `viola list` (chunk 2026-10-11-the-board-viola-list).

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### cognitive-accessibility#3.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) Each new refusal must be pinned per refusal (the `unable` layout with the final `hint:`), with assert_cmd checking its typed exit code and a hint-free `--json`, on all three OS legs.
  > - **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal

### cognitive-accessibility#3.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:26) If the verbs take the server check, the server-identity negative and its control must cover `link` and `unlink` in both modes too, and the "six frame verbs" count and verb list move with it; the case stays gated off macOS.
  > the six frame verbs' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint, beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`), and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only.

### cognitive-accessibility#3.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:27
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:27) Any server-verify case added for the two verbs must compile out on macOS like the existing ones, its macOS witness staying owed to "Unix endpoint and home hardening".
  > - **cli, macOS:** the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".

### cognitive-accessibility#4.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:36
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:36) The three result lines (`link … since …Z`, `link … already linked`, `unlink …`) and every new refusal string must be pinned, and the `handoff:  link, unlink` row must land by moving the `HELP` literal of `help_is_the_grouped_verb_table` and the served set of `help_lists_only_served_verbs` on purpose.
  > - trycmd pins every fixed CLI string, except `viola list`'s, which literals pin (the board, the empty line and the refusal pairs: `tests/cli_list.rs` and the `src/human/board/` units `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list), and `viola --help`'s, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` with the served set held by `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`, chunk 2026-10-17-cli-output-discipline),

### cognitive-accessibility#4.1 · listed (lost at classify) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:36
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:36) A copy change to any pinned string this chunk touches must fail CI until the pin is updated on purpose.
  > Any copy change therefore fails CI until the pinned string is updated on purpose.

### visual-design-verification#1.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/visual-design-verification.md:8
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) The `viola link` / `viola unlink` result lines (`link … since …Z`, `link … already linked`, `unlink …`) and their refusals must not add a styled span: viola's SGR set is closed to the `DIALOG` word, stale rows and the NAME column of the board, so a driver or driven name printed on a link result line is plain text, never bold.
  > amber SGR only on the `DIALOG` word, dim SGR on stale rows and bold SGR only on NAME column values (the callsign; design cli), always beside the word.

### visual-design-verification#1.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/visual-design-verification.md:8
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) The two verbs' `--json` mode must carry no SGR, whatever the terminal and colour facts.
  > it writes no SGR under `--json`, a passthrough role or any of those rungs

### visual-design-verification#1.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/visual-design-verification.md:9
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The new verbs must emit zero SGR of their own on non-TTY, `NO_COLOR`, `TERM=dumb` and `--json` output, and their `unable` refusal lines (e.g. exit 21 `instance-unreachable`, a refused name) must never be coloured; the verbs' output-discipline cases run with assert_cmd on all three OS legs.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`

### visual-design-verification#1.1 · listed (lost at strong) · .andromeda/a11y-plan-detail/visual-design-verification.md:9
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The `unable` word on a `link` / `unlink` refusal stays uncoloured, and the check is assert_cmd across the three-OS matrix.
  > `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.

## Held

- a11y-scope-summary#1.1
- a11y-scope-summary#4.1
- a11y-scope-summary#6.1
- aria-patterns-roles#1.1
- aria-patterns-roles#3.1
- cognitive-accessibility#2.1

## Cascade only

- a11y-scope-summary#2.1
- a11y-scope-summary#5.1
