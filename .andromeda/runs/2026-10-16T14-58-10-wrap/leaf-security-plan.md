group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:7   class: retired
  leaf-text: «can allow PermissionRequest tool calls). The loopback GUI streams persistent prompt and tool content. Several state files»
  source: .andromeda/security-plan.md:21 «It still goes beyond a plain Minimal profile in two ways.»
  replace-with: «can allow PermissionRequest tool calls). Several state files»

- leaf: .claude/docs/security-summary.md:10   class: stale
  leaf-text: «OS-user ambient trust enforced per OS (IPC endpoints + `~/.viola/`), plus a per-launch 256-bit GUI token exchanged once for a `SameSite=Strict` HttpOnly cookie (v1 reads and v1.x brake writes)»
  source: .andromeda/security-plan.md:168 «**Auth approach:** OS-user ambient trust, enforced per OS, on the IPC endpoints and `~/.viola/`.»
  replace-with: «OS-user ambient trust enforced per OS (IPC endpoints + `~/.viola/`)»

- leaf: .claude/docs/security-summary.md:17   class: retired
  leaf-text: «- **Another local user reads the feed over loopback TCP** → cookie gate on `/api/*` + SSE; Host allowlist first (DNS rebinding); no CORS; CSP + Trusted Types; text-only rendering.»
  source: .andromeda/security-plan.md:12 «_**Stack adaptability:** viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract. It has no public network surface.»
  replace-with: «»

- leaf: .claude/docs/security-summary.md:20   class: stale
  leaf-text: «render as text»
  source: .andromeda/security-plan.md:212 «a row's `name` is never a `ViolaName` or `target` in CLI `list` or the MCP `list` tool. Human-mode CLI `list` escapes C0/C1 controls in it before printing (§ Logging).»
  replace-with: «are escaped of C0/C1 controls before human-mode CLI `list` prints them»

- leaf: .claude/docs/security-summary.md:27   class: retired
  leaf-text: «| Critical (secret) | GUI token, launch URL, `viola_<port>` cookie, `CLAUDE_CODE_MESSAGING_TOKEN`/`_SOCKET`, other R8-stripped `CLAUDE*` | Never logged anywhere; token only in process memory + the 0600 `ui/<port>.url` + one stderr launch line; rotated every launch |»
  source: .andromeda/security-plan.md:441 «- The inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `CLAUDE_CODE_MESSAGING_SOCKET`.»
  replace-with: «| Critical (secret) | `CLAUDE_CODE_MESSAGING_TOKEN`/`_SOCKET`, other R8-stripped `CLAUDE*` | Never logged anywhere; held only in the `viola run` wrapper's environment, stripped from the child |»

- leaf: .claude/docs/security-summary.md:28   class: stale
  leaf-text: «Only in contract payloads (`events.ndjson`, SSE, `wait`/`last`, `hook.dialog`) and `instances/<name>/diagnostics/detail-*`; never in stderr, `--json` errors, MCP errors, Problem Details»
  source: .andromeda/security-plan.md:390 «It does not restrict the contract payloads that carry content by design: `events.ndjson`, `wait`/`last` results (CLI and MCP), `hook.dialog`, and `statusline_command` in `snapshot.json`»
  replace-with: «Only in contract payloads (`events.ndjson`, `wait`/`last`, `hook.dialog`) and `instances/<name>/diagnostics/detail-*`; never in stderr, `--json` errors, MCP errors, channel `error.data`»

- leaf: .claude/docs/security-summary.md:30   class: stale
  leaf-text: «pids, `agent_session_id`, budget percentages, `bind`, OS username in paths | Typed fields in diagnostics; paths never in external errors (only the cookie-gated `/api/info` `viola_home`)»
  source: .andromeda/security-plan.md:525 «- NEVER expose absolute paths in any external error.»
  replace-with: «pids, `agent_session_id`, budget percentages, OS username in paths | Typed fields in diagnostics; paths never in external errors»

- leaf: .claude/docs/security-summary.md:36   class: retired
  leaf-text: «- NEVER assume loopback TCP is same-user: the cookie check is mandatory on `/api/*` and SSE.»
  source: .andromeda/security-plan.md:541 «- NEVER add a listener (TCP port, WebSocket, `http` hooks, MCP HTTP transport) or a new channel method without a Security Decisions Log entry mapping it to this plan's IPC controls.»
  replace-with: «»

- leaf: .claude/docs/security-summary.md:44   class: stale
  leaf-text: «1. v1 GUI cookie on `/api/*` + SSE. 2. Unix endpoint in a per-user 0700 dir. 3. `not-delivered`/`control-character` + URNs `unauthorized` / `cross-origin-forbidden`. 4. `release` with `from` → `-32602 release-from-driver`. 5. `<home>/ui/<port>.url` (0600). 6. `MAX_FRAME` = 16 MiB in `viola-core`. 7. Ledger row "largest hook payload seen". 8. `<hash>` = truncated SHA-256 (16 hex). Amendments 1, 5 and amendment 3's two URNs were folded into architecture at the 2026-10-01 0-pending wrap; amendment 3's `control-character` detail lands with the route's "Confirmed send" entry.»
  source: .andromeda/security-plan.md:366 «  - Amendments 3 (`control-character` detail) and 6 (`MAX_FRAME`): before the `viola-core` chunk. Its `control-character` detail lands with the route's "Confirmed send" entry.»
  replace-with: «2. Unix endpoint in a per-user 0700 dir. 3. `not-delivered`/`control-character`. 4. `release` with `from` → `-32602 release-from-driver`. 6. `MAX_FRAME` = 16 MiB in `viola-core`. 7. Ledger row "largest hook payload seen". 8. `<hash>` = truncated SHA-256 (16 hex). Amendment 3's `control-character` detail lands with the route's "Confirmed send" entry.»

- leaf: .claude/docs/security-summary.md:66   class: retired
  leaf-text: «- GUI cookie is per-port (`viola_<port>`) with no `Max-Age`; cross-port cookie exposure accepted as residual.»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/rules/security.md:6   class: stale
  leaf-text: «Secrets in v1: the GUI per-launch token (32 `getrandom` bytes), the launch URL, the `viola_<port>` cookie, inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value. None may reach a log, diagnostic, event, snapshot, fixture or error body. The one exception: the launch line `viola ui` prints once to stderr and writes to the 0600 `ui/<port>.url`.»
  source: .andromeda/security-plan.md:389 «  - Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.»
  replace-with: «Secrets in v1: inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value. None may reach a log, diagnostic, event, snapshot, fixture or error body.»

- leaf: .claude/rules/security.md:7   class: retired
  leaf-text: «- HTTP logging records `uri.path()` only (never the `?t=` query) and never the `Cookie` header.»
  source: .andromeda/security-plan.md:389 «  - Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.»
  replace-with: «»

- leaf: .claude/rules/security.md:9   class: retired
  leaf-text: «- Compare the token/cookie only with `constant_time_eq`, never `==`; issue the cookie `HttpOnly; SameSite=Strict; Path=/`, 303-redirect away from `?t=` at once, send `Referrer-Policy: no-referrer`.»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/rules/security.md:27   class: retired
  leaf-text: «- `Last-Event-ID`: parse every pair with `ViolaName::try_new` + `u64`; any bad pair drops the whole header.»
  source: .andromeda/security-plan.md:221 «**Parser surfaces (scope for fuzz / property coverage; tests owns the cases):** `validate_paste_text`, channel ndjson framing with `MAX_FRAME`, the hook stdin parser (including `hook statusline`),»
  replace-with: «»

- leaf: .claude/rules/security.md:43   class: stale
  leaf-text: «; Problem Details `detail` strings are fixed and never name a path.»
  source: .andromeda/security-plan.md:454 «- The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages.»
  replace-with: «.»
