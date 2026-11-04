@apply_row n=1 ref_file=fanout-a11y-plan--cascade.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade.md ref_n=2 v=applied edit=E3,E4
@apply_row n=4 ref_file=fanout-a11y-plan--cascade.md ref_n=3 v=applied edit=E5
@apply_row n=5 ref_file=fanout-a11y-plan--cascade.md ref_n=4 v=applied edit=E6
@fact id=fct-lchghj7u expect_rev=1
old: **Owed:** dim only on stale rows is proven at the unit tier only; its outer-PTY witness is owed to "Linux and macOS parity".
new: **Unix legs only:** dim only on stale rows is proven at the unit tier and, under the outer PTY, on ubuntu-latest and macos-latest; the outer-PTY witness does not run on windows-2025.

@fact id=fct-uok334yy expect_rev=1
old: dim only on stale rows is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on "Linux and macOS parity";
new: dim only on stale rows is proven at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) and under the outer PTY on ubuntu-latest and macos-latest only, not on windows-2025 (the stale-row witness, below);

@fact after=fct-uok334yy file=aria-patterns-roles gid=new:path-p6-stale-dim kind=procedure subject=path-p6-stale-dim
- **P6 — stale-row dim under the outer PTY** (cli, Unix legs). `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, `cfg(unix)`, on a booted wrapper): the wrapper is frozen under the `Frozen` guard (`tests/support/frozen.rs`, test-plan's), its `heartbeat` back-dated 60 s so its row reads `stale`, and `viola list` runs under the outer PTY with `TERM=xterm-256color`; the row is dim from its first byte to its last and `stale` is printed (the exact SGR sequence it asserts: Section 6). It runs on ubuntu-latest and macos-latest and not on windows-2025.

@fact id=fct-owu3duhc expect_rev=1
old: path-p6-board · aria-landmarks
new: path-p6-board · path-p6-stale-dim · aria-landmarks
old: visual-sgr · cognitive-timeouts
new: visual-sgr · visual-sgr-stale-row · cognitive-timeouts

@fact id=fct-es7le4as expect_rev=1
old: `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json` and `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it`; dim only on stale rows is proven at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity".
new: `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json`, `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it` and, on ubuntu-latest and macos-latest only (`cfg(unix)`, not run on windows-2025), `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, the outer-PTY witness of a stale row dim from its first byte to its last.
old: Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
new: Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`, the stale-row dim witness on the two Unix legs only.

@fact after=fct-es7le4as file=visual-design-verification gid=new:visual-sgr-stale-row kind=values subject=visual-sgr-stale-row
- **A stale row on a terminal:** `viola list`'s stream for a board whose wrapped row is `stale` carries exactly the escape sequences `[DIM, BOLD, RESET, DIM, RESET]`, the word `stale` printed beside the dim (the spans as above), as the outer-PTY stale-row witness above asserts it on the two Unix legs.
@pattern id=a11y-01
fixed: is proven at the unit tier only; its outer-PTY witness is owed to
@pattern id=a11y-02
fixed: is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on
@pattern id=a11y-03
fixed: is proven at the unit tier only, its outer-PTY witness a CARRY on
