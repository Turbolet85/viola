# tests · picked accounting · group 2 of 3

## Picked
e2e-test-strategy#1.5 · carried · the ipc-internal MCP stdio driver row is the Tokio-bearing E2E path (`crates/viola-e2e`, rmcp `client` only, no `[dev-dependencies]`, never `TokioChildProcess`) the chunk re-runs on the bumped tokio
e2e-test-strategy#3.1 · carried · names `mcp_tools_path2_path3_send_wait_last_match_the_cli` in `crates/viola-e2e/tests/mcp_tools.rs`, the witness of the Path 2 MCP `send` leg the chunk re-runs on the bumped tokio
e2e-test-strategy#3.2 · not in scope · Path 2's driving steps gain, lose and move none; the bump re-runs them unchanged inside the workspace suite
e2e-test-strategy#3.3 · carried · its MCP signal (`is_error:false`, the CLI `ok` key set) is what the re-run of the E2E MCP stdio path re-asserts on the bumped tokio
e2e-test-strategy#4.1 · carried · names the MCP `wait` / `last` witnesses in `crates/viola-e2e/tests/mcp_tools.rs` and the handler case `handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`, both Tokio-bearing paths the chunk re-runs
e2e-test-strategy#4.2 · not in scope · Path 3's driving steps gain, lose and move none; the bump re-runs them unchanged inside the workspace suite
e2e-test-strategy#4.3 · carried · its MCP `last`, live `wait` wake and `instance-unreachable` signals are what the re-run of the E2E MCP stdio path re-asserts on the bumped tokio
e2e-test-strategy#5.1 · carried · names `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, the witness of the Path 4 MCP `answer` leg the chunk re-runs on the bumped tokio
e2e-test-strategy#5.2 · not in scope · Path 4's driving steps gain, lose and move none; the bump re-runs them unchanged inside the workspace suite
e2e-test-strategy#5.3 · carried · its MCP `is_error:false` signal is what the re-run of the E2E MCP stdio path re-asserts on the bumped tokio
e2e-test-strategy#6.1 · carried · names the Path 5 MCP `send` refusal witness in `crates/viola-e2e/tests/mcp_tools.rs`, re-run on the bumped tokio
e2e-test-strategy#6.2 · not in scope · Path 5's driving steps gain, lose and move none; the bump re-runs them unchanged inside the workspace suite
e2e-test-strategy#6.3 · carried · its MCP `human-typing` refusal text and no-`release` signals are what the re-run of the E2E MCP stdio path re-asserts on the bumped tokio
e2e-test-strategy#7.3 · carried · names `mcp_tools_path6_budget_paused_send` and the `crates/viola-e2e` harness binary `path6_budget_harness.rs`, Tokio-crate test binaries the chunk re-runs
e2e-test-strategy#8.1 · not in scope · Path 7's driving steps gain, lose and move none; the bump re-runs them unchanged inside the workspace suite
e2e-test-strategy#8.2 · carried · its MCP `unverified-cli` refusal signal is what the re-run of the E2E MCP stdio path re-asserts on the bumped tokio
e2e-test-strategy#12.1 · carried · names `crates/viola-mcp/tests/handlers.rs` `handlers_schema_violations_write_no_frame` and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`, the adapter-level and E2E Tokio-bearing tests the scope names
e2e-test-strategy#13.1 · not in scope · the post-test sanitization and canary scan gain no case and change no check, and the bump moves no error payload, log line or diagnostics file
e2e-test-strategy#15.1 · not in scope · the security control negatives (socket dir, SQOS, Windows listener, modes, single writers) ride no tokio and gain or change no case
test-data-fixtures#1.12 · carried · records the locked tokio `=1.53.1` and each crate's tokio features with `test-util` in no manifest and no `start_paused` test, the pin the chunk moves and the containment it re-reads
mocking-stubbing-discipline#1.1 · carried · names tokio 1.53.1 `test-util` for a `viola-mcp` timer and the paused clock never reaching a wrapper, the time discipline the scope says stays as it is on the bumped tokio
mocking-stubbing-discipline#2.2 · carried · the Time mocking row records tokio 1.53.1 `test-util` as in no manifest, a version value the bump moves and a feature it must not add
