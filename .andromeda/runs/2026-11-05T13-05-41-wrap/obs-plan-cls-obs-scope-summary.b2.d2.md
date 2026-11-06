@answer n=1 entity=obs-scope-summary#8.3 v=no
why: the R8 strip change is the Windows registry reader and the case fold, and on Linux env names are already case-sensitive, so the claim that R8-stripped CLAUDE values are never logged still holds.
@answer n=2 entity=obs-scope-summary#9.1 v=no
why: the Stack (rmcp) and Workspace sources it cites are components the session keeps, and removing Windows and macOS code moves no MCP or workspace membership claim.
@answer n=3 entity=obs-scope-summary#9.2 v=no
why: the instrumentable label holds, since the session adds or removes no instrumentation point on the MCP surface.
@answer n=4 entity=obs-scope-summary#9.3 v=no
why: stdout stays reserved for MCP frames, and the session adds no stdout write or telemetry path.
@answer n=5 entity=obs-scope-summary#10.1 v=no
why: portable-pty stays pinned at =0.8.1 and still hosts the unmodified claude; only the ConPTY sideload and its Windows arm go.
@answer n=6 entity=obs-scope-summary#10.2 v=no
why: the boundary-only label is unchanged, since the session adds no instrumentation point for the PTY child.
@answer n=7 entity=obs-scope-summary#10.3 v=no
why: spawn and exit stay instrumented with exit detected on the process handle wait, which the session keeps, and no EOF detection is introduced.
@answer n=8 entity=obs-scope-summary#11.1 v=no
why: the Hook contract and MCP tools sources are not moved, since the session changes no hook or tool contract.
@answer n=9 entity=obs-scope-summary#11.2 v=no
why: the boundary-only label is unchanged and the session changes no instrumentation of the caller side.
@answer n=10 entity=obs-scope-summary#11.3 v=no
why: the hook-invoked and hook-decision points, the MCP tool-call handling and the mcp-stdin MAX_FRAME cut all stand, and the session does not move the stdin cut.
@answer n=11 entity=obs-scope-summary#12.1 v=no
why: sysinfo and claude agents --json remain sources, since sysinfo stays on the OS API boundary and the external CLI call is kept.
@answer n=12 entity=obs-scope-summary#12.2 v=no
why: the boundary-only label is unchanged and the session adds no instrumentation point for the external CLI.
@answer n=13 entity=obs-scope-summary#12.3 v=no
why: spawn, exit and parse-outcome instrumentation stand, and resolve_program changes only how a binary path is found, not what is logged.
@answer n=14 entity=obs-scope-summary#13.1 v=no
why: the viola hook statusline contract is unchanged, since the session changes no statusline behavior.
@answer n=15 entity=obs-scope-summary#13.2 v=no
why: the boundary-only label is unchanged and the session adds no shell-out instrumentation.
@answer n=16 entity=obs-scope-summary#13.3 v=no
why: the statusline shell-out keeps logging only duration and exit status, and the session changes no command-string logging.
@answer n=17 entity=obs-scope-summary#14.1 v=no
why: its cited source is the Stack, and the components its call-site list rests on (sysinfo, the Unix socket and filesystem locks) are kept by the session.
@answer n=18 entity=obs-scope-summary#14.2 v=no
why: the boundary-only label is unchanged, since the session adds no call site of its own.
@answer n=19 entity=obs-scope-summary#14.3 v=yes mode=contradict
why: the session retires the Windows owner and DACL reads of the strict-modes checks (windows_read_verdict) and the off-Unix arm of check_socket_dir, so the Windows reads this entity names no longer happen.
@answer n=20 entity=obs-scope-summary#15.1 v=no
why: the event line, snapshot envelope and heartbeat sources are unchanged, since the session changes no store shape.
@answer n=21 entity=obs-scope-summary#15.2 v=no
why: the instrumentable label holds through the viola-state call sites, which the session keeps.
@answer n=22 entity=obs-scope-summary#15.3 v=no
why: the scrub rules hold, since endpoint and pinned_bin remain absolute paths on Linux, the replay snapshot still carries no endpoint or pid, and the session changes no heartbeat or collision-check read.
@answer n=23 entity=obs-scope-summary#16.1 v=no
why: the config management source is unchanged, since the session changes no config value or config parse.
@answer n=24 entity=obs-scope-summary#16.2 v=no
why: the not-instrumentable label holds, since the session adds no instrumentable config path.
@answer n=25 entity=obs-scope-summary#16.3 v=no
why: config parse logging through viola-state and the root bin is unchanged, and the session adds no config key or skipped-key count change.
