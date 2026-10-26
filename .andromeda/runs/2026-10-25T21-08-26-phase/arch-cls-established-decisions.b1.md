# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · Tokio only in mcp (sync core): the chunk's server.rs and win.rs edits and their tests stay sync, with no Tokio runtime or client
established-decisions#2.1 · no · the chunk edits no PTY, pump or ConPTY code, and its start tests reach the child only through the unchanged start path
established-decisions#3.1 · no · the chunk changes no screen or readiness-gate code, and its send test asserts a budget refusal before send-issued, not a gate verdict
established-decisions#4.1 · no · the chunk changes no confirmation code, and its no-cursor check is the budget refusal before send-issued, which this rule states only for the empty-text refusal
established-decisions#5.1 · no · the chunk changes no log, snapshot, budget-reading or lock writer, and a refused bind writes no endpoint, so no append, heal or snapshot write is reached
established-decisions#6.1 · no · the chunk writes no file through `replace_private`, and its DACL and handle checks are on the pipe handle, not a persisted file
established-decisions#7.1 · yes · the listener is interprocess local_socket behind the channel seam, one endpoint per run under a hashed instance name, which the listener edits, the `viola-<h12>` names and the squatted-name refusal rest on
established-decisions#8.1 · no · the chunk changes no MCP server, tool or frame, and the MCP surface is outside its reach
established-decisions#9.1 · no · the chunk changes no hook command, tier or connect deadline, and the hook transport is outside its scope
established-decisions#10.1 · no · the chunk touches no hook, and the fail-open and no-stderr rules here govern only `hook`
established-decisions#11.1 · no · the dialog-expired line belongs to the dialog hold and the wheel, which the chunk does not reach
established-decisions#11.2 · no · a measured 671 s dialog-hold run that the chunk neither reproduces nor relies on
established-decisions#12.1 · no · the 3600 s ceiling and its long-hold Windows leg are owed to other route entries, and the chunk measures no dialog hold
