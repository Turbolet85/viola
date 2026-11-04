@answer n=1 entity=established-decisions#1.1 v=yes
why: the sync-core and Tokio-only-in-mcp rule binds the chunk's new run and pty code (std threads) and the CI job 3 sync-crate check on all three OSes (item 16 and item 3).
@answer n=2 entity=established-decisions#2.1 v=yes
why: the pty seam's exit-on-handle, no-human-key-reordered-behind-resize and no-DA1 rules govern the openpty backend the chunk brings to parity (item 1).
@answer n=3 entity=established-decisions#3.1 v=no
why: this is the open Windows-host reading of the wide-character bytes; Windows is unmoved (item 6) and the Unix oracle it names is already asserted, not changed by the chunk.
@answer n=4 entity=established-decisions#4.1 v=yes
why: run's tee, feed thread, send readiness gate and poisoning-on-resize rules run on the openpty path the chunk brings to parity.
@answer n=5 entity=established-decisions#5.1 v=yes
why: send confirmation, typed_text and the empty-text refusal are proved under the fake agent on the three CI OSes, so the Unix write path must meet them (items 1 and 2).
@answer n=6 entity=established-decisions#6.1 v=yes
why: the events.ndjson append, torn-line and unknown-kind counting rules are the one-schema event-line contract (item 6) and the Unix events.ndjson write rule (item 14).
@answer n=7 entity=established-decisions#7.1 v=yes
why: the snapshot helper sets the 0600 mode before any byte is written, the Unix strict-modes rule (item 14) the chunk's snapshots must meet on Linux and macOS.
@answer n=8 entity=established-decisions#8.1 v=no
why: the no-clobber placement of the pinned copy belongs to the pin layout under bin/, which is not_needed (item 13), and the chunk places no copy.
@answer n=9 entity=established-decisions#8.2 v=no
why: the placed-hook test parameter exists only for the pinned-copy placement, which the chunk does not touch.
@answer n=10 entity=established-decisions#8.3 v=no
why: closing the write handle before placement fixes an ETXTBSY race on the pinned copy, outside the chunk's scope.
@answer n=11 entity=established-decisions#9.1 v=yes
why: the local_socket wait, answer and dialog-id contract and the short-hash endpoint name are what the Unix fake-agent suite drives on Linux and macOS (items 3 and 8).
@answer n=12 entity=established-decisions#10.1 v=yes
why: the socket-dir check stands at the channel callers, which item 8 requires before bind and on every Unix connect.
@answer n=13 entity=established-decisions#11.1 v=yes
why: the peer-pid rule, with no pid reported on macOS, is the macOS no-peer-pid carve-out of item 9.
@answer n=14 entity=established-decisions#11.2 v=yes
why: peer_euid_admitted and PEER_EUID_REPORTED are the owning-user euid admission of item 8 and the peer-euid seam of item 10.
@answer n=15 entity=established-decisions#12.1 v=yes
why: the wrapper's accept_loop euid filter before any frame is the Unix admission both sides owe under items 8 and 10.
@answer n=16 entity=established-decisions#13.1 v=yes
why: on macOS server_verdict makes no pid comparison, the recorded gap item 9 keeps and the Unix suite must read.
@answer n=17 entity=established-decisions#14.1 v=yes
why: check_socket_dir (absent passes, symlink and non-directory refused, directory left as found) is the socket-dir check item 8 requires on Unix.
@answer n=18 entity=established-decisions#15.1 v=no
why: the replay of unreadable or newer snapshots is a version-reader rule the scope does not name, and the chunk adds no snapshot reader and changes no field.
@answer n=19 entity=established-decisions#16.1 v=yes
why: item 14 names hook statusline's /bin/sh -c shell-out per OS, and this states that a replayed snapshot runs nothing and prints nothing.
@answer n=20 entity=established-decisions#17.1 v=no
why: the MCP tool list, the CLI-only release, pause and link rules and the wait cap are the MCP surface item 1 of not_needed excludes, and the chunk adds no tool.
@answer n=21 entity=established-decisions#18.1 v=no
why: the CappedLines MAX_FRAME stdin cap is named not_needed under the MCP server (item 1 of not_needed).
@answer n=22 entity=established-decisions#19.1 v=yes
why: each frame tool's check_instance strict-modes gate is the Unix strict-modes check item 14 names, and its refusal result is one the parity suite reads on Linux and macOS.
@answer n=23 entity=established-decisions#19.2 v=yes
why: the UnreachableCause mapping (StrictModes, SocketDir, ServerVerify) is the closed error word item 6 names, which must read the same on every OS.
@answer n=24 entity=established-decisions#20.1 v=no
why: the MCP stdin oversize line and its parse-rejected record sit on the MCP stdin cap, which is not_needed (item 1 of not_needed).
@answer n=25 entity=established-decisions#20.2 v=no
why: the exit-1 pre-initialize oversize test covers the MCP stdin cap, which the chunk leaves unchanged and which is not_needed (item 1 of not_needed).
