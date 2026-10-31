# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · yes · stale · src/cmd/wait.rs and src/cmd/run.rs are reached whole by the exit-21 and log-code change, so the wait.block and run.wait_dispatch attributes are not shown unmoved
span-trace-coverage#7.3 · yes · contradict · the session retires instance-dead with during call for endpoint-lost (F4), gives the exit-21 --json and MCP instance refusals a cause in place of detail null, and makes the liveness-only pre-check without_endpoint (F5)
span-trace-coverage#7.4 · yes · stale · src/cmd/wait.rs is reached whole by the exit-21 document change, so the cleanup on a server-verification refusal is not shown unmoved
span-trace-coverage#8.1 · yes · stale · src/cmd/answer.rs, src/cmd/client.rs and the MCP check_instance path are reached whole by the exit-21 and cause change, so the refusal and no-frame claims are not shown unmoved
span-trace-coverage#8.2 · yes · stale · src/cmd/run.rs, which holds run.dialog_register and run.dialog_await, is reached whole by the start and collision change, so these attributes are not shown unmoved
span-trace-coverage#8.3 · yes · contradict · MCP answer refused by check_instance no longer returns a bare instance-unreachable with detail null, since the isError body carries the cause, and the CLI --json instance document now names its cause too
span-trace-coverage#8.4 · yes · stale · src/cmd/run.rs, which holds the dialog-expired append and the run.dialog_await close, is reached whole by the start and collision change, so the cleanup is not shown unmoved
span-trace-coverage#9.1 · yes · stale · src/cmd/pause.rs and src/cmd/run.rs are reached whole by the exit-21 and cause change, so the pause and release span claims are not shown unmoved
span-trace-coverage#9.2 · yes · stale · src/cmd/run.rs, which holds run.wheel_transition, is reached whole by the start and cause change, so the attribute list is not shown unmoved
span-trace-coverage#9.3 · yes · stale · src/cmd/pause.rs and src/cmd/run.rs are reached whole, and the server-verification refusal on pause and release sits in the exit-21 path that changed, so the line list is not shown unmoved
span-trace-coverage#9.4 · yes · stale · src/cmd/run.rs, which holds the point-in-time run.wheel_transition span, is reached whole by the change, so the audit-record statement is not shown unmoved
span-trace-coverage#10.1 · yes · stale · src/cmd/run.rs, which holds run.budget_eval, is reached whole by the start and send change, so the span and attribute claim is not shown unmoved
span-trace-coverage#10.2 · yes · stale · src/cmd/run.rs, which holds run.budget_eval, is reached whole by the start change, so the attribute claim is not shown unmoved
span-trace-coverage#10.3 · yes · stale · src/cmd/send.rs, reached whole, holds the send-refused budget-paused line, so the log-field list is not shown unmoved
span-trace-coverage#10.4 · no · the statusline shell-out bound and stdout pass-through live in src/cmd/hook.rs, which is not in the diff, and the hook role's rules stand
span-trace-coverage#11.1 · yes · stale · src/cmd/run.rs holds the claude-child process-start that the strip annotates and is reached whole by the spawn-path change, so the env_stripped and env_kept claim is not shown unmoved
span-trace-coverage#12.1 · yes · stale · src/cmd/send.rs and src/cmd/answer.rs are reached whole and hold the implicit link record inside those calls, so the trace claim is not shown unmoved
span-trace-coverage#13.1 · no · the driven wrapper's link and unlink serving lives in run::links, a file outside the reached set, and no touch names it
span-trace-coverage#14.1 · yes · stale · src/cmd/revive.rs, src/cmd/run.rs, src/cmd/list.rs and viola_state::liveness are reached whole and hold the state-recovered callers, so the caller list is not shown unmoved
span-trace-coverage#15.1 · no · the torn-line heal lives in the appender, and the Untouched list states the replay and appender code is not in the diff
span-trace-coverage#16.1 · yes · stale · src/cmd/run.rs (run.version_gate) and src/cmd/verify.rs (the stamp writer) are reached whole, so the ledger-read and parse-rejected claims are not shown unmoved
span-trace-coverage#17.1 · yes · add-to · the verify edge flow's process-exit now carries each refusal's code while a refused home tree stays lineless, and plugin install's --json exit-21 document now names its cause
