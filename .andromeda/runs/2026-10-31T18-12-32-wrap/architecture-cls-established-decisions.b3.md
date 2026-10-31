# Classified — architecture · established-decisions.b3

## Answers
established-decisions#27.1 · yes · add-to · verify's refusals now log an Ended code on process-exit (F13, a failing row none), and the unclean-recording refusal's wording stands
established-decisions#28.1 · no · the home-tree refusal stays lineless before the log opens (F13), and the new codes mark only refusals after that
established-decisions#29.1 · yes · add-to · the stale holder's refusal now reads stale-heartbeat and a live holder's reads already-live (F9), the refuse-to-start rule standing
established-decisions#29.2 · yes · add-to · without_endpoint is the no-endpoint split beside live_endpoint (F3), and MCP's frame tools read the no-endpoint, refused and failed-verify cases to causes (F6)
established-decisions#29.3 · no · a fresh replayed beat still refuses already-live and an older one stays free, as F9 keeps them, and no replayed-path change is reported
established-decisions#29.4 · no · the preflight's one snapshot read and its one state-recovered line are untouched, since the revive --json change sits at the start after preflight
established-decisions#29.5 · yes · stale · under revive --json a refusal of the start, the home-tree one included, prints one exit-1 document where this row states the home pair alone
established-decisions#30.1 · no · viola-core gains only UnreachableCause, a plain enum and not a thiserror error or RefusalReason, so the error enums and RefusalReason gain nothing
established-decisions#30.2 · no · AgentError's two refusals and resolve_program's signature sit in viola-agent-claude and src/run/mod.rs, neither reached, and the session maps them only at the caller
established-decisions#30.3 · no · the crate error enums gain nothing, so the nine thiserror derives stand
established-decisions#31.1 · no · no member, dependency edge or module boundary changes, viola-core stays pure, and the root Cargo.toml change is a test target only
established-decisions#32.1 · no · the pinned-copy refusal stays exit 1 with pinned-hash-mismatch, and hooks.json, .mcp.json and the plugin files are unmoved since hook.rs is not in the diff
established-decisions#33.1 · no · run's one exe read, its pin key, the pin_bytes call and verify's pin_exe are not among the changed outputs, which are error documents around them
established-decisions#33.2 · no · the sideload's strict-modes fallback is unmoved, the session's StrictModes documents are CLI client refusals, and src/conpty.rs is not reached
established-decisions#33.3 · no · the two-builds measurement is of a dated tree, and the only Cargo.toml change is a test target outside the release build
