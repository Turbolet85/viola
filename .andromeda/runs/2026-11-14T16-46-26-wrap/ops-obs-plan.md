@row_add n=1 row=absence/usage-budget-gate area=1-obs-scope-summary fact=F1 detector=none
capability: A usage budget gate: viola reads no rate limits, keeps no budget file and never pauses a send on usage
basis: fct-vwne6ov4
because: The budget gate was retired whole, so the absence of the capability is what the master now keeps.
@row_add n=2 row=contract/cli-exit-codes/11 area=6-log-coverage fact=F3 detector=none
boundary: CLI exit codes
item: 11
promise: Retired (was budget-paused), never renumbered or reused; an older wrapper's budget-paused refusal reads unknown, exit 14.
basis: fct-xxks7iqj
because: No refusal maps to exit 11 any more and an older wrapper's budget-paused falls back to unknown, a promise at the exit-code boundary.
@row_add n=3 row=contract/statusline-span-attributes/statusline-shell-out area=span-trace-coverage detector=none
boundary: statusline span attributes
item: statusline.shell_out
promise: statusline.shell_out carries shell_exit_status and duration_ms.
basis: fct-vwne6ov4
because: The budget span attributes were retired while the status-line shell-out stays, so its attributes stand under a boundary that is not the budget's.
@row_add n=4 row=contract/statusline-spans/span-chain area=span-trace-coverage detector=none
boundary: statusline spans
item: span chain
promise: hook.statusline (SERVER) › statusline.shell_out (CLIENT).
basis: fct-xxks7iqj
because: The status-line arm no longer writes a budget file and run.budget_eval is gone, so the chain keeps the hook span and the shell-out alone.
@row_set n=1 row=decision/diag-schema-additive-only cell=except seen=1 detector=none
value: Removing a retired capability drops its fields and words from the schema while v stays 1.
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with v kept at 1, an exception the additive-only rule must name.
@row_set n=2 row=decision/log-fields-additive-only cell=except seen=1 detector=none
value: Removing a retired capability drops its fields from the schema while v stays 1.
basis: fct-xalfd6ij
because: budget_written and the send lines' budget fields were removed from the schema, an exception the additive-only rule must name.
@row_set n=3 row=decision/obs-aligns-to-tests-log-format cell=except seen=1 detector=none
value: Removing a retired capability drops its fields and words from the schema while v stays 1.
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with v kept at 1, an exception the no-remove rule must name.
@row_set n=4 row=decision/log-schema-additive-only cell=except seen=1 detector=none
value: Removing a retired capability drops its fields and words from the schema while v stays 1.
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with v kept at 1, an exception the additive-only rule must name.
@row_set n=5 row=decision/unemittable-schema-words-removed cell=rule seen=1 fact=F2 detector=none
value: A field or closed word of the diag-line schema that no build can emit leaves the schema with its emitter, a retired capability's fields and words included, and v stays 1; os is closed at linux.
basis: fct-xalfd6ij
because: The retired budget gate's fields and words left the diag-line schema with v kept at 1, which this row states only for closed words.
@row_set n=6 row=decision/statusline-command-confined cell=rule seen=1 detector=none
value: The statusline_command string never appears outside diagnostics/.
basis: fct-vwne6ov4
because: The budget governor path is retired, while the status-line pass-through that runs the command stays.
@row_set n=7 row=decision/exit-code-log-field cell=rule seen=1 detector=none
value: The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
basis: fct-xxks7iqj
because: Nothing maps to exit 11 any more, so the listed codes lose it.
@row_set n=8 row=contract/channel-response/invalid-params-causes cell=promise seen=1 detector=D-obs-instrumentation
value: detail invalid-params covers send's invalid params, a release whose from is not a string or that carries any budget key, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
basis: fct-tjydxlay
because: A release carrying a budget key is now refused -32602 with channel-response detail invalid-params, a cause the list lacks.
@row_set n=9 row=contract/run-start-events/order cell=promise seen=1 detector=none
value: A run start writes wheel{cause:"start"}, then session-start.
basis: fct-xxks7iqj
because: The start writes one start record and no budget-gate record any more.
@row_set n=10 row=contract/config-json/known-keys cell=promise seen=1 fact=F5 detector=none
value: config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, a retired five_hour or seven_day included, is counted as skipped in one parse-rejected config-json unknown-keys line at its process's start, and nothing refuses.
basis: fct-xalfd6ij
because: The threshold keys left the config schema and a config still carrying them logs one unknown-keys line without refusing.
@row_set n=11 row=contract/config-json/v cell=promise seen=1 detector=none
value: A document that is no object, or whose v is not 1, gives the hold default and no further line.
basis: fct-xalfd6ij
because: The threshold keys are gone, so there is no threshold default left to give.
@row_set n=12 row=decision/product-order-from-events-log cell=rule seen=1 detector=none
value: The product order wheel{cause:"start"} then session-start{source:"hook"} is verified from the instance's events log, not duplicated into process logs.
basis: fct-dp3hpsyg
because: A start now writes wheel then the hook's session-start, with no budget-gate between them.
@row_set n=13 row=contract/release-refusal/invalid-params cell=promise seen=1 fact=F4 detector=D-obs-instrumentation
value: A from of another type, or any budget key (true, false, a string or null), is a plain -32602 (data null) after the release-from-driver check, with no release-from-driver line, no wheel moved and no record; its channel-response carries detail invalid-params on both sides at WARN.
basis: fct-xxks7iqj
because: A release carrying any budget key is now refused as invalid-params with no wheel moved and no record, where the row named only a non-bool budget.
@row_set n=14 row=contract/run-start-spans/span-chain cell=promise seen=1 detector=none
value: run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
basis: fct-xxks7iqj
because: The start has no run.budget_eval span any more.
@row_set n=15 row=contract/statusline-log-fields/hook-decision cell=promise seen=1 detector=none
value: hook-decision{hook_event:"statusline", duration_ms}, deadline_hit only when true.
basis: fct-xxks7iqj
because: The status-line arm writes no budget file and its hook-decision carries no budget_written.
@row_set n=16 row=decision/gauges-from-harness-snapshots cell=rule seen=1 detector=none
value: A current-value gauge is read from harness snapshots: liveness via agent-run status.
basis: fct-vwne6ov4
because: budget.json and its read_at are retired, so no budget gauge remains.
@row_set n=17 row=contract/diag-line-fields/hook-events cell=promise seen=1 detector=none
value: hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
basis: fct-xalfd6ij
because: The diag-line schema dropped budget_written from hook-decision.
@row_set n=18 row=contract/diag-line-fields/send-events cell=promise seen=1 detector=none
value: from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
basis: fct-xalfd6ij
because: The diag-line schema dropped window, used_percentage, resets_at and paused from the send lines.
@row_set n=19 row=contract/send-refused-detail/refusal-reason-words cell=promise seen=1 detector=none
value: Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
basis: fct-vwne6ov4
because: budget-paused and its five-hour and seven-day details are retired.
@row_set n=20 row=decision/typed-ids-never-in-detail cell=rule seen=1 detector=none
value: pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
basis: fct-vwne6ov4
because: The budget percentages and resets_at are no longer read or logged.
@row_retire n=1 row=decision/budget-metadata-loggable seen=1 detector=none
why: budget.json and its percentages and resets_at no longer exist.
basis: fct-vwne6ov4
because: The budget file was retired whole, so a rule about logging its metadata states nothing.
@row_retire n=2 row=contract/budget-log-line/window seen=1 detector=none
why: No log line carries a budget window any more.
basis: fct-xalfd6ij
because: The window field left the diag-line schema with the budget gate.
@row_retire n=3 row=contract/budget-path-log-events/event-set seen=1 detector=none
why: The budget path and the budget-paused refusal are retired.
basis: fct-vwne6ov4
because: No send is refused budget-paused and the status-line arm writes no budget file, so the budget path's event set is gone.
@row_retire n=4 row=contract/config-json/five-hour seen=1 detector=none
why: five_hour is no longer a config.json key.
basis: fct-xalfd6ij
because: The config schema dropped five_hour, which now reads as an unknown key.
@row_retire n=5 row=contract/config-json/seven-day seen=1 detector=none
why: seven_day is no longer a config.json key.
basis: fct-xalfd6ij
because: The config schema dropped seven_day, which now reads as an unknown key.
@row_retire n=6 row=decision/unreadable-budget-reads-as-none seen=1 detector=none
why: No budget file is kept or read.
basis: fct-xxks7iqj
because: The budget.json readers were removed with the state crate's budget module.
@row_retire n=7 row=contract/budget-span-attributes/run-budget-eval seen=1 detector=none
why: The run.budget_eval span is retired.
basis: fct-vwne6ov4
because: The governor's evaluation and its span left the product.
@row_retire n=8 row=contract/budget-span-attributes/state-budget-write seen=1 detector=none
why: No budget file is written, so state.budget_write is gone.
basis: fct-xxks7iqj
because: The budget.json writer was removed.
@row_retire n=9 row=contract/budget-span-attributes/statusline-shell-out seen=1 heir=contract/statusline-span-attributes/statusline-shell-out detector=none
why: The budget span attributes boundary is retired while the shell-out span stays.
basis: fct-vwne6ov4
because: Its boundary names the retired budget capability, so the shell-out attributes move to a status-line boundary.
@row_retire n=10 row=contract/budget-spans/span-chain seen=1 heir=contract/statusline-spans/span-chain detector=none
why: The budget governor spans are retired; the status-line chain keeps the hook span and the shell-out alone.
basis: fct-xxks7iqj
because: state.budget_write and run.budget_eval are gone with the governor.
@row_retire n=11 row=contract/events-log/budget-gate seen=1 detector=none
why: The budget-gate event kind is retired.
basis: fct-xxks7iqj
because: EventKind::BudgetGate was removed and no budget-gate record is appended.
@row_retire n=12 row=contract/release-budget/budget-gate seen=1 heir=contract/release-refusal/invalid-params detector=none
why: A release carrying budget is now refused as invalid-params instead of appending a budget-gate record.
basis: fct-xxks7iqj
because: The release method lost its budget param, and any budget key is refused -32602.
@row_retire n=13 row=contract/send-refused/budget-paused seen=1 detector=none
why: The budget-paused refusal and its exit 11 are retired.
basis: fct-xxks7iqj
because: RefusalReason::BudgetPaused was removed and the send path has no budget rung.
