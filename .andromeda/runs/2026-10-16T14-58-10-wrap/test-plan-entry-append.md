
## 2026-10-16T14-58-10-wrap — the GUI surfaces and their tests leave 0.1.0; the harness keeps what the tree holds
**Section:** §1 Test Scope Summary (entities, surfaces, triggers, coverage counts); §2 Test Strategy; §3 Test Harness Contract → 5-command implementation, Status endpoint shape, Log format, PID file, Test data bootstrap, Bootstrap phases (and the index labels); §4–§6 (viola-ui unit tests, GUI HTTP integration, drivers, selector strategy, Paths 1–7, E1, E3, E4, security sweep, cross-surface parity, bay layout states); §7–§11
**Change:**
- Was an api-service and a web-spa surface: `viola ui` boot with `--ui`, `/health` / `/ready` / `/api/*` reads, the GUI token exchange, SSE E4, Playwright web assertions inside Rust paths, the loopback-GUI security sweep, `http_` / `sse_` tests, the viola-ui unit tests and axum-test integration. Now all of it is cut.
- E4 keeps its heading over the stub line «The frontend is deferred to 0.2.0; its technology is not chosen.». E3 keeps its CLI link half.
- The status shape is `viola list --json` alone. Its item, `budget` and `skipped` shapes are the `list` payload contract.
- `cleanup` has no UI port or url-file step.
- Recounted: six surface entries, fifteen §1 entities, six crates, six parser surfaces and six fuzz parser targets (a seventh seed target), four expansion scenarios.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen.
**Kept:** By the operator's two answers at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree. Each item below is owned by the route entry "Remove the browser leg".
- The browser test leg: `run --browser`, the `playwright` suite and gate value, `browser-missing`, Node, npm and the CI browser job, with one marker line in §9.
- The harness leftovers: `status`'s `null` `ui` and `api_sessions_equal_list`, `cleanup`'s `null` `port_free` / `url_file_removed`, the `token-query` / `gui-token` / `cookie` scan classes, `ui-restart` forwarding, the `fixed-port` nextest group and the `cookie_file` field.
- The closed obs vocabulary in the bound Log format key (the `ui` process, `ui-<port>`, `http-request`, `liveness-changed`, `sse-*`), marked reserved: no emitter in 0.1.0.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/
