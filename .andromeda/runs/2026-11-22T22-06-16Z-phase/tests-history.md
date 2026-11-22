@history_item n=1 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration) — moved the test-plan Decisions Log out of the body; with decision/closed-enums-need-decisions-log a new closed value (the `lineage` event kind, any new list column or refusal word) still needs its recorded decision, which later chunks recorded in the amendment entry itself (2026-10-03-mutation-scoring-completion did so for pre-push enums).

@history_item n=2 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — staged boot readiness (process-start lines, snapshot, heartbeat < 5 s) and liveness as `classify(beat_age, same_process)` (5.0 s live, 5.1 s stale); the board's live/stale rows and contract/instance-liveness/* rest on it, and a lineage line written at launch must not disturb the staged readiness checks.

@history_item n=3 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (torn append) — the reader counts torn_lines / unknown_kinds / unknown_fields and rewrites nothing, the appender heals with one `state-recovered` line (`three_counts`, `torn_tail`), and the chaos case stops the wrapper and shortens the log with `File::set_len`; the new `lineage` line must round-trip through these counts and a lineage line on an older build reads as an unknown kind, counted and skipped.

@history_item n=4 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (snapshot read and replay) — the classified read and read-or-replay (`snapshot_cause`, `replay_recovers`), a field no line gave stays absent, the replay writes no file, and `crates/viola-state/tests/` holds `state_events.rs` / `state_replay.rs`; the scope's "read back the same after a crash and a replay" for lineage lands on exactly this replay and these crate-level suites.

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive was the replay's first product reader, a replayed snapshot carries no `cwd`, a revived start's first record is `wheel{driver, start}`, and the instance check / exit-1 causes landed in `tests/cli_revive.rs`; "parent kept by revive" extends this verb and its tests.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (resume payload, waits, kill path) — the fake agent sets `source`/`session_id` under `--resume`, `Wrapper::kill` waits on `holder_gone` because a killed wrapper leaves its socket file; a revive-keeps-parent case after a crash must use the kill path's wait, never wait for the socket file to vanish (decision/killed-wrapper-endpoint-wait).

@history_item n=7 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (state-file ban scoped) — booted-wrapper tests never hand-write `snapshot.json` (or stamps); a reader's own unit cases may plant a malformed state file only by altering the writer's own bytes; lineage replay negatives must follow this rule.

@history_item n=8 marker=2026-10-10-session-links
**2026-10-10-session-links** (links in the replay, kinds) — replay recovers `links` via `apply_link_line` (inline `link_rule_*`), a link line with no string `ts` gets `since: "unknown"`, and `state_events.rs` round-trips every kind by the rule `EventKind::ALL`'s length (was 13 → 15); a new `lineage` kind moves that count and must be shown apart from link edges in the replay.

@history_item n=9 marker=2026-10-10-session-links
**2026-10-10-session-links** (E3/E5) — the implicit link leg of E5 (`e5_the_replay_reads_the_link_the_wrapper_wrote`) and `links_survive_a_wrapper_restart` hold the start's own restore apart from the replay; the "links drawn apart" requirement reads links from these same paths.

@history_item n=10 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board is pinned by literals (`CAPTION`, `list_human_board_is_the_literal_rows`, `bay_line_*` / `board_row_*`), not trycmd; list rows escape every control character; `board::read` is a replay caller; `list --json`'s `torn_lines`/forward-compat legs landed; the fleet-tree drawing must keep the six fixed captions and extend these literal pins.

@history_item n=11 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (moved unit tests) — the board units `bay_line_*` / `board_row_*` moved into the human board module (later `src/human/board/`, per 2026-10-20-epoch-5-cleanup-ii); the tree drawing's unit cases belong there.

@history_item n=12 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the GUI left 0.1.0: the harness status shape is `viola list --json` alone, its item and `skipped` shapes being the list payload contract; any field `list --json` gains (the fork on what it gains) reaches the harness status document too.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — `claude agents --json` is served by the fake agent as `claude` in a test-scoped bin dir from files beside its exe, no env var, and every test-started `list`/`revive` runs under a test-scoped `PATH` (`spawn_on`/`viola_on`); new list and revive cases must keep that PATH discipline.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (board witnesses) — the unwrapped separator, `board_row_*`/`board_join_*`, `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode` (names escaped, columns at caption offsets) and revive's preflight readings; a tree drawing must keep rows one line with columns at caption offsets.

@history_item n=15 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (status over viola list) — harness `status` runs `viola list --json` and reads `ready` from every booted instance `wrapped:true, liveness:"live"`; a reshaped list payload is read by the harness.

@history_item n=16 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive `--json` on both arms with one case per code, and the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` (the `--help` Options, `VIOLA_*` literals, config keys); a new `viola run` parent flag or variable must be a row of the controls table, and test-started viola drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` (the launching session's identity the scope may read).

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` pinned by the `HELP` literal and `help_lists_only_served_verbs`; the SGR parser in `tests/support/sgr.rs`; bold NAME / amber DIALOG under the outer PTY; a new run flag changes the help literal and the tree must keep no SGR when piped.

@history_item n=18 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `tests/cli_output_plain.rs` holds no ESC byte into a pipe and no SGR beyond the parsed set; the "plain when piped" tree form rests on this witness.

@history_item n=19 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (result lines) — the link verbs' three lines pinned by literal, the printed time by shape; `link_verbs_log_the_channel_pair_and_no_names` holds the wrapper log free of driver/driven names; links shown apart on the board must keep names out of the wrapper log.

@history_item n=20 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (E3 built) — `e3_two_links_and_a_send_leave_one_link_record`, `links_an_unlink_survives_a_wrapper_restart`; the driver is a self-reported `VIOLA_NAME` and need not be booted, a precedent for a self-declared parent that is not a booted instance.

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the instance check) — the link verbs run `check_instance` before any role file or log (order trap: opening a role file narrows a 0770 home), the self-pair refused by the wrapper alone; any new parent-reading path must check before it opens.

@history_item n=22 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (raw-channel name negative) — `channel_link_names_are_validated_by_the_wrapper` and the `links_set_method_*` units: bad names `-32602`, data null, no line; the "name-checked" link display rests on the same viola-name parse.

@history_item n=23 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (unserved method line) — the channel's `method_label` names only the closed method list, so an unserved method's response line carries no `method` key; a new channel method needs a label or its tests must expect none.

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`since` outside the clock seam) — `LinkSet::line` stamps `since` with `Utc::now()` directly, asserted by shape only (`is_clock_time`); a lineage record's timestamp shown on the board falls under the same shape-only rule (ratified 2026-11-15T18-01-44-wrap).

@history_item n=25 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (kill line) — `channel_links` joined the `ci` 20 s kill line, its count moving by the `binary(…)` rule; any new stamped-home binary for lineage joins it and moves the count (26 binaries at 2026-11-05-linux-only-code-and-ci).

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (start's home-tree check) — `run`/`revive` check the home tree before the wrapper log opens; a refused tree writes no line and starts no child; the lineage line belongs after those checks, in the start's written records.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (four readers on the replay) — every product snapshot reader is on `read_snapshot_or_replay`; collision check over a replay reads the beat alone; no product reader is on `read_snapshot`; a lineage reader must go through read-or-replay (decision/snapshot-read-through-replay).

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built) — `tests/chaos_snapshot_replay.rs`; the replay reads links and agent_session_id with dialog_pending false and writes no file; every start appends `wheel{driver, start}`; recovering the parent after a crash extends this scenario and the replay field set.

@history_item n=29 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (enumeration, code words) — `tests/contract_error_bodies.rs` names every error writer; `is_code_word` and `sanitised_fault` close refusal words; a new refusal (e.g. an invalid declared parent) needs a named writer, a witness and a code-word name.

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `tests/cross_exit_causes.rs` built with `no_two_codes_share_a_hint`, `no_hint_names_release`, `every_code_is_a_code_word`; the unwrapped-name cause by instance directory (`without_endpoint`); a stale holder reads `stale-heartbeat`; revive's codes; shared start helpers in `tests/support/start.rs`; any new start refusal joins the table with its own hint.

@history_item n=31 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list` on the read rule) — `board::scan` holds the home to the read rule and refuses the whole board exit 21, opening nothing (`list_refuses_a_home_another_user_may_read_with_and_without_an_instance`); the tree board inherits this refusal.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (program resolution) — revive's closed `--json` codes read ten (`batch-script-child` retired); revive-keeps-parent cases assert against that closed list.

@history_item n=33 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the stale-row dim witness `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (exact SGR `[DIM, BOLD, RESET, DIM, RESET]`, no SGR on caption or separator) and `Frozen` moved to `tests/support/frozen.rs`; the tree drawing must keep this exact SGR sequence for a stale row (ungated at 2026-11-05-linux-only-code-and-ci).

@history_item n=34 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — the cross-surface trigger's source became the fixed captions and slot order of CLI `viola list` alone; the fleet-tree view must stay inside those fixed columns.

@history_item n=35 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — decision/snapshot-replay-from-log's guard re-pointed to the `e5_the_replay_reads_` cases (then to cli_links alone by 2026-11-14-budget-gate-removed); a lineage replay case joins or extends this guard.

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Test Scope Summary) — `viola list --json` items lost `budget_paused` and the replay recovers only links, agent_session_id and wheel; run start events read wheel then session-start; these are the rows a lineage field or record changes next.

@history_item n=37 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Test Harness Contract) — boot readiness reads event lines 1 and 2 (wheel start, session-start) and the status shape lost its budget members; a lineage line at launch moves this readiness contract and the start-record order — the trap the budget-gate record once was.

@history_item n=38 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (E2E, start records) — contract/event-log/start-wheel-record states session-start as the second and last start record and contract/run-start-records/first-three was retired; adding a start record (lineage) re-opens this exact count.

@history_item n=39 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — start-wheel-record allows a human-input wheel before session-start, and decision/start-lines-read-from-start-offset: a start's lines are read from its own offset because a reused name keeps earlier starts' lines; directly governs "a reused name never inherits an old parent".

@history_item n=40 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — every long-lived viola child a root test spawns is tied by a parent-death kill, with the `no-untied-viola-spawn` census; new run/revive lineage tests must spawn tied (and viola-e2e boots tied, per 2026-11-17-supply-chain-green-and-booted-tests-end).

@history_item n=41 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — test runs and revives run under a test-owned user home, and harness wrappers under a session user home; a parent-declaring start test must too.

@history_item n=42 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (start order) — + decision/run-start-verify-and-trust-after-bind: between the exclusive bind and the first snapshot a start runs any automatic verify and the trust record; places where a launch-time lineage record can sit in the start sequence (decision/run-checks-before-child-spawn).

@history_item n=43 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses) — a control whose neutralised guard lets a product process start needs a witness bounded by `WITHIN` through `tests/support/cli.rs` `Running` (an unbounded one hung 27 minutes).
