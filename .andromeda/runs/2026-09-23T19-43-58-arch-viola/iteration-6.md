## Design Philosophy

- **Mechanism, not policy.** viola carries input, answers dialogs when told to, logs everything and holds the wheel, but it never decides what to answer. Andromeda's decision rights and any other driver-side policy stay outside the binary, so every feature has to be expressible as a general bridge verb.
- **Measured, never assumed.** Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger. On a CLI build nobody has verified, viola degrades to transport-only, and every send is confirmed after the fact (a matching `prompt-submitted` or, for a local command on the ledger, its measured post-condition; otherwise `not-delivered`), never presumed. The one exception is a ledger-listed local command whose post-condition is "none" or not yet measured on this CLI version: it returns `ok` with `confirmed: false` and detail `unconfirmable`.
- **The human always wins the wheel.** Human keystrokes are never blocked, refused or delayed past the current atomic paste. Automation is refused with a typed reason (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`) instead of competing with the human.
- **No daemon; the disk is the shared truth.** Every `viola run` owns its own local-socket endpoint, every other verb is a short-lived process, and the state all processes share lives in ndjson logs and atomic snapshots that survive a crash on either side.
- **Cross-platform from the first commit, Windows first.** Windows is the live-supported target. The macOS and Linux paths build and pass CI on every commit against the fake agent, so no design choice may depend on tmux, a shell or POSIX-only behaviour.

## Stack and Technologies

| Layer | Technology | Role |
|---|---|---|
| Language / runtime | Rust stable (host 1.95; workspace `rust-version = "1.89"`, edition 2024) | One native binary `viola` / `viola.exe`. 1.89 is the highest floor among the dependencies (std `File::lock`); axum needs 1.80 and rmcp 1.88 |
| Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp`, `ui` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp and axum require it |
| HTTP server (GUI) | axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, with `text/event-stream` excluded) | View-only GET routes and the SSE feed on 127.0.0.1 |
| CLI parser | clap 4.6.7 (derive) | Subcommands `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install` |
| PTY layer | portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; windows-sys 0.61.2 for the `TerminateProcess` kill fallback | Hosts the unmodified `claude` in ConPTY (Windows) or openpty (Unix) |
| Screen model | vt100 0.16.2 | Pre-send readiness and modal detection on `run`'s pump thread. Never used to read content |
| Wrapper IPC | interprocess 2.4.4 `local_socket` (sync API; Tokio flavour only in `viola-mcp`) | One endpoint per `viola run`: a named pipe on Windows, a Unix domain socket elsewhere |
| Database | None. ndjson append logs + atomically replaced JSON snapshots on the local filesystem | Authoritative audit trail and mutable state (wheel mirror, links, budget readings) |
| ORM / migrations | N/A. Every record carries `v`, readers skip unknown kinds and fields, and a snapshot with an unsupported `v` is rebuilt by replaying the log | Stands in for schema migration |
| State-file primitives | atomic-write-file 0.3.1 (snapshots); std `File::lock` on separate `.lock` files; std `OpenOptions::append` (one `write` per line) | Crash-safe multi-process writes with no C code |
| Serialization | serde 1.0.229, serde_json 1.0.151, serde_path_to_error 0.1.20 | Channel frames, log events, snapshots, tolerant parsing of external payloads with path-precise drift reports |
| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100) |
| Timestamps | chrono 0.4.45 | RFC 3339 UTC, millisecond precision. Already in the tree through rmcp |
| Error types | thiserror 2.0.20 (per crate); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains only at dispatch |
| Message broker | None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response; notify 8.2.0 tailing the ndjson logs for fan-out to `ui` | Local IPC and event fan-out without a broker or daemon |
| Push / real-time | SSE through axum 0.8.9 `Sse::keep_alive`, fed by notify 8.2.0 | Live GUI feed `/api/events` |
| Process liveness | sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file |
| MCP server | rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list` |
| Fixture schema check | jsonschema 0.57.0 (optional, dev-dependency only; adoption is the tests specialist's call) | Validates the hook-payload fixtures recorded by `viola verify`. No runtime crate depends on it |
| AI/ML serving | N/A | viola calls no model API. It drives the unmodified `claude` CLI on the user's subscription |
| Mobile framework | N/A | The phone view is a later version: the same web page behind authentication |
| Container runtime / deployment | None. `cargo install --path .`; Claude Code plugin compiled into the binary (`include_str!`) and written out by `viola run` | Local-only v1, no hosting |
| CI/CD | GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`; dtolnay/rust-toolchain; Swatinem/rust-cache 2.9.2 | Build, lint and test on all three OSes against the fake agent, on native runners |
| Code quality | rustfmt, clippy (`-D warnings`), `cargo check`; cargo-deny 0.20.2 (licences, C-dependency bans); cargo-modules 0.27.0 (module graph review) | Lint, typecheck, dependency policy, boundary review |
| Release (v1.x, not v1) | dist (cargo-dist) 0.33.0 + cargo-auditable 0.7.6; later self_update 1.3.0 | Public signed releases and installers once distribution is in scope |

## Established Decisions

- **[Scale / Product] Personal v1, cross-platform from day one.** The founder runs v1 on their own subscription, with no accounts, no hosting and the GUI on 127.0.0.1. It is still built and CI-tested on Windows, macOS and Linux from the first commit, and the on-disk state is parsed defensively. Rationale: retrofitting cross-platform support onto a PTY/IPC tool costs far more than carrying it from the start, and the later public version (individual Claude Code subscribers) must not need a rewrite.
- **[Scope Boundary] General bridge, nothing Andromeda-specific inside.** Rationale: R4 (mechanism, not policy). Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair (Pulse, Conductor, viola itself).
- **[Agent Coverage] Claude-first, with one concrete `viola-agent-claude` crate and no adapter trait yet.** The PTY, channel, wheel and state layers speak only viola's normalised events. Hook JSON parsing, the dialog answer mapping (S3/S7/S8), the R8 identity strip, npm-shim → `claude.exe` resolution and the CLI version gate live only in `viola-agent-claude`. Rationale: a trait designed before a second agent exists would encode guesses. Isolating the agent in one crate keeps the later extraction mechanical.
- **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` and `ui` (KEYSTONE).** Rationale: portable-pty's handles are blocking, so the pump needs threads either way. `hook` spawns once per hook event and must not pay for runtime startup. rmcp, axum and interprocess-async all require Tokio, so it is confined to the subcommands that use them. CI enforces this with the `cargo deny` `tokio` ban, evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of every sync crate without the `tokio` feature on all three OSes (CI job 3).
- **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).** Child exit is detected on the process handle, never on EOF, because ConPTY does not close the output stream on exit. If 0.8.1's Windows `kill()` misbehaves, the fallback is `TerminateProcess` on `child.process_id()` via windows-sys. Rationale: 0.8.1 is spike-proven (S1, O1 render stress, the Pulse live test, and about an hour driving this arch session in Windows Terminal with no PTY defect). 0.9.0 is excluded because of the Windows garbage-read bug wezterm#6783. Swap triggers: portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run` (then test `PASSTHROUGH_MODE`). An own ConPTY on windows-sys 0.61.2 + nix 0.31.3 comes only after a measured defect the seam cannot absorb.
- **[Screen Model] vt100 0.16.2 as a pre-send readiness gate only.** After `turn-ended`, the gate waits for the screen to go quiet, then requires the version-stamped input-box signature and the absence of every known modal signature. The quiet period and the gate's maximum wait are open items, held as ledger rows beside the signatures. If the screen is not quiet within the maximum wait, or either check fails, it types nothing and reports `not-delivered` with detail `input-not-ready`. Rationale: CLI-native modals bypass every hook (measured on 2.1.280: the "Teach auto mode…" modal swallowed a paste), and the screen lags the hooks. Signatures live in the capability ledger. On unverified builds the gate falls back to delivery confirmation only.
- **[Delivery Confirmation] Every `send` is confirmed.** If no matching `prompt-submitted` arrives within the confirmation window, the send is reported as `not-delivered` with detail `no-prompt-submitted`. The confirmation window is an open item: its default comes from `viola verify` measurements and is held as a capability-ledger row per CLI version, not in `config.json`. A CLI version with no stamped window (every unverified build) uses a built-in fallback window compiled into `viola-agent-claude` (value: open item). `send` blocks for at most the readiness gate's maximum wait, plus the atomic paste, plus the window, and returns its result no later than when the window closes. On the MCP surface that total must stay below the MCP client's tool-call timeout. Before comparing, matching reverses the CLI's escaping of tag-like text (a ledger row). Skills are expanded into a prompt and fire UserPromptSubmit like any prompt (measured: `/andromeda-arch`), so a slash send that is not on the ledger's local-command list is an ordinary send, and if it is not confirmed the result is `not-delivered`. A built-in local command fires no UserPromptSubmit, so the ledger lists the known local commands, each with its post-condition or "none". If a listed command's measured post-condition is met, the send returns `ok`, confirmed. For `/clear`, that post-condition is a SessionStart with source `clear` and a new `session_id`. If a listed command's post-condition is "none" or not yet measured on this CLI version (`/remote-control` today), the send returns `ok` with `{"confirmed":false,"detail":"unconfirmable"}`. Nothing else is ever unconfirmable. Any other unconfirmed send is `not-delivered`. The decision depends on the ledger, never on a leading slash. Rationale: a turn boundary per the hooks does not prove the input box is ready. Confirmation is the backstop behind the screen gate. Local commands would otherwise be false negatives (measured: `/remote-control` fired no UserPromptSubmit), and `/clear` matters most because drivers clear between pipeline skills.
- **[Database / State Store] ndjson append logs + atomic JSON snapshots + std `File::lock` on separate `.lock` files (KEYSTONE).** Each event line is written with one `write` call. Readers heal a torn last line. The UI rebuilds by replaying and tailing. The wheel is decided in the wrapper's memory and mirrored to a snapshot. Budget readings are timestamped, last-writer-wins, under `File::lock`. Rationale: the audit trail stays human-readable and `tail`-able, there is no C in the build, and there is no daemon to serialise writers. SQLite (C build, binary format) and redb (multi-process mode hardened only 2026-09-06) were rejected. Lock files are separate from logs because append + exclusive lock fails on Windows (rust-lang/rust#54118). Upgrade path if a real query need appears: a derived, rebuildable SQLite index (rusqlite) beside the authoritative log. The log stays the source of truth.
- **[Snapshot writer] atomic-write-file 0.3.1 over tempfile 3.27.0 `persist`.** Rationale: it is purpose-built for same-directory temp + fsync + rename, so each call site has less hand-written code. tempfile remains the drop-in alternative.
- **[Message Broker / IPC] interprocess 2.4.4 `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam.** `hook` blocks on the channel for a dialog answer. `wait` blocks until the next driver-relevant event (`turn-ended`, `question`, `permission`, `plan` or `session-end`) and returns that event, so nothing polls and a driver parked in `wait` learns that a dialog is pending. If a dialog is already pending when `wait` is called without `after`, or with an `after` at or before that dialog's event line, it returns that dialog at once. With an `after` past that line the driver has already seen the dialog, so `wait` blocks for the next driver-relevant event, and a driver that leaves a dialog to the human never spins. Each dialog event carries a wrapper-assigned `dialog_id` (an integer, monotonic per instance, with the counter restored from `events.ndjson` on start so an id never repeats across wrapper restarts), and `answer` names its target by that id. At most one dialog is pending per instance, because the dialog blocks the turn. `wait` takes an optional `after` cursor and an optional `timeout_ms`. `after` is a byte offset into the instance's `events.ndjson`. `wait` returns the first driver-relevant event whose line starts at or after it, at once if that event is already logged, so a turn that ends before the driver calls `wait` is never missed. Every `send` `ok` payload and every `wait` event result carries `cursor`, the offset to pass as the next `after`. Without `after`, `wait` returns the next such event appended after the call. With no `timeout_ms` it blocks until an event arrives, and if the endpoint vanishes first the caller gets `instance-unreachable`. On timeout it returns `ok` with `{"timed_out":true}`. `last` returns the `last_assistant_message` of the newest `turn-ended` event. The wrapper holds it in memory and rebuilds it from `events.ndjson` on start. Endpoint names are a short hash of the instance, never the raw `VIOLA_NAME` + path. Rationale: one API over named pipes and Unix sockets, sync and Tokio flavours, and no daemon. macOS caps socket paths at about 104 bytes. Accepted risk: interprocess has one main maintainer, which the seam contains.
- **[API Style] JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde (KEYSTONE).** Requests carry an `id` (hook dialogs, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`). `list` is a caller-side read of the disk and `claude agents --json`, not a channel request. Hook events are notifications without one. Refusals travel in `result.refusal`. `error` is reserved for protocol faults. Every `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. Rationale: it has the same shape as MCP, so `mcp` is a thin adapter. jsonrpsee is Tokio-bound and would drag a runtime into `hook`. Mixed binary versions are a measured fact, so every frame self-describes.
- **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.** Refusals are `isError: true` tool results mapped from `result.refusal`. Rationale: it is the official SDK, tracks spec 2026-07-28, and the pinned minor limits churn from its near-weekly releases. `answer` and `list` are how a driver answers dialogs under R7 and sees sessions, and the prototype driver uses both every session. `release` stays with the human so an agent cannot hand the wheel back to automation. `pause`, `link` and `unlink` are CLI-only for the same reason: they are human controls over the wheel and the link topology, and links made by drivers are already recorded from `from` on `send` and `answer`. Tool inputs:
  - `send {target, text}`
  - `wait {target, after?, timeout_ms?}`
  - `last {target}`
  - `answer {target, dialog_id, response}`, with `response` in the channel `answer` shape
  - `list {}`

  `target` is a `ViolaName`, and the server adds `from` from its own `VIOLA_NAME`. The default `timeout_ms` for MCP `wait` is an open item and must stay below the MCP client's tool-call timeout.
- **[Hook Transport] Exec-form `command` hooks: `"command": "<pinned viola copy>", "args": ["hook", "<event>"]`.** Tiers:
  - sync with a decision: PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, plus PermissionRequest.
  - sync with no reply body (the ordering spine): SessionStart, UserPromptSubmit, Stop.
  - SessionEnd: sync with viola's own ~1 s deadline (channel, else a direct `try_lock` append, else nothing). `run` writes `session-end` from the process exit if no hook arrived.
  - `async: true`, printing nothing: Notification, PostToolUse, PostToolUseFailure.

  Rationale: exec form skips Git Bash path rewriting and shell startup. `http` hooks need a fixed URL, which means a central listener and so a daemon. `mcp_tool` is unavailable at SessionStart. The 1.5 s SessionEnd budget is never raised. The tier map is itself a ledger row that `viola verify` re-stamps.
- **[Hook Contract] Hooks fail open.** Exit 0 with a JSON body is a decision. Exit 0 with no body is no decision (the `unverified-cli` degrade: the dialog renders for the human). Exit 2 is never produced. `hook` exits 0 at once when `VIOLA_NAME` is absent. A sync dialog hook waits on `hook.dialog` for at most viola's dialog deadline. That deadline is an open item: one built-in value, compiled into both `hook` and the embedded `hooks.json`. The `timeout` in `hooks.json` exceeds the deadline by a margin, so viola, never Claude Code, ends the wait. On expiry the hook exits 0 with no body and the dialog renders for the human. The wrapper then clears the pending `dialog_id`, and a later `answer` for it is refused `not-delivered` with detail `unknown-dialog`. `hook` never writes to stderr. Its diagnostics go only to the instance's `diagnostics/` directory (format owned by obs), and a failure to write them is ignored. Rationale: exit 2 is a blocking error in Claude Code's contract, and a viola bug must never block the user's session.
- **[Human Takeover / Wheel] Byte-source claim + atomic send window + after-the-fact confirmation, explicit release.** Any human editing key since the last turn boundary moves the wheel to the human, and `send` is refused with `human-typing`. Focus, mouse and resize sequences do not count. A bridge send is written as one `ESC[200~…ESC[201~` + CR write. Human bytes arriving during that write are held in the pump and passed on straight after. An unsent `prompt-submitted` confirms and logs the take. Harness-injected prompts (starting `<agent-message from=` or `<task-notification>`) are classified as harness turns, never as a human take. The wheel starts with `driver` when `viola run` starts, so automation is accepted until the first human editing key. A `send` while a turn is running (after `prompt-submitted`, before `turn-ended`) is not queued. It is refused `not-delivered` with detail `turn-running`, and the driver calls `wait` first. CLI `viola pause` moves the wheel to the human without a keystroke, after which automated `send` is refused `human-typing` with detail `manual-pause`. It is the CLI twin of the reserved v1.x GUI brake route. The wheel returns only through CLI `viola release`. While the wheel is `human`, the wrapper answers `hook.dialog` with `null` at once, so the dialog renders for the human without waiting out the dialog deadline, and `answer` is refused `human-typing`. If the wheel moves to the human while a `hook.dialog` is already pending (a human editing key or `viola pause`), the wrapper answers that pending request with `null` at once and clears its `dialog_id`, so a dialog held by the hook is never hidden from the human for the rest of the deadline. `budget-paused` gates only `send`, never `hook.dialog` or `answer`, because a running turn finishes. Rationale: `viola run` owns the real stdin, so it knows byte provenance exactly. The prototype paused twice on harness turns before the classifier was fixed.
- **[Links] A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`. `viola unlink` removes it.** A call counts as coming from inside a wrapped session when the calling process has `VIOLA_NAME` set. The CLI and the MCP server pass that name as `from` in the request `params`, and the driven instance's wrapper appends a `link` event to its `events.ndjson`. `unlink` appends an `unlink` event. Links are events in the log. The link set in the snapshot, and so in `/api/links`, is derived by replaying them. Rationale: D5 asks the GUI to show which sessions are linked, and the log stays the single source of truth for that as it is for everything else.
- **[Budget Governor] Soft stop at the turn boundary.** Once a configured threshold is crossed (defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %), automated `send` is refused with `budget-paused` through the same refusal path as `human-typing`. Human keys are never blocked, and a running turn finishes. The pause lifts when `resets_at` passes or on `viola release --budget`. `release --budget` is sent to one instance's wrapper and lifts the pause for that instance only, until the crossed window's `resets_at` passes, so the next over-threshold reading does not re-pause it. The wrapper mirrors its budget-pause state, including any override, to the instance snapshot, where `list` and `ui` read it. With figures missing, viola uses the newest reading from any wrapped session and shows its age. With no reading at all it records `unknown` and does not block. The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that then runs the user's own statusline command unchanged. Rationale: the per-model weekly window is hidden (claude-code#91920), so thresholds are set low. The user's global statusline config is never touched, and a visible, conservative governor supports the "ordinary individual usage" posture.
- **[CLI Version Compatibility] Per-behaviour capability ledger + transport-only degrade + `viola verify` re-stamp.** Ledger rows:
  - S3: AskUserQuestion via PreToolUse `updatedInput`.
  - S7: ExitPlanMode approves only through PreToolUse; revise via `deny` + `message`.
  - S8: free text and `annotations`.
  - The R8 `CLAUDE*` strip list.
  - npm-shim → `claude.exe` resolution.
  - Statusline `rate_limits.*`, the settings-override mechanism, and the shell Claude Code runs a statusline command through on each OS.
  - Harness prompt prefixes.
  - The `<pasted_content id=…>` wrapper around long pastes (981 bytes wrapped, 749 not, on 2.1.280).
  - Modal and input-box screen signatures.
  - The hook tier map.
  - Local commands: built-in local commands fire no UserPromptSubmit (measured: `/remote-control`), so the ledger lists each known local command with its delivery post-condition or "none". For `/clear` it is a SessionStart with source `clear` and a new `session_id`, measured in `viola verify`. `/remote-control` has no measured post-condition today. Skills are not local commands: they expand into a prompt and fire UserPromptSubmit (measured: `/andromeda-arch`).
  - Tag escaping: the CLI inserts a backslash after `<` in tag-like text the user typed, so a literal `<pasted_content` arrives in UserPromptSubmit as `<\pasted_content`. Delivery matching reverses this escaping before comparing sent text with submitted text.

  `viola run` learns the CLI version by running the resolved child executable once with `--version` before spawning it. Output it cannot parse counts as an unlisted version. The fake agent answers `--version` the same way. On an unlisted version viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get `unverified-cli`. `viola verify` runs the live probe suite (Haiku, local only), checks post-conditions, stamps the version and records the hook-payload fixtures the fake agent replays in CI. Rationale: the docs lag the installed build, and several load-bearing behaviours are measured, not documented.
- **[Real-time Strategy] SSE through axum `Sse::keep_alive`, fed by notify 8.2.0 tailing the ndjson logs.** Rationale: a one-way stream fits a view-only page. WebSocket would open a two-way surface before any auth exists, and polling lags the hooks and adds disk reads.
- **[Session Liveness] Heartbeat file primary; pid + process start time and `claude agents --json` only enrich it.** Rationale: the prototype's heartbeat proved reliable. A bare pid is unsafe because pids get reused, which is why start time is matched too. `claude agents --json` adds idle/busy status and shows unwrapped sessions as read-only rows. A `viola run` whose `ViolaName` has an answering endpoint or a `stale` heartbeat refuses to start (exit 1, with a message naming the live instance). A gone instance's directory is reused, and its `events.ndjson` is appended to, never truncated.
- **[GUI Control Scope] View-only in v1.** Only GET routes plus SSE, bound to 127.0.0.1, with a Host allowlist of `127.0.0.1:<port>` and `localhost:<port>`, and no token or CSRF. `ui` is a pure reader of the on-disk ndjson and `claude agents --json`. Rationale: a page that cannot change state gives DNS rebinding nothing to exploit beyond reading. The page shows each session row (wrapped, or read-only unwrapped), its liveness (`live` · `stale`), idle/busy status, wheel holder and any pending dialog. It also shows the budget reading and its age, the link set, the `skipped` counts, and a live event feed over SSE. In v1 it is view-only, and no control accepts input. The v1.x step is a brake: the reserved `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink` routes, with the same effect as CLI `viola pause` and `viola unlink`. What must hold is that no state-changing route is reachable cross-origin or without per-launch proof that the caller is the local user. The mechanism is owned by the security specialist.
- **[Timestamps] chrono 0.4.45, RFC 3339 UTC, millisecond precision everywhere viola writes.** External `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error. Rationale: chrono is already in the tree through rmcp, and external clocks must never break parsing.
- **[Validation] Two regimes.** External payloads (hook JSON, `claude agents --json`, statusline) are parsed tolerantly: no `deny_unknown_fields`, `#[serde(default)]` / `Option<T>`, the raw `serde_json::Value` kept for the fixture recorder, and serde_path_to_error for drift reports. viola's own formats are versioned-tolerant: every frame, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. nutype newtypes `ViolaName` and `Percent`. No garde or validator. Rationale: the CLI changes between builds, and parsing must never break when the ledger is the thing that decides behaviour. `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
- **[Error Handling] thiserror 2.0.20, one enum per workspace crate; anyhow 1.0.104 only in the `viola` bin's `main` and dispatch.** `RefusalReason` lives in `viola-core`. Rationale: callers branch on refusal variants across three surfaces (channel, CLI exit codes, MCP `isError`). anyhow's context chains are only useful where errors reach a human.
- **[Module Boundaries] Cargo workspace, flat `crates/` layout, with the root `Cargo.toml` as both the `viola` bin package and `[workspace]`.** Members: `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui`. Only `viola-mcp` and `viola-ui` list `tokio`, and `viola-channel`'s Tokio client sits behind a `tokio` feature only `viola-mcp` enables. Rationale: the compiler enforces that `viola-pty` cannot see `viola-agent-claude`. `cargo install --path .` requires the bin at the root. cargo_pup needs nightly and was rejected.
- **[Deployment / Distribution] `cargo install --path .`, with the plugin compiled into the binary (KEYSTONE).** `hooks/hooks.json`, `.mcp.json` and `.claude-plugin/plugin.json` are embedded via `include_str!`, and `plugin.json`'s version comes from `CARGO_PKG_VERSION`. At start, `viola run` copies its own exe to `<viola home>/bin/<CARGO_PKG_VERSION>-<short content hash>/viola(.exe)` if that copy is absent. It writes the plugin files to `<viola home>/plugin/<CARGO_PKG_VERSION>-<short content hash>/` and substitutes the copy's path (forward slashes) into every exec-form `command` in `hooks.json` and `.mcp.json`. It then passes that folder with `--plugin-dir`, sets `VIOLA_BIN` to the copy and puts the copy's folder first on the child's PATH. Hooks and MCP never search PATH. Rationale: the binary and the plugin can no longer drift apart. The prototype proved that PATH-found hooks run a different binary from the wrapper after a rebuild. The content hash is part of the key because `CARGO_PKG_VERSION` stays `0.1.0` across development rebuilds, so the version alone would collide. A rebuild therefore never changes what a running session calls. Known trade-off: a hook fix applies only to sessions started after the rebuild. The per-frame `v` + sender version stays as the second safety net, for the optional user-level install (`viola plugin install`), which is not bound to a wrapper's pinned copy.
- **[Plugin Scope] The plugin loads only into wrapped sessions via `--plugin-dir`.** Both driver and driven run under `viola run`. `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver, and its hooks still exit 0 fast without `VIOLA_NAME`. Rationale: unwrapped sessions carry no viola hooks at all, so there is no per-tool-call spawn cost outside viola.
- **[CLI Conventions] Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument.** viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths (brief §6), and agents need machine-parseable refusals.
- **[CI/CD] GitHub Actions native matrix on three OSes, tests against the fake agent, the real CLI only in local live tests.** The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`). The child program is spawned as given, and npm-shim → `claude.exe` resolution applies only when it resolves to the `claude` npm shim. Each test runs in its own viola home (`--home`, see Config management), so parallel tests never share an endpoint, log or `budget.json`. Rationale: no tokens spent and no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
- **[Hosting] None.** Rationale: local-only v1. Docker, serverless and PaaS do not apply.
- **[Mobile] N/A in v1.** Rationale: the phone view is the same web page behind authentication, in a later version.
- **[AI/ML] N/A.** Rationale: D2, subscription only through the unmodified `claude` CLI. The binary makes no model API calls.
- **[Primary Keys] No database keys.** Instances are identified by `ViolaName`, endpoints by its short hash. Rationale: identity is by instance, never by location (R5).
- **[Naming] Product, binary and identifiers use the `viola` prefix.** Env vars are `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, and the newtype is `ViolaName`. `BRIDGE_NAME` and `BridgeName` in earlier artifacts are superseded. Rationale: one name everywhere removes the need to translate between prototype and product.
- **[Deferred] Deferred to specialists and later versions:** the frontend framework (design specialist); brake auth and auth generally (security, v1.x); the test framework, fake-agent harness and `viola verify` probe design (tests); logging and observability (obs); self-update, signing, winget/Homebrew/Scoop and a marketplace entry (later version).

## Conventions

**Interfaces and versioning**
- **Five interfaces from one binary:** CLI verbs, MCP tools (stdio JSON-RPC 2.0), the hook stdin/stdout contract (owned by Claude Code), the wrapper channel (JSON-RPC 2.0 over ndjson on a local socket), and GUI HTTP GET + SSE. There is no REST write surface, GraphQL, gRPC or tRPC.
- **Protocol versioning:** an integer `v` starting at `1` in every channel `params`, ndjson event line, snapshot and GUI JSON body. Adding a field or event kind is additive and needs no bump. Removing or changing a field's meaning bumps `v`. A wrapper receiving `params.v` higher than it supports answers `-32602` with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
- **Sender version:** every channel `params` carries `"sender": "<CARGO_PKG_VERSION>"`. A snapshot's `writer` holds the same value for the binary that wrote it. `/api/info`'s `proto` is the highest channel and event-line `v` the binary speaks, while its own `v` versions the body.
- **GUI routes:** unversioned paths under `/api/` (`/api/sessions`, `/api/links`, `/api/events`, `/api/info`) plus `/health` and `/ready`. Only the GET method exists. Any other method gets `405`. Content types:
  - `application/json` for `/health`, `/ready`, `/api/info`, `/api/sessions` and `/api/links`.
  - `application/problem+json` for errors, carrying `v` as an extension member.
  - `text/event-stream` for `/api/events`.
  - `text/html` and each asset's own type for `/` and `/assets/*`.

  `/ready` answers 503 with its own readiness body, never Problem Details. `state-unreadable` applies to `/api/*` routes only.
- **Channel methods:** lower-case, dot-namespaced for hook traffic. Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method. Notifications: `hook.event`. JSON-RPC ids are integers, monotonic per connection.

**Error handling schema**
- **Channel refusal (a normal outcome, not an error):** `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
- **Channel success:** `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
- **Channel protocol fault:** `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}` with codes `-32700` parse error, `-32600` invalid request, `-32601` unknown method, `-32602` bad params / unsupported `v`, `-32603` internal error.
- **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown`. `unknown` is `#[serde(other)]`, so an older client can still read a refusal from a newer wrapper. `detail` values are kebab-case strings from a closed set per reason, extended only in `viola-core`:
  - `human-typing`: `null` · `manual-pause`
  - `budget-paused`: `five-hour` · `seven-day`
  - `unverified-cli`: `null`
  - `not-delivered`: `input-not-ready` · `no-prompt-submitted` · `turn-running` · `unknown-dialog`
  - `unknown`: any value

  Readers treat an unrecognised `detail` as opaque text, never as an error. `unconfirmable` is not a refusal detail. It appears only in an `ok` payload (`{"confirmed":false,"detail":"unconfirmable"}`), for a ledger-listed local command whose post-condition is "none" or not yet measured on this CLI version.
- **CLI exit codes:**
  - `0` ok
  - `1` internal error (anyhow edge), and `viola run` refusing to start because its `ViolaName` has an answering endpoint or a `stale` heartbeat ([Session Liveness])
  - `2` usage error (clap; never produced by `viola hook`)
  - `10` `human-typing`
  - `11` `budget-paused`
  - `12` `unverified-cli`
  - `13` `not-delivered`
  - `14` `unknown` refusal
  - `20` protocol fault from the wrapper
  - `21` instance unreachable (no live endpoint for the `ViolaName`)
- **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
- **MCP:** a refusal is a tool result with `isError: true` and the same `refusal` + `detail` in `structuredContent`. An unreachable instance and a wrapper protocol fault are also `isError: true` tool results. Their `structuredContent` is `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`, the same as CLI `--json`. Only faults on the MCP connection itself are JSON-RPC errors.
- **GUI HTTP errors:** RFC 9457 Problem Details (`application/problem+json`) with `type` URNs:
  - `urn:viola:problem:host-not-allowed` (403)
  - `urn:viola:problem:method-not-allowed` (405)
  - `urn:viola:problem:not-found` (404)
  - `urn:viola:problem:state-unreadable` (503)
- **Rust error types:** one `thiserror` enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `UiError`, `CoreError`). nutype validation errors fold into the owning crate's enum. `anyhow` appears only in the root bin crate.

**Naming patterns**
- **Crates:** `viola-<area>`, kebab-case. Rust modules and files are snake_case, types are UpperCamelCase, functions and variables are snake_case, and constants are SCREAMING_SNAKE_CASE (standard Rust).
- **JSON field names:** snake_case in every viola format (`last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them).
- **Enum string values on the wire:** kebab-case (`#[serde(rename_all = "kebab-case")]`), covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
- **Normalised event kinds (`viola-core`):** `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`. New kinds are added only in `viola-core`, never with Claude-specific names.
- **Hook → kind map (in `viola-agent-claude`):**
  - SessionStart → `session-start`. `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
  - UserPromptSubmit → `prompt-submitted`.
  - PreToolUse `AskUserQuestion` → `question`.
  - PreToolUse `ExitPlanMode` → `plan`.
  - PermissionRequest → `permission`.
  - Stop → `turn-ended`.
  - SessionEnd → `session-end`.
  - Notification, PostToolUse and PostToolUseFailure → `activity`. It is log-only and never wakes `wait`.
  - `hook statusline` emits no event.
- **Environment variables:** `VIOLA_` prefix, SCREAMING_SNAKE_CASE.
- **On-disk files:** lower-case with `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file (`<name>.lock`).
- **CLI:** flags are kebab-case long options (`--json`, `--file`, `--budget`, `--port`, and the global `--home`). Subcommands are single lower-case words.

**Data model conventions**
- **No database, so no UUID or serial keys.** The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, enforced by nutype.
- **Endpoint name:** `viola-<h>`, where `<h>` is the first 12 hex characters of an FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute viola home path. It is hand-written in `viola-channel` and recorded in the instance snapshot. std `DefaultHasher` is forbidden because its output is not stable across Rust releases, and mixed binary versions must agree on the name.
- **Timestamps:** field `ts` on every event and frame, and `*_at` for other instants (`started_at`, `resets_at`, `written_at`). Always RFC 3339 UTC with milliseconds and a `Z` suffix, for example `2026-09-23T19:43:58.123Z` (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`).
- **Optional fields:** Rust `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. viola omits absent fields on write, and readers treat missing and `null` the same.
- **Unparseable external values** become the string `"unknown"` in viola's own records, never an error. This covers `resets_at`, `used_percentage` and a missing statusline window.
- **Percentages:** `Percent` newtype, 0–100, serialised as a JSON number.
- **ndjson line discipline:** one complete JSON object plus `\n` per single `write` call. Multi-line text travels as an escaped JSON string on one line.

## Standard Contracts

**GUI liveness: `GET /health`** (200 whenever the `ui` process serves requests)
```json
{"v":1,"status":"ok","name":"viola","version":"0.1.0","ts":"2026-09-23T19:43:58.123Z"}
```

**GUI readiness: `GET /ready`** (200 when viola home is readable. 503 with the same shape and `"status":"not-ready"` otherwise. `claude_agents` failing alone does not make it not-ready, because it only enriches.)
```json
{"v":1,"status":"ready","ts":"2026-09-23T19:43:58.123Z",
 "checks":{"viola_home":"ok","event_tail":"ok","claude_agents":"ok"}}
```
Check values: `"ok"` · `"unavailable"` · `"error"`.

**Service info: `GET /api/info`**
```json
{"v":1,"name":"viola","version":"0.1.0","proto":1,"pid":12345,
 "started_at":"2026-09-23T19:43:58.123Z","bind":"127.0.0.1:47319",
 "viola_home":"C:/Users/<user>/.viola",
 "verified_cli_versions":["2.1.280"]}
```

**GUI list envelope** (`/api/sessions`, `/api/links`). There is no pagination: lists are personal-scale and returned whole. `skipped` counts records a reader could not interpret, so the page can show them.
- `/api/sessions` item: `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified}`.
  - `budget_paused` is the instance's own gate from its snapshot. It is `false` after a `release --budget` override, and it is absent when the session is unwrapped. The envelope's `budget.paused` only says that the newest reading crosses a threshold.
  - `wrapped: false` marks a read-only unwrapped row from `claude agents --json`.
  - `liveness` is `live` or `stale`. Gone instances are omitted.
  - `status` is `idle`, `busy` or `unknown`.
  - `wheel` is `human` or `driver`, and is absent when the session is unwrapped.
  - The envelope also carries a top-level `budget`: `{five_hour, seven_day, resets_at, read_at, paused}`, or `"unknown"`. The page derives the reading's age from `read_at`.
- `/api/links` item: `{driver, driven, since}`.
```json
{"v":1,"generated_at":"2026-09-23T19:43:58.123Z","items":[],
 "skipped":{"unknown_kinds":0,"unknown_fields":0,"torn_lines":0}}
```

**GUI HTTP error** (RFC 9457)
```json
{"v":1,"type":"urn:viola:problem:host-not-allowed","title":"Host not allowed","status":403,
 "detail":"Host header must be 127.0.0.1:47319 or localhost:47319"}
```

**SSE feed: `GET /api/events`** (`text/event-stream`, never compressed, `Sse::keep_alive` comment every 15 s)
- `event:` is the normalised event kind, for example `turn-ended`.
- `id:` is a composite cursor over every tailed instance: `<ViolaName>:<byte offset>` pairs joined by `,`, where each offset is the end of the last line sent for that instance, or, if none has been sent yet, the offset its tail started from. Every tailed instance is always listed, so an instance that was present but quiet is never replayed from 0. On reconnect, the browser's `Last-Event-ID` resumes every listed instance from its offset.
  - An instance missing from the cursor appeared after the stream started, so it is sent from offset 0.
  - An offset beyond the file's length means the file was replaced outside viola, since viola never truncates `events.ndjson`. That instance is replayed from 0.
  - Without `Last-Event-ID`, the stream starts at each file's current end, and the page first loads current state from `/api/sessions` and `/api/links`.
- `data:` is the ndjson event line, verbatim.

**ndjson event line** (`events.ndjson`, per instance)
```json
{"v":1,"ts":"2026-09-23T19:43:58.123Z","instance":"builder","kind":"turn-ended","source":"hook","data":{}}
```
`source` ∈ `hook` · `wrapper` · `cli`. `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.

**Snapshot envelope** (atomically replaced JSON)
```json
{"v":1,"written_at":"2026-09-23T19:43:58.123Z","writer":"0.1.0","data":{}}
```
A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`.

**Wrapper channel frames** (JSON-RPC 2.0, one per line)
```json
{"jsonrpc":"2.0","id":7,"method":"send","params":{"v":1,"sender":"0.1.0","text":"line one\nline two"}}
{"jsonrpc":"2.0","id":7,"result":{"ok":{"submitted_at":"2026-09-23T19:43:59.004Z","cursor":48213}}}
{"jsonrpc":"2.0","id":8,"result":{"ok":{"confirmed":false,"detail":"unconfirmable","cursor":48213}}}
{"jsonrpc":"2.0","id":7,"result":{"refusal":"not-delivered","detail":"input-not-ready"}}
{"jsonrpc":"2.0","id":7,"error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}}
{"jsonrpc":"2.0","method":"hook.event","params":{"v":1,"sender":"0.1.0","ts":"2026-09-23T19:43:58.123Z","event":{"kind":"turn-ended"}}}
```

**Channel methods: `params` → `ok` payload.** Every `params` also carries `v` and `sender`. `from` is the caller's `VIOLA_NAME` when set, and is omitted otherwise.
- `send` `{text, from?}` → `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`. `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
- `wait` `{after?, timeout_ms?}` → `{event:<ndjson event line>, cursor}`, where `cursor` is the end offset of that event's line, or `{timed_out:true}`.
- `last` `{}` → `{last_assistant_message:<string|null>, ts:<string|null>}`.
- `answer` `{dialog_id, from?, response}` → `{}`. `response` is normalised per dialog kind:
  - `question`: `{answers:{<question>:<text>}, annotations?}`
  - `permission`: `{behavior:"allow"|"deny", message?}`
  - `plan`: `{behavior:"approve"|"revise", message?}`
- `pause` `{}` → `{wheel:"human"}`.
- `release` `{budget?:bool}` → `{wheel:<holder>, budget_paused:bool}`.
- `link` and `unlink` `{driver, driven}`, sent to the driven instance's wrapper → `{}`.
- `hook.dialog`, sent by `hook`: `{kind:"question"|"permission"|"plan", data}` → `{dialog_id, response:<object|null>}`. `null` means no decision, so the hook prints nothing. On an unverified CLI the wrapper replies `null` at once.
- `list` is not a channel method. The calling process reads `instances/*/` (heartbeat and snapshot) and `claude agents --json` itself, so `list` needs no live wrapper. Its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, with the same item, `budget` and `skipped` shapes as `/api/sessions`.

Dialog events (`question`, `permission`, `plan`) carry `dialog_id` in `data`.

**CLI `--json` output** mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code. Outcomes that have no channel `result` use `error`:
- `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
- `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` (exit 20)

**MCP refusal result**
```json
{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,
 "structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}
```

**Hook contract** (produced only by `viola-agent-claude`)
- A decision is exit 0 with a `hookSpecificOutput` body. PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`. PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
- No decision is exit 0 with an empty stdout. Async-tier hooks always print nothing.
- `viola hook statusline` is exempt from both rules, because it is the statusline wrapper, not a Claude Code hook. It records the `rate_limits` reading to `budget.json` and runs the user's own statusline command with the same stdin. `run` resolves that command from the user's effective Claude Code settings at start (in `viola-agent-claude`) and records it in the instance snapshot. `hook statusline` reads it from there through `VIOLA_DIR`. The command is a user-written shell string, so it runs through the same shell Claude Code would use for it (a ledger row). This is the only shell-out in viola. It prints that command's stdout unchanged (empty if the command fails) and exits 0.

**Session liveness**
- The wrapper touches `<instance dir>/heartbeat` every 1 s. A beat older than 5 s means the session is gone, unless the pid + start-time check from the snapshot says the process is still alive, in which case it shows as `stale`.
- `claude agents --json` adds `status` (idle/busy) and lists unwrapped sessions as read-only rows.

**Communication protocols**
- SSE for the GUI.
- JSON-RPC 2.0 over ndjson on local sockets for the wrapper channel.
- JSON-RPC 2.0 over stdio for MCP.
- No WebSocket and no polling endpoints in v1.

## Occupied Resources

**Network**
- **TCP `127.0.0.1:47319`:** the `viola ui` HTTP server (built-in default, overridable in `config.json` or with `viola ui --port <n>`). It never binds `0.0.0.0` or `::`. The Host allowlist is `127.0.0.1:<port>` and `localhost:<port>`.
- **No database port, no broker port, no other listening TCP port.** The MCP server is stdio only.

**HTTP routes (all GET)**
- `/`: the view page
- `/assets/*`: the page's static files, embedded in the binary
- `/health`, `/ready`, `/api/info`, `/api/sessions`, `/api/links`
- `/api/events`: SSE
- Reserved for the v1.x brake (not served in v1): `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink`.

**IPC endpoints**
- **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
- **Linux / macOS:** Unix domain socket file `$TMPDIR/viola-<h12>.sock` (`/tmp` when `TMPDIR` is unset), which stays within the ~104-byte macOS limit. Linux abstract-namespace names are never used, because they carry no filesystem permissions and any local user could connect (see Local endpoint trust boundary).

**Binary, subcommands and exit codes**
- Binary: `viola` (`viola.exe`).
- Subcommands: `run`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `ui`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.
- Hook event arguments: `session-start`, `user-prompt-submit`, `pre-tool-use`, `permission-request`, `stop`, `session-end`, `notification`, `post-tool-use`, `post-tool-use-failure`, plus `statusline` (the per-session statusline wrapper).
- Exit codes `0, 1, 2, 10–14, 20, 21` as defined in Conventions.

**Claude Code integration names**
- Plugin name `viola`. MCP server name `viola` in `.mcp.json`. MCP tools `send`, `wait`, `last`, `answer`, `list`.
- Registered hook events: SessionStart, UserPromptSubmit, PreToolUse (matcher `AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse, PostToolUseFailure.
- Flag passed to the child: `--plugin-dir <viola home>/plugin/<version>-<hash>`.

**Workspace crates**
- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui`.

**Environment variables**
- `VIOLA_NAME`: the instance's `ViolaName`. It is set by `viola run` in the child, and its absence makes every hook a silent exit 0.
- `VIOLA_DIR`: the absolute path of the instance's state directory (`<viola home>/instances/<ViolaName>/`), set by `viola run` in the child. It is read by `hook` (for the SessionEnd direct-append fallback, the statusline wrapper and the diagnostics directory) and by `mcp` and every CLI verb run inside the session. Each derives the viola home as the grandparent of `VIOLA_DIR`, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home.
- `VIOLA_BIN`: the absolute path of the pinned copy `<viola home>/bin/<version>-<hash>/viola(.exe)` (forward slashes), set by `viola run` in the child. viola itself never reads it, because hooks and `.mcp.json` carry the path literally. It exists so that skills and Bash tool calls inside the session can call the same pinned copy explicitly (`"$VIOLA_BIN" send …`) instead of relying on the PATH prefix.
- `PATH`: prefixed by `viola run` with the pinned copy's folder for the child.
- Removed from the child's environment (R8 strip list, a ledger row): `CLAUDECODE`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN` and the remaining `CLAUDE*` parent-identity variables on the measured list.
- Read-only, provided by Claude Code to plugin processes: `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`. viola never uses them to locate its binary.

**Filesystem (viola home = `<user home>/.viola/`)**
- `config.json`: user settings (budget thresholds, GUI port).
- `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary that `viola run` creates if it is absent, where `<hash>` is a short hash of the exe's content. v1 never deletes pinned copies or plugin folders. Each instance snapshot records the pinned path it uses, so any later cleanup (an open item) can skip copies that a live or `stale` instance still calls.
- `plugin/<version>-<hash>/.claude-plugin/plugin.json`, `plugin/<version>-<hash>/hooks/hooks.json`, `plugin/<version>-<hash>/.mcp.json`.
- `marketplace/`: written only by `viola plugin install`.
- `instances/<ViolaName>/events.ndjson`, `events.ndjson.lock`, `snapshot.json`, `snapshot.json.lock`, `heartbeat`, `settings.json` (the per-session settings override that wraps the statusline), `diagnostics/` (reserved for `hook` and wrapper diagnostics; file format owned by obs).
- `budget.json` + `budget.json.lock`: the newest `rate_limits` reading across wrapped sessions, timestamped, last-writer-wins.
- `ledger/stamps.json` + `ledger/stamps.json.lock`: `viola verify` stamps (CLI version → verified behaviours, plus the values measured for that version, such as the confirmation window). The rows themselves (probes and expected post-conditions) are compiled into `viola-agent-claude`. Read by `run`'s version gate and by `ui` (`verified_cli_versions`).

**Repository**
- `fixtures/claude/<cli-version>/`: hook-payload fixtures recorded by `viola verify` and replayed by the fake agent in CI.

**Docker volumes, containers, service names:** none.

## Infrastructure Patterns

**Build system**
- Cargo workspace with `resolver = "2"`. Shared `version.workspace = true` and `[workspace.dependencies]` pin every third-party version in one place (portable-pty as `=0.8.1`; rmcp minor pinned as `~3.4`).
- Lint: `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`.
- Typecheck: `cargo check --workspace --all-targets`.
- Dependency policy: `cargo deny check` (licences, bans on C-building crates such as `cc`, `libsqlite3-sys` and `openssl-sys`, and a ban on `tokio` in `viola-core`, `viola-pty`, `viola-channel` without its feature, `viola-state` and `viola-agent-claude`, evaluated for the Windows, macOS and Linux target triples so `cfg`-gated dependencies are covered).
- Boundary review: `cargo modules` graph on demand.
- Install: `cargo install --path .` from the repo root.
- Publishing: every workspace member, including the root `viola` bin, sets `publish = false`. Nothing goes to crates.io in v1.

**Deployment model**
- Local-only. There is no Docker, Compose, Kubernetes or serverless.
- The single `viola` binary is installed on PATH. `viola run <name> -- claude <args>` pins a copy of itself in `~/.viola/bin/<version>-<hash>/`, writes the embedded plugin (pointing at that copy) to `~/.viola/plugin/<version>-<hash>/` and launches the child with `--plugin-dir`. `viola ui` is started by hand and serves 127.0.0.1.

**Crate dependency direction** (enforced by manifests)
- `viola-core` depends on no viola crate. Third-party: nutype (`ViolaName`, `Percent`).
- `viola-pty` depends on no viola crate and knows no agent. Third-party: portable-pty, windows-sys (Windows only).
- `viola-channel` → `viola-core`, interprocess. It has a sync client and server, plus a Tokio client behind the `tokio` feature.
- `viola-state` → `viola-core`, atomic-write-file, notify (tailing), sysinfo (the pid + start-time liveness check, shared by CLI `list`, `mcp` and `ui`).
- `viola-agent-claude` → `viola-core`, `viola-state`, serde_path_to_error (drift reports on external payloads), vt100 (the screen model and signatures behind the readiness gate; `run`'s pump feeds it bytes). This is the only crate that knows Claude payload shapes, including `claude agents --json`.
- `viola-mcp` → `viola-core`, `viola-channel[tokio]`, `viola-state`, `viola-agent-claude`, rmcp, schemars, tokio.
- `viola-ui` → `viola-core`, `viola-state`, `viola-agent-claude`, axum, tower-http, tokio.
- jsonschema, if adopted, is a dev-dependency only. No runtime crate lists it.
- The `viola` root bin → all members, clap, anyhow. It contains the subcommand dispatch, the `run` pump, the wheel and the budget governor, all of which consume only normalised events.

**Project directory structure**
```
viola/
├── Cargo.toml                  # [package] viola (bin) + [workspace] members = ["crates/*"]
├── Cargo.lock
├── rust-toolchain.toml         # channel = "stable", components = ["rustfmt", "clippy"]
├── deny.toml                   # cargo-deny: licences, C-crate bans, tokio bans
├── .gitignore
├── plugin/                     # embedded via include_str!, written out by `viola run`
│   ├── .claude-plugin/plugin.json
│   ├── hooks/hooks.json        # exec-form commands, placeholder for the pinned bin copy
│   └── .mcp.json
├── src/                        # the `viola` bin: anyhow edge only
│   ├── main.rs                 # clap 4.6.7 dispatch
│   ├── cmd/                    # one module per subcommand: run, send, wait, last, list,
│   │                           #   answer, hook, mcp, ui, verify, pause, release, link, unlink, plugin
│   └── run/                    # PTY pump, wheel, budget governor, readiness gate wiring
├── crates/
│   ├── viola-core/             # normalised events, RefusalReason, ViolaName, Percent, `v` constants
│   ├── viola-pty/              # pty seam over portable-pty =0.8.1 (+ windows-sys kill fallback)
│   ├── viola-channel/          # JSON-RPC 2.0 ndjson over interprocess local sockets
│   ├── viola-state/            # ndjson logs, atomic snapshots, File::lock, torn-line healing, tailing
│   ├── viola-agent-claude/     # hook parsing, dialog mapping, R8 strip, shim resolution,
│   │                           #   capability ledger, screen signatures, statusline parsing
│   ├── viola-mcp/              # rmcp 3.4.1 stdio server, thin adapter over viola-channel
│   └── viola-ui/               # axum 0.8.9 GET routes + SSE, Host allowlist
├── fixtures/
│   └── claude/<cli-version>/   # hook-payload fixtures recorded by `viola verify`
├── .github/
│   └── workflows/ci.yml        # 3-OS matrix
├── refs/                       # brief and prior-art survey (arch input)
└── .andromeda/                 # pipeline runs and cache
```

**CI/CD approach**
- GitHub Actions, one workflow `ci.yml`, triggered on push and pull request, with matrix `os: [windows-2025, macos-latest, ubuntu-latest]` on native runners.
- Setup steps: `dtolnay/rust-toolchain@stable` with rustfmt and clippy, then `Swatinem/rust-cache@v2.9.2`.
- Jobs per OS:
  1. `cargo fmt --all --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo check -p viola-core -p viola-pty -p viola-channel -p viola-state -p viola-agent-claude` (proves the sync crates, including the `hook` path, compile on each OS without `viola-channel`'s `tokio` feature; the ban itself is enforced by job 4)
  4. `cargo deny check` (once, on ubuntu)
  5. the workspace test suite against the fake agent, replaying `fixtures/claude/*`
  6. `cargo build --release`
- The real `claude` CLI and `viola verify` run only locally, never in CI.
- There is no deploy stage in v1. The v1.x release path adds a dist 0.33.0-generated release workflow with cargo-auditable 0.7.6.

## Cross-cutting Patterns

- **Config management.** Precedence: CLI flags override `<viola home>/config.json`, which overrides built-in defaults.
  - Viola home: a global `--home <dir>` flag on every subcommand overrides the default `<user home>/.viola/`. Resolution order: `--home`, then the grandparent of `VIOLA_DIR` when it is set, then the default. `viola run --home <dir>` passes the home to the child only through `VIOLA_DIR`, so the pinned copy, plugin folder, endpoint hash, logs, `budget.json` and ledger stamps all follow it. This is the isolation point for tests: each test runs in its own home, and that home's `ledger/stamps.json` can stamp the fake agent's reported version.
  - `config.json` is parsed tolerantly (unknown keys skipped and counted) and carries `v`.
  - Settings: budget thresholds as `Percent` (defaults `five_hour` 90, `seven_day` 85) and the GUI port (default 47319).
  - There are no secrets in v1, and environment variables are not a configuration channel. The `VIOLA_*` variables are wrapper-to-child plumbing set by `viola run` only.
- **Development Style signal:** agent-driven.
- **Identity by instance, never by location.** Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` (or an explicit `ViolaName` argument), never through the working directory. Two sessions in one folder never share records.
- **Fail open toward the human.** Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human. Any failure to confirm delivery is reported as `not-delivered`, never retried blindly. The one exception is a local command on the ledger whose post-condition is "none" or not yet measured on this CLI version. It is reported as `ok` with `confirmed: false` and detail `unconfirmable`.
- **Mixed-version tolerance.** Every frame, event, snapshot and config carries `v`, and channel frames also carry the sender's `CARGO_PKG_VERSION`. Readers skip unknown kinds and fields and surface the counts. A newer request than the wrapper supports gets a `-32602` naming the wrapper's version.
- **Capability ledger as the single gate for CLI-specific behaviour.** Any new dependence on an undocumented `claude` behaviour is added as a ledger row in `viola-agent-claude`, with a `viola verify` probe that has a post-condition check. It is never hard-coded as an unconditional assumption.
- **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` and `viola-ui` may build a runtime. CLI `wait` is a blocking read on `viola-channel`'s sync client.
- **Crash-safe disk writes.** Append-only logs use one `write` per line. Mutable files are replaced atomically. Every guarded file has its own `.lock` sibling. Readers always tolerate a torn last line.
- **Diagnostic output channels.** `hook` writes only its decision body to stdout and nothing to stderr (Hook Contract). While the child runs, `run` writes nothing to the terminal except the child's own output, because the terminal carries the child's screen. Its diagnostics go to the instance's `diagnostics/`. `mcp` writes only MCP frames to stdout. Its diagnostics go to stderr, or to the instance's `diagnostics/` when `VIOLA_DIR` is set. `ui` and short-lived CLI verbs may use stderr. The logger and the format are owned by obs.
- **Cross-platform discipline.**
  - Every path written into plugin files uses forward slashes.
  - Nothing shells out through `sh`, `bash` or `cmd`, except `hook statusline` running the user's own statusline command (see Hook contract).
  - Child processes are spawned directly.
  - Endpoint names are hashed and short.
  - Every OS-specific branch compiles and is tested on its CI runner.
- **Untrusted upstream text.** A driven session's `Stop` text and dialog payloads are content, never commands to viola. viola forwards them as escaped strings and never interprets them (R1).
- **Local endpoint trust boundary.** Any process that can open an instance's endpoint can `send` prompts and `answer` dialogs, including PermissionRequest. v1 trusts every process of the same OS user and no other. Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user, and the GUI stays read-only. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.

## Project Intent

- **Product type.** Hybrid local developer tool made of three parts:
  - a native cross-platform CLI binary;
  - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
  - a minimal view-only local web GUI served by the same binary.
- **Core function.** Lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment.
- **Growth model.** Modular monolith: one binary with compiler-enforced internal crates (`pty · channel · state · agent-claude · mcp · ui` around `core`), and no daemon.
- **How new functionality is added.**
  - New capabilities arrive as new subcommands or channel verbs, exposed consistently on the CLI (`--json`) and, where a driver needs them, as MCP tools.
  - New Claude behaviours arrive as capability-ledger rows in `viola-agent-claude`.
  - New GUI views arrive as GET routes.
  - State-changing GUI routes wait for the v1.x brake and its auth.
  - A second driven agent (for example Codex) arrives as a sibling `viola-agent-<name>` crate, at which point the adapter trait is extracted into `viola-core`.
- **Template patterns.** None. This is a single-product repository, not a fleet.
- **Scale path.**
  - v1 is personal (Windows live, macOS and Linux CI-tested).
  - v1.x adds the GUI brake and then public distribution (dist 0.33.0 releases, signing, winget/Homebrew/Scoop, a marketplace entry, self_update 1.3.0).
  - A phone or remote view comes later, behind authentication.

## Inherited Defaults

- Framework: Rust stable (MSRV 1.89, edition 2024). A sync std-thread core; Tokio 1.53.1 only in `mcp`/`ui`; axum 0.8.9 for HTTP/SSE; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.
- Database: none. ndjson append logs + atomic JSON snapshots (atomic-write-file 0.3.1) + std `File::lock`, serde_json 1.0.151.
- IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- API style: view-only GET under `/api/` + SSE `/api/events` on `127.0.0.1:47319`; RFC 9457 Problem Details for HTTP errors; typed `RefusalReason` in `result.refusal`.
- Errors: thiserror 2.0.20 per crate, anyhow 1.0.104 only in the root bin; typed CLI exit codes; `viola hook` always exits 0.
- Validation: tolerant serde for external payloads, versioned-tolerant own formats, nutype 0.8.0 newtypes.
- Timestamps: chrono 0.4.45, RFC 3339 UTC with milliseconds and `Z`, field `ts` / `*_at`.
- Agent isolation: Claude-specific shapes only in `viola-agent-claude`, and every undocumented behaviour is a capability-ledger row.
- Deployment: local `cargo install --path .`, plugin embedded in the binary and loaded with `--plugin-dir`; no containers, no hosting.
- CI: GitHub Actions on windows-2025 / macos-latest / ubuntu-latest against the fake agent.
- Development Style: agent-driven.
- Publishability: every workspace crate, including the root `viola` bin, sets `publish = false`. Nothing is published to crates.io in v1.

## Existing Scopes

None — new project.

[Iteration 1] [substantive] Pinned copies are never deleted automatically. Snapshots record the pinned path, and cleanup is an open item.
[Iteration 2] [substantive] Viola-home resolution order is stated: `--home`, then the `VIOLA_DIR` grandparent, then the default.
[Iteration 3] [substantive] Added diagnostic output channel requirements for `run`, `mcp`, `ui` and the CLI verbs, with the logger and format left to obs.
[Iteration 4] [substantive] The Local endpoint trust boundary bullet no longer refers to a removed `$TMPDIR` fallback.
[Iteration 5] [substantive] clap and anyhow placed in the root `viola` bin in "Crate dependency direction".
