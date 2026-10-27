# Classified — security · error-handling.b1

## Answers
error-handling#1.1 · yes · the scope names the hook's fail-open (exit 0, no body, no stderr) and bars any hook-side refusal from the fix, so the pin fix and the witness must keep it
error-handling#1.2 · no · the server-verification failure path (server-verify-failed detail, direct SessionEnd append) is not reached: the chunk changes no client server check and opens no endpoint; its exit-0 fail-open is carried by 1.1
error-handling#1.3 · no · the replayed or absent-snapshot connect fallback is not reached: the chunk changes no connect or snapshot read, and P3's build-hash readers ask which readers key on pinned_bin, not this fail-open
error-handling#1.4 · no · the statusline arm over a replayed snapshot is untouched: the statusline source read, its /bin/sh -c shell-out and the settings.json override are in the task's Not needed list
error-handling#1.5 · no · the viola mcp frame tools' server-check refusal is untouched: no send, answer or frame path changes and the MCP tools are in the Not needed list
error-handling#1.6 · no · the viola mcp check_instance-before-liveness refusal is untouched: the MCP tools and frame paths are in the Not needed list
error-handling#1.7 · no · the wrapper's link / unlink channel methods and their -32603 reply are untouched: no frame path changes and the chunk opens no endpoint
error-handling#1.8 · no · the CLI frame verbs' check_instance refusal (exit 21, instance pair) is not reached: the chunk changes no frame path, and its check is check_home_tree, not check_instance
error-handling#1.9 · yes · the scope names the order: check_home_tree (home, bin/, plugin/, the pinned copy's dirs) runs before the pin and before any log for run, revive's start and verify, so run's pin_bytes and verify's pin_exe calls stay after it
error-handling#1.10 · yes · PinError::StrictModes's fixed Display (no path or payload) and run's mapping of it to sideload_fallback strict-modes-failed bind the pin.rs absent-copy arm and pin_companions that the chunk edits; the scope's fixed-message rule on pin-path faults names it
error-handling#1.11 · no · the check_diagnostics failed-open path is not reached: the chunk opens no diagnostics folder or detail file, and its new log lines go through obs_event! (S2), not user-content detail
