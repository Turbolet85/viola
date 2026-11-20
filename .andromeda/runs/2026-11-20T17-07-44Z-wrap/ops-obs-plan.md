@row_set n=1 row=contract/config-json/known-keys cell=promise seen=2 detector=none
value: config.json has five known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s, verify_new_cli; any other key, a retired five_hour or seven_day included, is counted as skipped in one parse-rejected config-json unknown-keys line at its process's start, and nothing refuses.
basis: fct-wb3kvjz5
because: The known config.json keys moved from four to five with verify_new_cli, so the row's count and key list must follow.
@row_set n=2 row=contract/diag-line-fields/process-start cell=promise seen=3 detector=D-obs-instrumentation
value: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe, auto-verify), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
basis: fct-xgc7qjiw
because: The diag-line subject enum gained auto-verify, so the row's closed subject list must name it.
@row_set n=3 row=contract/run-terminal-output/child-output cell=promise seen=1 detector=none
value: run writes nothing to the terminal except the child's own output and, with verify_new_cli on, the start's one automatic-verify line and the verify child's own output before the child starts; while the child runs, neither run nor a revived start writes anything of its own.
basis: fct-zfntgo5a
because: A start with verify_new_cli on now writes its automatic-verify line and lets the verify child's output reach the terminal before the child starts, so the row's only-the-child promise is false as it stands.
@row_add n=1 row=contract/diag-subject/auto-verify area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: diag-line subject enum
item: auto-verify
promise: Subject of the process-start and process-exit pair of the viola verify child that a viola run start spawns, the exit carrying child_exit_status and duration_ms, written to the run log.
basis: fct-xgc7qjiw
because: The closed subject enum of the diag-line schema gained auto-verify for the start's verify child, a contract item no row names yet.
@row_add n=2 row=contract/config-json/verify-new-cli area=otel-sdk-init fact=F2 detector=none
boundary: config.json
item: verify_new_cli
promise: A JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line, and the key is read only when v is 1.
basis: fct-xgc7qjiw
because: config.json gained the verify_new_cli key, a promised file format item with its default and rejection line.
@row_add n=3 row=contract/run-terminal-output/auto-verify-line area=3-observability-harness-contract fact=F3 detector=none
boundary: run terminal output
item: automatic-verify line
promise: With verify_new_cli on, viola run writes one fixed ASCII stderr line per start (verifying, did not verify, earlier red, verifying elsewhere, or no stamp with spawn-failed, unstamped or lock-refused), one write, no SGR, carrying only the parsed version and a closed word, and none under revive --json.
basis: fct-zfntgo5a
because: The start's automatic verify adds fixed stderr lines, the named exception to run's silence on the terminal, which no row states.
@row_add n=4 row=decision/auto-verify-on-run-start-span area=span-trace-coverage fact=F4 detector=D-obs-instrumentation
name: Start's automatic verify on run.start's span
rule: The start's automatic verify opens no span of its own: its auto-verify lines sit on run.start's span, after channel.bind and before the trust record.
basis: fct-zfntgo5a
because: The new automatic verify step is instrumented by its log pair alone on the start's root span, which the span rows must record so no span is expected of it.
@row_add n=5 row=contract/run-process-log/auto-verify-stop area=span-trace-coverage fact=F5 detector=D-obs-instrumentation
boundary: run process log
item: stop during the automatic verify
promise: A SIGINT or SIGTERM during the start's automatic verify TERMs the verify child, KILLs it after a 30 s grace, writes the auto-verify process-exit, then re-raises the signal: the start ends with no process-exit of its own, no stamp, no snapshot and no session.
basis: fct-zfntgo5a
because: A stop in the start's verify window now ends the start by the signal with no process-exit of its own, an exception to the run log's self exit line that no row states.
@row_add n=6 row=decision/auto-verify-lock-writes-no-line area=span-trace-coverage fact=F6 detector=D-obs-instrumentation
name: Automatic verify attempt lock writes no line
rule: The automatic verify's attempt lock is an owner-only 0600 file taken after the strict stamps-modes check and separate from the stamps writer's lock; a held lock reads as verifying elsewhere and writes no log line.
basis: fct-u3yuycke
because: The new attempt lock is uninstrumented by design, so the span rows must say a held lock leaves no line rather than read it as a gap.
@row_add n=7 row=decision/attempted-cli-never-auto-verified area=span-trace-coverage fact=F7 detector=none
name: Attempted CLI version never verified automatically again
rule: A CLI version whose ledger entry is already attempted, every relied row holding a word whatever it reads, is never verified again automatically: the start shows the earlier-red line and runs unverified.
why: A red verify cannot loop.
basis: fct-zfntgo5a
because: The start's automatic verify decides by the entry's attempted reading, a decision with its stated reason that no row holds.
