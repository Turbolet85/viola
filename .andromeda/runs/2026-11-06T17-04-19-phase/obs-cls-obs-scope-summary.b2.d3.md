@answer n=1 entity=obs-scope-summary#8.2 v=no
why: bare instrumentability value of the drift subject, which the chunk's scope does not name
@answer n=2 entity=obs-scope-summary#8.3 v=yes
why: the rule that CLAUDE* values are never logged anywhere binds the live real-claude diagnostics and any upload (item 11)
@answer n=3 entity=obs-scope-summary#9.1 v=no
why: provenance citation of rmcp and the workspace, stating no rule the chunk must honour
@answer n=4 entity=obs-scope-summary#9.2 v=no
why: bare instrumentability value of the MCP subject, and the chunk touches no MCP tool-call handling or stdin cut
@answer n=5 entity=obs-scope-summary#9.3 v=yes
why: stdout reserved for MCP frames is the rule the print_stdout and print_stderr clippy gate and the lint probes enforce (item 8)
@answer n=6 entity=obs-scope-summary#10.1 v=yes
why: names portable-pty, the dependency the bump moves, and states that it hosts the unmodified claude
@answer n=7 entity=obs-scope-summary#10.2 v=yes
why: boundary-only is the PTY seam the portable-pty bump reaches (item 6)
@answer n=8 entity=obs-scope-summary#10.3 v=yes
why: spawn and exit on the handle-wait thread, paste as bytes and the vt100 gate are the PTY boundary that item 6 keeps
@answer n=9 entity=obs-scope-summary#11.1 v=no
why: provenance citation of the Hook contract and MCP tools, stating no rule the chunk must honour
@answer n=10 entity=obs-scope-summary#11.2 v=yes
why: boundary-only confines the dialog-hold evidence to the hook boundary (item 12)
@answer n=11 entity=obs-scope-summary#11.3 v=yes
why: hook-invoked and hook-decision instrumentation are the evidence of the item 12 dialog read
@answer n=12 entity=obs-scope-summary#12.1 v=no
why: provenance citation of sysinfo and claude agents --json, which the chunk does not read
@answer n=13 entity=obs-scope-summary#12.2 v=no
why: bare boundary-only value of an external CLI that the scope does not name
@answer n=14 entity=obs-scope-summary#12.3 v=no
why: parse-outcome rule over claude agents --json output, which the chunk does not touch
@answer n=15 entity=obs-scope-summary#13.1 v=no
why: provenance citation of the statusline shell-out, which the scope leaves unchanged (not-needed item 2)
@answer n=16 entity=obs-scope-summary#13.2 v=no
why: boundary-only value of the statusline shell-out, which the scope excludes (not-needed item 2)
@answer n=17 entity=obs-scope-summary#13.3 v=no
why: statusline shell-out duration and command-string rule, which the scope excludes (not-needed item 2)
@answer n=18 entity=obs-scope-summary#14.1 v=no
why: provenance citation of the Stack, stating no rule the chunk must honour
@answer n=19 entity=obs-scope-summary#14.2 v=yes
why: boundary-only call-site instrumentation of the home and socket-dir checks is what the item 14 refusal wording rests on
@answer n=20 entity=obs-scope-summary#14.3 v=yes
why: names the check_socket_dir refusal under socket-dir-refused, the code item 14 ratifies or re-words
@answer n=21 entity=obs-scope-summary#15.1 v=no
why: provenance citation of the ndjson and snapshot stores, stating no rule the chunk must honour
@answer n=22 entity=obs-scope-summary#15.2 v=no
why: bare instrumentability through viola-state call sites, and the chunk touches no state reader or appender (not-needed item 5)
@answer n=23 entity=obs-scope-summary#15.3 v=yes
why: the scrub rules for external output bind the diag-<os> and junit-<os> uploads and the live-session diagnostics (items 7, 9, 11)
@answer n=24 entity=obs-scope-summary#16.1 v=no
why: provenance citation of config management, stating no rule the chunk must honour
@answer n=25 entity=obs-scope-summary#16.2 v=no
why: bare not-instrumentable value for config values, and the chunk changes no viola config value
