# Classified — arch · established-decisions.b3

## Answers
established-decisions#27.1 · yes · the start runs the `--version` child probe before endpoint bind, so the socket-directory refusal must come before any child is spawned; the ledger rows themselves are Not needed
established-decisions#28.1 · yes · verify runs `strict::check_home_tree` before its log opens and a refused home writes no line, the rule the chunk's strict-mode checks run through
established-decisions#28.2 · yes · verify's `strict-modes-failed` refusal, reached through `check_home_tree`, which the chunk's read rule changes, and its no-line refusal
established-decisions#29.1 · yes · the start order puts the new socket-directory check before the endpoint bind and before the first snapshot and spawn, after the strict-modes check; the `.lock` arbiter sits in this chain
established-decisions#29.2 · yes · `collision_check` outcomes and `live_endpoint` giving none for a stale wrapper (`instance-dead`) bound the leftover-socket takeover and the client's recorded-path verification
established-decisions#29.3 · yes · the existing server check (peer pid and OS start time against `pid` and `started_at`, `verify_server`) that the chunk sits beside, with the Linux and Windows comparison and the macOS carve-out
established-decisions#29.4 · yes · `without_endpoint`'s `InstanceDead` and `UnwrappedName` readings feed the `UnreachableCause` set the chunk extends, and its strict-check-first placement at every caller
established-decisions#29.5 · yes · a replayed snapshot carries no endpoint, and a free collision decision lets the bind take over the leftover socket file the chunk re-measures
established-decisions#29.6 · no · revive's single-read preflight is untouched; the chunk's start-side directory check reads no snapshot and sits after the preflight
established-decisions#29.7 · yes · every start runs `check_home_tree` on the home, instance set, `bin/` and `plugin/` before its log opens, and a refused check writes no line, the rule the chunk's Unix strict modes follow
established-decisions#30.1 · yes · one thiserror enum per crate, closed values extended in `viola-core`, and anyhow only in the root bin, for the new refusals
established-decisions#30.2 · no · `AgentError`'s two `resolve_program` refusals are not changed; the chunk's new refusals sit after program resolution and in the channel seam
established-decisions#30.3 · yes · the census of nine thiserror derives, which the new refusal enums extend
established-decisions#31.1 · yes · `viola-channel`'s `tokio` feature and the sole-root tokio ban bind the new peer-euid check added to the Tokio client that `viola mcp` uses
established-decisions#32.1 · no · plugin files, the nine registered hook entries, the `.mcp.json` registration and the pinned copy are named unchanged in Not needed, and no rule here moves with the endpoint checks
established-decisions#33.1 · no · `pin_key`, `pin_bytes` and `pin_exe` are pinned-copy hashing, named unchanged in Not needed, and the chunk's start-side directory check uses no pin key
