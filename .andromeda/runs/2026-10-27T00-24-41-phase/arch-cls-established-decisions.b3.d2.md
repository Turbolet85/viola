# Classified — arch · established-decisions.b3

## Answers
established-decisions#22.1 · yes · Run B's statusLine command is a `hook statusline` spawn of the pinned copy after verify's pin_exe call, so the fix must hand over a complete copy before the probes; the ledger rows and probe rules are not reached
established-decisions#23.1 · no · verify's pre-log home check (home, ledger/, bin/, plugin/, None and None) and the stamps writer's check are units of verify.rs the change does not edit, which edits only its pin_exe call
established-decisions#24.1 · yes · the start order puts the pinned copy and plugin folder step (the chunk's run pin_bytes call and per-start plugin rewrite) after the collision check and before the ConPTY sideload, version gate and endpoint bind; the heartbeat and gone rules are not reached
established-decisions#24.2 · no · the snapshot's pid and started_at as server identity (live_endpoint, verify_server) govern frames, which the chunk sends none of and whose methods it does not change
established-decisions#24.3 · no · run's collision check over one snapshot read and its heartbeat-only rule over a replay are units of run.rs the change does not edit; the replay readers are not reached
established-decisions#24.4 · no · revive's preflight single snapshot read and its one state-recovered line are not reached; the chunk edits no preflight or replay unit
established-decisions#24.5 · yes · checked_exe's one read and pin key, then check_home_tree over bin/<key>/, the pinned copy and plugin/<key>/ before any log; run pins the bytes it checked, and a new .lock or temp entry there must pass that write rule
established-decisions#25.1 · no · standing S7 · a new error type follows the per-crate <Crate>Error thiserror enum with fixed messages, a convention the task lists
established-decisions#26.1 · yes · the fix stays std-only in viola-state, a sync crate: no tokio in its normal graph, and only viola-mcp enables tokio
established-decisions#27.1 · yes · the pinned copy's content-hash key and re-hash-before-reuse refusal, the companions' write-if-absent and per-start re-hash, and the per-start plugin rewrite to the absolute pinned command are what the fix keeps
established-decisions#28.1 · yes · pin_key, pin_bytes and pin_exe are the units the chunk edits, and run's start pins the same bytes its home-tree check read
established-decisions#28.2 · yes · pin_companions' owner/DACL check on conpty/ and each companion, before the held open, with a refusal writing nothing and falling back to the inbox ConPTY, sits in the edited pin_companions
