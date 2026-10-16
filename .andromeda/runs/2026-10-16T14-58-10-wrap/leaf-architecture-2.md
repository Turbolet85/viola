group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/services/viola-state.md:6   class: retired
  leaf-text: «notify 8.2.0 tailing, liveness (heartbeat + sysinfo pid/start time)»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «sysinfo (the pid + start-time liveness check, shared by CLI `list` and `mcp`), tempfile (the one `persist` helper), thiserror, tracing; libc»
  replace-with: «liveness (heartbeat + sysinfo pid/start time)»

- leaf: .claude/docs/services/viola-state.md:11   class: retired
  leaf-text: «thiserror; tracing; notify 8.2.0 with tailing; windows-sys»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «sysinfo (the pid + start-time liveness check, shared by CLI `list` and `mcp`), tempfile (the one `persist` helper), thiserror, tracing; libc»
  replace-with: «thiserror; tracing; windows-sys»

- leaf: .claude/docs/services/viola-state.md:14   class: retired
  leaf-text: «`viola-mcp`, `viola-ui` (tail for SSE, sessions/links envelopes).»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «`viola-mcp`.»

- leaf: .claude/docs/services/viola-state.md:17   class: retired
  leaf-text: «Tokio banned in its graph; tailing and liveness are sync (shared by `list`, `mcp`, `ui`).»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «sysinfo (the pid + start-time liveness check, shared by CLI `list` and `mcp`), tempfile (the one `persist` helper), thiserror, tracing; libc»
  replace-with: «Tokio banned in its graph; liveness is sync (shared by `list` and `mcp`).»

- leaf: .claude/docs/services/viola-state.md:29   class: retired
  leaf-text: «- Tailing only `instances/<name>/events.ndjson` where `<name>` passes `ViolaName::try_new`; symlinks ignored (`parse-rejected{parser:"state-entry", detail:"symlink-ignored"}`).»
  source: .andromeda/architecture.md:343 «one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero scan count (naming no entry)»
  replace-with: «- Symlinks under `instances/` are ignored: `viola list` logs one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero scan count, naming no entry.»

- leaf: .claude/docs/services/viola-state.md:39   class: retired
  leaf-text: «**Logs / snapshots / tail / liveness:** same crate»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:68 «holds `endpoint`, `pinned_bin`, `cwd`, `statusline_command`, `pid`, `started_at` or `links`)»
  replace-with: «**Logs / snapshots / liveness:** same crate»

- leaf: .claude/docs/services/viola-ui.md:1   class: tree
  leaf-text: «# viola-ui»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(remove this leaf: `viola-ui` is no longer a workspace crate; the frontend is deferred to 0.2.0 and its technology is not chosen)»

- leaf: .claude/docs/services/viola-ui.md:3   class: retired
  leaf-text: «[GUI Control Scope], [Real-time Strategy], §Standard Contracts (GUI routes, SSE feed, list envelopes)»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:6   class: retired
  leaf-text: «`viola ui`: the view-only loopback HTTP server (axum 0.8.9 + tower-http 0.7.1) on `127.0.0.1:47319`»
  source: .andromeda/architecture.md:291 «- **No database port, no broker port, no listening TCP port.** The MCP server is stdio only.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:11   class: retired
  leaf-text: «`viola-core`, `viola-state` (tail, sessions, links, liveness, budget), `viola-agent-claude` (`claude agents --json`); axum, tower-http»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:14   class: retired
  leaf-text: «The browser page (React + TypeScript bundle from the route's frontend-toolchain entry; toolchain OPEN until then)»
  source: .andromeda/architecture.md:106 «the frontend (deferred to 0.2.0; its technology is not chosen)»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:17   class: retired
  leaf-text: «Tokio allowed. axum without `http2`. No CORS layer, no `ServeDir`.»
  source: .andromeda/architecture.md:411 «Only `viola-mcp` may build a runtime.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:20   class: retired
  leaf-text: «- Layer order: Host allowlist (outermost»
  source: .andromeda/architecture.md:92 «- **[GUI Control Scope]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:21   class: retired
  leaf-text: «- Token: 32 `getrandom` bytes, hex, memory only»
  source: .andromeda/architecture.md:92 «- **[GUI Control Scope]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:22   class: retired
  leaf-text: «- SSE: `id:` = composite cursor»
  source: .andromeda/architecture.md:90 «- **[Real-time Strategy]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:23   class: retired
  leaf-text: «- Logs: `http-request` with `uri.path()` only»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:24   class: retired
  leaf-text: «- Page: `/assets/app.css` token layers»
  source: .andromeda/architecture.md:106 «the frontend (deferred to 0.2.0; its technology is not chosen)»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:27   class: retired
  leaf-text: «- A v1.x brake route (`POST /api/sessions/{name}/pause|unlink`)»
  source: .andromeda/architecture.md:92 «- **[GUI Control Scope]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:28   class: retired
  leaf-text: «- tower-http 0.7 compression can 406 on `Accept-Encoding: identity;q=0`»
  source: .andromeda/architecture.md:90 «- **[Real-time Strategy]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:29   class: retired
  leaf-text: «- `/ready` 503 uses its own readiness body»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:32   class: tree
  leaf-text: «- **Router, middleware, handlers:** `crates/viola-ui/src/`»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:33   class: tree
  leaf-text: «- **Embedded page:** `crates/viola-ui/assets/`»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:34   class: retired
  leaf-text: «- **Tests:** crate `tests/` with axum-test (route/header/Problem matrix)»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:37   class: retired
  leaf-text: «- **Unit tests:** `cargo nextest run -p viola-ui`»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:38   class: retired
  leaf-text: «- **Integration:** axum-test in-process; every request sets Host explicitly»
  source: .andromeda/architecture.md:314 «- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola-ui.md:39   class: retired
  leaf-text: «- **Browser (ubuntu):** `scripts/agent-run.sh run --browser` — bay layout states, axe verdicts, keyboard walk, announcements»
  source: .andromeda/architecture.md:92 «- **[GUI Control Scope]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/services/viola.md:14   class: retired
  leaf-text: «the MCP server via `viola mcp`), the page (`viola ui`).»
  source: .andromeda/architecture.md:301 «- Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.»
  replace-with: «the MCP server via `viola mcp`).»

- leaf: .claude/docs/stack.md:9   class: stale
  leaf-text: «rmcp 1.88, axum 1.80)»
  source: .andromeda/architecture.md:13 «sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88). CI checks the floor in the `msrv` job»
  replace-with: «rmcp 1.88)»

- leaf: .claude/docs/stack.md:10   class: stale
  leaf-text: «Tokio 1.53.1 built only inside `mcp`, `ui` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp and axum require it |»
  source: .andromeda/architecture.md:14 «Tokio 1.53.1 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it |»
  replace-with: «Tokio 1.53.1 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it |»

- leaf: .claude/docs/stack.md:11   class: retired
  leaf-text: «| HTTP server (GUI) | axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, with `text/event-stream` excluded) | View-only GET routes and the SSE feed on 127.0.0.1 |»
  source: .andromeda/architecture.md:106 «the frontend (deferred to 0.2.0; its technology is not chosen)»
  replace-with: «(delete this line)»

- leaf: .claude/docs/stack.md:12   class: stale
  leaf-text: «`run · revive · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`»
  source: .andromeda/architecture.md:15 «Subcommands `run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`»
  replace-with: «`run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`»

- leaf: .claude/docs/stack.md:25   class: stale
  leaf-text: «for request/response; notify 8.2.0 tailing the ndjson logs for fan-out to `ui` | Local IPC and event fan-out without a broker or daemon |»
  source: .andromeda/architecture.md:28 «| Message broker | None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response | Local IPC without a broker or daemon |»
  replace-with: «for request/response | Local IPC without a broker or daemon |»

- leaf: .claude/docs/stack.md:26   class: retired
  leaf-text: «| Push / real-time | SSE through axum 0.8.9 `Sse::keep_alive`, fed by notify 8.2.0 | Live GUI feed `/api/events` |»
  source: .andromeda/architecture.md:90 «- **[Real-time Strategy]** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «(delete this line)»

- leaf: .claude/docs/stack.md:30   class: stale
  leaf-text: «The phone view is a later version: the same web page behind authentication»
  source: .andromeda/architecture.md:32 «| Mobile framework | N/A | A phone view is a later version, behind authentication |»
  replace-with: «A phone view is a later version, behind authentication»

- leaf: .claude/docs/stack.md:33   class: omission
  leaf-text: «(axe lands with the a11y chunks). Not a Cargo dependency |»
  source: .andromeda/architecture.md:35 «stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text.»
  replace-with: «(axe lands with the a11y chunks). Not a Cargo dependency. The browser test leg named here (the Node pin, `scripts/install-node.sh` and the CI workflow's `NODE_PIN_*` lines, `e2e-web/` with `npm ci` and its lockfile audit `scripts/npm-audit.sh`, Playwright and its bundled Chromium, the suite `run --browser` and the native `pre-push` browser stage, the test-side install sites, and the `target/npm-audit/`, `e2e-web/` and `junit-playwright.xml` outputs under Occupied Resources) stands until the route entry "Remove the browser leg" removes it; that entry's wrap amends this text. |»

- leaf: .claude/docs/stack.md:57   class: stale
  leaf-text: «React + TypeScript, a built bundle embedded in `viola` (founder ruling 2026-09-30); the bundler, versions, embedding, CSP needs and npm gates are OPEN, owned by the route's frontend-toolchain entry, and until it lands the page has no JS build step»
  source: .andromeda/architecture.md:106 «the frontend (deferred to 0.2.0; its technology is not chosen)»
  replace-with: «The frontend is deferred to 0.2.0; its technology is not chosen»
