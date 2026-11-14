@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (Path 6 landed in part, expiry as a pure-function case) Path 6's cli/ipc signals became seven `path6_` cases of `tests/cli_budget.rs`; override expiry moved to unit cases of the pure `decide` (`gate_decide_an_expired_override_is_dropped`, `gate_release_override_ends_at_its_instant`), and `release --budget` gives `{budget_paused:false, override_until}`; E5's budget leg landed (`e5_the_replay_reads_what_the_wrapper_wrote`, two `state_replay_` cases); the gate's three evaluation points were named the exception to the clock seam. This is the origin of most budget rows this removal retires (decision/budget-decision-takes-the-instant, decision/override-expiry-tested-at-unit-level, decision/budget-governor-thresholds, contract/release-budget-json/document) and of the E5 budget leg that decision/snapshot-replay-from-log's guard still names.

@history_item n=2 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (strict-modes cases without a log line, the gate's negatives) out-of-range `Percent` falls back to the key's default, never refusing to start: `setting_does_not_disable_the_budget_gate` in `tests/cli_controls_not_disableable.rs`, each row exit 11 with one `parse-rejected{parser:"config-json"}` line; the controls table lists the budget-gate negative as joined. That case is the guard of decision/bad-config-value-falls-back and stands on exit 11; with the threshold keys and exit 11 gone, the guard needs another config key (e.g. `dialog_hold_s`) or retires, and the `Percent` negative of decision/cli-env-vector-negative-tests loses its subject.

@history_item n=3 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the state-file ban scoped) the ban on a hand-written `stamps.json`, `snapshot.json` or `budget.json` binds every booted-wrapper test, `budget.json` only through `viola hook statusline`; a reader's own unit cases may plant a malformed file only by altering the writer's bytes; `cli_budget` joined the verify-driven 20 s kill line. Explains decision/state-files-never-hand-written, decision/budget-json-from-statusline, decision/budget-file-from-statusline-hook and decision/budget-seeded-through-statusline-hook, whose `budget.json` clauses leave with this chunk, and the kill-line entry `cli_budget` that must leave `.config/nextest.toml`.

@history_item n=4 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (Path 6 and Path 1 as landed) named the statusline source, landed the pass-through as root tests (`tests/hook_statusline.rs`, `path6_wrapped_statusline_prints_the_user_output_unchanged` in `tests/cli_instance_state.rs`), and fixed that `budget.json` is written only when the payload holds a `rate_limits` object, the reader taking `resets_at` as epoch seconds or RFC 3339. Source of contract/budget-json/rate-limits, contract/budget-json/write-condition, contract/viola-hook-statusline/budget-json and contract/statusline-payload/resets-at (all leaving) — and of the pass-through rows (contract/viola-hook-statusline/passthrough, decision/statusline-passthrough) that stay for the next entry.

@history_item n=5 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (fake agent statusline options, the reader's property and seeds) the statusline reader's property at 512 cases with `proptest-regressions/statusline.txt`, its strategy drawing whole seconds; `hook_stdin` fuzz seeds reach the statusline reader; `hook_statusline` joined the verify-driven kill line. Explains decision/statusline-proptest-whole-seconds and the statusline parser in decision/property-tests-on-parsers / decision/parser-property-fuzz-coverage: if the `rate_limits` parse leaves, decide what the property and `proptest-regressions/statusline.txt` still cover; the `fuzz/corpus/hook_stdin/statusline-epoch` seed the scope keeps comes from here.

@history_item n=6 marker=2026-10-10-session-links
**2026-10-10-session-links** — (Path 6's four verbs) `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) holds that answer/wait/last/pause are not gated: one `send` refused exit 11 `budget-paused`/`five-hour`. It is the guard of decision/budget-gate-covers-send-only, which retires whole with the gate.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — (the board's readings landed) `list --json`'s `budget`, `budget_paused` and the board's WHEEL cell landed (`path6_list_shows_the_reading_and_the_pause`); the board is pinned by literals in `tests/cli_list.rs` (`CAPTION`, `list_human_board_is_the_literal_rows`) and `bay_line_*`/`board_row_*` units; the harness-session form of Path 6 owed onward. Source of contract/viola-list-json/budget, contract/list-json/budget-paused, contract/list-human/wheel-budget-paused and the `budget_paused?` field of contract/viola-list-json/items — the board header/cells and literal pins this removal must re-pin.

@history_item n=8 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (moved unit tests re-cited) the `send_budget_paused_*` unit tests sit in `src/run/send/budget.rs`, the board units under `src/human/board`; locates the unit cases of the `budget-paused` rung of decision/send-refusal-order that leave with `src/run/send/budget.rs`.

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the relied set and the three statusline arms with nine `check_statusline_*` cases, the `StatusLine` variants replayed byte for byte under `--framing` and checked against `schemas/claude-statusline-fixture.v1.json`, the fake agent's first-Stop status-line replay. The `statusline-rate-limits` row, its fixture variant and the fake agent's rate-limits replay the scope moves out come from here; the shared schema and the `default` variant stay with the next entry.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (twenty-four rows) set the relied count literals that stand now: `[NN/24]`, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` `[&str; 24]`, `contract_ledger_probes` `ROW_IDS: [&str; 24]`, the no-framing and modal summaries. Retiring `statusline-rate-limits` moves every one of these literals 24 → 23 (decision/ledger-row-verify-probe, decision/ledger-probe-contract-suite, contract/verify-output/step-line, contract/viola-verify/stamped-line); the older-stamp oracle (decision/version-gate-needs-all-relied-rows) must be re-read against the new set.

@history_item n=11 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the 2.1.302 set joined the hygiene walk, the Recorded hook payloads row and `contract_ledger_probes`' stamped sets with its status-line variants. That is why `fixtures/claude/2.1.302/StatusLine.rate-limits-1.json` exists beside the 2.1.287 one: both sets' variants, their hygiene walk and the drift contract move together.

@history_item n=12 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the statusline-rate-limits arm's live red) a live `viola verify` reads `statusline-rate-limits` red (`23 pass  1 fail`) because the row reads the account's plan state, not the CLI; owner 0.2.0 (finding F-06i), the arm and its nine `check_statusline_*` cases kept and the ledger at 24 rows. This chunk is the 0.2.0 owner the entry deferred to: retiring the row closes F-06i, and this note must go with it.

@history_item n=13 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `state_events.rs` round-trips the kinds by the rule «the length of `EventKind::ALL`» (16), and the controls table's `SETTINGS` gained `dialog_hold_s` (ten rows, rule `SETTINGS` in the file). Removing `EventKind::BudgetGate` moves that count by its rule (contract/event-line/round-trip, contract/event-line/keys); `dialog_hold_s` is the config key that stays to carry decision/bad-config-value-falls-back once the threshold keys leave.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` (guard of decision/controls-not-disableable) compares `schemas/config.v1.json`'s properties with `CONFIG_KEYS` (`config_keys_are_the_config_schema_properties`); `setting_does_not_disable_the_budget_gate` gained `seven_day_over_one_hundred` and `v_two`. Dropping `five_hour`/`seven_day` must touch the config schema, `CONFIG_KEYS` and the `SETTINGS` rows together, or the completeness case fails.

@history_item n=15 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the harness status shape became `viola list --json` alone, its item, `budget` and `skipped` shapes being the `list` payload contract. Source of contract/agent-run-status-polled-fields/budget and /budget-paused, which leave with the `list` payload's `budget` field (`crates/viola-e2e/src/harness/status.rs`).

@history_item n=16 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (boot's two flags, status over viola list) `boot --statusline-echo` plants the statusline source and writes no `budget.json`; `status` reads `viola list --json`. Source of contract/harness-boot/statusline-echo (its «no budget file» clause goes moot) and of the harness's budget references in `crates/viola-e2e/src/harness/{boot,status}.rs`.

@history_item n=17 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (Path 1 as one flow and Path 6's harness-session leg) Path 6 landed as `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session` (lifted `builder`, still-paused `overseer`, one override record, the WHEEL cell, the statusline branches); Path 1 runs as one flow in `tests/path1_run_start.rs`. The suite this chunk deletes, and the guard of decision/budget-override-per-instance; Path 1 asserts the start records whose `budget-gate` line leaves.

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the MCP tests as landed) `mcp_tools_path6_budget_paused_send` in `crates/viola-e2e/tests/mcp_tools.rs` compares a budget-paused MCP send with the CLI's `--json` less `v`. Source of contract/mcp-tool-result/budget-paused; that case leaves, while the parity rows (contract/mcp-cli-parity/refusal, contract/mcp-tools/result-body) need another refusal to stand on.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (harness keys and the time mechanism) the `ci` profile's 20 s kill line lists its verify-driven binaries by the rule «the `binary(…)` names on that filter line», with `path6_budget_harness` noted as not on it. Deleting `cli_budget` moves that count by its rule; `path6_budget_harness` leaves `.config/nextest.toml` by its own Path 6 entry.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the bootstrap phases without the Windows arms) the kill line reads twenty-six verify-driven binaries and the root waits `Instant::now() + WITHIN` read 31 sites in 23 files, each by its stated rule. Deleting `tests/cli_budget.rs` (on the kill line, with root waits) changes both counts; the precedent is to re-measure each by its rule and amend the bootstrap key file.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the statusline pass-through on every build) the arm runs `/bin/sh -c <command>` unconditionally, `run` writes the `settings.json` override at each start, and Path 6's two-instance leg prints the marker output and files the marker. The pass-through half stays for the next entry; the Path 6 leg carrying it leaves with the Path 6 suite, so the pass-through needs a witness that does not stand on `path6_budget_harness`.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (E5 built) E5 steps 1–3 as `tests/chaos_snapshot_replay.rs`; the replay reads `links`, `budget_paused:true` and `agent_session_id` with `dialog_pending:false` and writes no file. decision/snapshot-replay-fields and decision/snapshot-replay-from-log list `budget_paused`/`budget_override_until` among the recovered fields, and the chaos test asserts them; the scope keeps old disk state readable, so the replay drops the fields while an old log's `budget-gate` lines still read as skipped.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the four readers on the replay) Path 6 step 6 and the statusline signal: `statusline_command` never runs over a replayed snapshot (`statusline_over_a_replayed_snapshot_runs_nothing`). This statusline guard (decision/statusline-command-never-runs-unsafe) stays with the pass-through; only its Path 6 framing leaves here.

@history_item n=24 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ decision/snapshot-replay-from-log` guard: now «a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ cases of cli_budget and cli_links». Deleting `tests/cli_budget.rs` breaks this guard; it must be re-pointed to the `cli_links` case alone.

@history_item n=25 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (the snapshot read and the replay as landed) replay is library code where a field no line gave is absent; E5 was owed for a budget pause from "Budget governor"; `crates/viola-state/tests/state_{events,replay}.rs` were named. These are the crate-level suites whose budget cases the scope lists; it also sets the precedent that a known kind no build emits any more is counted and not returned (decision/tolerant-event-reader).

@history_item n=26 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link line's `since` outside the clock seam) `chrono::Utc::now()` is read only through the clock seam with two exceptions: the budget gate's three evaluation points and `LinkSet::line`. With the gate gone, the §8 Time line and §7 Time-sensitive data must name only `LinkSet::line`.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader enumeration; two owed test gaps) `tests/contract_bounded_readers.rs` holds `BOUNDARIES` rows, the `config.json` thresholds reader among them, checked by `boundaries_each_name_a_bounded_read_that_stands` (each row's read in its file, its witness a defined test). Removing `read_thresholds` makes that row's read and witness stop standing, so the row must leave with it (decision/bounded-readers).

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the bounded readers' cap-driving witnesses) the thresholds row's witness is `read_thresholds_keeps_no_key_past_the_frame_cap` (`src/run/budget.rs`), beside the dialog-hold, env-keep, statusline shell-out and version-gate drain witnesses. decision/bounded-readers-keep-the-cap lists «the thresholds» reader; that clause and the `BOUNDARIES` row leave, the statusline shell-out row staying.

@history_item n=29 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/bounded-readers-keep-the-cap` (added the hyprctl reader beside «the thresholds, dialog hold and env-keep readers»); `~ decision/fakes-for-external-services` (the list of replaced services, keeping «Anthropic usage values by generated statusline stdin»). These are the current texts this chunk amends: the thresholds reader drops from the first, and the usage-values clause from the second once nothing reads rate limits.

@history_item n=30 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — `~ decision/fakes-for-external-services` gained the viola-hyprland planted-stub exception. It is the latest text of a row this chunk edits; keep that exception when the Anthropic-usage clause is removed.

@history_item n=31 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (stamped root seam, boot readiness line 3) `boot` readiness checks `events.ndjson` lines 1–3 through `start_records` (wheel start, budget-gate, session-start). With the `budget-gate` start record gone, contract/boot-readiness/events, contract/run-start-events/order, contract/run-start-records/first-three and contract/viola-run-start/event-order all go from three records to two, and the harness's `start_records` check moves with them.

@history_item n=32 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5's `tests/cli_wheel.rs` covers `pause`/`release`, hints without `release` and `release-from-driver` exit 20. That is the driver `release` guard the scope keeps (contract/cli-release/release-from-driver, contract/viola-release/from-driver); budget cases in `tests/cli_wheel.rs` leave around it.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the exit-cause table is built) `tests/cross_exit_causes.rs` with `CAUSES`' codes, exits and hints, and the self-checks `no_two_codes_share_a_hint` and `no_hint_names_release`. The budget-paused hint and exit 11 leave the hint set those guards walk (decision/driver-hints-never-name-release, decision/each-exit-cause-own-hint); exit 11 must stay unreused (contract/cli-exit-codes/10-14).

@history_item n=34 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the `Mcp.` exchange) `Mcp.tools-list-1.json` in `2.1.302` is recorded only by the test-only `viola-mcp-tap` (between one live `claude -p` start and `viola mcp`), held by `tests/mcp_recorded_exchange.rs` to the served `tools/list` shape and tool names. Editing the MCP `list` description that names the budget hits this fixture; re-recording it needs a live start the directive forbids, which is the fork the scope flags.

@history_item n=35 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the witness `scripts/profraw-census.sh` is a dev-host script with recorded one-shot controls, no unit test and no gate (decision/profraw-census-dev-host-only). The scope edits its budget mention; it stays a dev-host tool with no gate added.
