# Classified — arch · established-decisions.b3

## Answers
established-decisions#27.1 · no · standing S2 · each new test runs against the fake agent; the real claude runs only in local live tests, never in CI
established-decisions#28.1 · yes · verify's home-tree check runs viola_state::strict::check_home_tree, where the chunk's Unix read and write rules land; its refusal is exit 1 with the home pair
established-decisions#28.2 · yes · verify's strict-modes-failed code is what the chunk's stricter Unix read rule, through viola_state::strict, reaches on the stamps path
established-decisions#29.1 · yes · the start order (strict checks, bind, first snapshot, spawn) where the socket-directory check must sit before bind, with the run-prints-nothing gate rule for its refusal; a revived start runs it too
established-decisions#29.2 · yes · collision_check's stale-heartbeat and already-live outcomes, and a stale wrapper's instance-dead reading, are what the killed-wrapper leftover socket takeover and the root kill probe rest on
established-decisions#29.3 · yes · verify_server's peer pid and OS start-time check against the snapshot is the existing server check the chunk sets beside its euid check, on Linux and Windows, with the macOS carve-out
established-decisions#29.4 · yes · without_endpoint's symlink_metadata reading refuses a symlink before connect, the same not-a-symlink reading the client's socket-directory verification must hold before connect
established-decisions#29.5 · yes · run's collision check over a replayed snapshot (no pid, no endpoint) decides free or refused from the heartbeat alone, which the leftover socket file's takeover rests on
established-decisions#29.6 · no · the chunk does not edit revive's preflight snapshot read or its state-recovered line; a revived start meets the new checks through the shared start function, carried by 29.1
established-decisions#29.7 · yes · every start's check_home_tree over home, instance, bin, plugin and pinned copy runs ahead of any log, so the chunk's Unix strict rules must keep the ordering and the no-line refusal
established-decisions#30.1 · yes · one thiserror enum per crate, closed values in viola-core, anyhow only in the root bin: the rule the chunk's new refusal enums follow
established-decisions#30.2 · no · resolve_program's two refusals (BatchScriptChild, ProgramNotFound) are not new and not reached; the chunk does not edit program resolution, which runs before the socket check
established-decisions#30.3 · yes · the census of nine thiserror derives must name and count the new error enums the chunk adds
established-decisions#31.1 · yes · viola-channel's tokio is optional and no product member but viola-mcp reaches tokio; the chunk's peer-euid code in the channel seam must stay tokio-free, with new dependencies inside the pins
established-decisions#32.1 · no · pinned copy, hook registration and plugin files are the chunk's Not-needed set; the chunk changes none of them, and viola mcp's exit-1 start is unchanged
established-decisions#33.1 · no · pin_key and pin_bytes naming, placement and hashing are the chunk's Not-needed set; the endpoint's checks do not move the pinned copy
