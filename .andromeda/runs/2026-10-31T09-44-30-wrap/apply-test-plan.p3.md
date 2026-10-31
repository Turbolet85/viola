# Apply — test-plan · part 3 of 3

## Verdicts
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:10#1
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:13#1
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:14#1
- fanout-test-plan--cascade-g3.md 4 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:14#2
- fanout-test-plan--cascade-g4.md 1 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:259#1
- fanout-test-plan--cascade-g4.md 5 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:260+#1
- fanout-test-plan--cascade-g4.md 7 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:260+#2
- fanout-test-plan--cascade-g4.md 8 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:261+#1
- fanout-test-plan--cascade-g4.md 9 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:264+#1
- fanout-test-plan--cascade-g4.md 11 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:372+#1
- fanout-test-plan--cascade-g4.md 12 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:372+#2
- fanout-test-plan.md 4 · applied · .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#1, .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#2, .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#3, .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#4
- cascade-compare-test-plan.md 1 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:23+#1

## Edits
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:10
  old: «its outer-PTY witness a CARRY on "Linux and macOS parity".»
  new: «its outer-PTY witness a CARRY on "Linux and macOS parity". `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text` holds each CLI `--json` failure it drives, across dead, refused and live rows, to one document, a literal exit, empty stderr, no ESC and a clean body.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:13
  old: «The endpoint is read only from `snapshot.json`.»
  new: «The endpoint is read only from `snapshot.json`. `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` writes seven fault rows to the channel of a wrapper booted with a planted value (through `OuterPty::spawn_without`'s `env_set`, as `tests/tui_env_strip.rs` does), its bodies clean and the three `detail` words read on the lines.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:14
  old: «and its exit awaited under an 8 s kill deadline.»
  new: «and its exit awaited under an 8 s kill deadline. Beside them, `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path), `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` and `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (exit 1, both streams empty, one `parse-rejected` `mcp-stdin` / `oversize` line, count 1) stand in `tests/mcp_tools.rs`.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:14
  old: «no `transport-child-process`.»
  new: «no `transport-child-process`. rmcp's client reads the wire's `"data":null` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product's `Some(Value::Null)`.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:259
  old: «after every E2E test, the `events` support helper checks:»
  new: «no `events` helper runs after every E2E test: that check is not built, replaced by the three surface witnesses (`tests/channel_error_bodies.rs`, `tests/cli_error_bodies.rs`, the MCP cases of `crates/viola-e2e/tests/mcp_tools.rs`), the enumeration and the census of writers (`tests/contract_error_bodies.rs`), reading the check in `tests/support/events.rs`. The rules:»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:260
  anchor: «  - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`»
  add: «  - The check: `tests/support/events.rs` `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes; self-tested by `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` (`tests/cli_error_bodies.rs`).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:260
  anchor: «  - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`»
  add: «  - `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]`, each on the `ci` profile's 20 s kill line.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:261
  anchor: «  - every -32603 is exactly `"internal error"` / `null`»
  add: «  - No `-32603` is driven in a scanned home, and no failpoint or counted panic stands for one; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:264
  anchor: «  - **Canary** (obs-plan §8, High-class verification): every integration and E2E input is synthetic, and every prompt, send text, tool `input`, dialog answer and statusline payload the tests feed embeds one fixed canary string, a constant in `tests/support`. The scan asserts that the canary never appears in any home-level `<home>/diagnostics/*.ndjson` line. It may appear in `events.ndjson` (contract payloads) and in `instances/<name>/diagnostics/detail-*.ndjson` only. The same scan runs in CI over `target/e2e-home/**`, the harness capture in `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff` (repository source text, excluded from the scan and from the `harness-<os>` upload), and `target/nextest/ci/junit.xml` before any upload (obs-plan §9 step order). A hit is reported as file, line, byte offset and pattern class, never the matched bytes.»
  add: «  - `v1-47`'s "after every end-to-end test" reads "after each job's end-to-end run", on the delegate overseer's PROVISIONAL word, owed the founder's own word: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()` (`.github/workflows/ci.yml`), the `viola-chaos-*` home out (`.andromeda/obs-plan-detail/ci-integration.md`).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:372
  anchor: «    - The census (`product_readers_are_each_bounded_or_named`, its file carrying the `andromeda:walks-tree` token) walks the product source roots, comments, strings and test items left out: a reader call passes when a `take(` stands in its own statement up to the call, or when an `EXCEPTIONS` needle stands on the call's whole source line; its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one` reads a planted unbounded read as a hit; with the `replace_private_shared` compare bound undone it reads red at exactly `crates/viola-state/src/fs.rs:354` (as measured at `viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/evidence/controls.md`).»
  add: «  - `tests/contract_error_bodies.rs` (a `contract_` binary, `run --e2e`) is the error-body enumeration: the tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`, its file carrying the `andromeda:walks-tree` token.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:372
  anchor: «    - The census (`product_readers_are_each_bounded_or_named`, its file carrying the `andromeda:walks-tree` token) walks the product source roots, comments, strings and test items left out: a reader call passes when a `take(` stands in its own statement up to the call, or when an `EXCEPTIONS` needle stands on the call's whole source line; its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one` reads a planted unbounded read as a hit; with the `replace_private_shared` compare bound undone it reads red at exactly `crates/viola-state/src/fs.rs:354` (as measured at `viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/evidence/controls.md`).»
  add: «    - Its guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, with the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «then the twenty-two verify-driven binaries»
  new: «then the twenty-four verify-driven binaries»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «channel_endpoint channel_links»
  new: «channel_endpoint channel_error_bodies channel_links»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «cli_budget cli_fake_agent»
  new: «cli_budget cli_error_bodies cli_fake_agent»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «which start from one since that chunk's CI fix)»
  new: «which start from one since that chunk's CI fix; `channel_error_bodies` and `cli_error_bodies` on it because each one's witness starts from `stamped_home`)»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:23
  anchor: «| Time-sensitive data | mock_instant 0.6.1 `MockClock` (global mode) behind `viola_core::Clock` in the sync crates, and tokio `test-util` `start_paused = true` in `viola-mcp`. No manifest names mock_instant yet (as measured at the chunk 2026-10-10-budget-governor wrap: `mock_instant` in no `Cargo.toml`); the budget gate's override expiry takes its instant as a function argument instead. The link line's `since` is stamped outside the clock seam and asserted by shape only (§8). No manifest names tokio `test-util` either (as landed at chunk 2026-10-17-mcp-server-for-drivers; tokio `=1.53.3` with default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`); `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)` = `min(given, 50 000)`, a unit case — so no `start_paused` test exists | injected per test |»
  add: «| The channel error-body witness's planted value | `tests/channel_error_bodies.rs` `booted_with_the_value` starts from `stamped_home` and boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env` (`Wrapper` takes no planted value); the value reaches that one child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does. The check `tests/support/events.rs` `assert_error_body_clean` takes the planted values and reads a hit as class `planted`, reporting place and class, never bytes. | per-test (rstest `stamped_home`) |»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = 'the `events` support helper checks'

[[pattern]]
id = "tp3-02"
fixed = 'twenty-two verify-driven'
