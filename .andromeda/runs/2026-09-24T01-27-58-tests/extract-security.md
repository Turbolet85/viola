## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal (0), with targeted elevations for the local privilege boundary
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data." The elevations are: IPC endpoint access control and server impersonation, paste breakout in `send.text`, GUI output encoding, GUI readability by other users and origins, v1.x brake auth, and `~/.viola/` integrity.
- **Context:** No compliance triggers. Local-only install. CI runs on windows-2025, macos-latest and ubuntu-latest, with tests against the fake agent and recorded fixtures. The real `claude` CLI and `viola verify` run only locally. macOS is CI-only in v1.

### Attack Vectors
- **Vector 1:** Local IPC channel (JSON-RPC over ndjson: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`). This is the highest-impact surface. Scope: `viola-channel` server/client and `viola` bin dispatch.
  Mitigation: Windows pipe DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, remote clients rejected. A squatted name makes `run` exit 1. Unix sockets live in a verified 0700 per-user dir, with `mode(0o600)` secondary. The server drops peers whose euid differs before reading any frame. Before writing any frame, the client checks the server's pid (and euid on Unix) plus start time against `snapshot.json`; macOS checks euid and the dir only. On mismatch, CLI/MCP report `instance-unreachable` (exit 21) and `hook` exits 0 with no body. `release` carrying `from` gets `-32602` `release-from-driver` (CLI exit 20). A frame over `MAX_FRAME` (16 MiB) gets `-32600` and the connection closes.
- **Vector 2:** `send` text and `answer` free text written into the PTY as a bracketed paste. Scope: `viola_core::validate_paste_text`, the `run` paste writer and the `answer` handler.
  Mitigation: LF, CR and TAB are allowed. Every other C0 control (ESC above all), DEL and C1 (U+0080–U+009F) is refused. The check runs on decoded `char`s, not bytes. Text is rejected, never stripped, as `not-delivered`/`control-character`, checked before the other refusal reasons. It runs on the client and again in the wrapper, which is authoritative.
- **Vector 3:** Loopback GUI (`127.0.0.1:47319`). Routes: `/`, `/assets/*`, `/health`, `/ready`, `/api/info`, `/api/sessions`, `/api/links`, SSE `/api/events`; v1.x adds `POST .../pause` and `.../unlink`. Scope: `viola-ui`.
  Mitigation:
  - **Host allowlist:** outermost layer, every route. Host must be exactly `127.0.0.1:<port>` or `localhost:<port>`, else 403 `host-not-allowed`. Missing, wrong-case, trailing-dot or wrong-port all fail.
  - **Token exchange:** `GET /?t=<token>` (32 getrandom bytes, constant-time compare). A match sets `viola_<port>` (HttpOnly, SameSite=Strict, Path=/, no Max-Age) and returns 303 to `/`. A mismatch returns 401 `unauthorized` with no cookie.
  - **Cookie gate:** `/api/*` and SSE need the cookie, else 401. `/`, `/assets/*`, `/health` and `/ready` are ungated.
  - **Methods and CORS:** any method other than GET gets 405. No CORS headers are sent.
  - **Headers on every response:** CSP `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`, plus `nosniff`, `no-referrer` and CORP `same-origin`. `no-store` on `/api/*` and on the exchange response, including its 401.
  - **`Last-Event-ID`:** each pair parses as `ViolaName` and `u64`. Any bad pair drops the whole header, and the stream starts from each file's current end.
  - **Assets and SSE:** only `/assets/*` is compressed, never SSE. Assets are embedded, never served through `ServeDir`. Event text is rendered as text only.
  - **v1.x POSTs:** cookie plus `Sec-Fetch-Site` of `same-origin` or `none`. Without that header, `Origin` must exactly match the allowed host, else 403 `cross-origin-forbidden`. A non-empty body gets 400. A bad `{name}` gets 404.
  - **Token handling:** no route re-issues the token.
- **Vector 4:** Hook stdin from Claude Code, covering all hook events and `hook statusline`. Scope: the `viola-agent-claude` parser and `viola hook`.
  Mitigation: Stdin is capped at `MAX_FRAME`; oversize fails open (exit 0, no body, a diagnostics note). Parsing is tolerant, but dialog kind and decision map to closed enums. A bad `resets_at` becomes `"unknown"`. `hook` never writes stderr or exits non-zero, even when a security check fails. `statusline_command` runs only after the strict-modes check passes. A non-null dialog decision needs a `viola verify` stamp.
- **Vector 5:** MCP over stdio (`send`, `wait`, `last`, `answer`, `list`). The driver is an LLM. Scope: `viola-mcp`.
  Mitigation: schemars schemas are advisory, since rmcp does not enforce them. Handlers parse `target` as `ViolaName`, `dialog_id` as `u64` and `response` into closed enums, and apply `validate_paste_text` to `text` and free-text answers. rmcp features are only `server` and `transport-io`. `isError` results carry only codes.
- **Vector 6:** CLI args, flags, env and stdin (`--home`, `VIOLA_DIR`, `answer` stdin/`--file`, `run <name> -- <program>`). Scope: clap in `src/main.rs` and `viola-state`.
  Mitigation: Names go through `parse_viola_name`. `--home` and `VIOLA_DIR` are canonicalized and must pass strict-modes before any snapshot or ledger read. `answer` JSON is capped at `MAX_FRAME` and parsed into closed enums. `config.json` is parsed tolerantly, with thresholds as `Percent` (0–100), port as `u16` and a `v` check. No config value, env var or flag can disable a control.
- **Vector 7:** Filesystem state under `~/.viola/`. Scope: `viola-state`.
  Mitigation:
  - **Unix modes:** dirs 0700, files 0600, pinned exe 0700. The mode is set explicitly on the temp file, and the write is discarded if it can't be set.
  - **Unix strict-modes:** refuse if a trusted dir or file is group/world-writable or not owned by euid. Also refuse if the home, `instances/<name>/` or `ui/` has `mode & 0o077 != 0`.
  - **Windows strict-modes:** refuse on:
    - an unreadable descriptor, a NULL DACL, or a volume without persistent ACLs
    - an owner other than the user or Administrators
    - any allow ACE, inherit-only included, granting write, delete, WRITE_DAC or WRITE_OWNER to a SID other than the user, SYSTEM or Administrators (`CREATOR OWNER` counts as the user)
    - a read grant to such a SID on the home, `instances/<name>/`, `diagnostics/`, `ui/` or an existing `events.ndjson` (generic bits tested)
  - **Where it runs:** `run`, `ui`, `mcp`, every `hook`, and CLI verbs that resolve an endpoint.
  - **On failure:** exit 1 for `run`, `ui` and `mcp`; exit 0 with no output for `hook`; exit 21 for CLI verbs.
  - **`--home` outside `%USERPROFILE%`:** gets a protected user + SYSTEM DACL, or is refused.
  - **Pinned exe:** re-hashed with SHA-256 (first 16 hex chars) against `<hash>`; a mismatch exits 1.
  - **Rewritten every start:** `plugin/` files and `settings.json`.
  - **ndjson reads:** lines are capped at `MAX_FRAME`; an over-long line counts as `torn_lines`. Tailing ignores symlinks and invalid names.
- **Vector 8:** Child spawning and PATH resolution (`claude agents --json`, the `run` child and its `--version` probe). Scope: `viola-agent-claude` and `viola-pty`.
  Mitigation: Output is capped at `MAX_FRAME`; a parse failure becomes `unknown`. A row `name` is never a `ViolaName` or `target`. Human-mode `list`, `wait` and `last` escape C0/C1 (keeping `\n` and `\t`). On Windows the npm shim resolves to `claude.exe`, and a `.cmd`/`.bat` PTY child makes `run` exit 1. Hook, MCP and statusline commands use the absolute pinned path. Handles are never inherited. vt100 runs under `catch_unwind`; a panic gives `not-delivered`/`input-not-ready` and passthrough continues.
- **Vector 9:** Supply chain (Cargo deps, GitHub Actions, v1.x release). Scope: CI and `deny.toml`.
  Mitigation: `cargo deny check` runs in CI, plus a weekly advisories run. zizmor checks the workflows. Actions are SHA-pinned, with `permissions: {}` and `contents: read`. `Cargo.lock` is committed. The only advisory ignore is RUSTSEC-2017-0008.
- **Parser surfaces for fuzz/property coverage ("tests owns the cases"):** `validate_paste_text`, the `Last-Event-ID` parser, channel ndjson framing with `MAX_FRAME`, the hook stdin parser (incl. `hook statusline`), the `claude agents --json` parser, and the vt100 feed under `catch_unwind`.
- **Error-sanitization expectations:**
  - External errors (CLI `--json`, MCP `isError`, Problem Details, channel `error.data`) contain no absolute paths, upstream text, tool `input`, serde paths or values, internal type names, anyhow chains, or token/URL/`.url` contents.
  - `-32603` has the fixed message "internal error" and `data: null`.
  - `state-unreadable` never names the path.
  - The only path disclosure is the cookie-gated `/api/info` `viola_home`.
  - Logs never contain the token, the `Cookie` header, the `?t=` query string, or `CLAUDE_CODE_MESSAGING_*` and other stripped `CLAUDE*` values.

### Anti-Patterns Rejected
**IPC and authentication**
- **Default interprocess Windows DACL**: grants read to Everyone and anonymous.
- **Logon SID in the pipe SDDL**: other logon sessions of the same user must still connect.
- **Socket in `/tmp` or at the `$TMPDIR` root**: another user can squat it (Kea CVEs).
- **`mode(0o600)` as the only Unix control**: unsupported on macOS and may be ignored.
- **Linux abstract-namespace sockets**: no filesystem permissions.
- **Writing any frame before server verification, or taking reference values from a snapshot that failed strict-modes**: impersonation, attacker-written references.
- **Default Windows client connect without SQOS, even as a fallback**: client impersonation.
- **Treating channel `from` as identity**: it is self-reported.
- **Per-instance 0600 proof file for `release`**: a same-user agent can read it.
- **Comparing token or cookie with `==`**: must be constant-time.
- **Cookie without HttpOnly and SameSite=Strict, or token left in the URL**: token exposure.

**Input**
- **Trusting schemars schemas or client checks alone**: the wrapper must re-validate.
- **`instances/<name>` joined from a non-`ViolaName` string**: path traversal.
- **Skipping bad `Last-Event-ID` pairs**: must drop the whole header.
- **Stripping controls silently, or refusing LF, CR or TAB**: breaks exact-match delivery confirmation; multi-line text is normal.
- **Control check over raw bytes**: UTF-8 continuation bytes look like C1.
- **`read_line` or `read_to_end` without `Read::take(MAX_FRAME)`**: unbounded memory.
- **serde_json `unbounded_depth`**: keep the 128 limit.
- **Free `String` for `behavior`, `dialog_id`, wheel holder or dialog kind**: must be closed enums or integers.
- **Using a `claude agents --json` row `name` as a `ViolaName` or `target`**: untrusted child output.
- **garde or validator crates**: arch chose nutype.

**Data protection and filesystem**
- **Default umask, or a group/world-writable pinned exe**: must be 0700 dirs and 0600 files.
- **Skipping the Windows owner+DACL check, or passing a NULL DACL, a non-persistent-ACL volume or an inherit-only GENERIC grant**: other users could read or write.
- **Running `statusline_command` when the home fails strict-modes**: code execution.
- **Reusing the pinned exe without a SHA-256 re-hash, a non-crypto `<hash>` (FNV, `DefaultHasher`), or reusing `plugin/` files unrewritten**: substitution goes undetected.
- **`.cmd`/`.bat` PTY child via portable-pty**: bypasses BatBadBut escaping.
- **Inheritable channel handles, or a PTY replacement with `bInheritHandles = TRUE`**: handle leak to the child.
- **Binding to `0.0.0.0` or `::`**: plain HTTP is loopback-only.
- **Token not from getrandom**: predictable.
- **Fixtures from real prompts, or unchecked for paths and usernames**: PII leak.

**API and GUI**
- **`CorsLayer` or `Access-Control-Allow-*`**: would enable cross-origin reads.
- **axum-extra `Host` extractor, or axum `http2`**: can lose the port.
- **Host allowlist on `/api/*` only**: DNS rebinding on other routes.
- **`/api/*` or SSE without the cookie, or a state-changing route without cookie + `Sec-Fetch-Site`/`Origin`**: reachable by other local users.
- **Compressing SSE, or excluding it by content-type predicate only**: breaks SSE, 406s.
- **`ServeDir` or any filesystem asset handler**: path traversal.
- **Access decisions on client address or `X-Forwarded-For`/`Forwarded`**: there is no proxy.
- **`innerHTML`, `v-html`, `dangerouslySetInnerHTML`, Markdown-to-HTML, inline scripts or handlers, `eval`**: XSS, CSP and Trusted Types violations.
- **rmcp `transport-streamable-http-server` or `auth` features**: all 2026 advisories sit there.
- **`tower-csrf` or `axum_csrf`**: rejected in research; the Origin check is hand-written.
- **Rate limiting in v1**: not needed at this tier (256-bit token); exhaustion is an accepted risk.
- **Missing size limits (`MAX_FRAME`, POST body rejection)**: exhaustion.

**Secrets and logging**
- **Logging `CLAUDE_CODE_MESSAGING_*` or other stripped `CLAUDE*` values, the token outside the launch line and `.url` file, the `Cookie` header or the `?t=` query**: secret leak.
- **Credential in a URL past the exchange, or one token shared across launches**: Referer leak, no rotation.
- **Committing `.env`, `--home` test dirs or signing material; hardcoded credentials in source or plugin files**: secret leak.
- **Drift reports or anyhow chains in external errors, or a serde-sourced chain on stderr**: quotes input values.
- **stderr output or a non-zero exit from `viola hook`**: hooks must fail open.
- **Unescaped C0/C1 in `wait`, `last` or `list` human output**: terminal injection.
- **tracing-subscriber `<0.3.20`**: RUSTSEC-2025-0055.
- **Readable `diagnostics/`, or absolute paths in external errors**: content and PII leak.

**Code patterns and universal**
- **Upstream text treated as a command or config, or a second shell-out**: R1.
- **Screen signatures, harness prefixes or local-command lists built from runtime text**: must come only from compiled ledger rows.
- **Cookie treated as anything but an opaque random value**: it carries no claims.
- **interprocess `try_overwrite`**: TOCTOU deletion.
- **vt100 outside `catch_unwind`**: a panic kills the terminal path.
- **Actions by mutable ref**: supply-chain drift.
- **rust-cache in the release workflow; `self_update` without `signatures`**: cache poisoning, unsigned updates.
- **New listener or channel method without a Decisions Log entry**: unmapped surface.
- **A security refusal blocking the human**: refusals are for automation only.
- **Hook, MCP or statusline commands resolving `viola` via PATH**: code execution by whatever is first on PATH.
- **Non-null `hook.dialog` without a verify stamp, or a non-`verify` process writing `stamps.json`**: the stamp gates `allow`.
- **Any process other than the wrapper writing `snapshot.json`**: it controls endpoint and exec.
- **Config, `VIOLA_*` env or a flag disabling any control**: env is not a config channel.

### Data Classifications
- **Prompts and assistant output** (high): in `instances/<name>/events.ndjson`, wrapper memory, SSE, and `wait`/`last`. Never rotated; retention is open (must keep 0600 and valid offsets). Testability hint: testable (temp `--home` + fake agent).
- **Tool args, plans, dialog Q&A** (high; may hold secrets): `permission {tool, input}`, `plan` and `question` in `events.ndjson`, SSE and `hook.dialog`. Drift reports go only to `diagnostics/` (0600). Testability hint: testable (recorded hook fixtures).
- **Config and code-bearing state** (integrity-high): `config.json`, `settings.json`, `snapshot.json` (`statusline_command`, `endpoint`, `pinned_bin`, `pid`, `started_at`), `ledger/stamps.json`, plugin `hooks.json`/`.mcp.json`, and the pinned exe. Module: `viola-state`, `run`, `verify`. Testability hint: testable for Unix modes, hash mismatch and rewrite; partially testable with stubs for the Windows DACL cases (per-OS CI matrix).
- **GUI per-launch token** (high, secret): process memory, `ui/<port>.url` (0600, removed on graceful shutdown) and the `viola_<port>` cookie. Module: `viola-ui`. Testability hint: testable.
- **Inherited credentials** (high, transient): `CLAUDE_CODE_MESSAGING_TOKEN`/`_SOCKET` and other `CLAUDE*` values, held only in the wrapper env and stripped from the child. Testability hint: partially testable with a stub child.
- **Operational metadata** (low; PII is the OS username in paths): `/api/info` (cookie-gated), snapshot pids and ids, `budget.json`, `/api/sessions`. Testability hint: testable.
- **Payment, health or multi-user PII**: none (no database, no accounts).
