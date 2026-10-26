# Classified — architecture · established-decisions.b3

## Answers
established-decisions#21.1 · no · the liveness rule, the run start order and revive's preflight are unchanged, and the start's link restore still follows the log
established-decisions#21.2 · yes · add-to · F6: the link and unlink verbs run the strict-modes check on the home and instances/<driven> before client::start, ahead of the liveness pre-check and the server identity check
established-decisions#22.1 · no · no dependency or crate changed and the root bin gained only two command modules, so anyhow stays bin-only and no library crate gains an enum
established-decisions#23.1 · no · no crate, dependency or workspace member changed, and the root bin gained only two command modules
established-decisions#24.1 · no · hooks, the MCP tool set and the ConPTY binaries are untouched, and the run.rs change serves the link and unlink methods without touching the start's pin or plugin steps
established-decisions#25.1 · no · no MCP tool was added and link and unlink stay CLI-only, so the installed plugin's viola mcp surface and the --plugin-dir loading are unchanged
established-decisions#26.1 · no · the new verbs follow the convention as built, with human text by default, --json, and typed exits 21 and 20
