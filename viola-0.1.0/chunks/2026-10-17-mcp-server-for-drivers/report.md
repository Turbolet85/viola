# Report — 2026-10-17-mcp-server-for-drivers

**Chunk:** MCP server for drivers — stdio send/wait/last/answer/list, CLI-parity results, code-only refusals
**Date:** 2026-10-17
**Commits:** `a902e6e chore(2026-10-17-mcp-server-for-drivers): operator pre-CI commit, for the run this chunk's verdict reads`
(the only commit since `last_wrap` 2026-10-17T08:50:30Z beyond `574ad96`, the last wrap's flip; basis `574ad969`, the
parent of the oldest pre-CI commit). Uncommitted after the push: records only (two implement run dirs' journal and
outcome, `evidence/{operator-pass.md, live-mcp.md, live-sessions.ndjson}`, `inputs/` I4–I5,
`viola-0.1.0/verification-matrix.json`, the friction log, this wrap's run dir).

## Changes (structured — detectors read this)

- **Files:** (basis: `git diff --name-only 574ad969` = 104 paths, of them 37 under `gate.py scope`'s set; the new text
  per file in `## New text, by line` below)
  - New: `crates/viola-mcp/{Cargo.toml, src/lib.rs, src/tools.rs, src/result.rs, tests/handlers.rs}`,
    `crates/viola-channel/src/tokio_client.rs`, `src/cmd/mcp.rs`, `crates/viola-e2e/tests/mcp_tools.rs`,
    `scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh`.
  - Modified: `Cargo.toml`, `Cargo.lock`, `crates/viola-channel/{Cargo.toml, src/lib.rs, src/client.rs, src/frame.rs}`,
    `crates/viola-state/src/liveness.rs`, `src/cmd/{client, list, mod, send, wait, last, answer, pause}.rs`,
    `src/main.rs`, `plugin/.mcp.json`, `crates/viola-agent-claude/src/lib.rs`, `crates/viola-e2e/{Cargo.toml,
    src/harness/session.rs}`, `.github/workflows/{ci.yml, windows-mutants.yml}`, `scripts/{sync-crates.txt,
    deny-probes.sh}`, `tests/{contract_lints, cli_output_plain}.rs`, `.config/nextest.toml`.
  - Records: `chunks/…/{scope-record.md, evidence/*, inputs/*}`, `viola-0.1.0/verification-matrix.json` (`v1-38`),
    `.andromeda/runs/2026-10-17T11-33-11-implement/`, `…T14-14-49-implement/`, the friction log.
- **Symbols / APIs:**
  - `viola mcp` — a hidden clap subcommand (`#[command(hide = true)]`, doc "Serve the MCP tools to a Claude Code session
    (run by the plugin, never by a person)"), no argument but the global `--home`; `src/cmd/mcp.rs` `mcp(home)`: the
    home and its `diagnostics/` (each that exists) through `viola_state::strict::check_path` before anything under the
    home opens — a refusal exits 1 with nothing on stdout or stderr and no role file; then obs init as `ObsProcess::Mcp`
    with `own_name()`, `process-start{subject:"self"}`, `viola_mcp::serve(home, own_name, board)`, `process-exit` on
    stdin EOF. Its dispatch arm's `DetailSink` is process `Mcp` when `VIOLA_NAME` parses.
  - `viola-mcp` crate (pub): `serve(home, own_name, list) -> Result<(), ServeError>` (one current-thread Tokio runtime,
    rmcp stdio transport, until stdin closes); `Tools::new` / `Tools::call(name, args)` (the transport-free handle the
    handler tests drive, the same `Server::call` `serve` answers with); `ListProvider = Arc<dyn Fn() -> Value + Send +
    Sync>`; `McpError::Internal` (Display `internal error`); `ServeError::{Runtime, Initialize, Session}` (fixed
    messages); `TOOL_NAMES` = `send, wait, last, answer, list`; `WAIT_CAP` = 50 000; `wait_timeout(given)` =
    `min(given, 50 000)`, default 50 000.
  - The tools: each handler takes the raw argument object and re-validates — `target` through `ViolaName::try_new`,
    `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`, `response` an object
    through `Response::parse` — a failure is `isError: true`, `structuredContent`
    `{"error":"invalid-params","detail":"<field>"}` (closed field words `target · text · after · timeout_ms ·
    dialog_id · response`), text `refused: invalid-params`, no frame. `send`: `validate_paste_text` then
    `typed_text(..).is_empty()` → `{"refusal":"not-delivered","detail":"control-character"|"empty-text"}` and one
    client-side `send-refused` line. `answer`: `validate_paste_text` on every free-text value. Frame tools: the
    liveness-only `live_endpoint`, else `{"error":"instance-unreachable","detail":null}`; one Tokio-client request with
    `from` = the server's own name when it has one; `wait` sends `timeout_ms = wait_timeout(given)` and `after` as given.
    Results: `ok` → `isError:false`, `structuredContent` the `ok` payload; refusal → `{refusal, detail}`; JSON-RPC fault →
    `{"error":"wrapper-fault","detail":{code,message,data}}`; `Connect`/`Closed`/`Io` → `instance-unreachable`; every
    `isError` text is `refused: <code>`; any other channel error, an unknown reply shape, or a `wait`/`last` refusal →
    the JSON-RPC error `-32603` `internal error`, `data: null`. An unknown tool name → JSON-RPC `-32602` (`unknown
    tool`). Each call runs as its own spawned task, a panic answered as the `-32603`. `list` runs the provider on a
    blocking thread and returns its document's `ok`, or `instance-unreachable` for the board's refusal.
  - `tools/list`'s result carries `ttlMs` 0 and `cacheScope` `private` (`tools_result()`): claude 2.1.287 negotiates
    protocol `2026-07-28` and drops every tool of a list result without them (live start 1, `evidence/live-mcp.md`).
  - Span `mcp.tool_call` (`skip_all`, field `tool`) wraps the Tokio client's `channel.request`; tool descriptions name
    no `release`, `pause`, `link` or `unlink`; server info `viola` `0.1.0`, capabilities tools only.
  - `viola_channel::tokio_client::TokioClient` (behind `viola-channel`'s `tokio` feature): `connect(endpoint, process)`,
    `conn()`, `request(method, params)`; the sync client's envelope, `MAX_FRAME`-bounded read, `ChannelError` classes,
    `channel-request` / `channel-response` lines and `channel.request` span; on Windows the SQOS + overlapped open
    through the shared `client::open_handle` (a busy pipe waited for on the calling thread, 100 ms steps, 2 s).
    Shared crate-private helpers, the sync client now calling them too: `next_conn`, `request_frame`, `reply_from`,
    `open_handle` (Windows), `busy_deadline`; `frame::framed`.
  - `viola_state::liveness::live_endpoint(instance_dir)` — moved unchanged from `src/cmd/client.rs`; callers
    `src/cmd/{send, wait, last, answer, pause}.rs` and `viola-mcp`'s frame tools.
  - `src/cmd/list.rs` `board_document(home) -> anyhow::Result<Value>` (scan → read → document, or the refusal document;
    the `mcp` provider's) and `refused_document()`; CLI `list` keeps its own order (scan, its log opened, read,
    document) and its output, exits and lines.
  - `src/main.rs` `Role::Mcp`: `role_of` reads a first word `mcp` (after `--home <dir>` / `--home=<dir>`) as `Mcp`;
    `exit_code` maps it like `Other`; `prints_internal_error` stays `Cli`-only.
  - `plugin/.mcp.json` = `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}` — the pinned path
    substituted at every start; the plugin's tool names read live `mcp__plugin_viola_viola__<tool>`.
  - `viola_e2e::harness::session::SessionOptions::unstamped` (default false; boot `stamp: !unstamped`).
- **Crates / modules:** added `viola-mcp` (edges `viola-core`, `viola-channel` with `tokio`, `viola-state`,
  `viola-agent-claude`, rmcp, schemars, tokio, serde, serde_json, thiserror, tracing; `[lints] workspace = true`;
  `fake-agent = []`); `viola-channel` gains features `tokio = ["dep:tokio", "interprocess/tokio"]` and
  `fake-agent = ["tokio"]` (was `[]`: a package-scoped mutation build now compiles and tests the Tokio client); the root
  bin gains `cmd::mcp`.
- **Dependencies:** `rmcp = ">=3.4.1, <3.5"` (default features off, `server` + `transport-io`; resolved 3.4.1);
  `tokio = "=1.53.1"` (default features off; `viola-mcp` `rt`, `viola-channel` `io-util`, its dev-dep `rt`;
  `viola-e2e` `rt` + `process`); `schemars = "=1.2.2"`; `viola-e2e` takes rmcp `client` (not `transport-child-process`).
  `cargo deny check` green; `scripts/rmcp-features.sh` reads the release graph's rmcp features exactly `server
  transport-io`.
- **Schema / config:** none new: `ObsProcess::Mcp`, `mcp.ndjson` and the schema's `process` value `mcp` stood before.
  New writers into `mcp.ndjson`: the `list` tool's `agents-probe` pair, the board's `state-recovered` /
  `parse-rejected` lines, the `config-json` rejections (fields already in the catalogue). A new MCP error code
  `invalid-params` beside `instance-unreachable` / `wrapper-fault`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** (each fact's pattern from `report-patterns.toml`; counts are the pre-pass sweep's
  `standing` figures, trail `cascade-2026-10-17-mcp-server-for-drivers-pre-pass.json`)
  - The sole-root tokio loop's crate list 5 → 4: `viola-channel` left `scripts/sync-crates.txt` for the tree form
    (patterns `sync-crates` 10 standing, `channel-cover` 3 standing). Stale rows: `architecture.md:316` (lists
    `viola-channel` among the sync crates), `registries/…/architecture/build-system.md:8` (@c886, @c1883 "covered through
    the sync roots that depend on it" — disproved, below), `…/ci-cd-approach.md:15`, `test-plan.md:399`; the other
    `sync-crates` rows (`architecture.md:43`, `test-plan.md:1021, 1022, 1062`, `project-directory-structure.md:8, 84`)
    state the list's role without naming `viola-channel` — true.
  - The `ci` profile's verify-driven 20 s kill-line binaries 20 → 21 (`binary(mcp_tools)`; pattern `kill-line`, 4
    standing): stale `registries/…/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` (@c1184, @c2417,
    @c3075 — its list and "twenty"); `test-plan.md:646, 878, 1089` true.
  - Dated liveness-only gaps 7 → 8 (pattern `dated-gap`, 2 standing): `security-plan.md:182` (@c4797 "the seventh
    dated gap overall", @c5473 "borrow none of the six earlier gaps") and `:476` — the eighth: MCP `send` / `wait` /
    `last` / `answer` frames after the liveness-only pre-check, borrowing none of the seven (`inputs#I2` Q1).
  - `viola-channel`'s Windows mutation scope 5 → 6 files (+ `tokio_client.rs`; pattern `win-mutants`, 18 standing — all
    18 state the dispatch-only workflow's role, none lists `viola-channel`'s files: true, a token shared).
  - No master site: the live start count (2 of a cap raised 1 → 2, `inputs#I4`) — an evidence fact.
  - Sweep reach: 20 patterns · 20 moved facts in this report, 20 with a pattern (+1, `invalid-params`, whose control
    did not fire: `control: invalid-params, owner-generic never fired over the seven masters' pre-pass text (a902e6e4)`
    — its master count is 0 on that basis) · owner pattern 3 standing · by line 7 lines · 11 offsets.
- **Dev-tool versions:** none — `claude` re-read at `2.1.287 (Claude Code)` by path before each live start; the bare
  name 2.1.296 on the dev host, unused.
- **Harness / gate surface:**
  - `scripts/channel-tokio-free.sh` (new): `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none`
    must read no `tokio v` line (exit 0 "nothing to print", or exit 101 "did not match" when tokio is not in the lock),
    its control `--features tokio` must read one; verdict `channel-tokio-free: ok`.
  - `scripts/rmcp-features.sh` (new): `cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe
    --prefix depth`; rmcp must be present (`rmcp-features: FAILED — rmcp is absent from viola's release graph`), and
    the features a non-rmcp package requests must be exactly `server transport-io`; verdict `rmcp-features: server
    transport-io`.
  - `ci.yml`: lint job step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS) after `Lint probes`;
    supply-chain step `Tokio ban per sync crate (sole root)` runs `bash scripts/channel-tokio-free.sh` after its loop.
  - `windows-mutants.yml`: `viola-channel`'s files gain `crates/viola-channel/src/tokio_client.rs`
    (`contract_windows_mutation_scope` requires every Windows-gated source listed).
  - `scripts/deny-probes.sh`: probe `channel-tokio-edge` (a throwaway workspace with a normal tokio edge must fail the
    tree form) and a channel control (the same edge behind a `tokio` feature must pass); reads `13/13 banned, control
    clean`.
  - `.config/nextest.toml`: `binary(mcp_tools)` on the `ci` profile's 20 s line.
  - `SessionOptions::unstamped` (above).
- **Cross-project / external claims:**
  - CI: `ci#39033242394` on `a902e6e4ae470765aa5aa55d7e926c570794af4f`, `push`, attempt 1, `completed/success`,
    green 15/15, wall 871 s (`evidence/operator-pass.md`).
  - The installed claude 2.1.287 CLI: its own MCP log for the server
    (`~/.cache/claude-cli-nodejs/<the driver dir>/mcp-logs-plugin-viola-viola/`, read live, not snapshotted) recorded
    `negotiatedProtocolVersion` `2026-07-28` and, at start 1, `tools/list failed` on `ttlMs` / `cacheScope`.
  - rmcp 3.4.1 source (`~/.cargo/registry/src/…/rmcp-3.4.1`, read): `TokioChildProcess::graceful_shutdown` kills its
    child 3 s after a close; a list result's `ttl_ms` / `cache_scope` default to absent.
  - `inputs.py verify` (this wrap): `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 5 · uncited 3 · unparsed 0`; each row, the three printed `UNCITED` cited here:
    - `I1 · message: the operator, the /andromeda-phase invocation, 2026-10-17T09:28Z · copy message · n/a` (cited
      scope.md / research.md).
    - `I2 · message: the founder, live in the overseer dialog, relayed by the overseer, 2026-10-17T11:18Z · copy
      message · n/a` (cited plan.md) — the four P4 forks: Q1 the eighth gap + MCP `list`'s agents read (widening
      ratified), Q2 one live start, Q3 gap-list group 2 only, Q4 a new switch entry, `v1-33` not claimed here.
    - `inputs#I3 · message: the operator, in the session, 11:30Z on 2026-10-17 · copy message · n/a` — the P5 word
      "yes" on `.andromeda/runs/2026-10-17T09-28-44-phase/p5-card-1.md`, the operator's authority. The card's
      `## Beyond the plan` rows, verbatim:
      1. **Route insertion** (`inputs#I2` Q3): a new markerless entry at the end of Epoch 5, after "Compact post-condition
         ledger row":
         "Dialog hold for the driver — a longer or configurable dialog hold and a record of an expired hold (gap-list
         group 2)."
      2. **Route insertion** (`inputs#I2` Q4): a new markerless entry directly after row 1:
         "Overseer switch to viola — the overseer drives through viola instead of the prototype; claims v1-33 with its
         acceptance re-worded to the Linux dev host and a key typed in the session terminal."
      3. **Owner for a part left out:** whether a registered `viola mcp` that exits 1 blocks the CLI's start, unmeasured
         here by the founder's one-start cap. Proposed as a CARRY on the existing entry "Optional plugin install", which
         holds the plugin-registration measurements.
      4. **Owner for the eighth dated gap's closure:** a CARRY on the existing entries "Server verification before any
         frame" and "Home and code-bearing file integrity", which close the other seven.
    - `inputs#I4 · message: the founder, live in the overseer dialog, relayed verbatim by the operator in the
      /andromeda-implement invocation, 2026-10-17T14:14Z · copy message · n/a` — «Ещё один старт»: the cap on live
      `claude` 2.1.287 starts rises 1 → 2; the founder's own word.
    - `inputs#I5 · message: the operator, the /andromeda-wrap-session invocation, 2026-10-17T14:19Z · copy message ·
      n/a` — this wrap in the two-window form (P1 only); every curation candidate into Decisions & corrections; the
      `pin_exe` race needs an owner at P5.
- **Reverted / negative API facts:** `crates/viola-e2e` first took rmcp `transport-child-process` (the plan's step 13)
  and dropped it: the transport kills the server 3 s after a close, and an instrumented `viola mcp` killed mid-exit
  left a truncated coverage profile (`viola-706996-…_20.profraw`, 114 688 B against 7361 siblings of 141 984 B) that
  `llvm-profdata` refused, reddening pre-push; the E2E now owns the child and awaits its exit. `McpArgs` (an empty
  clap args struct) written and removed (the variant is unit `Mcp`).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `registries/contracts/architecture/build-system.md:8` @c1883 — `viola-channel` "is covered through the sync roots
    that depend on it": no crate in `scripts/sync-crates.txt` has an edge into `viola-channel` (research.md, Mechanism
    re-derivations, `[premise-corrected]`, the code-graph crate-edges query); its own normal graph is now covered by
    the tree form (pattern `channel-cover`).
  - `test-plan.md:601` — the MCP handler tests run "using the sync channel client inside the handler under test": the
    handlers use the Tokio client (`viola-channel[tokio]`, architecture ×3; plan lean 6) and `handlers.rs` drives
    `viola_mcp::Tools` against a `viola_channel::Server` (pattern `tokio-client`).
  - `test-plan.md:618` — `viola-e2e` "uses dev-dep features `client` and `transport-child-process` only": it takes rmcp
    `client` only, as a normal dependency (viola-e2e carries no `[dev-dependencies]`), the child owned by the test
    (pattern `e2e-client`).
  - `architecture.md:96` @c2660 — "Until the `mcp` verb exists, `.mcp.json` ships `{"mcpServers": {}}`": it registers
    `viola` (pattern `mcp-json`).
  - `architecture.md:281`, `security-plan.md:110` (×2), `security-plan.md:213` @c566 — "`mcp`'s `list` is not built" /
    "`mcp` spawns none": the `list` tool makes the one `claude agents --json` read in the `mcp` process (pattern
    `mcp-list-built`; `inputs#I2` Q1, ratified).
  - `architecture.md:129` @c4539 — `role_of` "files every first word but `run`, …": it names `mcp` too (pattern
    `role-mcp`).
  - `architecture.md:46` @c855, `:58` — the MCP `wait` bound "is an open item": closed at a 50 000 ms cap and default
    (pattern `wait-cap`); live start 2 measured a default `wait` answered `timed-out` at 50 012 ms without a client
    timeout.
  - The owner rows naming this chunk as future (`test-plan.md:766` @c2276, `:1021`; `build-system.md:12`; pattern
    `owner-title`, 3 standing): landed.
  - `obs-plan.md:184` — `mcp.ndjson` "(tests excerpt) when `VIOLA_DIR` is …" (the plan's Expected amendments: the stale
    "stderr when `VIOLA_DIR` is unset"; pattern `mcp-ndjson`): `viola mcp` resolves its home through `resolved_home`
    and always writes `mcp.ndjson`.
- **Expected amendments (from plan):** (each with the pattern that located its sites; counts are `standing`)
  - security-plan §Authentication & Authorization / §Security Anti-Patterns → Authentication, the eighth dated gap —
    carried: Counts / qualifiers moved (dated gaps 7 → 8); `dated-gap` 2 hits in security-plan (`:182`, `:476`).
  - security-plan §Input Validation, Child process output, "`mcp` spawns none (its `list` is not built)" → "`mcp`'s
    `list` makes the one read" — carried: Spec claims disproved; `mcp-list-built` 2 hits in security-plan, 1 in
    architecture.
  - architecture [MCP]: the `wait` default and cap 50 000 ms; `invalid-params` with a closed field `detail`; an internal
    handler fault the JSON-RPC `-32603` `internal error`, data null; `mcp`'s `list` built — carried: Symbols / APIs,
    Spec claims disproved; `wait-cap` 2 hits in architecture; `invalid-params` 0 (control unfired, quoted above);
    `mcp-fault` 1 hit (security-plan:450-451, the channel's own `-32603` — a true claim sharing the token; the MCP fault
    is stated nowhere).
  - architecture §Conventions (Error handling schema, MCP bullet): `invalid-params` beside `instance-unreachable` /
    `wrapper-fault` — carried: Schema / config; `invalid-params` 0 standing (control unfired).
  - architecture §Standard Contracts: "`mcp`'s `list` is not built" → built — carried: `mcp-list-built`
    (`architecture.md:281`).
  - architecture → Build system: `viola-channel` covered by the `cargo tree` form, not "through the sync roots that
    depend on it" (`[premise-corrected]`); the rmcp release-graph check has landed — carried: Spec claims disproved,
    Harness / gate surface; `channel-cover` 3 hits (registries' build-system.md ×2 offsets, ci-cd-approach.md:15,
    architecture.md:316), `rmcp-features` 2 hits (`test-plan.md:1021`, `build-system.md:12` ×2).
  - architecture → Crate dependency direction: `viola-mcp` as landed, `viola-state`'s `live_endpoint` — carried: Crates
    / modules, Symbols / APIs; `tokio-client` 1 hit in `crate-dependency-direction.md:4`; `live-endpoint` 1 hit
    (`security-plan.md:182`, the name, still true).
  - architecture → CI/CD approach: the lint job's `rmcp-features.sh`, the supply-chain step's channel form — carried:
    Harness / gate surface; `channel-cover` `ci-cd-approach.md:15`, `win-mutants` 2 rows in `ci-cd-approach.md`.
  - obs-plan §6 Boundary-call wrappers: `mcp` writes the `agents-probe` pair; §3 Logging stack: the board's lines in
    `mcp.ndjson`; §1: the stale "stderr when `VIOLA_DIR` is unset" — carried: Schema / config, Spec claims disproved;
    `mcp-ndjson` 9 hits (obs-plan 3, architecture 2, test-plan 1, `logging-stack.md` 3).
  - test-plan §5 MCP: the Tokio client; §6 Path 3: the MCP error shape with `detail:null`; §9 Lint and Supply-chain: the
    two scripts — carried: Spec claims disproved (`test-plan.md:601`), Symbols / APIs, Harness / gate surface;
    `tokio-client` 6 hits, `rmcp-features` 2, `sync-crates` 10.
  - design-system §Surface: cli · Platform-Specific Notes, and layout-templates §Output structure — `viola --help`:
    `mcp` a hidden plugin-run verb — carried: Symbols / APIs; `mcp-hidden` 6 hits (`design-system.md:403`,
    `layout-templates.md:270` — both true, neither says hidden).
  - a11y-plan §8 Timeout extensions: the `wait` cap as a driver-only bound — carried: Symbols / APIs; `wait-cap` 0 hits
    in a11y-plan (its 2 hits are architecture's).
- **Coverage of new surfaces:**
  - `viola mcp` stdio server (5 tools) → validation ViolaName/u64/Response::parse/validate_paste_text✓ ·
    instrumentation `mcp.tool_call` span, `channel-*` lines, `send-refused`, `process-start`✓ · PII send text and
    responses in no span or line (asserted: `mcp_tools` reads `mcp.ndjson` free of the canary)✓ · tests unit
    (`tools.rs`, `result.rs`) / integ (`handlers.rs`) / e2e (`mcp_tools.rs`) + live (start 2) · a11y n/a · tokens n/a
  - `TokioClient` → validation `MAX_FRAME`-bounded read✓ · instrumentation `channel.request` span + lines✓ · PII
    params never logged✓ · tests unit (5 cases under the feature) · a11y n/a · tokens n/a
  - `viola mcp`'s start-time home check → validation strict-modes `check_path`✓ · instrumentation none by design (a
    refusal opens no log) · PII n/a · tests unit (`mcp.rs`, Unix group-writable) / e2e (Unix widened home) · a11y n/a
    · tokens n/a
  - `scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh` → known-verdict controls
    (`evidence/instrument-controls.md`, 2 + 3 inputs) and `deny-probes.sh`'s pair · a11y n/a · tokens n/a

## Deviations from intent

1. **Step 7, the board provider.** `list::board_document` (scan → read → document) is what the `mcp` provider calls
   whole; CLI `list` keeps its order — scan, its own log opened, read, document — and calls the shared `read`,
   `document` and `refused_document`. One function cannot hold CLI `list`'s log open between the scan and the read
   without a role parameter; CLI `list`'s output, exits and lines are unchanged (its cases green).
2. **Step 3, the tree form's reading.** Research measured exit 101 `did not match`; with rmcp and tokio in `Cargo.lock`
   the form exits 0 `nothing to print`. The script reads presence by a `tokio v` line and takes both absent forms
   (`evidence/instrument-controls.md`).
3. **Step 6, the list result.** `tools_result()` sets `ttlMs` 0 and `cacheScope` `private` — the plan did not foresee
   the `2026-07-28` requirement; live start 1 measured it.
4. **Step 12, the handler tests' entry.** `viola_mcp::Tools` (a public transport-free handle) and the test wrapper's
   snapshot written through `viola_state::snapshot::write_snapshot` (the writer, never hand-written bytes).
5. **Step 13, the E2E client** over an owned child (rmcp `client` only), not `transport-child-process` (Reverted /
   negative API facts).
6. **Step 13, scripted turns.** `path3.json`'s `PostToolUse.default` has no 2.1.287 fixture, so the Path 3 leg writes
   its own two-step script (`PostToolUse.parallel-1`, `Stop.default`, both recorded) beside its control file.
7. **Step 13, Path 7** boots `builder` alone (the `pin_exe` race, Decisions & corrections).
8. **Step 13, the kill line.** `binary(mcp_tools)` joined the 20 s line (the plan's condition); its cases read
   0.01–5.9 s; `path6_budget_harness`, also a stamped harness session, is not on that line.
9. **Step 15, the rig.** `live-drive.py` copied byte for byte and not run (it drives the release-check binary and its
   turn loop answers dialogs; this leg answers none); the verbs typed against the sessions' own `target/debug/viola`.
10. **Step 15, two starts.** The plan and `v1-38`'s concrete form say "the start count is 1"; the founder raised the cap
    to 2 (`inputs#I4`) after start 1 read red; start 2 read green.
11. **CARRY 1's companion.** `viola-channel`'s `fake-agent = ["tokio"]` and `windows-mutants.yml` gaining
    `tokio_client.rs` — required by `contract_windows_mutation_scope`.

Scope record (`gate.py scope` → `scope: clean — changed 37 · listed 28 · recorded 9 (companion 6 · mechanical 0 ·
in-intent 3 · widening 0) · absorbed 0 · excluded 74`, basis `574ad969`):
- companion (self): `.github/workflows/windows-mutants.yml` — serves `crates/viola-channel/src/tokio_client.rs`;
  `src/cmd/{answer, last, pause, send, wait}.rs` — serve `crates/viola-state/src/liveness.rs`.
- in-intent (self): `crates/viola-channel/src/client.rs`, `crates/viola-channel/src/frame.rs` — serve
  `tokio_client.rs`; `crates/viola-e2e/src/harness/session.rs` — serves `crates/viola-e2e/tests/mcp_tools.rs`.
- mechanical, widening: none.

## Decisions & corrections

1. **A live CLI can refuse what the protocol library's own client accepts.** claude 2.1.287 negotiated MCP protocol
   `2026-07-28` and refused a `tools/list` result without `ttlMs` and `cacheScope` (three retries, then "Failed to
   fetch tools"), dropping every tool; the E2E's rmcp 3.4.1 client parses the same result without complaint, so 23
   green gates and CI 15/15 could not see it. Evidence: `evidence/live-mcp.md` start 1, the CLI's
   `mcp-logs-plugin-viola-viola` log.
2. **Diagnostic source for a plugin MCP server under Claude Code:** the CLI logs each server at
   `~/.cache/claude-cli-nodejs/<cwd with / as ->/mcp-logs-plugin-<plugin>-<server>/<ts>.jsonl` (connect, negotiated
   version, list failures). Measured at start 1; it named the cause in one read.
3. **Plugin MCP tool names and their allow.** A plugin `viola`'s server `viola` exposes tools named
   `mcp__plugin_viola_viola__<tool>`; `--allowedTools mcp__plugin_viola_viola` (server level) allowed both called tools
   with no permission dialog (start 2's events). The MCP tools load deferred: the model reaches them through
   `ToolSearch` first (one `ToolSearch` activity in each start).
4. **rmcp's child-process transport kills.** `TokioChildProcess` closes the server's stdin and kills it if it has not
   exited within 3 s (its drop kills at once); an instrumented `viola mcp` killed mid-exit left a truncated profraw
   (114 688 B vs 141 984 B for 7361 siblings) that `llvm-profdata` refused, failing pre-push with every test green.
   Fix: the test owns the child and awaits its exit. That the cut profile was a `viola mcp` is the likely reading,
   not measured.
5. **The `pin_exe` race — needs an OWNER at P5 (`inputs#I5`, the operator).** Two first `viola run`s of one version
   starting together on a fresh home both find `bin/<key>/viola` absent and replace it; one agent's first hook spawn
   then fails (receipt `ran:false`) and its `session-start` never lands. Measured two-sided: 6 of 24 concurrent
   unstamped two-instance harness boots failed, 0 of 24 single-instance (`evidence/unstamped-boot-race.md`); the
   kernel error is not read (`ETXTBSY` a hypothesis). `viola_state::pin::pin_exe` is untouched by this chunk; a stamped
   home is pinned by `viola verify` first, so every stamped session reads clean. For P5: a route owner (a CARRY or a
   minted entry), the product race being the founder's own experience of two first starts; the test-side workaround
   (Path 7 boots one instance) stands.
6. **Fixture gap:** the 2.1.287 set has no `PostToolUse.default`; `fixtures/fake-scripts/path3.json` fires it only
   where a root test plants its own fixtures dir (`tests/cli_wait_last.rs`). A harness session over the repository's
   fixtures gets no activity line from `path3.json` (measured: no `activity` within 7 s).
7. **Harness defaults a test must not assume:** an unscripted fake agent never ends a turn (a sent prompt is read back
   and the `wait` times out); its `agents --json` lists only live wrapped agents, so an unwrapped row needs
   `agents-rows.json` planted beside the session's `claude` (measured: the first `mcp_tools` firing).
8. **`cargo tree -i <crate>` reads differently once the crate is in `Cargo.lock`:** exit 101 "did not match any
   packages" while absent from the lock, exit 0 "nothing to print" once another member pulls it in — an absence check
   reads the printed `<crate> v` line, never the exit (`scripts/channel-tokio-free.sh`; the research's measurement
   predated the lock change).
9. **Windows mutation scope contract:** a new `cfg(windows)` line in a product source fails
   `contract_windows_mutation_scope` until `windows-mutants.yml` lists the file; a feature-gated module's package needs
   its `fake-agent` feature to enable that feature, or the package-scoped mutation build compiles the file out
   (measured: pre-push red at the first firing).
10. **A red read to its cause before any re-run** (the standing rule), three times this chunk: the path7 timeout
    two-sided (6/24 vs 0/24), the coverage merge by the profile sizes, the `mcp_tools` premises by their logs.
11. **Rulings and words:** the founder's cap of one live start (`inputs#I2` Q2), raised to two by his own word «Ещё один
    старт» (`inputs#I4`), each start ledgered with the clock before it, a red never re-fired; the operator's run
    directions (tools bare, long runs journalled in the run dir before the wait, no mutation run of any kind, the
    operator pass with the `ci.py` read; at re-entry: re-read the run dir from disk first, no push unless a file a gate
    reads changes); this wrap's form (`inputs#I5`: P1 only, every curation candidate here, the `pin_exe` owner at P5).
    The P5 card's `## Beyond the plan` rows (`inputs#I3`, above) carry two route insertions and two CARRYs for P5.
12. **Guard refusals** (the Bash guard): a `cd` moving the cwd (twice, implement), a `cat` heredoc to a file (once,
    implement); each re-issued once in the guard's form (absolute paths, a subshell, the Write tool).
13. **A sweep hazard:** `invalid-params` and the generic owner phrase have no master site, so the pre-pass sweep
    refuses them (`control … never fired`); a zero for such a token is the refusal line, never a grep's 0.
14. **The live server logs no `process-exit`:** Claude Code ends `viola mcp` at the session's stop by a signal, not a
    stdin close, so `mcp.ndjson` holds a `process-start` and no exit in both starts.
15. **A Claude Code tool-call default over 50 s:** start 2's `wait` returned after 50 012 ms with no client timeout,
    so the 2.1.287 stdio tool-call default exceeds 50 s; research's `So=1e8` reading stays a hypothesis for the exact
    figure.

## Outcome

**Acceptance criteria** (re-asserted against the diff):
- (arch) `crates/viola-mcp` with exactly the arch edges + serde / serde_json / thiserror / tracing — met (Crates /
  modules); tokio in no other product member's normal graph — met (`channel-tokio-free: ok`; the four sync crates'
  sole-root loop green); `viola-mcp` not in `scripts/sync-crates.txt` — met; `deny-probes.sh` proves the tree form
  fires — met (`13/13 banned, control clean`).
- (tests) `rmcp-features.sh` green and in CI's lint job — met (`rmcp-features: server transport-io`; CI lint ×3 green).
- (obs) `role_of` classifies `viola mcp` / `viola --home <dir> mcp` as `Role::Mcp`; a failing `mcp` exits 1 with no
  `error: internal error`; its lines in `diagnostics/mcp.ndjson` with `process:"mcp"` — met (`role_of_…` rows,
  `prints_internal_error_…` rows, the widened-home E2E, live `mcp.ndjson`).
- (security) every input re-validated server-side; the four schema violations code-only with no `send-issued`; empty
  / LF-only text `not-delivered` / `empty-text` — met (`tool_inputs_are_revalidated_before_any_frame`,
  `handlers_schema_violations_write_no_frame`, `mcp_tools_list_…`).
- (arch, tests) each tool's success and refusal equal its CLI `--json` counterpart less `v`, with `isError` and
  `refused: <reason>` — met (`mcp_tools` Paths 2–7 and `list`, every refusal body compared with the CLI's document).
- (design, layouts) no result, body or description names `release`; `budget-paused` five-hour with no cursor — met.
- (security) no refusal or fault body holds a home path, `Caused by` or upstream text — met (`assert_sanitized`).
- (layouts, a11y) `viola --help` byte-equal to `HELP`, `mcp` in no row — met (`cli_output_plain`).
- (arch) the written `plugin/<version>-<hash>/.mcp.json` registers `viola` with the absolute pinned path and `["mcp"]`
  — met (`mcp_tools_list_…` reads it against the snapshot's `pinned_bin`).
- (arch, a11y) `wait`'s `timeout_ms` clamped to 50 000 ms, its default — met (`wait_timeout_is_clamped_to_the_cap`,
  `handlers_wait_sends_the_clamped_bound_…`; live: one frame `timeout_ms` 50000).
- (manual, live) `evidence/live-mcp.md`: a wrapped driver's `viola mcp` started, `instance` `driver`; its default
  `wait` one frame answered `timed-out` in 50–60 s (50 012 ms); the turn ended with the fixed word; no permission
  dialog — met at start 2. **"The live start count is 1": UNMET as written** — the count is 2, under the founder's
  raised cap (`inputs#I4`); routed to P2 (the plan's own text) and the matrix (`v1-38`'s concrete form names
  `evidence/live-mcp.md` and no count).
- (tests) every gate green and CI green on the final HEAD — met (below).
- `verification-matrix.json#v1-38` — `implemented`, ref the `mcp_tools` cases, the `viola-mcp` unit and handler
  cases and `evidence/live-mcp.md` start 2 (`.andromeda/runs/2026-10-17T14-14-49-implement/`); P7.3 flips it.

**Gates** (implement's fifth full firing, then `--only 3,23` on the final tree; trail
`.andromeda/runs/2026-10-17T11-33-11-implement/gate-2026-10-17-mcp-server-for-drivers.json`): `cargo fmt --all
--check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · the
`target/wincheck` Windows-target clippy green · the G1 `rg` (exit 1, no output) green · `bash scripts/lint-probes.sh`
green · `bash scripts/rmcp-features.sh` green (last line `rmcp-features: server transport-io`) · `bash
scripts/channel-tokio-free.sh` green (last line `channel-tokio-free: ok`) · `cargo deny check` green · the sole-root
loop over `scripts/sync-crates.txt` green · `bash scripts/deny-probes.sh` green · `run --unit` green · the filtered
unit run (`package(viola-mcp) | test(/role_of_|exit_code_maps_|live_endpoint_/)`) green · the filtered integration run
green · `run --integration` green · `run --e2e --filter 'binary(mcp_tools)'` green · `run --e2e` green · the four
smoke entries (cleanup, boot, status, cleanup) green · `g2-zero-panics.sh` green · `schema-check` green · `pre-push`
green (`"ok":true`, `"stage":"linux-tests"`, coverage `breaches []`) · `gate.py hygiene` (leg operator) — fired as
written, `hygiene: clean` · the push (leg operator) — `--operator 25` green, `history moved: …build/viola-0.1.0
574ad969→a902e6e4` · `ci.py conclusion --sha HEAD --wait 1800` (leg operator) — `verdict: green · checks 15/15 · wall
871 s · runs ci#39033242394 completed/success`, attempt 1 (`evidence/operator-pass.md`). No `defer`, no deferral, no
walk-class void. Earlier firings' reds, each fixed and named in Deviations / Decisions: the `mcp_tools` premises, the
Windows mutation scope contract, the path7 timeout, the truncated profile.

**Smoke:** the booted harness smoke (entries cleanup → boot → status → cleanup) green in every firing; the live leg
was the real boot of `viola mcp` under claude 2.1.287 — red at start 1, green at start 2.

**Watches:** none folded. Recorded, not a red: `contract_fake_agent_drift
fake_agent_dialog_replay_matches_every_recorded_dialog_set` read `SLOW > 10 s` (passed, 10.49 s) in the second full
E2E firing.

**Outcome basis:** the conversation holds every run of this chunk; the operator pass ran (Setup 4's commit list:
`a902e6e`); the gate verdicts rest on the final tree's re-run and CI `ci#39033242394` on `a902e6e4`. The two
`outcome-2026-10-17-mcp-server-for-drivers.md` files, in dir-name order —
`.andromeda/runs/2026-10-17T11-33-11-implement/` (SURFACED, its live criterion superseded by the next) and
`.andromeda/runs/2026-10-17T14-14-49-implement/` (GREEN + LIVE LEG) — hold the same facts.

**Process hygiene:** every process this chunk's runs started — the live legs' wrappers, fake agents, two `claude`
sessions and their `viola mcp`, the 48 reproduction boots, every gate entry's harness sessions, the session nextest
killed — terminated (both outcomes' census tables; re-measured at 2026-10-17T14:21:42Z: no process whose executable or
working directory lies under the repository or the `target/e2e-home` link's tmpfs base).
## Addendum — resumed 2026-10-17
- inputs#I6 · message: the operator, the /andromeda-wrap-session resume invocation · copy message · n/a — the live-start count 2 is the founder's own raised cap (`inputs#I4`), a recorded ruling, not an unmet criterion; the `pin_exe` race's owner at P5 is a CARRY on "Home and code-bearing file integrity" (it owns the home copy of the binary)
  - bears on **Outcome**: supersedes «"The live start count is 1": UNMET as written»
  - bears on **Decisions & corrections**: adds to it (entry 5, the `pin_exe` race's owner)
- inputs.py verify, re-fired: `inputs: 6 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 1 · unparsed 0`; moved since the report: a sixth entry (I6) snapped by this resume, cited here

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 574ad969 (the parent of the oldest pre-CI commit a902e6e4) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### .github/workflows/ci.yml — added 7 line(s) in 3 range(s)
added: 337-340 · 407-408 · 416
### .github/workflows/windows-mutants.yml — added 1 line(s) in 1 range(s)
added: 29
### Cargo.lock — added 269 line(s) in 26 range(s)
added: 121-126 · 151 · 261-266 · 362-386 · 393-426 · 439 · 441-443 · 445 · 539-540 · 561 · 564 · 723-733 · 939-944
       1189-1210 · 1265-1290 · 1333-1343 · 1453-1462 · 1481-1490 · 1631-1682 · 1781-1791 · 1856 · 1891 · 1916 · 1923
       1930-1948 · 2007-2012
- 367-375 «dependencies = [»
- 382-385 «dependencies = [»
- 398-402 «dependencies = [»
- 415-419 «dependencies = [»
- 728-732 «dependencies = [»
- 1194-1209 «dependencies = [»
- 1270-1277 «dependencies = [»
- 1284-1289 «dependencies = [»
- 1338-1342 «dependencies = [»
- 1458-1461 «dependencies = [»
- 1486-1489 «dependencies = [»
- 1636-1645 «dependencies = [»
- 1652-1656 «dependencies = [»
- 1663-1667 «dependencies = [»
- 1674-1681 «dependencies = [»
- 1786-1790 «dependencies = [»
- 1933-1947 «dependencies = [»
### Cargo.toml — added 11 line(s) in 5 range(s)
added: 142 · 200-201 · 219-221 · 223-224 · 236-238
### crates/viola-agent-claude/src/lib.rs — added 9 line(s) in 2 range(s)
added: 153-154 · 606-612
### crates/viola-channel/Cargo.toml — added 9 line(s) in 4 range(s)
added: 10-12 · 15-17 · 22 · 39-40
### crates/viola-channel/src/client.rs — added 73 line(s) in 8 range(s)
added: 46 · 61 · 66 · 71 · 121-174 · 237 · 249-261 · 301
- 121-126 @123 «pub(crate) fn next_conn(process: &str) -> String {»
- 128-154 @130 «pub(crate) fn request_frame(»
  - 142-152 «obs_event!(»
- 156-173 @157 «pub(crate) fn reply_from(»
  - 162-168 «let line = match exchanged {»
  - 250-252 «PipeStream::try_from(open_handle(endpoint, busy_until)?)»
### crates/viola-channel/src/frame.rs — added 6 line(s) in 1 range(s)
added: 27-32
### crates/viola-channel/src/lib.rs — added 4 line(s) in 2 range(s)
added: 3-4 · 12-13
### crates/viola-channel/src/tokio_client.rs — new file · 255 line(s)
- 22-26 «pub struct TokioClient {»
- 28-59 «impl TokioClient {»
  - 29-37 @30 «pub async fn connect(endpoint: &str, process: &'static str) -> Result<Self, ChannelError> {»
  - 39-42 @40 «pub fn conn(&self) -> &str {»
  - 44-58 @46 «pub async fn request(»
- 61-74 @62 «async fn exchange(»
  - 68-72 «reader»
- 76-86 @78 «async fn read_frame(reader: &mut (impl AsyncBufRead + Unpin)) -> Result<Vec<u8>, ChannelError> {»
  - 80-84 «(&mut *reader)»
- 88-93 @89 «async fn open(endpoint: &str) -> io::Result<Stream> {»
- 95-105 @98 «async fn open(endpoint: &str) -> io::Result<Stream> {»
  - 100-101 «let handle =»
  - 102-104 «PipeStream::try_from(handle)»
- 107-255 @108 «mod tests {»
  - 122-132 «impl Dispatch for Shapes {»
  - 134-139 «fn runtime() -> tokio::runtime::Runtime {»
  - 141-171 @142 «fn tokio_client_reads_each_reply_shape_and_counts_ids() {»
  - 173-182 @174 «fn tokio_client_connect_to_a_missing_endpoint_is_connect() {»
  - 184-200 @185 «fn raw_peer(endpoint: &str, reply: Option<Vec<u8>>) -> std::thread::JoinHandle<()> {»
  - 202-215 @203 «fn tokio_client_reads_a_peer_that_hangs_up_as_closed() {»
  - 217-231 @218 «fn tokio_client_refuses_a_reply_over_the_frame_bound() {»
  - 233-254 @234 «fn tokio_read_frame_takes_a_line_up_to_the_bound_and_closes_on_a_torn_one() {»
### crates/viola-e2e/Cargo.toml — added 3 line(s) in 1 range(s)
added: 34-36
### crates/viola-e2e/src/harness/session.rs — added 6 line(s) in 5 range(s)
added: 12-13 · 19 · 28 · 39 · 66
### crates/viola-e2e/tests/mcp_tools.rs — new file · 704 line(s)
- 24-46 @27 «fn scripted_builder(script: &str, steps: Option<&Value>) -> (tempfile::TempDir, PathBuf, String) {»
  - 32-39 «let script = match steps {»
  - 40-44 «let spec = format!(»
- 48-55 @49 «fn release(control: &Path, n: usize) {»
  - 50-54 «std::fs::OpenOptions::new()»
- 57-65 @61 «struct Mcp {»
- 67-70 «fn viola_exe(s: &HarnessSession) -> PathBuf {»
  - 68-69 «s.bin_dir()»
- 72-74 «fn session_bin(s: &HarnessSession) -> PathBuf {»
- 76-131 «impl Mcp {»
  - 77-110 @79 «fn spawn(s: &HarnessSession) -> Self {»
  - 112-114 «fn client(&self) -> &RunningService<RoleClient, ()> {»
  - 116-120 «fn tools(&self) -> rmcp::model::ListToolsResult {»
  - 122-130 «fn call(&self, name: &'static str, args: Value) -> CallToolResult {»
- 133-140 «impl Drop for Mcp {»
  - 134-139 «fn drop(&mut self) {»
- 142-146 «struct Ran {»
- 148-184 @150 «fn viola(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>, env: &[(&str, OsString)]) -> Ran {»
  - 152-166 «command»
  - 167-169 «for (key, value) in env {»
  - 171-177 «if let Some(bytes) = stdin {»
  - 179-183 «Ran {»
- 186-196 @187 «fn cli_json(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>) -> Value {»
  - 189-194 «assert!(»
- 198-203 @199 «fn less_v(stdout: &str) -> Value {»
- 205-210 «fn body(result: &CallToolResult) -> Value {»
  - 206-209 «result»
- 212-218 «fn text(result: &CallToolResult) -> String {»
  - 213-217 «result»
- 220-229 «fn keys(value: &Value) -> Vec<String> {»
  - 221-226 «let mut keys: Vec<String> = value»
- 231-239 @233 «fn assert_refused(result: &CallToolResult, reason: &str, detail: Option<&str>) {»
- 241-250 @242 «fn assert_sanitized(results: &[&CallToolResult], home: &Path) {»
  - 244-249 «for result in results {»
- 252-258 «fn events(instance_dir: &Path) -> Vec<Value> {»
  - 254-257 «text.split_inclusive('\n')»
- 260-270 @261 «fn wait_for_kind(instance_dir: &Path, kind: &str) {»
  - 263-269 «while count(instance_dir, kind) == 0 {»
- 272-277 «fn count(instance_dir: &Path, kind: &str) -> usize {»
  - 273-276 «events(instance_dir)»
- 279-386 @281 «fn mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli() {»
  - 289-292 «assert_eq!(»
  - 297-299 «for word in ["release", "pause", "link", "unlink"] {»
  - 314-326 «let violations = [»
  - 328-337 «assert_eq!(»
  - 339-349 @340 «for (text_in, at) in [("x\u{1b}[2J", 3), ("\n", 4)] {»
  - 350-354 «assert_eq!(»
  - 357-360 @358 «let snapshot: Value =»
  - 362-365 «let plugins: Vec<PathBuf> = std::fs::read_dir(s.home().join("plugin"))»
  - 367-368 «let registered: Value =»
  - 369-372 «assert_eq!(»
  - 378-381 «let started = lines»
- 388-394 «fn diagnostics(home: &Path, file: &str) -> Vec<Value> {»
  - 390-393 «text.split_inclusive('\n')»
- 396-425 @400 «fn mcp_tools_a_widened_home_exits_1_and_prints_nothing() {»
  - 409-418 «let out = Command::new(bin_dir.join("viola"))»
- 427-530 @431 «fn mcp_tools_path2_path3_send_wait_last_match_the_cli() {»
  - 432-436 @433 «let steps = json!([»
  - 438-445 «let s = harness_session(»
  - 449-452 «let sent = mcp.call(»
  - 473-476 «assert_eq!(»
  - 479-482 «let timed_out = mcp.call(»
  - 488-492 @489 «let request = diagnostics(&home, "mcp.ndjson")»
  - 494-499 «assert!(»
  - 500-501 «let shown =»
  - 508-517 «let cli_gone = Command::new(&exe)»
  - 519-522 «assert_eq!(»
  - 524-527 «assert_eq!(»
- 532-612 @535 «fn mcp_tools_path4_path5_answer_by_id_and_a_paused_send() {»
  - 538-545 «let s = harness_session(»
  - 552-554 «let dialog_id = raised["event"]["data"]["dialog_id"]»
  - 556-558 «let fixture = ws»
  - 559-560 «let fixture: Value =»
  - 561-564 «let asked = fixture["tool_input"]["questions"][0]["question"]»
  - 566-569 «let answered = mcp.call(»
  - 571-574 «let again = mcp.call(»
  - 577-582 «let cli_again = viola(»
  - 588-591 «let refused = mcp.call(»
  - 594-599 «let cli_refused = viola(»
  - 602-606 «assert!(»
- 614-665 @617 «fn mcp_tools_path6_budget_paused_send() {»
  - 618-625 «let s = harness_session(»
  - 627-634 «let usage = json!({»
  - 636-639 «let feed_env = [»
  - 640-645 «let fed = viola(»
  - 650-653 «let refused = mcp.call(»
  - 657-660 «assert_eq!(»
- 667-704 @669 «fn mcp_tools_path7_unverified_cli_answer() {»
  - 670-680 «let s = harness_session(»
  - 682-685 «let refused = mcp.call(»
  - 687-692 «let cli = viola(»
  - 695-698 «assert_eq!(»
### crates/viola-mcp/Cargo.toml — new file · 32 line(s)
### crates/viola-mcp/src/lib.rs — new file · 81 line(s)
- 25-32 @29 «pub enum McpError {»
- 34-43 @36 «pub enum ServeError {»
- 48-58 «impl Tools {»
  - 49-51 «pub fn new(home: PathBuf, own_name: Option<ViolaName>, list: ListProvider) -> Self {»
  - 53-57 @55 «pub async fn call(&self, name: &str, args: JsonObject) -> Result<CallToolResult, ErrorData> {»
- 60-81 @61 «pub fn serve(»
  - 66-69 «let runtime = tokio::runtime::Builder::new_current_thread()»
  - 70-80 «runtime.block_on(async move {»
### crates/viola-mcp/src/result.rs — new file · 251 line(s)
- 13-16 @14 «pub(crate) fn ok(payload: Value) -> CallToolResult {»
- 18-21 @19 «pub(crate) fn refused(reason: &str, detail: Option<&str>) -> CallToolResult {»
- 23-26 @24 «pub(crate) fn error(code: &str, detail: Value) -> CallToolResult {»
- 28-30 «pub(crate) fn unreachable() -> CallToolResult {»
- 32-35 @33 «pub(crate) fn invalid(field: &'static str) -> CallToolResult {»
- 37-41 «fn failed(code: &str, body: Value) -> CallToolResult {»
- 43-76 @45 «pub(crate) fn of_reply(»
  - 49-55 «let frame = match sent {»
  - 56-60 «if let Some(fault) = frame.get("error") {»
  - 62-64 «if let Some(payload) = result.get("ok") {»
  - 65-69 «let reason = result»
  - 70-74 «let detail = match result.get("detail") {»
- 78-88 @80 «fn fault_detail(error: &Value) -> Option<Value> {»
  - 83-86 «let data = match &error["data"] {»
- 90-99 @91 «pub(crate) fn of_board(document: &Value) -> Result<CallToolResult, McpError> {»
  - 92-94 «if let Some(payload) = document.get("ok") {»
  - 95-98 «match document.get("error").and_then(Value::as_str) {»
- 101-251 @102 «mod tests {»
  - 109-115 «fn text_of(result: &CallToolResult) -> String {»
  - 161-178 «fn of_reply_maps_each_reply_shape(»
  - 180-192 @184 «fn of_reply_reads_a_vanished_instance_as_unreachable(#[case] error: ChannelError) {»
  - 194-208 @202 «fn of_reply_reads_any_other_outcome_as_an_internal_fault(»
  - 210-232 @212 «fn result_bodies_hold_only_their_two_keys() {»
  - 234-250 @235 «fn of_board_takes_the_ok_member_or_the_unreachable_refusal() {»
### crates/viola-mcp/src/tools.rs — new file · 585 line(s)
- 11-15 «use rmcp::model::{»
- 38-41 @39 «pub fn wait_timeout(given: Option<u64>) -> u64 {»
- 43-50 @44 «enum ToolName {»
- 52-73 «impl ToolName {»
  - 53-62 «fn of(name: &str) -> Option<Self> {»
  - 64-72 «fn as_str(self) -> &'static str {»
- 77-84 @79 «struct SendInput {»
- 86-95 @88 «struct WaitInput {»
- 97-102 @99 «struct LastInput {»
- 104-114 @106 «struct AnswerInput {»
- 120-154 @121 «pub(crate) fn tool_list() -> Vec<Tool> {»
  - 122-153 «vec![»
- 156-163 @159 «pub(crate) fn tools_result() -> ListToolsResult {»
  - 160-162 «ListToolsResult::with_all_items(tool_list())»
- 165-170 @166 «struct Ctx {»
- 172-175 @173 «pub(crate) struct Server {»
- 177-183 «impl Server {»
  - 178-182 «pub(crate) fn new(home: PathBuf, from: Option<ViolaName>, list: ListProvider) -> Self {»
- 185-192 @186 «fn internal_fault() -> ErrorData {»
  - 187-191 «ErrorData::new(»
- 194-217 «impl ServerHandler for Server {»
  - 195-198 «fn get_info(&self) -> ServerConfig {»
  - 200-206 «async fn list_tools(»
  - 208-216 «async fn call_tool(»
- 219-240 «impl Server {»
  - 220-239 @222 «pub(crate) async fn call(»
- 242-251 @243 «async fn call(ctx: &Ctx, tool: ToolName, args: JsonObject) -> Result<CallToolResult, McpError> {»
  - 244-250 «match tool {»
- 256-261 «fn target_of(args: &JsonObject) -> Checked<ViolaName> {»
  - 257-260 «args.get("target")»
- 263-265 «fn text_of(args: &JsonObject) -> Checked<&str> {»
- 267-273 @268 «fn optional_u64(args: &JsonObject, field: &'static str) -> Checked<Option<u64>> {»
  - 269-272 «match args.get(field) {»
- 275-279 «fn dialog_id_of(args: &JsonObject) -> Checked<u64> {»
  - 276-278 «args.get("dialog_id")»
- 281-286 «fn response_of(args: &JsonObject) -> Checked<Response> {»
  - 282-285 «args.get("response")»
- 288-297 @290 «fn refusal_of(text: &str) -> Option<NotDelivered> {»
  - 291-293 «if let Err(detail) = viola_core::validate_paste_text(text) {»
  - 294-296 «typed_text(text)»
- 299-308 «fn refused_client_side(detail: NotDelivered) -> CallToolResult {»
  - 300-306 «obs_event!(»
- 310-316 @311 «fn params_from(ctx: &Ctx, mut params: Map<String, Value>) -> Map<String, Value> {»
  - 312-314 «if let Some(from) = &ctx.from {»
- 318-338 @319 «async fn frame(»
  - 326-329 «let instance_dir = ctx»
  - 330-332 «let Some(endpoint) = live_endpoint(&instance_dir) else {»
  - 333-336 «let sent = match TokioClient::connect(&endpoint, "mcp").await {»
- 340-351 «async fn send(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {»
  - 341-344 «let (target, text) = match (target_of(args), text_of(args)) {»
  - 345-347 «if let Some(detail) = refusal_of(text) {»
- 353-374 «async fn wait(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {»
  - 354-360 «let checked = (|| {»
  - 361-364 «let (target, after, timeout_ms) = match checked {»
  - 366-368 «if let Some(after) = after {»
  - 369-372 «params.insert(»
- 376-382 «async fn last(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {»
  - 377-380 «let target = match target_of(args) {»
- 384-405 «async fn answer(ctx: &Ctx, args: &JsonObject) -> Result<CallToolResult, McpError> {»
  - 385-386 «let checked =»
  - 387-390 «let (target, dialog_id, response) = match checked {»
  - 391-394 «let refused = response»
  - 395-400 «if let Some(detail) = refused {»
- 407-414 @408 «async fn list(ctx: &Ctx) -> Result<CallToolResult, McpError> {»
  - 410-412 «let document = tokio::task::spawn_blocking(move || provider())»
- 416-585 @417 «mod tests {»
  - 423-425 «fn args(value: Value) -> JsonObject {»
  - 427-432 «fn runtime() -> tokio::runtime::Runtime {»
  - 434-443 @435 «fn empty_home() -> (tempfile::TempDir, Ctx) {»
  - 445-459 @446 «fn tool_list_is_exactly_the_five_tools_in_order() {»
  - 461-468 @463 «fn tools_result_carries_its_ttl_and_cache_scope() {»
  - 470-478 @476 «fn wait_timeout_is_clamped_to_the_cap(#[case] given: Option<u64>, #[case] sent: u64) {»
  - 480-511 @500 «fn tool_inputs_are_revalidated_before_any_frame(»
  - 513-532 @520 «fn tool_inputs_that_pass_reach_the_liveness_check(»
  - 534-542 @535 «fn list_returns_the_providers_ok_member() {»
  - 544-554 @545 «fn list_provider_panic_is_an_internal_fault() {»
  - 556-564 @557 «fn tool_name_reads_only_the_five_names() {»
  - 566-575 @567 «fn params_from_carries_the_servers_own_name_when_it_has_one() {»
  - 577-584 @578 «fn internal_fault_is_the_fixed_jsonrpc_error() {»
### crates/viola-mcp/tests/handlers.rs — new file · 299 line(s)
- 16-21 @18 «struct TestWrapper {»
- 23-38 «impl Dispatch for TestWrapper {»
  - 24-37 «fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {»
- 40-46 @41 «struct Booted {»
- 48-57 «fn endpoint(dir: &Path, label: &str) -> String {»
  - 50-56 «if cfg!(windows) {»
- 59-99 «fn booted(label: &str, replies: Vec<(&str, Result<Value, ProtocolError>)>) -> Booted {»
  - 66-69 «*wrapper.replies.lock().expect("replies") = replies»
  - 70-72 «let serving = Server::bind(&endpoint)»
  - 74-90 «let snapshot = InstanceSnapshot {»
  - 93-98 «Booted {»
- 101-114 «impl Booted {»
  - 102-109 «fn tools(&self, from: Option<&str>) -> Tools {»
  - 111-113 «fn frames(&self) -> Vec<(String, Value)> {»
- 116-123 «fn call(tools: &Tools, name: &str, args: Value) -> Result<CallToolResult, rmcp::model::ErrorData> {»
  - 118-122 «tokio::runtime::Builder::new_current_thread()»
- 125-132 @126 «fn own_params(params: &Value) -> Value {»
  - 128-130 «for key in ["v", "sender", "conn"] {»
- 134-161 @135 «fn handlers_send_maps_the_readback_and_carries_from() {»
  - 136-142 «let wrapper = booted(»
  - 143-147 «let result = call(»
  - 150-153 «assert_eq!(»
  - 157-160 «assert_eq!(»
- 163-184 @164 «fn handlers_send_without_a_name_sends_no_from_and_maps_a_refusal() {»
  - 165-171 «let wrapper = booted(»
  - 172-176 «let result = call(»
  - 179-182 «assert_eq!(»
- 186-210 @188 «fn handlers_wait_sends_the_clamped_bound_and_maps_a_timeout() {»
  - 189-192 «let wrapper = booted(»
  - 194-209 «for (args, sent) in [»
- 212-248 @213 «fn handlers_last_and_answer_map_their_replies() {»
  - 214-226 «let wrapper = booted(»
  - 229-232 «assert_eq!(»
  - 233-237 «let answer = call(»
  - 239-242 «assert_eq!(»
- 250-272 @251 «fn handlers_map_a_wrapper_fault_and_a_wait_refusal_as_the_two_faults() {»
  - 252-258 «let wrapper = booted(»
  - 261-266 «assert_eq!(»
  - 268-271 «assert_eq!(»
- 274-299 @276 «fn handlers_schema_violations_write_no_frame() {»
  - 279-295 «for (name, args) in [»
### crates/viola-state/src/liveness.rs — added 22 line(s) in 4 range(s)
added: 5-6 · 11-12 · 71-82 · 150-155
- 71-81 @74 «pub fn live_endpoint(instance_dir: &Path) -> Option<String> {»
  - 77-79 «if classify(age, same_process(&snapshot)) != Liveness::Live {»
  - 150-154 @151 «fn live_endpoint_without_a_snapshot_is_none() {»
### plugin/.mcp.json — added 6 line(s) in 1 range(s)
added: 2-7
  - 2-7 «"mcpServers": {»
### scripts/channel-tokio-free.sh — new file · 42 line(s)
- 12-24 @13 «reading() {»
  - 17-18 «if printf '%s\n' "$out" | grep -q '^tokio v'; then»
  - 19-20 «elif [ "$rc" -eq 0 ] || printf '%s\n' "$out" | grep -qF 'did not match any packages'; then»
  - 21-23 «else»
- 30-33 «if [ "$own" = error ] || [ "$control" = error ]; then»
- 34-37 «if [ "$control" != present ]; then»
- 38-41 «if [ "$own" != absent ]; then»
### scripts/deny-probes.sh — added 34 line(s) in 2 range(s)
added: 69-93 · 106-114
- 69-79 @72 «make_channel_workspace() {»
- 86-88 «if [ "$rc" -ne 0 ] && grep -qF "channel-tokio-free: FAILED — viola-channel reaches tokio without its feature"…»
- 89-92 «else»
- 110-113 «if [ "$rc" -ne 0 ] || ! grep -qxF "channel-tokio-free: ok" "$channel_control/tree.log"; then»
### scripts/rmcp-features.sh — new file · 37 line(s)
- 16-23 «if ! printf '%s\n' "$out" | grep -q '^0rmcp v'; then»
  - 17-18 «if [ "$rc" -eq 0 ] || printf '%s\n' "$out" | grep -qF 'did not match any packages'; then»
  - 19-21 «else»
- 25-30 @27 «requested=$(printf '%s\n' "$out" | awk '»
- 33-36 «if [ "$requested" != "server transport-io" ]; then»
### scripts/sync-crates.txt — nothing added (lines removed only)
### src/cmd/answer.rs — added 3 line(s) in 1 range(s)
added: 20-22
### src/cmd/client.rs — added 4 line(s) in 1 range(s)
added: 1-4
### src/cmd/last.rs — added 3 line(s) in 1 range(s)
added: 15-17
### src/cmd/list.rs — added 19 line(s) in 2 range(s)
added: 77-94 · 98
- 77-88 @80 «pub(crate) fn board_document(home: &Path) -> anyhow::Result<Value> {»
  - 81-87 «Ok(match board::scan(home)? {»
- 90-93 @91 «fn refused_document() -> Value {»
### src/cmd/mcp.rs — new file · 81 line(s)
- 19-39 «pub(crate) fn mcp(home: &Path) -> anyhow::Result<ExitCode> {»
  - 20-24 @22 «if !home_passes(home) {»
  - 29-31 «if let Some(rejection) = rejection {»
  - 33-35 @34 «let board: ListProvider =»
- 41-50 @43 «fn home_passes(home: &Path) -> bool {»
  - 44-49 «[home.to_path_buf(), home.join("diagnostics")].iter().all(»
- 52-81 @53 «mod tests {»
  - 56-63 @57 «fn home_passes_a_new_home_and_refuses_an_unreadable_one() {»
  - 65-80 @67 «fn home_passes_refuses_a_group_writable_home_or_diagnostics_dir() {»
### src/cmd/mod.rs — added 14 line(s) in 3 range(s)
added: 6 · 84-86 · 170-179
### src/cmd/pause.rs — added 3 line(s) in 1 range(s)
added: 16-18
### src/cmd/send.rs — added 3 line(s) in 1 range(s)
added: 22-24
### src/cmd/wait.rs — added 3 line(s) in 1 range(s)
added: 15-17
### src/main.rs — added 21 line(s) in 7 range(s)
added: 82-83 · 90 · 102-103 · 136-140 · 358-362 · 380-383 · 401-402
### tests/cli_output_plain.rs — added 1 line(s) in 1 range(s)
added: 120
### tests/contract_lints.rs — added 3 line(s) in 1 range(s)
added: 79-81
  - 79-81 «for member in ["viola-core", "viola-mcp"] {»
