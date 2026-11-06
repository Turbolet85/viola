@answer n=1 entity=obs-scope-summary#8.3 v=no
why: the never-logged rule for R8-stripped CLAUDE values and the drift-to-diagnostics rule still hold, since the exact-match strip changes which names strip and not what is logged
@answer n=2 entity=obs-scope-summary#9.1 v=no
why: the cited Stack (rmcp) and Workspace sections name rmcp and the workspace crates, which the session does not remove
@answer n=3 entity=obs-scope-summary#9.2 v=yes mode=add-to
why: fact F1 says the core is Linux only on any current distribution with the Windows and macOS code removed, so the core's instrumentability is stated for Linux
@answer n=4 entity=obs-scope-summary#9.3 v=no
why: stdout stays reserved for MCP frames and the session changes no stdout path
@answer n=5 entity=obs-scope-summary#10.1 v=no
why: portable-pty stays pinned at =0.8.1 and the cited Stack and Project Intent sources still hold
@answer n=6 entity=obs-scope-summary#10.2 v=no
why: the claude process is still a third-party binary run unmodified, so boundary-only stands
@answer n=7 entity=obs-scope-summary#10.3 v=no
why: spawn and exit stay instrumented with exit detected on the handle wait, and the session removes only the TerminateProcess kill and the sideload fields, which this entity does not name
@answer n=8 entity=obs-scope-summary#11.1 v=no
why: the hook contract and MCP tools source is untouched by the session
@answer n=9 entity=obs-scope-summary#11.2 v=no
why: the hook and MCP surface stays boundary-only, unchanged by the session
@answer n=10 entity=obs-scope-summary#11.3 v=no
why: the hook events, MCP tool-call handling and the mcp-stdin cut are not touched by the removal
@answer n=11 entity=obs-scope-summary#12.1 v=no
why: sysinfo stays in the stack and the claude agents --json source is unchanged
@answer n=12 entity=obs-scope-summary#12.2 v=no
why: the external CLI stays boundary-only, unchanged by the session
@answer n=13 entity=obs-scope-summary#12.3 v=no
why: spawn, exit and parse-outcome instrumentation of the claude CLI is unchanged, and ProgramNotFound stays as it was
@answer n=14 entity=obs-scope-summary#13.1 v=no
why: the statusline shell-out source is untouched by the session
@answer n=15 entity=obs-scope-summary#13.2 v=no
why: the statusline shell-out stays boundary-only
@answer n=16 entity=obs-scope-summary#13.3 v=no
why: the shell-out duration and exit status logging is unchanged and the command string is still never logged outside diagnostics
@answer n=17 entity=obs-scope-summary#14.1 v=yes mode=stale
why: the cited arch Stack reading of the OS APIs moved, since ConPTY, the named pipe, TerminateProcess and windows-sys are removed, leaving openpty, the Unix socket, locks, rename and sysinfo
@answer n=18 entity=obs-scope-summary#14.2 v=no
why: the OS APIs boundary stays boundary-only, the removal narrows its list but not its instrumentability
@answer n=19 entity=obs-scope-summary#14.3 v=yes mode=contradict
why: it says the strict-modes checks read owner and DACL on Windows and that check_socket_dir reads nothing off Unix, both Windows arms being retired
@answer n=20 entity=obs-scope-summary#15.1 v=no
why: the event line, snapshot envelope and heartbeat sources are unchanged
@answer n=21 entity=obs-scope-summary#15.2 v=no
why: the stores stay instrumentable through the viola-state call sites, which the session does not remove
@answer n=22 entity=obs-scope-summary#15.3 v=no
why: the per-store scrub rules hold, since endpoint and pinned_bin remain absolute paths scrubbed from external output
@answer n=23 entity=obs-scope-summary#16.1 v=no
why: the config management source and config value classification are unchanged
@answer n=24 entity=obs-scope-summary#16.2 v=no
why: static configuration stays not-instrumentable, unchanged by the session
@answer n=25 entity=obs-scope-summary#16.3 v=no
why: the load-time parse logging through viola-state and the root bin is unchanged
