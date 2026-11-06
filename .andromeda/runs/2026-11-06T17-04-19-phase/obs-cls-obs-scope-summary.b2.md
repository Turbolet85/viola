@answer n=1 entity=obs-scope-summary#8.2 v=yes
why: the instrumentability label of the CLAUDE*-sensitive entity; it binds through its reason line 8.3
@answer n=2 entity=obs-scope-summary#8.3 v=yes
why: R8-stripped CLAUDE* values are never logged anywhere, which scope item 11 names as the live real-claude session's diagnostics check and the upload drop
@answer n=3 entity=obs-scope-summary#9.1 v=no
why: source provenance line (arch Stack, rmcp, Workspace) that states no rule the chunk must honour
@answer n=4 entity=obs-scope-summary#9.2 v=yes
why: the instrumentability label of the MCP stdout entity; it binds through its reason line 9.3
@answer n=5 entity=obs-scope-summary#9.3 v=yes
why: stdout is reserved for MCP frames and no telemetry may go to stdout; scope done-check item 8 names the clippy print_stdout and print_stderr gates over this
@answer n=6 entity=obs-scope-summary#10.1 v=no
why: source provenance naming portable-pty; the rules it grounds are carried by lines 10.2 and 10.3
@answer n=7 entity=obs-scope-summary#10.2 v=yes
why: the boundary-only label of the claude child seam; it binds with line 10.3, whose seam rules scope item 6 names
@answer n=8 entity=obs-scope-summary#10.3 v=yes
why: spawn and exit as process-start and process-exit detected on the process handle never EOF, and paste as boundary writes, which the portable-pty bump must keep holding (scope item 6)
@answer n=9 entity=obs-scope-summary#11.1 v=no
why: source provenance for the hook contract and MCP tools; it states no rule the chunk's code or tests must honour
@answer n=10 entity=obs-scope-summary#11.2 v=no
why: the boundary-only label of the hook and MCP entity; its reason line 11.3 covers code the chunk does not reach
@answer n=11 entity=obs-scope-summary#11.3 v=no
why: the MCP stdin cut and hook-invoked instrumentation are in neither the chunk's edits nor its scope, and no trace context is added (scope item 3 not needed)
@answer n=12 entity=obs-scope-summary#12.1 v=no
why: source provenance for sysinfo and claude agents --json; it states no rule the chunk must honour
@answer n=13 entity=obs-scope-summary#12.2 v=no
why: the boundary-only label of the external-CLI entity; its reason line 12.3 covers a parse unit the chunk does not edit
@answer n=14 entity=obs-scope-summary#12.3 v=no
why: the claude agents --json parse outcome lies in a unit of viola-agent-claude the chunk does not edit (only line 2 is edited), and no scope item names it
@answer n=15 entity=obs-scope-summary#13.1 v=no
why: source provenance for viola hook statusline; it states no rule the chunk must honour
@answer n=16 entity=obs-scope-summary#13.2 v=no
why: the boundary-only label of the statusline shell-out; it binds only through its reason line 13.3, which is not reached
@answer n=17 entity=obs-scope-summary#13.3 v=no
why: statusline shell-out duration and command-string logging, which scope item 2 excludes since no change touches the statusline shell-out
@answer n=18 entity=obs-scope-summary#14.1 v=no
why: source provenance for the Stack entry of the home and socket directory; it states no rule the chunk must honour
@answer n=19 entity=obs-scope-summary#14.2 v=yes
why: the boundary-only label of the home and socket-directory call sites; it binds with line 14.3 through the socket-dir-refused call site
@answer n=20 entity=obs-scope-summary#14.3 v=yes
why: the socket-dir-refused code written at the check_socket_dir refusal is the wording that scope item 14 ratifies or re-words
@answer n=21 entity=obs-scope-summary#15.1 v=no
why: source provenance for the ndjson and snapshot stores; it states no rule the chunk must honour
@answer n=22 entity=obs-scope-summary#15.2 v=no
why: the instrumentable label of the state-store call sites; it binds only through reason line 15.3, which is not reached
@answer n=23 entity=obs-scope-summary#15.3 v=no
why: per-store scrub rules over events.ndjson, snapshot.json and budget.json, and no state-store reader or appender is touched (scope item 5 not needed)
@answer n=24 entity=obs-scope-summary#16.1 v=no
why: source provenance for config management; it states no rule the chunk must honour
@answer n=25 entity=obs-scope-summary#16.2 v=no
why: config management is not-instrumentable and the chunk adds no config telemetry
