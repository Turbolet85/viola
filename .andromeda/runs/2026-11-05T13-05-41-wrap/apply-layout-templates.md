@apply_row n=1 ref_file=fanout-layout-templates.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-layout-templates.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-layout-templates.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-layout-templates.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-layout-templates.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-layout-templates.md ref_n=6 v=applied edit=E6
@apply_row n=7 ref_file=fanout-layout-templates.md ref_n=7 v=folded_into to_file=fanout-layout-templates.md to_n=5
@apply_row n=8 ref_file=fanout-layout-templates.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-layout-templates.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-layout-templates.md ref_n=10 v=applied edit=E9
@apply_row n=11 ref_file=fanout-layout-templates.md ref_n=11 v=applied edit=E10
@fact id=fct-7zli33yx expect_rev=1
old: `install Claude Code or put it on PATH`; `the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim`; `the CLI version could not be read`
new: `install Claude Code or put it on PATH`; `the CLI version could not be read`

@fact id=fct-i322ygxm expect_rev=1
old: `<code>` is `batch-script-child`, `pinned-hash-mismatch`
new: `<code>` is `pinned-hash-mismatch`

@fact id=fct-vnzdqtob expect_rev=1
old: std `IsTerminal`. Windows VT output is enabled with windows-sys 0.61.2 `SetConsoleMode`, and colour is off if that fails. Styling
new: std `IsTerminal`. Styling

@fact id=fct-mdoqfj5o expect_rev=1
old: `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable;
new: `NO_COLOR` (an empty value included) or `TERM=dumb`;

@fact id=fct-pf5uzbca expect_rev=1
old: `LedgerRow::relied()`'s length: 24 on Unix, 21 on Windows, where the relied set skips the three statusline rows because the status line is not wrapped there and ends at `local-command-compact`; it grows
new: `LedgerRow::relied()`'s length: all 24 rows on every build, ending at `local-command-compact`; it grows
old: Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before
new: Ctrl-C or TERM (`SIGINT` / `SIGTERM`) before

@fact id=fct-hk2gtsxu expect_rev=1
old: (a Unix host: 24 relied rows)
new: (24 relied rows)

@fact id=fct-qhojlk2s expect_rev=1
old: in plain ASCII. On Linux, Windows and macOS (on macOS an euid mismatch alone, the pid and start-time comparison absent, the named carve-out, owned by no route entry) a frame verb (`send`,
new: in plain ASCII. A frame verb (`send`,

@fact id=fct-4rbi2th6 expect_rev=1
old: The usage line is the override literal on every OS.
new: The usage line is the override literal.

@fact id=fct-33x2qanb expect_rev=1
old: names the program by its executable, `viola.exe` on Windows.
new: names the program by its executable.

@fact id=fct-bievnfhg expect_rev=1
old: printed bare unless the host's shell would split it (Unix: single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`; Windows: double-quoted when it holds a space).
new: printed bare unless the shell would split it: single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`.
@pattern id=lt-01
fixed: the claude CLI is a .cmd or .bat script
@pattern id=lt-02
fixed: `<code>` is `batch-script-child`
@pattern id=lt-03
fixed: Windows VT output is enabled
@pattern id=lt-04
fixed: or a failed Windows VT enable
@pattern id=lt-05
fixed: 21 on Windows, where the relied set skips
@pattern id=lt-06
fixed: a Unix host: 24 relied rows
@pattern id=lt-07
fixed: Windows Ctrl-C / Ctrl-Break
@pattern id=lt-08
fixed: On Linux, Windows and macOS (on macOS an euid mismatch alone
@pattern id=lt-09
fixed: the override literal on every OS
@pattern id=lt-10
fixed: `viola.exe` on Windows
@pattern id=lt-11
fixed: Windows: double-quoted when it holds a space
