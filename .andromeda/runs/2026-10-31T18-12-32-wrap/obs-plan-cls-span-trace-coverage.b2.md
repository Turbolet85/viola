# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · yes · stale · src/cmd/wait.rs and src/cmd/last.rs are reached whole, so the wait.block and run.wait_dispatch readings can move with them
span-trace-coverage#7.3 · yes · contradict · instance-dead no longer carries during call (endpoint-lost does, F4), the instance refusal's --json and MCP check_instance refusal now name a cause rather than null, and F5 is the liveness-only pre-check's no-endpoint reading
span-trace-coverage#7.4 · yes · add-to · a mid-call disconnect's client process-exit now reads endpoint-lost with during call, the F4 record
span-trace-coverage#8.1 · yes · stale · src/cmd/answer.rs and src/cmd/client.rs are reached, and the refuse_untrusted and check_instance refusals it describes now carry a cause
span-trace-coverage#8.2 · yes · stale · src/cmd/run.rs is reached whole and holds the run.dialog_register and run.dialog_await spans, whose readings can move with it
span-trace-coverage#8.3 · yes · contradict · MCP answer's check_instance refusal is stated with detail null, now a cause (F10), and the CLI --json instance-unreachable document now names its cause
span-trace-coverage#8.4 · yes · stale · src/cmd/run.rs is reached whole and holds the run.dialog_await close and dialog-expired path the cleanup describes
span-trace-coverage#9.1 · yes · stale · src/cmd/pause.rs and src/cmd/client.rs are reached, so the pause and release server check and the run.wheel_transition refusal can move
span-trace-coverage#9.2 · yes · stale · viola-core's lib.rs, which holds WheelCause and gained UnreachableCause, is reached whole, and src/cmd/run.rs holds the wheel span
span-trace-coverage#9.3 · yes · stale · src/cmd/pause.rs and src/cmd/client.rs are reached, so the pause and release server-verify refusal's process-exit can move with them
span-trace-coverage#9.4 · yes · stale · src/cmd/run.rs is reached whole and holds the run.wheel_transition point-in-time span the cleanup names
span-trace-coverage#10.1 · yes · stale · run.budget_eval sits in src/cmd/run.rs, which is reached whole, so its trigger path can move
span-trace-coverage#10.2 · yes · stale · run.budget_eval's attributes sit in src/cmd/run.rs, which is reached whole
span-trace-coverage#10.3 · no · the statusline and hook-stdin lines live in src/cmd/hook.rs, which is not in the diff, and no exit-21 or process-exit detail statement here moves
span-trace-coverage#10.4 · no · the statusline.shell_out bound and the hook-decision deadline live in src/cmd/hook.rs, which is not in the diff
span-trace-coverage#11.1 · yes · stale · the process-start{subject:"claude-child"} line the strip writes is emitted by src/cmd/run.rs, which is reached whole
span-trace-coverage#12.1 · no · the implicit link record is run::links, which emits no span or line and sits outside the reached set, and its trace is the channel pair the diff does not touch
span-trace-coverage#13.1 · no · the driven wrapper's link and unlink serving is run::links and the untouched channel pair, and the reached src/cmd/link.rs is the CLI client only
span-trace-coverage#14.1 · no · the state-recovered line and its spans live in viola_state::replay, which the diff does not touch, and without_endpoint opens nothing and writes no line
span-trace-coverage#15.1 · no · the torn-line heal is the appender's, which the diff does not touch, and the session adds no writer
span-trace-coverage#16.1 · yes · stale · src/cmd/run.rs (the version gate) and src/cmd/verify.rs (the stamps writer) are reached whole, and verify's refusals now write codes on process-exit (F2)
span-trace-coverage#17.1 · yes · stale · plugin install's refused check under --json now prints a cause, and verify's process-exit now carries a code per refusal (F2); src/cmd/plugin.rs and src/cmd/verify.rs are reached
