@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (Path 3 as landed) — Path 3's landed half is `tests/cli_wait_last.rs` (`path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`, `last_survives_a_wrapper_restart`) over `fixtures/fake-scripts/path3.json`, plus `tests/chaos_wait_vanish.rs`. These are the wait witnesses this chunk extends with background-task and notification cases; the gated `path3.json` script is the fixture shape to copy.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (relayed dialog fixtures, Paths 3 and 4 as landed) — the `question` / `plan` end-to-end wake witness landed in Path 3; the fake agent's matchers now evaluate `tool_name`. This is where a dialog kind's wake was first proven over the fake agent, which matters if the chunk's background-task turn end raises a `question`.

@history_item n=3 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** (the dialog wake witness complete) — the `permission` wake is end to end in `path4_permission_…`, and the PostToolUse `activity` line wakes no `wait`. This is the standing witness for the log-only half (`contract/wait/wake-kinds`, `contract/viola-wait/non-waking-records`). A separate gated script kept the exact counts uncoupled, a precedent for a new background-task script.

@history_item n=4 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (sixteen kinds, six wake kinds) — the wake set is six by rule `EventKind::WAIT_WAKE` (`dialog-expired` added), `ALL` is 16, `state_events.rs` round-trips by the length of `ALL`, and the channel `wait` handler's unit cases name the set. This is the register the chunk documents and pins (`contract/channel-wait/wake-kinds`, `contract/wait/wake-kinds`). A new or reclassified kind moves both counts by their rules, and `decision/closed-enums-need-decisions-log` applies.

@history_item n=5 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (the `option_details` and `background_tasks` cases named, with the switch's live readings) — names `turn_ended_background_tasks_counts_the_array` / `…_is_null_when_absent_or_not_an_array`, `hook_a_recorded_stop_carries_its_background_task_count` (the 2.1.287 fixture's empty array reads 0; non-zero and null are unit-tier only), and the live reading on 2.1.296: `wait --json` woke on `turn-ended` with `background_tasks` 0. This is the direct precedent for the chunk's driver-side instance. A non-zero `background_tasks` has no fixture or end-to-end witness yet, and live readings stand as a local measurement, not a suite.

@history_item n=6 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Integration: `contract/event-data/turn-ended`) — the turn-ended data keys moved from 2 to 3 with `cause`, present only on an API-error end. A second producer of `turn-ended` (StopFailure) now wakes `wait`, and the chunk's wake documentation must cover both ends. This is PROVISIONAL, and the founder's own word is owed.

@history_item n=7 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Test Data: `contract/fake-script/format`; `decision/hook-payloads-recorded-by-verify` except; + `decision/send-cases-copy-recorded-fixtures`) — the fake-script schema covers ten registered hook events (StopFailure added). StopFailure runs on a synthetic payload with no recorded fixture or ledger row. A purely synthetic fixtures root fails any send after the first turn, so send-after-turn cases must copy the recorded fixtures. That trap applies to any new wait script that sends twice.

@history_item n=8 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (E2E: `contract/viola-send/turn-running`) — after `turn-ended`, an API-error end included, a send is accepted with no pause or release. A turn ended while a background task still holds the session is the same question from the send side, and the chunk's wake semantics must stay consistent with it.

@history_item n=9 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a running turn refuses `send`, so every test that sends twice must end the first turn with a scripted Stop; the fake agent's interactive submit fires none. A trap for new wait scripts: the turn end must be a scripted step, gated through the control file.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam, stop waits for the endpoint) — root `stamped_home` runs `viola verify` against the fake agent and is stamped only on exit 0 with the `stamped … 0 fail` last line; `StampedHome::unstamped` serves unverified paths. This is the fixture whose unbounded verify the carried CI red blames (`tests/support/verify.rs`, `within: None`), and the seam option (a) would change.

@history_item n=11 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (Bootstrap phases) — `[profile.ci]` gained the verify-driven 20 s (10 s × 2) override, and a verify a test drives has no test-side bound. The 7 s bound gave way because four runs make a designed floor, not a regression. This is the origin of the class's kill and of `decision/verify-driven-tests-20s-kill`, the line the carried red hit.

@history_item n=12 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (Bootstrap key) — the 20 s kill stands on a measured reading: the longest verify-driven test read 9.98 s on the ubuntu leg in ci#37534758441. This is the sizing basis option (b) would re-measure under load.

@history_item n=13 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (kills follow the 8.5 s gate) — standing rule: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways (TIMEOUT under the moved bound, PASS with none). This binds option (b): sizing the `ci` kill to the fixture's loaded time needs that pair, and a load-driven floor is not obviously "by design".

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — override order is first-match-wins, pinned by a test; the class's floor was measured and its kill moved with a planted-hang control. This is a precedent for any reshaping of `.config/nextest.toml`'s `ci` overrides.

@history_item n=15 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the `verify_window_` kill at 60 s) — both floors were by design and measured across runs. Each kill moved with its planted-hang pair, never by raising the default, and the replay contracts got faster instead. This is the precedent for option (a), removing slow work, over option (b).

@history_item n=16 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (Path 1 as one flow) — `path1_run_start_one_home_in_order` runs steps 1–6 over one stamped home in `tests/path1_run_start.rs`, on the verify-driven 20 s kill line, with wall time 4.69 s on the dev host. This is the exact test the carried red names. The companion entry (`run --e2e`, the E2E regex) put `path1_run_start` on the kill line.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases; the CI matrix [ubuntu-latest]) — the `ci` kill line lists twenty-six verify-driven binaries; the rule is the `binary(…)` names on the filter line. This is the register a sizing or a moved binary changes by its rule; earlier counts moved with each join (e.g. 2026-11-04-linux-and-macos-parity added `run_cli` to reach 27 before this cut).

@history_item n=18 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Harness: `decision/root-wait-bound`, `decision/within-7s-wait-bound`, `decision/mutants-kill-below-cargo-mutants-timeout`) — `WITHIN` moved from 7 s to 12 s and the mutants default kill from 10 s to 15 s, under cargo-mutants' 20 s floor. This was a stated departure from the 2026-10-20 line ("fix a slow case by removing its slow work, never raise the default"), taken because the stamped-home class is now the default population. It is the nearest precedent and constraint for the carried kill-line choice, and the chunk whose CI slow mode the carry dates.

@history_item n=19 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (+ `decision/record-home-names-no-home`) — record homes given to `verify --record` are never named home. This bears on any reshaping of the stamped-home or verify test support.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded by `WITHIN`, because nextest's default profile has no kill line (an unbounded witness hung for 27 minutes). This is a trap for the chunk's `gate.py control` runs on each new guard test (`decision/bounded-remove-the-guard-witness`).

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola: `create_private_dir`) — a home a test makes must come from `create_private_dir`, since a plain `create_dir_all` home is refused by the read rule (`decision/test-homes-made-private`). This applies if option (a) copies or shares a stamped home.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a bare `tempfile::tempdir()` is 0755 and the socket-directory check refuses it, which nine `handlers` cases first measured red (`decision/test-endpoint-dirs-made-private`). This is a trap for any MCP `wait` handler case the chunk adds.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a wrapper lost under an open call) — a wrapper killed under a parked `wait` is `endpoint-lost`, `during:"call"`, with its own hint, witnessed by `tests/chaos_wait_vanish.rs` through `support::start::kill_wrapper`. This holds `contract/cli-wait/endpoint-lost`, `contract/viola-wait/endpoint-lost` and the endpoint-lost cause rows the chunk's wait cases sit beside.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `instance-unreachable` body names its cause; MCP's lost-cause witness) — the MCP body carries `detail:"<cause>"`, equal to the CLI exit-21 document less `v`, and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` is MCP's lost cause on Path 3. These are the parity rows (`contract/mcp-cli-parity/exit-21`, `contract/mcp-unreachable-detail/*`) a new MCP `wait` case must respect.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's CLI document; the `cli` catch site under `--json`; a stale holder reads `stale-heartbeat`) — exit 21 prints `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, Path 3 step 7 included. `json_asked` reads `--json` only before `--` (`contract/cli-arguments/json-flag`). A frame verb against a stale instance reads `instance-dead`.

@history_item n=26 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed; harness keys and the time mechanism) — MCP `wait` is held by `mcp_tools_path2_path3_send_wait_last_match_the_cli` against the CLI's `--json` less `v` and by the clamp case `handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`. `viola-mcp` owns no timer: its bound is the pure `wait_timeout`, `min(given, 50 000)`, a unit case, and no `start_paused` test exists. This constrains how the chunk proves MCP `wait`'s wake (`contract/mcp-wait/timeout-ms`, `decision/mcp-handlers-use-tokio-client`).

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the reply screen and the fault screen; the exit-2 paths that write no document) — `of_reply` / `reply_of` read a non-code-word refusal as `unknown` and a non-code-word string detail as null, and a peer fault goes through `sanitised_fault` (`contract/reply-screen/*`, `contract/mcp-reply/screened-words`). The exit-2 paths of `answer` / `send` write a fixed stderr line and no document (the `decision/cli-json-one-document` except).

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs and the MCP frame tools under the instance check) — `wait` runs `check_instance` through `refuse_untrusted` before any role file or log. MCP `wait` runs it before the liveness pre-check (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`), with no frame and no line. These are the order rows the chunk's `wait` changes must keep (`decision/frame-verbs-check-instance-first`, `decision/mcp-frame-tools-check-instance-first`).

@history_item n=29 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (the board's readings landed) — `wait` / `last` human output keeps `\n` and `\t` while `list` rows escape every control character. This is the escaping rule (`decision/human-output-control-escaping`, `decision/human-output-escapes-control`) for any new `src/human/wait.rs` line.

@history_item n=30 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the torn append as landed: the reader counts three names) — the event reader counts torn lines, unknown kinds and unknown fields (`three_counts`) and never panics, and the appender heals a torn tail. A new kind or key the chunk adds is read through this reader, and older builds count it as unknown rather than failing (`decision/defensive-log-reader`, `decision/unknown-events-counted-skipped`).

@history_item n=31 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (pre-push VM release and Windows tests, endpoint readiness, channel corr rules, E2 fd premise fix) — `tests/support/ndjson.rs` is the one complete-lines reader for live-appended files (`decision/complete-lines-reader`). A test that keys on `events.ndjson` offsets reads through it.

@history_item n=32 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named) — `tests/support/events.rs` holds the one `events.ndjson` reader and its wait, `cli.rs` the one started-child guard, and no root test file defines its own copy. New wait cases must reuse these helpers (`decision/waits-key-on-events`, `decision/no-sleep-synchronization`).

@history_item n=33 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (`contract/event-log/start-wheel-record`; + `decision/start-lines-read-from-start-offset`) — trust keys can add a `wheel{human, human-input}` record before `session-start`. A reader of a reused instance name's log must read a start's lines from that start's own offset. Wheel lines never wake `wait`, and a wait test armed after a start must use the start's offset or cursor.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary: `contract/run-start-events/order`) — the start records are now `wheel` with cause start, then `session-start`; the `budget-gate` kind is gone, and with it a former kind that never woke `wait`. This is the event-kind register (`ALL`) the chunk's wake and log-only lists are stated against.

@history_item n=35 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (the fake-agent drift contract, byte for byte) — `contract_fake_agent_drift` compares each hook's `stdin_hex` byte for byte with the recorded fixtures (`decision/fake-agent-matches-recordings`, `decision/fixture-drift-byte-compare`). A Stop variant with a non-empty `background_tasks` needs a recorded payload, not a hand-built one, or it falls outside the contract.

@history_item n=36 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: the fake agent's agents files, the test-scoped PATH) — the fake agent answers from files beside its exe with no env var, and every test-started `list` / `revive` runs under a test-scoped PATH (`decision/fake-agent-on-test-path`, `decision/tests-never-read-host-sessions`). This constrains any new fake-agent mode for a background-task or notification turn.

@history_item n=37 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: `contract/fake-agent-cli/argv-only-options`, `decision/fake-agent-argv-only`) — the fake agent's options come from argv only, with the hyprctl-by-file-name exception. A new fake-agent option for this chunk follows the argv-only rule.

@history_item n=38 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** (8. Mocking: `decision/fakes-for-external-services` except) — `viola-hyprland`'s own unit tests use a planted shell stub because cargo-mutants runs only the owning package's tests. This explains the picked row's exception.

@history_item n=39 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named) — on Unix every command the fake agent spawns runs in its own process group, so a PTY hangup no longer cuts a hook's exit-time profile (`decision/fake-agent-own-process-group`). Fake-agent script changes must keep this property.

@history_item n=40 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (mutation moves to the epoch boundary) — no per-chunk, pre-push or CI mutation gate. `run --mutants` runs on demand and at the code audit. This explains the mutation rows and the scope's "no mutation run"; each new guard instead carries a one-off remove-the-guard control.

@history_item n=41 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the client seam's always-admit `peer_admitted` survivor stands as the version's known limit, read at each epoch-boundary audit. This is the source of the picked `decision/mutation-run-verdict` exception.

@history_item n=42 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `contract_ledger_probes` and `cli_verify` list every relied row once and end `24 pass  0 fail` (`decision/ledger-probe-contract-suite`, `contract/verify-output/last-line`). This applies if the chunk touches what verify stamps (StopFailure has no row, by the api-error chunk).

@history_item n=43 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables; the real CLI has two local routes) — a route entry's live readings of a real session are that chunk's own local measurement: not a test layer, not harness surface, never in CI (`decision/live-readings-not-a-test-layer`). This governs how any granted live `claude` start for the wake condition is recorded.

@history_item n=44 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — under `CI`, `--local-live` refuses `live-in-ci` (exit 2) before any build (`contract/run-reason/live-in-ci`, `decision/live-verify-never-in-ci`). This is a constraint on any live reading this chunk asks for.

@history_item n=45 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — trap: an oracle moved by a shared body can hide a test that never reached its subject; a bare temp-dir home never reached the liveness check. Each new wait case must prove it reaches the wake path it names.
