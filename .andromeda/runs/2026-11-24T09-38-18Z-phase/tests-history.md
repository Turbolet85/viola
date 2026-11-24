@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive in the test plan) — put `viola revive` into the verb lists and made it the first reader of the read-or-replay function. A replayed snapshot carries no `cwd`, and a revived start's first record is `wheel{driver, start}`. It added the `strict_instance_*` instance-check cases and listed revive's exit-1 causes in preflight order as cases of `tests/cli_revive.rs`. Why it matters: `adopt` is built on revive's start path. Its preflight order, refusal tests and replay behaviour are the precedent `adopt` extends.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (fake agent's resume payload) — under `--resume <id>` the fake agent sets `source:resume` and `session_id` on the launch SessionStart; `--fork-session` gives a compiled id. The plan's `wait_endpoint_gone` after a kill was disproved: a killed wrapper leaves its socket file. Why it matters: an `adopt` resume over the fake agent rests on this resume rewrite. The kill-path wait trap also applies to any adopt test that kills a wrapper.

@history_item n=3 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the fake agent's `--resume` builds on the recorded `SessionStart.resume-1.json` where the set holds it, else on `SessionStart.default`. Run E recorded the resume variant and the drift contract replays it. Why it matters: this is the recorded resume fixture an adopt-resume E2E replays.

@history_item n=4 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (strict-modes cases without a log line) — revive's strict-modes refusal became the stderr pair alone, with no log line, no child and the mode left as found, on both arms. The check comes first, ahead of the wrapper log. The group-writable-home case reads the mode while the revive still runs. Why it matters: this is the trap and test shape for `adopt`'s own strict-modes refusal (rows `contract/revive-exit-codes/1`, `contract/strict-modes-refusal/revive`).

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — `claude agents --json` is answered by `viola-fake-agent` placed as `claude` in a test-scoped bin dir (`tests/support/fake.rs` `claude_bin` / `claude_in` / `empty_bin`). It reads `agents-mode`, `agents-rows.json` and the `sessions/<pid>.json` registry beside its exe, with no env var. Every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`) so no test reads the host's live sessions. Why it matters: `adopt`'s live-elsewhere refusal and plain-session id need exactly this seam. A test must never reach the operator's real sessions (rows `decision/agents-stub-from-files-no-env`, `decision/tests-never-read-host-sessions`).

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (revive's session-live) — revive's preflight gained the `session-live` reading as its fourth. Witnesses: `revive_of_a_session_open_in_another_claude_is_session_live` (exit 1, one `process-exit` `session-live`, no child), `revive_passes_the_session_live_reading` (`--fork`), `revive_whose_agents_read_fails_starts` (fail open) and the `preflight_order_*` units. Why it matters: this is the live-elsewhere refusal that guards `adopt`. Its fail-open reading and its tests are the template (row `decision/session-live-read-fails-open`).

@history_item n=7 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json` on both arms, with one `revive_json_*` case per code. The completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` landed: the `Options:` of `viola --help`, the `VIOLA_*` literals and the config keys must all be rows of the controls table. `tests/cli_machine_contract.rs` holds each verb's document and literal exit, and the `resolve_home_*` units hold the home resolution. A test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`. Why it matters: a new verb with `--name` / `--parent` flags must join the controls table and the machine-contract cases.

@history_item n=8 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the CLI verbs' `VIOLA_DIR` home step and its canonicalisation were ratified as a widening. Why it matters: `adopt`'s home resolution follows `decision/home-resolution-order`, which rests on this.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs`. `help_lists_only_served_verbs` runs each served verb's `--help` (exit 0). No trycmd is used. Why it matters: adding `adopt` to the grouped verb table means updating the `HELP` literal and the served-verbs case (rows `contract/viola-help/verb-table`, `decision/human-output-pinned-by-literals`).

@history_item n=10 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `help_lists_only_served_verbs` reads the table's words from the group rows alone, each served verb path answering `--help`. Why it matters: this is the help-pin shape a new verb row must satisfy.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (help table and instance check) — when the link verbs became served they joined the help table, and `help_lists_only_served_verbs`' excluded list shrank. It also records an order trap: opening a role file narrows an existing home to 0700, so an instance check placed after it passes a 0770 home. The check must therefore come before any log. Why it matters: this is the precedent for a new verb joining help, and the trap behind `decision/strict-modes-before-any-log`, which `adopt` must keep.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (start's home-tree check) — `run`'s and `revive`'s start run `check_home_tree` before the wrapper log. A refused tree exits 1 with the home or instance pair, no log, no line and no child. Witnesses: `run_refuses_a_widened_home_tree_before_its_log`, and under `revive --json` the `strict-modes-failed` document. Why it matters: `adopt` shares that start, and its refusal must hold the same before-the-log order (rows `contract/viola-run-start/home-tree-check`, `decision/widened-home-tree-refused-at-start`).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (readers on the replay) — every snapshot reader moved to `read_snapshot_or_replay`. Revive's preflight reads once for its collision check and its recorded-directory reading. The collision check over a replayed snapshot reads the beat alone. A replayed snapshot gives no endpoint and no cwd, so revive over it is refused `cwd-missing`. Why it matters: `adopt` must define where cwd comes from when no snapshot ever recorded one, and this entry is the measured reason revive cannot supply it (rows `decision/replayed-snapshot-has-no-endpoint`, `decision/collision-check-over-replay-reads-beat`).

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built) — `tests/chaos_snapshot_replay.rs` landed: `a_start_over_an_unreadable_snapshot_replays_the_log` and `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`. Why it matters: these are the replay and collision guards that an adopted name's start over an existing log must keep green.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes refusals' exits) — exit 1 is now `run` / `mcp` / `revive` / `verify`, and 21 is every other CLI verb. A refused check at each entry point writes no line. Why it matters: `adopt`'s strict-modes exit and its no-line promise have to be placed in this list (rows `contract/strict-modes-refusal/*`).

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (bounded remove-the-guard witnesses) — a remove-the-guard witness hung 27 minutes under nextest's default profile, which has no kill line. Witnesses are now bounded through `tests/support/cli.rs` `Running` under `WITHIN`. Why it matters: any control `adopt` adds whose neutralised guard lets a product process start needs a bounded witness (row `decision/bounded-remove-the-guard-witness`).

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive `--json`'s codes) — a start refusal after a passed preflight prints one exit-1 document under `revive --json`, before `spawn_child`. The codes became a closed list (ten at that chunk), and a `pump_child` error prints no document. Rows exist in `tests/cross_exit_causes.rs` beside the `tests/cli_revive.rs` cases. Why it matters: `adopt` reuses these closed refusal codes. Any new closed word (a malformed id, a second adopt, a held name) extends this register and its `revive-refusal-code/*` rows.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (stale holder) — a stale holder refuses the start exit 1 `stale-heartbeat`, and a live holder or a fresh replayed beat refuses it `already-live`. Witnesses are `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`, both on `Frozen`. Why it matters: an adopt onto a name already held must map to these codes (rows `contract/run-refusal/*`, `contract/start-exit-1/*`).

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the table built) — `tests/cross_exit_causes.rs` asserts exit, `--json` document and the last `hint:` line per cause in both modes. Its self-checks include `no_two_codes_share_a_hint`, `no_hint_names_release` and `every_code_is_a_code_word`. Why it matters: each new `adopt` cause needs its own hint and a table row, and the self-checks will fail on a shared hint (rows `decision/hint-names-its-cause`, `decision/each-exit-cause-own-hint`).

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (shared start helpers and the root `cross_` binary) — `tests/support/start.rs` shares `first_life`, `stopped_life`, `plant_agents`, `listed`, `revive`, `kill_wrapper` and other start setups. A root test file holds no copy of its own of what two files share. Why it matters: `adopt`'s tests should reuse and extend these helpers, not copy them.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (unwrapped name, bare temp-dir trap) — a name with no instance directory reads `unwrapped-name` (`without_endpoint`). Trap: a bare temp-dir home reads 0755 and is refused before reaching the check a case names, so an oracle moved by a shared body can hide a test that never reached its subject. Why it matters: an adopted plain session is an unwrapped row until adopted. Adopt's refusal cases must start from viola-made homes.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `tests/contract_error_bodies.rs` enumerates every error body (`CLI_DOCUMENTS`, `NOT_ERROR_BODIES`, `PLUMBING`). The census `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` fail on any unnamed writer. Why it matters: any `--json` refusal document `adopt` writes must be named in the enumeration with a witness (rows `decision/error-writers-each-named`, `decision/error-bodies-hold-no-path`).

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (start's socket-directory refusal) — the start checks the socket directory after the version gate and before the spawn, and revive's start shares the check. A refused directory exits 1 with no snapshot, child or bind, and one `process-exit` `socket-dir-refused`. Witnesses: `security_negative_a_start_refuses_the_socket_directory_before_any_bind` and the `cross_exit_causes` row. Why it matters: `adopt`'s shared start inherits this cause (rows `contract/viola-run-start/socket-dir-check`, `contract/start-exit-1/socket-dir-refused`).

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`revive`'s code list) — `revive --json`'s exit-1 codes gained `socket-dir-refused` after `squatted-name`, with `internal-error` last. Why it matters: this is the order of the closed list `adopt`'s codes join (row `contract/revive-refusal-code/socket-dir-refused`).

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (program resolution) — the `.cmd` / `.bat` refusal and `batch-script-child` were retired, so revive's closed `--json` codes read ten. Program resolution is a given path as given, or a bare name on `PATH`. Why it matters: this is the current count and shape of the revive refusal register `adopt` reuses.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (kill line) — `[profile.ci]`'s 20 s kill line read twenty-six verify-driven binaries after `conpty_sideload` left. Its rule is the `binary(…)` names on that filter line. Why it matters: a new stamped-home binary for `adopt` joins this line by name. It also frames the folded CI item about the case killed at the 20 s line.

@history_item n=27 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the standing rule: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways. Why it matters: this governs the folded CI item. The two `security_negatives_env_capture` cases get their margin by class or by less work, never by raising the kill for every test.

@history_item n=28 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (override order and class) — a test that waits by design joined the `verify_window_` class by its name, not by a new override. The override order is pinned by a unit, the first match winning. A planted hang read TIMEOUT under the class kill. Why it matters: this is the precedent for giving the two slow env-capture cases a class margin.

@history_item n=29 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (paste-hint class) — the paste-hint class got its own override with a planted-hang pair (TIMEOUT/PASS), never a raised default, and `mutants_paste_hint_override_holds_the_class_above_the_default_kill` pins it. Why it matters: this is the form the folded CI item's margin should take: a named class with a measured floor and both controls.

@history_item n=30 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `WITHIN` moved 7 s to 12 s and the mutants default kill 10 s to 15 s, below cargo-mutants' 20 s floor. This was a stated departure from the "never raise the default" line, on the operator's word. Why it matters: these are the current wait and kill bounds the env-capture cases are measured against, and it is a recorded exception the folded item must not quietly repeat.

@history_item n=31 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `stamped_home` copies one verify stamp per build, taken under a lock (`stamped_home_reuses_one_stamp_for_every_home`). That is what keeps each stamped-home test under the 20 s kill. Why it matters: reducing the work, rather than raising the kill, is the measured remedy behind rows `decision/one-verify-stamp-per-build` and `decision/verify-driven-tests-20s-kill`.

@history_item n=32 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — the `verify_window_` 80 s override was widened to the fake-agent replay tests, and the harness abort wait was cut to 15 s so a stuck boot ends below the ci kill. Why it matters: these are recent precedents for fitting slow cases under the kill by class and by bound (row `decision/harness-abort-wait-below-ci-kill`).

@history_item n=33 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the identity floor grew to thirteen names, and test oracles name the floor by literal, never by count. A strip control's oracle must keep the guarded names, or removing the floor guard is not seen (`tui_env_strip`). Why it matters: the folded CI item's `security_negatives_env_capture` cases test the env strip/keep path, and `adopt`'s child inherits the R8 strip (rows `decision/strip-control-oracle-keeps-guarded-names`, `decision/identity-floor-tested-against-literal-copy`).

@history_item n=34 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the start writes a `lineage` line (source wrapper) after `wheel`. Replay recovers lineage from the last line, and a later line with no parent clears it. `list --json` items gain `parent?` / `role?`, and boot readiness reads the lineage line. Why it matters: `adopt`'s name and parent land as this lineage record and the board reads it. This is the register `adopt` writes into (rows `contract/run-start-events/order`, `decision/snapshot-replay-fields`, `contract/viola-list-json/items`).

@history_item n=35 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — the start writes a `launch` line third. Replay recovers the launch pair. `list --json` gains `model?` / `effort?`, and the fake agent's `start` receipt carries the model and effort handed on argv. The replay proof goes through `list --json`, because a revive over a replayed snapshot is refused `cwd-missing`. Why it matters: `adopt` must give its cwd/model/effort a defined source, and the receipt is the test seam proving the pair reached the child.

@history_item n=36 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — a start under `--origin` writes one `window` line before session-start. Replay recovers window, and list items gain `window?` / `origin?`. The fake agent stands as `foot`, and real placement is proven only live (`decision/window-placement-proven-live`). It also added `decision/echo-oracle-skips-allowed-word` and the PROVISIONAL `decision/live-rig-clean-start`: the rig unsets inherited `VIOLA_*` names and makes no board read with `claude` on `PATH`. Why it matters: the scope's fork on whether `adopt` opens a terminal or runs in place rides on these records. Any adopt live reading must keep the rig clean of the firing session.

@history_item n=37 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `viola focus` joined `decision/strict-modes-before-any-log` and `contract/strict-modes-refusal/no-line` (PROVISIONAL). A real raise is proven only by the gated live reading R3. Why it matters: this is the newest precedent for adding a verb to these two rows, and for planning a live-only clause gated on a reading.

@history_item n=38 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — viola up's board scan joined the strict-modes-before-any-log and no-line rows, while its later home-tree check writes a `process-exit`. `boot-time-unread` became an exception to `decision/hint-names-its-cause`. It added the gated R4 rig, `decision/live-rig-root-outside-repositories`, `decision/child-fd-table-read-after-launch-settles` and the fake-foot spawn record (all PROVISIONAL). Why it matters: this is the latest verb that rests on revive-like starts with gated live readings. Its exceptions and rig rules bind `adopt`'s live clause planned GATED.

@history_item n=39 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `decision/standing-live-rig` replaces the old live-readings row. One committed test-only rig is used: its rehearsals over the fake agent run in CI, and its live starts are run by hand under the founder's start cap. Why it matters: every live reading of `adopt` (which ends a real session and spends real starts) must go through this rig, gated on the operator's grant.

@history_item n=40 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `decision/live-failing-reading-not-verification`: a live reading that fails is recorded as read, never as verification. Every test-started verify runs under a test-owned user home. Why it matters: this is how `adopt`'s gated live readings must be recorded.

@history_item n=41 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — every long-lived viola child a root test spawns is tied to its test thread by a parent-death kill, and a census holds that no root test spawns viola untied. Why it matters: a new `adopt` test that spawns the wrapper must use the tied spawn, or the census fails.

@history_item n=42 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `decision/run-under-test-owned-user-home`: every test-started `viola run` / `viola revive` runs under a test-owned user home, because the start's trust step writes the Claude config. Trust-start test folders sit outside any repository. Why it matters: `adopt` shares revive's start, so its tests must run under a test-owned HOME.

@history_item n=43 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — `decision/run-start-verify-and-trust-after-bind` was added: the start's automatic verify and its trust record run after the exclusive bind and before the first snapshot. Why it matters: `adopt`'s start inherits this step order, which its test of start events and refusals must respect.

@history_item n=44 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the start-wheel record row now allows a `wheel{human, human-input}` before session-start when trust keys pass through. `decision/start-lines-read-from-start-offset` says a reader of a start's lines reads from that start's own offset, since a reused name keeps the earlier lines at the head of the log. Why it matters: `adopt` resumes under a given name, possibly over an existing log, so its record assertions must read from the start's offset (rows `contract/event-log/start-wheel-record`).

@history_item n=45 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned copy is placed no-clobber and mode-first (`place_private_new`), so two first starts on one home both land. Why it matters: `adopt`'s start shares the pinned copy (row `decision/private-placement-mode-first`).

@history_item n=46 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `decision/single-writer-state-files`: only the wrapper writes `snapshot.json` and only verify writes `stamps.json`, the verify a keyed start runs included. Every other writer-capable entry point leaves both byte-identical. Why it matters: `adopt` must not write the snapshot itself. The name and parent reach the first snapshot through the wrapper.

@history_item n=47 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — `decision/stamping-verify-test-owned-runtime-dir`: every stamping verify runs with a test-owned runtime dir. Why it matters: new stamped-home adopt tests inherit this setup through the shared helpers.

@history_item n=48 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (CLI tests over `std::process::Command`) — CLI tests run `std::process::Command` on `CARGO_BIN_EXE_viola`. Fixed strings are literals in the test file. Never use `.output()` on the long-lived wrapper; use `.spawn()` plus the `Running` guard. Why it matters: this is the driver and the ban an `adopt` E2E test (which starts a long-lived wrapper) must follow (row `decision/cli-e2e-via-process`).

@history_item n=49 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — the E2E layer regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, and the integration layer is its complement. Why it matters: a new adopt test binary's name prefix decides its layer (row `decision/e2e-layer-by-binary-prefix`).

@history_item n=50 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — there is no per-chunk, pre-push or CI mutation gate. Mutation runs at the epoch-boundary audit and on demand through `run --mutants`. Why it matters: this settles the conflict between picked rows `decision/mutation-testing` (per chunk) and `decision/mutation-at-epoch-boundary` for this chunk's plan.
