@task_prose part=session
The session removed the usage-budget gate whole, on the founder's ruling: no rate-limits reading of the status line, no budget.json, no five_hour or seven_day thresholds, no send gate, no budget-paused state, refusal, hint or board word, no turn-end re-evaluation.
viola release loses its --budget flag and the channel release loses its budget param; any budget key on the channel release is now refused -32602 invalid-params with no wheel moved, after the release-from-driver guard, which stays; a plain release answers with the wheel holder alone.
Exit 11 is retired (was budget-paused), never renumbered or reused; nothing maps to it; an older wrapper's budget-paused reads unknown, exit 14.
viola list reads no budget: its --json ok payload is items and skipped; the BAY line keeps only the skip counts (BAY  skipped a - b - c), its 5H, 7D, read and gate cells gone; the WHEEL cell has no budget-paused form and stays padded to 23.
A start writes two records (wheel start, then the hook's session-start); viola verify runs 23 steps against 23 relied ledger rows (the statusline-rate-limits row retired); the status-line hook still passes the line through and still refuses a payload that is not one JSON object.
Every own-format v stays 1 (snapshot, channel params, event line, list document); readers of old budget-era snapshots and logs skip the budget fields; the old shapes and retired words live as committed fixtures.
@task_file n=1 path=src/run/budget.rs
what: the budget gate and governor: thresholds, the pause decision, the lift (deleted)
@task_file n=2 path=src/run/send/budget.rs
what: the send path's budget-paused rung (deleted)
@task_file n=3 path=crates/viola-state/src/budget.rs
what: the budget.json writer and readers (deleted)
@task_file n=4 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget-pause end-to-end suite (deleted)
@task_file n=5 path=tests/cli_budget.rs
what: the CLI budget gate tests, release --budget and the budget-paused send (deleted)
@task_file n=6 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: a recorded status-line payload carrying rate_limits (deleted)
@task_file n=7 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: a recorded status-line payload carrying rate_limits (deleted)
@task_file n=8 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the regression seeds of the rate-limits reading's property test (deleted)
@task_file n=9 path=fixtures/viola-0.1.0/events.ndjson
what: a budget-era 0.1.0 event log, read by the replay compatibility cases (new)
@task_file n=10 path=fixtures/viola-0.1.0/retired-words.json
what: the list of retired budget words (snapshot keys, event kind, ledger row, refusal) the compatibility cases read (new)
@task_file n=11 path=fixtures/viola-0.1.0/snapshot.json
what: a budget-era 0.1.0 instance snapshot, read by the compatibility cases (new)
@task_file n=12 path=crates/viola-core/src/lib.rs
what: the shared vocabulary: event kinds, refusal reasons (budget-gate kind and budget-paused reason removed)
@task_file n=13 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot (budget_paused and budget_override_until removed)
@task_file n=14 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the event log (recovers links, agent session id, wheel alone)
@task_file n=15 path=crates/viola-state/src/board.rs
what: the board item read for viola list (budget_paused removed)
@task_file n=16 path=crates/viola-agent-claude/src/statusline.rs
what: the status-line payload reading (the rate_limits parse removed)
@task_file n=17 path=crates/viola-agent-claude/src/ledger.rs
what: the CLI verification ledger of relied rows (23; statusline-rate-limits retired)
@task_file n=18 path=src/run/wheel.rs
what: the wheel and the channel release: the release-from-driver guard, the budget key refused as invalid-params
@task_file n=19 path=src/cmd/run.rs
what: the start of viola run and revive: start records, channel dispatch (no budget evaluation)
@task_file n=20 path=src/run/send.rs
what: the send refusal order (budget rung removed)
@task_file n=21 path=src/cmd/list.rs
what: viola list, the board, and its --json document (items and skipped)
@task_file n=22 path=src/human/board/bay.rs
what: the board's BAY line (skip counts only)
@task_file n=23 path=src/human/board/cells.rs
what: the board's cells, the WHEEL cell padded to 23 (no budget-paused form)
@task_file n=24 path=src/human/hints.rs
what: the refusal hints (the budget-paused hint removed)
@task_file n=25 path=src/human.rs
what: the human output lines (the budget lifted and open lines and the override time removed)
@task_file n=26 path=src/cmd/hook/statusline.rs
what: viola hook statusline: the status-line pass-through (no budget write)
@task_file n=27 path=src/cmd/verify.rs
what: viola verify: the step lines and summary (23 steps)
@task_file n=28 path=src/cmd/send.rs
what: viola send and the CLI exit-code map for send refusals (11 unused)
@task_file n=29 path=src/cmd/release.rs
what: viola release, its arguments (the --budget flag removed)
@task_file n=30 path=src/bin/viola-fake-agent.rs
what: the fake agent used by the tests (no rate-limits status-line replay)
@task_file n=31 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness readiness check, the start records it awaits (two)
@task_file n=32 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness's ledger row count and live verify literals (23)
@task_file n=33 path=schemas/config.v1.json
what: the config.json schema (five_hour and seven_day dropped)
@task_file n=34 path=schemas/diag-line.v1.json
what: the diagnostic log line schema (budget fields of hook-decision and send lines dropped)
@task_file n=35 path=schemas/claude-statusline-fixture.v1.json
what: the recorded status-line fixture schema (rate_limits pin dropped)
@task_file n=36 path=src/obs.rs
what: the config keys and the observability setup (CONFIG_KEYS now four)
@task_file n=37 path=crates/viola-agent-claude/Cargo.toml
what: the agent adapter crate's dependencies (chrono removed)
@task_file n=38 path=Cargo.toml
what: the root package manifest and its test targets (cli_budget dropped)
@task_file n=39 path=.config/nextest.toml
what: the test runner's profiles and kill filters (cli_budget dropped)
@task_file n=40 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene check, walking the new 0.1.0 fixture directory
@task_item n=1 part=touches
decision/budget-paused-stops-new-sends: its subject, the budget-paused state as minimum fuel, is retired whole; no send is ever stopped on usage.
@task_item n=2 part=touches
decision/budget-as-ruled-figures and decision/expired-window-shown-expired: the budget figures and the budget window are no longer read or shown anywhere.
@task_item n=3 part=touches
decision/readings-carry-their-age, contract/viola-list-board/reading-age and contract/viola-list-board/window-figure: where they hold the budget reading's age or window figure, that reading is gone from the board.
@task_item n=4 part=touches
contract/viola-list-table/bay-line and contract/viola-list-board/skip-counts: the BAY line is now BAY  skipped a - b - c; its 5H, 7D, read and gate cells are gone.
@task_item n=5 part=touches
contract/viola-list-board/human-holder-budget-paused: the human holder under a budget pause has no state left to show; retire.
@task_item n=6 part=touches
contract/flight-progress-strip/wheel, decision/wheel-as-words and contract/viola-list-board/status: the WHEEL cell and the board words lose the budget-paused form.
@task_item n=7 part=touches
decision/wheel-column-width-23: the width stays 23 but its reason, the widest word unknown - budget-paused, is gone; the reason needs rewording.
@task_item n=8 part=touches
contract/refusal-hints/budget-paused: the refusal and its hint are retired.
@task_item n=9 part=touches
contract/cli-exit-codes/11 and contract/cli-exit-codes/10-14: exit 11 (budget-paused) is retired, never reused; state it as 11 — retired (was budget-paused), never reused; 10, 12, 13, 14 unchanged.
@task_item n=10 part=touches
contract/cli-exit-codes/14 and contract/wrapper-fault-detail/unknown-code: an older wrapper's budget-paused now reads unknown, exit 14.
@task_item n=11 part=touches
contract/cli-verb-output/release-budget, contract/viola-release-budget/lifted-line, contract/viola-release-budget/open-line and contract/viola-release-budget/override-time: viola release --budget is gone (clap refuses the flag, exit 2), with its lifted and open lines and the override time; retire.
@task_item n=12 part=touches
contract/cli-verb-output/release, name/command/release and decision/pause-release-transfer-control: release now only transfers the wheel; its reply carries the holder alone, no budget_paused or override_until.
@task_item n=13 part=touches
contract/wrapper-fault-detail/release-from-driver and contract/wrapper-fault-detail/other-invalid-params: a string from is still refused release-from-driver first; any budget key after it is now -32602 invalid-params, data null, no wheel moved.
@task_item n=14 part=touches
decision/fixed-phraseology-vocabulary and the refusal vocabulary rows: budget-paused leaves the fixed vocabulary and the send refusal order (control-character, empty-text, human-typing, turn-running, readiness, no-prompt-submitted).
@task_item n=15 part=touches
contract/viola-list-table/columns, contract/viola-list/columns and decision/list-six-fixed-columns: the --json list document is items and skipped, no budget member; the columns themselves are unchanged.
@task_item n=16 part=touches
contract/viola-verify-output/step-line, contract/viola-verify-output/summary and contract/viola-verify-output/summary-line: verify runs 23 steps, NN/23, 23 pass; any row holding 24 is stale.
@task_item n=17 part=touches
contract/viola-run-output/child-started and decision/run-prints-nothing-while-wrapped: the start writes two records, wheel start then session-start; no budget evaluation at start or at turn end.
@task_item n=18 part=touches
decision/stale-strip-dimmed-not-deleted and contract/flight-progress-strip/no-reading: where a row cites the budget reading as a field with no reading, that field is gone.
@task_fact n=1 slug=surface-cli
fact: viola has no usage-budget gate: it reads no rate limits, keeps no budget file or thresholds, never pauses sends on usage, and release takes no --budget.
@task_fact n=2 slug=surface
fact: A channel release carrying any budget key (true, false, a string, null) is refused -32602 invalid-params, data null, no wheel moved, no record; the release-from-driver refusal is checked first.
@task_fact n=3 slug=surface
fact: Exit 11 is retired (was budget-paused) and never reused; a budget-paused from an older wrapper reads unknown, exit 14.
@task_item n=1 part=untouched
The colour palette, colour tokens, colour depth and the colour decision order: no colour rule moved.
@task_item n=2 part=untouched
Typography, spacing, iconography glyphs and separators, and the rendered-page and web-surface absences.
@task_item n=3 part=untouched
viola revive output, its refusals and --json refusal codes; viola run start refusals: unchanged.
@task_item n=4 part=untouched
Exit 21 causes, the strict-modes refusals, the socket directory refusals and the fixed refusal pairs.
@task_item n=5 part=untouched
viola link and unlink output, viola wait output, viola last output and viola answer: no change.
@task_item n=6 part=untouched
The readback box, the send readback and send mirror forms (open, read back, unable, unconfirmable): unchanged.
@task_item n=7 part=untouched
viola verify refusals (CLI not found, stamps update, unclean fixture, version unreadable, interrupted): unchanged.
@task_item n=8 part=untouched
The self-validation protocol and the anti-pattern rows other than budget-as-ruled-figures: no change.
@task_item n=9 part=untouched
The peer reply screening and the catch-site rows: unchanged.
