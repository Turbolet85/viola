# obs-plan · picked accounting

## Picked
obs-scope-summary#11.3 · proposed · the inbound MCP stdin payload it names is now read in lines of at most `MAX_FRAME`, a longer line ending the session with one `parse-rejected{mcp-stdin}` (report Symbols / APIs, `serve`)
obs-scope-summary#22.3 · proposed · the `viola mcp` surface the report names as the site for the bounded stdin (`obs-plan.md:125`); carries F2 beside its exporter line, plus the broad Surface line in the body
obs-scope-summary#49.1 · proposed · the per-parser parse-failure counts list gains the MCP stdin (report Counts moved, `parse-counts`, `obs-scope-summary.md:500` change)
telemetry-strategy#3.15 · not touched · the row maps the creator-explicit trigger to §3 and §6; the schema gained one `parser` value under §6 and the mapping still holds
log-coverage#1.11 · proposed · the `parse-rejected` `parser` list moves 9 → 10 values with `mcp-stdin` (report Schema / config `:87`, `log-coverage.md:20` change)
log-coverage#2.1 · not touched · the schema changed only the `parse-rejected.parser` enum at `:87`; no exit code, error variant or `process-exit` detail was added, so the exit-1 details and the 13-value enum still hold
log-coverage#2.5 · proposed · the per-parser details gain `oversize` for `mcp-stdin` (report `log-coverage.md:34` change); carries F1 beside it
log-coverage#2.6 · not touched · no `RefusalReason` or refusal detail was added; the paste rule witnesses stay green unchanged
log-coverage#3.1 · not touched · no spawn, `subject` value or spawn pair changed; the schema change is the `parser` enum alone, and the child pipe drain is only a census row
error-capture-reporting#4.1 · proposed · its witness `tests/chaos_feed_panic.rs` gained the passthrough half after the forced feed panic; carries F6
error-capture-reporting#5.1 · proposed · the MCP error group gains the stdin cut, which is no `isError` reject and ends the session; carries F3
pii-scrubbing-compliance#2.1 · proposed · the content-canary verification gains the `mcp_tools` oversize case reading the canary in every home-level file of its fresh home; carries F4
