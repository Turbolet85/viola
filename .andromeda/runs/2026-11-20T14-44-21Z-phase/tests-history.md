@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's lines pinned by literal asserts) — `tests/cli_verify.rs` pins verify's `[NN/MM]` step lines, the `stamped` summary, the `unable:`/`hint:` pairs and `error: internal error` by literal asserts. Any new verify output from an automatic run (the run that stamps a new version, a red attempt's report) must be pinned the same way, by literals.

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam) — root `stamped_home` runs `viola verify` against the fake agent and counts as stamped only on exit 0 with a last line `stamped <ver>  <n> pass  0 fail`; `StampedHome::unstamped` serves the unverified-path tests. This is the seam an automatic stamp at `viola run` sits beside. Unverified-path tests must stay unverified unless the case means to stamp.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — `--local-live` refuses `live-in-ci` (exit 2) under CI, builds, then runs one `viola verify` against the real `claude` through the runner seam, with closed failure codes (`build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`). This is the existing local live-verify path and its closed codes. An automatic verify must not leak into CI.

@history_item n=4 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (fake-agent default version) — the fake agent answers `--version` at its recorded set's version, so the wrapper's version gate reads a stamped version (the harness boot default equals it). A new-version case has to drive `--cli-version` to a version with no stamp, which is the unstamped path the chunk automates.

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (five perf rows on an unstamped session) — the `--perf` session boots unstamped (`stamp: false`) so that `pre-tool-use` times the unverified-CLI path. An automatic stamp on `viola run` would turn that session verified and silently change what the perf row times. The perf session's unstamped premise needs a guard or a restatement.

@history_item n=6 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — adds `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (exit 12, no body). This and the other unstamped-home e2e cases rest on a start not stamping its version. Each needs the auto-verify form kept off (or not reached) so the unverified-cli assertions still hold.

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the turn-running control negative) — the controls table includes `answer` on an unstamped home → 12 under every `FAKE_AGENT_HOOK_PANIC` setting. An automatic stamp must not let a setting or the start itself erase that refusal row. The `decision/no-switch-disables-a-control` precedent applies to any opt-out key.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify gained interactive runs (`--screens --turn-stop --trusted-root`). `verify_window_` tests carry no test-side deadline, bounded only by a nextest kill override. A verify started by `viola run` inherits that timing floor, so any test driving it falls under the verify-driven kill rules, not `WITHIN`.

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — `[profile.ci]` gained the verify-driven 20 s override and the `verify_window_` override, and "a verify a test drives has no test-side bound" became the rule. The 7 s bound gave way only for a designed floor. This is the precedent for sizing a run-triggered verify's test bound.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — a stamp of the fourteen older ids reads unverified, and the 20 s kill stands on a measured reading (the longest verify-driven test at 9.98 s). These are the older-stamp and kill-sizing precedents. A stale stamp from an older row set is unverified, which an auto-verify trigger must also detect.

@history_item n=11 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (`--local-live` alone runs no other suite) — because `verify` names no program, it stamps the first `claude` on `PATH`. This bears on which binary an automatic verify started by `viola run` probes: the same resolution `run` used, or PATH's.

@history_item n=12 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — the harness test `verify_window_boot_with_an_unknown_cli_version_is_verify_failed` joined the class: a boot at a version with no fixtures reads `verify-failed`, and the override order is pinned. This is the existing witness of an unknown-version verify failing. It is the shape a red automatic verify reuses.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — adds `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` (SIGINT/SIGTERM: own dirs removed, exact pair, exit 1, no `stamps.json`, `process-exit` `interrupted`). An automatic verify that is interrupted (the run stopped, the user's Ctrl-C) must keep this contract and leave no stamp.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — each time the relied set grows, an older live home stamp (17 rows) reads `row-missing` until its next `run --local-live` re-stamps it. A growing relied set unstamps a version, which is the same trigger as a new CLI version. An auto-verify must say whether a stamp short of the relied set re-verifies.

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines became `[NN/MM]` with `MM` = `LedgerRow::relied()`. `cli_verify` and `contract_ledger_probes` hold per-count literals, and boot stamps every relied row. The literals and counts in the suites move with the relied set, so a chunk that reads a verify's result must take the counts by rule, not as literals of its own.

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E joined, the older-stamp oracle covers the shorter-id stamps, and the no-screen `verify_window_` case went to 42.98 s against the kills with no kill moved. The cost of one full verify (a run-triggered verify's wall time) is measured here and in later kill entries.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (agents-join, revive's session-live) — `revive`'s preflight gained the `session-live` reading (`revive_of_a_session_open_in_another_claude_is_session_live`, `revive_passes_the_session_live_reading`, `revive_whose_agents_read_fails_starts`). Verify's Run B makes one agents read. This is revive's preflight order, which the shared start gate (and any auto-verify at revive's start) must respect.

@history_item n=18 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's exit-1 causes in preflight order, and a revived start's first record being `wheel{driver, start}`. Revive shares `run`'s start and version gate. If the auto-verify form reaches revive's start, its tests sit among these cases.

@history_item n=19 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (boot's two flags, harness_session, status over viola list) — `boot --agents-mode` writes its file after the stamping step 4, because Run B's agents read goes through that copy. `status` reads `viola list --json`, whose items carry `cli_verified`. A harness boot that runs an automatic verify at start must keep this ordering and the status fields.

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (harness keys and the time mechanism) — `harness_session` stamps with `stamp: !opts.unstamped` (the in-test `boot --unstamped`) for Path 7's MCP leg. Path 7 relies on a session staying unstamped, so an automatic stamp at `viola run` must leave that option meaningful.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (Path 7's MCP leg on the default pair, unstamped) — `mcp_tools_path7_unverified_cli_answer` boots the default `overseer` + `builder` pair unstamped: two first starts on one fresh unstamped home. Path 7 is the unverified-CLI scenario this chunk changes. Two concurrent first starts meeting one unstamped version is the double-verify race an automatic form must handle.

@history_item n=22 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `every_flag_variable_and_config_key_is_a_row_of_the_table` landed: every `viola --help` option, `VIOLA_*` literal and `config.v1.json` property must be a row of the controls table (`config_keys_are_the_config_schema_properties`). A consent or opt-out config key or flag added for the automatic verify must join that table and the schema.

@history_item n=23 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — this was the last precedent for adding a `config.json` key. The `dialog_hold_s` row joined `SETTINGS` with a booted case (`setting_does_not_unbound_the_dialog_hold`, each bad value one `parse-rejected{config-json, malformed}` line), and the widening was the founder's own answer. A new config key for the auto-verify's consent follows the same shape.

@history_item n=24 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (strict-modes cases without a log line, the gate's negatives) — an out-of-range config value falls back to that key's default with exactly one `parse-rejected{config-json, malformed}` line and never refuses the start (`decision/bad-config-value-falls-back`). A consent key's bad value takes that rule.

@history_item n=25 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — a booted-wrapper test never hand-writes `ledger/stamps.json`. Only a reader's own unit cases may plant a malformed state file, by altering the writer's bytes. Test cases for "a failed attempt recorded so it is not repeated" must get any such record from the product, not by hand.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log) — `run` checks the home tree before its log opens. `verify` checks the home, `ledger/`, `bin/` and `plugin/` before any step (`run_refuses_a_widened_home_tree_before_its_log`, `verify_refuses_a_widened_ledger_before_any_step`). A verify started by `run` sits after the start's checks and must keep its own refusal order.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the tree, diagnostics, stamps and pin checks at the unit tier) — `update_stamps_refuses_a_widened_ledger_and_leaves_it`. The check runs before the `ledger/` create and again under the lock, because a check under the lock alone comes after the create narrows a widened `ledger/`. This trap bears on any new stamp writer or caller.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the security control negatives) — `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`: both files are byte-identical after every other writer-capable entry point (`decision/single-writer-state-files`). If `viola run` triggers the stamp write, this single-writer census and its wording must be re-read: still verify's process, or a widening.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola) — a test-made home goes through `create_private_dir`, never `create_dir_all`, because a plain home is refused by the read rule (`decision/test-homes-made-private`).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded by a time limit (an unbounded one hung 27 min). A remove-the-guard control for an automatic verify (one that would loop) needs a bounded witness.

@history_item n=31 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration) — `tests/contract_bounded_readers.rs` holds `BOUNDARIES` and `EXCEPTIONS` literal tables and a census, where every product reader of external input is bounded in-statement or named. Any new read added by the auto-verify (verify's output read by `run`, a failed-attempt record) must pass this census.

@history_item n=32 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`) is the version gate drain's cap-driving case. The version gate's reader is bounded and witnessed, and the chunk extends that gate.

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the exit-2 paths that write no document) — the exit-2 paths of `answer` and `send` write a fixed stderr line whatever `--json` says (`NOT_ERROR_BODIES`), which is the named exception to one-document-per-verb. Any new output the auto-verify adds to `run` must fit the closed set or be named there.

@history_item n=34 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the reply screen and the fault screen) — a refusal word outside the closed list stays visible with `unknown`'s exit 14 (`reply_of_reads_the_three_shapes`). A new refusal or code word introduced by the auto-verify (say, a failed-attempt word) meets this rule and the code-word format.

@history_item n=35 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`. A failing row is no refusal and carries no code, verify takes no `--json`, and `verify_window_a_widened_stamps_file_refuses_with_its_code` runs two whole verifies. A red automatic verify reads these codes and must not turn a failing row into a refusal.

@history_item n=36 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table is built) — `tests/cross_exit_causes.rs` asserts each cause's exit, document and own last `hint:` line, with `no_two_codes_share_a_hint`, `no_hint_names_release` and `every_code_is_a_code_word`. Any new cause or hint the auto-verify adds (a failed attempt, a consent refusal) joins this table.

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json`'s ten codes) — a refusal of `run`'s start after a passed preflight prints one exit-1 document under `revive --json` before `spawn_child`. `run` takes no `--json`. Any new start-time refusal from the shared version gate becomes a revive `--json` code and a table row.

@history_item n=38 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder reads `stale-heartbeat`) — a stale holder refuses a start exit 1 with `stale-heartbeat`, while a live holder or a fresh replayed beat stays `already-live`, on a real `Frozen` wrapper. These are the start refusals ahead of the child that an automatic verify before the session must not disturb.

@history_item n=39 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a start's socket directory refusal) — the start's order gained the socket directory check after the version gate and before the spawn, with a refused directory exiting 1 before any snapshot, child or bind. This is the latest change to the start sequence (`decision/run-checks-before-child-spawn`). Placing a verify step in it must keep that order statement true.

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a test binding an endpoint in its own directory makes it through `create_private_dir`, never a bare `tempfile::tempdir()`, which reads 0755 and is refused (`decision/private-dir-for-test-endpoints`).

@history_item n=41 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` became the same on every build, the literals ungated, and the older-stamp oracle reads both shorter sets unverified. The relied set the auto-verify must meet is one Linux set.

@history_item n=42 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (bootstrap phases without the Windows arms) — the `[profile.ci]` kill line lists the verify-driven binaries by its `binary(…)` rule, and `Instant::now() + WITHIN` counts by rule. A new verify-driven binary from this chunk joins that line by name and moves its count.

@history_item n=43 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip by exact name) — the strip removes `CLAUDE*` names by exact match, keeps `claude_env_keep`, and holds the eleven-name floor tested against a literal copy. A verify child started by `run` gets the same strip discipline.

@history_item n=44 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a live `viola verify` on an unchanged `claude` read a row red (`23 pass  1 fail`) because of the account's plan state, not the CLI. A red automatic verify can come from the account or host rather than the version, which matters for how a failed attempt is recorded and cleared.

@history_item n=45 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `decision/version-gate-needs-all-relied-rows` gained "a retired row it also holds changing nothing". `contract/viola-send/exit-11` is retired and never renumbered or reused. `contract/run-start-events/order` now reads the wheel line then session-start. The exit-code rule and the start-records order are what a new auto-verify outcome must not reuse or disturb.

@history_item n=46 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — adds `contract/viola-run-start/settings`: a start hands its child no `--settings`, writes no `settings.json`, and leaves the user's settings byte- and mtime-equal. An automatic verify at start must keep this promise toward the user's real settings.

@history_item n=47 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `decision/ci-never-real-claude` became "never the real claude or the real hyprctl", `decision/fake-agent-as-claude` names the fake on a test-scoped PATH, and `decision/fake-agent-on-test-path` was extended. The auto-verify is proved over the fake agent alone in CI.

@history_item n=48 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `WITHIN` moved 7 s → 12 s and the mutants default kill 10 s → 15 s, and `decision/record-home-names-no-home` was added. These are the wait and kill bounds a run-triggered verify test must sit under, and they record the departure from "never raise the default".

@history_item n=49 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — adds `decision/one-verify-stamp-per-build`: stamped homes copy one stamp per build under a lock, keyed by binary hashes. `decision/fresh-home-per-test` now says a stamped home is a copy, never a link, and the 20 s verify-driven kill's rationale moved to the stamp taken once per build. An automatic verify per test start would break that budget, so tests must not trigger a full verify per start unless they mean to.

@history_item n=50 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — adds `decision/stamping-verify-passes-transcript`: every stamping verify gives the fake agent `--transcript`, or Run B's echo probe row fails. `contract/send-result/unconfirmable` covers any listed command on an unverified CLI. A verify started by `run` over the fake agent must take the same arguments to reach a full stamp.

@history_item n=51 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — adds `decision/standing-live-rig`: one committed test-only rig, rehearsals in CI, live starts by hand under the founder's cap. `decision/real-claude-local-only` routes live reaching of the real CLI through `viola verify` and the rig's live starts. Per scope, any live start of this chunk is a scratch home under the rig.

@history_item n=52 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — every long-lived viola child of a root test is tied by a parent-death kill (`decision/wrapper-drop-guard`). Adds `decision/no-untied-viola-spawn` (census) and `decision/tied-child-spawn-thread`, and an in-test harness ties its stamping verify. A verify spawned by a tied `viola run` must not outlive the test either, so the tie has to reach the grandchild.

@history_item n=53 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — the in-test harness ties its stamping verify by SIGTERM, and the supervisor by SIGKILL. Adds `decision/tied-verify-kill-witness`, because a process-and-dir witness cannot tell a tied verify from an untied one, so it reads the stamp's absence. It also adds `decision/no-untied-harness-boot`. Any killed-test witness for a verify started by `run` must follow this design.

@history_item n=54 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes and removes its own trust record (`contract/verify-stop/signal`). It adds `decision/no-verify-start-outside-test-home`, a census holding every test-started verify start tied to a test-owned home with a closed exempt list, and `decision/verify-under-test-owned-user-home`. A verify started by `viola run` inside tests is a verify start this census and rule must cover. It also adds `decision/live-failing-reading-not-verification`, which a red live attempt is recorded under.

@history_item n=55 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `decision/verify-window-60s-kill` was replaced by `decision/verify-window-80s-kill` (ci and mutants). `decision/record-only-at-zero-failing-rows` gained the rig-only failed-recording exception. These are the current verify kill bound and the rule that a failed verify writes no fixtures.

@history_item n=56 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — adds `decision/run-under-test-owned-user-home` and `decision/harness-wrappers-under-session-user-home`, because a start now writes the user's Claude config. A verify triggered by `run` in tests writes trust and config too, so it must run under the same test-owned user home.

@history_item n=57 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `decision/version-gate-needs-all-relied-rows` now reads "counting: pass, or absent on the browser-tools-prompt row alone". Step and summary lines gained `absent` (`contract/verify-output/step-line`, `contract/verify-output/summary-line`), `contract/local-live-suite/pass` was widened, and `decision/ledger-row-verify-probe` was restated. Whatever reads "verified" after an automatic verify must use this counting rule.

@history_item n=58 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — `decision/record-home-names-no-home` extended to the rig's record arm (refuses `record-home`, exit 3, before any credentials read). It applies if the chunk's live proof records under the rig.

@history_item n=59 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the 2.1.302 set joined `contract_ledger_probes`' stamped sets, and the hygiene walk added `2.1.302`. This is the last time a new CLI version was handled as a recorded set (`decision/fixtures-per-cli-version`, `decision/replay-every-cli-version-fixture`). It also adds `wait_ready`'s readiness-stall report. These are the precedent and the witness for a new-version start in tests.

@history_item n=60 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — adds `decision/screen-fixture-box-rows-as-literals` and the first rmcp advisory ignore (`decision/no-advisory-ignored`). Verify's readiness reads the input box on its rule and prompt rows. This is a change to verify's own probes, which the scope keeps untouched unless the founder's form needs it.

@history_item n=61 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `contract/event-log/start-wheel-record` now allows a `wheel{human, human-input}` before session-start where trust keys pass through, and adds `decision/start-lines-read-from-start-offset`. Any new start-time record from the auto-verify changes the start's record sequence these rows pin, and a reused name's earlier start lines must be read from the start's own offset.

@history_item n=62 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — adds `decision/fired-wake-keeps-wheel`, whose relabel applies only when "the CLI is verified". This is one of the behaviours that turns on once a session goes verified mid-life, so the chunk's "session turning verified when the stamp lands" form must say when the gate's reading changes for it.

@history_item n=63 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `decision/no-child-screen-assertions` was restated (the rig reads closed words: compiled literals, their shapes, box-row cell attributes, never text). This is the latest form of the screen-reading exception that any rig-based live proof of the auto-verify stays inside.
