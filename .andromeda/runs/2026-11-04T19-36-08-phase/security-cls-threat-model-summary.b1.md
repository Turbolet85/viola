@answer n=1 entity=threat-model-summary#1.1 v=yes
why: events.ndjson carries submitted prompts and final replies, which the chunk's events.ndjson schema and NEVER-log floor (items 5, 10) must keep out of every log.
@answer n=2 entity=threat-model-summary#1.2 v=yes
why: permission, plan and question events carry tool input and dialog text, which the events.ndjson schema and NEVER-log floor (items 5, 10) keep out of every log.
@answer n=3 entity=threat-model-summary#1.3 v=yes
why: the statusline command override and the snapshot's statusline_command are the Unix-only settings override and /bin/sh statusline shell-out the chunk brings in (item 8).
@answer n=4 entity=threat-model-summary#1.4 v=yes
why: the R8-stripped CLAUDE* values in the wrapper environment are what the NEVER-log floor (item 10) forbids in any output on every OS.
@answer n=5 entity=threat-model-summary#1.5 v=yes
why: the snapshot's recorded cwd must stay only in the 0600 snapshot and off every log line, as item 10 requires on every OS.
@answer n=6 entity=threat-model-summary#2.1 v=yes
why: the socket path, the per-user 0700 directory and the socket-dir and peer refusals are the Unix IPC access control and client verification items (3 to 6) the chunk builds.
@answer n=7 entity=threat-model-summary#3.1 v=yes
why: the Unix pty write carries bridge sends and the shared send end-to-end tests move onto both Unix OSes, so the framing and delivery check bind (item 1, write verb).
@answer n=8 entity=threat-model-summary#4.1 v=yes
why: the stdin classifier's closed non-editing list (the wheel and its seven reply shapes) is item 12 itself.
@answer n=9 entity=threat-model-summary#5.1 v=yes
why: the hook stdin tolerance, fail-open exit 0 and the strict-modes ordering of the hook paths are the hook contract (item 11), and the statusline shell-out is item 8.
@answer n=10 entity=threat-model-summary#6.1 v=yes
why: the MCP send, wait, last and answer frame path runs the socket-dir and peer euid checks and the instance-unreachable closed details the chunk makes uniform (items 3 to 5, 9).
@answer n=11 entity=threat-model-summary#7.1 v=no
why: the MCP stdin CappedLines reader and its oversize WARN are not named by the scope, the pty seam, the IPC items or the diagnostics schema change.
@answer n=12 entity=threat-model-summary#8.1 v=yes
why: the CLI --home and VIOLA_DIR refusals, check_home_tree and check_instance, and the json_asked internal-error form are the Unix home strict-modes (item 7) and the --json schema (item 9).
@answer n=13 entity=threat-model-summary#9.1 v=yes
why: the ~/.viola files, their check_home_tree strict-modes and the reader rules are the Unix home strict-modes of item 7.
@answer n=14 entity=threat-model-summary#10.1 v=yes
why: the pinned binary placed by place_private_new is the 0700 pinned viola whose Unix mode and placement order item 7 sets.
@answer n=15 entity=threat-model-summary#11.1 v=yes
why: run resolves and spawns its child by absolute path and the claude and verify PTY children are spawned under the same R8 strip, which item 1 hands to portable-pty.
@answer n=16 entity=threat-model-summary#12.1 v=yes
why: the portable-pty openpty backend that item 1 brings to Unix adds a Unix dependency edge that cargo deny and Cargo.lock, this entity's trust boundary, must check.
@answer n=17 entity=threat-model-summary#13.1 v=yes
why: ci.yml's test, perf, lint and supply-chain jobs are the CI jobs the chunk re-measures green on ubuntu-latest and macos-latest (items 4, 15, 16).
