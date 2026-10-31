# Classified — architecture · established-decisions.b3

## Answers
established-decisions#27.1 · yes · stale · record and refuse_recording in src/cmd/verify.rs now return Ended, so its unclean-recording refusal reading moved
established-decisions#28.1 · yes · add-to · F13: verify's logged process-exit carries Ended::Refused's code under a parsed VIOLA_NAME, the refused home tree staying lineless
established-decisions#29.1 · yes · add-to · F9: collision_check prints and logs stale-heartbeat for a stale holder and already-live for a live one, the refusal staying exit 1
established-decisions#29.2 · yes · add-to · F3: viola_state::liveness::without_endpoint sits beside live_endpoint behind the strict-modes check; F6: viola-mcp's frame tools map its no-endpoint and failed server_verified readings to causes
established-decisions#29.3 · no · the replayed-snapshot rule holds on the changed collision check: a fresh replayed beat still reads already-live (F9) and an older one stays free
established-decisions#29.4 · no · revive's one snapshot read and one state-recovered line are unchanged; the change adds only an internal-error document on a preflight error
established-decisions#29.5 · yes · stale · under revive --json the home-tree refusal now prints one exit-1 document and checked_exe's error prints internal-error, so its home-pair reading moved
established-decisions#30.1 · no · anyhow stays in the bin, RefusalReason and the crate error enums gain nothing, and UnreachableCause is an as_str cause enum, not an error enum
established-decisions#30.2 · no · AgentError and resolve_program sit in viola-agent-claude and src/run/mod.rs, neither reached, and the two fixed refusal messages are unmoved
established-decisions#30.3 · no · the census of 9 thiserror derives holds, since the session adds an as_str cause enum and no error enum
established-decisions#31.1 · no · no workspace member is added, viola-core stays pure, and the Cargo.toml change is one [[test]] entry
established-decisions#32.1 · no · hooks.json, .mcp.json and plugin.json are not reached, the package version source is unchanged, and pinned-hash-mismatch keeps exit 1 and its code word
established-decisions#33.1 · no · checked_exe still reads the exe's bytes once and pins them through pin_bytes, and verify's change is its log codes and Ended return
established-decisions#33.2 · no · the Windows pin and sideload path (pin_companions, PinError::StrictModes, sideload_fallback) is not in the changes and the exit code is unchanged
established-decisions#33.3 · no · the claim concerns repeated builds of one tree, a build property the session does not touch; Cargo.toml's change is one test entry
