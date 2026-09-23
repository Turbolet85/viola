## Backend Framework Comparison

Viola has no "backend" in the web sense. The choice that plays that role is the **concurrency model and the server stack inside one Rust binary**, which serves `run` (PTY pump), `hook` (spawned once per Claude Code hook event), `mcp` (stdio), `ui` (HTTP + SSE) and `send`/`wait`. The PTY layer is the part most likely to cause problems, so it is compared separately below. Versions were pulled live from the crates.io API on 2026-09-23.

**Option A: all-async, Tokio 1.53.1 (https://crates.io/crates/tokio) + axum 0.8.9 (https://crates.io/crates/axum) + rmcp 3.4.1 (https://crates.io/crates/rmcp)**
- Why it fits: rmcp (the official MCP Rust SDK) requires `tokio ^1` as a non-optional dependency. axum's `sse` feature gives the view-only SSE feed almost for free. interprocess 2.4.4 (https://crates.io/crates/interprocess) says Tokio is its only supported async runtime. So the three edge libraries all assume Tokio.
- Why it might not fit: portable-pty's reader and writer are blocking `std::io` handles, so the PTY pump still needs dedicated threads or `spawn_blocking`. You end up with two concurrency styles in `run` anyway. The `hook` path pays for runtime startup on every hook call, which is small but unmeasured on this host (brief O6).
- Status: Tokio released 2026-07-20 with about 231M recent downloads. axum 0.8.9 released 2026-04-14 (repo pushed 2026-09-23, 27.2k stars, MSRV 1.80). rmcp ships almost weekly: 3.0.0 on 2026-07-28, 3.4.1 on 2026-09-23, MSRV 1.88. The host's Rust 1.95 satisfies all of these.
- Deal-breaker check: none. Every MSRV is below 1.95.

**Option B: sync core, std threads only, tiny_http 0.12.0 (https://crates.io/crates/tiny_http) + a hand-rolled stdio JSON-RPC MCP server**
- Why it fits: fewest dependencies and the fastest cold start for `viola hook`. The PTY pump, named pipe and file tailing are naturally thread-per-stream. MCP stdio is newline-delimited JSON-RPC (https://modelcontextprotocol.io/specification), and three tools (`send`, `wait`, `last`) are a small surface.
- Why it might not fit: tiny_http's last stable release was 0.12.0 on **2022-10-06**, so it is effectively unmaintained. SSE keep-alive, the Host-header check and a clean shutdown would all be hand-written. A hand-rolled MCP server has to track spec revisions (2025-11-25 → 2026-07-28) on its own.
- Status: tiny_http has about 14M recent downloads but no release in almost 4 years. rouille 3.6.2 (https://crates.io/crates/rouille, built on tiny_http) was last released 2023-04-24.
- Deal-breaker check: tiny_http going stale is a maintenance risk for a GUI you plan to grow into a brake in v1.x (POST routes with Origin and token checks).

**Option C: hybrid. A sync `run`/`hook` fast path, with a Tokio runtime built only inside the `mcp`, `ui` and `wait` subcommands (same crates as A)**
- Why it fits: `hook` and the PTY pump stay plain threads with no runtime on the hot path. The async ecosystem (rmcp, axum, interprocess async) is used only where it pays off. This matches the "no daemon, each subcommand is its own process" shape.
- Why it might not fit: two I/O styles in one codebase. The channel/state layer must expose both a blocking and an async client, or `hook` must use interprocess's sync local-socket API while `wait` uses the Tokio one.
- Status: the same crates and versions as A. interprocess ships both sync and Tokio local sockets (2.4.4, 2026-09-03).
- Deal-breaker check: none. It costs some discipline in the `channel` module.

**Not viable**: smol 2.0.2 (https://crates.io/crates/smol, last release 2024-09-07). rmcp, axum and interprocess's async API are all Tokio-bound.

**CLI surface (not a fork, for completeness)**: clap 4.6.7 (https://crates.io/crates/clap, 2026-09-14) is the de facto parser for the `run · send · wait · hook · mcp · ui · verify · release · pause · unlink` verbs.

**PTY layer (the real fork inside "framework")**
- **portable-pty 0.8.1 pinned** (https://crates.io/crates/portable-pty/0.8.1, 2023-03-13). The spike measured it working on ConPTY (brief §4.1). Its Windows fixes are frozen: upstream main has since fixed `kill()` on Windows (2026-06-07) and moved to windows-sys (2026-08-25), and none of that is in 0.8.1.
- **portable-pty 0.9.0** (https://crates.io/crates/portable-pty, 2025-02-11, the only newer release). **Deal-breaker as released**: wezterm issue #6783 (https://github.com/wezterm/wezterm/issues/6783, filed 2025-03-11, still open) reports that `pty.read` returns garbage on Windows starting with 0.9.0, while 0.8.1 works.
- **portable-pty from wezterm git main** (https://github.com/wezterm/wezterm/tree/main/pty; repo active, pushed 2026-09-21). Gets the 2026 Windows fixes but no published release. A git dependency blocks a crates.io publish of viola later.
- **portable-pty-psmux 0.9.7** (https://crates.io/crates/portable-pty-psmux, 2026-08-18). Adds the ConPTY flags `PSEUDOCONSOLE_RESIZE_QUIRK`, `WIN32_INPUT_MODE` and `PASSTHROUGH_MODE` (passthrough only on Windows 11 22H2+). It is a single maintainer's patch set, not a GitHub fork, with about 2.7k recent downloads. Passthrough could matter for the O1 stray-spaces artifact, but that is untested.
- **Own ConPTY via windows-sys 0.61.2** (https://crates.io/crates/windows-sys), with a Unix PTY on rustix or nix 0.31.3 (https://crates.io/crates/nix). Full control over flags and exit detection (the spike found ConPTY does not close the output stream on exit, brief §4.1). This is roughly 500–1000 lines of platform code you own. pty-process 0.5.3 (https://crates.io/crates/pty-process) is Unix-only: it depends on rustix unconditionally.
- **Screen model, used only for readiness and modal detection, never for content**: vt100 0.16.2 (https://crates.io/crates/vt100, 2025-07-12, newer than the spike's 0.15.2) or alacritty_terminal 0.26.0 (https://crates.io/crates/alacritty_terminal, 2026-04-06). This is relevant because CLI-native modals bypass every hook (Quiz I, wheel amendment b).

## Database Comparison

What gets stored: an append-only event log per instance (the audit trail), small mutable state (wheel owner, budget pause, links, capability ledger stamps), and read-only consumers (`ui`, `wait`). Several **independent processes** write at once: the wrapper, N short-lived `viola hook` processes and the CLI verbs, with no daemon to serialise them. That rules some options in or out.

**Option A: ndjson append logs + atomically replaced JSON snapshots (plain files)**
- Why it fits: this is the brief's proposal (R4, "ndjson on disk"). It is human-readable, `tail`-able and survives restarts on both sides. The writes come from Rust `OpenOptions::append` (OS-positioned appends; docs: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html). Snapshots are replaced atomically via atomic-write-file 0.3.1 (https://crates.io/crates/atomic-write-file, 2026-08-11) or tempfile 3.27.0 `persist` (https://crates.io/crates/tempfile). Mutual exclusion uses std `File::lock`, stable since Rust 1.89, so no fs2/fs4 crate is needed.
- Why it might not fit: std docs say append mode "does not necessarily guarantee that data appended by different processes or threads does not interleave". Each event must be one `write` call, and readers must skip a torn last line (the brief's "parses defensively and heals itself"). A historical Windows problem, append plus exclusive lock returning "Access is denied" (rust-lang/rust#54118, closed), means the lock file should be separate from the log. There are no queries: the UI rebuilds its view by replaying or tailing.
- Status: std only, plus serde_json 1.0.151 (https://crates.io/crates/serde_json). Change detection via notify 8.2.0 (https://crates.io/crates/notify, 2026-08-30).

**Option B: SQLite in WAL mode via rusqlite 0.40.2 (https://crates.io/crates/rusqlite, bundled SQLite 3.53.2)**
- Why it fits: made for multiple processes writing and reading the same file with transactions. Wheel ownership and the budget pause become transactional, so there are no torn lines. The UI can query "sessions + links + last N events" directly.
- Why it might not fit: the `bundled` feature compiles C (MSVC on Windows, fine on this host but adds CI time). The file is not human-readable, and debugging the audit trail needs `sqlite3`. It departs from the brief's ndjson proposal and adds a schema-migration story.
- Status: 0.40.2 released 2026-08-08, about 35M recent downloads, actively maintained.

**Option C: redb 4.3.0 (https://crates.io/crates/redb, 2026-09-15), a pure-Rust embedded key-value store**
- Why it fits: no C toolchain, transactional, and fast.
- Why it might not fit / **deal-breaker for no-daemon**: redb has long returned `DatabaseAlreadyOpen` when a second process opens the file, and its multi-process safety has been an open topic (issues #678 and #932). The multi-writer mode is very new: PR #1462, which fixes lost commits and concurrent repairs in that mode, was merged 2026-09-06. With short-lived hook processes writing concurrently, this is the riskiest option. fjall 3.1.10 (https://crates.io/crates/fjall) is also single-process by design.
- Status: active, but its multi-process path is weeks old.

## Message Queue / Event System

No broker is needed. The requirement is **local IPC between per-instance wrappers and short-lived clients** (`hook`, `send`, `wait`, `mcp`), plus **event fan-out** to `ui` and `wait`.

**Option A: a per-wrapper local socket (named pipe on Windows, Unix socket elsewhere) via interprocess 2.4.4 (https://crates.io/crates/interprocess)**
- Why it fits: one API (`local_socket`) maps to named pipes on Windows and Unix domain sockets on Unix. It has sync and Tokio variants. Each wrapper owns its own endpoint, so there is no daemon. `hook` can make a blocking request and wait for the dialog answer. `wait` blocks on the socket until `Stop`, so nothing polls.
- Why it might not fit: the crate has one main maintainer (600 stars, release 2026-09-03). macOS limits Unix socket paths to about 104 bytes, so endpoint names must be short and derived from a hash rather than the full BRIDGE_NAME plus a path.
- Status: 2.4.x has been released regularly through 2026. The Tokio integration covers Windows named pipes and local sockets.

**Option B: Tokio-native endpoints behind your own `channel` trait**: `tokio::net::windows::named_pipe` plus `tokio::net::UnixListener`, both in Tokio 1.53.1 (https://docs.rs/tokio/1.53.1/tokio/net/windows/named_pipe/index.html)
- Why it fits: no extra dependency beyond Tokio, and first-party maintenance.
- Why it might not fit: async only, which forces Tokio into the `hook` fast path (see Framework option C). You write the cross-platform abstraction and the endpoint naming yourself.
- Status: stable parts of Tokio.

**Option C: file-based mailbox (the ndjson log as the bus) + notify 8.2.0 (https://crates.io/crates/notify) watchers**
- Why it fits: the log already exists and survives restarts. `ui` is a pure reader of disk anyway (Quiz I).
- Why it might not fit: a dialog answer needs a request/response round-trip inside the hook's timeout, which a file bus makes awkward. Watcher latency and coalescing vary by OS (ReadDirectoryChangesW, FSEvents, inotify). Good for `ui`, weak for `hook`, `send` and `wait`.

**Framing on the channel**: newline-delimited JSON (tokio-util 0.7.19 `LinesCodec`, https://crates.io/crates/tokio-util) matches the log and MCP stdio framing. Length-prefixed frames avoid any issue with embedded newlines but are harder to read by eye.

## Mobile Framework Comparison
N/A. Quiz I puts the phone view in a later version, as the same local web page behind authentication. There is no native mobile app.

## AI/ML Infrastructure

Viola hosts no model and calls no LLM API (D2: subscription only, through the unmodified `claude` CLI). Its "AI infrastructure" is the **Claude Code integration surface**: how hooks reach the binary, and how the MCP server is built. Sources: https://code.claude.com/docs/en/hooks and https://code.claude.com/docs/en/plugins-reference, fetched 2026-09-23.

**Hook transport**
- **`type: "command"` in exec form** (`"command": "viola", "args": ["hook", "pre-tool-use"]`):
  - Per the docs, when `args` is present the command is "spawned directly… No shell interpretation", so it skips Git Bash on Windows. That avoids the MSYS path rewriting the brief found (§6) and the cost of starting a shell.
  - Returns decisions via stdout JSON, including `permissionDecision` and `updatedInput`.
  - Costs one process spawn per event, and the latency is unmeasured (O6).
  - Every globally installed hook runs in every session, so the binary must exit 0 fast when `BRIDGE_NAME` is absent.
  - `SessionEnd` hooks share a **1.5 s budget**.
- **`type: "http"`** (POST of the event JSON, the same decision JSON back, default timeout 600 s, 30 s on `UserPromptSubmit`):
  - Removes the process spawn.
  - **Deal-breaker for no-daemon**: the docs describe env-var interpolation only for `headers` (via `allowedEnvVars`), not for `url`. A plugin's hook URL is static, so every session would POST to one fixed port, which amounts to a central listener. That contradicts "no central daemon".
  - Also subject to the `allowedHttpHookUrls` managed allowlist.
- **`type: "mcp_tool"`**: not usable for `SessionStart` at launch (the docs say it fires before MCP servers are available). That makes it unfit as the sole event channel.
- **`async: true` command hooks**: suit fire-and-forget events (`PostToolUse`, `Notification`) without blocking the turn. They cannot return decisions.

**MCP server (stdio, tools `send` / `wait` / `last`)**
- **rmcp 3.4.1** (https://crates.io/crates/rmcp, official SDK https://github.com/modelcontextprotocol/rust-sdk, 3.9k stars):
  - Implements MCP 2026-07-28 and stays compatible with 2025-11-25.
  - Features `server` + `transport-io` for stdio. `schemars ^1` is optional, for tool input schemas.
  - Churn risk: a 3.0.0 major on 2026-07-28 and 11 releases since.
- **rust-mcp-sdk 2.0.0** (https://crates.io/crates/rust-mcp-sdk, 2026-08-27): a community SDK that also implements 2026-07-28. It has about 112k recent downloads against rmcp's 14.9M, and its major versions move fast too (1.0 on 2026-07-26, 2.0 a month later).
- **Hand-rolled JSON-RPC over stdio**: three tools, newline-delimited messages, no SDK churn. You own the handshake and capability negotiation across spec revisions.

**Plugin packaging facts that constrain the design**
- `hooks/hooks.json` and `.mcp.json` sit at the plugin root.
- `${CLAUDE_PLUGIN_ROOT}` and `${CLAUDE_PLUGIN_DATA}` (`~/.claude/plugins/data/{id}/`, survives updates) are exported to hook and MCP processes.
- A plugin `bin/` directory is added to the Bash tool's PATH, so an overseer's Bash can run `viola send` bare. `bin/` is not allowed for plugins distributed via claude.ai organization settings.
- Component paths with backslashes are rejected on macOS and Linux.

## Push Notification / Real-time

This covers v1's view-only GUI feed. Phone push is a later version and out of scope.

- **SSE via axum 0.8.9 `sse` feature** (https://docs.rs/axum/0.8.9/axum/response/sse/). A one-way stream matches "GET routes plus an SSE event feed, no state-changing route". `Sse::keep_alive` is built in, and browsers reconnect by themselves via `EventSource`. One known gotcha: tower-http 0.7.1 (https://crates.io/crates/tower-http) `CompressionLayer` buffers `text/event-stream` unless you exclude it.
- **WebSocket via axum `ws` feature (0.8.9)**. Two-way, so it is ready for the v1.x brake. But a two-way channel for a read-only page widens the attack surface you would otherwise leave for the Host/Origin/token work, and origin handling differs from plain GET.
- **Plain polling** (the page re-fetches JSON every N seconds). Simplest to reason about and to verify with a headless browser. Laggier than the hooks, and it does more disk reads per viewer.
- **Server-side source for any of the above**: tailing the ndjson log with notify 8.2.0 (https://crates.io/crates/notify) versus re-reading on a timer. Session liveness comes from `claude agents --json` (brief §4) and/or sysinfo 0.39.6 (https://crates.io/crates/sysinfo) pid checks.

## Deployment & Infrastructure

Scale is personal, local-only v1 with no hosting. Docker, serverless and PaaS do not apply. The real choices are **how the binary and plugin reach the founder's machines** and **the 3-OS CI**.

**CI (all options): a GitHub Actions matrix**
- Runners:
  - `windows-latest`/`windows-2025`, moved to VS 2026 by default in June 2026 (https://github.blog/changelog/2026-05-14-github-actions-upcoming-image-migrations/); `windows-2022` still available.
  - `macos-latest`, which now points to macOS 26.
  - `ubuntu-*`: an `ubuntu-26.04` label is now available (https://github.blog/changelog/2026-06-11-new-runner-images-in-public-preview/).
- Toolchain and caching: dtolnay/rust-toolchain (https://github.com/dtolnay/rust-toolchain) and Swatinem/rust-cache v2.9.2 (https://github.com/Swatinem/rust-cache, 2026-08-06).
- Cross-compilers are not needed with native runners. cross 0.2.5 (https://crates.io/crates/cross) was last released 2023-02-04. cargo-zigbuild 0.23.4 (https://crates.io/crates/cargo-zigbuild) exists if you ever build Linux from one host.

**Option A: local build + PATH install; plugin loaded in place**
- `cargo install --path .` puts `viola` on PATH.
- The plugin comes from a local-directory marketplace, which "load[s] in place" per the plugin docs, or from `--plugin-dir` (the spike's method).
- Fits the v1 scope exactly. It stays "one binary, hooks call the binary" because hooks resolve `viola` from PATH.
- Risks: PATH drift between the terminal and Claude's hook environment on Windows, and version skew between the binary and the plugin (the plugin's `version` field is separate from the binary's).

**Option B: dist (cargo-dist) 0.33.0** (https://github.com/axodotdev/cargo-dist/releases/tag/v0.33.0, 2026-09-11; crates.io `cargo-dist` is at 0.32.0)
- Generates a GitHub Actions release workflow, per-OS archives, shell and PowerShell installers, and later Homebrew formulas.
- Actively maintained in 2026 (0.31.0 in Feb, 0.32.0 in May, 0.33.0 in Sep).
- It is the natural v1.x/public step. For v1 it is setup cost for something Quiz I defers (signed binaries, winget/Homebrew/Scoop).
- Pairs with cargo-auditable 0.7.6 (https://crates.io/crates/cargo-auditable) if you want dependency metadata embedded later.

**Option C: binary shipped inside the plugin (`bin/`)**
- A single install gives the Bash tool PATH access, so the overseer runs `viola send` bare. Hooks can reference `${CLAUDE_PLUGIN_ROOT}/bin/...`.
- But one plugin tree has to carry per-OS binaries (`viola.exe` and the Unix builds) and pick among them.
- `viola run` is launched by the human from a terminal outside Claude, so the binary still has to be on the user's PATH as well.
- Not allowed for claude.ai org-settings distribution.

**Upgrade path (later)**: self_update 1.3.0 (https://crates.io/crates/self_update, 2026-09-02) for in-binary updates once public releases exist.

**Monitoring and observability**: left to the obs specialist.

## API Style & Conventions

Viola exposes **five interfaces from one binary**, none of which is a classic REST/GraphQL/gRPC/tRPC service:
1. CLI verbs (clap 4.6.7)
2. MCP tools over stdio (JSON-RPC 2.0)
3. The hook stdin/stdout JSON contract, owned by Claude Code
4. The wrapper channel protocol over the local socket
5. The GUI's HTTP GET + SSE

GraphQL, gRPC and tRPC are irrelevant here: no remote clients, and no TypeScript or proto consumers.

**Wrapper channel protocol**
- **JSON-RPC 2.0 over ndjson.** The same shape as MCP, so `mcp` can be a thin adapter. It has standard error objects (`code`, `message`, `data`), and request ids help the `hook` → answer round trip.
- **A custom tagged-enum protocol** (`{"op":"send",...}` / `{"ok":false,"refusal":"human-typing"}`) via serde's internally tagged enums (serde 1.0.229, https://crates.io/crates/serde). Smaller, and the refusal reasons are first-class. You version it yourself: put a `v` field in every frame.

**CLI conventions**
- `--json` output for agents plus human text by default.
- Typed exit codes per refusal (`human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`).
- Text taken from stdin or `--file` (brief §6: Git Bash rewrites leading-slash arguments).

**MCP conventions**
- Per the MCP spec (https://modelcontextprotocol.io/specification), tool-level failures such as a refusal are returned as a tool result with `isError: true`. JSON-RPC errors are reserved for protocol faults.
- Map every refusal reason onto the same typed enum the CLI uses.

**Hook contract conventions**
- Exit 0 with a JSON body means a decision. Exit 0 with no body means "no decision", which is the `unverified-cli` degrade path: the dialog renders for the human.
- Exit code 2 is a blocking error in Claude Code's contract, so it must never be produced by accident (fail-open).

**HTTP (GUI)**: plain resource-style GETs (`/api/sessions`, `/api/links`, `/api/events` SSE), plus the Host-header allowlist from Quiz I.

**Error handling pattern (Rust ecosystem)**
- **thiserror 2.0.20** (https://crates.io/crates/thiserror). Typed enums per module, such as `RefusalReason` and `ChannelError`. This is the standard choice where callers must branch on the variant, and rmcp itself depends on it.
- **anyhow 1.0.104** (https://crates.io/crates/anyhow). Context-chained errors only at the binary edge (`main`, subcommand dispatch). This is the usual "thiserror in modules, anyhow in the bin" split.
- **snafu 0.9.2** (https://crates.io/crates/snafu). Context selectors carry typed context (instance name, path) without writing them by hand. It is heavier and less common.
- **miette 7.6.0** (https://crates.io/crates/miette, last release 2025-04-27). Rich terminal diagnostics for human-facing CLI errors. Largely irrelevant to agent-facing `--json` output.

## Validation Library

Viola does little form-style validation. It **parses external JSON it does not own** (hook payloads, `claude agents --json`, statusline `rate_limits`), which changes between CLI builds, and validates its own inputs (instance names, thresholds, channel frames).

**serde 1.0.229 + serde_json 1.0.151, tolerant mode** (https://crates.io/crates/serde, https://crates.io/crates/serde_json)
- No `deny_unknown_fields` on external payloads. Use `#[serde(default)]` and `Option<T>` for fields that may vanish, and keep the raw `serde_json::Value` alongside for the fixture recorder.
- Fits the "degrade on unverified builds" posture: parsing never breaks, and the capability ledger decides behaviour.
- Pair with serde_path_to_error 0.1.20 (https://crates.io/crates/serde_path_to_error) so drift is reported with the exact JSON path that broke.
- serde_with 3.23.0 (https://crates.io/crates/serde_with) adds helpers such as "default on parse error".

**serde, strict mode for viola's own formats**
- `deny_unknown_fields` on channel frames and ndjson events that viola writes. A mismatch between binary versions then fails loudly rather than silently.
- The trade-off: an older `ui` reading a newer log breaks, unless events carry a version and readers skip unknown event kinds.

**Newtype and domain validation**
- nutype 0.8.0 (https://crates.io/crates/nutype, 2026-09-20): validated newtypes, e.g. `BridgeName` (charset and length limits, which also keep socket paths short) and `Percent` (0–100).
- garde 0.23.0 (https://crates.io/crates/garde) or validator 0.21.0 (https://crates.io/crates/validator): derive-based struct validation. Mostly overkill for a handful of config fields.

**Schema for MCP and fixtures**
- schemars 1.2.2 (https://crates.io/crates/schemars): the version family rmcp's optional `schemars ^1` feature uses to emit tool input schemas.
- jsonschema 0.57.0 (https://crates.io/crates/jsonschema, 2026-09-21): could check that `viola verify`'s recorded hook-payload fixtures still match an expected schema.

**Time fields** (`resets_at`, event timestamps): jiff 0.2.37 (https://crates.io/crates/jiff) versus chrono 0.4.45 (https://crates.io/crates/chrono). chrono already comes in transitively through rmcp.

## Module Boundary Enforcement

The growth model is a modular monolith: one binary with the modules `pty · channel · hook · mcp · ui · state · agent::claude`. The rule to enforce is that nothing outside `agent::claude` knows Claude-specific shapes, and the PTY, channel and state layers know no agent.

**Option A: Cargo workspace, flat `crates/` layout** (matklad, https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
- Proposed crates: `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui`, plus the `viola` bin crate.
- Boundaries are enforced by the compiler: a crate cannot import a crate it does not list in `Cargo.toml`. `viola-pty` simply cannot see `viola-agent-claude`.
- Parallel and incremental builds help CI on three OSes.
- Cost: more manifests, and `pub` APIs between crates, which is more ceremony for a small codebase.

**Option B: single crate, `pub(crate)` / `pub(super)` visibility + clippy `disallowed_methods` / `disallowed_types` in `clippy.toml`** (built into clippy, https://rust-lang.github.io/rust-clippy/master/index.html#disallowed_methods)
- Lowest ceremony.
- Visibility stops outside access but not sideways imports inside the crate. `pty` can still `use crate::agent::claude::...` unless a review or lint catches it.
- clippy's disallow lists can forbid, for example, `std::process::Command` outside the spawn module.

**Option C: single crate + an architecture linter**
- cargo_pup 0.1.8 (https://github.com/DataDog/cargo-pup, crates.io 2026-06-09): module import rules as rustc lints. It **requires nightly**, which is friction for a stable 3-OS CI.
- modou 0.4.0 (https://github.com/tacticaldoll/modou, crates.io 2026-07-11): "like cargo-deny, but for architecture", with forbid and allowlist import rules. Very young (about 170 recent downloads).

**Supporting tools (any option)**
- cargo-deny 0.20.2 (https://crates.io/crates/cargo-deny): crate-level bans and licence checks. With a workspace it can also ban a crate from part of the tree.
- cargo-modules 0.27.0 (https://crates.io/crates/cargo-modules): visualises the module graph for review.

## Key Trade-offs Summary

1. **PTY layer**
   - Options: portable-pty **0.8.1 pinned** (spike-proven, frozen since 2023) vs **wezterm git main** (the 2026 Windows fixes, no release) vs **portable-pty-psmux 0.9.7** (modern ConPTY flags, single maintainer) vs **own ConPTY on windows-sys + Unix PTY**.
   - The stakes: whether Windows correctness is borrowed or owned. 0.9.0 has an open Windows garbage-read bug (#6783), so "just use latest" is not an option. A git dependency blocks a future crates.io publish.
2. **Hook transport**
   - Options: **exec-form `command` hook → `viola hook` → local socket** vs **`http` hook to a listener**.
   - The stakes: exec form keeps "no daemon" and skips Git Bash path mangling, but pays one process spawn per event (O6, unmeasured) and the 1.5 s `SessionEnd` budget. HTTP removes the spawn but needs a fixed URL, since per the docs only headers interpolate env vars. That fixed URL means a central listener, which reverses the no-daemon decision.
3. **Concurrency model**
   - Options: **all-Tokio** vs **sync core with Tokio only in `mcp` / `ui` / `wait`** vs **no Tokio** (hand-rolled MCP + tiny_http 0.12.0, stale since 2022).
   - The stakes: rmcp and axum pull in Tokio. The PTY pump stays blocking threads either way. The fork decides whether the hot `hook` path carries a runtime, and how many I/O styles the `channel` module must serve.
4. **State store**
   - Options: **ndjson logs + atomic snapshots + std `File::lock`** vs **SQLite WAL (rusqlite 0.40.2)**. redb 4.3.0 is weak for this, because its multi-process mode was only hardened on 2026-09-06.
   - The stakes: human-readable audit trail and zero C, at the cost of hand-written torn-line healing and replay-to-query, versus transactional multi-process writes and queries at the cost of a binary format and a C build.
5. **Module boundaries and distribution**
   - Options: **workspace crates** (compiler-enforced `agent::claude` isolation) vs **single crate + clippy disallow lists** (lighter, enforced by convention). Separately, for v1, a **PATH install + plugin loaded in place** vs **binary bundled in the plugin's `bin/`**.
   - The stakes: how strictly D1/R4's "nothing Andromeda- or Claude-specific leaks into the core" is guaranteed, and whether the binary and plugin versions can drift apart.
