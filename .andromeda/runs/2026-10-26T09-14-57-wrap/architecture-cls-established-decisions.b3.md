# Classified — architecture · established-decisions.b3

## Answers
established-decisions#17.1 · yes · add-to · the snapshot's pid and started_at now also identify the server that every frame-writing client checks before its first frame, after the liveness-only pre-check (live_endpoint now returns LiveEndpoint from the one read)
established-decisions#18.1 · no · the new root-bin Unreachable and CallError and viola-state's ServerVerdict are not crate error enums, so the one-enum-per-crate text is not broken
established-decisions#19.1 · no · no crate, feature or dependency changed and the tokio containment re-read is unmoved (viola-channel's normal graph stays tokio-free, and the MCP start-time read runs on spawn_blocking inside viola-mcp, the one crate that enables tokio)
established-decisions#20.1 · no · no plugin file, hooks.json entry, timeout or pinned-copy rule is reached or touched, and the only hook change is one shared connect inside src/cmd/hook.rs
established-decisions#21.1 · no · the session changes the MCP frame tools and the hook's connect, while plugin install is listed as unchanged and no plugin load path is touched
established-decisions#22.1 · no · the session adds no exit code, --json code or refusal, a server mismatch is the existing exit-21 unreachable outcome with its unchanged document, and human text stays the default
