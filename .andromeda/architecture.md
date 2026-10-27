## Design Philosophy

- **Mechanism, not policy.** viola carries input, answers dialogs when told to, logs everything and holds the wheel, but it never decides what to answer. Andromeda's decision rights and any other driver-side policy stay outside the binary, so every feature has to be expressible as a general bridge verb.
- **Measured, never assumed.** Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger. On a CLI build nobody has verified, viola degrades to transport-only (the readiness gate is partial there, while a verified build runs the full gate on the compiled signatures: a poisoned screen model or a screen not quiet within the maximum wait still refuses, and no signature is read), and every send is confirmed after the fact (a matching `prompt-submitted` or, for a local command on the ledger, its measured post-condition; otherwise `not-delivered`), never presumed. The one exception is a ledger-listed local command whose post-condition is "none" or not yet measured on this CLI version: it returns `ok` with `confirmed: false` and detail `unconfirmable`.
- **The human always wins the wheel.** Human keystrokes are never blocked, refused or delayed past the current atomic paste. Automation is refused with a typed reason (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`) instead of competing with the human.
- **No daemon; the disk is the shared truth.** Every wrapper owns its own local-socket endpoint, whether it was started by `viola run` or by a `viola revive` whose preflight passed (both go through the one start path). Every other verb is a separate process that owns no endpoint (short-lived, except the long-running `mcp`; `viola revive --list` and a refused revive are among them), and the state all processes share lives in ndjson logs and atomic snapshots that survive a crash on either side.
- **Cross-platform from the first commit, Windows first.** Windows is the first target. The Windows, macOS and Linux paths build and pass CI on every commit against the fake agent, so no design choice may depend on tmux, a shell or POSIX-only behaviour. The first live run was on the Linux dev host, by the founder's ruling R-L1 (2026-10-07, relayed by the overseer), at chunk 2026-10-08-first-live-test-and-self-drive (`claude` 2.1.287 in foot 1.28.0 under Hyprland 0.56.2); Windows live behaviour stays proven on the CI runner under the fake agent until an interactive Windows host exists.

## Stack and Technologies

Detail: .andromeda/architecture-detail/stack-and-technologies.md — read whole.

One native Rust binary `viola` / `viola.exe` (Rust 1.98.1, workspace floor 1.96, edition 2024): std threads with Tokio built only inside `mcp`; clap for the CLI; portable-pty `=0.8.1` behind viola's own `pty` seam; vt100 for readiness; interprocess local sockets (JSON-RPC 2.0 over ndjson); ndjson append logs + atomically replaced JSON snapshots, no database, broker, container or hosting; rmcp for the stdio MCP server; GitHub Actions on three OSes against the fake agent.

## Established Decisions

Detail: .andromeda/architecture-detail/established-decisions.md — read whole.

- **[Scale / Product] Personal v1, cross-platform from day one.** The founder runs v1 on their own subscription, with no accounts and no hosting. It is still built and CI-tested on Windows, macOS and Linux from the first commit, and the on-disk state is parsed defensively. Rationale: retrofitting cross-platform support onto a PTY/IPC tool costs far more than carrying it from the start, and the later public version (individual Claude Code subscribers) must not need a rewrite.
- **[Scope Boundary] General bridge, nothing Andromeda-specific inside.** Rationale: R4 (mechanism, not policy). Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair (Pulse, Conductor, viola itself).
- **[Agent Coverage] Claude-first, with one concrete `viola-agent-claude` crate and no adapter trait yet.** The PTY, channel, wheel and state layers speak only viola's normalised events. Hook JSON parsing, the dialog answer mapping (S3/S7/S8), the R8 identity strip, npm-shim → `claude.exe` resolution and the Claude-specific half of the CLI version gate (`parse_version`, the capability-ledger rows, the stamp merge and the `verified` verdict, the fixture scrub) live only in `viola-agent-claude`, which stays pure (no I/O, no `viola-state` dependency). The gate's bounded `--version` spawn and stamps verdict are the root bin's `run::version_gate`, and the stamps file I/O is `viola_state::stamps` (as landed by chunk 2026-09-28-capability-ledger-and-viola-verify). Rationale: a trait designed before a second agent exists would encode guesses. Isolating the agent in one crate keeps the later extraction mechanical.

- **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` (KEYSTONE).** CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes. Rationale: portable-pty's handles are blocking, so the pump needs threads either way; `hook` spawns once per hook event and must not pay for runtime startup; rmcp and interprocess-async require Tokio, so it is confined to the subcommand that uses them.

- **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).** Child exit is detected on the process handle, never on EOF; on Windows x64 the ConPTY is sideloaded, failing open to the inbox one, and every `viola` process restricts its DLL search to System32; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself. Rationale: 0.8.1 is spike-proven; 0.9.0 has a Windows garbage-read bug.

- OPEN — whether the real `claude` loses a key typed right after a resize, and the sideloaded ConPTY's DA1 start stall with no terminal on viola's stdio · owner: "Windows-only live measurements"

- **[Screen Model] vt100 0.16.2 as a pre-send readiness gate only.** After `turn-ended` the gate waits for a quiet screen, then requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; a screen not ready at the bound is refused `not-delivered` / `input-not-ready` with nothing typed, and the passthrough never waits on the model. Rationale: CLI-native modals bypass every hook, and the screen lags the hooks.

- **[Delivery Confirmation] Every `send` is confirmed.** No matching `prompt-submitted` within the compiled confirmation window (never in `config.json`) is `not-delivered` / `no-prompt-submitted`; a ledger-listed local command is confirmed by its measured post-condition, or `unconfirmable` when it has none or the CLI unverified, and nothing else is ever unconfirmable; the ledger decides, never a leading slash. Rationale: a turn boundary per the hooks does not prove the input box is ready.

- A `send` types its text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters, and a typed text left empty is refused `not-delivered` / `empty-text`, nothing issued ([Delivery Confirmation]).

- **[Database / State Store] ndjson append logs + atomic JSON snapshots + std `File::lock` on separate `.lock` files (KEYSTONE).** Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log. Rationale: the trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected, and lock files are separate because append + exclusive lock fails on Windows.

- **[Snapshot writer] tempfile 3.27.0 `persist` and `persist_noclobber` through the shared helpers of `viola_state::fs`.** `viola_state::fs::replace_private` sets the temp file's mode before any byte is written, then writes, `sync_all`s and `persist`s, a failure leaving the target as it was; the pinned copy is never replaced: its absent-copy write is `place_private_new`, a no-clobber placement of a closed temp file; no call site carries its own temp + rename code. Rationale: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.

- **[Message Broker / IPC] interprocess 2.4.4 `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam.** `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls; endpoint names are a short hash of the instance; on Windows the client connects only through the SQOS Identification open, never interprocess's default connect. Rationale: one API over named pipes and Unix sockets, sync and Tokio flavours, and no daemon.

- **[API Style] JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde (KEYSTONE).** Requests carry an `id` (hook dialogs, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`). `list` is a caller-side read of the disk and `claude agents --json`, not a channel request (as landed, chunk 2026-10-16-unwrapped-sessions-on-the-board, it reads the disk through `board::read`, then makes one `claude agents --json` read joined into the board). Hook events are notifications without one. Refusals travel in `result.refusal`. `error` is reserved for protocol faults. Every `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. Rationale: it has the same shape as MCP, so `mcp` is a thin adapter. jsonrpsee is Tokio-bound and would drag a runtime into `hook`. Mixed binary versions are a measured fact, so every frame self-describes.

- **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.** Refusals are `isError: true` tool results mapped from `result.refusal`; `pause`, `link` and `unlink` are CLI-only too, as human controls over the wheel and the link topology. Its stdin is read under a `MAX_FRAME` line cap, each line's newline counted: a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it; the bound sits in the reader viola hands rmcp, because rmcp 3.4.1's stdio transport reads each line unbounded. Rationale: the official SDK, its minor pinned; `release` stays off the driver's tool surface, so a driver is never offered a way to hand the wheel back to automation — an affordance, not an enforcement.

- **[Hook Transport] Exec-form `command` hooks: `"command": "<pinned viola copy>", "args": ["hook", "<event>"]`.** Tiers: sync with a decision (PreToolUse `AskUserQuestion|ExitPlanMode`, PermissionRequest); the sync spine, no reply body, within viola's spine deadline (SessionStart, UserPromptSubmit, Stop; SessionEnd inside its 1.5 s budget, never raised); async, printing nothing. A lost endpoint loses the event, not the turn. Rationale: exec form skips Git Bash path rewriting and shell startup.

- **[Hook Contract] Hooks fail open.** Exit 0 with a JSON body is a decision, with no body no decision; exit 2 is never produced, and `hook` never writes to stderr. A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60–3600 s, default 60; ceiling PROVISIONAL, owner: "Version done-check"; its Windows leg, owner: "Windows-only live measurements"), the dialog then rendering for the human. Rationale: exit 2 blocks Claude Code, and a viola bug must never block the session.

- **[Human Takeover / Wheel] Byte-source claim + atomic send window + after-the-fact confirmation, explicit release.** The wheel starts with `driver` at every start; any human editing key since the last turn boundary moves it to the human and `send` is refused `human-typing`; only a closed non-editing list never counts; a `send` mid-turn or mid-send is refused, never queued; the wheel returns only through `viola release`. Rationale: `viola run` owns the real stdin, so it knows byte provenance.

- **[Links] A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`. `viola unlink` removes it.** Links are events in the log, the snapshot's link set derived by replaying them; the implicit link and the explicit verbs are built, the verbs checking the driven instance's files before any log and its wrapper serving `link` / `unlink`. Rationale: the log stays the single source of truth for the link set.

- **[Budget Governor] Soft stop at the turn boundary.** Once a configured threshold is crossed (defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %), automated `send` is refused `budget-paused`; human keys are never blocked and a running turn finishes; the pause lifts when `resets_at` passes or on `viola release --budget`; the gate never polls and fails open. Rationale: the per-model weekly window is hidden, so thresholds are set low; the user's global statusline config is never touched.

- **[CLI Version Compatibility] Per-behaviour capability ledger + transport-only degrade + `viola verify` re-stamp.** A version is verified only when its stamp holds every relied ledger row `pass`; on an unverified one viola still types, runs the wheel and emits events, but withholds dialog answers (`unverified-cli`); no `viola verify` run writes a byte into a CLI dialog. Rationale: the docs lag the installed build, and several load-bearing behaviours are measured, not documented.

- **[Real-time Strategy]** The frontend is deferred to 0.2.0; its technology is not chosen.

- **[Session Liveness] Heartbeat file primary; pid + process start time decide `gone`, and `claude agents --json` only enriches it.** A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start; a revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated. Rationale: the prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.

- **[GUI Control Scope]** The frontend is deferred to 0.2.0; its technology is not chosen. As the 0.2.0 frontend's starting intent, the page shows each session row (wrapped, or read-only unwrapped), its liveness (`live` · `stale`), idle/busy status, wheel holder and any pending dialog. It also shows the budget reading and its age, the link set, the `skipped` counts, and a live event feed.
- **[Timestamps] chrono 0.4.45, RFC 3339 UTC, millisecond precision everywhere viola writes.** External `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error. Rationale: chrono is already in the tree through rmcp, and external clocks must never break parsing.
- **[Validation] Two regimes.** External payloads (hook JSON, `claude agents --json`, statusline) are parsed tolerantly: no `deny_unknown_fields`, `#[serde(default)]` / `Option<T>`, the raw `serde_json::Value` kept for the fixture recorder, and serde_path_to_error for drift reports. viola's own formats are versioned-tolerant: every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. nutype newtypes `ViolaName`, `Percent` and `DialogHold`. No garde or validator. Rationale: the CLI changes between builds, and parsing must never break when the ledger is the thing that decides behaviour. `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.

- **[Error Handling] thiserror 2.0.20, one enum per workspace crate; anyhow 1.0.104 only in the `viola` bin.** One ratified exception, `viola-pty`'s hand-written `PtyError`, and three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, the rule standing; `RefusalReason` lives in `viola-core`. Rationale: callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr.

- **[Module Boundaries] Cargo workspace, flat `crates/` layout, with the root `Cargo.toml` as both the `viola` bin package and `[workspace]`.** Members `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp` and the test-only `viola-e2e`; `fuzz/` is its own workspace; among the product crates only `viola-mcp` enables `tokio`. Rationale: the compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.

- **[Deployment / Distribution] `cargo install --path .`, with the plugin compiled into the binary (KEYSTONE).** At start `viola run` pins its own exe under `<viola home>/bin/<version>-<hash>/`, refusing a copy that fails its re-hash, and rewrites the plugin for that copy, loaded with `--plugin-dir`; hooks and MCP never search PATH. Rationale: the binary and the plugin can no longer drift apart; the content hash keys the copy because the version alone collides across rebuilds; one tree can build two files whose loaded sections are equal and so key two copies, its cause not explained and acting on it owed to the working-route entry "Version done-check".

- **[Plugin Scope] The plugin loads only into wrapped sessions via `--plugin-dir`.** `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver; the installed plugin carries only `.mcp.json` and no hooks, its `command` an absolute path, never a PATH lookup. Rationale: unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola; `viola verify`'s transient unwrapped children are the exceptions.

- **[CLI Conventions] Human text by default, `--json` for agents, a typed exit code per refusal. Prompt text comes from stdin or `--file`, never from a leading-slash argument.** viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths, and agents need machine-parseable refusals.

- **[CI/CD] GitHub Actions native matrix on three OSes, tests against the fake agent, the real CLI only in local live tests.** The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`). viola resolves the child program to an absolute path itself (on Windows a PATH search over PATHEXT only, never the bare name) and spawns that. A `.cmd`/`.bat` child is refused (exit 1, `batch-script-child`), except the `claude` npm shim: `claude.cmd`/`claude.bat` resolves to `<dir>/node_modules/@anthropic-ai/claude-code/bin/claude.exe` when that file exists, and the shim is never read. Each test runs in its own viola home (`--home`, see Config management), so parallel tests never share an endpoint, log or `budget.json`. Rationale: no tokens spent and no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
- **[Hosting] None.** Rationale: local-only v1. Docker, serverless and PaaS do not apply.
- **[Mobile] N/A in v1.** Rationale: a phone view comes in a later version, behind authentication.
- **[AI/ML] N/A.** Rationale: D2, subscription only through the unmodified `claude` CLI. The binary makes no model API calls.
- **[Primary Keys] No database keys.** Instances are identified by `ViolaName`, endpoints by its short hash. Rationale: identity is by instance, never by location (R5).
- **[Naming] Product, binary and identifiers use the `viola` prefix.** Env vars are `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, and the newtype is `ViolaName`. Two groups of ratified exceptions: the test seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, each read only by a `viola` built with the test-only `fake-agent` feature and never present in a release build; and the terminal's own colour names `NO_COLOR`, `TERM` and `COLORTERM`, read (never set) by every build's human output to decide only its SGR bytes (the founder's own word, chunk 2026-10-17-cli-output-discipline) (§Occupied Resources → Environment variables). `BRIDGE_NAME` and `BridgeName` in earlier artifacts are superseded. Rationale: one name everywhere removes the need to translate between prototype and product.
- **[Deferred] Deferred to specialists and later versions:** the frontend (deferred to 0.2.0; its technology is not chosen); auth generally (security, v1.x); the test framework, fake-agent harness and `viola verify` probe design (tests); logging and observability (obs); self-update, signing, winget/Homebrew/Scoop and a marketplace entry (later version).

## Conventions

Detail: .andromeda/architecture-detail/conventions.md — read whole.

**Interfaces and versioning**
- **Four interfaces from one binary:** CLI verbs, MCP tools (stdio JSON-RPC 2.0), the hook stdin/stdout contract (owned by Claude Code), and the wrapper channel (JSON-RPC 2.0 over ndjson on a local socket). There is no REST write surface, GraphQL, gRPC or tRPC.
- **Protocol versioning:** an integer `v` starting at `1` in every channel `params`, ndjson event line and snapshot. Adding a field or event kind is additive and needs no bump. Removing or changing a field's meaning bumps `v`. A wrapper receiving `params.v` higher than it supports answers `-32602` with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
- **Sender version:** every channel `params` carries `"sender": "<CARGO_PKG_VERSION>"`. A snapshot's `writer` holds the same value for the binary that wrote it.
- **Channel methods:** lower-case, dot-namespaced for hook traffic. Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method. Notifications: `hook.event`. JSON-RPC ids are integers, monotonic per connection.

**Error handling schema**
- **Channel refusal (a normal outcome, not an error):** `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
- **Channel success:** `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
- **Channel protocol fault:** `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}` with codes `-32700` parse error, `-32600` invalid request, `-32601` unknown method, `-32602` bad params / unsupported `v`, `-32603` internal error.

- **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown` (`#[serde(other)]`), each `detail` from a closed set per reason, extended only in `viola-core`, an unknown one opaque text to a reader; of several refusals only the first is returned, in a fixed order per verb, `control-character` first (any C0 but LF, CR and TAB, DEL or C1: refused, never stripped). `unconfirmable` is no refusal detail, only an `ok` payload's.

- **CLI exit codes:** `0` ok · `1` internal error, and the start, revive-preflight and verify refusals (a refused home-tree check of `run`, `viola revive`'s start or `verify` among them) · `2` usage error · `10` `human-typing` · `11` `budget-paused` · `12` `unverified-cli` · `13` `not-delivered` · `14` `unknown` refusal · `20` protocol fault from the wrapper · `21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`).

- **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.

- **MCP:** a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`; JSON-RPC errors are only the MCP connection's own faults: an unknown tool name (`-32602`) and an internal handler fault (`-32603`).

- **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`), each a `thiserror` enum except `PtyError`, whose `Display`/`Error` are hand-written with fixed messages ([Error Handling]; `viola-agent-claude`'s pre-existing `Refusal` beside `AgentError`, `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences named there). nutype validation errors fold into the owning crate's enum. `anyhow` appears only in the root bin crate.

**Naming patterns**
- **Crates:** `viola-<area>`, kebab-case. Rust modules and files are snake_case, types are UpperCamelCase, functions and variables are snake_case, and constants are SCREAMING_SNAKE_CASE (standard Rust).
- **JSON field names:** snake_case in every viola format (`last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them).
- **Enum string values on the wire:** kebab-case (`#[serde(rename_all = "kebab-case")]`), covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
- **Normalised event kinds (`viola-core`):** `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`, `wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `dialog-expired`. New kinds are added only in `viola-core`, never with Claude-specific names.
- **Hook → kind map (in `viola-agent-claude`):**
  - SessionStart → `session-start`. `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
  - UserPromptSubmit → `prompt-submitted`.
  - PreToolUse `AskUserQuestion` → `question`.
  - PreToolUse `ExitPlanMode` → `plan`.
  - PermissionRequest → `permission`, except a PermissionRequest that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
  - Stop → `turn-ended`.
  - SessionEnd → `session-end`.
  - Notification, PostToolUse and PostToolUseFailure → `activity`. It is log-only and never wakes `wait`.
  - `hook statusline` emits no event.
- **Environment variables:** `VIOLA_` prefix, SCREAMING_SNAKE_CASE, for every variable the product reads or sets. Test-harness-only variables use the `AGENT_RUN_` prefix and are never read by the `viola` binary. Two of the ratified exceptions are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s; operator ruling, 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`; the founder's ratification, 2026-09-28): each is read only under `cfg(feature = "fake-agent")`, is absent from release builds, and carries neither prefix because it is neither product plumbing nor harness-only. A third ratified exception, in every build: `NO_COLOR`, `TERM` and `COLORTERM`, the terminal's own cross-tool names, which the product reads (never sets) only to decide whether and how deep its human output's SGR bytes are written (the founder's own word, chunk 2026-10-17-cli-output-discipline; §Occupied Resources → Environment variables).
- **On-disk files:** lower-case with `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file (`<name>.lock`).
- **CLI:** flags are kebab-case long options (`--json`, `--file`, `--budget`, and the global `--home`). Subcommands are single lower-case words. Verb arguments mirror the channel `params`: `viola send <target>` (text from stdin or `--file`), `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`, `viola last <target>`, `viola answer <target> <dialog_id>` (the `response` object as JSON from stdin or `--file`), `viola pause <target>`, `viola release <target> [--budget]`, `viola link <driver> <driven>`, `viola unlink <driver> <driven>`. `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments) send no channel frame. Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.

**Data model conventions**
- **No database, so no UUID or serial keys.** The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, enforced by nutype.
- **Endpoint name:** `viola-<h>`, where `<h>` is the first 12 hex characters of an FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute viola home path. It is hand-written in `viola-channel` and recorded in the instance snapshot. std `DefaultHasher` is forbidden because its output is not stable across Rust releases, and mixed binary versions must agree on the name.
- **Timestamps:** field `ts` on every event and frame, and `*_at` for other instants (`started_at`, `resets_at`, `written_at`). Always RFC 3339 UTC with milliseconds and a `Z` suffix, for example `2026-09-23T19:43:58.123Z` (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`).
- **Optional fields:** Rust `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. viola omits absent fields on write, and readers treat missing and `null` the same.
- **Unparseable external values** become the string `"unknown"` in viola's own records, never an error. This covers `resets_at`, `used_percentage` and a missing statusline window.
- **Percentages:** `Percent` newtype, 0–100, serialised as a JSON number. An external budget reading's `used_percentage` is not that newtype: it is a `Reading<f64>` in `viola_core::BudgetWindow`, kept only from 0 to 100 and otherwise the word `"unknown"`.
- **ndjson line discipline:** one complete JSON object plus `\n` per single `write` call. Multi-line text travels as an escaped JSON string on one line.

## Standard Contracts

Detail: .andromeda/architecture-detail/standard-contracts.md — read whole.

**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.

**`list` payload** (the `ok` of CLI `list --json`, and MCP `list`'s `structuredContent`). There is no pagination: lists are personal-scale and returned whole. `skipped` counts records a reader could not interpret, so the board can show them.

- `items` entry: `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`; `liveness` `live` or `stale`, gone instances omitted; `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`; the payload also carries a top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`.

**ndjson event line** (`events.ndjson`, per instance)
```json
{"v":1,"ts":"2026-09-23T19:43:58.123Z","instance":"builder","kind":"turn-ended","source":"hook","data":{}}
```
`source` ∈ `hook` · `wrapper` · `cli`. `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.

**Snapshot envelope** (atomically replaced JSON): `{v, written_at, writer, data}`. A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.
- The replay's product readers are `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`, `viola_state::liveness::live_endpoint` (the CLI frame verbs, `viola link` / `viola unlink` and `viola mcp`'s frame tools), `viola hook`'s connect, `hook statusline` and `run`'s collision check; no product code under `src/` calls `read_snapshot`. A replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `live_endpoint` and the hook's connect read the instance as unreachable, `hook statusline` runs nothing, and the collision check judges liveness from the heartbeat alone (a beat within `STALE_AFTER` refuses `already-live`).

**Ledger stamps envelope** (`ledger/stamps.json`, atomically replaced JSON, written only by `viola verify` through `viola_state::stamps::update_stamps`): per CLI version, each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none; a version is verified only when every row of `LedgerRow::relied()` reads `pass`; a malformed envelope makes `run`'s gate record `cli_verified:false`.

**Instance snapshot** (`instances/<ViolaName>/snapshot.json`, written only by that instance's wrapper, rewritten on every change to a field below): `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`. `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`. `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it. The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
- `pid` and `started_at` identify the wrapper process for the liveness check and, on Linux and Windows, for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame (macOS reports no peer pid, the recorded gap).
- `child_pid` is the pid of the spawned agent child, and `agent_session_id` is the one from the newest `session-start` event. Both exist only to join `claude agents --json` rows to this instance (see Session liveness).
- `cwd` is the directory the start spawned its child in. It is written into the first snapshot when it is valid UTF-8 and omitted otherwise (an additive field; `v` stays 1). No event carries it. It is a host path that lives only in this 0600 file, on no log line, error body or stdout, and its one reader is `viola revive`, which uses it as the revived child's spawn directory and for nothing else.
- `statusline_command` is the user's statusline command that `run` read from the named source at start (Hook contract). It is omitted when the source gave none (an additive field; `v` stays 1). No event carries it. `hook statusline` reads it and never changes the snapshot.
- `cli_version` and `cli_verified` come from `run`'s version gate.
- `pending_dialog` is `{dialog_id, kind}` while a `hook.dialog` is open. It is written before the dialog's event line is appended, and written back to none when that append fails, so a reader that sees the line already sees it. It is removed when the dialog is answered, expires or is answered `null` on a wheel move. It is the only source of `dialog_pending` in `list`. It is not rebuilt from the log, so a reader that falls back to replay reports `dialog_pending: false`.

- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`, `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and the set follows the log: a failed append leaves it as it was.

**Wrapper channel frames** (JSON-RPC 2.0, one per line)
```json
{"jsonrpc":"2.0","id":7,"method":"send","params":{"v":1,"sender":"0.1.0","text":"line one\nline two"}}
{"jsonrpc":"2.0","id":7,"result":{"ok":{"submitted_at":"2026-09-23T19:43:59.004Z","cursor":48213}}}
{"jsonrpc":"2.0","id":8,"result":{"ok":{"confirmed":false,"detail":"unconfirmable","cursor":48213}}}
{"jsonrpc":"2.0","id":7,"result":{"refusal":"not-delivered","detail":"input-not-ready"}}
{"jsonrpc":"2.0","id":7,"error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}}
{"jsonrpc":"2.0","method":"hook.event","params":{"v":1,"sender":"0.1.0","ts":"2026-09-23T19:43:58.123Z","event":{"kind":"turn-ended","data":{"last_assistant_message":null,"background_tasks":null}}}}
```

A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed. Only `hook.event` may be id-less: any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`. A client may add `conn` (obs-plan D-10) to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.

**Channel methods: `params` → `ok` payload.** Every `params` also carries `v` and `sender`. `from` is the caller's `VIOLA_NAME` when set, and is omitted otherwise. The frame envelope does not type `from`: each method that reads it (`send`, `wait`, `last`, `answer`, `pause`) takes it absent, `null` or a valid `ViolaName` and answers anything else `-32602` `invalid params` with `data: null`, never `-32600`; `release` refuses any string `from` (below).

- `send` `{text, from?}` → `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste. A closed budget gate refuses it `budget-paused` before `send-issued`, nothing typed; a `from` naming another instance records the link before `send-issued`.

- `wait` `{after?, timeout_ms?, from?}` → `{event:<ndjson event line>, cursor}`, or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call; a pending dialog comes first unless `after` is past its line; only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake it, and a parked `wait` never polls the file.

- `last` `{from?}` → `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`. The value is the `WaitFeed`'s newest turn, updated under the same lock hold as the append, so a caller who has seen a `turn-ended` line on disk always reads that turn; it is rebuilt once from the whole `events.ndjson` (`WaitFeed::rebuild`) before `server.serve` at `run` start.
- `answer` `{dialog_id, from?, response}` → `{}`. `response` is normalised per dialog kind:
  - `question`: `{answers:{<question>:<text>}, annotations?}`
  - `permission`: `{behavior:"allow"|"deny", message?}` — no suggestion slot: an offered `permission_suggestions` entry cannot be picked, a deliberate v1 limit (the founder's live ruling, 2026-10-04)
  - `plan`: `{behavior:"approve"|"revise", message?}`
  - As landed (chunk 2026-10-10-session-links): once every refusal has passed and the response's kind matched the pending dialog, and before the answer is taken, an `answer` whose `from` is a name other than the wrapper's own records the link the same way as a `send` ([Links]). A link that cannot be recorded answers `-32603`, the answer is not taken and the dialog stays held. A refused answer, one with no `from` and one under the wrapper's own name record nothing.
- `pause` `{from?}` → `{wheel:"human"}` (the wheel `human` / `manual-pause`).
- `release` `{budget?:bool, from?}` → `{wheel:<holder>, budget_paused:bool, override_until?}`. A string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` (`viola_channel::ProtocolError::ReleaseFromDriver`) plus one `release-from-driver` obs line; a `from` of another type, or a `budget` that is not a bool (`null` included), is `-32602` with `data: null`. `budget:true` leaves the wheel where it is and lifts the instance's gate: the answer is `{wheel, budget_paused:false, override_until:<the crossed window's resets_at>}` when a pause was lifted and `{wheel, budget_paused:false}` when none stood, and then nothing is recorded. A plain `release` (no `budget`, or `budget:false`) lifts no pause and answers `budget_paused` as the gate's last recorded state (it answered the constant `false` before chunk 2026-10-10-budget-governor); a `release` on a driver-held wheel changes nothing. The string-`from` refusal and the `budget` type check come first.
- `pause` and `release` are answered only after the move's `wheel` record has landed (`-32603` if it did not). A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed (`-32603` if they did not, and the pause stands).
- `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` ([Links]).

- `hook.event`, sent id-less by `hook` (a notification, no reply): `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended; a `prompt-submitted` equal to the in-flight `send`'s typed text is appended with `origin:"driver"`, and only then settles that send.

- `hook.dialog`, sent by `hook` (served since chunk 2026-10-04-dialog-answers-by-dialog-id): `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` → `{dialog_id, response:<object|null>}`. `null` means no decision, so the hook prints nothing. On an unverified CLI the wrapper replies `null` at once. A PermissionRequest that repeats the armed PreToolUse dialog's tool with an equal `tool_input` (question and plan, matched per tool) is that dialog's continuation: it appends no event, takes no new `dialog_id`, and is answered only from an armed `plan` revise (otherwise `null`).

- `list` is not a channel method: the calling process reads `instances/*/` and `budget.json` itself and makes one `claude agents --json` read joined into the board, so `list` needs no live wrapper; its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, and a failed agents read leaves every `status` `unknown`.

**Event `data` per kind** (normalised in `viola-agent-claude`; upstream text is content, never commands, per R1):
- `session-start`: `{cause, agent_session_id}` (see the Hook → kind map).

- `prompt-submitted`: `{text, origin}`. `text` is the submitted prompt as it was typed, the long-paste wrapper and its frame removed and tag escaping reversed; `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.

- `turn-ended`: `{last_assistant_message, background_tasks}`. `last_assistant_message` is a string or `null`, and is the source `last` rebuilds from. `background_tasks` is the Stop payload's `background_tasks` array length, a count only (no element is read or carried), or `null` when the key is absent, `null` or not an array.
- `question`: `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`. Each `question` string is a key in `answer`'s `answers`. `option_details` is parallel to `options`, one per option, each field the option's string or `null` (a missing or non-string value never refuses the dialog); the option text is upstream content, carried into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
- `permission`: `{dialog_id, tool, input}`. `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload.
- `plan`: `{dialog_id, plan}`, with the plan as text.
- `session-end`: `{}`.
- `activity`: `{}`, plus `tool` when the hook names one.
- `link` / `unlink`: `{driver, driven}`.
- `wheel`: `{holder, cause}`, with `holder` `human` · `driver` and `cause` `start` · `human-input` · `manual-pause` · `release`.
- `budget-gate`: `{paused, window?, override_until?}` for the instance's own gate, with `window` `five-hour` · `seven-day`.
- `send-issued`: `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
- `send-confirmed`: `{cursor, confirmed?}`. A confirmed send writes `{cursor}`. An unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (it writes no `send-refused`). The field is additive: no `v` bump, and no product code reads the record.
- `send-refused`: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
- `dialog-expired`: `{dialog_id, dialog_kind}` (`source: wrapper`), appended once when a held dialog's hold ends; it carries no question, plan or tool text.

The wrapper appends `wheel` and `budget-gate` (`source: wrapper`) once at start and on every change, and `send-issued` / `send-confirmed` / `send-refused` (`source: wrapper`) for each send. They are log-only and never wake `wait`. They are how the audit trail and a replay learn the wheel holder, the budget gate and each send's outcome. The wrapper also appends `dialog-expired` (`source: wrapper`, `{dialog_id, dialog_kind}`) once when a held dialog's hold ends; it is the one wrapper-appended kind that wakes `wait`. The driven instance's wrapper also appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance, [Links]) or a served `link` call, and `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes (`LinkSet::remove`). Both are log-only and never wake `wait` either, and they are how the audit trail and a replay learn the link set.

`hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`. Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog. On receipt, the wrapper assigns the `dialog_id` and appends the dialog event (`source: hook`) before it can wake `wait` or reply. It does this even when it replies `null` at once (wheel `human`, unverified CLI, another dialog already pending, or a `question` first raised by PermissionRequest with no matching armed dialog — its body has no ledger row, so a late `answer` is refused `unknown-dialog`), so every dialog appears exactly once in the log; a matched continuation (above) appends nothing, being the same dialog.

**CLI `--json` output** mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code. Outcomes that have no channel `result` use `error`:
- `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
- `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` (exit 20)
- `{"v":1,"error":"<code>","detail":null}` (exit 1): `viola revive --json` refusing its strict-modes check or its preflight, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, written by `client::refused_document` (codes only: no path, pid or id), stderr empty. `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order (the escaped `ts`, the cause word, the session-shaped id; exit 0), or the `no-session` document with exit 1 on an empty chain (chunk 2026-10-17-cli-machine-contract). A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".
- `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (exit 0): `viola plugin install --json`, no path; its refused strict-modes check prints the `instance-unreachable` document (exit 21) through `client::refused_document`.

**MCP refusal result**
```json
{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,
 "structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}
```

**Hook contract** (produced only by `viola-agent-claude`)
- A decision is exit 0 with a `hookSpecificOutput` body. PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`. PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
- No decision is exit 0 with an empty stdout. Async-tier hooks always print nothing.

- `viola hook statusline` is exempt from both rules, because it is the statusline wrapper, not a Claude Code hook: it records the `rate_limits` reading to `budget.json` and runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin (the only shell-out in viola; none on Windows), printing its stdout unchanged; it always exits 0 with empty stderr.

**Session liveness**
- The wrapper touches `<instance dir>/heartbeat` every 1 s. When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age. Otherwise a beat at most 5 s old is `live`, and an older or absent beat is `stale`.

- `claude agents --json` adds `status` (idle/busy) and lists unwrapped sessions as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately. If it is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.

**Communication protocols**
- JSON-RPC 2.0 over ndjson on local sockets for the wrapper channel.
- JSON-RPC 2.0 over stdio for MCP.
- No WebSocket and no polling endpoints in v1.

## Occupied Resources

Detail: .andromeda/architecture-detail/occupied-resources.md — read whole.

**Network**
- **No database port, no broker port, no listening TCP port.** The MCP server is stdio only.

**IPC endpoints**
- **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
- **Test-only:** named pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` (Windows), created by `tests/channel_sqos_open.rs`, and endpoints named `viola-test-chan-<pid>-<label>` (a named pipe on Windows, a socket of that base name elsewhere), created by the channel tests (`tests/channel_endpoint.rs`, `tests/security_negatives_channel.rs`, the `viola-channel` crate tests and the `viola-e2e` cleanup tests). Both are outside the `viola-<h12>` namespace (the `test-` segment is never 12 lowercase hex characters), unique per test and never bound by `viola`.

- **Linux / macOS:** Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root, chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot. Linux abstract-namespace names are never used, because they carry no filesystem permissions and any local user could connect.

**Binary, subcommands and exit codes**
- Binary: `viola` (`viola.exe`).

- Test-only binaries, never in a release build: `viola-fake-agent` (a `[[bin]]` of the root package behind feature `fake-agent`, the stand-in `claude`), `viola-harness` (in `viola-e2e`; its command contract is test-plan §3) and `viola-mcp-tap` (a `[[bin]]` of `viola-e2e` that records a live MCP exchange for the `Mcp.` fixtures).

- Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp` (hidden from `viola --help`, run by the plugin), `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`.

- Hidden flags: `hook <event> --capture <DIR> [--answers <DIR>]` and its statusline form `hook statusline --capture <DIR> [--shell <WORD>]`, `viola verify`'s probe capture arms: an absolute, existing `<DIR>` only; no `VIOLA_*` read, no obs init, no channel; always exit 0 with empty stderr.

- Hook event arguments: `session-start`, `user-prompt-submit`, `pre-tool-use`, `permission-request`, `stop`, `session-end`, `notification`, `post-tool-use`, `post-tool-use-failure`, plus `statusline` (the per-session statusline wrapper).
- Exit codes `0, 1, 2, 10–14, 20, 21` as defined in Conventions.

**Claude Code integration names**
- Plugin name `viola`. MCP server name `viola` in `.mcp.json`. MCP tools `send`, `wait`, `last`, `answer`, `list`, which reach a session as `mcp__plugin_viola_viola__<tool>` (plugin `viola`, server `viola`; as measured live on claude 2.1.287 at chunk 2026-10-17-mcp-server-for-drivers, `evidence/live-mcp.md`: `--allowedTools mcp__plugin_viola_viola` allowed them at server level with no permission dialog, and the tools loaded deferred, reached through `ToolSearch` first).
- `viola plugin install`'s local marketplace `viola` (`viola_agent_claude::marketplace::NAME`) holds one plugin `viola` (source `./viola`), added to Claude Code as `viola@viola` (`PLUGIN_REF`); its `.mcp.json` registers the MCP server `viola`, and it carries no hooks.
- Registered hook events: SessionStart, UserPromptSubmit, PreToolUse (matcher `AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse, PostToolUseFailure. Registered today: all nine (PreToolUse and PermissionRequest since chunk 2026-10-04-dialog-answers-by-dialog-id, `timeout` 3615).

- Flags passed to the child: `--plugin-dir <viola home>/plugin/<version>-<hash>`, first on every start; then `--settings <instance dir>/settings.json` when the start wrote a settings override; then the caller's arguments. A revived start launches `claude` and adds `--resume <id>`, then `--fork-session` under `--fork`, the id one argv element of a direct spawn.

- `viola verify`'s probe plugin `viola-verify-probe`, written only under `ledger/probes/<pid>/plugin/`, its exec-form hooks on the pinned path each `hook <event> --capture`; verify's interactive children follow the print probe, each a direct `<program>` spawn at 80×24 under the R8 strip with no key written into a CLI dialog.

- Per-session settings override: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn; written only on Unix, and only for a pinned path of plain characters, else no override is written and no `--settings` is passed.

**Workspace crates**
- `viola` (root bin), `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`.
- Test-only: `viola-e2e` (harness library + bins `viola-harness` and `viola-mcp-tap`, `publish = false`; declares a no-op `fake-agent` feature so package-scoped `--features fake-agent` invocations resolve).
- Landed so far: every member — `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-mcp`, `viola-e2e`. `viola-pty`, `viola-agent-claude` and `viola-state` are sync crates (listed in `scripts/sync-crates.txt`, beside `viola-core`: four crates since chunk 2026-10-17-mcp-server-for-drivers) and, like `viola-mcp` and `viola-e2e`, each declares a code-free `fake-agent = []` feature so a package-scoped `--features fake-agent` build (cargo-mutants) resolves. `viola-channel` left that list at chunk 2026-10-17-mcp-server-for-drivers: it declares `tokio = ["dep:tokio", "interprocess/tokio"]` (its Tokio client, enabled by `viola-mcp`) and `fake-agent = ["tokio"]`, so its package-scoped mutation build compiles and tests the Tokio client; its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh`.
- Test-only feature: `viola-channel` `test-support = []` gates `pub mod test_support` (the `JsonFields` tracing field visitor; on Windows `canonical_sddl`, `dacl_of` and an independent `user_sid`). It is enabled ONLY by the root package's `[dev-dependencies]` entry (`viola-channel = { path = "crates/viola-channel", features = ["test-support"] }`), so no product build carries it, and `scripts/release-check.sh` refuses a release artifact built with it.
- Not a member: `viola-fuzz` (`fuzz/`, test-only, `publish = false`). It is its own cargo workspace with its own `Cargo.lock`, excluded from the root, and depends on `viola-core` and `viola-agent-claude` by path.

**Environment variables**
- `VIOLA_NAME`: the instance's `ViolaName`. It is set by `viola run` in the child, and its absence, a `VIOLA_DIR` naming no existing instance directory, or an instance its strict-modes check refuses makes every hook event and dialog path a silent exit 0: no `config.json` read, no stdin read, no line, no frame and no SessionEnd append (the hidden `hook --capture` arms, taken first, read no `VIOLA_*` at all, [Hook Contract]).

- `VIOLA_DIR`: the absolute path of the instance's state directory, set by `viola run` in the child; `hook`, `mcp` and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home.

- `VIOLA_BIN`: the absolute path of the pinned copy `<viola home>/bin/<version>-<hash>/viola(.exe)` (forward slashes), set by `viola run` in the child. viola itself never reads it, because hooks and `.mcp.json` carry the path literally. It exists so that skills and Bash tool calls inside the session can call the same pinned copy explicitly (`"$VIOLA_BIN" send …`) instead of relying on the PATH prefix.
- `PATH`: prefixed by `viola run` with the pinned copy's folder for the child.

- Removed from the child's environment (R8): every inherited `CLAUDE*` name except a name the user defines persistently, and the identity floor (`viola-agent-claude` `IDENTITY_FLOOR`, 11 names, its ledger row owed) whatever the persistent environment says. Names only: the strip, the registry reader and the logs never read a value.

- Read-only, provided by Claude Code to plugin processes: `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`. viola never uses them to locate its binary. A hook process of 2.1.287 was also handed `CLAUDE_ENV_FILE` and `CLAUDE_PROJECT_DIR` (as measured on the Linux dev host at chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host, names only); viola reads none of the four.

- Test-harness only, never read by `viola`: `AGENT_RUN_CHUNK_BASE`, `AGENT_RUN_KEEP_HOMES`, `AGENT_RUN_KEEP_FAILED` and `CI`, read for presence only, under which `run --local-live` refuses before any build, so the real `claude` never runs in CI.

- Test seams, read only by a `viola` built with the test-only `fake-agent` feature (never by a release build): `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, each a test seam, not a configuration channel: it disables no control and widens no redaction.

- Test-only, read by no `viola` build: `MCP_SERVE_TEST_CHILD`, a marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn, read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`; outside the `VIOLA_*` namespace, which is product plumbing.
- Read by every `viola` build's human output (the colour decision of `viola list`, `src/human/style.rs` `read`; release builds included), never set by viola: `NO_COLOR` (present, an empty value included, switches colour off), `TERM` (exactly `dumb` switches it off; a value containing `256color` picks 256 colours) and `COLORTERM` (`truecolor` or `24bit` picks truecolor). They are read through `std::env::var_os` as text, only compared, never echoed, and decide only whether and how deep SGR bytes are written: they configure nothing, reach no log line, error or refusal, disable no control and widen no redaction. `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read (the board's width is `viola_pty::host_size()`). The founder's own answer «Читать их» to P4 fork Q1 of chunk 2026-10-17-cli-output-discipline (its `inputs#I2`, relayed verbatim by the operator).

**Filesystem (viola home = `<user home>/.viola/`)**

- `config.json`: user settings — the budget thresholds `five_hour` and `seven_day` (whole percentages, defaults 90 and 85), `diagnostics_level`, `claude_env_keep` (the Unix R8 pass-list) and `dialog_hold_s` (60–3600 s, default 60); read with a `MAX_FRAME` cap, carrying `v` = 1; an unusable file or value falls back to the default with a `parse-rejected{parser:"config-json"}` line, so no value switches the gate off or makes the hold zero or unbounded.

- `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary, which `viola run` places once when it is absent, with no clobber, and never replaces (0700 on Unix), where `<hash>` is the first 16 hex of the SHA-256 of the exe's content; an existing copy is re-hashed on every start, and so is one a concurrent first start placed first, and a mismatch refuses the start (exit 1, `pinned-hash-mismatch`) and leaves the file as found. v1 never deletes pinned copies or plugin folders. Each instance snapshot records the pinned path it uses, so any later cleanup (an open item) can skip copies that a live or `stale` instance still calls.
- Home tree: before any log, `viola run` and `viola revive`'s start check the home tree (`viola_state::strict::check_home_tree`): the home to the read rule, then the instance's files (as `instances/<ViolaName>/` below), `ledger/`, `bin/`, `plugin/`, `bin/<version>-<hash>/`, the pinned copy and `plugin/<version>-<hash>/` with its `hooks/` and `.claude-plugin/` to the write rule, each that exists; a refusal exits 1 with the home-state `unable:` / `hint:` pair (the instance-state pair for a refusal inside the instance), opens no log, writes no line and starts no child, and `run` pins the very exe bytes it checked. `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` the same way before its log (exit 1, the home-state pair).
- `bin/<version>-<hash>/conpty/OpenConsole.exe` and `bin/<version>-<hash>/conpty/conpty.dll` (Windows x64 only): Microsoft's ConPTY companions embedded in the binary, written by `viola run` if absent (`pin_companions` through `replace_private_shared`) and never overwritten; at every start `bin/<version>-<hash>/conpty/` and each companion that exists first pass the strict-modes check (owner and DACL), then each is held with `FILE_SHARE_READ` only and fully re-hashed against its compiled SHA-256 pin until the spawn. A refused check or a mismatch writes nothing, leaves the file as found, never loads it, never refuses the start, and runs the child on the inbox ConPTY (`sideload_fallback` `strict-modes-failed` or `hash-mismatch`). v1 never deletes them.
- `plugin/<version>-<hash>/.claude-plugin/plugin.json`, `plugin/<version>-<hash>/hooks/hooks.json`, `plugin/<version>-<hash>/.mcp.json`.
- `marketplace/`: written only by `viola plugin install`, after its strict-modes check: the dirs `.claude-plugin/`, `viola/` and `viola/.claude-plugin/` (each 0700, `create_private_dir`) and the files `.claude-plugin/marketplace.json`, `viola/.claude-plugin/plugin.json` and `viola/.mcp.json` (each 0600, `replace_private`, rewritten whole on every run); any other entry is left as found, and viola never reads the tree back.

- `diagnostics/`: home-level per-role process logs, codes only: `run-<name>.ndjson`, `hook-<name>.ndjson`, `mcp.ndjson` and `cli-<name>.ndjson`. One line per `write`; dir 0700, files 0600 on Unix; line format owned by obs and fixed by `schemas/diag-line.v1.json`. Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (format fixed by `schemas/diag-detail.v1.json`). Every open of a role file or a detail file first checks the existing `diagnostics/` folder and the file, before anything is narrowed: a symbolic link refused, the folder to the read rule, the file to the read rule on Unix and the write rule on Windows, an absent path passing; a refusal is a failed open (the role file's writer ends at a sink, a detail line is dropped).

- `instances/<ViolaName>/`: `events.ndjson`, `snapshot.json`, their `.lock` siblings, `heartbeat`, `settings.json` and `diagnostics/`. v1 never rotates or truncates `events.ndjson`, because `wait`'s `after` and `send`'s `cursor` are byte offsets into it; bounding its size is an open item, any scheme keeping existing offsets valid. `viola list`, `viola revive`, `viola run`'s start, `hook statusline`, `viola hook`'s event and dialog paths, `viola link` / `viola unlink`, the six CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) and `viola mcp`'s frame tools check these files before they use them, ahead of any log, because opening a log narrows an existing home: the home, the instance directory and its `diagnostics/` to the read rule (Unix no group or other bit; Windows no read right granted to another SID), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check writes no line.

- `budget.json` + `budget.json.lock`: the newest `rate_limits` reading across wrapped sessions, timestamped, last-writer-wins, replaced whole under the lock; written only by `hook statusline` when the payload holds a `rate_limits` object; read by each wrapper's budget gate and by `list` through one checked read behind the strict-modes check, every failure no reading.

- `statusline-source.json`: the named statusline source of this home, in the Claude settings shape. viola never writes it. `viola run` reads it at every start through `take(MAX_FRAME)`, and whenever it exists it is the source of the user's statusline command (§Standard Contracts, Hook contract). Tests plant it.
- Read outside the viola home, never written: `<user home>/.claude/settings.json` (`viola_agent_claude::statusline::USER_SETTINGS`). `viola run` reads it at start only when the home is the default home (`cmd::default_home`) and holds no `statusline-source.json`, through the same bound, and takes only its `statusLine` command.

- `ledger/stamps.json` + `ledger/stamps.json.lock`: `viola verify` stamps in the Ledger stamps envelope; `ledger/` 0700 and both files 0600 on Unix. The ONE writer is `viola_state::stamps::update_stamps`, called only by `viola verify`; `run`'s version gate reads it behind the strict-modes check, a refusal recording `cli_verified:false`.

- `ledger/probes/<pid>/`: `viola verify`'s probe dir (0700), written only by `verify` and removed whole by a drop guard when verify passes, fails or is stopped by Ctrl-C or TERM; its captures are transient, content-bearing raw hook payloads (0600).

- verify's interactive probe dirs, all outside the viola home and all 0700 (Run A's under the OS temp dir, Runs B to E's `<cwd>/.viola-verify-<pid>*/`), each a drop guard: removed when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing.

- Test homes only: `fake/<name>.control` (append-only; each `\n`-terminated line releases one gated fake-agent step) and `fake/<name>.receipt.ndjson` (the fake agent's receipt, `"v":1` + kebab `kind`, one write per line). The test chain passes them as `--control` / `--receipt`. `viola` never reads or writes them.
- Test-only, outside the viola home: `<temp dir>/viola-pty-watch/<test name>.<pid>.report` and its sibling `<test name>.<pid>.test.report` (`<pid>` the test process's id, so two test processes running the same test never share a report file) — viola-pty's real-PTY tests have their child stream its report to the first, one line per step (`start`, the key-free size watcher's `size`, `byte`, `restored`), and the test stream its own steps to the second (`resize-returned` · `key-written` · `key-flushed` · `dsr-cpr {n}`, the count of `ESC [ 6 n` the rig's output drain saw), so a test the runner kills keeps its evidence. Both are kept when the test panics or is killed and removed when it passes. `viola` never reads or writes either.

- Test-only, outside the viola home: `<temp dir>/viola-root-watch/<test name>.<label>.report`, the evidence file of the root test support's `Watch` (bound `WITHIN` = 7 s), kept when the test panics or is killed and removed when it passes. `viola` never reads or writes it.

- Test-only, outside the viola home: `<temp dir>/viola-chaos-*/home` — `tests/support/home.rs` `TestHome::outside_scan()`, used only by `tests/chaos_feed_panic.rs`. Its forced vt100 feed panic writes a G2-counted `event:"panic"` line, so the home sits outside `target/e2e-home` and therefore outside G2 (zero panics), G4 (schema conformance) and the secret scan; the test asserts that panic line and its `parse-rejected{vt100-feed, panicked}` line present itself. It is the second named carve-out to the test-data rule, beside `seed_conpty` (the founder's ruling, 2026-10-04). `viola` never names it.

- Test-only, outside the repository, on the Linux dev host only: `/tmp/viola-e2e-home-<uid>/` (mode 0700, owned by the user), the tmpfs directory the dev host's `target/e2e-home` link points at, holding every test and session home of that host. `viola` never reads or writes it by that name. No CI runner has it.

**Repository**

- `fixtures/claude/<cli-version>/`: the hook-payload, screen and variant fixtures `viola verify --record` writes (only when 0 rows fail), replayed by the fake agent in CI, and the `Mcp.` exchanges the test-only `viola-mcp-tap` records, replayed against `viola mcp`; each scrubbed and hygiene-walked, an unclean recording refused whole, naming the file and the check code, never the content.

- `fixtures/fake-scripts/<scenario>.json`: committed fake-agent turn scripts (synthetic text only), validated against `schemas/fake-script.v1.json`.
- `schemas/fake-script.v1.json`: the tolerant JSON schema for fake-agent scripts (test-side).
- `schemas/claude-fixture.v1.json`: the tolerant schema for recorded hook-payload fixtures (`required: [hook_event_name]`, its enum the fake agent's nine `REGISTERED_EVENTS`; test-side).
- `schemas/claude-statusline-fixture.v1.json`: the tolerant schema for recorded status line fixtures (an object, `session_id` required as a string; `rate_limits`, when present, an object requiring `five_hour` and `seven_day` objects; test-side). `tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/StatusLine.*.json` against it and every other hook payload against `claude-fixture.v1.json`.
- `schemas/claude-agents-fixture.v1.json`: the tolerant schema for recorded `claude agents --json` fixtures (one array of rows over the measured keys; test-side; chunk 2026-10-16-unwrapped-sessions-on-the-board).
- `schemas/claude-mcp-fixture.v1.json`: the tolerant schema for recorded MCP exchanges, the `Mcp.` class (`v` const 1; `client` an array of JSON-RPC 2.0 frames, each `method` a string, that contains a `tools/list` request with an `id`; `results` an object keyed by method, `tools/list` required with a `tools` array of named tools, any other key an object; test-side). `tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/Mcp.*.json` against it with the path, username and email walk, and its hook-payload walk leaves `Mcp.` files out.
- `schemas/claude-screen.v1.json`: the schema for recorded screen fixtures (`screen_phase` ∈ modal|ready|turn, `cols` const 80, `rows` exactly 24 strings; test-side).
- `schemas/config.v1.json`: the tolerant (`additionalProperties: true`) schema of `config.json`: `v` (const 1), `diagnostics_level` (`info` | `debug`), `claude_env_keep` (at most 32 strings `^CLAUDE[A-Z0-9_]*$`), `five_hour` / `seven_day` (integers 0-100), `dialog_hold_s` (integer 60-3600), the six `CONFIG_KEYS` of `src/obs.rs` pinned equal as a set by `config_keys_are_the_config_schema_properties`; test-side, its readers unchanged (chunk 2026-10-17-cli-machine-contract).
- `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`: the obs-owned default-deny line schemas for home-level process-log lines and instance detail lines. Each is self-contained (no cross-file `$ref`), and both are pinned by `tests/contract_diag_schema.rs`.
- `vendor/conpty/<version>/x64/{conpty.dll,OpenConsole.exe}` (today `1.24.260710001`): the committed Microsoft binaries from the nuget package `Microsoft.Windows.Console.ConPTY` (MIT), byte-identical to the pinned nupkg, `.gitattributes` `vendor/conpty/** binary`, embedded by `src/conpty.rs`. `scripts/conpty-vendor.sh` is their one writer (the default re-vendor) and their verifier (`--verify`: a fetch outside the tree, the nupkg SHA-256 before extraction, a byte compare and the Authenticode signer; `--probe`: its four refusals and a control). Its pins are parsed from `src/conpty.rs`'s text.
- `crates/viola-core/proptest-regressions/` and `crates/viola-agent-claude/proptest-regressions/` (`hook.txt`, `statusline.txt`): committed proptest failure seeds.
- `fuzz/`: the separate cargo-fuzz workspace (`viola-fuzz`, `[package.metadata] cargo-fuzz = true`, its own `[workspace]` and committed `Cargo.lock`). It holds `fuzz/rust-toolchain.toml` (the fuzz-only `channel = "nightly-2026-09-20"`), `fuzz/fuzz_targets/<target>.rs` (today six, by `ls fuzz/fuzz_targets | wc -l`: `viola_name`, `channel_frame`, `hook_stdin`, `vt100_feed`, `paste_text` and `claude_agents_json`) and `fuzz/corpus/<target>/` (committed synthetic seeds; `hook_stdin` has 12 by `ls fuzz/corpus/hook_stdin | wc -l`, two of them for the statusline payload reader that target also feeds, `vt100_feed` 6, `paste_text` 8 and `claude_agents_json` 6, each kept byte-exact by `.gitattributes` (`fuzz/corpus/paste_text/** binary`, `fuzz/corpus/claude_agents_json/** binary`)). `fuzz/target/`, `fuzz/artifacts/` and `fuzz/coverage/` are gitignored; the nightly `fuzz` job uploads `fuzz/artifacts/` on failure.

- `target/agent-run/<session>/`, `target/agent-run/chunk.diff` and `target/agent-run/artifacts/`: written by `viola-harness`, gitignored. `chunk.diff` is the one file `secret-scan` skips, by exact path, and the `harness-<os>` upload excludes it.

- `target/lcov.info`: the `run --coverage` LCOV output. Never uploaded, gitignored.

- `target/e2e-home/viola-session-*/home`, `target/e2e-home/viola-test-*/home` and `target/e2e-home/viola-live-<pid>/home`: every harness and test home but the one `viola-chaos-*` home, gitignored and kept in CI for the obs gates; a new test home removes only the dirs whose recorded owner process is gone, never by age or name; the base is prepared by two test-side keepers that hold one contract.

- `target/conpty-seed/<key>/` (Windows x64): the one per-run copy of the ConPTY companions that root `tests/support/home.rs` `seed_conpty` hard-links (copy fallback) into a test home's `bin/<key>/conpty/` before `Wrapper::boot` and the piped and outer-PTY `viola run` starts (never in `run_viola_unseeded` starts or `tests/conpty_sideload.rs`, which keep the product's write path). Regenerated on demand, gitignored; `viola` never reads it.
- `target/harness/`: the harness's own cargo target dir (`CARGO_TARGET_DIR` for its builds and test runs), because a running `target/debug/viola-harness.exe` cannot be relinked on Windows.
- `target/mutants/`: the `run --mutants` cargo target dir (`MUTANTS_TARGET`). The root pre-build writes `<repo>/target/mutants`, and cargo-mutants gets the relative `CARGO_TARGET_DIR=target/mutants` (still `--copy-target=true`), so each copied tree builds every test binary in its own copy: a copied default `target/` keeps the original tree's `CARGO_BIN_EXE_*` paths, so root integration tests would drive the unmutated binary (as measured at chunk 2026-09-27-hooks-to-normalised-events `evidence/red-mutants-stale-test-binary.md`). Gitignored.
- `target/deny-probes/run-<utc>-<pid>/`: the throwaway Cargo projects `scripts/deny-probes.sh` writes per run, gitignored.
- `target/viola-answers-*/` (under the test process's current dir): the `tempfile` dir of `src/cmd/hook.rs`'s unit case `answer_reads_no_relative_answers_dir_that_resolves`, reached only by the relative path `../<current dir's name>/target/<tmp>/answers` and removed on drop, gitignored. It sits under the current dir, not the OS temp dir, because a temp dir on another Windows drive (`windows-2025`: the checkout on `D:`, the temp dir on `C:`) has no relative path from the current dir. `viola` never reads it by that name.

- `target/profraw-census/<utc>-<pid>/`, or `target/profraw-census/<name>/`: the per-run tallies `scripts/profraw-census.sh` writes, a dev-host witness for the corrupt-profile mechanism (test-plan §10 Stack adjustments), gitignored. No CI job, `pre-push` stage or harness command runs it.

- `target/supply-chain/`: `deny.json` (the root graph), `deny-fuzz.json` (the `fuzz/Cargo.lock` advisories and sources audit) and `zizmor.json` from the CI `supply-chain` job, uploaded as artifact `supply-chain` (7 days, admissible by content: obs-plan §8 item 6), gitignored.
- `target/orphans-probes/run-<utc>-<pid>/`: the throwaway crates `scripts/orphans-check.sh --probe` writes per run, each with an empty `[workspace]`: a planted `src/stray.rs`, a file gated on a target no CI triple compiles, and a `cfg(windows)`/`cfg(unix)` file-pair control (verdict `orphans-check probes: 2/2 fired, control clean`), gitignored.
- `target/release-check/`: the cargo target dir the local release-check gate entry passes as `CARGO_TARGET_DIR` (the script honours an inherited one and never sets it), gitignored.
- `target/perf/`: the `run --perf` target dir (`cargo build --release --workspace --features viola/fake-agent --target-dir target/perf`, binaries under `target/perf/release`; test-plan §10), gitignored. Its hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json`, one per row (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`), all five required by `gate --require perf` (obs-plan §9); the perf session boots unstamped.
- `target/g2-probe/`: the throwaway scope `scripts/g2-zero-panics.sh --probe` builds per run and removes after it (verdict `g2-probe: all cases as expected`), gitignored.
- `target/nextest/ci/junit.xml`: the nextest `ci` profile's JUnit, gitignored. The `junit-<os>` upload (behind the secret scan) carries it alone.
- `target/secret-scan/hits.json`: the `viola-harness secret-scan` hit report. It is written only when there are hits and removed on every clean run. CI uploads it as artifact `secret-scan-<os>` (7 days) only when the scan fails. Gitignored.
- `target/tools/ripgrep/bin/`: ripgrep 15.2.0, installed by `scripts/install-ripgrep.sh`, gitignored. `target/tools/ripgrep/probe-*`: the scratch dirs of `scripts/install-ripgrep.sh --probe` (TLS failure retried then fail-closed, digest mismatch refused, matching digest passes; no network), gitignored.
- `target/lint-probes/run-<utc>-<pid>/` and `target/lint-probes/target/`: the throwaway crates `scripts/lint-probes.sh` writes per run, and their shared cargo target dir, gitignored.
- `target/run-archive/<n>/`: the per-run archive `viola-harness run` writes at the end of every run with a source to archive — the JUnit files that run's nextest or coverage suites wrote (`junit-{nextest-unit,nextest-integration,coverage}.xml`), and the `outcomes.json` its mutation arm read. `<n>` is one more than the highest existing slot, and the newest 10 are kept. The run document names it as `archived` (repo-relative). It is gitignored, sits outside `target/agent-run/`, and is never uploaded, because `outcomes.json` carries absolute argv paths.

- `<repo parent>/viola-mutants-scratch/` (outside the repository, a sibling of it): the host mutation scratch, where on a Windows host `run --mutants` puts cargo-mutants' temp copies and its `--output`; a guard refuses any other final component or a repository inside it, never a fallback to `%TEMP%` or the repo, and its path is never printed. Off Windows the harness arm is inert.

**Docker volumes, containers, service names:** none.

## Infrastructure Patterns

Contracts: .andromeda/registries/architecture-contracts.toml — ask registry.py contracts; read one contracts/architecture/{key}.md; never whole.

## Cross-cutting Patterns

Detail: .andromeda/architecture-detail/cross-cutting-patterns.md — read whole.

- **Config management.** Precedence: CLI flags override `<viola home>/config.json`, which overrides built-in defaults. The viola home resolves `--home`, then the grandparent of `VIOLA_DIR`, then `<user home>/.viola/`; `config.json` is parsed tolerantly and carries `v`. There are no secrets in v1, and environment variables are not a configuration channel: a dependency or feature that reads one breaks this rule like a direct read does.

- **Development Style signal:** agent-driven.
- **Identity by instance, never by location.** Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` (or an explicit `ViolaName` argument), never through the working directory. Two sessions in one folder never share records.
- **Fail open toward the human.** Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human. Any failure to confirm delivery is reported as `not-delivered`, never retried blindly. The one exception is a local command on the ledger whose post-condition is "none" or not yet measured on this CLI version. It is reported as `ok` with `confirmed: false` and detail `unconfirmable`.
- **Mixed-version tolerance.** Every channel `params`, event, snapshot and config carries `v`, and channel `params` also carry the sender's `CARGO_PKG_VERSION`. Readers skip unknown kinds and fields and surface the counts. A newer request than the wrapper supports gets a `-32602` naming the wrapper's version.

- **Capability ledger as the single gate for CLI-specific behaviour.** Any new dependence on an undocumented `claude` behaviour is added as a ledger row in `viola-agent-claude`, with a `viola verify` probe that has a post-condition check; it is never hard-coded as an unconditional assumption. One ruled limit: a CLI shape no `send` relies on needs no probe.

- **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime. CLI `wait` is a blocking read on `viola-channel`'s sync client.
- **Crash-safe disk writes.** Append-only logs use one `write` per line. Mutable files are replaced atomically. Every guarded file has its own `.lock` sibling. Readers always tolerate a torn last line.
- **Diagnostic output channels.** `hook` writes only its decision body to stdout and nothing to stderr; on the `hook statusline` pass-through stdout carries the user's status line output instead, and nothing else, and its `--capture` probe arm prints nothing (Hook Contract). While the child runs, `run` writes nothing to the terminal except the child's own output, because the terminal carries the child's screen. Every role's codes-only process log goes to its home-level role file under `diagnostics/`: `run-<name>`, `hook-<name>`, `mcp` or `cli-<name>` `.ndjson`. Content-bearing detail belongs only in the instance's `diagnostics/detail-<role>.ndjson`. `mcp` writes only MCP frames to stdout, and its log goes to `diagnostics/mcp.ndjson`; it writes JSON to stderr only when that file cannot be opened. Short-lived CLI verbs may use stderr for their human output. The logger and the format are owned by obs.
- **Cross-platform discipline.**
  - Every path written into plugin files uses forward slashes.
  - Nothing shells out through `sh`, `bash` or `cmd`, except `hook statusline` running the user's own statusline command (see Hook contract).
  - Child processes are spawned directly.
  - Endpoint names are hashed and short.
  - Every OS-specific branch compiles and is tested on its CI runner.
- **Untrusted upstream text.** A driven session's `Stop` text and dialog payloads are content, never commands to viola. viola forwards them as escaped strings and never interprets them (R1).
- **Bounded readers.** Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.
- **Local endpoint trust boundary.** Any process that can open an instance's endpoint can `send` prompts and `answer` dialogs, including PermissionRequest. v1 trusts every process of the same OS user and no other. Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.

## Project Intent

- **Product type.** Hybrid local developer tool made of three parts:
  - a native cross-platform CLI binary;
  - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
  - a minimal view-only frontend, deferred to 0.2.0; its technology is not chosen.
- **Core function.** Lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment.
- **Growth model.** Modular monolith: one binary with compiler-enforced internal crates (`pty · channel · state · agent-claude · mcp` around `core`), and no daemon.
- **How new functionality is added.**
  - New capabilities arrive as new subcommands or channel verbs, exposed consistently on the CLI (`--json`) and, where a driver needs them, as MCP tools.
  - New Claude behaviours arrive as capability-ledger rows in `viola-agent-claude`.
  - A second driven agent (for example Codex) arrives as a sibling `viola-agent-<name>` crate, at which point the adapter trait is extracted into `viola-core`.
- **Template patterns.** None. This is a single-product repository, not a fleet.
- **Scale path.**
  - v1 is personal (first live on the Linux dev host, by the founder's ruling R-L1; Windows, macOS and Linux CI-tested against the fake agent; Windows live awaits an interactive Windows host).
  - v1.x adds public distribution (dist 0.33.0 releases, signing, winget/Homebrew/Scoop, a marketplace entry, self_update 1.3.0).
  - A phone or remote view comes later, behind authentication.

## Inherited Defaults

- Framework: Rust 1.98.1, pinned by `rust-toolchain.toml` (MSRV 1.96, edition 2024). A sync std-thread core; Tokio 1.53.3 only in `mcp`; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.
- Database: none. ndjson append logs + atomic JSON snapshots (tempfile 3.27.0 `persist` through one shared helper) + std `File::lock`, serde_json 1.0.151.
- IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- API style: typed `RefusalReason` in `result.refusal`.
- Errors: thiserror 2.0.20, one enum per crate (except `viola-pty`'s hand-written `PtyError`; `viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are open divergences, [Error Handling]), anyhow 1.0.104 only in the root bin; typed CLI exit codes; `viola hook` always exits 0.
- Validation: tolerant serde for external payloads, versioned-tolerant own formats, nutype 0.8.0 newtypes.
- Timestamps: chrono 0.4.45, RFC 3339 UTC with milliseconds and `Z`, field `ts` / `*_at`.
- Agent isolation: Claude-specific shapes only in `viola-agent-claude`, and every undocumented behaviour is a capability-ledger row.
- Deployment: local `cargo install --path .`, plugin embedded in the binary and loaded with `--plugin-dir`; no containers, no hosting.
- CI: GitHub Actions on windows-2025 / macos-latest / ubuntu-latest against the fake agent.
- Development Style: agent-driven.
- Publishability: every workspace crate, including the root `viola` bin, sets `publish = false` and inherits `license = "MIT OR Apache-2.0"` from `[workspace.package]`. Nothing is published to crates.io in v1.

## Existing Scopes

None — new project.

<!-- subjects: dec-hook-contract · stack · dec-concurrency-backend-framework · dec-pty · dec-screen-model · dec-delivery-confirmation · dec-database-state-store · dec-snapshot-writer · dec-message-broker-ipc · dec-mcp · dec-hook-transport · dec-human-takeover-wheel · dec-links · dec-budget-governor · dec-cli-version-compatibility · dec-session-liveness · dec-error-handling · dec-module-boundaries · dec-deployment-distribution · dec-plugin-scope · dec-cli-conventions · conv-refusal-reason · conv-exit-codes · conv-mcp-results · contract-list-payload · contract-snapshot-envelope · contract-ledger-stamps · contract-instance-snapshot · contract-channel-methods · contract-event-data · contract-cli-json-output · contract-hook-statusline · contract-session-liveness · res-ipc-endpoints · res-binaries · res-claude-integration · res-env-vars · res-filesystem · res-repository · crosscut-config · crosscut-capability-ledger · crosscut-bounded-readers · infra-crate-dependency-direction -->

