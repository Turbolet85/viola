@answer n=1 entity=visual-design-verification#1.1 v=yes
why: the task's item 5 names the CLI SGR discipline under the outer PTY (no SGR under non-TTY, NO_COLOR, TERM=dumb, --json, run, revive; amber only on DIALOG, bold only on NAME, dim only on stale), and item 6 keeps the plain-mode assertions the chunk's edited tests/cli_output_plain.rs carries.
@answer n=2 entity=visual-design-verification#2.1 v=yes
why: the stale-row escape sequence [DIM, BOLD, RESET, DIM, RESET] is the outer-PTY witness on ubuntu-latest that the task's item 5 names and that must stay green after the portable-pty bump (item 4).
