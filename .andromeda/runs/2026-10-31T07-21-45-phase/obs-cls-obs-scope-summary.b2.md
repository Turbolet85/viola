# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · yes · drift reports only in diagnostics/ and the R8-stripped CLAUDE* never-log floor, which the detail-file leg and the planted-value witnesses must hold
obs-scope-summary#9.1 · no · bare source citation (rmcp stack); states no rule the chunk must honour, the rule is in #9.3
obs-scope-summary#9.2 · no · instrumentability label for viola mcp; the tool-call instrumentation it implies is carried by #11.3
obs-scope-summary#9.3 · yes · stdout is reserved for MCP frames, which the scope names for the viola mcp stdin cut (stdout carries MCP frames only on this path), so it is carried, not dropped as standing S5
obs-scope-summary#10.1 · no · bare source citation (portable-pty, the hosted claude); states no rule the chunk must honour
obs-scope-summary#10.2 · no · instrumentability label for the PTY host; the chunk touches no PTY instrumentation
obs-scope-summary#10.3 · no · spawn, exit on the process handle, paste and readiness-gate boundary of the PTY-hosted claude, none of which the chunk touches
obs-scope-summary#11.1 · no · bare source citation (hook contract, MCP tools); states no rule the chunk must honour
obs-scope-summary#11.2 · no · instrumentability label for the hook and MCP surface; the stdin and response rules are in #11.3
obs-scope-summary#11.3 · yes · the viola mcp stdin cut (lines at most MAX_FRAME, a longer line refused whole, one parse-rejected{mcp-stdin, oversize} line, no byte of the line) and MCP tool-call instrumentation
obs-scope-summary#12.1 · no · bare source citation (sysinfo, claude agents --json); states no rule the chunk must honour
obs-scope-summary#12.2 · no · instrumentability label for the external CLI; the parse-outcome rule is in #12.3
obs-scope-summary#12.3 · yes · the child pipe drain shared by claude agents --json: a parse failure gives unknown plus one parse-rejected line, and rows are never logged verbatim
obs-scope-summary#13.1 · no · bare source citation (viola hook statusline); states no rule the chunk must honour, the shell-out rule is in #13.3
obs-scope-summary#13.2 · no · instrumentability label for the user's statusline command; the rule is in #13.3
obs-scope-summary#13.3 · yes · the bounded status-line read: shell-out duration and exit status only, and statusline_command never logged outside diagnostics/
obs-scope-summary#14.1 · no · bare source citation (Stack); states no rule the chunk must honour
obs-scope-summary#14.2 · no · instrumentability label for the OS and IPC call sites; the rule is in #14.3
obs-scope-summary#14.3 · yes · a failed server check is logged by the CLI and hook and by no line on MCP, the no-line record the scope gives instance-unreachable; the OS-level hardening behind it stays out
obs-scope-summary#15.1 · no · bare source citation (ndjson, snapshots, heartbeat); states no rule the chunk must honour
obs-scope-summary#15.2 · no · instrumentability label for the store; the scrub rules are in #15.3
obs-scope-summary#15.3 · yes · snapshot.json endpoint and pinned_bin and config values scrubbed from external output, endpoint and pinned_bin never logged, paths only in diagnostics/
obs-scope-summary#16.1 · no · bare source citation (config management); states no rule the chunk must honour
obs-scope-summary#16.2 · no · not-instrumentable label for config values; the load-time parse rule is in #16.3
obs-scope-summary#16.3 · yes · the config.json readers of the thresholds, dialog hold and claude_env_keep observe only the load-time parse, with config values kept out of external output and kept variables named only
