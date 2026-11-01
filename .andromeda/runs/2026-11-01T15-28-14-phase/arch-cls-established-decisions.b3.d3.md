# Classified — arch · established-decisions.b3

## Answers
established-decisions#27.1 · yes · the version gate runs the resolved child with `--version` before the endpoint bind, so the socket-directory refusal at a wrapper's start must come ahead of it (no child before the refusal); the ledger rows and stamps stay out.
established-decisions#28.1 · yes · `verify` calls `strict::check_home_tree`, the check whose Unix read and write rules the chunk applies; a refusal there is exit 1 with the home pair and no line, which the new strict-modes checks must keep.
established-decisions#28.2 · no · verify's own `Ended::Refused` process-exit code set; the chunk adds no code to it and no verify path moves, and the check_home_tree refusal is covered by #28.1.
established-decisions#29.1 · yes · the start order: the socket-directory check and the endpoint bind sit after the collision check and before the first snapshot, the directory check ahead of the version gate's child spawn, and the strict-modes check ahead of the log.
established-decisions#29.2 · yes · `live_endpoint` gives none for a stale wrapper, so no client's directory check or connect runs against it, and the stale-heartbeat and already-live outcomes decide which starts reach the bind that takes over a leftover socket file.
established-decisions#29.3 · yes · the server identity check (`verify_server`, pid and OS start time from the snapshot) is the existing server check the chunk extends with the peer euid check and records the macOS peer-pid carve-out against.
established-decisions#29.4 · yes · the `without_endpoint` symlink_metadata reading stays behind the strict-modes instance check at every client, the order the chunk's not-a-symlink and euid checks join before connect.
established-decisions#29.5 · yes · a replayed or failed snapshot read decides the collision check from the heartbeat alone, which fixes when a killed wrapper's leftover socket name counts as free for the next bind's takeover.
established-decisions#29.6 · yes · revive's preflight reads the snapshot once and emits one state-recovered line; a directory or endpoint check the chunk adds on the revive path must not add a second replay or line.
established-decisions#29.7 · yes · every start runs `check_home_tree` over the home, instance, `bin/`, `plugin/` and pinned paths before any log opens, and a refused check writes no line; this is where the chunk's Unix read and write rules run.
established-decisions#30.1 · yes · the new refusals take one thiserror enum per crate, closed values extended in `viola-core`, and anyhow only in the root bin; this binds the chunk's new error types.
established-decisions#30.2 · no · `AgentError` holds only `resolve_program`'s two refusals; the chunk's new refusals are not program-resolution refusals and that enum is not reached.
established-decisions#30.3 · yes · the thiserror census must name any new derive the chunk adds for its refusals (the `cross_exit_causes` and channel refusal types).
established-decisions#31.1 · yes · the `TokioClient`'s peer euid check stays behind `viola-channel`'s optional `tokio` feature, and no tokio enters another product crate's normal graph (sole-root ban, `scripts/channel-tokio-free.sh`).
established-decisions#32.1 · no · pinned copy placement, hook and `.mcp.json` registration and the plugin contents are on the Not needed list and do not change with the endpoint's checks.
established-decisions#33.1 · no · `pin_key` and `pin_bytes` hashing and placement are on the Not needed list; the start's home-tree check only consumes the key (covered by #29.7), and the chunk does not change the pin.
