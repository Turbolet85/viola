# Classified — arch · established-decisions.b3

## Answers
established-decisions#21.1 · yes · the verbs' liveness pre-check (pid and start time alive, heartbeat live, endpoint present, else exit 21), the start's restore of the link set from the log, and the append-only events.ndjson
established-decisions#21.2 · yes · the server identity check every frame-writing client makes before its first frame (snapshot pid and started_at against the connected peer, macOS the recorded gap), which the two verbs take through client::connect
established-decisions#22.1 · no · standing S5 · every new error type or variant is a thiserror enum with fixed messages, anyhow only in the root bin, a convention the task lists
established-decisions#23.1 · yes · the two verbs and the wrapper's two methods run on the sync client and std-thread server, so no product crate's normal graph may reach tokio
established-decisions#24.1 · no · the chunk adds no plugin file and touches no pinned copy, ConPTY companion or install rule (the task's Not needed list)
established-decisions#25.1 · no · the chunk adds no plugin file and no MCP tool (link and unlink are CLI-only), so the plugin scope and the installed plugin's MCP registration are untouched
established-decisions#26.1 · yes · the CLI conventions the two verbs must honour: human text by default, --json for agents, a typed exit code per refusal
