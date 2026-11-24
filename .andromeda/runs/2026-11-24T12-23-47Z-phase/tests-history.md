@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (all seven sub-entries) — the direct precedent this chunk extends. It added the `launch` start record after `lineage` (`~ contract/run-start-events/order`). It moved boot readiness to four lines (`~ contract/boot-readiness/events`) and added `model?`/`effort?` to list items (`~ contract/viola-list-json/items`, PROVISIONAL W2). It put the config.json default-pair reader into BOUNDARIES (`~ decision/bounded-readers-keep-the-cap`) and made the fake agent's start receipt carry the model and effort it got on argv (`~ contract/fake-agent-receipt/start`). It also made the replay recover the launch pair, last launch line winning (`~ decision/snapshot-replay-fields`, `~ decision/snapshot-replay-from-log`), proved through `list --json` after a cut snapshot because a revive over a replayed snapshot is refused cwd-missing. A switch that records a new pair rides exactly these rows: the start order, the last-line-wins replay rule and the receipt are where "resume replays the NEW pair" gets tested.

@history_item n=2 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (seven sub-entries) — the latest new-verb precedent on the start path. `adopt's start` joined the silent list at the internal-error catch site (`~ contract/cli-human/internal-error`), and adopt joined the strict-modes no-line list (`~ contract/strict-modes-refusal/no-line`). The session-live fail-open rule now covers adopt (`~ decision/session-live-read-fails-open`). `+ decision/adopt-proven-live` keeps the real-CLI clause on the gated reading R5. `~ decision/real-claude-local-only` names the gated live-reading scripts, and `~ decision/verify-driven-tests-20s-kill` gives verify_window_ tests their 80 s kill. `+ decision/child-argv-read-from-proc` is the trap that matters here: the start receipt carries no argv, so any claim about the child's `--model`/`--effort` argv is read from `/proc/<pid>/cmdline`. A switching verb on the restart path must join the same closed lists.

@history_item n=3 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (six sub-entries) — `viola up` re-opens the fleet; this chunk wants its resume to replay the switched pair. It added up's board scan to `~ decision/strict-modes-before-any-log` and `~ contract/strict-modes-refusal/no-line`; only the scan joined, because its later home-tree check writes a process-exit. `~ decision/hint-names-its-cause` gained the boot-time-unread except (a cause no case can drive is held by its unit pair). It also added `+ decision/window-reopen-proven-live` (R4), `+ decision/child-fd-table-read-after-launch-settles` and `+ decision/live-rig-root-outside-repositories`, all PROVISIONAL. Precedent for how a new verb's refusals join the closed lists and the hint rule.

@history_item n=4 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added viola focus to `~ decision/strict-modes-before-any-log` and `~ contract/strict-modes-refusal/no-line` (PROVISIONAL, fork F-A): each new verb that checks strict modes must be named in both closed lists. `+ decision/window-raise-proven-live` is the pattern of a gated live reading whose rehearse form never counts as live.

@history_item n=5 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added the window start record under `--origin` (`~ contract/run-start-events/order`), `window?`/`origin?` list items (`~ contract/viola-list-json/items`) and window to the replay's field set (`~ decision/snapshot-replay-fields`, "the last such line winning"). It added `+ decision/echo-oracle-skips-allowed-word`, a refusal-oracle trap for a verb that echoes model/effort words, and `+ decision/live-rig-clean-start` (PROVISIONAL). A new events record for the switch extends the same order, replay and list rows.

@history_item n=6 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the lineage start record and its replay, last lineage line winning, with a later parent-less line clearing it (`~ decision/snapshot-replay-fields`, `~ contract/run-start-events/order` PROVISIONAL F3, `~ contract/boot-readiness/events`). Also `+ decision/supervisor-spawn-strips-bridge-env`. This is where the last-line-wins replay rule a switch record must follow came from.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — brought revive into the test plan: its exit-1 causes in preflight order, `viola revive` as the replay's first product reader, and the fact that a replayed snapshot carries no cwd. It gave the fake agent its `--resume` payload (source resume, the given session_id) and ten argv options. The kill path waits for the recorded child to be gone. A switch carried out by a restart through revive rests on these surfaces.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (all sub-entries) — built `tests/cross_exit_causes.rs` with the self-checks `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`. It gave exit 21's document a cause in `detail`, and under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}`. `stale-heartbeat` became a start cause of its own, apart from already-live (`Frozen` on Unix). revive `--json` grew its codes, with `pump_child` errors printing no document. `UnreachableCause`, the human hint cause words apart from the log's, and `json_asked` reading `--json` only before `--` all date from here. Any new verb's exit codes, hints and `--json` document join this table and its self-checks.

@history_item n=9 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added `socket-dir-refused` as exit 21's sixth cause (frame verbs only) and as a start exit-1 cause, checked before any snapshot, child or bind, with one process-exit line. It added revive's eleventh code and the `SOCKET_DIR` hint literal. It made private endpoint dirs a test rule (`create_private_dir`, never a bare tempdir) and added `setting_does_not_disable_the_socket_dir_check`. Behind the picked rows `contract/run-refusal/socket-dir-refused`, `contract/exit-21-detail/socket-dir-refused`, `decision/private-dir-for-test-endpoints` and `decision/controls-not-disableable`.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux only and one `ubuntu-latest` leg. It retired `batch-script-child`, taking revive's closed `--json` codes back to ten. The OS-gate census (`contract_os_gated_tests`) reads any windows/macos gate as a fault. It retired the per-OS strict-modes ACL arms, and the server check and its negatives run ungated. These explain `decision/linux-only`, the revive-refusal-code rows and the single CI leg any new test lands on.

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `+ contract/viola-run-start/settings`: a start hands its child no `--settings` and writes no settings.json, and the user's settings stay byte- and mtime-equal. It also retired the settings override (`~ decision/plugin-rewritten-on-start`). This is the row a switch "with no side effect on the user's settings" must keep, and the oracle form (byte- and mtime-equal) its acceptance leg will reuse.

@history_item n=12 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `+ decision/run-under-test-owned-user-home` and `+ decision/harness-wrappers-under-session-user-home`: the start's trust step writes the user's Claude config, so every test-started run or revive runs under a test-owned user home. Any switch test that restarts a session, or that asserts `~/.claude*` byte-identical, must run under a test-owned HOME.

@history_item n=13 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ decision/verify-under-test-owned-user-home` and `+ decision/no-verify-start-outside-test-home` (a census), plus `+ decision/live-failing-reading-not-verification`. These are the companion rules for a keyed start whose verify writes config, and for how a confounded live reading is recorded.

@history_item n=14 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — `+ decision/run-start-verify-and-trust-after-bind`, a picked row: the automatic verify and the trust record run between the bind and the first snapshot. It added the `~ decision/human-never-blocked` except for the verify window, and ended Epoch 4's PROVISIONAL marks. A restart-based switch inherits this start order.

@history_item n=15 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/event-log/start-wheel-record`: every start, revived ones included, appends wheel{driver,start} first and session-start last, with a human-input wheel line before session-start when trust keys pass through. `+ decision/start-lines-read-from-start-offset`: a reused name keeps the earlier starts' lines, so a start's own lines are read from its offset. A switch that restarts the same name must read the second start's lines from its own offset.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — took the budget-gate record out of the start order and replay. `~ contract/viola-send/exit-11` is a picked row: exit 11 is retired and never renumbered or reused, and an older wrapper's word reads unknown with exit 14. Any new exit code must not reuse 11. It also removed `release --budget` and the budget fields from list.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (sub-entries) — the start's `check_home_tree` runs before any log. The four readers moved onto `read_snapshot_or_replay` (`decision/snapshot-read-through-replay`), the collision check over a replay reads the beat alone, and E5 was built (`tests/chaos_snapshot_replay.rs`). The frame verbs and MCP tools were put under the instance check. Strict-modes refusals write no line. `+` the bounded remove-the-guard witness (`decision/bounded-remove-the-guard-witness`). These are behind the picked start-refusal, replay and strict-modes rows that a new verb's reads and refusals must follow.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — the last time CLI verbs were added. The verbs joined the `HELP` literal and `help_lists_only_served_verbs`. Their result lines are pinned by literal, and their instance check runs before any role file or log (the order trap: opening a role file narrows the home to 0700). The CLI frame verbs grew to eight under the server check. Precedent for `decision/frame-verbs-check-instance-first`, `decision/instance-check-before-role-file` and the help pins a new verb moves.

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the server-identity check before any frame (`tests/security_negatives_server_identity.rs`, `request_refuses_an_unverified_server_inside_its_dispatch_span` per frame verb, `server_verdict_rows`). It added the impostor held process and listener, and `live_endpoint` as the endpoint's one reader. If the switch talks to the wrapper over the channel, it becomes a frame verb under these rows (`decision/server-checked-before-any-frame`, `contract/cli-frame-verbs/server-verify-failed`).

@history_item n=20 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive `--json` on both arms. `every_flag_variable_and_config_key_is_a_row_of_the_table` is the completeness case: every `Options:` flag, `VIOLA_*` variable and config key must be a row of the controls table. Home resolution is pinned in `tests/cli_machine_contract.rs`. A new verb's flags (`--model`/`--effort`) fall under the completeness case and `decision/controls-not-disableable`.

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` is pinned by the `HELP` literal and `help_lists_only_served_verbs` in `tests/cli_output_plain.rs`, with no SGR or escape in plain output. The NO_COLOR/TERM rows sit under the controls table. A new verb changes this literal (`decision/human-layouts-pinned-by-literal`, `decision/help-no-bold-underline`, `decision/no-escape-in-plain-output`).

@history_item n=22 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — the help pin was reshaped for a verb with sub-words (`plugin install`, one entry of `setup:`), and each served verb path answers `--help`. Precedent for where a new verb lands in the grouped table.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the error-body enumeration `tests/contract_error_bodies.rs` (census of writers, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`) and the surface witnesses `cli_error_bodies` and `channel_error_bodies`. The reply and fault screens date from here. The exit-2 paths that write a fixed line and no document (`NOT_ERROR_BODIES`) are behind the `decision/cli-json-one-document` except. Any new error document or writer must be named in the enumeration.

@history_item n=24 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs` (`BOUNDARIES`/`EXCEPTIONS`, `product_readers_are_each_bounded_or_named`). A new reader of external input, such as a recorded-pair or config read, must be bounded in-statement or named with a witness (`decision/bounded-readers`).

@history_item n=25 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the identity floor went from eleven names to every floor name (`~ decision/r8-claude-env-strip`, `~ decision/claude-env-strip`, `~ decision/literal-test-oracles`). It added `+ decision/strip-control-oracle-keeps-guarded-names` (guard `tui_env_strip`). A restart-based switch re-spawns the child through the same strip.

@history_item n=26 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ decision/single-writer-state-files`: only the wrapper writes snapshot.json and only verify writes stamps.json, including a keyed start's verify. Every other writer-capable entry point leaves both byte-identical, so a new verb must not write the snapshot itself.

@history_item n=27 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's `session-live` reading through one `claude agents --json` read. The fake agent stands as claude in a test-scoped bin dir with no env, and every test-started list or revive runs under a test-scoped PATH (`decision/tests-never-read-host-sessions`, `decision/agents-stub-from-files-no-env`). A switch that resumes a session goes through the same reading.

@history_item n=28 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — a running turn refuses `send` with exit 13 `turn-running`. That refusal joined the controls table as a control, and every test that sends twice must end the first turn with a scripted Stop. Relevant to the scope's open premise of what a switch mid-turn or holding a dialog does, and whose wheel it needs.

@history_item n=29 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `~ decision/fresh-home-per-test`: a stamped home is a copy of its build's one verify stamp, never a link. Also `+ decision/one-verify-stamp-per-build`. New switch tests start from `stamped_home` under these rules.

@history_item n=30 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound`: root waits are bounded at 12 s (`WITHIN`), below the 15 s mutants kill and cargo-mutants' 20 s floor. A restart-based switch test waits within this bound.

@history_item n=31 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — `~ decision/wrapper-drop-guard` (a parent-death tie), with `+ decision/no-untied-viola-spawn` (a census) and `+ decision/tied-child-spawn-thread`. Every long-lived viola child a new root test spawns must be tied, or the census fails.

@history_item n=32 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — `Frozen` has its one home in `tests/support/frozen.rs` (a SIGSTOP guard), behind `decision/real-process-faults` and the stale-heartbeat refusals a restart path can meet. It also added the OS-gate census.

@history_item n=33 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP `tools/list` is exactly the five tools, and each result equals the CLI's `--json` less `v`. The scope keeps an MCP tool for the switch out unless a fork decides otherwise; if one is added, these pins move.

@history_item n=34 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — the MCP-only `call-deadline` cause (`+ contract/mcp-unreachable-detail/call-deadline`, `~ decision/cli-mcp-identical-payloads` except) and the injected deadline seam (`~ decision/time-faked-through-seams`). These bind only if a fork adds an MCP switch tool.

@history_item n=35 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `+ decision/standing-live-rig` and `~ decision/real-claude-local-only` (ratified 2026-11-22): the real CLI is reached only locally, through verify and the rig. The directive says nothing is live in this phase, and the real CLI observed on the pair is carried to «The fleet on the founder's host».

@history_item n=36 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the CLI tests run over `std::process::Command` against `CARGO_BIN_EXE_viola`, with each fixed string pinned by a literal in its own test file. There is no trycmd or assert_cmd (`decision/cli-e2e-via-process`, `decision/cli-fixed-strings-pinned-by-literal`, `decision/cli-outputs-pinned-by-literal`). This is the form a new verb's output pins take.

@history_item n=37 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — dropped the Cookie clause from `~ decision/logs-carry-no-secrets`, `~ decision/no-secrets-in-logs` and `~ decision/secrets-never-in-logs`. It added `+ decision/os-gated-cases-in-binary-root`. The new verb's process-log lines fall under these secret-scan rows.
