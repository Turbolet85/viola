
## 2026-10-16T14-58-10-wrap — the loopback GUI leaves 0.1.0: no HTTP surface, no GUI secret
**Section:** Threat Model Summary (scope, the Local HTTP vector, data classification, the elevations); Authentication & Authorization (auth approach, the token, cookie and GUI principal rows, recovery flows, strict-modes `ui/` reads); Input Validation (`Last-Event-ID`, Host and brake rows); Data Protection; API Security; Dependency Security; Bootstrap phases; Secret Management; Error Handling; Security Anti-Patterns
**Change:**
- Was a loopback HTTP GUI: per-launch 256-bit token, `ui/<port>.url`, `viola_<port>` cookie, Host allowlist, CSP, `/api/*`, SSE, RFC 9457 Problem Details, a threat vector and its elevations. Now viola has no HTTP surface. API Security holds the stub line ("The frontend is deferred to 0.2.0; its technology is not chosen.") and keeps only the MCP transport row.
- The only secrets are the inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and the R8-stripped `CLAUDE*` values. The NEVER-log floor and the external-error rules name no GUI token, cookie, launch URL, `/api/info` or Problem Details.
- The strict-modes and DACL reads no longer name `ui/`. The elevations are two, not three.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen.
**Kept:** By the operator's answer at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree.
- The npm / Node / npm-audit part of Dependency Security stands, marked as owned by the route entry "Remove the browser leg".
- The secret-scan classes `?t=`, `cookie:` / `viola_<port>=` and the GUI token from `ui/*.url` stand, with their `?t=` note, because `secret_scan.rs` still tests them.
- The `axum` `http2` ban stands, because `deny.toml` still holds it.
- The listener ban stands (only «/GUI» is cut).
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/
