@task_prose part=session
The session removed the budget gate whole on the founder's ruling: no rate_limits reading, no budget.json, no send budget rung, no budget-paused refusal, no budget-gate event, no board or list budget fields, no release --budget, and the Path 6 suite retired.
Exit 11 is retired (was budget-paused) and never renumbered or reused; nothing maps to 11; an older wrapper's budget-paused reads unknown (14).
Channel release now refuses ANY budget key (true, false, string, null) with -32602 invalid-params, data null, no wheel moved, no record; the string-from release-from-driver guard stays first; a plain release answers ok.wheel alone.
A start now writes two records, wheel{cause:start} then the hook's session-start; list --json ok is {items, skipped}; the BAY line is skipped counts alone; WHEEL width stays 23; relied ledger rows 24 to 23 and viola verify 23 steps; config keys drop five_hour and seven_day (a stale key logs parse-rejected unknown-keys, nothing refuses).
Every own-format v stays 1 (founder's word), compat measured by new cases: a budget-era snapshot and log read whole skipping the budget fields; the retired words and 0.1.0 shapes live in a new viola-0.1.0 fixture directory walked by the fixture hygiene check.
@task_file n=1 path=src/run/budget.rs
what: the budget governor and gate (thresholds, decide, the lift, the turn-end re-evaluation) — deleted
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget rung (budget-paused refusal context) — deleted
@task_file n=3 path=crates/viola-state/src/budget.rs
what: the budget.json single writer and its readers — deleted
@task_file n=4 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the statusline rate_limits parser's persisted proptest seeds — deleted
@task_file n=5 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget E2E case over the harness — deleted
@task_file n=6 path=tests/cli_budget.rs
what: the Path 6 budget CLI integration suite (budget pause, override, release --budget, snapshot replay guard) — deleted
@task_file n=7 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: the recorded status-line rate-limits fixture for CLI 2.1.287 (statusline-rate-limits ledger row) — deleted
@task_file n=8 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: the recorded status-line rate-limits fixture for CLI 2.1.302 — deleted
@task_file n=9 path=fixtures/viola-0.1.0/events.ndjson
what: new committed fixture, a 0.1.0 budget-era event log replayed by the compat cases
@task_file n=10 path=fixtures/viola-0.1.0/retired-words.json
what: new committed fixture, the retired words (snapshot keys, event kind, ledger row, refusal word) read by the cases
@task_file n=11 path=fixtures/viola-0.1.0/snapshot.json
what: new committed fixture, a 0.1.0 budget-era instance snapshot
@task_file n=12 path=crates/viola-core/src/lib.rs
what: the closed event kinds and refusal reasons (EventKind, RefusalReason) — budget-gate and budget-paused removed
@task_file n=13 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot.json writer and reader — budget_paused and budget_override_until gone
@task_file n=14 path=crates/viola-state/src/replay.rs
what: snapshot replay from the event log — the replayed field set now links, agent_session_id, wheel
@task_file n=15 path=crates/viola-state/src/board.rs
what: the list board items read through the replay — budget_paused gone
@task_file n=16 path=crates/viola-agent-claude/src/statusline.rs
what: the statusline payload parser — the rate_limits reading removed
@task_file n=17 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger rows and relied rows behind verify and the version gate — statusline-rate-limits row removed
@task_file n=18 path=src/run/wheel.rs
what: the wheel and the channel release method (release-from-driver guard, budget key refusal)
@task_file n=19 path=src/run/send.rs
what: the driver send path and its refusal order
@task_file n=20 path=src/cmd/list.rs
what: viola list and its --json document
@task_file n=21 path=src/cmd/release.rs
what: the viola release verb and its arguments — --budget flag removed
@task_file n=22 path=src/cmd/send.rs
what: the CLI send exit-code map (exit 10-14)
@task_file n=23 path=src/cmd/run.rs
what: viola run start (start records, start state) and the channel method dispatch
@task_file n=24 path=src/cmd/hook/statusline.rs
what: viola hook statusline — passthrough and the payload check; no budget file write
@task_file n=25 path=src/cmd/verify.rs
what: viola verify — its step lines and fixture recording
@task_file n=26 path=src/obs.rs
what: the config keys and the structured process logs
@task_file n=27 path=src/human/board/bay.rs
what: the human board's BAY line
@task_file n=28 path=src/human/board/cells.rs
what: the human board's WHEEL cell and its width
@task_file n=29 path=src/human/hints.rs
what: the human hints per refusal and exit cause — budget-paused hint removed
@task_file n=30 path=src/bin/viola-fake-agent.rs
what: the fake agent serving the claude boundary — rate-limits status-line replay removed
@task_file n=31 path=schemas/config.v1.json
what: the config.json schema — threshold keys dropped
@task_file n=32 path=schemas/diag-line.v1.json
what: the diagnostics log line schema — budget_written and the send budget fields dropped
@task_file n=33 path=schemas/claude-statusline-fixture.v1.json
what: the recorded statusline fixture schema — rate_limits pin dropped
@task_file n=34 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot readiness probe (start records it waits for)
@task_file n=35 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command and the local-live suite's expected verify lines
@task_file n=36 path=crates/viola-e2e/src/harness/status.rs
what: the agent-run status document and its polled fields
@task_file n=37 path=.config/nextest.toml
what: the nextest profiles and the CI 20 s kill line for stamped-home tests
@task_file n=38 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene run-time walk — now covering the viola-0.1.0 fixtures
@task_file n=39 path=crates/viola-mcp/src/result.rs
what: the MCP tool result and refusal body — budget-paused refusal removed
@task_file n=40 path=crates/viola-agent-claude/Cargo.toml
what: the agent-claude crate dependencies — chrono removed
@task_item n=1 part=touches
Every budget gate and governor decision retires: decision/budget-pause-thresholds, decision/budget-governor-thresholds, decision/budget-pause-threshold, decision/five-hour-pause-threshold, decision/budget-decision-takes-the-instant, decision/budget-gate-covers-send-only, decision/unknown-reset-does-not-pause.
@task_item n=2 part=touches
Every budget override decision retires: decision/budget-override-ends-with-wrapper, decision/budget-override-per-instance, decision/override-expiry-tested-at-unit-level.
@task_item n=3 part=touches
The budget.json file and its writer retire: decision/budget-json-from-statusline, decision/budget-file-from-statusline-hook, decision/budget-seeded-through-statusline-hook, contract/budget-json/rate-limits, contract/budget-json/write-condition, contract/viola-hook-statusline/budget-json, contract/statusline-payload/resets-at.
@task_item n=4 part=touches
The budget-gate event kind retires: contract/event-log/budget-gate, contract/budget-gate-record/paused-false, contract/budget-gate-record/paused-true; the closed event kind set loses one kind.
@task_item n=5 part=touches
Exit 11 is retired and never reused: contract/viola-send/exit-11, contract/cli-send-json/budget-paused, contract/budget-paused-refusal/send-exit-11 retire; contract/cli-exit-codes/10-14 must state 11 as retired (was budget-paused), never reused, and an older wrapper's budget-paused reads unknown 14.
@task_item n=6 part=touches
The budget-paused refusal word leaves the send refusal order and every surface: decision/send-refusal-order, contract/mcp-tool-result/budget-paused, contract/list-human/wheel-budget-paused, contract/list-json/budget-paused, contract/code-word/format and contract/mcp-reply/screened-words where they list the refusal vocabulary.
@task_item n=7 part=touches
release --budget retires and a budget key is now refused: contract/viola-release/budget, contract/cli-release/budget, contract/release-budget-json/document retire; contract/viola-release/document and contract/cli-release/wheel-driver now answer ok.wheel alone; contract/channel-response-detail/invalid-params and contract/json-rpc-errors/-32602 gain the release budget key.
@task_item n=8 part=touches
The driver release guard stays first: contract/cli-release/release-from-driver, contract/viola-release/from-driver, contract/channel-response-detail/release-from-driver, decision/wheel-returns-only-via-cli-release hold true but any guard naming a deleted Path 6 or cli_budget case must be re-pointed.
@task_item n=9 part=touches
The list --json budget member retires: contract/viola-list-json/budget retires; contract/viola-list-json/items, contract/viola-list-json/mirrors-channel, contract/list-json/skipped-counts and contract/list-human/columns now hold {items, skipped} and a BAY line of skipped counts alone.
@task_item n=10 part=touches
The agent-run status polled fields lose the budget: contract/agent-run-status-polled-fields/budget and contract/agent-run-status-polled-fields/budget-paused retire; contract/status-document/list holds no budget member.
@task_item n=11 part=touches
Start records move 3 to 2: contract/run-start-records/first-three, contract/viola-run-start/event-order, contract/run-start-events/order, contract/event-log/start-wheel-record, contract/boot-readiness/events and decision/start-hands-wheel-to-driver must read wheel{cause:start} then session-start.
@task_item n=12 part=touches
The snapshot replay field set narrows: decision/snapshot-replay-fields and contract/instance-snapshot fields drop budget_paused and budget_override_until; decision/snapshot-replay-from-log's guard named cli_budget and must be re-pointed to a_start_over_an_unreadable_snapshot_replays_the_log and the cli_links case.
@task_item n=13 part=touches
The statusline-rate-limits ledger row retires: relied rows 24 to 23, so decision/version-gate-needs-all-relied-rows, decision/ledger-row-verify-probe, decision/ledger-probe-contract-suite, contract/viola-verify/stamped-line, contract/verify-output/step-line, contract/verify-output/summary-line and contract/local-live-suite/pass where a cell holds 24 or the [NN/24] form.
@task_item n=14 part=touches
viola hook statusline keeps passthrough but reads no rate_limits and writes no file: decision/statusline-passthrough, contract/viola-hook-statusline/passthrough, contract/hook-statusline/passthrough, contract/statusline-hook/stdout, decision/statusline-source-read-only stay; decision/statusline-proptest-whole-seconds retires with the rate_limits property.
@task_item n=15 part=touches
The fake agent no longer replays a rate-limits status line after the first Stop: contract/fake-agent-hooks/status-line-command, contract/fake-agent-receipt/statusline, decision/fake-agent-matches-verify-fixtures and decision/fixtures-per-cli-version where they name the rate-limits fixture.
@task_item n=16 part=touches
Config keys move 6 to 4 (v, diagnostics_level, claude_env_keep, dialog_hold_s): decision/bad-config-value-falls-back where it names the threshold keys; a stale threshold key logs parse-rejected unknown-keys and nothing refuses.
@task_item n=17 part=touches
The diagnostics schema drops budget_written and the send budget fields: decision/diagnostics-schema-conformance, decision/diag-line-schema-requires-corr, contract/log-event-corr/send-refused and contract/log-event-corr/hook-decision where their fields list them.
@task_item n=18 part=touches
The Path 6 critical path retires: decision/e2e-for-critical-paths and decision/every-surface-asserted where they count Path 6, decision/no-switch-disables-a-control and decision/controls-not-disableable where the budget gate is one of the controls listed.
@task_item n=19 part=touches
The CI 20 s kill line drops binary(cli_budget): decision/ci-stamped-home-kill-line and decision/verify-driven-tests-20s-kill where their filter names it.
@task_item n=20 part=touches
The MCP tools/list description still says and the usage budget until a live re-record: contract/mcp-tools-list/tool-set, contract/mcp-server/tools-list, contract/mcp-tools-list/tools-list where they quote that description.
@task_item n=21 part=touches
The fixture hygiene walk gains a viola-0.1.0 directory of committed own-format fixtures: decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/scrubbed-fixtures, decision/product-state-never-hand-written and decision/state-files-never-hand-written, which the hand-held 0.1.0 shapes are an exception to.
@task_item n=22 part=touches
Time decisions lose their budget instance: decision/time-as-argument, decision/injected-clock and decision/utc-now-read-directly where the budget decision or override end is their example; chrono leaves the agent crate.
@task_item n=23 part=touches
The human board WHEEL cell keeps width 23 with no budget-paused form; the hint for budget-paused is gone: decision/hint-names-its-cause, decision/each-exit-cause-own-hint, decision/human-layouts-pinned-by-literal, contract/list-human/columns.
@task_item n=24 part=touches
The pause and wheel rules lose their budget cause: decision/pause-forces-human-wheel, decision/release-without-pause-records-nothing, decision/human-never-blocked where the budget pause is named beside the manual pause.
@task_fact n=1 slug=6-e2e-test-strategy
fact: Exit 11 is retired (was budget-paused) and never renumbered or reused; no refusal maps to 11, and an older wrapper's budget-paused word reads unknown, exit 14.
@task_fact n=2 slug=6-e2e-test-strategy
fact: A channel release carrying any budget key (true, false, a string or null) is -32602 invalid-params with data null, moves no wheel and writes no record; a string from is refused release-from-driver first; viola release --budget is a clap usage error, exit 2.
@task_fact n=3 slug=unit-test-strategy
fact: The send refusal order is control-character, empty-text, human-typing, turn-running, the readiness gate, then human-typing and turn-running again, then no-prompt-submitted; no budget rung.
@task_fact n=4 slug=e2e-test-strategy
fact: A start writes two records, wheel{holder:driver, cause:start} then the hook's session-start; viola revive the same.
@task_fact n=5 slug=e2e-test-strategy
fact: A snapshot or event log written by a 0.1.0 budget-era wrapper is read whole, its budget fields and budget-gate lines skipped; a new snapshot holds no budget key.
@task_fact n=6 slug=test-data-fixtures
fact: The 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases that must spell an old word, and walked by the fixture hygiene check.
@task_fact n=7 slug=1-test-scope-summary
fact: No usage budget gate: viola reads no rate_limits, writes no budget file and refuses no send for usage; the status-line pass-through and the driver release guard stay.
@task_fact n=8 slug=integration-test-strategy
fact: A config.json still carrying five_hour or seven_day logs one parse-rejected{config-json, unknown-keys} at its process's start and nothing refuses.
@task_fact n=9 slug=unit-test-strategy
fact: A stamp holding the retired statusline-rate-limits row verifies on the 23 relied rows alone.
@task_item n=1 part=untouched
Performance, hook-latency, SessionEnd and perf-budget rows (perf gates, hyperfine, spine deadlines) use another budget and stand.
@task_item n=2 part=untouched
Coverage floors, mutation thresholds, the zero-flakiness budget and heartbeat staleness age thresholds are other thresholds and stand.
@task_item n=3 part=untouched
Dialog rows (hook decision bodies, dialog hold, one pending dialog, answer refusal order) are not reached.
@task_item n=4 part=untouched
Link and unlink methods, link records and their survival across restart are not reached beyond start-record counting in tests.
@task_item n=5 part=untouched
Revive refusal codes, start exit-1 causes, strict-modes, socket-directory and server-identity checks are not reached.
@task_item n=6 part=untouched
Harness pre-push, gate breach, secret-scan, schema-check, mutants and cleanup contracts are not reached.
@task_item n=7 part=untouched
CI workflow pins, Actions SHA pins, artifact retention and toolchain pins are not reached.
@task_item n=8 part=untouched
Fuzz workspace and corpus rows are not reached beyond a lock refresh dropping chrono.
