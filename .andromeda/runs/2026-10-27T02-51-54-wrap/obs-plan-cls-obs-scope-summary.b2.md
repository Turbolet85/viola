# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · its drift-report and R8 never-log rules still hold, since the session adds no log line or field and the fake-agent receipt gained only an integer
obs-scope-summary#9.1 · no · the rmcp stack and workspace layout are unchanged, since no Cargo manifest or crate member is among the reached files
obs-scope-summary#9.2 · no · viola-mcp is not reached, and the test-side Path 7 boot change in mcp_tools.rs leaves the server instrumentable
obs-scope-summary#9.3 · no · stdout stays reserved for MCP frames, since the fake-agent receipt field is test-only and under the existing print-lint exemption
obs-scope-summary#10.1 · no · the portable-pty dependency and its source are not touched, since the pin.rs change moves only the pinned-copy placement
obs-scope-summary#10.2 · no · the race is on the pinned viola copy and not on the claude PTY child, so the boundary-only class holds
obs-scope-summary#10.3 · no · the PTY child's boundary events are untouched, since the placement adds no spawn, exit, paste write or readiness outcome
obs-scope-summary#11.1 · no · the hook and MCP tool shapes are unchanged, since the only product change is the pinned-copy placement and the rest is test-side
obs-scope-summary#11.2 · no · no hook or MCP handling code was touched, so the boundary-only class holds
obs-scope-summary#11.3 · no · viola still sees only the hook stdin payload and its own response, since the hook and MCP handling lines are unchanged
obs-scope-summary#12.1 · no · the sysinfo and claude agents --json sources stand, since no agents-read or sysinfo file is among the reached files
obs-scope-summary#12.2 · no · the external-CLI boundary-only class holds, since no agents-read code was touched
obs-scope-summary#12.3 · no · the spawn, exit and parse-outcome instrumentation of the agents read is unchanged, since its parse code is not reached
obs-scope-summary#13.1 · no · the statusline remains the only shell-out, since the reached changes are Rust file operations with no shell-out
obs-scope-summary#13.2 · no · the boundary-only class holds, since no statusline code is reached
obs-scope-summary#13.3 · no · the shell-out duration, exit status and never-logged command string are unchanged, since no statusline code is reached
obs-scope-summary#14.1 · no · the Stack entry is unchanged, since no Cargo manifest is among the reached files
obs-scope-summary#14.2 · no · the boundary-only class holds, since the new placement is a viola call site and not an OS internal
obs-scope-summary#14.3 · no · the strict-modes checks it names still hold on the reached strict.rs, whose change is only a cfg(windows) test oracle, and the rename-class placement in fs.rs is observed through run.pin_copy
obs-scope-summary#15.1 · no · the event line, snapshot envelope and heartbeat contracts are unchanged, since schemas/diag-line.v1.json is untouched and no store field was added
obs-scope-summary#15.2 · no · the stores stay instrumentable through the same viola-state call sites, since the placement writes the pinned copy rather than a store and adds no line, field or span
obs-scope-summary#15.3 · no · pinned_bin stays scrubbed from external output, since the snapshot field is unchanged and place_private_new adds no line or span
obs-scope-summary#16.1 · no · config values are unchanged, since the session added no config key, env var or flag
obs-scope-summary#16.2 · no · the not-instrumentable class holds, since the session added no config key, env var or flag
obs-scope-summary#16.3 · no · the load-time parse and its skipped-key logging are unchanged, since no config key or flag was added
