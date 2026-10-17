proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Workspace crates
    change: >-
      The "Landed so far" bullet lists every member as landed (`viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-mcp`, `viola-e2e`); the sync crates listed in `scripts/sync-crates.txt` are four, `viola-core`, `viola-pty`, `viola-agent-claude` and `viola-state` (`viola-channel` left the list at chunk 2026-10-17-mcp-server-for-drivers; its normal graph without its feature is proven tokio-free by `scripts/channel-tokio-free.sh`); `viola-pty`, `viola-agent-claude`, `viola-state`, `viola-mcp` and `viola-e2e` each declare a code-free `fake-agent = []`, while `viola-channel` declares `tokio = ["dep:tokio", "interprocess/tokio"]` (enabled by `viola-mcp`) and `fake-agent = ["tokio"]`, so a package-scoped `--features fake-agent` build compiles and tests the Tokio client.
    sidecar: >-
      Workspace crates: viola-mcp landed; viola-channel off the sync-crate list (4), its `tokio` feature and `fake-agent = ["tokio"]` registered.
    rationale: >-
      Report Changes, Crates / modules (viola-mcp added; viola-channel gains `tokio` and `fake-agent = ["tokio"]`, was `[]`) and Counts / qualifiers moved (sync-crate list 5 → 4, stale row `architecture.md:316`).
    basis: .andromeda/architecture.md:316
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Build system
    change: >-
      The code-graph rust plane's member list names `viola-mcp` beside the other members (`viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-mcp`, `viola-e2e`).
    sidecar: >-
      Build system: the rust code-graph plane's "every member" list gains viola-mcp.
    rationale: >-
      Report Changes, Crates / modules: `viola-mcp` added as a workspace member; the key's "indexes every member (…)" enumeration omits it.
    basis: .andromeda/registries/contracts/architecture/build-system.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Deployment / Distribution]
    change: >-
      Replace "Until the `mcp` verb exists, `.mcp.json` ships `{"mcpServers": {}}` (…)" with: since chunk 2026-10-17-mcp-server-for-drivers `.mcp.json` is `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}`, the pinned copy's path substituted at every start (the empty form held until the verb existed, on the operator ruling at the 2026-09-27-instance-state-and-start-order plan); whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured (the founder's start cap).
    sidecar: >-
      [Deployment / Distribution]: plugin `.mcp.json` now registers the `viola` server (`@@VIOLA_BIN@@ mcp`), no longer `{"mcpServers": {}}`.
    rationale: >-
      Report Symbols / APIs (`plugin/.mcp.json` = the `viola` server, pinned path substituted at every start) and Spec claims disproved (`architecture.md:96` @c2660, pattern `mcp-json`); inputs#I3 row 3 names the exit-1 question as left out.
    basis: .andromeda/architecture.md:96
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names
    change: >-
      MCP tools `send`, `wait`, `last`, `answer`, `list` reach a session as `mcp__plugin_viola_viola__<tool>` (plugin `viola`, server `viola`; as measured on 2.1.287 at chunk 2026-10-17-mcp-server-for-drivers, where `--allowedTools mcp__plugin_viola_viola` allowed them at server level with no permission dialog and the tools loaded deferred, reached through `ToolSearch`).
    sidecar: >-
      Claude Code integration names: the live MCP tool names `mcp__plugin_viola_viola__<tool>` registered.
    rationale: >-
      Report Symbols / APIs (`plugin/.mcp.json`: tool names read live `mcp__plugin_viola_viola__<tool>`) and Decisions & corrections 3.
    basis: .andromeda/architecture.md
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Error handling schema (MCP bullet)
    change: >-
      Replace "Only faults on the MCP connection itself are JSON-RPC errors" with: each handler re-validates its raw arguments (`target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`, `response` through `Response::parse`), a failure being `isError: true` with `structuredContent` `{"error":"invalid-params","detail":"<field>"}` (closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) and no frame; every `isError` text is `refused: <code>`; an unknown tool name is JSON-RPC `-32602`, and an internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a handler panic) is JSON-RPC `-32603` `internal error` with `data: null`; `tools/list`'s result carries `ttlMs` 0 and `cacheScope` `private`, without which claude 2.1.287 (negotiating protocol `2026-07-28`) drops every tool.
    sidecar: >-
      Conventions, MCP: `invalid-params` beside `instance-unreachable` / `wrapper-fault`; unknown tool `-32602`; internal fault `-32603` data null; `tools/list` `ttlMs` / `cacheScope`.
    rationale: >-
      Report Symbols / APIs (the tools' re-validation, result mapping, `-32602` unknown tool, `-32603` internal fault, `tools_result()`), Schema / config (new MCP error code `invalid-params`) and Expected amendments (§Conventions MCP bullet; `invalid-params` 0 standing).
    basis: .andromeda/architecture.md
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Session liveness
    change: >-
      Replace "`mcp`'s `list` is not built" with: since chunk 2026-10-17-mcp-server-for-drivers the `mcp` process's `list` tool makes the same one read — the provider `list::board_document` (scan → read → the agents read and join → document) run on a blocking thread, its `agents-probe` pair in `mcp.ndjson`, the board's refusal returned as `instance-unreachable` (the widening ratified, that chunk's `inputs#I2` Q1); the spawner list names it beside CLI `list`, `viola revive`'s preflight and verify's Run B.
    sidecar: >-
      Session liveness: MCP `list` built; it makes the one `claude agents --json` read in the `mcp` process.
    rationale: >-
      Report Spec claims disproved (`architecture.md:281`, pattern `mcp-list-built`) and Symbols / APIs (`board_document`, `list` runs the provider on a blocking thread).
    basis: .andromeda/architecture.md:281
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Channel methods (`list` bullet)
    change: >-
      The "As landed" sentence names MCP `list` beside CLI `list`: since chunk 2026-10-17-mcp-server-for-drivers MCP `list` builds the same board through `list::board_document` (the provider `viola mcp` hands `viola_mcp::serve`) and returns the document's `ok`, or `instance-unreachable` for the board's refusal; CLI `list` keeps its own order (scan, its log opened, read, document).
    sidecar: >-
      Channel methods, `list`: MCP `list` reads the board through `board_document`, not CLI `list` alone.
    rationale: >-
      Report Symbols / APIs (`board_document`, CLI `list` keeps its order) and Deviations 1; the bullet's as-landed text names CLI `list` as the only builder.
    basis: .andromeda/architecture.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      `src/run/agents.rs`'s comment reads "shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`)".
    sidecar: >-
      Project directory structure: run/agents.rs also serves the MCP `list` tool.
    rationale: >-
      Report Symbols / APIs (`board_document` is the `mcp` provider's) and Spec claims disproved (`mcp-list-built`: the `list` tool makes the one agents read).
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → CLI exit codes (exit 1)
    change: >-
      `role_of` files every first word but `run`, `revive`, `mcp`, `hook` and a leading `-` flag under `cli`; a first word `mcp` (after any `--home <dir>` / `--home=<dir>`) is `Role::Mcp`, whose exit maps like `Other` and which never prints `error: internal error` (`prints_internal_error` stays `Cli`-only).
    sidecar: >-
      CLI exit codes: `role_of` names `mcp` as `Role::Mcp`, outside the `cli` role's internal-error line.
    rationale: >-
      Report Symbols / APIs (`src/main.rs` `Role::Mcp`) and Spec claims disproved (`architecture.md:129` @c4539, pattern `role-mcp`).
    basis: .andromeda/architecture.md:129
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Binary, subcommands and exit codes
    change: >-
      The subcommands bullet adds: `mcp` is hidden from `viola --help` (`#[command(hide = true)]`, run by the plugin, never by a person), takes no argument but the global `--home`, serves the five tools over stdio until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` fails `viola_state::strict::check_path`.
    sidecar: >-
      Subcommands: `mcp` registered as a hidden plugin-run verb with its home-check exit 1.
    rationale: >-
      Report Symbols / APIs (`viola mcp` hidden clap subcommand, `src/cmd/mcp.rs` home check, refusal exits 1 printing nothing).
    basis: .andromeda/architecture.md
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables (`VIOLA_DIR`)
    change: >-
      Replace "the `mcp` verb is not built" with: `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) resolves its home through the same `resolved_home`, and reads `VIOLA_NAME` for its own name (the `from` it adds and its detail sink).
    sidecar: >-
      Environment variables: `VIOLA_DIR` / `VIOLA_NAME` now read by the built `mcp` verb.
    rationale: >-
      Report Spec claims disproved (obs-plan site, pattern `mcp-ndjson`: `viola mcp` resolves its home through `resolved_home`) and Symbols / APIs (`own_name()`, `DetailSink` when `VIOLA_NAME` parses).
    basis: .andromeda/architecture.md
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`diagnostics/`)
    change: >-
      `mcp.ndjson` has a producer since chunk 2026-10-17-mcp-server-for-drivers: `viola mcp`, once its home and `diagnostics/` pass `check_path` (a refusal opens no role file and writes no line), writes `process-start{subject:"self"}`, the Tokio client's `channel-request` / `channel-response` lines, one client-side `send-refused` per refused `send` text, the `list` tool's `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines (the `config-json` rejections included), and `process-exit` on stdin EOF (none when Claude Code ends it by a signal, as measured live on 2.1.287).
    sidecar: >-
      Filesystem, diagnostics: `mcp.ndjson`'s producer and lines registered.
    rationale: >-
      Report Schema / config (new writers into `mcp.ndjson`), Symbols / APIs (`process-start`, `channel-*` lines, `send-refused`) and Decisions & corrections 14 (no `process-exit` live).
    basis: .andromeda/architecture.md
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      The `viola-mcp` row reads as landed: → `viola-core`, `viola-channel[tokio]`, `viola-state`, `viola-agent-claude`, rmcp (default features off, `server` + `transport-io`), schemars, tokio (`rt`), serde, serde_json, thiserror, tracing; `[lints] workspace = true`; `fake-agent = []`.
    sidecar: >-
      Crate dependency direction: viola-mcp's edges as landed (+ serde / serde_json / thiserror / tracing).
    rationale: >-
      Report Changes, Crates / modules (viola-mcp's edges) and Dependencies (tokio `rt` for viola-mcp; rmcp features).
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      The `viola-state` row names `liveness::live_endpoint(instance_dir)` (moved unchanged from `src/cmd/client.rs` at chunk 2026-10-17-mcp-server-for-drivers), the liveness-only endpoint read shared by CLI `send` / `wait` / `last` / `answer` / `pause` and `viola-mcp`'s frame tools.
    sidecar: >-
      Crate dependency direction: viola-state hosts `live_endpoint`, shared by the CLI frame verbs and viola-mcp.
    rationale: >-
      Report Symbols / APIs (`viola_state::liveness::live_endpoint`, its callers) and Expected amendments (Crate dependency direction: `viola-state`'s `live_endpoint`).
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      The `scripts/` tree gains `channel-tokio-free.sh` (the `cargo tree -i tokio` form: `viola-channel`'s normal graph without its feature reads no `tokio v` line, its `--features tokio` control reads one; CI supply-chain) and `rmcp-features.sh` (viola's release graph requests rmcp features exactly `server transport-io`; CI lint job).
    sidecar: >-
      Project directory structure: the two new gate scripts registered.
    rationale: >-
      Report Changes, Files (new `scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh`) and Harness / gate surface.
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Module Boundaries]
    change: >-
      Replace "Only `viola-mcp` lists `tokio`, and `viola-channel`'s Tokio client sits behind a `tokio` feature only `viola-mcp` enables" with: among product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature (`["dep:tokio", "interprocess/tokio"]`, the `TokioClient`), enabled by `viola-mcp` and, in a package-scoped test build, by the channel's own `fake-agent = ["tokio"]`; the test-only `viola-e2e` takes `rt` + `process`; no other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`).
    sidecar: >-
      [Module Boundaries]: the tokio listing restated as landed (viola-channel optional feature, its fake-agent enabling it, viola-e2e test-only).
    rationale: >-
      Report Dependencies (tokio for `viola-mcp` `rt`, `viola-channel` `io-util`, `viola-e2e` `rt` + `process`) and Crates / modules (`fake-agent = ["tokio"]`); Outcome: tokio in no other product member's normal graph.
    basis: .andromeda/architecture.md
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies (Wrapper IPC row)
    change: >-
      "(sync API; Tokio flavour only in `viola-mcp`)" reads "(sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`)".
    sidecar: >-
      Stack, Wrapper IPC: the Tokio flavour sits in viola-channel's feature-gated TokioClient.
    rationale: >-
      Report Symbols / APIs (`viola_channel::tokio_client::TokioClient` behind the `tokio` feature, the shared `client::open_handle`) and Crates / modules.
    basis: .andromeda/architecture.md
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      The `viola-channel` row's "the Tokio client arrives behind the `tokio` feature with `viola-mcp`" reads as landed: behind `tokio = ["dep:tokio", "interprocess/tokio"]` (tokio `io-util`, dev-dep `rt`) sits `tokio_client::TokioClient` (`connect`, `conn`, `request`: the sync client's envelope, `MAX_FRAME`-bounded read, `ChannelError` classes, `channel-request` / `channel-response` lines and `channel.request` span, sharing the crate-private `next_conn`, `request_frame`, `reply_from`, `busy_deadline`, `frame::framed` and Windows `open_handle`), enabled by `viola-mcp` and by the crate's own `fake-agent = ["tokio"]`; without the feature its normal graph stays tokio-free.
    sidecar: >-
      Crate dependency direction: viola-channel's Tokio client landed behind its `tokio` feature.
    rationale: >-
      Report Symbols / APIs (TokioClient, shared helpers) and Crates / modules; Expected amendments name `crate-dependency-direction.md:4` (pattern `tokio-client`).
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:4
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Build system
    change: >-
      The `deny-sync.toml` ban covers `viola-core`, `viola-pty`, `viola-state` and `viola-agent-claude`, one sole-root run per crate in `scripts/sync-crates.txt`; the limit sentence's "so it is covered through the sync roots that depend on it" is replaced: no listed crate depends on `viola-channel`, so its normal graph is covered by `scripts/channel-tokio-free.sh` (`cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line, read as absent on exit 0 "nothing to print" or exit 101 "did not match"; its `--features tokio` control must print one; verdict `channel-tokio-free: ok`), run in CI's supply-chain step after the loop, and `deny-probes.sh`'s `channel-tokio-edge` probe and channel control prove it fires (`13/13 banned, control clean`).
    sidecar: >-
      Build system: viola-channel leaves the sole-root list; covered by the `cargo tree` form, not "through the sync roots that depend on it" (premise corrected).
    rationale: >-
      Report Spec claims disproved (`build-system.md:8` @c1883, `[premise-corrected]`), Counts / qualifiers moved (5 → 4, stale @c886) and Harness / gate surface (the script, the deny-probes pair).
    basis: .andromeda/registries/contracts/architecture/build-system.md:8
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Concurrency / Backend Framework]
    change: >-
      The KEYSTONE's enforcement sentence adds: `viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list; its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in the same CI supply-chain step.
    sidecar: >-
      [Concurrency] KEYSTONE: enforcement names the channel's `cargo tree` form beside the per-crate sole-root ban.
    rationale: >-
      Report Counts / qualifiers moved (viola-channel left `scripts/sync-crates.txt` for the tree form; `architecture.md:43` states the list's role) and Harness / gate surface (supply-chain step); without it the keystone reads as if the listed-crate ban alone covered every sync-side crate.
    basis: .andromeda/architecture.md:43
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: >-
      Target job 3's set is `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude` (the four in `scripts/sync-crates.txt`), proving the sync crates compile on each OS with no tokio; `viola-channel`'s no-feature graph is the supply-chain step's `scripts/channel-tokio-free.sh`.
    sidecar: >-
      CI/CD approach: job 3's sync-crate set drops viola-channel (4).
    rationale: >-
      Report Counts / qualifiers moved (stale row `ci-cd-approach.md:15`, pattern `channel-cover`).
    basis: .andromeda/registries/contracts/architecture/ci-cd-approach.md:15
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: >-
      The `supply-chain` job's step list reads "the sole-root `deny-sync.toml` tokio ban per listed sync crate, then `bash scripts/channel-tokio-free.sh` (step `Tokio ban per sync crate (sole root)`)".
    sidecar: >-
      CI/CD approach: the supply-chain tokio step runs the channel tree form after its loop.
    rationale: >-
      Report Harness / gate surface (`ci.yml` supply-chain step `Tokio ban per sync crate (sole root)` runs `bash scripts/channel-tokio-free.sh` after its loop).
    basis: .andromeda/registries/contracts/architecture/ci-cd-approach.md
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Build system
    change: >-
      Replace "has no subject until `viola-mcp` exists; it lands with the 'MCP server for drivers' chunk" with: the rmcp release-graph check is `scripts/rmcp-features.sh` (`cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`; rmcp must be present, and the features a non-rmcp package requests must be exactly `server transport-io`; verdict `rmcp-features: server transport-io`), run in CI's lint job on every OS after `Lint probes` (landed at chunk 2026-10-17-mcp-server-for-drivers).
    sidecar: >-
      Build system: the rmcp release-graph check landed as `scripts/rmcp-features.sh` in the lint job.
    rationale: >-
      Report Harness / gate surface (`scripts/rmcp-features.sh`, the `ci.yml` lint step) and Spec claims disproved (owner rows naming this chunk as future, `build-system.md:12`, pattern `owner-title`).
    basis: .andromeda/registries/contracts/architecture/build-system.md:12
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: >-
      The `lint` job's step list adds `rmcp features (exactly server + transport-io, non-vacuous)` (`bash scripts/rmcp-features.sh`, every OS) after `Lint probes`.
    sidecar: >-
      CI/CD approach: lint job gains the rmcp features step.
    rationale: >-
      Report Harness / gate surface (`ci.yml`: lint job step after `Lint probes`, every OS) and Expected amendments (CI/CD approach: the lint job's `rmcp-features.sh`).
    basis: .andromeda/registries/contracts/architecture/ci-cd-approach.md
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Build system
    change: >-
      The `[workspace.dependencies]` sentence reads "rmcp minor pinned as `>=3.4.1, <3.5` (default features off; resolved 3.4.1)" in place of "`~3.4`".
    sidecar: >-
      Build system: rmcp's workspace requirement recorded as landed, `>=3.4.1, <3.5`.
    rationale: >-
      Report Dependencies: `rmcp = ">=3.4.1, <3.5"` (default features off, resolved 3.4.1); the key's `~3.4` admits 3.4.0.
    basis: .andromeda/registries/contracts/architecture/build-system.md
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [MCP]
    change: >-
      Replace "The default `timeout_ms` for MCP `wait` is an open item and must stay below the MCP client's tool-call timeout" with: MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`, `wait_timeout`), 50 000 ms when none is given, and `after` as given; the server adds `from` only when it has a name of its own. The bound sits below claude 2.1.287's stdio tool-call default, which exceeds 50 s (as measured live at chunk 2026-10-17-mcp-server-for-drivers: a default `wait` answered `timed-out` at 50 012 ms with no client timeout; the exact default unmeasured).
    sidecar: >-
      [MCP]: the `wait` open item closed — cap and default 50 000 ms.
    rationale: >-
      Report Symbols / APIs (`WAIT_CAP` = 50 000, `wait_timeout(given)`), Spec claims disproved (`architecture.md:46` @c855, `:58`, pattern `wait-cap`) and Decisions & corrections 15.
    basis: .andromeda/architecture.md:58
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: >-
      "On the MCP surface that total must stay below the MCP client's tool-call timeout" adds: which on claude 2.1.287 exceeds 50 s (as measured live at chunk 2026-10-17-mcp-server-for-drivers), above the 18.5 s total.
    sidecar: >-
      [Delivery Confirmation]: the MCP client tool-call timeout's measured floor recorded beside the send total.
    rationale: >-
      Report Spec claims disproved (`architecture.md:46` @c855, pattern `wait-cap`) and Decisions & corrections 15 (start 2's `wait` returned after 50 012 ms with no client timeout).
    basis: .andromeda/architecture.md:46
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies (MCP server row)
    change: >-
      The row adds: the release graph's rmcp features are exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); the test-only `viola-e2e` takes rmcp `client` alone (never `transport-child-process`, which kills the server 3 s after a close), its E2E owning the `viola mcp` child and awaiting its exit.
    sidecar: >-
      Stack, MCP server: rmcp `client` recorded as viola-e2e's test-only feature; release graph exactly server + transport-io.
    rationale: >-
      Report Dependencies (`viola-e2e` takes rmcp `client`, not `transport-child-process`) and Reverted / negative API facts (the transport's 3 s kill truncated a coverage profile).
    basis: .andromeda/architecture.md
  - detector: D-arch-decisions
    severity: escalate
    section: §Established Decisions → [Error Handling]
    change: >-
      The one-enum-per-crate rule records a second open divergence beside `viola-agent-claude`'s `Refusal`: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`); its fold into `McpError` or its ratification is open.
    sidecar: >-
      [Error Handling]: viola-mcp's `ServeError` beside `McpError` recorded as an open divergence from one enum per crate.
    rationale: >-
      Report Symbols / APIs: `McpError::Internal` and `ServeError::{Runtime, Initialize, Session}` are two error enums in one crate, against the locked "one enum per workspace crate"; escalated because fold vs ratification is a decision, not a text fix.
    basis: .andromeda/architecture.md
  - detector: D-arch-decisions
    severity: escalate
    section: §Conventions → Error handling schema (Rust error types)
    change: >-
      The Rust error types bullet names `viola-mcp`'s `ServeError` beside `McpError` as the second open divergence, with `viola-agent-claude`'s `Refusal`.
    sidecar: >-
      Conventions, Rust error types: ServeError named as an open divergence.
    rationale: >-
      Same claim as the [Error Handling] proposal ("one error enum per crate, named `<Crate>Error`"), restated in Conventions; report Symbols / APIs lists both enums.
    basis: .andromeda/architecture.md
    dependent-of: D-arch-decisions
note: >-
  Not proposed, left to the orchestrator: build-system.md's Licence bullet lists the members inheriting `license.workspace = true` without viola-mcp (the report does not state viola-mcp's licence key, so it was not asserted); the `send` / `wait` / `last` / `answer` MCP frames after the liveness-only pre-check (the eighth dated gap) are security-plan's, not architecture's; whether the plugin-precedence and MCP `ttlMs` / `cacheScope` CLI shapes need capability-ledger rows is a policy question the report does not raise.
