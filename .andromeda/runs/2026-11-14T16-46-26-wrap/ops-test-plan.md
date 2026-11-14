@row_set n=1 row=decision/cli-env-vector-negative-tests cell=rule seen=1 detector=D-tests-count-literals
value: Path-traversal names, a --home or VIOLA_DIR failing strict-modes and oversize answer JSON are each covered by negative tests that show them refused.
basis: fct-vwne6ov4
because: The thresholds and their Percent type are retired, so no out-of-range Percent in config.json remains to be refused.
@row_set n=2 row=decision/self-bootstrapping-test-data cell=rule seen=1 detector=D-tests-count-literals
value: Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
basis: fct-vwne6ov4
because: budget.json and its writer are retired, so no test seeds it through viola hook statusline.
@row_set n=3 row=contract/viola-hook-statusline/passthrough cell=promise seen=1 detector=D-tests-count-literals
value: The user's statusline is passed through unchanged.
basis: fct-xxks7iqj
because: viola hook statusline no longer writes a budget file, so the pass-through follows no write.
@row_set n=4 row=contract/viola-list-json/items cell=promise seen=1 detector=D-tests-count-literals
value: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?}.
basis: fct-xxks7iqj
because: The board item's budget_paused field is removed from the list document.
@row_set n=5 row=decision/snapshot-replay-fields cell=rule seen=1 detector=D-tests-count-literals
value: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id and wheel with dialog_pending false; the replay writes no file and gives no cwd.
basis: fct-xxks7iqj
because: The replay no longer recovers budget_paused or budget_override_until; it recovers links, agent_session_id and wheel alone.
@row_set n=6 row=contract/run-start-events/order cell=promise seen=1 detector=none
value: The events log reads wheel with cause start, then the agent's session-start with source hook.
basis: fct-xxks7iqj
because: A start writes no budget-gate record any more, so its records are the wheel line then the hook's session-start.
@row_set n=7 row=decision/e2e-for-critical-paths cell=rule seen=1 detector=D-tests-count-literals
value: Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
basis: fct-vwne6ov4
because: The Path 6 budget suite is retired with the gate, so the stated count of seven critical paths no longer stands.
@row_set n=8 row=contract/boot-readiness/events cell=promise seen=1 detector=D-tests-obs-harness
value: Event lines 1 and 2 are a wheel line with cause start, then session-start with source hook; missing reads events.
basis: fct-uzgo6c7t
because: The harness readiness probe now waits for two start records, the budget-gate line gone.
@row_set n=9 row=contract/harness-boot/statusline-echo cell=promise seen=1 detector=D-tests-count-literals
value: --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var.
basis: fct-vwne6ov4
because: The budget file is retired whole, so naming it as a write the flag avoids keeps a retired surface standing.
@row_set n=10 row=decision/bounded-readers-keep-the-cap cell=rule seen=2 detector=D-tests-count-literals
value: The status line shell-out, the version gate drain, the hyprctl reader and the dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
basis: fct-xalfd6ij
because: The threshold keys and their reader are removed, so no thresholds reader remains to be bounded.
@row_set n=11 row=decision/send-refusal-order cell=rule seen=1 fact=F3 detector=none
value: send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
basis: fct-xxks7iqj
because: The budget rung leaves the send refusal order.
@row_set n=12 row=decision/version-gate-needs-all-relied-rows cell=rule seen=1 fact=F9 detector=none
value: A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified.
basis: fct-f7ziiwwy
because: The statusline-rate-limits row is retired, and a stamp still holding it verifies on the relied rows alone.
@row_set n=13 row=decision/hook-exec-against-live-wrapper cell=rule seen=1 detector=D-tests-count-literals
value: viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.
basis: fct-vwne6ov4
because: The hook writes no budget.json any more, so none is asserted.
@row_set n=14 row=decision/bad-config-value-falls-back cell=guard seen=1 detector=D-tests-count-literals
value:
basis: fct-vwne6ov4
because: The named guard setting_does_not_disable_the_budget_gate is deleted with the Path 6 suite.
@row_set n=15 row=decision/hook-tests-real-binary cell=rule seen=1 detector=D-tests-count-literals
value: Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line and the hook diagnostics trace.
basis: fct-vwne6ov4
because: budget.json is retired, so hook tests assert no budget file.
@row_set n=16 row=decision/property-tests-on-parsers cell=rule seen=2 detector=D-tests-count-literals
value: proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
basis: fct-xxks7iqj
because: The statusline rate_limits reader and its proptest are removed, its regression seeds deleted.
@row_set n=17 row=contract/hook-statusline/passthrough cell=promise seen=1 detector=D-tests-count-literals
value: The statusline output passes through byte for byte.
basis: fct-xxks7iqj
because: viola hook statusline reads no rate_limits and writes no budget.json, so no fed reading is held.
@row_set n=18 row=contract/viola-send/exit-11 cell=promise seen=1 fact=F1 detector=D-tests-count-literals
value: Retired (was budget-paused) and never renumbered or reused: no refusal maps to 11, and an older wrapper's budget-paused word reads unknown, exit 14.
basis: fct-xxks7iqj
because: The budget-paused refusal is removed and exit 11 is retired, never reused.
@row_set n=19 row=decision/property-tests-with-proptest cell=rule seen=2 detector=D-tests-count-literals
value: Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
basis: fct-z7blgxhx
because: The statusline proptest-regressions file is deleted with the rate_limits reader.
@row_set n=20 row=contract/event-log/start-wheel-record cell=promise seen=1 fact=F4 detector=none
value: Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record, and the hook's session-start is its second and last start record.
basis: fct-xxks7iqj
because: A start now writes two records, the budget-gate record gone, and viola revive shares the start.
@row_set n=21 row=contract/viola-release/document cell=promise seen=1 detector=D-tests-count-literals
value: Gives {wheel:"driver"}, after which a send exits 0.
basis: fct-xxks7iqj
because: A plain release now answers {wheel} alone, budget_paused gone.
@row_set n=22 row=decision/per-test-home cell=rule seen=1 detector=D-tests-count-literals
value: Each test has its own home, so no endpoint or log is shared between tests.
basis: fct-vwne6ov4
because: budget.json is retired, so it is no longer a state a test could share.
@row_set n=23 row=decision/time-as-argument cell=rule seen=1 detector=D-tests-count-literals
value: A pure time decision takes its instant as an argument instead of reading a clock.
basis: fct-vwne6ov4
because: The budget gate's override expiry is retired, so it no longer stands as the example.
@row_set n=24 row=decision/fakes-for-external-services cell=rule seen=3 detector=D-tests-count-literals
value: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services.
basis: fct-vwne6ov4
because: viola reads no Anthropic usage values any more, so no generated statusline stdin stands in for them.
@row_set n=25 row=decision/utc-now-read-directly cell=rule seen=1 detector=D-tests-count-literals
value: chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only.
basis: fct-vwne6ov4
because: The budget gate is retired, so it no longer stands as the pure decision named.
@row_set n=26 row=decision/state-files-never-hand-written cell=rule seen=1 detector=D-tests-count-literals
value: Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand.
basis: fct-vwne6ov4
because: budget.json and its writer are retired.
@row_set n=27 row=decision/snapshot-replay-from-log cell=rule seen=2 detector=D-tests-count-literals
value: Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel and the agent session id included, with no dialog pending, and write no file.
basis: fct-xxks7iqj
because: The replay recovers links, agent_session_id and wheel alone, the budget pause and override end gone; its guard still names the deleted cli_budget cases (to re-point to a_start_over_an_unreadable_snapshot_replays_the_log and the cli_links case) in a later pass.
@row_retire n=1 row=decision/budget-pause-thresholds seen=1 heir=absence/usage-budget-gate detector=D-tests-count-literals
why: The usage budget gate, its thresholds and release --budget are removed whole.
basis: fct-vwne6ov4
because: The gate is retired on the founder's ruling, so no threshold pauses send.
@row_retire n=2 row=contract/viola-hook-statusline/budget-json seen=1 detector=D-tests-count-literals
why: viola hook statusline reads no rate_limits and writes no budget.json.
basis: fct-xxks7iqj
because: The budget file write is removed from the statusline hook.
@row_retire n=3 row=contract/viola-list-json/budget seen=1 detector=D-tests-count-literals
why: The list ok payload is {items, skipped}, with no budget member.
basis: fct-xxks7iqj
because: viola list reads no budget and its document lost the budget member.
@row_retire n=4 row=contract/budget-json/rate-limits seen=1 detector=D-tests-count-literals
why: budget.json and the rate_limits reading are retired.
basis: fct-vwne6ov4
because: No writer of budget.json remains.
@row_retire n=5 row=contract/cli-release/budget seen=1 detector=D-tests-count-literals
why: release --budget and budget_paused are removed.
basis: fct-xxks7iqj
because: release lost its --budget flag and its budget reply fields.
@row_retire n=6 row=contract/cli-send-json/budget-paused seen=1 detector=D-tests-count-literals
why: No send is refused by a budget gate, and exit 11 is retired.
basis: fct-velgct6n
because: The budget-paused refusal and its exit 11 are retired.
@row_retire n=7 row=contract/event-log/budget-gate seen=1 detector=D-tests-count-literals
why: The budget-gate event kind is removed.
basis: fct-xxks7iqj
because: EventKind lost budget-gate, so no budget pause is logged.
@row_retire n=8 row=decision/budget-json-from-statusline seen=1 detector=D-tests-count-literals
why: budget.json is retired, so no test takes it from the statusline hook.
basis: fct-vwne6ov4
because: The budget file and its single writer are removed.
@row_retire n=9 row=contract/agent-run-status-polled-fields/budget seen=1 detector=D-tests-obs-harness
why: The status document's list carries no budget member.
basis: fct-uzgo6c7t
because: The list document lost its budget member, so the status shape no longer holds it.
@row_retire n=10 row=contract/agent-run-status-polled-fields/budget-paused seen=1 detector=D-tests-obs-harness
why: List items carry no budget_paused field.
basis: fct-uzgo6c7t
because: The board item's budget_paused is removed, so the status document no longer holds it.
@row_retire n=11 row=decision/budget-decision-takes-the-instant seen=1 detector=D-tests-count-literals
why: The budget decision and its thresholds and override expiry are removed.
basis: fct-xxks7iqj
because: run::budget with decide and the lift is deleted.
@row_retire n=12 row=decision/budget-governor-thresholds seen=1 detector=D-tests-count-literals
why: The budget governor and its thresholds are removed.
basis: fct-vwne6ov4
because: No send is refused budget-paused and release --budget is gone.
@row_retire n=13 row=decision/budget-pause-threshold seen=1 detector=D-tests-count-literals
why: No budget reading pauses the driver any more.
basis: fct-vwne6ov4
because: The thresholds and the gate are retired.
@row_retire n=14 row=contract/run-start-records/first-three seen=1 heir=contract/event-log/start-wheel-record detector=none
why: A start writes two records, not three, the budget-gate record gone.
basis: fct-xxks7iqj
because: Its item names the first three records, which no longer stand.
@row_retire n=15 row=contract/viola-release/budget seen=1 heir=contract/viola-release/budget-flag detector=D-tests-count-literals
why: release --budget no longer lifts anything: the flag is a usage error.
basis: fct-xxks7iqj
because: release lost its --budget flag with the budget pause.
@row_retire n=16 row=decision/budget-gate-covers-send-only seen=1 detector=D-tests-count-literals
why: The budget gate is removed whole and its Path 6 guard is deleted.
basis: fct-vwne6ov4
because: No gate remains to cover send.
@row_retire n=17 row=decision/budget-override-ends-with-wrapper seen=1 detector=D-tests-count-literals
why: The budget override and its Path 6 guard are removed.
basis: fct-vwne6ov4
because: release --budget and the override are retired.
@row_retire n=18 row=decision/budget-override-per-instance seen=1 detector=D-tests-count-literals
why: The budget override and its guards are removed.
basis: fct-vwne6ov4
because: release --budget and the per-instance gate are retired.
@row_retire n=19 row=decision/driver-hints-never-name-release seen=1 heir=decision/hints-never-suggest-release detector=D-tests-count-literals
why: Its rule names the removed budget-paused hint and its guard the deleted path6 case.
basis: fct-xxks7iqj
because: The budget-paused hint and the Path 6 suite are removed, two cells at once.
@row_retire n=20 row=decision/five-hour-pause-threshold seen=1 detector=D-tests-count-literals
why: No five_hour reading pauses send and exit 11 is retired.
basis: fct-vwne6ov4
because: The thresholds and the gate are retired.
@row_retire n=21 row=decision/override-expiry-tested-at-unit-level seen=1 detector=D-tests-count-literals
why: The budget override and the pure gate decision are removed.
basis: fct-xxks7iqj
because: run::budget with decide and the lift is deleted.
@row_retire n=22 row=decision/release-without-pause-records-nothing seen=1 detector=D-tests-count-literals
why: release --budget is removed and its Path 6 guard deleted.
basis: fct-xxks7iqj
because: The flag is a usage error now, so no release --budget runs.
@row_retire n=23 row=decision/statusline-proptest-whole-seconds seen=1 detector=D-tests-count-literals
why: The statusline rate_limits reader and its property are removed.
basis: fct-xxks7iqj
because: statusline::reading and its proptest are deleted.
@row_retire n=24 row=decision/unknown-reset-does-not-pause seen=1 detector=D-tests-count-literals
why: No budget window pauses anything any more.
basis: fct-vwne6ov4
because: The gate and its Path 6 guard are retired.
@row_retire n=25 row=contract/budget-gate-record/paused-false seen=1 detector=D-tests-count-literals
why: The budget-gate event kind and budget_override_until are removed.
basis: fct-xxks7iqj
because: No budget-gate record is logged any more.
@row_retire n=26 row=contract/budget-gate-record/paused-true seen=1 detector=D-tests-count-literals
why: The budget-gate event kind is removed.
basis: fct-xxks7iqj
because: No budget pause is logged any more.
@row_retire n=27 row=contract/budget-json/write-condition seen=1 detector=D-tests-count-literals
why: budget.json is never written.
basis: fct-vwne6ov4
because: The budget file and its writer are retired.
@row_retire n=28 row=contract/budget-paused-refusal/send-exit-11 seen=1 detector=D-tests-count-literals
why: The budget-paused refusal is removed and exit 11 retired.
basis: fct-velgct6n
because: No send is refused for usage any more.
@row_retire n=29 row=contract/list-human/wheel-budget-paused seen=1 detector=D-tests-count-literals
why: The WHEEL cell has no budget-paused form.
basis: fct-xxks7iqj
because: The board word budget-paused is removed.
@row_retire n=30 row=contract/list-json/budget-paused seen=1 detector=D-tests-count-literals
why: The list document carries neither ok.budget nor an item budget_paused.
basis: fct-xxks7iqj
because: Both budget fields of the list document are removed.
@row_retire n=31 row=contract/mcp-tool-result/budget-paused seen=1 detector=D-tests-count-literals
why: The budget-paused refusal is removed from the MCP result.
basis: fct-vwne6ov4
because: RefusalReason lost budget-paused, its MCP sample retired.
@row_retire n=32 row=contract/release-budget-json/document seen=1 detector=D-tests-count-literals
why: release --budget is removed.
basis: fct-xxks7iqj
because: release lost its --budget flag and its reply fields.
@row_retire n=33 row=contract/statusline-payload/resets-at seen=1 detector=D-tests-count-literals
why: The statusline rate_limits reading and budget.json are removed.
basis: fct-xxks7iqj
because: statusline::reading is deleted, so no resets_at is read.
@row_retire n=34 row=contract/viola-run-start/event-order seen=1 heir=contract/event-log/start-wheel-record detector=none
why: A start writes two records, not three, the budget-gate record gone.
basis: fct-xxks7iqj
because: Its item names the first three events, which no longer stand.
@row_retire n=35 row=decision/budget-file-from-statusline-hook seen=1 detector=D-tests-count-literals
why: budget.json is retired.
basis: fct-vwne6ov4
because: The budget file and its reader are removed.
@row_retire n=36 row=decision/budget-seeded-through-statusline-hook seen=1 detector=D-tests-count-literals
why: No budget file is seeded or read any more.
basis: fct-vwne6ov4
because: The budget file, its writer and its reader are removed.
@row_add n=1 row=absence/usage-budget-gate area=1-test-scope-summary fact=F7 detector=none
capability: A usage budget gate: no rate_limits reading, no budget file and no send refused for usage
basis: fct-vwne6ov4
because: The budget gate is removed whole while the status-line pass-through and the driver release guard stay.
@row_add n=2 row=contract/channel-release/budget-key area=6-e2e-test-strategy fact=F2 detector=none
boundary: the channel release method
item: budget key
promise: A release carrying any budget key, true, false, a string or null, is -32602 invalid-params with data null, moves no wheel and writes no record; a string from is refused release-from-driver first.
basis: fct-xxks7iqj
because: The budget key is refused rather than skipped, since a skipped key would hand a human-held wheel to the driver.
@row_add n=3 row=contract/viola-release/budget-flag area=6-e2e-test-strategy fact=F2 detector=none
boundary: viola release
item: --budget flag
promise: viola release --budget is a clap usage error, exit 2.
basis: fct-tjydxlay
because: The --budget flag is removed from viola release.
@row_add n=4 row=decision/budget-era-state-read-whole area=e2e-test-strategy fact=F5 detector=none
name: Budget-era state read whole
rule: A snapshot or event log written by a 0.1.0 budget-era wrapper is read whole, its budget fields and budget-gate lines skipped; a new snapshot holds no budget key.
why: No paused state remains, so no compatibility budget key is written for older readers.
basis: fct-u67vyamn
because: Old disk state must still read after the removal, and the compat shim for 0.1.0 readers was rejected.
@row_add n=5 row=decision/retired-words-as-fixtures area=test-data-fixtures fact=F6 detector=none
name: Retired words kept as committed fixtures
rule: The 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases that must spell an old word, and walked by the fixture hygiene check.
why: A removal's identifier probe and a case that must spell the removed word cannot share a tree.
basis: fct-p5p6ndjb
because: The probe forbidding the retired words and the compatibility cases spelling them were jointly unsatisfiable until the words moved to fixtures.
@row_add n=6 row=decision/retired-config-keys-not-refused area=integration-test-strategy fact=F8 detector=none
name: Retired threshold keys logged, never refused
rule: A config.json still carrying five_hour or seven_day logs one parse-rejected config-json unknown-keys line at its process's start, and nothing refuses.
basis: fct-xalfd6ij
because: The threshold keys left the config schema, and an older config holding them must not stop the product.
