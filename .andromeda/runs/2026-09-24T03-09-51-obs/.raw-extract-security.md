## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal, stated as "Minimal (0), with targeted elevations for the local privilege boundary". The elevations cover four things: IPC endpoint access control and server impersonation, GUI cross-user and cross-origin readability, GUI output encoding, and the integrity of `~/.viola/`.
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data […] which rules out Standard's user-account and HTTPS concerns and Hardened's compliance drivers."
- **Compliance triggers:** None. There is no payment, health, children's or third-party personal data. All logged content is the operator's own Claude session content, kept on their own machine.
- **Obs-relevant framing from the plan:**
  - "Logging format and fields belong to obs."
  - Obs owns the logger and the format. Security skips its own Logging section at Minimal tier, and its only contribution is a consolidated NEVER-log floor (the `logging-redaction-wire` bootstrap phase).
  - `diagnostics/` file format is owned by obs.
  - Error reporting integration: none. viola makes no outbound network calls. "If obs ever adds an external reporter, the Logging bans below apply before transmission."

### Logging-Sensitive Vectors
- **Vector 1:** Local IPC wrapper channel (JSON-RPC 2.0 over ndjson; named pipe `\\.\pipe\viola-<h12>` or Unix socket; methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`). Logging implications:
  - `hook.event` notifications (UserPromptSubmit, Stop, PostToolUse), `wait`/`last` results and `hook.dialog` carry prompt text, `last_assistant_message` and tool data. That content may only go into the contract payloads. It never goes into log or error lines or into channel `error.data`.
  - Channel `-32603` uses the fixed message "internal error" with `data: null`.
  - The `from` parameter is self-reported. It must not be treated as an authenticated identity.
- **Vector 2:** CLI `send` text written into the PTY (from stdin, `--file` or MCP `send.text`). Logging implications:
  - Sent text is user content. It never goes to stderr, CLI `--json` error output or MCP `isError` results. Only `instances/<name>/diagnostics/` may hold it.
  - A refusal is reported as the code `not-delivered` with detail `control-character`, never by echoing the text.
- **Vector 3:** Loopback HTTP GUI (`viola ui` on `127.0.0.1:47319`; SSE `/api/events` streams raw event lines including prompts, assistant output and permission `input`). Logging implications:
  - The per-launch token travels in `GET /?t=<token>` until the 303 redirect. Any HTTP request or span logging in `viola-ui` must record the path without its query string.
  - The `Cookie` header (`viola_<port>`) must never be logged.
  - Problem Details use fixed `detail` strings. `urn:viola:problem:unauthorized` never contains the token, the launch URL, or the `.url` path or contents. `urn:viola:problem:state-unreadable` must not name the unreadable path.
  - The only intended path disclosure is `viola_home` in `/api/info`, which is behind the cookie.
- **Vector 4:** Hook stdin payloads from Claude Code (`viola hook <event>`, plus `hook statusline`, which shells out `statusline_command`). Logging implications:
  - `hook` never writes to stderr and never exits non-zero, even when a security check fails (server verification mismatch, oversized stdin, strict-modes failure). It fails open with exit 0 and no body, and writes a note only to `diagnostics/`.
  - serde_path_to_error drift reports can quote tool `input` and go only to `diagnostics/`.
- **Vector 5:** MCP over stdio (tools `send`, `wait`, `last`, `answer`, `list`; the driver is an LLM). Logging implication: MCP `isError` results carry only the `structuredContent` codes (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault`). They never carry upstream text, absolute paths, internal type names or drift-report content.
- **Vector 6:** CLI arguments, flags and environment. Logging implications:
  - `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every other R8-stripped `CLAUDE*` value must never appear in any log, stderr, diagnostic, snapshot, `events.ndjson` or fixture.
  - An anyhow chain printed to human-mode stderr must never still contain a serde_json or serde_path_to_error source, because their `Display` quotes input values. Map it to a fixed message before it joins the chain, and write the full error only to `diagnostics/` when an instance is resolved.
- **Vector 7:** Filesystem state under `~/.viola/`. Logging implications:
  - `events.ndjson` and `instances/<name>/diagnostics/` files are 0600 inside 0700 directories. On Windows they are covered by the home DACL read rule (user, SYSTEM and Administrators only).
  - `events.ndjson` is never rotated or truncated today, and its retention is an open arch item. Any rotation scheme must keep 0600/0700 permissions and keep `Last-Event-ID` offsets valid.
  - Unbounded growth, which can exhaust disk, is an accepted availability risk.
- **Vector 8:** Child process spawning (`claude agents --json`, the `--version` probe). Logging implication: output rows are untrusted, so human-mode `list` output (every field, including unwrapped session names) and human-mode `wait`/`last` output must escape C0/C1 controls (keeping `\n` and `\t`) before printing to a terminal.
- **Vector 9:** Supply chain. Logging implication: if obs selects tracing-subscriber, the version must be `>=0.3.20` (RUSTSEC-2025-0055, terminal escape injection).

### Anti-Patterns Rejected
The plan's `## Security Anti-Patterns (NEVER do these)` section is its rejection list. The items below are the ones with a telemetry, logging, diagnostics or error-output bearing. Bans limited to IPC ACLs, input parsing, CSP/CORS wiring, supply-chain CI and PTY spawning are not included.

- **Logging CLAUDE session credentials:** writing `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log, stderr, `events.ndjson`, snapshot, `diagnostics/` or fixture — rejected because these are inherited credentials held only in the wrapper's memory (consolidated NEVER-log floor).
- **Printing or logging the GUI token, the `Cookie` header, or the query string of a GUI request URI** — rejected because `/?t=<token>` carries the credential. The only exceptions are the one-time `viola ui` launch line on stderr and the 0600 `ui/<port>.url` file. No other stderr line may carry the token.
- **Echoing serde_path_to_error drift reports or anyhow chains into Problem Details `detail`, MCP `structuredContent` or channel `error.data`** — rejected because they leak upstream content and internal paths.
- **Printing an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source** — rejected because its `Display` quotes input values such as tool `input`.
- **Writing to stderr from `viola hook`, or exiting non-zero from it (including on security-check failures)** — rejected because of the Hook Contract: hooks must fail open to the human. Diagnostics go to `diagnostics/` only.
- **Printing `wait`/`last` human-mode output, or `list` rows from `claude agents --json`, to a terminal without escaping C0/C1 controls** — rejected because of terminal escape injection (the same class as RUSTSEC-2025-0055).
- **Using tracing-subscriber `<0.3.20`** — rejected because of RUSTSEC-2025-0055.
- **Creating `diagnostics/` files readable by other users** — rejected because diagnostics hold user content, tool `input` and drift reports. They must be 0600, or protected by the Windows home DACL.
- **Exposing absolute paths in any external error** (the only exception is `/api/info`'s cookie-gated `viola_home`) — rejected because paths carry the OS username, the only PII in scope.
- **Putting user content, tool `input`, the `statusline_command` string or drift reports (paths and values) into stderr log or error lines, CLI `--json` error output, MCP `isError`, Problem Details or channel `error.data`** — rejected because these may go only to `instances/<name>/diagnostics/`, apart from the by-design contract payloads.
- **Internal type names in external error output** — rejected. `thiserror` `Display` impls on `PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `UiError` and `CoreError` use fixed messages and leave out any fields holding paths or payloads.
- **Adding a listener (TCP port, WebSocket, `http` hooks, MCP HTTP transport) without a Security Decisions Log entry mapping it to the plan's IPC/GUI controls** — rejected because loopback TCP is reachable by every local user. This applies to any metrics or telemetry endpoint too.
- **Binding anything to `0.0.0.0` or `::`** — rejected. Plain HTTP is acceptable only on loopback.
- **Making an access decision based on the client address or `X-Forwarded-For` / `Forwarded`** — rejected because there is no proxy. Access rests on the Host allowlist and the cookie.
- **Committing `fixtures/claude/*` recorded from real (non-synthetic) prompts, or without first checking them for home paths and usernames** — rejected because fixtures are committed to git.
- **Committing `.env` files, local `--home` test directories or signing material** — rejected because recorded state must never land in git.
- **Letting `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any security control** — rejected because environment variables are not a configuration channel, and a same-session agent can influence `VIOLA_DIR`. This also rules out a debug or verbose switch that disables redaction.
- **Letting a security refusal block the human** — rejected. Refusals go only to automation (`send`/`answer`), and hooks fail open.
- **External error reporter sending data before the Logging bans are applied** — rejected. v1 has no outbound network calls, so any future reporter must apply the NEVER-log floor before transmission.

### Data Classifications
The plan does not assign Critical/High/Medium/Low labels, except "low sensitivity" for operational metadata. Where sensitivity is marked "(sensitivity inferred)", the level was derived from the plan's description. The obs handling below is stated explicitly by the plan unless marked "(handling inferred from sensitivity)".

- **GUI per-launch token, launch URL, `ui/<port>.url` contents, `viola_<port>` cookie value / `Cookie` header** (Critical, sensitivity inferred) — obs handling: never-log. Also strip the query string from any HTTP request path that is logged. Sole exceptions: the one-time stderr launch line and the 0600 `.url` file. Appears in: `viola-ui` process memory, `<viola home>/ui/<port>.url`, `GET /?t=<token>` exchange, browser cookie.
- **Inherited credentials: `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` variables** (Critical, sensitivity inferred; transient, held in process memory only) — obs handling: never-log anywhere, including `diagnostics/`, snapshots, `events.ndjson` and fixtures. Appears in: the `viola run` wrapper process environment (stripped from the child).
- **v1.x signing credentials (Azure Artifact Signing, zipsign private key)** (Critical, sensitivity inferred) — obs handling: never-log; held only as CI secrets in the v1.x release workflow. Appears in: the v1.x release workflow (does not exist in v1).
- **User content: prompt text and assistant output** (`prompt-submitted.data.text`, `turn-ended.data.last_assistant_message`) (High, sensitivity inferred; persistent, never rotated) — obs handling: never-log in log or error lines (stderr, CLI `--json` errors, MCP `isError`, Problem Details, channel `error.data`). Permitted only in the by-design contract payloads and in `instances/<name>/diagnostics/` (0600). Appears in: `instances/<name>/events.ndjson`, SSE `/api/events`, MCP/CLI `wait`/`last`, channel `hook.event`, wrapper memory.
- **User content: tool-call arguments (`permission.input`, including shell commands and file bodies), plan text, dialog questions and answers** (High, sensitivity inferred; may incidentally contain secrets or personal data) — obs handling: same as prompt and assistant content (never-log outside contract payloads; `diagnostics/` only). Appears in: `events.ndjson`, SSE, channel `hook.dialog`, `answer` free text.
- **serde_path_to_error drift reports (paths and values; can quote tool `input`)** (High, sensitivity inferred) — obs handling: `diagnostics/` only. Never in stderr, `--json`, MCP, Problem Details or `error.data`. Appears in: `viola-agent-claude` hook parser, tolerant parsers.
- **`statusline_command` (user-written shell string; the only shell-out)** (High for integrity, sensitivity inferred) — obs handling: never in log or error lines; `diagnostics/` only. Appears in: `snapshot.json`, `instances/<name>/settings.json`, `hook statusline`.
- **Config values: `config.json` budget thresholds and GUI port; snapshot `endpoint` / `pinned_bin`; `ledger/stamps.json` verified CLI versions; `hooks.json` / `.mcp.json` exec paths** (Low confidentiality, high integrity; sensitivity inferred) — obs handling: OK to log structured, except that values containing absolute paths follow the operational-metadata rule below (handling inferred from sensitivity). Appears in: `~/.viola/` state files.
- **Operational metadata: absolute paths carrying the OS username (the only PII in scope), internal type names** (Medium, sensitivity inferred) — obs handling: scrub-required in external output (CLI `--json` errors, MCP `isError`, Problem Details, channel `error.data`). Paths are permitted in `diagnostics/`. The only intended external path disclosure is the cookie-gated `/api/info` `viola_home`. Appears in: error paths across all crates, `/api/info`.
- **Operational metadata: `pid`, `child_pid`, `agent_session_id`, `started_at`, `budget.json` usage percentages and `resets_at`, `verified_cli_versions`, `bind`** (Low; "low sensitivity" per the plan) — obs handling: OK to log structured, and explicitly permitted in `diagnostics/`. Appears in: snapshots, `budget.json`, `/api/info`, `/api/sessions`.
- **Unwrapped session identifiers / rows from `claude agents --json`** (Low but untrusted, sensitivity inferred) — obs handling: OK to log structured, but C0/C1 controls must be escaped in any human-mode terminal output. Appears in: CLI `list`, the MCP `list` tool, `/api/sessions`.
- **Protocol codes and refusal details** (`-32600`, `-32602`, `release-from-driver`, `not-delivered` / `control-character`, `instance-unreachable`, `wrapper-fault`, Problem Details URNs) (Low, sensitivity inferred) — obs handling: OK to log structured (handling inferred from sensitivity). Appears in: channel, MCP, CLI, `viola-ui`.
- **Recorded fixtures (`fixtures/claude/<cli-version>/`)** (Medium, sensitivity inferred) — obs handling: scrub-required. Probe prompts must be synthetic, and home paths, usernames and tool `input` must be reviewed before committing. Appears in: the repository, recorded by `viola verify`.
- **Payment, health or multi-user PII:** none. There is no database, no accounts and no payment, identity or health SDK.
