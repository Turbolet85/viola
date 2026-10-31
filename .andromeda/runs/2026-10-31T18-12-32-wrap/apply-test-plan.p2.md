# Apply — test-plan · part 2 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:9+#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:28+#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:69+#1
- fanout-test-plan--cascade-g1.md 13 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:87#1
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:87+#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:40+#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:56#1
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:57#1
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:58+#1
- fanout-test-plan--cascade-g2.md 5 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:58+#2
- fanout-test-plan--cascade-g2.md 6 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:68#1
- fanout-test-plan--cascade-g2.md 7 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:69#1
- fanout-test-plan--cascade-g2.md 8 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:85+#1
- fanout-test-plan--cascade-g2.md 9 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:85+#2
- fanout-test-plan--cascade-g2.md 10 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:85+#3
- fanout-test-plan--cascade-g2.md 11 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:99+#1

## Edits
- after: .andromeda/test-plan-detail/unit-test-strategy.md:9
  anchor: «  - `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first, held by `code_word_literal_table` (`crates/viola-core/src/lib.rs`).»
  add: «  - `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words (`crates/viola-core/src/lib.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:28
  anchor: «  - The server check: `ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows`; where no peer pid is reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap. `verify_server_reads_the_observed_start_time_from_the_os` covers the joined `verify_server`'s read of the observed start time from the OS (`process_start_time`, none for a zombie). `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot_or_replay` call: only a present snapshot proceeds, and a replayed or absent snapshot or a read error is `None`, since a replayed snapshot carries no `endpoint`, `pid` or `started_at`.»
  add: «  - `viola_state::liveness::without_endpoint(instance_dir)` (`crates/viola-state/src/liveness.rs`) reads by `symlink_metadata` a directory as `InstanceDead` and anything else (absent, a regular file, a symlink) as `UnwrappedName`, opening nothing and writing no line; `without_endpoint_maps_the_directory_reading` (`src/cmd/client.rs`) holds the client's mapping of that reading.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:69
  anchor: «  - A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, its exit `unknown`'s 14 by `reason_of`.»
  add: «  - `json_asked` (`src/main.rs`) reads `--json` from argv only before a `--`, held by `json_asked_reads_the_flag_before_a_separator`.»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:87
  old: «(`instance-dead` · `server-verify-failed`, the server cause `during` `connect`)»
  new: «(`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`)»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:87
  anchor: «  - The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, the server cause `during` `connect`).»
  add: «  - The `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause of `instance-dead` keys on `not-running`, a word apart from the log's.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:40
  anchor: « Such a home passes the instance check on Windows, where a home a test makes with plain `create_dir_all` before viola is refused `Writable`, the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (as measured on `windows-2025`: ci#40439188053 for the refusal, by `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, the plain home's canonical SDDL not recorded; ci#40403389629 for the seeded homes, the three `hook_statusline` cases green). A case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies). The reason for the seed is the first start's companion write, as measured at that chunk on the dev host under a parallel suite: a median 1 193 ms, which pushed first-start tests past their 7 s waits; no bound was raised. The product's first-start write stays covered by the unseeded starts: `tests/conpty_sideload.rs` and `run_viola_unseeded` never seed. A second carve-out (chunk 2026-10-04-confirmed-send-with-cl-1-records, the founder's ruling, live, 2026-10-04, relayed by the overseer): `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, a home under the system temp dir (`viola-chaos-*`), not under `target/e2e-home`, because its forced vt100 feed panic writes a G2-counted `event:"panic"` line; that home is therefore outside G2 (zero panics), G4 (schema conformance) and the secret scan, and the test asserts its panic line and its `parse-rejected{vt100-feed, panicked}` line present itself. Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.»
  add: «**Bare temp-dir home:** a bare temp dir reads 0755 under the umask and the read rule refuses it; `tool_inputs_that_pass_reach_the_liveness_check` reaches the check it names from a viola-made `empty_home` and reads `unwrapped-name`.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:56
  old: «with `{"error":"instance-unreachable","detail":null}` and text»
  new: «with `{"error":"instance-unreachable","detail":"server-verify-failed"}` and text»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:57
  old: «with `{"error":"instance-unreachable","detail":null}` before any snapshot read»
  new: «with `{"error":"instance-unreachable","detail":"strict-modes-failed"}` before any snapshot read»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:58
  anchor: «  - the MCP screen case `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` (`crates/viola-mcp/tests/handlers.rs`) holds `of_reply`'s screen against a test wrapper: a `refusal` of free text reads `unknown`, with no detail.»
  add: «  - MCP's mapping (`crates/viola-mcp/src/tools.rs` `frame`, `crates/viola-mcp/src/result.rs` `of_reply`): a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `server-verify-failed` (off macOS), `Connect` is `instance-dead`, and `Closed` or `Io` is `endpoint-lost`, with no frame and no line; `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` (`crates/viola-mcp/tests/handlers.rs`) and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`) hold each cause its word.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:58
  anchor: «  - the MCP screen case `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` (`crates/viola-mcp/tests/handlers.rs`) holds `of_reply`'s screen against a test wrapper: a `refusal` of free text reads `unknown`, with no detail.»
  add: «  - the unserved row in `crates/viola-mcp/tests/handlers.rs` records, through `record_live`, an endpoint no one bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:68
  old: «MCP frame tools `instance-unreachable` with `detail: null` and no frame or line»
  new: «MCP frame tools `instance-unreachable` with `detail` `strict-modes-failed` and no frame or line»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:69
  old: «`{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «`{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:85
  anchor: «  - `tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line. `verify_under_a_regular_file_prints_only_the_internal_error_line` forces that catch-site line on every OS with a regular file where `ledger/` goes in a viola-made home.»
  add: «  - `viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture`, beside `interrupted`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:85
  anchor: «  - `tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line. `verify_under_a_regular_file_prints_only_the_internal_error_line` forces that catch-site line on every OS with a regular file where `ledger/` goes in a viola-made home.»
  add: «  - a refused home tree under `verify` stays lineless, a failing row is no refusal and no code, the fixture's file name enters no code, and `verify` takes no `--json`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:85
  anchor: «  - `tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line. `verify_under_a_regular_file_prints_only_the_internal_error_line` forces that catch-site line on every OS with a regular file where `ledger/` goes in a viola-made home.»
  add: «  - the refusal codes' witnesses: `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` in `tests/cli_verify.rs`, the second under the `verify_window_` prefix because it runs two whole verifies, and `verify_with_no_claude_on_its_path_is_program_not_found` in the table `tests/cross_exit_causes.rs`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:99
  anchor: «  - Home resolution and the machine contract (chunk 2026-10-17-cli-machine-contract): `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance (`verbs_over_a_home_with_no_instance_print_their_documents`) and over a home another user could write (`verbs_over_a_home_another_user_could_write_print_their_documents`), and pins the home resolution order `--home` → the `VIOLA_NAME` + `VIOLA_DIR` instance home → `<user home>/.viola`, canonicalised once it exists (`home_resolution_takes_the_flag_then_the_session_then_the_default`, `two_homes_are_isolated`, `a_home_reached_through_a_link_is_canonicalised`), backed by the `src/cmd/mod.rs` units `resolve_home_*`, `canonical_home_*` and `plain_drive_*` (the plain drive form on Windows); the CLI verbs' `VIOLA_DIR` step and the canonicalisation are a widening the founder ratified (security-plan §Input Validation). A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`) and a path oracle compares `resolved(home)`, never `TestHome::path()`.»
  add: «  - `Frozen` (`tests/cli_instance_state.rs`) is a guard on Windows and on Unix, a `SIGSTOP` on Unix; `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` stand on it.»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = '`instance-dead` · `server-verify-failed`, the server cause'

[[pattern]]
id = "tp2-02"
fixed = '{"error":"instance-unreachable","detail":null}` and text'

[[pattern]]
id = "tp2-03"
fixed = '{"error":"instance-unreachable","detail":null}` before any snapshot read'

[[pattern]]
id = "tp2-04"
fixed = '`instance-unreachable` with `detail: null`'

[[pattern]]
id = "tp2-05"
fixed = '{"v":1,"error":"instance-unreachable","detail":null}'
