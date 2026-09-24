---
paths:
  - "crates/viola-channel/**"
  - "crates/viola-ui/src/**"
  - "crates/viola-mcp/**"
  - "src/cmd/**"
---

# API Rules

Path-scoped rules for viola's interfaces: the wrapper channel, the MCP tools, the CLI verbs and the loopback GUI HTTP. Authoritative sources: `.andromeda/architecture.md` §Conventions + §Standard Contracts; `.andromeda/security-plan.md` §API Security + §Error Handling.

## Five interfaces, one binary
- CLI verbs · MCP tools (stdio JSON-RPC 2.0) · the hook stdin/stdout contract (owned by Claude Code) · the wrapper channel (JSON-RPC 2.0 over ndjson on a local socket) · GUI HTTP GET + SSE. No REST write surface, GraphQL, gRPC, tRPC or WebSocket.
- A new listener or channel method needs a Security Decisions Log entry first.

## Versioning
- Every channel `params`, event line, snapshot and GUI JSON body carries integer `v` (starts at 1); every `params` also carries `sender` (`CARGO_PKG_VERSION`). Adding a field/kind is additive; removing or re-meaning bumps `v`. A newer `params.v` → `-32602` with `data:{supported, wrapper}`.

## Wrapper channel
- Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog` (requests, integer ids monotonic per connection); `hook.event` (notification). `list` is not a channel method — it reads the disk + `claude agents --json` caller-side.
- Success `{"result":{"ok":<payload>}}`; refusal `{"result":{"refusal":"<reason>","detail":<string|null>}}` (a normal outcome); protocol fault `{"error":{code,message,data}}` with `-32700/-32600/-32601/-32602/-32603` — `-32603` is exactly `"internal error"`, `data: null`.
- `RefusalReason` (kebab-case): `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown` (`#[serde(other)]`). Details are closed per reason; `control-character` precedes the documented order (`send`: human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted; `answer`: human-typing → unverified-cli → unknown-dialog). `unconfirmable` only ever appears inside an `ok` payload.
- Frames: `Read::take(MAX_FRAME)` before `read_line` (over → `-32600` + close); `behavior`, `dialog_id` (u64), wheel holder and dialog kind are closed types; `link`/`unlink` names through `ViolaName::try_new`; optional additive `conn`.
- `release` carrying `from` → `-32602` with `data:{"reason":"release-from-driver"}`.

## CLI
- Human text by default, `--json` for agents: one JSON document on stdout mirroring the channel result (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":"instance-unreachable"|"wrapper-fault",…}`).
- Exit codes: 0 ok · 1 internal / `run` start refusal · 2 usage (never from `viola hook`) · 10 human-typing · 11 budget-paused · 12 unverified-cli · 13 not-delivered · 14 unknown · 20 wrapper fault · 21 instance unreachable. `viola hook` always exits 0.
- Prompt text only from stdin or `--file`; warn on a Git Bash rewritten-path argument; `ViolaName` arguments via `value_parser = parse_viola_name`; `--home`/`VIOLA_DIR` canonicalised and strict-modes-checked before any read.
- The CLI adds `from` from its own `VIOLA_NAME` when set.

## MCP
- Tools exactly `send · wait · last · answer · list`; `release`, `pause`, `link`, `unlink` stay CLI-only. Schemas are advisory — handlers re-deserialise into `ViolaName`, `u64`, closed enums and re-run `validate_paste_text`.
- Success: `isError:false` + the channel `ok` payload as `structuredContent`; refusal/unreachable/fault: `isError:true` + codes only. rmcp features exactly `server` + `transport-io`; every call returns below the client's tool-call timeout.

## GUI HTTP (`viola ui`)
- Bind `127.0.0.1` only. Outermost middleware: raw `Host` header exactly `127.0.0.1:<port>` or `localhost:<port>` → else 403 `urn:viola:problem:host-not-allowed` (never axum-extra's `Host` extractor). GET only; anything else 405.
- `/api/*` and SSE need the `viola_<port>` cookie (constant-time compare) → else 401 `urn:viola:problem:unauthorized`; `/`, `/assets/*`, `/health`, `/ready` ungated.
- No CORS headers ever; CSP + `nosniff` + `no-referrer` + CORP `same-origin` on every response; `Cache-Control: no-store` on `/api/*` and the `?t=` exchange. `CompressionLayer` on `/assets/*` only — SSE never compressed. Assets from `include_bytes!`, never `ServeDir`.
- Errors are RFC 9457 `application/problem+json` with `v` and fixed `detail`; `/ready` 503 uses its own body, never Problem Details. Lists are returned whole with `skipped{unknown_kinds,unknown_fields,torn_lines}`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
