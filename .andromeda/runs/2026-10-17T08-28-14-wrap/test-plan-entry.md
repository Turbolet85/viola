
## 2026-10-17-cli-output-discipline — viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6
**Section:** §5 Integration Test Strategy (Human-mode layouts; CLI: trycmd cases, `tests/cli_output_plain.rs`, `tests/cli_controls_not_disableable.rs`); §6 E2E Test Strategy (surface table, cli row); §7 Test Data & Fixtures (Human-mode expected output)
**Change:**
- Was: `viola-help` a trycmd `tests/cmd/*.toml` case; now `viola --help` (the grouped verb table) is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`) — §5 Human-mode layouts, §5 CLI, §6 cli tool cell and §7 each name it beside the `viola list` board's literals.
- `tests/cli_output_plain.rs` adds `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs` (each served verb's `--help` exit 0, its usage program `viola` or `viola.exe`; no `hook`, `help`, `link`, `unlink`, `plugin`) and, on Unix, no CR not followed by LF; the parser `sgr_attributes` (with `redraws`, `sets_a_foreground`) lives in `tests/support/sgr.rs`, its self-check case staying in that file.
- §6 cli row verdict: bold NAME and amber `DIALOG` under the outer PTY (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`; the parsed attribute set on windows-2025); dim `stale` at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity".
- Vector 6: `NO_COLOR`, `TERM`, `COLORTERM` are cases of `setting_does_not_disable_the_send_paste_control`, whose helper removes the three inherited values first.
**Why:** measured this chunk — no manifest holds trycmd and no `tests/cmd/` exists; the help is pinned by literal. The stale-dim witness's owner is the halt's answer (a delegate's, the Viola overseer). The other trycmd claims are carried to "Version done-check", not amended here.
**Kept:** the remaining trycmd / assert_cmd tooling claims (§3, §5, §6, §7, §10) stand until "Version done-check" decides tool or text.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/
