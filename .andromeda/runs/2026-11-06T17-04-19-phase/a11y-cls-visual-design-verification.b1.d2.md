@answer n=1 entity=visual-design-verification#1.1 v=yes
why: the CLI SGR discipline (no SGR under non-TTY, NO_COLOR, TERM=dumb, --json, viola run, revive; amber only on DIALOG, bold only on NAME, dim only on stale rows) is asserted under the outer PTY that the portable-pty bump reaches (task item 5).
@answer n=2 entity=visual-design-verification#2.1 v=yes
why: the outer-PTY stale-row witness asserting the exact escape sequence [DIM, BOLD, RESET, DIM, RESET] is re-run under the bumped portable-pty driver on ubuntu-latest (task item 5, path P6).
