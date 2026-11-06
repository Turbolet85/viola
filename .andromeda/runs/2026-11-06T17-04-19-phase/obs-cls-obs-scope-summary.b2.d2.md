@answer n=1 entity=obs-scope-summary#8.2 v=no
why: instrumentability of the drift-report component; no chunk item or scope unit touches drift
@answer n=2 entity=obs-scope-summary#8.3 v=yes
why: the never-logged R8 CLAUDE* rule is named by item 11 for the live real-claude session diagnostics and their upload
@answer n=3 entity=obs-scope-summary#9.1 v=no
why: source line for the rmcp and Workspace row; provenance only, no rule the chunk honours
@answer n=4 entity=obs-scope-summary#9.2 v=no
why: instrumentability label of the rmcp stdio component; the chunk changes no MCP code path
@answer n=5 entity=obs-scope-summary#9.3 v=yes
why: stdout is reserved for MCP frames, and item 8 requires the clippy print_stdout and print_stderr gates to clear
@answer n=6 entity=obs-scope-summary#10.1 v=no
why: source line naming portable-pty without a version; provenance only, so the bump leaves it true
@answer n=7 entity=obs-scope-summary#10.2 v=yes
why: the claude child is boundary-only, so the portable-pty seam wrapper the bump re-verifies must instrument only the boundary
@answer n=8 entity=obs-scope-summary#10.3 v=yes
why: spawn and exit on the process handle never on EOF, and paste writes, are the boundary items 6 names for the bumped seam
@answer n=9 entity=obs-scope-summary#11.1 v=no
why: source line for the hook and MCP contracts; provenance only, no rule the chunk honours
@answer n=10 entity=obs-scope-summary#11.2 v=yes
why: the hook contract is boundary-only, which is why item 12's dialog read keeps question, answer and tool input out of the logs
@answer n=11 entity=obs-scope-summary#11.3 v=yes
why: hook-invoked written after the hook.dialog response is item 12's read evidence, and the viola mcp stdin cut is the MCP boundary the live sessions drive
@answer n=12 entity=obs-scope-summary#12.1 v=no
why: source line for sysinfo and claude agents --json; provenance only
@answer n=13 entity=obs-scope-summary#12.2 v=no
why: instrumentability label of the claude agents and sysinfo reads, which the chunk names nowhere
@answer n=14 entity=obs-scope-summary#12.3 v=no
why: the unknown-on-parse-failure rule over claude agents output; the chunk touches no such parse
@answer n=15 entity=obs-scope-summary#13.1 v=no
why: source line for the statusline shell-out; provenance only
@answer n=16 entity=obs-scope-summary#13.2 v=no
why: the statusline shell-out is excluded by the chunk's not-needed item on the statusline and budget-gate
@answer n=17 entity=obs-scope-summary#13.3 v=no
why: the statusline command-string rule; the statusline is untouched by the chunk
@answer n=18 entity=obs-scope-summary#14.1 v=no
why: source line for the stack and viola call sites; provenance only
@answer n=19 entity=obs-scope-summary#14.2 v=yes
why: the socket directory check is a boundary-only viola call site where the chunk's socket-dir-refused wording sits
@answer n=20 entity=obs-scope-summary#14.3 v=yes
why: check_socket_dir refusal is written under socket-dir-refused with no MCP line, the wording item 14 ratifies or rewords
@answer n=21 entity=obs-scope-summary#15.1 v=no
why: source line for the ndjson and snapshot stores; provenance only
@answer n=22 entity=obs-scope-summary#15.2 v=no
why: viola-state call sites; the chunk's not-needed item says no state-store reader or appender is touched
@answer n=23 entity=obs-scope-summary#15.3 v=yes
why: the live-session diagnostics upload and the gated diag uploads are external output, so the snapshot path and events payload scrub rules bind
@answer n=24 entity=obs-scope-summary#16.1 v=no
why: source line for config values; provenance only, the chunk touches no viola config management
@answer n=25 entity=obs-scope-summary#16.2 v=no
why: config values are not-instrumentable and nothing in the chunk instruments config
