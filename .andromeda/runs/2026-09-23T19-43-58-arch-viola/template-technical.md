## Quiz II Fields

Fields are ordered by impact. Fields that Quiz I or a closed research finding already decides are marked **[inferred from Quiz I]** and are not re-asked. Crate versions come from research-targeted.md (crates.io, checked 2026-09-23).

### 1. Language and Toolchain [maps to: Stack and Technologies, Inherited Defaults]
- **Rust, stable toolchain (host has 1.95)** [inferred from Quiz I: Primary Language = Rust]. It is above every MSRV in the research: axum 1.80, rmcp 1.88, and std `File::lock` needs 1.89+. Not re-asked.

### 2. Concurrency Model and Server Stack [maps to: Stack and Technologies, Established Decisions (General), Infrastructure Patterns]
This is viola's "framework" choice. It decides whether the hot `viola hook` path carries an async runtime.
- **A. All-async: Tokio 1.53.1 + axum 0.8.9 + rmcp 3.4.1.** One style for the edge libraries, since rmcp, axum and interprocess-async all require Tokio. The PTY pump still needs blocking threads or `spawn_blocking`, so `run` ends up with two styles anyway. Every hook call pays runtime startup (O6, not measured).
- **B. Sync core with std threads only: tiny_http 0.12.0 + a hand-rolled stdio JSON-RPC MCP server.** Fewest dependencies and the fastest `hook` cold start. But tiny_http has had no release since 2022-10-06. SSE keep-alive, the Host check and shutdown would all be hand-written, and the MCP spec revisions become yours to track. That is a maintenance risk for the v1.x brake, which adds POST routes.
- **C. Hybrid: sync `run`/`hook` fast path, with a Tokio runtime built only inside `mcp`, `ui` and `wait` (same crates as A).** No runtime on the hot path, and the async ecosystem is used only where it pays off. Costs: two I/O styles, and the `channel` module has to offer both blocking and async clients (interprocess ships both).
- Not viable: smol 2.0.2 (last release 2024-09-07; everything at the edges is Tokio-bound).

### 3. PTY Layer [maps to: Stack and Technologies, Established Decisions (General)]
This is the riskiest dependency: it decides whether Windows correctness is borrowed or owned. portable-pty 0.9.0 is **excluded**, because of wezterm#6783 (garbage reads on Windows, still open).
- **A. portable-pty 0.8.1, pinned.** The spike showed it working on ConPTY. It is frozen since 2023 and misses the 2026 upstream fixes (Windows `kill()`, move to windows-sys).
- **B. portable-pty from wezterm git main.** Has the 2026 Windows fixes but no published release. A git dependency blocks publishing viola to crates.io later.
- **C. portable-pty-psmux 0.9.7.** Adds the ConPTY flags `PSEUDOCONSOLE_RESIZE_QUIRK`, `WIN32_INPUT_MODE` and `PASSTHROUGH_MODE`. Passthrough could help with the O1 stray-spaces artifact, but that is untested. One maintainer, about 2.7k downloads.
- **D. Own ConPTY on windows-sys 0.61.2, plus a Unix PTY on nix 0.31.3 or rustix.** Full control over flags and exit detection (ConPTY does not close its output on exit). About 500–1000 lines of platform code to own. pty-process 0.5.3 does not help because it is Unix-only.

### 4. Screen Model for Readiness and Modal Detection [maps to: Stack and Technologies, Established Decisions (General)]
Only used to detect readiness and modals, never to read content. It matters because CLI-native modals bypass every hook (Quiz I wheel amendment b).
- **A. vt100 0.16.2.** Lightweight, and a newer version of the crate the spike used (0.15.2).
- **B. alacritty_terminal 0.26.0.** A fuller terminal emulator and heavier. Released 2026-04-06.
- **C. No screen model.** Rely only on the send-delivery confirmation that Quiz I already requires (no matching `prompt-submitted` within the window means `not-delivered`). A modal is caught only after a send fails.

### 5. State Store [maps to: Established Decisions (Data Persistence), Conventions (data model), Occupied Resources]
Several independent processes write at once (the wrapper, N short-lived hook processes and the CLI verbs), and there is no daemon to serialise them.
- **A. ndjson append logs + atomically replaced JSON snapshots + std `File::lock` on a separate lock file.** Snapshots are replaced with atomic-write-file 0.3.1 or tempfile 3.27.0 `persist`. Human-readable, `tail`-able, zero C, and it matches the brief (R4). Each event must be one `write`, readers must heal a torn last line, and there are no queries (the UI rebuilds by replaying the log).
- **B. SQLite in WAL mode via rusqlite 0.40.2 (bundled SQLite 3.53.2).** Transactional writes from many processes, so the wheel and budget-pause state get real transactions, and the UI can query directly. Costs: a C build (MSVC in CI), a binary audit trail, schema migrations, and a departure from the brief.
- Excluded: redb 4.3.0, whose multi-process mode was only hardened on 2026-09-06 (PR #1462), and fjall 3.1.10, which is single-process by design.

### 6. Wrapper IPC Channel [maps to: Stack and Technologies, Standard Contracts, Occupied Resources]
Local IPC between each wrapper and its short-lived clients (`hook`, `send`, `wait`, `mcp`). No broker.
- **A. interprocess 2.4.4 `local_socket`.** One API over named pipes (Windows) and Unix sockets, in both sync and Tokio flavours. Each wrapper owns its own endpoint, so no daemon is needed, and `hook` can block for the dialog answer. Main maintainer is one person. Because macOS caps socket paths at about 104 bytes, endpoint names are derived from a hash.
- **B. Tokio-native endpoints (`tokio::net::windows::named_pipe` + `UnixListener`) behind your own `channel` trait.** No extra dependency. Async only, so this forces Tokio into `hook` (it conflicts with option 2C), and you write the cross-platform naming yourself.
- **C. File-based mailbox (the ndjson log as the bus) + notify 8.2.0.** Survives restarts and is fine for `ui`. It is weak for the request/response round trip a hook needs within its timeout, and watcher latency differs by OS.

### 7. Hook Transport [maps to: Standard Contracts, Established Decisions (General)]
`type: "http"` is **excluded**. Its URL cannot interpolate env vars, so it needs a fixed central listener, which breaks Quiz I's no-daemon rule. `mcp_tool` is **excluded** because it is unavailable at `SessionStart`.
- **A. Exec-form `command` hooks (`"command": "viola", "args": ["hook", "<event>"]`) for every event, synchronous.** Skips Git Bash and its MSYS path rewriting, and returns decisions through stdout JSON. Costs one process spawn per event and has to fit the 1.5 s `SessionEnd` budget. The binary must exit 0 fast when `BRIDGE_NAME` is absent.
- **B. Exec-form `command` hooks, with `async: true` for fire-and-forget events (`PostToolUse`, `Notification`) and sync only for decision events (PreToolUse, UserPromptSubmit, Stop, …).** The turn no longer waits on events that make no decision. Those events can never return a decision.

### 8. MCP Server Implementation [maps to: Stack and Technologies, Standard Contracts]
Stdio transport with the tools `send`, `wait` and `last`.
- **A. rmcp 3.4.1 (official SDK), features `server` + `transport-io`, optional schemars 1.x.** Implements MCP 2026-07-28 and is compatible with 2025-11-25. Churn risk: a 3.0 major on 2026-07-28 and 11 releases since. Needs Tokio.
- **B. rust-mcp-sdk 2.0.0 (community).** Also implements 2026-07-28. Far less used (about 112k downloads against rmcp's 14.9M), and its majors move fast (1.0 on 2026-07-26, 2.0 a month later).
- **C. Hand-rolled newline-delimited JSON-RPC over stdio.** No SDK churn and no runtime needed. You own the handshake and capability negotiation across spec revisions.

### 9. Distribution and Install (v1) [maps to: Established Decisions (Deployment), Infrastructure Patterns]
CI is pre-filled (see Pre-filled Values). This field covers only how the binary and plugin reach the founder's machines.
- **A. `cargo install --path .` onto PATH, with the plugin from a local-directory marketplace loaded in place (or `--plugin-dir`).** Fits v1 exactly. Risks: PATH drift between the terminal and Claude's hook environment on Windows, and the binary and plugin versions drifting apart.
- **B. dist (cargo-dist) 0.33.0.** Generates per-OS archives, shell/PowerShell installers and a release workflow. It is the natural v1.x/public step, and for v1 it is setup cost for things Quiz I deferred.
- **C. Binary shipped inside the plugin's `bin/`.** One install, and the overseer's Bash can run `viola send` bare. But the plugin tree must carry per-OS binaries, `viola run` still needs `viola` on the user's PATH, and it is disallowed for claude.ai org distribution.

### 10. Wrapper Channel Protocol and Framing [maps to: Conventions (API/interface format), Standard Contracts]
- **A. JSON-RPC 2.0 over ndjson.** The same shape as MCP, so `mcp` becomes a thin adapter. Standard `code`/`message`/`data` errors, and request ids pair each hook with its answer.
- **B. Custom serde internally-tagged enum over ndjson (`{"op":"send",…}` / `{"ok":false,"refusal":"human-typing"}`) with a `v` field in every frame.** Smaller, with refusal reasons as first-class values. You own the versioning.
- **C. Either A or B with length-prefixed framing instead of ndjson.** Embedded newlines are never a problem, but frames are harder to read by eye and no longer match the log and MCP framing.

### 11. GUI Live Feed and Server-side Source [maps to: Standard Contracts, Occupied Resources (routes)]
The GUI is view-only (Quiz I). This field covers the transport and how the server learns about changes.
- **A. SSE (axum 0.8.9 `sse`, `Sse::keep_alive`), fed by tailing ndjson with notify 8.2.0.** One-way, matching view-only. Browsers reconnect by themselves. Gotcha: exclude `text/event-stream` from tower-http 0.7.1 `CompressionLayer`, which otherwise buffers it.
- **B. WebSocket (axum 0.8.9 `ws`).** Two-way, so it is ready for the v1.x brake. It also opens a wider attack surface and different Origin handling on a page that only reads.
- **C. Plain polling (the page re-fetches JSON every N s), with the server re-reading on a timer.** Simplest to reason about and to check with a headless browser. Laggier, and each viewer adds disk reads.
- Session liveness for any option: `claude agents --json`, and/or sysinfo 0.39.6 pid checks.

### 12. Validation and Parsing Strategy [maps to: Established Decisions (Validation Library), Conventions (data model)]
External payloads (hook JSON, `claude agents --json`, statusline `rate_limits`) are always parsed **tolerantly** with serde 1.0.229 / serde_json 1.0.151: no `deny_unknown_fields`, `#[serde(default)]` / `Option<T>`, the raw `Value` kept for the fixture recorder, and serde_path_to_error 0.1.20 to report drift [inferred from Quiz I: CLI Version Compatibility Posture — degrade, never break]. The fork is how viola treats **its own** formats and inputs:
- **A. Strict own formats (`deny_unknown_fields` on channel frames and ndjson events) + nutype 0.8.0 newtypes (`BridgeName`, `Percent`).** A version mismatch fails loudly. An older `ui` reading a newer log will break.
- **B. Versioned tolerant own formats (each event carries `v`, readers skip unknown event kinds) + nutype 0.8.0 newtypes.** Readers and writers of different versions can mix, which suits a log that outlives binary upgrades. Drift fails quietly unless it is logged.
- **C. Option B plus garde 0.23.0 (or validator 0.21.0) derive validation for config structs.** Declarative rules on thresholds and config. The research calls this mostly overkill for a handful of fields.
- Optional add-ons for any option: schemars 1.2.2 for MCP tool input schemas (the family rmcp uses), and jsonschema 0.57.0 to check the fixtures `viola verify` records.

### 13. Timestamp Type [maps to: Conventions (data model: timestamp handling)]
Used for `resets_at` and event timestamps.
- **A. jiff 0.2.37.** Modern API with correct time-zone handling. Adds a dependency.
- **B. chrono 0.4.45.** Already pulled in by rmcp. The familiar choice.

### 14. Module Boundary Enforcement [maps to: Established Decisions (Module Boundaries), Infrastructure Patterns (directory structure), Occupied Resources (module/crate names)]
The rule to enforce: nothing outside `agent::claude` knows Claude shapes, and `pty`/`channel`/`state` know no agent.
- **A. Cargo workspace with a flat `crates/` layout** (`viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui` + the `viola` bin). The compiler enforces the boundaries, and parallel/incremental builds help 3-OS CI. Costs more manifests and `pub` API ceremony.
- **B. Single crate using `pub(crate)`/`pub(super)` visibility plus clippy `disallowed_methods`/`disallowed_types` in `clippy.toml`.** Lowest ceremony. Sideways imports inside the crate (e.g. `pty` → `agent::claude`) are caught only by review.
- **C. Single crate plus an architecture linter: cargo_pup 0.1.8 or modou 0.4.0.** Import rules become lints. cargo_pup needs nightly, which is friction on stable 3-OS CI. modou is very young (about 170 downloads).
- Supporting tools for any option: cargo-deny 0.20.2 (crate bans and licences), cargo-modules 0.27.0 (module-graph review).

### 15. Error Handling Pattern [maps to: Established Decisions (Error Handling), Conventions (error schema / type registry)]
Fixed regardless of choice [inferred from Quiz I + hook contract]: one typed `RefusalReason` enum (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`) shared by the CLI (typed exit codes + `--json`) and MCP (`isError: true` tool results; JSON-RPC errors only for protocol faults). Hooks fail open: exit 0 with no body means "no decision", and exit code 2 is never produced by accident.
- **A. thiserror 2.0.20 typed enums in each module + anyhow 1.0.104 only at the binary edge (main / subcommand dispatch).** The standard split. rmcp already depends on thiserror.
- **B. snafu 0.9.2.** Context selectors carry typed context (instance name, path) without hand-writing it. Heavier and less common.
- **C. Option A + miette 7.6.0 for human-facing CLI diagnostics.** Richer terminal errors, but irrelevant to agent-facing `--json` output. Last release was 2025-04-27.

## Section Coverage Map

| Scope-template Section | Covered by |
|---|---|
| Design Philosophy | Derived from Quiz I (personal scale with cross-platform-from-day-one; general bridge with nothing Andromeda-specific; no daemon; confirm-never-assume; degrade on unverified CLI; human always wins the wheel) + Quiz II fields 5, 7, 12, 14 |
| Stack and Technologies | Quiz II fields: 1, 2, 3, 4, 5, 6, 8, 11, 13 + pre-filled clap 4.6.7 and serde/serde_json |
| Established Decisions | Every Quiz II answer becomes a decision (Data Persistence ← 5; API Style ← 10; Deployment ← 9; Validation Library ← 12; Module Boundaries ← 14; Error Handling ← 15; General ← 2, 3, 4, 6, 7, 8, 11, 13) |
| Conventions | Quiz II fields: 10 (interface format), 12 (data model / versioning), 13 (timestamps), 15 (error schema / RefusalReason registry) + pre-filled CLI/MCP/hook conventions |
| Standard Contracts | Quiz II fields: 6, 7, 8, 10, 11 + pre-filled hook stdin/stdout contract and viola's normalised event set |
| Occupied Resources | Quiz II fields: 5 (state files / lock files / DB name), 6 (endpoint naming), 11 (GUI routes), 14 (crate/module names) + defaults from stack choice (the `viola` binary name, subcommand verbs, `BRIDGE_NAME` and the `CLAUDE*` strip list, `${CLAUDE_PLUGIN_ROOT}` / `${CLAUDE_PLUGIN_DATA}`, the 127.0.0.1 GUI port, `/api/sessions`, `/api/links`, `/api/events`, plugin-root `hooks/hooks.json` and `.mcp.json`) |
| Infrastructure Patterns | Quiz II fields: 2, 9, 14 + pre-filled GitHub Actions 3-OS matrix |
| Cross-cutting Patterns | Quiz II fields: 7 (fail-fast exit when `BRIDGE_NAME` is absent), 12 (defensive parsing), 15 (refusal propagation) + Quiz I Development Style (agent-driven) + Quiz I budget-threshold config and capability ledger |
| Project Intent | [covered by Quiz I] (Growth Model: modular monolith, one binary, features grow as subcommands and verbs, no daemon) |
| Inherited Defaults | Derived from all Quiz I + Quiz II answers |
| Existing Scopes | Always "None" — new project |

## Pre-filled Values

- Primary language: Rust, stable (host 1.95) [inferred from: Quiz I Primary Language]
- Platform shape: one native CLI binary (`run · send · wait · hook · mcp · ui`, plus `verify · release · pause · unlink`) + a Claude Code plugin (hooks + stdio MCP) that calls the binary + a local web GUI served by the same binary [inferred from: Quiz I Platform]
- Growth model: modular monolith with the modules `pty · channel · hook · mcp · ui · state · agent::claude`, no daemon [inferred from: Quiz I Growth Model, Driven-Agent Coverage]
- Agent adapter: one concrete `agent::claude` module, no trait until a second agent exists. The core speaks viola's normalised events (turn-ended, prompt-submitted, question, permission, plan, session-end) [inferred from: Quiz I Driven-Agent Coverage]
- Development style: agent-driven [inferred from: Quiz I Development Style]
- CLI argument parser: clap 4.6.7 (research: "not a fork") [inferred from: Quiz I Platform + research]
- CLI conventions: human text by default, `--json` for agents, typed exit code per refusal, prompt text from stdin or `--file` (never leading-slash arguments, because of Git Bash path rewriting) [inferred from: Quiz I Wheel/Budget/Version posture + research]
- MCP refusal convention: tool result with `isError: true`, mapped onto the shared `RefusalReason` enum [inferred from: Quiz I refusal reasons + MCP spec]
- Hook contract: exit 0 + JSON means a decision; exit 0 with no body means no decision (the `unverified-cli` degrade path); exit 2 never happens by accident (fail-open); fast exit 0 when `BRIDGE_NAME` is absent [inferred from: Quiz I CLI Version Compatibility Posture]
- Hook transport family: exec-form `command` hooks. `http` and `mcp_tool` are excluded by the no-daemon rule and the SessionStart timing; only the sync/async split remains open (field 7) [inferred from: Quiz I Growth Model (no daemon)]
- External payload parsing: tolerant serde + serde_path_to_error, raw `Value` kept for fixtures [inferred from: Quiz I CLI Version Compatibility Posture]
- GUI surface: view-only; GET routes (`/api/sessions`, `/api/links`) + one live feed (`/api/events`); bound to 127.0.0.1; Host header allowlist `127.0.0.1:<port>` / `localhost:<port>`; no token or CSRF in v1 [inferred from: Quiz I Viewer Surface, GUI Control Scope]
- CI: GitHub Actions matrix on `windows-latest`/`windows-2025`, `macos-latest` (macOS 26) and `ubuntu-*`, with dtolnay/rust-toolchain and Swatinem/rust-cache v2.9.2; native runners, no cross-compiler; tests run against the fake agent [inferred from: Quiz I Cross-Platform Release Bar]
- Hosting / containers / serverless: N/A, local-only v1 [inferred from: Quiz I Scale Intent]
- Mobile framework: N/A. The phone view is a later version and will be the same web page behind authentication [inferred from: Quiz I Viewer Surface]
- LLM / AI infrastructure: none. Viola calls no model API and drives the unmodified `claude` CLI on the user's subscription [inferred from: Quiz I Core Functionality]
- Frontend framework: deferred to design specialist
- GUI brake auth (per-launch token, Origin checks) and auth generally: deferred to security specialist (v1.x)
- Test framework, fake-agent harness, `viola verify` probe suite design: deferred to tests specialist
- Logging / observability: deferred to obs specialist
- Self-update (self_update 1.3.0), signed binaries, winget/Homebrew/Scoop: deferred to a later version [inferred from: Quiz I Scale Intent]

## Quiz II Scope Note

Quiz II collects ARCHITECTURAL decisions: how to build it at arch level.
Each answer becomes an Established Decision in the architecture document.
Product decisions (what, for whom) were settled in Quiz I.

Specialist-domain decisions (auth library, test framework, logging/observability,
frontend framework / CSS tools / component libraries / visual tokens / typography,
a11y) are NOT in Quiz II — they're collected by specialist skills
(security / tests / obs / design / a11y) downstream.
