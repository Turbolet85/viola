# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Each new refusal of this chunk (a refused socket directory at a wrapper's start and at a client's connect, a peer euid mismatch on either side, the board's home refusal) must print its `unable` word uncoloured, and that check holds on the three OS legs.
  > `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) A socket-directory or peer euid refusal reached through `viola run`, a `viola revive`'s shared start, a pipe, `NO_COLOR`, `TERM=dumb` or `--json` must write no SGR of viola's own, and a revive refusal pair must carry no ESC byte.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) Under the board's one home rule, a home that passes while holding no instance must still print the empty-board line with no SGR.
  > the BAY line, caption, empty-board line and separator carry no SGR
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The change that makes the board's home check one rule must leave `viola list`'s row styling and width checks passing.
  > Checked by the `board_row_style_*` / `board_row_width_*` units and `cross_list_rows_style_each_span_beside_its_word_on_a_terminal`, `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json` and `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it`;
- (.andromeda/a11y-plan.md:340) The plan's visual ban covers every refusal surface this chunk adds: it must colour no `unable` and emit no SGR under these modes.
  > - NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **Uncoloured refusal ↔ §11 Visual anti-patterns (the body)**: the category's rule and the body's ban state the same requirement for this chunk's refusals, (.andromeda/a11y-plan-detail/visual-design-verification.md:9) and (.andromeda/a11y-plan.md:340).
- **Uncoloured refusal ↔ §8 Error recovery (Cognitive Accessibility)**: the category holds the refusal's colour and the error-recovery section holds its `unable` + `hint:` layout, so a new refusal must meet both, (.andromeda/a11y-plan-detail/visual-design-verification.md:9) and (.andromeda/a11y-plan.md:257).
