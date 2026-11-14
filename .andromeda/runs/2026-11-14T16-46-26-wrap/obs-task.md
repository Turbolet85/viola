@task_prose part=session
The session removed the usage budget gate whole, on the founder's ruling: no rate-limits reading from the status line, no budget.json, no send gate, no budget-paused refusal, no budget fields on the board, the list payload or the log lines, no Path 6 suite.
Retired with it: the budget-gate event kind and its data, the snapshot's budget_paused and budget_override_until, release --budget and the release budget param, the run.budget_eval span and the budget span attributes, the statusline-rate-limits ledger row and its fixtures, the five_hour and seven_day config thresholds (defaults 90 and 85).
Exit 11 is retired, never renumbered or reused; an older wrapper's budget-paused now reads unknown (exit 14) through the unknown fallback.
A channel release carrying any budget key is now refused -32602 invalid-params (data null, no wheel moved, no record); a string from is still refused release-from-driver first; a plain release answers with the wheel holder alone.
A start now writes two records (wheel cause start, then the hook's session-start); relied ledger rows went 24 to 23 (verify prints [NN/23]); CONFIG_KEYS went 6 to 4 (v, diagnostics_level, claude_env_keep, dialog_hold_s); EventKind went 16 to 15 kinds.
The diag-line schema dropped budget_written (hook-decision) and window, used_percentage, resets_at, paused (send lines); every own-format v stays 1 by the founder's word, an exception to the version-bump rule. What stays: the driver release guard, the status-line pass-through, config.json with dialog_hold_s, the readers of old disk state.
@task_file n=1 path=src/run/budget.rs
what: the budget governor in viola run: thresholds, the gate decision, the turn-end re-evaluation and the lift (deleted)
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget rung, the budget-paused refusal of a send (deleted)
@task_file n=3 path=crates/viola-state/src/budget.rs
what: the budget.json writer and readers in the state crate (deleted)
@task_file n=4 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the proptest regressions of the status-line rate_limits parse (deleted)
@task_file n=5 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget e2e suite (deleted)
@task_file n=6 path=tests/cli_budget.rs
what: the Path 6 budget CLI integration tests (deleted)
@task_file n=7 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: the recorded claude status-line fixture carrying rate_limits (deleted)
@task_file n=8 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: the recorded claude status-line fixture carrying rate_limits (deleted)
@task_file n=9 path=fixtures/viola-0.1.0/events.ndjson
what: a 0.1.0-era events.ndjson sample holding budget-gate lines, read by the replay compatibility cases (new)
@task_file n=10 path=fixtures/viola-0.1.0/retired-words.json
what: the list of retired budget words the compatibility and retired-row cases read (new)
@task_file n=11 path=fixtures/viola-0.1.0/snapshot.json
what: a 0.1.0-era snapshot.json holding the budget fields, read by the snapshot compatibility case (new)
@task_file n=12 path=crates/viola-core/src/lib.rs
what: viola-core's protocol types: the event kind enum, the refusal reasons, the closed vocabulary
@task_file n=13 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows and the status-line fixture variants
@task_file n=14 path=crates/viola-agent-claude/src/statusline.rs
what: the claude status-line payload parse (its rate_limits reading removed)
@task_file n=15 path=crates/viola-agent-claude/Cargo.toml
what: the agent-claude crate's dependencies (chrono removed)
@task_file n=16 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp cases, now with a stamp holding the retired row read on the relied rows alone
@task_file n=17 path=crates/viola-state/src/snapshot.rs
what: the snapshot.json writer and reader in the state crate
@task_file n=18 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the events log (now recovers links, agent_session_id and wheel alone)
@task_file n=19 path=crates/viola-state/src/board.rs
what: the state crate's board items read by viola list
@task_file n=20 path=crates/viola-state/tests/state_replay.rs
what: the replay tests, now with a budget-era snapshot and log read whole
@task_file n=21 path=src/run/wheel.rs
what: the wheel and the channel release method, its driver guard and its params
@task_file n=22 path=src/run/send.rs
what: the wrapper-side send path and its refusal order
@task_file n=23 path=src/cmd/run.rs
what: viola run's start sequence and the channel method dispatch (shared by viola revive)
@task_file n=24 path=src/cmd/release.rs
what: the viola release CLI verb and its arguments
@task_file n=25 path=src/cmd/send.rs
what: the viola send CLI verb and its exit code map of refusals
@task_file n=26 path=src/cmd/list.rs
what: viola list: the list document and the board
@task_file n=27 path=src/human.rs
what: the human-facing board, BAY line and hint writers
@task_file n=28 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm: instance check, capped stdin, snapshot read, the pass-through and its hook lines
@task_file n=29 path=src/cmd/verify.rs
what: viola verify: its step lines and summary, and the fixture recording
@task_file n=30 path=src/obs.rs
what: the obs module: the config keys, the diagnostics lines and the obs_event macro
@task_file n=31 path=src/bin/viola-fake-agent.rs
what: the fake agent that replays recorded claude behaviour in CI
@task_file n=32 path=schemas/config.v1.json
what: the config.json schema
@task_file n=33 path=schemas/diag-line.v1.json
what: the diag-line v1 schema of every diagnostics log line
@task_file n=34 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the recorded claude status-line fixtures
@task_file n=35 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot and its readiness on the start records
@task_file n=36 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run: the ledger row count and the live verify literals
@task_file n=37 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract walk (now over the viola-0.1.0 fixtures too)
@task_file n=38 path=.config/nextest.toml
what: the nextest profiles and the ci kill filter
@task_file n=39 path=Cargo.toml
what: the root manifest and its test tables
@task_item n=1 part=touches
decision/budget-metadata-loggable: retired, no budget metadata remains to log.
@task_item n=2 part=touches
contract/budget-log-line/window and contract/budget-path-log-events/event-set: retired with the gate; send-refused lines carry no window, used_percentage, resets_at or paused.
@task_item n=3 part=touches
decision/unreadable-budget-reads-as-none: retired, there is no budget file to read.
@task_item n=4 part=touches
contract/budget-span-attributes rows (run-budget-eval, state-budget-write, statusline-shell-out) and contract/budget-spans/span-chain: the run.budget_eval span, state.budget_write and the budget attributes are gone; retire, or set the statusline.shell_out row to what stays.
@task_item n=5 part=touches
contract/events-log/budget-gate: the budget-gate event kind and its paused, window, override_until data are removed; event kinds 16 to 15.
@task_item n=6 part=touches
contract/release-budget/budget-gate: retired; a release carrying any budget key (true, false, a string, null) is now -32602 invalid-params, no wheel moved, no record.
@task_item n=7 part=touches
contract/release-refusal/invalid-params, contract/channel-response/invalid-params-causes and contract/mcp-invalid-params/detail: a budget key on release is a new invalid-params cause, logged as channel-response detail invalid-params; a string from is still refused release-from-driver first.
@task_item n=8 part=touches
contract/send-refused/budget-paused and contract/send-refused-detail/refusal-reason-words: budget-paused leaves the RefusalReason vocabulary and the send refusal order.
@task_item n=9 part=touches
contract/send-path-log-events/event-order, contract/send-log-line rows, contract/send-log-fields rows and contract/diag-line-fields/send-events: the budget rung leaves the order (control-character, empty-text, human-typing, turn-running, the readiness gate, then human-typing and turn-running again, then no-prompt-submitted) and the send lines lose their budget fields.
@task_item n=10 part=touches
contract/refusal-reason/unknown-fallback: an older wrapper's budget-paused now reads unknown (exit 14) through the fallback.
@task_item n=11 part=touches
Any row naming exit 11 or budget-paused in an exit code table: exit 11 is retired, never renumbered or reused; human-typing 10, unverified-cli 12, not-delivered 13, unknown 14.
@task_item n=12 part=touches
contract/config-json/five-hour and contract/config-json/seven-day: retired; contract/config-json/known-keys is now v, diagnostics_level, claude_env_keep, dialog_hold_s (6 to 4).
@task_item n=13 part=touches
contract/diagnostics-events/config-parse-rejected and contract/parse-rejected-detail/unknown-keys: a config.json still carrying a threshold key logs one parse-rejected config-json unknown-keys at its process's start and nothing refuses.
@task_item n=14 part=touches
contract/run-start-events/order and decision/product-order-from-events-log: a start writes two records, wheel cause start then the hook's session-start (was three, with budget-gate between).
@task_item n=15 part=touches
contract/statusline-log-fields/hook-decision, contract/diag-line-fields/hook-events and contract/hook-decision-detail/statusline-codes: hook-decision loses budget_written; the statusline arm no longer reads rate_limits nor writes a budget file; malformed-json and its parse-rejected stay.
@task_item n=16 part=touches
decision/diag-schema-additive-only, decision/log-schema-additive-only and decision/log-fields-additive-only: the diag-line schema dropped budget_written, window, used_percentage, resets_at and paused with v kept at 1, an exception to additive-only for a retired capability.
@task_item n=17 part=touches
decision/unemittable-schema-words-removed: applied here, the budget words no build can emit left the diag-line schema.
@task_item n=18 part=touches
contract/viola-mcp-tools/send and contract/viola-mcp-tools/list: send no longer refuses budget-paused; the list payload is items and skipped, no budget member.
@task_item n=19 part=touches
contract/agent-run-status rows and contract/list-json rows: the status list document carries no budget member.
@task_item n=20 part=touches
contract/verify-stdout/step-line and contract/verify-stdout/summary-line: verify has 23 steps ([NN/23], 23 pass) where a row holds the count 24; the statusline-rate-limits row is gone.
@task_item n=21 part=touches
contract/replayed-snapshot/live-fields and decision/no-raw-snapshot-reads: the snapshot holds no budget_paused or budget_override_until; replay recovers links, agent_session_id and wheel alone; a budget-era snapshot or log still reads whole, its budget fields and gate lines skipped.
@task_item n=22 part=touches
decision/direct-chrono-dependency: chrono left viola-agent-claude (its one user was the rate_limits parse); check the row still holds for the crates that keep it.
@task_item n=23 part=touches
contract/wheel-path-log-events/pause-and-release, contract/pause-release/answer-after-wheel-record and decision/release-from-driver-refused: release params are from alone, the reply is the wheel holder alone; the driver guard stands.
@task_item n=24 part=touches
contract/event-line/shape, contract/event-stream/line and decision/claude-fixtures-scrub-required: every own-format v stays 1; the new viola-0.1.0 fixtures are walked by the hygiene contract.
@task_fact n=1 slug=1-obs-scope-summary
fact: viola has no usage budget gate: it reads no rate limits, keeps no budget file and never pauses a send on usage (retired whole, v2-04).
@task_fact n=2 slug=log-coverage
fact: removing a retired capability drops its fields and words from the diag-line schema while v stays 1, an exception to additive-only.
@task_fact n=3 slug=6-log-coverage
fact: exit 11 is retired (was budget-paused), never renumbered or reused; an older wrapper's budget-paused reads unknown, exit 14.
@task_fact n=4 slug=span-trace-coverage
fact: a channel release carrying any budget key is refused -32602 invalid-params with data null, no wheel moved and no record, after the release-from-driver check.
@task_fact n=5 slug=otel-sdk-init
fact: a config.json still carrying five_hour or seven_day logs one parse-rejected config-json unknown-keys at its process's start and nothing refuses.
@task_item n=1 part=untouched
The hook latency, session-end, spine and performance budgets, the hyperfine max-sample gates and the error budget of section 10: another kind of budget, they stand.
@task_item n=2 part=untouched
The heartbeat staleness threshold and liveness rows: another threshold, unchanged.
@task_item n=3 part=untouched
Panic capture, panic lines, catch sites and the zero-panics gate: unchanged.
@task_item n=4 part=untouched
PII scrubbing, the secret scan, CI artifact uploads and retention: unchanged.
@task_item n=5 part=untouched
OTel, exporter, error reporter and network egress absences and decisions: unchanged.
@task_item n=6 part=untouched
Dialog raise and answer log rows, link and unlink rows, wait and last rows: unchanged.
@task_item n=7 part=untouched
Socket-directory, strict-modes and server-verify refusal codes and exit 21 cause words: unchanged.
@task_item n=8 part=untouched
Trace-context propagation through corr and conn, and service identity rows: unchanged.
