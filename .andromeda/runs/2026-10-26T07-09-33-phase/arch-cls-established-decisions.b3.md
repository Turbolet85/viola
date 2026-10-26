# Classified — arch · established-decisions.b3

## Answers
established-decisions#17.1 · yes · the liveness-only pre-check the chunk edits in liveness.rs must decide gone by pid plus process start time and the heartbeat, never a bare pid, and the new server check compares the same pid and started_at
established-decisions#18.1 · no · standing S5 · a new error variant is a fixed-message thiserror variant of the owning crate's enum, no path or payload
established-decisions#19.1 · yes · the Tokio client verification sits behind viola-channel's tokio feature in tokio_client.rs with only viola-mcp building a runtime, so viola-state and the hook and CLI paths stay tokio-free and blocking
established-decisions#20.1 · yes · the registered hook timeouts bound the check (5 s on SessionStart, UserPromptSubmit and Stop, 3615 s on PreToolUse and PermissionRequest, none on SessionEnd), so the hook.event and hook.dialog verification must finish inside them and still fail open at exit 0 with nothing printed
established-decisions#21.1 · no · the plugin scope decides which hooks and which viola mcp server load into a session, and the chunk changes neither; its check runs inside whichever server answers, and marketplace/ is owned by plugin install
established-decisions#22.1 · yes · each CLI verb's mismatch outcome must be the typed exit 21 and the instance-unreachable --json document that the typed-exit and --json convention sets for every refusal
