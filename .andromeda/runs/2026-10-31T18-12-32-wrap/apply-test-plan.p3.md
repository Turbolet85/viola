# Apply — test-plan · part 3 of 4

## Verdicts
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:76+#1
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:91#1
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:260+#1
- fanout-test-plan--cascade-g3.md 4 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:274#1
- fanout-test-plan--cascade-g3.md 5 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:275#1
- fanout-test-plan--cascade-g3.md 6 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:275+#1
- fanout-test-plan--cascade-g3.md 7 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:276+#1
- fanout-test-plan--cascade-g3.md 8 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:279#1
- fanout-test-plan--cascade-g3.md 9 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:282#1
- fanout-test-plan--cascade-g3.md 10 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:283#1
- fanout-test-plan--cascade-g3.md 11 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:284#1
- fanout-test-plan--cascade-g3.md 12 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:284+#1
- fanout-test-plan--cascade-g3.md 13 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:287+#1
- fanout-test-plan--cascade-g3.md 14 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:292#1
- fanout-test-plan--cascade-g3.md 15 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:293#1
- fanout-test-plan--cascade-g3.md 16 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:293+#1
- fanout-test-plan--cascade-g3.md 17 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:293+#2
- fanout-test-plan--cascade-g3.md 18 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:293+#3
- fanout-test-plan--cascade-g3.md 19 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:295#1
- fanout-test-plan--cascade-g3.md 20 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:300+#1
- fanout-test-plan--cascade-g3.md 21 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:307+#1
- fanout-test-plan--cascade-g3.md 22 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:319#1
- fanout-test-plan--cascade-g3.md 23 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:339#1
- fanout-test-plan--cascade-g3.md 24 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:340#1

## Edits
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:76
  anchor: « On live `claude` 2.1.296 on the Linux dev host, a local measurement and no suite, a product `wait --json` woke on `turn-ended` with `background_tasks` 0 and `last` read the turn (as measured at `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`).»
  add: « As landed, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`) witnesses MCP's lost cause, and the `mcp_tools` row after `cleanup` reads its cause from the directory: a home kept by `AGENT_RUN_KEEP_HOMES` reads `instance-dead`, a removed one `unwrapped-name`, its body compared with the CLI's.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:91
  old: «Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text»
  new: «Step 7: CLI exit 21 with `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`: `endpoint-lost` for a wrapper killed under a parked `wait`, and after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one; MCP `is_error:true` with the same body (the CLI document less `v`), text»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:260
  anchor: «  - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`»
  add: «  - Under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout and leaves stderr empty, witnessed by `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`) and `a_frame_verb_that_cannot_open_its_log_is_internal_error` (`tests/cross_exit_causes.rs`); without `--json` the line `error: internal error` stands byte for byte, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:274
  old: «(`cross_exit_causes.rs`, rstest `#[case]` table, one negative test per cause). Exit 21 and exit 1 each have several causes. Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause. A shared hint across causes fails the table, and exact strings are pinned with trycmd once the product defines them.»
  new: «(`tests/cross_exit_causes.rs`, a root `cross_` binary, `required-features = ["fake-agent"]`, on the `ci` profile's kill line; rstest `#[case]` by mode, one negative test per cause). Exit 21 and exit 1 each have several causes. Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause, a fault carrying none. A shared hint across causes fails the table, and the exact strings are literals in the file's constants.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:275
  old: «`cross_exit_causes.rs` does not exist and no rstest table joins the causes: each built cause is a case of the binary its row names.»
  new: «`tests/cross_exit_causes.rs` is built; a cause its top comment names outside the table (server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, `viola mcp`'s two silent exits) is a case of the binary its row names.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:275
  anchor: « As landed, `cross_exit_causes.rs` does not exist and no rstest table joins the causes: each built cause is a case of the binary its row names.»
  add: « `tests/cross_exit_causes.rs` asserts each cause in both modes where a mode exists: the document under `--json`, the human form ending in its hint, and each logged cause's one `process-exit` line with its own `detail`.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:276
  anchor: «  - Exit 21 (`instance-unreachable`) causes:»
  add: « As landed, exit 21's five closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the eight frame verbs, `list` and `plugin install`, with `error` `instance-unreachable` and MCP's text `refused: instance-unreachable`.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:279
  old: «(`tests/security_negatives_strict_modes.rs`, the three CI OSes), not `cross_exit_causes.rs` rows: a widened instance is refused first, exit 21 with `link`'s instance-state pair or the `instance-unreachable` document, no frame and no line.»
  new: «(`tests/security_negatives_strict_modes.rs`, the three CI OSes), beside the table's `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`: a widened instance is refused first, exit 21 with `link`'s instance-state pair or the `instance-unreachable` document, `detail` `strict-modes-failed`, no frame and no line.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:282
  old: «its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.»
  new: «its `--json` document is `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:283
  old: «no instance is running for the name: the endpoint is gone after `cleanup`;»
  new: «no instance is running for the name: the endpoint is gone after `cleanup` with the instance's directory standing, `detail` `instance-dead`, the table's `a_stopped_instance_is_instance_dead`;»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:284
  old: «the name is an unwrapped `claude agents --json` row, never a target (E1); as landed a `tests/cli_list.rs` case, `e1_send_naming_an_unwrapped_row_is_unreachable`, not a `cross_exit_causes.rs` row, proving exit 21 only: the unwrapped-name hint is not built, owed to the existing entry "Exit-cause code catalogue" (`v1-18`)»
  new: «the name holds no instance directory, as an unwrapped `claude agents --json` row, never a target (E1), or a name never started: `detail` `unwrapped-name`, its own hint; as landed the table's `a_name_never_started_is_unwrapped_name` and the `tests/cli_list.rs` case `e1_send_naming_an_unwrapped_row_is_unreachable`»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:284
  anchor: «    - the name is an unwrapped `claude agents --json` row, never a target (E1); as landed a `tests/cli_list.rs` case, `e1_send_naming_an_unwrapped_row_is_unreachable`, not a `cross_exit_causes.rs` row, proving exit 21 only: the unwrapped-name hint is not built, owed to the existing entry "Exit-cause code catalogue" (`v1-18`)»
  add: «    - a wrapper lost with the call open: `detail` `endpoint-lost`, `during:"call"`, its own hint, the table's `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:287
  anchor: «    - the heartbeat is `stale` (live pid, heartbeat older than 5 s)»
  add: « As landed, this cause prints and logs `stale-heartbeat`. A frame verb against a stale instance reads `instance-dead`, because `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1's cause alone.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:292
  old: «not rows of `cross_exit_causes.rs`»
  new: «beside the table's rows `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live` and `a_recorded_directory_gone_is_cwd_missing`»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:293
  old: «prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".»
  new: «prints one exit-1 document under `revive --json`, before `spawn_child`, each a row of the table (`a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`, `a_held_endpoint_is_squatted_name`, `a_revive_with_no_claude_on_its_path_is_internal_error`).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:293
  anchor: « A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".»
  add: « `viola revive --json` prints one exit-1 document for `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:293
  anchor: « A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".»
  add: « A `pump_child` error under `revive --json` prints no document, because the child has held the terminal; a program not found at the start is the `internal-error` row named above.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:293
  anchor: « A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".»
  add: « `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` is `verification-matrix.json#v1-39`'s Windows witness, and `run` passes `json` false to `Launch`, so its human pairs stand byte for byte.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:295
  old: «(`hint: viola list` last on a live name)»
  new: «(`hint: viola list` last on a live name); `already-live` is a live holder's and a fresh replayed beat's code, and a stale holder in the same check prints and logs `stale-heartbeat` (`revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`, `tests/cli_instance_state.rs`)»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:300
  anchor: «  - Driver-facing hints never contain `release`: this holds for every refusal row in the table (`human-typing`, `budget-paused`), not only in Paths 5 and 6.»
  add: « The table's self-checks are `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:307
  anchor: «  - **Unix socket directory refusal** (`#[cfg(target_os = "linux")]`, IPC access control). With a test-owned `XDG_RUNTIME_DIR`, pre-create `viola/` as (a) a symlink to another 0700 dir and (b) a 0755 dir. In each case `viola run` exits 1, a CLI client verb against an existing instance exits 21, and `viola hook <event>` exits 0 with no body and writes no frame. The foreign-owner case needs another uid, which a non-root runner cannot create, so it runs at unit level: the directory-check function gets an injected `lstat` result with `owner != geteuid()` and must refuse.»
  add: « The socket-directory refusal has no code: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening" with macOS's server check.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:319
  old: «pair or `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty»
  new: «pair or `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:339
  old: «endpoint vanishes during `wait`: exit 21»
  new: «endpoint vanishes during `wait`: exit 21, the `--json` `detail` `endpoint-lost` (`a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`); `tests/chaos_wait_vanish.rs` kills through `support::start::kill_wrapper`»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:340
  old: «and a new `run` exits 1; after SIGKILL»
  new: «and a new `run` exits 1 with `stale-heartbeat` (`run_refuses_a_stale_name`, on the `Frozen` guard); after SIGKILL»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "tp3-02"
fixed = 'exact strings are pinned with trycmd'

[[pattern]]
id = "tp3-03"
fixed = 'does not exist and no rstest table joins the causes'

[[pattern]]
id = "tp3-04"
fixed = 'not `cross_exit_causes.rs` rows'

[[pattern]]
id = "tp3-05"
fixed = 'is every exit-21 cause'

[[pattern]]
id = "tp3-06"
fixed = 'the unwrapped-name hint is not built'

[[pattern]]
id = "tp3-07"
fixed = 'not rows of `cross_exit_causes.rs`'

[[pattern]]
id = "tp3-08"
fixed = 'prints no document under `revive --json`'

[[pattern]]
id = "tp3-09"
fixed = '{"v":1,"error":"instance-unreachable","detail":null}'
