@answer n=1 entity=a11y-scope-summary#1.1 v=yes
why: the plain, NO_COLOR, TERM=dumb and --json output discipline across the CLI verbs (no SGR, plain ASCII, one document under --json) is what items 5 and 6 re-assert, and the revive, verify and plugin install lines sit in that discipline.
@answer n=2 entity=a11y-scope-summary#2.1 v=yes
why: the link and unlink result lines state the no-SGR, plain-ASCII, one-write discipline in every mode, which the chunk's plain-mode and SGR assertions (items 5 and 6) re-check; their exit-21 gating is not reached.
@answer n=3 entity=a11y-scope-summary#3.1 v=yes
why: the refusal's plain-ASCII, no-SGR hint line and the empty stdout and --json document are CLI output discipline that items 5 and 6 re-assert; the strict-modes gating itself is not changed.
@answer n=4 entity=a11y-scope-summary#4.1 v=yes
why: the socket-directory refusal states a plain-ASCII, no-SGR hint that names no path, pid or uid, and its --json document, which the chunk's plain-mode discipline assertions re-check; the directory check is not changed.
@answer n=5 entity=a11y-scope-summary#5.1 v=no
why: the run, revive-start and verify home-tree refusals gate on strict modes and state no SGR, escape or ASCII rule; the chunk changes no refusal and its wording is error recovery, not in reach (not_needed item 8).
@answer n=6 entity=a11y-scope-summary#6.1 v=yes
why: the socket-directory start refusal states a plain-ASCII, no-SGR stderr pair naming only the parsed name, which the chunk's plain-mode discipline assertions re-check; the start's gating is not reached.
@answer n=7 entity=a11y-scope-summary#7.1 v=yes
why: the viola run TUI passthrough boundary (zero viola bytes, human keystrokes never blocked, non-editing sequences not counted as editing) is what items 3, 4, 7 and 8 re-measure on the bumped outer-PTY driver.
@answer n=8 entity=a11y-scope-summary#8.1 v=yes
why: the vt100 feed-panic case is a TUI boundary case (tests/chaos_feed_panic.rs) run on the outer-PTY driver that the portable-pty bump changes, so it must stay green under item 4.
@answer n=9 entity=a11y-scope-summary#9.1 v=no
why: the plugin hook and stdio MCP frame-tool refusals against an impostor endpoint are not-assertable and write no human-facing byte; the plan holds no rule over them (not_needed item 9).
@answer n=10 entity=a11y-scope-summary#10.1 v=no
why: the hook and MCP frame-tool refusals against an instance that fails strict modes are not-assertable and write no human-facing byte; the plan holds no rule over them (not_needed item 9).
@answer n=11 entity=a11y-scope-summary#11.1 v=no
why: the hook calls against a loosened socket directory are not-assertable and write no human-facing byte; the plan holds no rule over them (not_needed item 9).
@answer n=12 entity=a11y-scope-summary#12.1 v=no
why: the MCP frame tools against a loosened socket directory are not-assertable and write no human-facing byte; the plan holds no rule over them (not_needed item 9).
@answer n=13 entity=a11y-scope-summary#13.1 v=yes
why: the cli surface's reach row names assert_cmd 2.2.2, predicates 3.1.4 and trycmd 1.2.1, which item 1 decides as tool or text, and its CLI checks must hold on ubuntu-latest (item 11).
@answer n=14 entity=a11y-scope-summary#14.1 v=yes
why: link's since line keeps newline, tab and non-ASCII through escape_message, a fork against the C0/C1 escape that the chunk's plain-mode discipline assertion re-checks (item 6).
@answer n=15 entity=a11y-scope-summary#15.1 v=yes
why: the tui boundary (zero viola bytes, unblocked human keystrokes, non-editing sequences and replies not moving the wheel) runs on the portable-pty =0.8.1 outer-PTY driver that the bump moves (items 3, 4, 7 and 8).
@answer n=16 entity=a11y-scope-summary#15.2 v=yes
why: the Linux-only platform rule is what the chunk implements by removing the Windows and macOS remnants and setting ci.yml to the single [ubuntu-latest] matrix (item 11).
@answer n=17 entity=a11y-scope-summary#16.1 v=yes
why: the caption line, the UNWRAPPED separator and the --help group labels are the board's structural equivalents, and item 14 keeps the CAPTION literal order and the board's parity with the 0.2.0 page undrifted.
