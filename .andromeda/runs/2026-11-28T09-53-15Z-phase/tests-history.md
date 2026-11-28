@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. Test Scope Summary: 2 rows) — gave `decision/cli-tests-as-subprocess` its except: `viola console` takes no `--json` and is tested under its own outer PTY, its signal being its exit code and its diagnostics lines. Also widened `decision/structured-json-process-logs` to run, hook, mcp, cli, console (PROVISIONAL, inputs#I2 Q3). The view's tests rest on this same driver and signal and never parse the screen.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. Test Scope Summary: 1 row) — `decision/cli-json-one-document` now excepts the console: it takes no `--json`, and its two refusals are fixed stderr pairs named in `NOT_ERROR_BODIES`. The view adds no document. Any new refusal must keep this form.

@history_item n=3 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (6. E2E Test Strategy: 2 rows) — `decision/hint-names-its-cause` excepts the console's two exit-1 causes (no-terminal, strict-modes-failed): they print no `--json` document. `decision/home-readable-by-others-refused` adds `viola console` exit 1. These are the console's standing refusal witnesses, and the view must not regress them.

@history_item n=4 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (2. Test Strategy; 5. Integration Test Strategy) — `decision/home-another-user-may-read-refused` and `decision/home-read-rule` now name `viola console`: exit 1 with its home pair, writing no line. Added `contract/strict-modes-refusal/console`. Changed `contract/strict-modes-refusal/21` to say console exits 1, and `contract/strict-modes-refusal/no-line` to list the console. The board re-read the view adds must keep the check ahead of any log.

@history_item n=5 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (6. E2E Test Strategy: 3 rows) — added `viola console` to `decision/strict-modes-before-any-log`. Widened `decision/tui-verdicts-from-records`: the fleet console's verdicts come from its `console.ndjson` lines and its terminal's line mode, never from the screen. Added `decision/console-restore-read-on-own-terminal`, because the PTY seam exposes no master fd. This is the load-bearing constraint: the view's rows, `LAST`, the cut and `... <n> more` must be held through `console-frame` fields (`shown`/`hidden`) and unit cases, not a screen read.

@history_item n=6 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (3. Test Harness Contract: 2 rows) — `contract/log-line/process` and `decision/process-logs-via-tracing-subscriber` gained the `console` role: `console.ndjson` is written with or without an instance (PROVISIONAL). `console-frame` gaining `shown`/`hidden` extends this harness-grepped register. `decision/harness-log-fields-owned-by-test-plan` and the diag-line schema check bind the new fields.

@history_item n=7 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (10. Quality Gates; 11. Test Anti-Patterns) — added `decision/boot-paths-declared-once`: the console's start passes through the entry point without the shared start. Gave `decision/literal-test-oracles` its except: the relied count is read from `RELIED_COUNT`, with one literal pin (PROVISIONAL). Test oracles for the view's captions and words stay literals in the test files, never the product's list.

@history_item n=8 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` rows now escape every control character, `\n`, `\t`, DEL and C1 included. `board::read` became the read-or-replay function's second product caller. The board is pinned by literals in `tests/cli_list.rs` (`CAPTION`, `list_human_board_is_the_literal_rows`) and the `bay_line_*`/`board_row_*` units, not by trycmd. `torn_lines` and forward-compat landed on `list --json`. The view reuses this board read model, its escaping and its six-caption literal, so its rows stay pinned the same way.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — `claude agents --json` is answered by the fake agent planted as `claude` in a test-scoped bin dir, reading files beside its exe (`agents-mode`, `agents-rows.json`, `sessions/<pid>.json`) with no env var. Every test-started `list` runs under a test-scoped PATH (`support::cli::spawn_on`/`viola_on`), so a test never reads the host's live sessions. The agents parser fails to unknown. The view's 5 s agents read off the loop needs this same seam.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (board witnesses) — the board cases `list_status_of_a_joined_wrapper_is_idle_then_busy`, `list_of_an_answer_that_does_not_parse_equals_the_no_claude_board` and `list_under_a_name_logs_the_agents_read`. Row integrity landed as `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode` (names escaped, columns aligned to the caption row). These are the precedent witnesses for "each session's state in the board's words" and "rows control-escaped".

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` and the board are pinned by literal, and the SGR parser lives in `tests/support/sgr.rs`. Under the outer PTY the verdicts are bold NAME and amber `DIALOG`. `NO_COLOR`/`TERM`/`COLORTERM` are control cases. Constraint: the console draws plain ASCII with no SGR (CARRY `cry-wz66zaxx`). Any styling claim is checked only by SGR parsing, never by screen text.

@history_item n=12 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the v1-40 stale-row dim witness; the Frozen guard's one home) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` freezes a wrapper through `Frozen` and back-dates its heartbeat 60 s to read a stale row. `Frozen` lives only in `tests/support/frozen.rs`. This is the precedent for a "stalled" (= `stale`) row in the view's tests. Reuse the guard and never copy it.

@history_item n=13 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — liveness is `classify(beat_age, same_process)`, fed ages with no clock. A pid or start-time mismatch is `gone`; for the same process 5.0 s reads live and 5.1 s stale. This backs `decision/heartbeat-staleness-pure-classifier` and `contract/heartbeat-staleness/age-threshold`. The view's LIVE word and its last-activity age must be held by pure functions fed an age, not by test sleeps.

@history_item n=14 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the torn append) — the reader counts torn, unknown-kind and unknown-field lines (`three_counts`) and rewrites nothing. The appender heals, logging one `state-recovered` line (`torn_tail`). This bears on the view's once-a-second re-read and its newest-line age. The age must come from complete lines only, and a torn tail must neither panic nor be taken as the newest line.

@history_item n=15 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — `tests/support/ndjson.rs` is the one complete-lines reader (`decision/complete-lines-reader`). Tests that read the console's live-appended `console.ndjson` must go through it, never a copy of their own.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built: the six replay readers) — `board::read` is one of six replay readers. Over an unreadable snapshot it replays the log, writes no file and logs one `state-recovered` line (`decision/list-reads-live-through-replay`, `decision/snapshot-replay-from-log`). The console reading the board every second means a replayed instance logs `state-recovered` into the console's role file on each pass. Tests must expect that, or the scope's lines-into-the-console-role-file item covers it.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list` on the read rule) — `board::scan` holds the home to the read rule and `instances/` to `check_path` (`board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`). A refused home fails the whole board, `ScanRefused::Home`, with nothing opened. The view calls `scan`/`read` in-process, so the console inherits this refusal and its no-open guarantee.

@history_item n=18 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `contract/viola-list-json/items` gained `parent?`/`role?`. The replay recovers lineage, the last lineage line winning (`decision/snapshot-replay-fields`). Start records and boot readiness gained the lineage line. The view's lineage trees reuse the board's `forest`, and its tests can seed parent/child trees only through real lineage lines from real starts, never hand-written state.

@history_item n=19 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — list items gained `model?`/`effort?`. The start records are now wheel → lineage → launch → session-start (`contract/boot-readiness/events` lines 1–4). The replay recovers the launch pair. The view reads the newest `events.ndjson` line for `LAST`, so fixtures must account for these start records. The list item shape grows only by ratified rows; `LAST` stays out of it.

@history_item n=20 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — list items gained `window? {address, workspace}` and `origin?` (later ratified at the 2026-11-27T19-07-11Z wrap). The fake agent also stands as `foot`, the terminal stand-in (`decision/fake-agent-on-test-path`, `decision/fake-agent-as-claude`, `decision/fakes-for-external-services`). Added `decision/window-placement-proven-live` and `decision/live-rig-clean-start`. The view's "tiled window at quarter-screen" clause sits in this split: the fakes prove words and records, and real placement is a live reading only.

@history_item n=21 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — widened `decision/window-placement-proven-live`: a placing open's kept view is proven only by the gated live reading R7. This is the precedent for the view's compositor half. Its "tiled at a quarter of the screen" clause is a gated live reading at the operator's host, and this chunk's tests never claim it.

@history_item n=22 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — widened `decision/window-placement-proven-live` to a re-open on the recorded workspace. Added `decision/window-reopen-proven-live` (R4). Added `decision/stand-in-terminal-tree-teardown`: one shared teardown ends a stand-in terminal's tree. Added `viola up`'s board scan to `decision/strict-modes-before-any-log`. All were ratified at 2026-11-27T19-07-11Z. A console test that opens a stand-in terminal must use the shared teardown.

@history_item n=23 marker=2026-11-27-epoch-5-cleanup
**2026-11-27-epoch-5-cleanup** — added `decision/r2-refuses-before-anything-starts`: R2 refuses an unfit scratch or a locked desktop before anything is made or started. It also widened the `verify_window_` 80 s class to tests the filter line names (`decision/verify-window-80s-kill`, `decision/verify-driven-tests-20s-kill`). These are the shape for any gated live reading v2-14 needs, and the kill-line register a new console binary must be placed on.

@history_item n=24 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — moved `decision/window-raise-proven-live`, `decision/window-reopen-proven-live`, `decision/live-rig-clean-start`, `decision/live-rig-root-outside-repositories`, `decision/child-fd-table-read-after-launch-settles` and `decision/stand-in-terminal-tree-teardown` from provisional to standing on the operator's word. The live-reading tier the view's gated claim will join is now ratified.

@history_item n=25 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added `viola focus` to `decision/strict-modes-before-any-log` and `contract/strict-modes-refusal/no-line`, and added `decision/window-raise-proven-live` (R3; its rehearse form is never a live reading). This is the precedent for a desktop-facing verb: the refusal comes before any log, and real compositor effects are proven only live.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary: 6 rows) — `contract/viola-list-json/items` lost `budget_paused`, and `contract/viola-list-json/budget` was retired, so the list ok payload is `{items, skipped}`. This shows how the item shape is changed by ratified rows only. The view's unserialised `LAST` must not leak into this item or the MCP list item (`contract/viola-list-json/mirrors-channel`).

@history_item n=27 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — `decision/cli-mcp-identical-payloads` gained its except (`call-deadline` has no CLI document). MCP and CLI payloads stay equal. `LAST` is the console's alone, so the parity witnesses (`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`) must still read equal list items.

@history_item n=28 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — home resolution runs `--home`, then `VIOLA_NAME` + `VIOLA_DIR`, then `~/.viola`, canonicalised (`tests/cli_machine_contract.rs`, `resolve_home_*`). A test-started `viola` drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN`. `2026-10-18-plugin-precedence-live-readings` ratified the `VIOLA_DIR` step. Console tests must run with a test-owned `--home` and no inherited session variables.

@history_item n=29 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `WITHIN` moved from 7 s to 12 s (`decision/within-7s-wait-bound`, `decision/root-wait-bound`), and the mutants default kill from 10 s to 15 s, below cargo-mutants' 20 s floor (`decision/mutants-kill-below-cargo-mutants-timeout`). The view's once-a-second re-read and 5 s agents read are waited on under these bounds: by logged lines with a deadline, never by sleeps.

@history_item n=30 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `decision/fresh-home-per-test` now says a stamped home is a copy of its build's verify stamp, never a link. `decision/one-verify-stamp-per-build` was added. A console test booting wrappers for live rows starts from these homes.

@history_item n=31 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — every long-lived viola child a root test spawns is tied to its test thread by a parent-death kill (`decision/wrapper-drop-guard`). Added `decision/no-untied-viola-spawn`, held by a census, and `decision/tied-child-spawn-thread`. A console spawned under an outer PTY in a new test file must be tied, or the census fails.

@history_item n=32 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — added `decision/run-under-test-owned-user-home`: every test-started `viola run`/`revive` runs under a test-owned user home. Booted fake-agent wrappers for the view's live rows (overseer T4) must keep this.

@history_item n=33 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary; 2. Test Strategy; 11. Test Anti-Patterns) — restored the full members of the screen-parsing excepts on `decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`, `decision/discipline-at-every-layer` and `decision/no-screen-parsing`: the live rig alone reads closed words. Moved the compiled-shapes reading to its own row. These bans bind the view's E2E. A console frame is never read as text, and there is no console exception.

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — added `decision/standing-live-rig`, replacing the retired live-readings row, and added the rig's screen-read excepts to the anti-parsing rows. This is the precedent and the limit: only the test-only rig may read a child's screen, and the console's tests get no such exception.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope; the OS-gate census reads another OS as a fault) — Linux only, CI on `ubuntu-latest`, window features on Hyprland (`decision/linux-only`). `tests/contract_os_gated_tests.rs` faults a gate naming `windows`/`macos`. Any OS gate in the view's tests needs a `GATES` row with its reason (`decision/os-gated-tests-registered`).

@history_item n=36 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (`run --e2e`; the E2E regex widened) — the E2E layer is binaries matching `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, and integration is the complement (`decision/e2e-layer-by-binary-prefix`, `decision/test-layer-filtersets`). The founder ratified this in `2026-10-20-t06-58-31-wrap`. A new console test binary's name decides its layer.

@history_item n=37 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs` holds the literal `BOUNDARIES` table and a census (`decision/bounded-readers`, `product_readers_are_each_bounded_or_named`). If the view adds any external read, for example a newest-line read of `events.ndjson` outside the board pass, it must be bounded in-statement or named with a witness.

@history_item n=38 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration; the exit-2 paths that write no document) — `tests/contract_error_bodies.rs` holds `NOT_ERROR_BODIES`, the guard of `decision/cli-json-one-document`, where the console's fixed stderr pairs are named. A new console refusal or stderr line must join that enumeration.

@history_item n=39 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table is built) — `tests/cross_exit_causes.rs` asserts each cause's exit, document and last `hint:` line, with self-checks (`no_two_codes_share_a_hint`). The console's exit-1 causes joined it at the scaffold. The view adds none, and any it adds must join this table.

@history_item n=40 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through `with_scoped_default`) — every scoped log capture in root-bin unit tests goes through `crate::test_support::with_scoped_default` (`decision/scoped-capture-pinned-dispatch`). A unit case for the console's skip-count WARN, written only when the count changes (F7), must capture through this helper.

@history_item n=41 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola; the home-widening helpers) — a test-made home goes through `viola_state::fs::create_private_dir`, never `create_dir_all` (`decision/test-homes-made-private`). Widened homes use `tests/support/home.rs` `widen`/`grant_read`/`unwiden` on the real file (`decision/real-filesystem-faults`). Overseer T4's planted instance dirs for the size cases must follow this.

@history_item n=42 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — a booted-wrapper test never hand-writes `snapshot.json`/`stamps.json`. Only a reader's own unit cases may plant a malformed file, by altering the writer's bytes (`decision/state-files-never-hand-written`, `decision/product-state-from-real-runs`). Overseer T4's planted instance dirs for the size cases sit at this boundary and must be written through the writer.

@history_item n=43 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another (ETXTBSY)) — a test planting more than one fake-agent copy plants every copy before any thread starts a child (`decision/fake-agent-copies-planted-first`). A multi-instance fleet case for the view, which needs a `claude` bin and wrappers, must keep this order.

@history_item n=44 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named) — the shared helpers are `tests/support/{home,outer_pty,fake,events,cli,ndjson}.rs` (later also `frozen.rs`, `start.rs`). No root test file outside `tests/support/` defines its own copy. Console tests reuse these for the outer PTY, events and spawning.

@history_item n=45 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added `decision/start-lines-read-from-start-offset`: a reused instance name's log is read from that start's own offset. `contract/event-log/start-wheel-record` now allows a human-input wheel line before session-start. This is a trap for the view's `LAST` cell, the newest line's age: tests must not assume a fixed record count per start.

@history_item n=46 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — the Cookie clause left `decision/logs-carry-no-secrets`, `decision/no-secrets-in-logs` and `decision/secrets-never-in-logs`, and the cookie scan class was retired. The secret scan and canary still cover `console.ndjson`. The console's new frame-count fields must hold no content.

@history_item n=47 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — declared the internal `schema-check` and `secret-scan` subcommands, the bodies behind G4 schema conformance (`decision/diagnostics-schema-conformance`, `decision/diagnostics-schema-validation`, `decision/jsonschema-validator`). `console-frame` gaining `shown`/`hidden` must land in `schemas/diag-line.v1.json`, or every console test home fails G4.

@history_item n=48 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (the coverage throwaway crate) — keeps the rule that `env_clear()` re-adds `LLVM_PROFILE_FILE` (`decision/env-clear-keeps-llvm-profile-file`). A console test that clears its child's environment for a test-scoped PATH must re-add it.

@history_item n=49 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — Scenario E4 reads N/A: viola has no web or graphical frontend. The ui process and the web racks are gone. The fleet console is a terminal surface tested under the outer PTY, and no browser suite is added (`absence/browser-suite`).
