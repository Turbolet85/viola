@apply_row n=1 ref_file=fanout-design-system--cascade.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-design-system--cascade.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-design-system--cascade.md ref_n=7 v=applied edit=E3
@apply_row n=4 ref_file=fanout-design-system--cascade.md ref_n=8 v=refused
why: a hand removal · fct-45kqhafb (its `.cmd or .bat script` start-refusal sub-item is a whole line of the fact; an in-line change never empties a line)
@apply_row n=5 ref_file=fanout-design-system--cascade.md ref_n=9 v=applied edit=E4
@apply_row n=6 ref_file=fanout-design-system--cascade.md ref_n=10 v=folded_into to_file=fanout-design-system--cascade.md to_n=9
@apply_row n=7 ref_file=fanout-design-system--cascade.md ref_n=11 v=refused
why: a hand removal · fct-a4ss54r5
@apply_row n=8 ref_file=fanout-design-system--cascade.md ref_n=12 v=applied edit=E5
@apply_row n=9 ref_file=fanout-design-system--cascade.md ref_n=13 v=folded_into to_file=fanout-design-system--cascade.md to_n=12
@apply_row n=10 ref_file=fanout-design-system--cascade.md ref_n=14 v=applied edit=E6
@apply_row n=11 ref_file=fanout-design-system--cascade.md ref_n=15 v=applied edit=E7
@apply_row n=12 ref_file=fanout-design-system--cascade.md ref_n=17 v=applied edit=E8
@apply_row n=13 ref_file=fanout-design-system--cascade.md ref_n=18 v=folded_into to_file=fanout-design-system--cascade.md to_n=17
@apply_row n=14 ref_file=fanout-design-system--cascade.md ref_n=19 v=applied edit=E9
@fact id=fct-kn56sdzi expect_rev=1
old: `viola_pty::host_size()` (Windows `GetConsoleScreenBufferInfo`, a `TIOCGWINSZ` ioctl elsewhere);
new: `viola_pty::host_size()` (a `TIOCGWINSZ` ioctl);

@fact id=fct-nmng4ymy expect_rev=1
old: holds the home to the read rule on every OS and
new: holds the home to the read rule and

@fact id=fct-ypei347w expect_rev=1
old: on the Unix runners (`ubuntu-latest`, `macos-latest`) by `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`), passing on both as measured at ci#41918694243:
new: on `ubuntu-latest`, the one CI runner, by `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, ungated), passing as measured at ci#42030025050:

@fact id=fct-45kqhafb expect_rev=1
old: `pinned-hash-mismatch`, `batch-script-child`, and for
new: `pinned-hash-mismatch`, and for
old: `revive`'s start. On macOS `server-verify-failed` means an euid mismatch alone: no peer pid is compared, a named gap no route entry owns.
new: `revive`'s start.

@fact id=fct-sx3u7aez expect_rev=1
old: `... fail` (`MM` is the relied count, `LedgerRow::relied()`: 24 on Unix, 21 on Windows, where the three statusline rows are not relied because the status line is not wrapped there;
new: `... fail` (the `shim-resolution` row passes when `claude --version` answers; `MM` is the relied count, `LedgerRow::relied()`: 24 on every build;
old: the newline row, on Unix the statusline rows,
new: the newline row, the statusline rows,
old: all three relied on every OS:
new: all three relied:
old: `stamped 2.1.287  24 pass  0 fail` on Unix.
new: `stamped 2.1.287  24 pass  0 fail`.

@fact id=fct-bctn63fi expect_rev=1
old: On Unix a home under
new: A home under
old: (exit 21). On Windows such a path stats `NotFound`, is passed as absent, and keeps the catch-site line.
new: (exit 21).

@fact id=fct-mep5dn6i expect_rev=1
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`

@fact id=fct-sedcqxww expect_rev=1
old: `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home
new: `the pinned viola copy failed its integrity check` / `the viola home
old: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version
new: the claude CLI was not found` / `the CLI version

@fact id=fct-pxdkcmfq expect_rev=1
old: or the check cannot be made; on Linux and macOS also a peer whose euid is not this user's or cannot be read, on macOS the euid alone)
new: or the check cannot be made; also a peer whose euid is not this user's or cannot be read)

@pattern id=ds2-01
fixed: a `TIOCGWINSZ` ioctl elsewhere
@pattern id=ds2-02 ignore_case=true
fixed: to the read rule on every OS
@pattern id=ds2-03 ignore_case=true
fixed: witnessed end to end on the Unix runners
@pattern id=ds2-04
fixed: `batch-script-child`, and for `viola revive`
@pattern id=ds2-05 ignore_case=true
fixed: means an euid mismatch alone
@pattern id=ds2-06 ignore_case=true
fixed: statusline rows are not relied
@pattern id=ds2-07
fixed: stats `NotFound`, is passed as absent
@pattern id=ds2-08
fixed: `stale-heartbeat` · `batch-script-child`
@pattern id=ds2-09 ignore_case=true
fixed: command is a .cmd or .bat script
@pattern id=ds2-10 ignore_case=true
fixed: the claude CLI is a .cmd or .bat script
@pattern id=ds2-11 ignore_case=true
fixed: on macOS the euid alone
