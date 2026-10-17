proposals:
  - detector: D-tests-framework
    severity: warning
    section: '§5 Integration Test Strategy → Cross-module patterns covered → MCP'
    change: 'MCP: adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper` implementing `Dispatch`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client (`viola_channel::tokio_client::TokioClient`, `viola-channel`''s `tokio` feature), never the sync client. The full stdio path is E2E (§6).'
    sidecar: 'test-plan §5 MCP: the handler tests drive `viola_mcp::Tools` with the Tokio channel client against a `viola_channel::Server` test wrapper (was: the sync channel client inside the handler) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Spec claims disproved names `test-plan.md:601` ("using the sync channel client inside the handler under test"): the handlers use the Tokio client and `handlers.rs` drives `viola_mcp::Tools` against a `viola_channel::Server`; Deviations step 12 names the `Tools` handle and the `write_snapshot` snapshot.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:174-176; crates/viola-mcp/tests/handlers.rs:16-21; crates/viola-mcp/src/lib.rs:48-58'
  - detector: D-tests-framework
    severity: warning
    section: '§6 E2E Test Strategy → Drivers per surface (ipc-internal (MCP stdio) row)'
    change: 'ipc-internal (MCP stdio): rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile) + jsonschema; lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.'
    sidecar: 'test-plan §6 Drivers: the MCP stdio driver is the rmcp `client` over a test-owned `viola mcp` child awaited to exit, a normal dependency of viola-e2e (was: `TokioChildProcess`, dev-dep features `client` + `transport-child-process`) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Spec claims disproved names `test-plan.md:618`: viola-e2e takes rmcp `client` only as a normal dependency, the child owned by the test; Reverted / negative API facts records `transport-child-process` taken and dropped (the 3 s kill, the truncated profraw refused by llvm-profdata reddening pre-push); Dependencies names rmcp `client` (not `transport-child-process`).'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:163-166; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:177-179; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:78-80'
  - detector: D-tests-framework
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'The viola-e2e dependency bullet reads: in `viola-e2e`: rmcp `>=3.4.1, <3.5` (resolved 3.4.1) with `client` only (no `transport-child-process`; the test owns the `viola mcp` child), tokio `=1.53.1` with default features off and `rt` + `process` (no `test-util`); syn and proc-macro2 as they stand.'
    sidecar: 'test-plan §3 Bootstrap phases: viola-e2e takes rmcp `client` only and tokio `rt` + `process` (was: rmcp `client` + `transport-child-process`, tokio `test-util`) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The same retired claim as the §6 Drivers row, restated in the key file''s dependency list: the report''s Dependencies bullet names viola-e2e''s rmcp `client` (not `transport-child-process`) and tokio `rt` + `process`; Reverted / negative API facts records the child-process transport dropped.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:78-80; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:163-166'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'The `ci` profile''s 20 s kill line holds twenty-one verify-driven binaries: the list gains `mcp_tools` (`crates/viola-e2e/tests/mcp_tools.rs`, its cases starting from a stamped harness session), joined at chunk 2026-10-17-mcp-server-for-drivers (its cases read 0.01–5.9 s); `path6_budget_harness`, also a stamped harness session, is not on that line.'
    sidecar: 'test-plan §3 Bootstrap phases: the ci profile''s verify-driven 20 s kill line twenty → twenty-one binaries, `mcp_tools` joined — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Counts / qualifiers moved: the verify-driven 20 s kill-line binaries 20 → 21 (`binary(mcp_tools)`), naming this key file''s line 10 (its list and "twenty") stale; Harness / gate surface: `.config/nextest.toml` `binary(mcp_tools)` on the 20 s line; Deviations 8: its cases read 0.01–5.9 s and `path6_budget_harness` is not on the line.'
    basis: '.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:96-98; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:261-262; .config/nextest.toml:24'
  - detector: D-tests-framework
    severity: warning
    section: '§3 → 5-command implementation'
    change: 'The `harness_session` bullet: it calls the library `boot` with `stamp: !opts.unstamped`; `SessionOptions` gains `unstamped` (default false, so the default session is stamped) beside `overseer` + `builder`, no `--statusline-echo`, `--agents-mode recorded` — the in-test form of `boot --unstamped` (the Path 7 MCP leg boots an unstamped `builder` alone).'
    sidecar: 'test-plan §3 5-command implementation: `harness_session` boots with `stamp: !unstamped`, `SessionOptions::unstamped` default false (was: always `stamp:true`) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Symbols / APIs and Harness / gate surface name `viola_e2e::harness::session::SessionOptions::unstamped` (default false; boot `stamp: !unstamped`); the key''s harness_session bullet still says it boots with `stamp:true`. Deviations 7: Path 7 boots `builder` alone.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:72; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:126; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:260'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Surface: ipc-internal (MCP stdio server `viola mcp`) → Signal'
    change: 'Signal: `isError:false` + `structuredContent` equal to the `ok` payload; `isError:true` + `{refusal, detail}`; `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written); every `isError` text `refused: <code>`; an internal handler fault (any other channel error, an unknown reply shape, a `wait`/`last` refusal, a handler panic) is the JSON-RPC error `-32603` `internal error`, `data: null`, an unknown tool `-32602`; `tools/list` is exactly the five tools and its result carries `ttlMs` 0 and `cacheScope` `private`; no `release` / `pause` / `link` / `unlink` tools.'
    sidecar: 'test-plan §1 MCP signal: the error bodies carry `detail` (`instance-unreachable` null, `wrapper-fault` the fault, the new `invalid-params` a closed field word), the `-32603` internal fault, and the list result''s `ttlMs` / `cacheScope` — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Symbols / APIs define the result shapes (`invalid-params` with a closed field `detail`, `instance-unreachable` with `detail:null`, `wrapper-fault` with `{code,message,data}`, the `-32603` fault, the `-32602` unknown tool) and the `tools/list` `ttlMs` / `cacheScope` claude 2.1.287 requires; Schema / config names `invalid-params` a new MCP error code. The plan''s signal names only `{"error":"instance-unreachable"|"wrapper-fault"}` with no `detail`, so these new paths (unit `tool_inputs_are_revalidated_before_any_frame`, `tools_result_carries_its_ttl_and_cache_scope`, `internal_fault_is_the_fixed_jsonrpc_error`) carry no plan signal.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:38-54; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:85-86; crates/viola-mcp/src/tools.rs:480-511; crates/viola-mcp/src/tools.rs:461-468; crates/viola-mcp/src/tools.rs:577-584'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 3 → Verification signal (Step 7)'
    change: 'Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.'
    sidecar: 'test-plan §6 Path 3 step 7: the MCP unreachable body carries `detail:null`, equal to the CLI''s less `v` — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The same retired claim (an MCP error body with no `detail`) restated in Path 3''s signal; the report''s Expected amendments name "§6 Path 3: the MCP error shape with `detail:null`", and Symbols / APIs give `Connect`/`Closed`/`Io` → `{"error":"instance-unreachable","detail":null}`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:44-49; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:222-224'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → Cross-module patterns covered → Wrapper channel (the impostor bullet)'
    change: 'The MCP half of the impostor case: `call_tool("send")` gets `is_error:true` with `{"error":"instance-unreachable","detail":null}`.'
    sidecar: 'test-plan §5 impostor bullet: the MCP unreachable body carries `detail:null` — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'A third occurrence of the MCP error body without `detail`; the report''s Symbols / APIs give `instance-unreachable` with `detail:null`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:44-49'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 2 → Surfaces involved (As landed)'
    change: 'Replace "the MCP `send` step and its verification are owed to `:118`" with: the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli` (a harness session, `binary(mcp_tools)` in the E2E layer), its result compared with the CLI''s `--json` document less `v`.'
    sidecar: 'test-plan §6 Path 2: the MCP `send` step landed as `mcp_tools_path2_path3_send_wait_last_match_the_cli` (was: owed) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Outcome: each tool''s success and refusal equal its CLI `--json` counterpart less `v` — met (`mcp_tools` Paths 2–7 and `list`, every refusal body compared with the CLI''s document); the New text lists the test. The plan still calls the MCP step owed.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:351-352; crates/viola-e2e/tests/mcp_tools.rs:427-530'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 3 → Surfaces involved (As landed)'
    change: 'Replace "The MCP `wait` / `last` steps and their verification are owed to `:118`" with: the MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, each result compared with the CLI''s `--json` document less `v`; the MCP `wait` sends `timeout_ms` clamped to 50 000 ms, its default (`handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`).'
    sidecar: 'test-plan §6 Path 3: the MCP `wait` / `last` steps landed in `mcp_tools` (was: owed) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The same retired claim (the MCP steps of a critical path owed to a later chunk); the report''s Outcome marks Paths 2–7 met in `mcp_tools` and the `wait` clamp met by `wait_timeout_is_clamped_to_the_cap` and `handlers_wait_sends_the_clamped_bound_…`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:351-352; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:358-359; crates/viola-e2e/tests/mcp_tools.rs:427-530; crates/viola-mcp/tests/handlers.rs:186-210'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 4 → Surfaces involved (As landed)'
    change: 'Replace "The MCP `answer` step is owed to its surface chunk." with: the MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, its result and refusal compared with the CLI''s `--json` document less `v`.'
    sidecar: 'test-plan §6 Path 4: the MCP `answer` step landed as `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (was: owed) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The same retired claim; the report''s Outcome marks the `mcp_tools` Paths 2–7 met and the New text lists the Path 4/5 test.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:351-352; crates/viola-e2e/tests/mcp_tools.rs:532-612'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 5 → Surfaces involved (As landed)'
    change: 'Replace "The MCP `send` step and its refusal checks are owed to `:118`." with: the MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (the refusal compared with the CLI''s document less `v`; no MCP result, body or tool description names `release`).'
    sidecar: 'test-plan §6 Path 5: the MCP `send` refusal checks landed as `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (was: owed) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The same retired claim; the report''s Outcome marks Paths 2–7 met and "no result, body or description names `release`" met.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:351-353; crates/viola-e2e/tests/mcp_tools.rs:532-612'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: Path 6 → As landed'
    change: 'Replace "Not asserted yet, owed by title: the MCP form of the refusal to “MCP server for drivers”." with: the MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`): `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}` with no cursor, equal to the CLI''s document less `v`.'
    sidecar: 'test-plan §6 Path 6: the MCP budget-paused refusal landed as `mcp_tools_path6_budget_paused_send` (was: owed by title to this chunk) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Spec claims disproved names the owner rows naming this chunk as future (`test-plan.md:766`): landed; the Outcome marks "`budget-paused` five-hour with no cursor — met".'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:190-191; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:353; crates/viola-e2e/tests/mcp_tools.rs:614-665'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Non-path suites → Security sweep'
    change: 'The MCP schema-violating calls live in `crates/viola-mcp/tests/handlers.rs` `handlers_schema_violations_write_no_frame` (no frame reaches the test wrapper), the `crates/viola-mcp/src/tools.rs` unit `tool_inputs_are_revalidated_before_any_frame`, and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli` (each violation `invalid-params` with its field, no `send-issued`; a control-character or empty text `not-delivered`) — no `mcp_schema_violations.rs` binary.'
    sidecar: 'test-plan §6 Security sweep: the MCP schema-violation cases are `handlers_schema_violations_write_no_frame`, `tool_inputs_are_revalidated_before_any_frame` and `mcp_tools_list_…` (was: `mcp_schema_violations.rs`) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Outcome: every input re-validated server-side, the four schema violations code-only with no `send-issued`, empty / LF-only text `empty-text` — met by `tool_inputs_are_revalidated_before_any_frame`, `handlers_schema_violations_write_no_frame` and `mcp_tools_list_…`; the plan names a file the chunk did not create.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:348-350; crates/viola-mcp/tests/handlers.rs:274-299; crates/viola-mcp/src/tools.rs:480-511; crates/viola-e2e/tests/mcp_tools.rs:279-386'
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline structure (Lint row)'
    change: 'The sync-crate `cargo check` target set is `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude` (four); `viola-channel` left `scripts/sync-crates.txt` at chunk 2026-10-17-mcp-server-for-drivers (its Tokio client sits behind its `tokio` feature) and its normal graph is held by the tree form `scripts/channel-tokio-free.sh` (Supply-chain row).'
    sidecar: 'test-plan §9 Lint: the sync-crate target set 5 → 4, `viola-channel` held by the `channel-tokio-free.sh` tree form — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Counts / qualifiers moved: the sole-root tokio loop''s crate list 5 → 4, `viola-channel` left `scripts/sync-crates.txt` for the tree form; Crates / modules: `viola-channel` gains `tokio = ["dep:tokio", "interprocess/tokio"]`. The Lint row''s target set still names `viola-channel`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:90-95; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:73-77'
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline structure (Lint row)'
    change: 'Replace "The rmcp release-graph assertion … has no subject until `viola-mcp` exists and joins this row with the “MCP server for drivers” chunk" with: the lint job step `rmcp features (exactly server + transport-io, non-vacuous)` (every OS, after `Lint probes`) runs `bash scripts/rmcp-features.sh` (`cargo tree -e features -p viola --edges normal --target all -i rmcp --no-dedupe --prefix depth`: rmcp must be present, and the features a non-rmcp package requests exactly `server transport-io`), last line `rmcp-features: server transport-io`.'
    sidecar: 'test-plan §9 Lint: the rmcp release-graph assertion landed as `scripts/rmcp-features.sh` in the lint job on every OS (was: no subject until this chunk) — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Harness / gate surface: `scripts/rmcp-features.sh` (new) and the ci.yml lint step on every OS; the Outcome: `rmcp-features.sh` green and in CI''s lint job (CI lint ×3 green); Spec claims disproved: the owner rows naming this chunk as future (`test-plan.md:1021`) landed.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:114-118; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:344; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:190-191; .github/workflows/ci.yml:337-340'
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline fails on (Lint errors)'
    change: 'Lint errors add: `scripts/rmcp-features.sh` not ending `rmcp-features: server transport-io` (rmcp absent from `viola`''s release graph, or a feature beyond `server` + `transport-io` requested).'
    sidecar: 'test-plan §9 failure list: a red `rmcp-features.sh` is a lint error — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The failure list restates the Lint row''s gates and omits the landed rmcp check; the report''s Harness / gate surface gives its FAILED and verdict lines.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:114-117'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline structure (Supply-chain row)'
    change: 'After the sole-root loop, step `Tokio ban per sync crate (sole root)` runs `bash scripts/channel-tokio-free.sh`: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must read no `tokio v` line (exit 0 "nothing to print", or exit 101 "did not match" when tokio is not in the lock; presence read by the printed line, never the exit), its control `--features tokio` must read one; verdict `channel-tokio-free: ok`.'
    sidecar: 'test-plan §9 Supply-chain: `scripts/channel-tokio-free.sh` holds `viola-channel`''s normal graph tokio-free after the sole-root loop — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Harness / gate surface: `scripts/channel-tokio-free.sh` (new) and the supply-chain step running it after its loop; Decisions 8: an absence check reads the printed `<crate> v` line, never the exit; Outcome: `channel-tokio-free: ok`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:111-113; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:118-119; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:311-314'
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline fails on (Supply-chain errors)'
    change: 'Supply-chain errors add: `scripts/channel-tokio-free.sh` not ending `channel-tokio-free: ok` (`viola-channel` reaching tokio without its feature, or its control reading no tokio).'
    sidecar: 'test-plan §9 failure list: a red `channel-tokio-free.sh` is a supply-chain error — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The failure list restates the Supply-chain row''s gates and omits the landed tree form; the report gives its verdict line and the FAILED wording `channel-tokio-free: FAILED — viola-channel reaches tokio without its feature`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:111-113; scripts/deny-probes.sh:86-88'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline structure (Supply-chain row)'
    change: '`bash scripts/deny-probes.sh` ends `deny-probes: 13/13 banned, control clean`: the twelve per-ban probes plus probe `channel-tokio-edge` (a throwaway workspace with a normal tokio edge must fail the tree form), with a channel control (the same edge behind a `tokio` feature must pass).'
    sidecar: 'test-plan §9 Supply-chain: deny-probes 12/12 → 13/13, the `channel-tokio-edge` probe and its channel control — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Harness / gate surface: `scripts/deny-probes.sh` gains probe `channel-tokio-edge` and a channel control and reads `13/13 banned, control clean`; the Outcome repeats it. The row still reads `12/12`, "twelve bans, one probe each".'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:122-124; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:342-343; scripts/deny-probes.sh:69-79'
  - detector: D-tests-framework
    severity: warning
    section: '§9 CI Integration → Pipeline fails on (Supply-chain errors)'
    change: '`scripts/deny-probes.sh` not ending `deny-probes: 13/13 banned, control clean`.'
    sidecar: 'test-plan §9 failure list: deny-probes 12/12 → 13/13 — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'A second occurrence of the deny-probes count in the failure list; the report reads `13/13 banned, control clean`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:122-124'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§7 Test Data & Fixtures (Time-sensitive data row)'
    change: 'Add the qualifier, as for mock_instant: no manifest names tokio `test-util` (as landed at chunk 2026-10-17-mcp-server-for-drivers: tokio `=1.53.1` with default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`); `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)` = `min(given, 50 000)`, a unit case — so no `start_paused` test exists.'
    sidecar: 'test-plan §7 Time-sensitive data: tokio `test-util` / `start_paused` in no manifest; viola-mcp''s wait bound is the pure `wait_timeout` — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'The report''s Dependencies bullet lists every tokio feature the chunk took (no `test-util`), and Symbols / APIs give `wait_timeout` as a pure clamp tested by `wait_timeout_is_clamped_to_the_cap`; the plan states `start_paused = true` in `viola-mcp` as the time mechanism.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:78-80; viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:36-37; crates/viola-mcp/src/tools.rs:38-41; crates/viola-mcp/src/tools.rs:470-478'
  - detector: D-tests-framework
    severity: warning
    section: '§8 Mocking & Stubbing Discipline (Time mocking row)'
    change: 'Time mocking: tokio 1.53.1 `test-util` is in no manifest yet (as measured at chunk 2026-10-17-mcp-server-for-drivers), beside the mock_instant qualifier.'
    sidecar: 'test-plan §8 Time mocking: tokio `test-util` in no manifest — chunk 2026-10-17-mcp-server-for-drivers'
    rationale: 'A second occurrence of the tokio `test-util` claim; the report''s Dependencies bullet names no `test-util`.'
    basis: 'viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/report.md:78-80'
    dependent-of: D-tests-framework
note: 'D-tests-obs-harness: no proposal — the report changes no 5-command harness command, status shape or log format (Schema / config: `mcp.ndjson` and process `mcp` stood before; only new writers into it), so §3 and obs-plan §3 still agree. Not proposed, for the orchestrator: the report names `test-plan.md:399` stale under the sync-crates pattern, but that line names no crate of the list (the crate-naming site is the §9 Lint row at :1021, which the report calls true — the two look swapped); and the §5 impostor bullet''s MCP half (like its CLI half) cannot fire while MCP frames go after the liveness-only pre-check, the eighth dated gap, owed to "Server verification before any frame".'
