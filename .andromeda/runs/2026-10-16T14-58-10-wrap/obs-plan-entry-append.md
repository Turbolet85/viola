
## 2026-10-16T14-58-10-wrap — the viola-ui entity, web-spa surface and SSE correlation leave 0.1.0; the vocabulary stays reserved
**Section:** §1 Obs Scope Summary (counts, entities, surfaces, triggers); §2 Telemetry Strategy; §3 Observability Harness Contract (and its keys: Bootstrap phases, Heartbeat ticks, Log file location, Log format JSON schema, Logging stack, OTel SDK init, Service identity, Snapshot paste-to-AI, Trace context propagation; the index label `HTTP boundaries`); §4 (the GUI view scenario, E4, instrumentation rows, the frontend boundary); §5; §6; §7; §8 (item 3 HTTP layer, items renumbered); §9; §10; §11
**Change:**
- The viola-ui and embedded-GUI entities, the web-spa surface and the Vector 3 trigger are cut, and so are the TraceLayer / tower-http / axum spans and bans, the `ui.*` spans, the HTTP layer and SSE keep-alive, `ui/<port>.url`, the launch line, `/api/*`, `/health`, Problem Details, `UiError`, the ui port parse and the notify tail.
- The GUI view scenario keeps its heading over the stub line «The frontend is deferred to 0.2.0; its technology is not chosen.». §8's old item 6 is item 5.
- Recounted: five surface entries, two product parts, six instrumentable crates, six must-trace paths, four edge flows, eighteen triggers.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen.
**Kept:** By the operator's two answers at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree.
- The closed vocabulary (`ObsProcess::Ui`, `ObsEvent::HttpRequest` / `LivenessChanged` / `SseOpened` / `SseClosed`, the `ui-<port>` role file, the `last-event-id` parser, the `viola_ui` / axum / tower_http / hyper targets) stays in the catalogue, enums and the bound §3 Log-format copy. Each mention is marked «reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg"».
- The secret-scan classes and the browser-leg CI text stand, the latter with one marker line in §9.
- The §3 copy already lagged test-plan before this pass (no `cli-<name>`, no detail sentence). It is named here; this pass does not widen it.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/
