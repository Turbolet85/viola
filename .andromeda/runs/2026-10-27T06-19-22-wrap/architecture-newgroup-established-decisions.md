# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade.md 2 · distinct · no standing group states the `viola mcp` stdin line cap or `CappedLines`; [MCP] (lines 99–112) states the tools, inputs, `WAIT_CAP` and the instance check only
- fanout-architecture--cascade.md 3 · distinct · no standing line states the `parse-rejected{parser:"mcp-stdin"}` line or `serve`'s exits after a cut; line 243 names `ServeError`'s variants only, not when `Initialize` is returned
- fanout-architecture--cascade.md 4 · distinct · no standing group states rmcp 3.4.1's unbounded stdin `read_until` or that its codec's max length serves only the writer
- fanout-architecture--cascade.md 6 · distinct · [Screen Model] (line 23) states the caught vt100 feed panic poisoning the model, but no standing line states the Windows ConPTY reading of the child's wide character after it or its owner
