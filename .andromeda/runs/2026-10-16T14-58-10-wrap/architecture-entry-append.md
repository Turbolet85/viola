
## 2026-10-16T14-58-10-wrap — 0.1.0 is a pure CLI; the frontend moves to 0.2.0
**Section:** §Stack and Technologies (HTTP server, Push / real-time, Mobile and Browser e2e rows); §Established Decisions ([Concurrency], [Real-time Strategy], [GUI Control Scope], [Mobile], [Deferred], and the wheel, link and budget entries); §Conventions (interfaces, `v`, GUI routes, GUI HTTP errors, error enums, CLI flags); §Standard Contracts (the GUI block, the `list` payload); §Occupied Resources (protocols, Network, HTTP routes, `ui/<port>.url`, role logs, `config.json`, `budget.json`, stamps); §Cross-cutting Patterns; §Project Intent; §Inherited Defaults; §Infrastructure Patterns → Deployment model, Build system, Crate dependency direction, Project directory structure
**Change:**
- Was a view-only loopback GUI: `viola ui` (axum + tower-http on `127.0.0.1:47319`), SSE `/api/events`, cookie-gated `/api/*`, RFC 9457 errors, `ui/<port>.url`, the `viola-ui` crate, Tokio in `mcp` and `ui`. Now the frontend is deferred to 0.2.0 and its technology is not chosen. [Real-time Strategy], [GUI Control Scope] and the GUI contracts block hold that stub line. [GUI Control Scope] keeps what the page shows as the 0.2.0 frontend's starting intent.
- Four interfaces from one binary. Tokio only in `viola-mcp`. No `ui` subcommand, `--port`, GUI port setting, `UiError`, `ui-<port>` role file, notify tail or listening TCP port.
- The item, `budget` and `skipped` shapes leave the GUI envelope and stand as the `list` payload contract (CLI `list --json`, MCP `list`). §Conventions, the board read and the MCP rows cite it.
- The product type's third part is the frontend, deferred to 0.2.0. A phone view is a later version, behind authentication.
- Build system: the 2026-09-30 React + TypeScript ruling is replaced by "its technology is not chosen".
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI, moves the whole frontend to 0.2.0 and cuts the masters' web sections to a stub. The operator's answer at this wrap (overseer, technical, founder-delegated) keeps the masters true to the tree, so the browser test leg stays, marked as owned by the route entry "Remove the browser leg".
**Kept:**
- The browser-e2e Stack row and its Occupied Resources rows, marked.
- `deny.toml`'s `axum` `http2` ban and the `crates/viola-ui/` tsconfig guard.
- The absences that stay true: no WebSocket and no polling endpoint.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/
