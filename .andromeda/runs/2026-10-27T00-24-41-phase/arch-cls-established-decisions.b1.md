# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the Tokio containment binds: the fix on the run and verify paths uses std threads and blocking I/O only, and viola-state stays a sync crate
established-decisions#2.1 · yes · the sideload paragraph binds: run's pin_companions writes the conpty/ companions and degrades to the inbox ConPTY (sideload_fallback), the exe-side counterpart the chunk reads
established-decisions#3.1 · no · the vt100 readiness gate, its signatures and feed thread are the Not needed screen model; run's start edit is the pin call, not the gate
established-decisions#4.1 · no · delivery confirmation is untouched (the task's Not needed list) and the race fix touches no send path
established-decisions#5.1 · yes · the lock rule binds any guard the fix adds: a std File::lock on a separate <name>.lock sibling, never an exclusive lock on the guarded file
established-decisions#6.1 · yes · the one-helper rule binds the fix: it stays inside replace_private and replace_private_shared (0700 for the pinned copy included), with no call site's own temp + rename
established-decisions#7.1 · no · the endpoint and the wait / answer semantics are the Not needed wrapper channel; the race fix sends no frame and changes no method
established-decisions#8.1 · no · the server_pid peer check guards frame-writing clients, the Not needed server identity check; the race fix sends no frame
established-decisions#9.1 · no · the macOS no-peer-pid gap is the Not needed server identity check, owed to «Unix endpoint and home hardening»
established-decisions#10.1 · no · the verified client's snapshot read is the Not needed replay readers and session liveness, which the race fix does not touch
established-decisions#11.1 · no · the hook statusline serve and its replay read are the Not needed statusline contract, untouched by the race fix
established-decisions#12.1 · no · the MCP tools, refusals and tool inputs are the Not needed MCP surface, and the per-start .mcp.json rewrite changes no tool
established-decisions#13.1 · no · the check_instance before live_endpoint on the frame tools is the Not needed MCP surface; the race fix touches no frame tool
established-decisions#14.1 · yes · the exec-form hook command the per-start hooks.json rewrite emits (the absolute pinned path, args ["hook", "<event>"]) and its tier map
established-decisions#15.1 · no · the connect, verify_server and fail-open path after the instance check is the untouched wrapper channel and replay; the race lies in the pinned write before the spawn
established-decisions#16.1 · yes · the rendered plugin stays byte-identical across starts, with the dialog hooks' 3615 s timeout in hooks.json, a rule over the per-start plugin rewrite
established-decisions#17.1 · no · the dialog-expired line goes through WaitFeed and the event line, both untouched; the fix appends no event kind
established-decisions#17.2 · no · a 671 s dialog-hold measurement record from another chunk; the dialog path is untouched and nothing in the scope re-measures it
