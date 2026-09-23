# Quiz II — Technical Layer Results

Provenance: every answer in this run (Phase 0 through Quiz II) was typed by the founder's overseer session
driving this arch session through the viola prototype, under the founder's delegation — recorded as
"overseer (founder-delegated)". The founder's one direct ruling is the Quiz I Scale Intent answer
(personal; D3 holds; public distribution is a later version), relayed by the overseer.

Naming convention (overseer, founder-delegated): the product and binary are `viola`; the instance environment
variables are `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (as the prototype used). `BRIDGE_NAME` was the brief's
working name; every bridge-prefixed identifier becomes viola-prefixed (e.g. the `ViolaName` newtype). Earlier
run artifacts kept verbatim (research-targeted.md, template-technical.md) still show `BRIDGE_NAME`/`BridgeName`
and are superseded by this convention.

## Stack Decisions
- Language / Toolchain: Rust, stable toolchain (host 1.95; above every MSRV: axum 1.80, rmcp 1.88, std `File::lock` 1.89)
- Concurrency Model and Server Stack (KEYSTONE): Hybrid — `run` / `hook` / `send` are synchronous on std threads with interprocess 2.4.4 sync local sockets; a Tokio 1.53.1 runtime is built only inside `mcp` (rmcp 3.4.1) and `ui` (axum 0.8.9 `sse`), plus `wait` if it needs one. `channel` keeps one wire format with a sync server inside `run` and a sync client; only `mcp` (and `wait` if async) uses interprocess's Tokio flavour. No runtime on the hot `hook` path.
- PTY Layer: portable-pty `=0.8.1` pinned, behind viola's own `pty` seam (spawn · read · write · resize · wait · kill). Child exit detected on the process, never on EOF; if 0.8.1's Windows `kill()` misbehaves, TerminateProcess on `child.process_id()` via windows-sys (already in the tree through interprocess). Swap triggers: → portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run` (then test `PASSTHROUGH_MODE`); → own ConPTY on windows-sys 0.61.2 + nix 0.31.3 only after a measured defect the seam cannot absorb. portable-pty 0.9.0 excluded (wezterm#6783). Evidence: spike S1, O1 render stress, the Pulse live test (§6), and (overseer, founder-delegated, 2026-09-23) this arch session running under the prototype on 0.8.1 with pass-through in Windows Terminal, about an hour, no PTY defect.
- Screen Model: vt100 0.16.2 as a narrow pre-send readiness gate on `run`'s pump thread — readiness and modal detection only, never content. After turn-ended, wait for the screen to go quiet (the screen lags the hooks), then require the version-stamped input-box signature and the absence of any known modal signature; otherwise hold the send, type nothing, and report `not-delivered` with detail `input-not-ready`. Delivery confirmation stays the backstop. Signatures live in `agent::claude` as the capability ledger's "CLI-native modals no hook reports" row, re-stamped by `viola verify`; on an unverified build the gate falls back to confirmation only.
- State Store (KEYSTONE): ndjson append logs (authoritative audit trail) + atomically replaced JSON snapshots (atomic-write-file 0.3.1 or tempfile 3.27.0 `persist`) + std `File::lock` on a separate `.lock` file (rust-lang/rust#54118). serde_json 1.0.151. One `write` call per event line; readers heal a torn last line; the UI rebuilds by replaying/tailing. The wheel is decided in the wrapper's memory and mirrored to a snapshot; budget readings are timestamped last-writer-wins under `File::lock`. No C in the build. Upgrade path if a real query need appears: a derived, rebuildable SQLite index (rusqlite) beside the authoritative log.
- Wrapper IPC Channel: interprocess 2.4.4 `local_socket` (named pipe on Windows, Unix socket elsewhere), one endpoint per `viola run` (no daemon), endpoint name derived from a short hash of the instance (macOS ~104-byte socket path limit), never the raw `VIOLA_NAME` + path. `hook` blocks on the channel for a dialog answer; `wait` blocks until turn-ended (no polling). Behind viola's own `channel` seam. Risk noted: one main maintainer. [determined by Q2; confirmed]
- MCP Server: rmcp 3.4.1 (official SDK), features `server` + `transport-io` (stdio), optional schemars 1.x for tool input schemas; implements MCP 2026-07-28, compatible with 2025-11-25; minor pinned. A thin adapter over the same channel verbs the CLI uses. Tools: `send` · `wait` · `last` · `answer` (question, permission and plan answers — how a driver answers dialogs under R7) · `list` (sessions, state, wheel). `release` is NOT an MCP tool (CLI only). Refusals are `isError: true` tool results mapped from `result.refusal`. [determined by Q2; confirmed with two tools added by overseer (founder-delegated): brief §3.2 named send/wait/last before hook-answered dialogs existed; the prototype driver uses answer and list in every session]
- Mobile Framework: N/A — the phone view is a later version and will be the same web page behind authentication
- AI/ML Serving: N/A — viola calls no model API; it drives the unmodified `claude` CLI on the user's subscription
- Push Notifications / Real-time (GUI Live Feed): SSE via axum 0.8.9 `sse` (`Sse::keep_alive`), fed by tailing the ndjson logs with notify 8.2.0; `text/event-stream` excluded from tower-http 0.7.1 `CompressionLayer`. Routes: `/api/sessions`, `/api/links` (GET), `/api/events` (SSE). Session liveness (overseer, founder-delegated): the prototype's heartbeat is the primary signal — the wrapper touches a heartbeat file every second, a stale beat means gone; a pid check matched together with the process start time recorded in the snapshot (guards against pid reuse; sysinfo 0.39.6) and `claude agents --json` (status enrichment; unwrapped sessions as read-only rows) only enrich it. [determined by Quiz I GUI + Q2 + Q5; confirmed with liveness change]
- Timestamp Type: chrono 0.4.45 (already in the binary via rmcp). RFC 3339 UTC with millisecond precision in frames, log events and snapshots. External `resets_at` parsed tolerantly; a malformed value is recorded as `unknown`, never rejected.

## Infrastructure Decisions
- Deployment / Distribution (KEYSTONE): `cargo install --path .` puts `viola` on PATH. The Claude Code plugin is compiled into the binary: `hooks/hooks.json`, `.mcp.json` and `.claude-plugin/plugin.json` via `include_str!`, `plugin.json` version from `CARGO_PKG_VERSION`. `viola run` writes them to `<viola home>/plugin/<version>/`, puts its own `current_exe()` path (forward slashes) into every exec-form `command`, passes that folder to the child with `--plugin-dir`, sets `VIOLA_BIN` and puts viola's own folder first on the child's PATH. Hooks and MCP never search PATH. An optional `viola plugin install` writes the same tree into a local marketplace for an unwrapped driver's MCP verbs. Evidence (overseer, founder-delegated, 2026-09-23): the prototype was rebuilt three times under this running session — a running viola.exe cannot be overwritten on Windows, so the old one was renamed aside; the wrapper kept the old binary while every PATH-found hook ran the new one (wanted that time, as a wheel-classifier hot-fix, but it proves mixed versions are real). Known trade-off: under this decision a hook fix applies only to sessions started after the rebuild. dist (cargo-dist) 0.33.0 + cargo-auditable 0.7.6 is the public v1.x step; self_update 1.3.0 later.
- Plugin scope (overseer, founder-delegated correction to Q7's premise): by default the plugin is loaded only into wrapped sessions via `--plugin-dir` from `viola run` (as the prototype does; this arch session runs that way). Per brief §3.2 both driver and driven run under `viola run`, so unwrapped sessions carry no viola hooks at all. A user-level plugin install stays optional (for the MCP verbs in an unwrapped driver); its hooks keep the fast exit 0 when `VIOLA_NAME` is absent.
- CI/CD: GitHub Actions matrix on `windows-2025` (windows-latest), `macos-latest` (macOS 26) and `ubuntu-*`; dtolnay/rust-toolchain + Swatinem/rust-cache 2.9.2; native runners, no cross-compiler; tests run against the fake agent (the real CLI only in local live tests); a CI step `cargo check -p viola-channel -p viola-pty -p viola-state` proves the sync path is Tokio-free on all three OSes.
- Hosting / containers / serverless: N/A — local-only v1

## Convention Decisions
- Hook Transport: exec-form `command` hooks (`"command": "<viola current_exe>", "args": ["hook", "<event>"]`; no shell, so no Git Bash path rewriting). Tiered split with scoped matchers:
  - sync, returns decisions: PreToolUse with matcher `AskUserQuestion|ExitPlanMode` only, and PermissionRequest (request/reply frames paired by id);
  - sync, no reply body — the ordering spine: SessionStart, UserPromptSubmit, Stop (one-way frames);
  - SessionEnd: sync with viola's own ~1 s deadline (channel connect, else a direct `try_lock` append, else nothing); `run` writes `session-end` from the process exit if no hook event arrived; the 1.5 s budget is never raised;
  - `async: true` and must print nothing: Notification, PostToolUse, PostToolUseFailure — unordered audit/feed entries sorted by hook-recorded timestamp; `asyncRewake` unused.
  The async/sync tier map is a capability-ledger row re-stamped by `viola verify`. `http` hooks excluded (fixed URL = central listener); `mcp_tool` excluded (unavailable at SessionStart).
- Hook contract: exit 0 + JSON body = a decision; exit 0 with no body = no decision (the `unverified-cli` degrade path — the dialog renders for the human); exit 2 is never produced (hooks fail open on any internal error); fast exit 0 when `VIOLA_NAME` is absent.
- API Style (Wrapper Channel Protocol, KEYSTONE): JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde 1.0.229 / serde_json 1.0.151 (no JSON-RPC crate; jsonrpsee is Tokio-bound). Hook dialog requests and `send`/`wait`/`list`/`answer` are requests with an `id`; timestamped hook events are notifications without one. A reply's `result` is `{"ok":…}` or `{"refusal":"<RefusalReason>","detail":…}`; `error` (`-32600`, `-32601` unknown method, `-32602` bad params / unsupported version) is for protocol faults only. Every `params` carries `"v": <proto>` plus the sender's `CARGO_PKG_VERSION`; no handshake round trip. Multi-line pastes, dialog payloads and stdin/`--file` text travel as escaped strings on one line.
- CLI conventions: clap 4.6.7; human text by default, `--json` for agents; typed exit code per refusal; prompt text from stdin or `--file`, never leading-slash arguments (Git Bash rewriting, brief §6), with a warning when an argument carries a rewritten-path prefix.
- Error Handling: thiserror 2.0.20 — one typed error enum per workspace crate; anyhow 1.0.104 only at the `viola` bin edge (main / subcommand dispatch). `RefusalReason` lives in `viola-core`: `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` (+ `detail`, e.g. `input-not-ready`) · `#[serde(other)] Unknown`. On the wire: refusal → `result.refusal`, protocol fault → `error.code`. CLI: typed exit code + `--json`; MCP: `isError: true`. nutype validation errors fold into the owning crate's enum.
- Validation Library: external payloads (hook JSON, `claude agents --json`, statusline `rate_limits`) parsed tolerantly — no `deny_unknown_fields`, `#[serde(default)]`/`Option<T>`, raw `serde_json::Value` kept for the fixture recorder, serde_path_to_error 0.1.20 to report drift. viola's own formats are versioned-tolerant: every frame, event and snapshot carries `v`; readers skip unknown event kinds and fields and count and show what they skipped; a channel request whose `params.v` is higher than supported gets `-32602` naming the wrapper's version; a snapshot of an unreadable major version falls back to replaying the log. nutype 0.8.0 newtypes for viola's own inputs: `ViolaName` (charset + length cap, keeps endpoint names short) and `Percent` (0–100, budget thresholds). No garde/validator. Optional: schemars 1.2.2 (MCP tool schemas), jsonschema 0.57.0 (check `viola verify` fixtures).
- Module Boundary Enforcement: Cargo workspace, flat `crates/` layout. The root `Cargo.toml` is both the `viola` bin package (required by `cargo install --path .`; also holds the `include_str!` plugin files) and `[workspace] members = ["crates/*"]`, `resolver = "2"`, `version.workspace = true`, `[workspace.dependencies]`. Members: `viola-core` (normalised events, `RefusalReason`, `ViolaName`/`Percent`), `viola-pty`, `viola-channel` (Tokio client behind a `tokio` feature only `viola-mcp` enables), `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui`. Only `viola-mcp` and `viola-ui` list `tokio`. cargo-deny 0.20.2 for licence and C-dependency bans; cargo-modules 0.27.0 for graph review.
- Primary Key Strategy: N/A — no database; instances are identified by `ViolaName`, endpoints by its short hash
- GUI surface: view-only; bound to 127.0.0.1; Host header allowlist `127.0.0.1:<port>` / `localhost:<port>`; no token or CSRF in v1 (brake auth deferred to security, v1.x)

## Deferred to specialists / later versions
- Frontend framework: deferred to design specialist
- GUI brake auth (per-launch token, Origin checks) and auth generally: security specialist (v1.x)
- Test framework, fake-agent harness, `viola verify` probe-suite design: tests specialist
- Logging / observability: obs specialist
- Self-update, signed binaries, winget/Homebrew/Scoop, marketplace entry: later version

## Recommendation Adherence
- Concurrency Model: accepted (option 3, hybrid) — KEYSTONE
- PTY Layer: accepted (option 1) + evidence added
- Screen Model: accepted (option 1)
- State Store: accepted (option 1) — KEYSTONE
- Wrapper IPC Channel: confirmed as determined by Q2 (no separate research)
- Hook Transport: accepted (option 3) with a correction to its premise (plugin only in wrapped sessions)
- MCP Server: confirmed as determined by Q2, with two tools added (answer, list)
- Distribution: accepted (option 2) + measured mixed-version evidence — KEYSTONE
- Channel Protocol: accepted (option 1) — KEYSTONE
- GUI Live Feed: confirmed as determined, with the liveness sub-choice changed (heartbeat primary; pid + start time)
- Validation: accepted (option 2)
- Timestamp Type: accepted (option 1, low-stakes, no research)
- Module Boundaries: accepted (option 1)
- Error Handling: accepted (option 1, grounded in prior answers, no separate research)
All amendments: overseer (founder-delegated). 0 overridden · 0 dug deeper.

## Defaults Applied
None — all fields explicitly chosen.
