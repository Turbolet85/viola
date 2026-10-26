# Classified — arch · established-decisions.b3

## Answers
established-decisions#17.1 · no · heartbeat, pid and start-time liveness, the agents join and the run and revive start order are untouched, and tokio sits only in the mcp and channel client code
established-decisions#18.1 · no · no error enum, anyhow use or fixed message changes, and thiserror 2.0.20 and anyhow 1.0.104 are other lock units the chunk only keeps holding, a re-check every change owes
established-decisions#19.1 · yes · the tokio containment the chunk re-reads binds: viola-mcp is the only product crate enabling tokio, viola-channel's tokio feature, viola-e2e's rt and process, and fuzz/ stays its own workspace, so the bump may add no tokio edge
established-decisions#20.1 · no · the embedded plugin, the pinned-copy content-hash key and the install command are untouched, and the rebuilt exe's new hash is what that key already absorbs
established-decisions#21.1 · no · no plugin file, .mcp.json registration or plugin install document changes
established-decisions#22.1 · no · no CLI flag, --json document or exit code changes
