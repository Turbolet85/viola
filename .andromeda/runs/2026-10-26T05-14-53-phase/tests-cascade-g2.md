# tests · cascade read · group 2 of 3

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) The E2E MCP stdio path re-run on the bumped tokio must keep its driver as written: an rmcp client over a test-owned `viola mcp` child, hosted in `crates/viola-e2e` with rmcp `client` only as a normal dependency, no `[dev-dependencies]`, no `transport-child-process` and never `TokioChildProcess`, the lock move pulling none of them in.
  > | ipc-internal (MCP stdio) | rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema | direct | Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.
  (.andromeda/test-plan.md:470)
  > - **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`; beside it, a replay of the recorded MCP exchange.
  (.andromeda/test-plan.md:873)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:46) The Path 2 MCP `send` witness in `crates/viola-e2e/tests/mcp_tools.rs` must read green on the bumped tokio, its result still equal to the CLI's `--json` document less `v`.
  > the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli` (a harness session, `binary(mcp_tools)`), its result compared with the CLI's `--json` document less `v`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:65)
  > - MCP: `is_error:false`, and the `structured_content` key set equals the CLI `ok` key set `{submitted_at, cursor}`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:74) The Path 3 MCP `wait` / `last` witnesses and the adapter-level clamped-bound handler case must read green on the bumped tokio, each result still equal to the CLI's document less `v`.
  > The MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, each result compared with the CLI's `--json` document less `v`; the MCP `wait` sends `timeout_ms` clamped to 50 000 ms, its default (`handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:88)
  >   - `last` gives `{last_assistant_message:"done: …", ts}`, and the MCP `structured_content` is equal.
  >   - Live MCP `wait` wake: an rmcp `call_tool("wait", {target:"builder", after:C})` is issued after the step 3 `activity` record is logged and before step 4. It resolves with `is_error:false` and the same `turn-ended` event and `cursor` as the CLI `wait`, never with the `activity` record.
  >   - Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:99) The Path 4 MCP `answer` witness must read green on the bumped tokio, its result and refusal still equal to the CLI's document less `v`.
  > The MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, its result and refusal compared with the CLI's `--json` document less `v`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:121)
  >   - MCP `is_error:false`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:132) The Path 5 MCP `send` refusal witness must read green on the bumped tokio, its refusal still the CLI's document less `v` and naming no `release`.
  > The MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (the refusal compared with the CLI's document less `v`; no MCP result, body or tool description names `release`).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:144)
  > MCP gives `is_error:true` with text `refused: human-typing`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:148)
  > The MCP refusal text and `structured_content` do not contain `release` either.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:183) The `crates/viola-e2e` test binaries of Path 6, the harness leg and the MCP budget refusal, must read green on the bumped tokio with the refusal unchanged.
  > The harness-session form and step 7's two-instance leg landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `path6_two_instance_release_over_a_harness_session` (`crates/viola-e2e/tests/path6_budget_harness.rs`, beside the unit case `gate_release_on_one_instance_leaves_another_paused`)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:183)
  > The MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`): `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}` with no cursor, equal to the CLI's document less `v`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:200) The Path 7 MCP `answer` refusal on an unverified CLI must hold unchanged on the bumped tokio.
  > MCP gives `is_error:true` with text `refused: unverified-cli`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:242) The adapter-level MCP tests over `viola_channel`'s Tokio client in `crates/viola-mcp/tests/handlers.rs` and the E2E tool-list case must read green on the bumped tokio, each schema violation still `invalid-params` with no frame written.
  > The MCP schema-violating calls live in `crates/viola-mcp/tests/handlers.rs` `handlers_schema_violations_write_no_frame` (no frame reaches the test wrapper), the `crates/viola-mcp/src/tools.rs` unit `tool_inputs_are_revalidated_before_any_frame` and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli` (each violation `invalid-params` with its field and no `send-issued`; a control-character or empty text `not-delivered`) — no `mcp_schema_violations.rs` binary (chunk 2026-10-17-mcp-server-for-drivers)
- (.andromeda/test-plan-detail/test-data-fixtures.md:21) The bumped pin keeps each crate's tokio feature set as recorded (default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`), adds `test-util` to no manifest and adds no `start_paused` test; the recorded `=1.53.1` is the value the chunk moves.
  > No manifest names tokio `test-util` either (as landed at chunk 2026-10-17-mcp-server-for-drivers: tokio `=1.53.1` with default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`); `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)` = `min(given, 50 000)`, a unit case — so no `start_paused` test exists
  (.andromeda/test-plan.md:653)
  > - **Time-sensitive data:** injected per test behind `viola_core::Clock`; the budget gate's override expiry takes its instant as an argument, and `viola-mcp` owns no timer.
  (.andromeda/test-plan.md:914)
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel).
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) The time discipline over tokio stays as it is on the bumped version: a paused tokio clock only for a timer `viola-mcp` itself owns, the `wait` deadline driven only by the injected `Clock` in the wrapper, and no test re-timed by the bump.
  > Async crates (`viola-mcp`) use `#[tokio::test(start_paused = true)]` from tokio 1.53.1 `test-util` for any timer the MCP adapter itself owns. The `wait` `timeout_ms` deadline runs in the wrapper process, and only the injected `viola_core::Clock` in the §4 root-bin case drives it. A paused tokio clock in a test never reaches a spawned wrapper (§11 Universal).
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17)
  > | Time mocking | mock_instant 0.6.1 (in no manifest yet, as measured at the chunk 2026-10-10-budget-governor wrap); tokio 1.53.1 `test-util` (in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers) |
  (.andromeda/test-plan.md:705)
  > `viola-mcp` uses tokio's paused clock for a timer it owns, and the budget gate's override expiry is a pure-function case; `chrono::Utc::now()` is read only through the clock seam, except at the budget gate's three evaluation points, and a paused clock in a test never reaches a spawned wrapper.

## Bindings
- **e2e-test-strategy (MCP stdio driver) ↔ test-data-fixtures (tokio features per crate)**: the E2E MCP path lives in `crates/viola-e2e`, whose tokio `rt` + `process` the bumped lock must keep as a normal dependency with no `[dev-dependencies]`, (.andromeda/test-plan-detail/e2e-test-strategy.md:14) ↔ (.andromeda/test-plan-detail/test-data-fixtures.md:21)
- **test-data-fixtures ↔ mocking-stubbing-discipline**: both record tokio 1.53.1 `test-util` as in no manifest and `viola-mcp` as owning no timer; the bump moves the version they name and adds neither the feature nor a `start_paused` test, (.andromeda/test-plan-detail/test-data-fixtures.md:21) ↔ (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) ↔ (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17)
- **e2e-test-strategy (MCP witnesses) ↔ the body §11 / CI matrix (group 3)**: the `mcp_tools.rs` and `handlers.rs` cases are the Tokio-bearing witnesses the bump re-runs, read green on each CI OS runner, never on the Linux dev host alone, (.andromeda/test-plan-detail/e2e-test-strategy.md:46) ↔ (.andromeda/test-plan.md:873)
- **mocking-stubbing-discipline ↔ e2e-test-strategy (Path 3)**: the MCP `wait` bound is the clamp of the handler case and the deadline runs in the wrapper under the injected `Clock`, so the Path 3 MCP `wait` signals are re-run with no paused tokio clock reaching the wrapper, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) ↔ (.andromeda/test-plan-detail/e2e-test-strategy.md:74)
