# obs · picked accounting · group 6 of 10

## Picked
span-trace-coverage#1.1 · not in scope · a span-kind record, unemitted in v1; the chunk adds no span and moves no kind
span-trace-coverage#1.2 · not in scope · a span-kind record, unemitted in v1; the chunk adds no server span
span-trace-coverage#1.3 · not in scope · a span-kind record, unemitted in v1; the chunk re-reads `run.readiness_gate`'s outcome, never its kind
span-trace-coverage#2.1 · carried · the client-side `send-refused{side:"client"}` the chunk re-verifies for CLI `send`
span-trace-coverage#2.2 · carried · `run.readiness_gate` and the `pty.*` seam are where the vt100 degrade and the PTY output bound land
span-trace-coverage#2.3 · carried · the hook's bounded stdin and its unresolved-instance exit sit inside this row's `hook.handle` contract
span-trace-coverage#2.4 · carried · channel frames on both sides are a bounded boundary, never recording `params` or result bodies
span-trace-coverage#2.5 · carried · MCP stdin and tool arguments are bounded inside `mcp.tool_call`, rejects codes-only
span-trace-coverage#3.1 · not in scope · the CLI server check is owned by "Unix endpoint and home hardening"; the client paste refusal precedes it and moves nothing in it
span-trace-coverage#3.3 · carried · the instance check on the joined `instances/<name>` path ahead of any role file is the order the CLI name and paste witnesses keep
span-trace-coverage#3.4 · carried · the hook `connect`'s bounded snapshot read keeps its `state.snapshot_recover` span and whole-read rule
span-trace-coverage#3.5 · carried · MCP frame tools' instance check returns `instance-unreachable` with no line, the codes-only reject the chunk re-asserts
span-trace-coverage#4.1 · carried · the start's bounded readers (version probe, snapshot, statusline source, settings, `events.ndjson` restore) keep this span chain and its no-line steps
span-trace-coverage#4.2 · carried · `run.version_gate`'s `cli_verified` is what a bounded or refused ledger read yields, and the probe spawn keeps names-only env fields
span-trace-coverage#4.3 · carried · the version-probe and agents-probe pairs and the oversize `claude-agents-json` rejection are the bounded spawns' lines
span-trace-coverage#4.4 · carried · a bound on the PTY output reader must not make reader EOF an exit source
span-trace-coverage#6.1 · carried · the send path's order (text read, instance check, readiness gate, paste write) is where the paste rule is re-verified
span-trace-coverage#6.2 · carried · `run.readiness_gate`'s `outcome` and `vt100_panicked` are the degrade witness's signal, `text_bytes` the paste's only trace
span-trace-coverage#6.3 · carried · the client and wrapper `control-character` / `empty-text` refusal lines are what the chunk re-verifies per deciding side
span-trace-coverage#6.4 · carried · `send.client`'s exit 13 on a paste refusal and the no-span instance-refused send bound the client witness
