proposals:
  - detector: D-security-input
    severity: escalate
    section: Input Validation (the Child executable resolution row)
    change: "Replace \"`mcp`'s `list` is not built\" with: the `mcp` process's `list` tool makes one `claude agents --json` read per call, through CLI `list`'s board path (`list::board_document`, run on a blocking thread), its `agents-probe` pair written into `diagnostics/mcp.ndjson`; its output is the same untrusted, `MAX_FRAME`-bounded input and a row's `name` is never a `ViolaName` or `target` in the MCP `list` tool (the widening ratified by the founder, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers)."
    sidecar: "Input Validation, Child executable resolution: `mcp`'s `list` is built and makes the one `claude agents --json` read per call (chunk 2026-10-17-mcp-server-for-drivers, inputs#I2 Q1)."
    rationale: "Report, Spec claims disproved: `security-plan.md:213` \"`mcp`'s `list` is not built\" — the `list` tool makes the one `claude agents --json` read in the `mcp` process (pattern `mcp-list-built`; inputs#I2 Q1, ratified); Schema / config: new writers into `mcp.ndjson` include the `list` tool's `agents-probe` pair; Expected amendments name this site."
    basis: .andromeda/security-plan.md:213
  - detector: D-security-input
    severity: escalate
    section: Threat Model Summary (the `claude agents --json` spawn bullet)
    change: "Replace \"`mcp` spawns none (its `list` is not built)\" with: the `mcp` process spawns one per `list` tool call (chunk 2026-10-17-mcp-server-for-drivers; the founder's ratified widening, `inputs#I2` Q1), its `agents-probe` pair logged in `diagnostics/mcp.ndjson` under the same no-row-text, no-id, no-`cwd`, no-path rule."
    sidecar: "Threat Model Summary: `mcp` now spawns `claude agents --json` once per `list` tool call (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Spec claims disproved: `security-plan.md:110` (x2) \"`mcp` spawns none\" — the `list` tool makes the one read in the `mcp` process (pattern `mcp-list-built`); Expected amendments: `mcp-list-built` 2 hits in security-plan."
    basis: .andromeda/security-plan.md:110
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation (the MCP tool inputs row)
    change: "State the landed rule: each `viola-mcp` handler re-validates the raw argument object — `target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64` (`wait`'s `timeout_ms` clamped to `WAIT_CAP` 50 000 ms, also its default), `dialog_id` a `u64`, `response` an object through `Response::parse`, `validate_paste_text` on `send`'s text and every free-text answer value — and a failure is `isError: true` with `structuredContent` `{\"error\":\"invalid-params\",\"detail\":\"<field>\"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`), text `refused: invalid-params`, no frame written; an unknown tool name is JSON-RPC `-32602`."
    sidecar: "Input Validation, MCP tool inputs: the landed re-validation of every field, the `wait` clamp at 50 000 ms and the closed `invalid-params` shape (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Symbols / APIs (the tools): each handler re-validates the six fields as listed, a failure is `invalid-params` with a closed field `detail` and no frame; `WAIT_CAP` = 50 000 and `wait_timeout(given)` = min(given, 50 000); an unknown tool -> `-32602`. The row names `after` / `timeout_ms` but states no rule for them and no refusal shape."
  - detector: D-security-input
    severity: escalate
    section: Error Handling (External responses, the MCP `isError` bullet)
    change: "MCP `isError` results carry only the closed codes: `refusal`/`detail`, or `error` one of `instance-unreachable` (detail null) · `wrapper-fault` (detail the JSON-RPC fault's `{code,message,data}`) · `invalid-params` (detail a closed field word); every `isError` text is `refused: <code>`; any other handler fault, an unknown reply shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null`."
    sidecar: "Error Handling: the MCP `isError` code set gains `invalid-params`; an MCP handler fault is `-32603` `internal error`, data null (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Schema / config: a new MCP error code `invalid-params` beside `instance-unreachable` / `wrapper-fault`; Symbols / APIs: JSON-RPC fault -> `wrapper-fault` with `{code,message,data}`, any other outcome or a panic -> `-32603` `internal error`, data null; Expected amendments: `mcp-fault` 1 hit (security-plan:450-451) — the MCP fault is stated nowhere. The bullet's closed list (`instance-unreachable | wrapper-fault`) is retired by `invalid-params`."
    basis: .andromeda/security-plan.md:450-451
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Input Validation (the Paste text row, Where it runs)
    change: "Replace \"MCP when it lands\" with: the MCP `send` tool in `viola-mcp` — `validate_paste_text`, then the empty-typed-text check (`typed_text(..).is_empty()`), each refused as `{\"refusal\":\"not-delivered\",\"detail\":\"control-character\"|\"empty-text\"}` with one client-side `send-refused` line, before any frame."
    sidecar: "Input Validation, Paste text: the MCP `send` tool's client-side checks have landed (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Symbols / APIs: `send`: `validate_paste_text` then `typed_text(..).is_empty()` -> not-delivered control-character / empty-text and one client-side `send-refused` line; Outcome (security): empty / LF-only text `not-delivered` / `empty-text`, no frame."
  - detector: D-security-auth
    severity: warning
    section: Authentication & Authorization (the IPC client-side server verification row)
    change: "After the seventh dated gap add: Sixth interim gap here, the eighth dated gap overall (the founder's ruling, live in the overseer dialog, relayed by the overseer, 2026-10-17, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers): until the route entries \"Server verification before any frame\" and \"Home and code-bearing file integrity\" land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `{\"error\":\"instance-unreachable\",\"detail\":null}`), checking neither the serving process's identity nor the snapshot's strict-modes; they borrow none of the seven earlier gaps."
    sidecar: "Authentication & Authorization: the eighth dated gap — MCP frame tools after the liveness-only pre-check, founder-ratified (inputs#I2 Q1), owned by the two Epoch 6 entries (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Counts / qualifiers moved: dated liveness-only gaps 7 -> 8 (`security-plan.md:182` @c4797 \"the seventh dated gap overall\", @c5473 \"borrow none of the six earlier gaps\", and `:476`) — the eighth: MCP send / wait / last / answer frames after the liveness-only pre-check, borrowing none of the seven (inputs#I2 Q1); Symbols / APIs: frame tools use `live_endpoint` and one Tokio-client request; Decisions & corrections / inputs#I3 row 4: the closure a CARRY on the two Epoch 6 entries."
    basis: .andromeda/security-plan.md:182
  - detector: D-security-auth
    severity: warning
    section: Security Anti-Patterns -> Authentication (the channel-frame NEVER bullet)
    change: "Extend the dated exceptions with `viola mcp`'s `send` / `wait` / `last` / `answer` tool frames after the liveness-only pre-check (the eighth dated gap, the founder's ruling of 2026-10-17, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers), until the same two Epoch 6 entries."
    sidecar: "Security Anti-Patterns -> Authentication: the MCP frame tools join the dated exceptions as the eighth gap (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Counts / qualifiers moved: dated gaps 7 -> 8, pattern `dated-gap` 2 standing in security-plan: `:182` and `:476`; Expected amendments: security-plan §Security Anti-Patterns -> Authentication, the eighth dated gap."
    basis: .andromeda/security-plan.md:476
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: Input Validation (the CLI arguments row's interim gaps reading `snapshot.json` without strict-modes)
    change: "After the seventh (`viola pause` / `viola release`) add: an eighth (the founder's ruling, live, 2026-10-17, relayed by the overseer, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers), until the same entries: `viola mcp`'s `send` / `wait` / `last` / `answer` tools read it the same way (`viola_state::liveness::live_endpoint`)."
    sidecar: "Input Validation: the eighth interim gap — the MCP frame tools read `snapshot.json` through `live_endpoint` without strict-modes (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Symbols / APIs: frame tools use the liveness-only `live_endpoint` (moved to `viola_state::liveness`, callers incl. `viola-mcp`'s frame tools); Counts / qualifiers moved: the eighth dated gap. This list restates gaps four to seven for the CLI verbs' snapshot read and would otherwise omit the eighth."
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: Authentication & Authorization (the `~/.viola/` access control row, Strict-modes check entry points)
    change: "Where `mcp` is listed among the entry points, state what it checks: at start, the home and its `diagnostics/` (each that exists) through `viola_state::strict::check_path` before anything under the home opens (a refusal exits 1, nothing on stdout or stderr, no role file); its frame tools then read `snapshot.json`'s `endpoint` under the eighth dated gap, without the check."
    sidecar: "Authentication & Authorization, strict-modes entry points: `mcp`'s start check is the home and `diagnostics/`, its snapshot read the eighth gap (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Symbols / APIs: `src/cmd/mcp.rs` `mcp(home)` checks the home and its `diagnostics/` through `check_path` before anything opens, refusal exit 1 with nothing printed; frame tools read the endpoint through the liveness-only `live_endpoint` (the eighth gap). Listing `mcp` as an entry point that strict-checks before reading `snapshot.json` restates the claim the eighth gap retires."
    dependent-of: D-security-auth
  - detector: D-security-deps
    severity: escalate
    section: Dependency Security (`deny.toml` additions, the arch bans bullet)
    change: "The tokio ban: `deny-sync.toml` run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt`; `viola-channel`, whose Tokio client (`tokio = \"=1.53.1\"`, `io-util`) sits behind its `tokio` feature, is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line; control `--features tokio` must print one; CI supply-chain step `Tokio ban per sync crate (sole root)`), proven by `scripts/deny-probes.sh`'s `channel-tokio-edge` probe and its channel control (`13/13 banned, control clean`)."
    sidecar: "Dependency Security: `viola-channel`'s feature-gated tokio edge and its tree-form ban (`scripts/channel-tokio-free.sh`, deny-probes `channel-tokio-edge`); the sole-root loop holds four crates (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Dependencies: tokio =1.53.1, `viola-channel` `io-util`; Crates / modules: `viola-channel` gains `tokio = [\"dep:tokio\", \"interprocess/tokio\"]`; Counts / qualifiers moved: the sole-root loop's crate list 5 -> 4, `viola-channel` left `scripts/sync-crates.txt` for the tree form; Harness / gate surface: `channel-tokio-free.sh`, the supply-chain step, deny-probes' `channel-tokio-edge` probe and control. As written the ban puts tokio in no sync crate's graph by the sole-root form alone, which no longer covers `viola-channel`."
  - detector: D-security-deps
    severity: escalate
    section: Threat Model Summary (Trust boundary, the supply-chain bullet)
    change: "Name both tokio-ban forms: the sole-root `deny-sync.toml` tokio ban per sync crate and `viola-channel`'s `cargo tree` form (`scripts/channel-tokio-free.sh`), its Tokio client behind the `tokio` feature."
    sidecar: "Threat Model Summary, Trust boundary: the tokio ban adds `viola-channel`'s tree form (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Counts / qualifiers moved (sole-root loop 5 -> 4, `viola-channel` to the tree form) and Harness / gate surface (`scripts/channel-tokio-free.sh`). This bullet restates the sole-root form as the whole tokio ban."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: Threat Model Summary (CI, the Jobs bullet)
    change: "The supply-chain job's tokio ban reads the sole-root loop and `scripts/channel-tokio-free.sh`; the lint job runs `scripts/rmcp-features.sh` (rmcp's features in viola's release graph exactly `server transport-io`, non-vacuous) on every OS."
    sidecar: "Threat Model Summary, CI jobs: the channel tree form in supply-chain and the rmcp release-graph features check in lint (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Harness / gate surface: `ci.yml` lint job step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS) and the supply-chain step running `bash scripts/channel-tokio-free.sh` after its loop. The jobs bullet names only the sole-root tokio ban."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: API Security (the MCP transport row)
    change: "rmcp features exactly `server` and `transport-io` in `viola`'s release graph, checked by `scripts/rmcp-features.sh` in CI's lint job (rmcp must be present; the features a non-rmcp package requests must be exactly those two); the test-only `viola-e2e` (`publish = false`) takes rmcp `client` only (not `transport-child-process`); `transport-streamable-http-server` and `auth` stay banned in `deny.toml`."
    sidecar: "API Security, MCP transport: the release-graph scope of \"exactly server + transport-io\", its gate `scripts/rmcp-features.sh`, and `viola-e2e`'s test-only rmcp `client` (chunk 2026-10-17-mcp-server-for-drivers)."
    rationale: "Report, Dependencies: rmcp `>=3.4.1, <3.5` with `server` + `transport-io`; `viola-e2e` takes rmcp `client` (not `transport-child-process`); `scripts/rmcp-features.sh` reads the release graph's rmcp features exactly `server transport-io`. The row's unscoped \"exactly\" is not true of the workspace graph now that a member takes `client`, and it names no gate."
note: "No primary site carries a report-stated line for the Dependency Security, Paste text, MCP tool inputs, CLI-arguments gap list, strict-modes entry-point, Trust boundary, CI jobs or API Security proposals, so those name no line. New dependencies tokio =1.53.1 and schemars =1.2.2 are exact pins and rmcp's range and features match §Dependency Security's pinning and feature bans: no banned crate was added."
