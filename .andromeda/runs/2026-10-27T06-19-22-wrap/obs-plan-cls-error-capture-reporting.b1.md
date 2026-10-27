# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic backtrace and its detail-line write (src/panic_frames.rs, the hook) are unchanged, and the session adds no panic path
error-capture-reporting#1.2 · no · the fs.rs change is only replace_private_shared's bounded compare, so check_diagnostics still runs before ensure_private_dir narrows and a refusal is still a failed open
error-capture-reporting#2.1 · no · the main catch site and the run worker threads are unchanged, and the cut reuses the existing stdin-close exit and Err mapping, so every role's Err still maps to its exit
error-capture-reporting#3.1 · no · the change adds no check or file open before the role file, the mcp home check still runs first, and the new line is written only after the session ends
error-capture-reporting#4.1 · yes · add-to · tests/chaos_feed_panic.rs now also asserts (F6) that no obs literal reaches the outer terminal after a forced vt100 feed panic, on every OS
error-capture-reporting#5.1 · yes · add-to · an MCP stdin line past MAX_FRAME ends the session with no isError result and no rmcp line (F3), so the MCP rejects record gains that case
