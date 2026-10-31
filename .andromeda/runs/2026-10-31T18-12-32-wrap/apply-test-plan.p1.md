# Apply — test-plan · part 1 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan.md:324#1
- fanout-test-plan.md 3 · folded into fanout-test-plan--cascade-g1.md 8
- fanout-test-plan--cascade-g3.md 25 · applied · .andromeda/test-plan.md:553#1
- fanout-test-plan.md 2 · folded into fanout-test-plan--cascade-g3.md 25
- fanout-test-plan--cascade-g3.md 26 · applied · .andromeda/test-plan.md:610#1, .andromeda/test-plan.md:610#2, .andromeda/test-plan.md:610#3
- fanout-test-plan.md 1 · folded into fanout-test-plan--cascade-g3.md 26
- fanout-test-plan--cascade-g3.md 27 · applied · .andromeda/test-plan.md:619#1, .andromeda/test-plan.md:619#2
- fanout-test-plan.md 5 · applied · .andromeda/test-plan.md:617+#1
- fanout-test-plan.md 6 · applied · .andromeda/test-plan.md:612#1
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:12#1, .andromeda/test-plan-detail/test-scope-summary.md:12#2, .andromeda/test-plan-detail/test-scope-summary.md:12#3
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:34#1, .andromeda/test-plan-detail/test-scope-summary.md:34#2, .andromeda/test-plan-detail/test-scope-summary.md:34#3, .andromeda/test-plan-detail/test-scope-summary.md:34#4
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:37#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:43#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:69#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:72#1
- cascade-compare-test-plan.md 10 · applied · .andromeda/test-plan-detail/test-scope-summary.md:134+#1
- cascade-compare-test-plan.md 11 · applied · .andromeda/test-plan-detail/test-scope-summary.md:182#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-strategy.md:43#1
- fanout-test-plan--cascade-g1.md 9 · applied · .andromeda/test-plan-detail/test-strategy.md:42+#1
- cascade-compare-test-plan.md 12 · applied · .andromeda/test-plan-detail/test-strategy.md:63#1

## Edits
- line: .andromeda/test-plan.md:324
  old: «`path_`, `mcp_`, `cross_` in `viola-e2e`.»
  new: «`path_`, `mcp_`, `cross_` in `viola-e2e`; the root package also holds a `cross_` binary, `tests/cross_exit_causes.rs`, which the E2E layer regex selects as it selects `path1_`.»
- line: .andromeda/test-plan.md:553
  old: «(exit 21; the unwrapped-name hint not built, owed to "Exit-cause code catalogue")»
  new: «(exit 21, `detail` `unwrapped-name` under `--json`, its hint built; the table's `a_name_never_started_is_unwrapped_name`, `tests/cross_exit_causes.rs`, holds a name never started)»
- line: .andromeda/test-plan.md:610
  old: «(`cross_exit_causes.rs`, not built): one negative test per cause of exit 21 and of exit 1, each asserting»
  new: «(`tests/cross_exit_causes.rs`, a root `cross_` binary): one negative test per cause of exit 21 and of exit 1 it holds, each asserting»
- line: .andromeda/test-plan.md:610
  old: «a hint shared across causes fails the table; the server-verification cause is a case of»
  new: «a hint shared across causes fails the table (`no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`); the causes outside it, named in its top comment, are server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits; beside the table, the server-verification cause is a case of»
- line: .andromeda/test-plan.md:610
  old: «driver-facing hints never contain `release`.»
  new: «driver-facing hints never contain `release` (`no_hint_names_release`).»
- line: .andromeda/test-plan.md:612
  old: «(`tests/security_negatives_strict_modes.rs`, three OSes), a Windows x64 ConPTY»
  new: «(`tests/security_negatives_strict_modes.rs`, three OSes), `viola list`'s whole-board refusal exit 21 opening no log and `viola plugin install`'s exit 21 creating nothing (`tests/cross_exit_causes.rs` `strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`), a Windows x64 ConPTY»
- after: .andromeda/test-plan.md:617
  anchor: «  - for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`»
  add: «  - for each cause of exit 21, the MCP `isError` body `{error, detail}` equals the CLI `--json` document less `v`, its text `refused: instance-unreachable` (`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`)»
- line: .andromeda/test-plan.md:619
  old: «an endpoint vanishing during `wait` (exit 21)»
  new: «an endpoint vanishing during `wait` (exit 21, `endpoint-lost`)»
- line: .andromeda/test-plan.md:619
  old: «its name refused to a new `run`,»
  new: «its name refused to a new `run` with `stale-heartbeat`,»
- line: .andromeda/test-plan-detail/test-scope-summary.md:12
  old: «(an impostor server gets no frame and the verb exits 21, beside a control»
  new: «(an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`, beside a control»
- line: .andromeda/test-plan-detail/test-scope-summary.md:12
  old: «(`instance-unreachable`, no frame), and the decision table»
  new: «(`instance-unreachable` with `detail` `server-verify-failed`, no frame), and the decision table»
- line: .andromeda/test-plan-detail/test-scope-summary.md:12
  old: «the macOS cases are owed (§6 Exit-cause matrix).»
  new: «the macOS cases are owed to the route entry "Unix endpoint and home hardening".»
- line: .andromeda/test-plan-detail/test-scope-summary.md:34
  old: «(chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints»
  new: «(an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints»
- line: .andromeda/test-plan-detail/test-scope-summary.md:34
  old: «`no-session`, `session-live`, `cwd-missing`; `--list --json` prints»
  new: «`no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`; `--list --json` prints»
- line: .andromeda/test-plan-detail/test-scope-summary.md:34
  old: «a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue")»
  new: «a `pump_child` error prints no document, the child having held the terminal)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:34
  old: «1 for a refusal (the strict-modes check ahead of the log, or a preflight reading)»
  new: «1 for a refusal (the strict-modes check ahead of the log, a preflight reading or a refusal of the start)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:37
  old: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and writes no line»
  new: «or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and writes no line»
- line: .andromeda/test-plan-detail/test-scope-summary.md:43
  old: «document (its five codes)»
  new: «document (its ten codes)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:69
  old: «`isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault"»
  new: «`isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}` (`<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`) | `{"error":"wrapper-fault"»
- line: .andromeda/test-plan-detail/test-scope-summary.md:72
  old: «returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame»
  new: «returns `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}` with no frame»
- after: .andromeda/test-plan-detail/test-scope-summary.md:134
  anchor: «    - A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).»
  add: «    - A stale holder refuses the start exit 1 with `stale-heartbeat`, a live holder or a fresh replayed beat with `already-live`; a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone (`tests/cli_instance_state.rs` `run_refuses_a_stale_name`, `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`; `tests/cross_exit_causes.rs` `a_live_holder_is_already_live`).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:182
  old: «endpoint vanishes during `wait`: `instance-unreachable`»
  new: «endpoint vanishes during `wait`: `instance-unreachable`, `detail` `endpoint-lost` (`tests/cross_exit_causes.rs` `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`)»
- after: .andromeda/test-plan-detail/test-strategy.md:42
  anchor: «  - **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`. `events.rs` holds the one `events.ndjson` reader and its wait (`events`, `wait_events`) and the `boot` that waits for the session-start record, and the one error-body check, `error_body_hit` and `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes); `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code, and `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions. `fake.rs` holds `empty_bin`, `claude_bin` and `claude_in`, the one place a test plants the fake agent as `claude` (chunk 2026-10-16-unwrapped-sessions-on-the-board). `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened. No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.»
  add: «  - **Shared start helpers:** `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.»
- line: .andromeda/test-plan-detail/test-strategy.md:43
  old: «(`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report»
  new: «(`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`) and a `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report»
- line: .andromeda/test-plan-detail/test-strategy.md:63
  old: «exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line;»
  new: «exit 21 with the instance pair or the `instance-unreachable` document, its `detail` `strict-modes-failed`, no frame and no line;»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = '`cross_` in `viola-e2e`'

[[pattern]]
id = "tp1-02"
fixed = 'unwrapped-name hint not built'

[[pattern]]
id = "tp1-03"
fixed = '`cross_exit_causes.rs`, not built'

[[pattern]]
id = "tp1-04"
fixed = '(`instance-unreachable`, no frame)'

[[pattern]]
id = "tp1-05"
fixed = 'macOS cases are owed (§6 Exit-cause matrix)'

[[pattern]]
id = "tp1-06"
fixed = 'strict-modes check or its preflight prints'

[[pattern]]
id = "tp1-07"
fixed = 'after a passed preflight prints no document'

[[pattern]]
id = "tp1-08"
fixed = 'the strict-modes check ahead of the log, or a preflight reading'

[[pattern]]
id = "tp1-09"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "tp1-10"
fixed = 'its five codes'

[[pattern]]
id = "tp1-11"
fixed = '+ `{"error":"instance-unreachable","detail":null}` |'

[[pattern]]
id = "tp1-12"
fixed = 'returns `isError:true` `{"error":"instance-unreachable","detail":null}`'

[[pattern]]
id = "tp1-13"
fixed = 'the `instance-unreachable` document, no frame and no line'
