@answer n=1 entity=obs-scope-summary#8.3 v=yes
why: CLAUDE values never logged anywhere is what the secret-scan test and the zero secret-scan hits gate (scope items 11 and 15) check on every OS
@answer n=2 entity=obs-scope-summary#9.1 v=no
why: a source citation for the MCP stack only, it states no rule the chunk must honour
@answer n=3 entity=obs-scope-summary#9.2 v=yes
why: the MCP server's instrumentability label pairs with the reason in #9.3, and the chunk's macOS MCP carve-out (scope item 6) runs under it
@answer n=4 entity=obs-scope-summary#9.3 v=yes
why: stdout is reserved for MCP frames, so new Unix code must not print to stdout, which the print_stdout lint and the hook stdout contract (scope items 17 and 19) enforce
@answer n=5 entity=obs-scope-summary#10.1 v=no
why: a source citation for portable-pty and the project intent, it states no rule
@answer n=6 entity=obs-scope-summary#10.2 v=yes
why: the claude child hosted by the pty seam is instrumented at the seam only, the regime scope item 1 names for the openpty boundary
@answer n=7 entity=obs-scope-summary#10.3 v=yes
why: spawn and exit on the process handle, never EOF, and paste logging are the seam's named process-start, process-exit and paste behaviours (scope items 2 and 3)
@answer n=8 entity=obs-scope-summary#11.1 v=no
why: a source citation for the hook contract and MCP tools, it states no rule
@answer n=9 entity=obs-scope-summary#11.2 v=yes
why: the boundary-only regime for the hook and MCP caller pairs with #11.3, and the Unix hook contract under the suite (scope item 19) runs under it
@answer n=10 entity=obs-scope-summary#11.3 v=yes
why: the hook-invoked, hook-decision and mcp-stdin parse-rejected oversize lines are closed-catalog words the re-asserted schema on every OS must hold (scope items 4 and 5)
@answer n=11 entity=obs-scope-summary#12.1 v=no
why: a source citation for sysinfo and claude agents --json, it states no rule
@answer n=12 entity=obs-scope-summary#12.2 v=no
why: the external claude agents CLI label concerns a call the chunk neither names nor changes
@answer n=13 entity=obs-scope-summary#12.3 v=no
why: the claude agents parse outcome and unknown fallback are not named by the chunk's scope or touched by the Unix pty, hook or MCP work
@answer n=14 entity=obs-scope-summary#13.1 v=no
why: a source citation for the statusline shell-out, it states no rule
@answer n=15 entity=obs-scope-summary#13.2 v=no
why: the statusline shell-out label concerns a user command the chunk does not name, and its stdout contract is carried by scope item 19 alone
@answer n=16 entity=obs-scope-summary#13.3 v=no
why: shell-out duration, exit status and the never-logged command string are not named by the chunk's scope
@answer n=17 entity=obs-scope-summary#14.1 v=no
why: a source citation for the Stack, it states no rule
@answer n=18 entity=obs-scope-summary#14.2 v=yes
why: the boundary-only label for viola's server check, socket directory and strict-modes call sites pairs with #14.3, which the Unix refusal and telemetry items (scope items 6 to 8) name
@answer n=19 entity=obs-scope-summary#14.3 v=yes
why: viola's call-site instrumentation for peer-euid, socket-dir-refused, server-verify-failed and the accept_loop refusal is what scope items 6 to 8 name for Unix
@answer n=20 entity=obs-scope-summary#15.1 v=no
why: a source citation for the ndjson, snapshot and heartbeat contracts, it states no rule
@answer n=21 entity=obs-scope-summary#15.2 v=yes
why: events.ndjson and snapshots are the stores whose fields the chunk re-asserts on every OS, and their instrumentation through viola-state call sites is what that check covers
@answer n=22 entity=obs-scope-summary#15.3 v=yes
why: the per-store scrub rules, the replay-rebuilt snapshot carrying no endpoint or pid, and the absolute-path scrub are what scope item 21 names
@answer n=23 entity=obs-scope-summary#16.1 v=no
why: a source citation for config management and the config-value classification, it states no rule
@answer n=24 entity=obs-scope-summary#16.2 v=yes
why: the not-instrumentable label for static config pairs with #16.3, and the config-value scrub that scope item 21 names runs under it
@answer n=25 entity=obs-scope-summary#16.3 v=yes
why: the config load-time parse logged as skipped-key counts through viola-state is a process-log line whose closed words the re-asserted schema covers on every OS
