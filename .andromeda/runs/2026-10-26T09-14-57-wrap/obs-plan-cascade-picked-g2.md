# obs-plan · picked accounting · group 2 of 2

## Picked
span-trace-coverage#2.1 · proposed · the cli row lacks `last.client` and the six verbs' pre-frame server check (F1), and the captured-span cases (F6) sit beside it
span-trace-coverage#2.3 · proposed · the hook row gains the shared pre-frame `connect` and the SessionEnd fallback (F4)
span-trace-coverage#2.5 · proposed · the mcp row gains the frame tools' server check after connect, no line on a mismatch (F5)
span-trace-coverage#4.1 · proposed · `send.client` now runs the server check before `channel.request`, a mismatch opening none
span-trace-coverage#4.3 · proposed · the send log fields omit the server-verification refusal: no channel lines, the client's exit 21 `process-exit`
span-trace-coverage#4.4 · proposed · the Cleanup names `instance-dead` as exit 21's only detail; `server-verify-failed` (during connect) is now written
span-trace-coverage#5.1 · proposed · the check now runs inside `wait.block` / `last.client` before `channel.request`, after the exit-13 refusals
span-trace-coverage#5.3 · proposed · «server-verify-failed cannot occur on wait / last» is false on Linux and Windows (Retired); strict-modes-failed still cannot
span-trace-coverage#5.4 · proposed · `wait.block` now also closes on a pre-frame server-verification refusal
span-trace-coverage#6.1 · proposed · the dialog hook's `hook.dialog` and `answer.client`'s `answer` frame now follow a server check that can end the path
span-trace-coverage#6.3 · proposed · the D-07 rule holds, but the server-verify fail-open before `dialog_id` and CLI `answer`'s exit 21 are unnamed
span-trace-coverage#7.1 · proposed · `pause.client` / `release.client` now verify the server before their frame
span-trace-coverage#7.3 · proposed · the wheel log fields lack the pause / release exit 21 `server-verify-failed` line
span-trace-coverage#11.1 · proposed · the five other snapshot readers measure three by the stated rule; `src/cmd/client.rs` holds no call, hook.rs one
metric-coverage#1.3 · not touched · the gauge reads `liveness` through `agent-run status`, whose harness surface the report leaves unchanged
metric-coverage#2.1 · not touched · the hook-latency metric and its hyperfine `max` gate are unchanged; the new 2.2–2.4 ms readings are an SLO fact
log-coverage#1.7 · not touched · no `hook-invoked` / `hook-decision` field was added; F3 is placed at the fail-open code list
log-coverage#2.2 · proposed · `server-verify-failed` under exit 21 is now written by this build (F2)
log-coverage#2.4 · proposed · `server-verify-failed` among the hook fail-open codes is now written, with the dialog path's line order (F3)
pii-scrubbing-compliance#1.6 · not touched · no drift-report path is reached; the new lines are codes only
pii-scrubbing-compliance#2.1 · proposed · the verification list gains the server-identity negatives' `digit_runs` scan (F7)
obs-anti-patterns#1.1 · not touched · the report adds no print macro, no `#[allow]` and no `refuse` caller; `src/human.rs` is not reached
