@row_set n=1 row=decision/driver-release-refused cell=rule seen=1 detector=none
value: The wrapper refuses a release whose params carry a string from, before any other check of its params.
basis: fct-xxks7iqj
because: The release method has no budget left to act on, its string-from refusal now coming first ahead of the non-string from and budget-key checks, so the rule's budget clause is false.
@row_set n=2 row=decision/release-from-driver-guard cell=rule seen=1 detector=none
value: The wrapper refuses a release whose params carry a string from, before any other check of its params.
basis: fct-xxks7iqj
because: The release method has no budget left to act on, its string-from refusal now coming first ahead of the non-string from and budget-key checks, so the rule's budget clause is false.
@row_set n=3 row=contract/channel-release/params cell=promise seen=1 detector=none
value: release params are {from?}; a release on a wheel the driver already holds changes nothing.
basis: fct-vwne6ov4
because: The budget param of release is retired, so the params are {from?} alone.
@row_set n=4 row=contract/channel-release/string-from cell=promise seen=1 detector=none
value: Refused with -32602 "invalid params" and data {"reason":"release-from-driver"}, one release-from-driver line logged (from_trust:"self-reported").
basis: fct-xxks7iqj
because: No budget pause or snapshot budget field remains, so the clause about them describes nothing.
@row_set n=5 row=contract/channel-frames/release-params cell=promise seen=1 fact=F1 detector=D-security-input
value: from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null; after those checks any budget key, true, false, a string or null, is -32602 invalid-params with data null, no wheel moved and no record written.
basis: fct-xxks7iqj
because: A budget key is no longer a bool param but refused whatever its value, after the from checks, so the frame's params contract changes.
@row_set n=6 row=decision/config-cannot-disable-controls cell=rule seen=1 detector=none
value: No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
basis: fct-vwne6ov4
because: The budget gate no longer exists, so no configuration value can switch it off.
@row_set n=7 row=decision/config-tolerant-parse cell=rule seen=1 fact=F4 detector=none
value: config.json is parsed tolerantly, its v checked and its read capped at MAX_FRAME; unknown keys, a retired threshold key among them, give one parse-rejected config-json unknown-keys line at the process start and refuse nothing; a bad value falls back to its default and is reported as parse-rejected.
basis: fct-xalfd6ij
because: five_hour and seven_day left the config keys, and a config.json still carrying them is read as holding unknown keys that refuse nothing.
@row_set n=8 row=decision/statusline-payload-fails-silent cell=rule seen=1 fact=F6 detector=none
value: The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr and nothing printed; the hook reads no rate_limits and writes no file.
basis: fct-xxks7iqj
because: hook statusline lost its rate_limits reading and budget-file write and is a pass-through after its instance check, capped stdin and malformed-json refusal.
@row_retire n=1 row=decision/budget-gate-fails-open seen=1 detector=none
why: The budget gate and budget.json are removed, and no send is refused for usage.
basis: fct-vwne6ov4
because: The gate the row describes was retired whole.
@row_retire n=2 row=decision/home-write-rule-checkers seen=1 heir=decision/mcp-and-plugin-install-check-home detector=none
why: The budget read is removed, so the row's name and rule name a checker that no longer exists.
basis: fct-xwb5j2vd
because: The budget module and its readers are deleted, leaving the start of viola mcp and viola plugin install as the write-rule checkers.
@row_retire n=3 row=decision/budget-file-single-writer seen=1 detector=none
why: budget.json is no longer written by anything.
basis: fct-vwne6ov4
because: budget.json, its writer and its single-writer decisions were retired with the gate.
@row_retire n=4 row=decision/budget-json-single-writer seen=1 detector=none
why: budget.json is no longer written by anything, and the rate_limits reading is gone.
basis: fct-vwne6ov4
because: budget.json, its writer and its single-writer decisions were retired with the gate.
@row_retire n=5 row=decision/failed-budget-read-no-reading seen=1 detector=none
why: No budget file is read any more.
basis: fct-vwne6ov4
because: The budget file's readers were deleted with the gate.
@row_retire n=6 row=contract/budget-json/resets-at seen=1 detector=none
why: The budget.json format no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields, resets_at among them, were retired.
@row_retire n=7 row=contract/budget-json/used-percentage seen=1 detector=none
why: The budget.json format no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields, used_percentage among them, were retired.
@row_retire n=8 row=contract/budget-json/window seen=1 detector=none
why: The budget.json format no longer exists.
basis: fct-vwne6ov4
because: budget.json and its contract fields, window among them, were retired.
@row_retire n=9 row=contract/config-json/five-hour seen=1 detector=none
why: five_hour is no longer a config.json key.
basis: fct-xalfd6ij
because: The config schema dropped five_hour and the key list no longer holds it.
@row_retire n=10 row=contract/config-json/seven-day seen=1 detector=none
why: seven_day is no longer a config.json key.
basis: fct-xalfd6ij
because: The config schema dropped seven_day and the key list no longer holds it.
@row_add n=1 row=decision/mcp-and-plugin-install-check-home area=authentication-authorization detector=none
name: mcp start and plugin install check the home
rule: The start of viola mcp and viola plugin install hold the home to the write rule.
basis: fct-xwb5j2vd
because: The row replaces the retired checker list without the deleted budget read.
@row_add n=2 row=contract/cli-release/budget-flag area=authentication-authorization fact=F2 detector=D-security-input
boundary: CLI viola release
item: --budget
promise: viola release takes no --budget flag; the argument parser refuses it with exit 2.
basis: fct-tjydxlay
because: The --budget flag left viola release and is now refused by the argument parser, a contract of the CLI surface.
@row_add n=3 row=contract/cli-exit-codes/11 area=threat-model-summary fact=F3 detector=none
boundary: CLI exit codes
item: 11
promise: Retired (was budget-paused), never renumbered or reused; an older wrapper's budget-paused refusal reads as unknown, exit 14.
basis: fct-xxks7iqj
because: No refusal maps to exit 11 any more and the code stays retired, a promise of the CLI exit table.
@row_add n=4 row=absence/usage-budget-gate area=threat-model-summary fact=F7 detector=none
capability: A usage budget gate: reading Claude rate limits, keeping a budget file or refusing a send for usage
basis: fct-vwne6ov4
because: The budget gate was removed whole, so the project no longer has this capability.
@row_add n=5 row=decision/budget-era-state-read-tolerantly area=input-validation fact=F5 detector=none
name: Budget-era state read tolerantly
rule: A 0.1.0 snapshot still carrying budget fields reads whole with those fields skipped, a log replay skips budget-gate lines, and every own-format v stays 1 through the budget gate's removal.
basis: fct-fke6bmz4
because: Readers of old disk state stay after the removal, tolerant of its retired fields and kind, with no version bump.
@row_add n=6 row=decision/viola-format-fixtures-hygiene area=data-protection fact=F8 detector=none
name: viola-format fixtures under the hygiene walk
rule: Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email.
guard: viola_format_fixtures_pass_hygiene
basis: fct-tjydxlay
because: The chunk committed viola-format fixtures and widened the hygiene walk to cover them.
